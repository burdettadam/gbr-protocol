//! `grimoire-gate-check` — Phase readiness gate checker. Rust port of `scripts/gate_check.py`.
//!
//! Usage:
//!   grimoire-gate-check [--workspace-path <path>] [--phase <PREFIX>]
//!                       [--no-report] [--no-yaml] [--quiet] [--no-color]

use std::{
    collections::{BTreeMap, HashMap, HashSet},
    fs,
    path::{Path, PathBuf},
};

use grimoire_tooling::{
    dag::SubPhaseDAG,
    enums::{CheckType, Severity},
    gates::{GateSpec, PhaseSpec, SubPhaseSpec},
};
use regex::Regex;
use serde::Deserialize;

// ── Phase directory list (fixed order) ───────────────────────────────────────

const PHASE_DIRS: &[&str] = &[
    "00-start-here",
    "01-concept",
    "02-collision",
    "03-characters",
    "04-world-building",
    "05-plot-and-structure",
    "06-scenes",
    "07-drafting",
    "08-revision",
    "09-polish-and-publish",
];

// ── CLI args ──────────────────────────────────────────────────────────────────

struct Args {
    workspace: PathBuf,
    phase_prefix: Option<String>,
    no_report: bool,
    no_yaml: bool,
    quiet: bool,
    no_color: bool,
}

fn parse_args() -> Args {
    let raw: Vec<String> = std::env::args().collect();
    let mut workspace = PathBuf::from(".");
    let mut phase_prefix = None;
    let mut no_report = false;
    let mut no_yaml = false;
    let mut quiet = false;
    let mut no_color = false;

    let mut i = 1;
    while i < raw.len() {
        match raw[i].as_str() {
            "--workspace-path" => {
                i += 1;
                if i < raw.len() {
                    workspace = PathBuf::from(&raw[i]);
                }
            }
            "--phase" => {
                i += 1;
                if i < raw.len() {
                    phase_prefix = Some(raw[i].clone());
                }
            }
            "--no-report" => no_report = true,
            "--no-yaml" => no_yaml = true,
            "--quiet" => quiet = true,
            "--no-color" => no_color = true,
            _ => {}
        }
        i += 1;
    }

    // Auto-disable color when piped
    let no_color = no_color || !atty();

    Args { workspace, phase_prefix, no_report, no_yaml, quiet, no_color }
}

fn atty() -> bool {
    // Simple check: TERM is set and stdout is not a pipe
    std::env::var("TERM").is_ok()
}

// ── ANSI helpers ──────────────────────────────────────────────────────────────

fn ansi(code: &str, text: &str, use_color: bool) -> String {
    if use_color {
        format!("{code}{text}\x1b[0m")
    } else {
        text.to_owned()
    }
}

const RED: &str = "\x1b[31m";
const YELLOW: &str = "\x1b[33m";
const GREEN: &str = "\x1b[32m";
const BOLD: &str = "\x1b[1m";
const DIM: &str = "\x1b[2m";

fn status_icon(status: &str) -> &'static str {
    match status {
        "green" => "✅",
        "yellow" => "⚠️ ",
        "red" => "❌",
        "locked" => "🔒",
        _ => "—",
    }
}

fn status_label(status: &str) -> &'static str {
    match status {
        "green" => "PASS",
        "yellow" => "WARN",
        "red" => "FAIL",
        "locked" => "LOCKED",
        _ => "N/A",
    }
}

fn sp_status_icon(status: &str) -> &'static str {
    match status {
        "locked" => "🔒",
        "ready" => "⬜",
        "in_progress" => "🟡",
        "complete" => "✅",
        _ => "—",
    }
}

fn color_for_status(status: &str) -> &'static str {
    match status {
        "green" => GREEN,
        "yellow" => YELLOW,
        "red" => RED,
        "locked" | "unknown" => DIM,
        _ => "",
    }
}

fn format_status(status: &str, use_color: bool) -> String {
    let icon = status_icon(status);
    let label = status_label(status);
    let text = format!("{icon} {label}");
    if use_color {
        let c = color_for_status(status);
        format!("{c}{text}\x1b[0m")
    } else {
        text
    }
}

// ── Frontmatter parsing helper ────────────────────────────────────────────────
// The YAML frontmatter in _gate.md does NOT contain phase_id (derived from dir name).

#[derive(Deserialize)]
struct GateFrontmatter {
    phase_label: Option<String>,
    #[serde(default)]
    sub_phases: Vec<SubPhaseSpec>,
    #[serde(default)]
    gates: Vec<GateSpec>,
}

fn parse_gate_md(phase_id: &str, path: &Path) -> Option<PhaseSpec> {
    let content = fs::read_to_string(path).ok()?;
    let front_re = Regex::new(r"(?s)\A\s*---\n(.*?)\n---\n?").ok()?;
    let m = front_re.captures(&content)?;
    let yaml_str = m.get(1)?.as_str();
    let fm: GateFrontmatter = serde_yaml::from_str(yaml_str).ok()?;
    Some(PhaseSpec {
        phase_id: phase_id.to_owned(),
        phase_label: fm.phase_label.unwrap_or_else(|| phase_id.to_owned()),
        sub_phases: fm.sub_phases,
        gates: fm.gates,
    })
}

// ── Text utilities (ported from chapter_utils.py) ────────────────────────────

fn strip_details_blocks(text: &str) -> String {
    let re = Regex::new(r"(?is)<details>.*?</details>").unwrap();
    re.replace_all(text, "").into_owned()
}

fn strip_frontmatter(text: &str) -> String {
    let re = Regex::new(r"(?s)\A\s*---\n.*?\n---\n?").unwrap();
    re.replace(text, "").into_owned()
}

fn strip_annotations(text: &str) -> String {
    let re = Regex::new(r"(?s)<!--.*?-->").unwrap();
    let cleaned = re.replace_all(text, "").into_owned();
    let multi_blank = Regex::new(r"\n{3,}").unwrap();
    multi_blank.replace_all(&cleaned, "\n\n").trim().to_owned()
}

/// Count (unfilled_placeholders, total_fields) matching chapter_utils.count_placeholders.
fn count_placeholders(content: &str) -> (usize, usize) {
    let mut text = strip_details_blocks(content);
    text = strip_frontmatter(&text);
    text = strip_annotations(&text);

    let placeholder_re = Regex::new(r"\[([^\[\]\n]{5,})\]").unwrap();
    let unfilled = placeholder_re.find_iter(&text).count();

    let field_label_re = Regex::new(r"(?m)^\*\*[^*]{2,}\*\*\s*[:：]").unwrap();
    let total = field_label_re.find_iter(&text).count().max(unfilled);
    (unfilled, total)
}

/// Count "filled" words: strip boilerplate, instructions, annotations, placeholders.
fn count_filled_words(content: &str) -> usize {
    let mut text = strip_details_blocks(content);
    text = strip_frontmatter(&text);
    text = strip_annotations(&text);
    // Strip placeholder brackets [Some instruction here]
    let ph_re = Regex::new(r"\[[^\[\]\n]{5,}\]").unwrap();
    text = ph_re.replace_all(&text, "").into_owned();
    text.split_whitespace().count()
}

/// Parse all <!-- key:value --> annotations in a file, returning Vec of k→v maps.
fn parse_annotations(content: &str) -> Vec<HashMap<String, String>> {
    let ann_re = Regex::new(r"(?s)<!--(.*?)-->").unwrap();
    let kv_re = Regex::new(r"(\w+):(\S+)").unwrap();
    ann_re
        .captures_iter(content)
        .map(|cap| {
            let inner = cap.get(1).map(|m| m.as_str()).unwrap_or("");
            kv_re
                .captures_iter(inner)
                .map(|kv| {
                    (
                        kv.get(1).unwrap().as_str().to_owned(),
                        kv.get(2).unwrap().as_str().to_owned(),
                    )
                })
                .collect()
        })
        .collect()
}

/// Return all values for `tag_type` (or `tag_type + "s"`) across annotations.
/// Comma-separated values are split. All tokens lowercased.
fn find_entity_tags(content: &str, tag_type: &str) -> HashSet<String> {
    let plural = format!("{tag_type}s");
    let mut result = HashSet::new();
    for ann in parse_annotations(content) {
        for (key, val) in &ann {
            if key == tag_type || key == &plural {
                for token in val.split(',') {
                    let t = token.trim().to_lowercase();
                    if !t.is_empty() {
                        result.insert(t);
                    }
                }
            }
        }
    }
    result
}

// ── Path resolver ─────────────────────────────────────────────────────────────

fn resolve_path(raw: &str, phase_dir: &Path, root: &Path) -> PathBuf {
    let p = Path::new(raw);
    if p.is_absolute() {
        return p.to_path_buf();
    }
    // Cross-directory paths (starting with "../" or a non-dot first component): try root first
    let first = p.components().next();
    let is_cross = raw.starts_with("../")
        || matches!(first, Some(c) if {
            let s = c.as_os_str().to_string_lossy();
            s != "." && s != ".."
        });
    if is_cross {
        let candidate = root.join(raw);
        if candidate.exists() {
            return candidate;
        }
    }
    phase_dir.join(raw)
}

// ── Check implementations ─────────────────────────────────────────────────────

struct CheckResult {
    status: &'static str,
    detail: String,
}

fn check_file_exists(gate: &GateSpec, phase_dir: &Path, root: &Path) -> CheckResult {
    let target_file = match &gate.target_file {
        Some(f) => f,
        None => return CheckResult { status: "yellow", detail: "No target_file specified".into() },
    };
    let path = resolve_path(target_file, phase_dir, root);
    match fs::read_to_string(&path) {
        Err(_) => CheckResult {
            status: "red",
            detail: format!("`{target_file}` not found"),
        },
        Ok(c) => {
            let words = c.split_whitespace().count();
            if words < 20 {
                CheckResult {
                    status: "red",
                    detail: format!("`{target_file}` exists but appears empty ({words} words)"),
                }
            } else {
                CheckResult {
                    status: "green",
                    detail: format!("`{target_file}` exists ({words} words)"),
                }
            }
        }
    }
}

fn check_placeholder_ratio(gate: &GateSpec, phase_dir: &Path, root: &Path) -> CheckResult {
    let target_file = match &gate.target_file {
        Some(f) => f,
        None => return CheckResult { status: "yellow", detail: "No target_file specified".into() },
    };
    let path = resolve_path(target_file, phase_dir, root);
    let content = match fs::read_to_string(&path) {
        Err(_) => return CheckResult { status: "red", detail: format!("`{target_file}` not found") },
        Ok(c) => c,
    };
    let (unfilled, total) = count_placeholders(&content);
    if total == 0 {
        return CheckResult {
            status: "yellow",
            detail: format!("No placeholder fields detected in `{target_file}` — verify template is correct"),
        };
    }
    let pct = (unfilled * 100 / total) as i64;
    let max_pct = gate.max_placeholder_pct.map(|v| (v * 100.0) as i64).unwrap_or(20);
    let status = if pct <= max_pct { "green" } else if pct <= max_pct + 25 { "yellow" } else { "red" };
    CheckResult {
        status,
        detail: format!(
            "{unfilled}/{total} placeholders remaining ({pct}%) in `{target_file}` (threshold: {max_pct}%)"
        ),
    }
}

fn check_checkbox_completion(gate: &GateSpec, phase_dir: &Path, root: &Path) -> CheckResult {
    let target_file = match &gate.target_file {
        Some(f) => f,
        None => return CheckResult { status: "yellow", detail: "No target_file specified".into() },
    };
    let path = resolve_path(target_file, phase_dir, root);
    let content = match fs::read_to_string(&path) {
        Err(_) => return CheckResult { status: "red", detail: format!("`{target_file}` not found") },
        Ok(c) => c,
    };
    let checked_re = Regex::new(r"(?im)^\s*-\s+\[x\]").unwrap();
    let unchecked_re = Regex::new(r"(?m)^\s*-\s+\[ \]").unwrap();
    let checked = checked_re.find_iter(&content).count();
    let unchecked = unchecked_re.find_iter(&content).count();
    let total = checked + unchecked;
    if total == 0 {
        return CheckResult {
            status: "yellow",
            detail: format!("No checkboxes found in `{target_file}`"),
        };
    }
    let pct = (checked * 100 / total) as i64;
    let min_pct = gate.min_completion_pct.map(|v| (v * 100.0) as i64).unwrap_or(50);
    let status = if pct >= min_pct { "green" } else if pct >= min_pct / 2 { "yellow" } else { "red" };
    CheckResult {
        status,
        detail: format!(
            "{checked}/{total} checkboxes completed ({pct}%) in `{target_file}` (min: {min_pct}%)"
        ),
    }
}

fn check_word_count_min(gate: &GateSpec, phase_dir: &Path, root: &Path) -> CheckResult {
    let target_file = match &gate.target_file {
        Some(f) => f,
        None => return CheckResult { status: "yellow", detail: "No target_file specified".into() },
    };
    let path = resolve_path(target_file, phase_dir, root);
    let content = match fs::read_to_string(&path) {
        Err(_) => return CheckResult { status: "red", detail: format!("`{target_file}` not found") },
        Ok(c) => c,
    };
    let words = count_filled_words(&content);
    let min_words = gate.min_words.unwrap_or(100);
    let status = if words >= min_words as usize {
        "green"
    } else if words >= (min_words as f32 * 0.6) as usize {
        "yellow"
    } else {
        "red"
    };
    CheckResult {
        status,
        detail: format!("{words} filled words in `{target_file}` (min: {min_words})"),
    }
}

fn check_tag_cross_ref(gate: &GateSpec, phase_dir: &Path, root: &Path) -> CheckResult {
    let source_file = match &gate.source_file {
        Some(f) => f,
        None => return CheckResult { status: "yellow", detail: "No source_file specified".into() },
    };
    let target_file = match &gate.target_file {
        Some(f) => f,
        None => return CheckResult { status: "yellow", detail: "No target_file specified".into() },
    };
    let source_tag = match &gate.source_tag {
        Some(t) => t.clone(),
        None => return CheckResult { status: "yellow", detail: "No source_tag specified".into() },
    };

    let source_content = match fs::read_to_string(resolve_path(source_file, phase_dir, root)) {
        Err(_) => return CheckResult { status: "red", detail: format!("Source `{source_file}` not found") },
        Ok(c) => c,
    };
    let target_content = match fs::read_to_string(resolve_path(target_file, phase_dir, root)) {
        Err(_) => return CheckResult { status: "red", detail: format!("Target `{target_file}` not found") },
        Ok(c) => c,
    };

    let source_entities = find_entity_tags(&source_content, &source_tag);
    if source_entities.is_empty() {
        return CheckResult {
            status: "yellow",
            detail: format!(
                "No `<!-- {source_tag}:... -->` tags in `{source_file}` — add tags to enable cross-reference check"
            ),
        };
    }

    let target_tag = gate.target_tag.as_deref().unwrap_or(&source_tag);
    let target_entities = find_entity_tags(&target_content, target_tag);
    let missing: Vec<String> = {
        let mut v: Vec<_> = source_entities.difference(&target_entities).cloned().collect();
        v.sort();
        v
    };

    if missing.is_empty() {
        return CheckResult {
            status: "green",
            detail: format!("All {} `{source_tag}` entities found in `{target_file}`", source_entities.len()),
        };
    }
    let pct_missing = missing.len() as f64 / source_entities.len() as f64;
    let status = if pct_missing > 0.5 { "red" } else { "yellow" };
    let list = missing.iter().map(|e| format!("`{e}`")).collect::<Vec<_>>().join(", ");
    CheckResult {
        status,
        detail: format!(
            "{}/{} entities missing from `{target_file}`: {list}",
            missing.len(),
            source_entities.len()
        ),
    }
}

fn check_entity_coverage(gate: &GateSpec, phase_dir: &Path, root: &Path) -> CheckResult {
    let source_file = match &gate.source_file {
        Some(f) => f,
        None => return CheckResult { status: "yellow", detail: "No source_file specified".into() },
    };
    let target_file = match &gate.target_file {
        Some(f) => f,
        None => return CheckResult { status: "yellow", detail: "No target_file specified".into() },
    };
    let source_tag = match &gate.source_tag {
        Some(t) => t.clone(),
        None => return CheckResult { status: "yellow", detail: "No source_tag specified".into() },
    };

    let source_content = match fs::read_to_string(resolve_path(source_file, phase_dir, root)) {
        Err(_) => return CheckResult { status: "red", detail: format!("Source `{source_file}` not found") },
        Ok(c) => c,
    };
    let target_content = match fs::read_to_string(resolve_path(target_file, phase_dir, root)) {
        Err(_) => return CheckResult { status: "red", detail: format!("Target `{target_file}` not found") },
        Ok(c) => c,
    };

    let source_entities = find_entity_tags(&source_content, &source_tag);
    if source_entities.is_empty() {
        return CheckResult {
            status: "yellow",
            detail: format!(
                "No `<!-- {source_tag}:... -->` tags in `{source_file}` — add tags to enable entity coverage check"
            ),
        };
    }

    let target_tag = gate.target_tag.as_deref().unwrap_or(&source_tag);
    let target_entities = find_entity_tags(&target_content, target_tag);
    let missing: Vec<String> = {
        let mut v: Vec<_> = source_entities.difference(&target_entities).cloned().collect();
        v.sort();
        v
    };

    if missing.is_empty() {
        return CheckResult {
            status: "green",
            detail: format!("All {} `{source_tag}` entities have coverage in `{target_file}`", source_entities.len()),
        };
    }
    let pct_missing = missing.len() as f64 / source_entities.len() as f64;
    let status = if pct_missing > 0.5 { "red" } else { "yellow" };
    let list = missing.iter().map(|e| format!("`{e}`")).collect::<Vec<_>>().join(", ");
    CheckResult {
        status,
        detail: format!("{} entities missing coverage in `{target_file}`: {list}", missing.len()),
    }
}

fn run_gate_check(gate: &GateSpec, phase_dir: &Path, root: &Path) -> CheckResult {
    match gate.check_type {
        CheckType::FileExists => check_file_exists(gate, phase_dir, root),
        CheckType::PlaceholderRatio => check_placeholder_ratio(gate, phase_dir, root),
        CheckType::CheckboxCompletion => check_checkbox_completion(gate, phase_dir, root),
        CheckType::WordCountMin => check_word_count_min(gate, phase_dir, root),
        CheckType::TagCrossRef => check_tag_cross_ref(gate, phase_dir, root),
        CheckType::EntityCoverage => check_entity_coverage(gate, phase_dir, root),
        CheckType::SchemaValid | CheckType::SourceCoverage => CheckResult {
            status: "yellow",
            detail: format!("Check type `{}` not yet implemented", gate.check_type),
        },
    }
}

// ── Running result types ──────────────────────────────────────────────────────

#[derive(Clone)]
struct GateRunResult {
    question: String,
    severity: Severity,
    sub_phase: Option<String>,
    status: String,
    detail: String,
}

#[derive(Clone)]
struct SubPhaseRunResult {
    id: String,
    label: String,
    order: u32,
    status: String,
    score: u32,
    gates: BTreeMap<String, GateRunResult>,
}

#[derive(Clone)]
struct PhaseRunResult {
    phase_label: String,
    status: String,
    score: Option<u32>,
    error: Option<String>,
    gates: BTreeMap<String, GateRunResult>,
    sub_phases: BTreeMap<String, SubPhaseRunResult>,
}

fn gate_weight(status: &str) -> f64 {
    match status {
        "green" => 1.0,
        "yellow" => 0.5,
        _ => 0.0,
    }
}

fn compute_sp_score_status(sp_gates: &BTreeMap<String, GateRunResult>) -> (String, u32) {
    let non_locked: Vec<_> = sp_gates.values().filter(|g| g.status != "locked").collect();
    if non_locked.is_empty() {
        return ("ready".to_owned(), 0);
    }
    let score = (non_locked.iter().map(|g| gate_weight(&g.status)).sum::<f64>()
        / non_locked.len() as f64
        * 100.0)
        .round() as u32;
    let statuses: HashSet<&str> = non_locked.iter().map(|g| g.status.as_str()).collect();
    let status = if statuses.iter().all(|s| *s == "green") {
        "complete"
    } else if statuses.iter().all(|s| *s == "red" || *s == "unknown") {
        "ready"
    } else {
        "in_progress"
    };
    (status.to_owned(), score)
}

fn compute_phase_score_status(gates: &BTreeMap<String, GateRunResult>) -> (String, u32) {
    let evaluable: Vec<_> = gates.values().filter(|g| g.status != "locked").collect();
    let score = if evaluable.is_empty() {
        0
    } else {
        (evaluable.iter().map(|g| gate_weight(&g.status)).sum::<f64>()
            / evaluable.len() as f64
            * 100.0)
            .round() as u32
    };
    let has_red_required = evaluable
        .iter()
        .any(|g| g.status == "red" && g.severity == Severity::Required);
    let has_yellow_or_red = evaluable.iter().any(|g| g.status == "red" || g.status == "yellow");
    let status = if has_red_required {
        "red"
    } else if has_yellow_or_red {
        "yellow"
    } else {
        "green"
    };
    (status.to_owned(), score)
}

// ── Phase runner ──────────────────────────────────────────────────────────────

fn run_phase(
    phase_id: &str,
    phase_dir: &Path,
    root: &Path,
    dag: Option<&SubPhaseDAG>,
    completed: &HashSet<String>,
) -> PhaseRunResult {
    let gate_md = phase_dir.join("_gate.md");
    if !gate_md.exists() {
        return PhaseRunResult {
            phase_label: phase_id.to_owned(),
            status: "unknown".into(),
            score: None,
            error: Some("No _gate.md found".into()),
            gates: BTreeMap::new(),
            sub_phases: BTreeMap::new(),
        };
    }

    let spec = match parse_gate_md(phase_id, &gate_md) {
        None => {
            return PhaseRunResult {
                phase_label: phase_id.to_owned(),
                status: "unknown".into(),
                score: None,
                error: Some("Failed to parse _gate.md frontmatter".into()),
                gates: BTreeMap::new(),
                sub_phases: BTreeMap::new(),
            }
        }
        Some(s) => s,
    };

    if spec.gates.is_empty() {
        return PhaseRunResult {
            phase_label: spec.phase_label.clone(),
            status: "unknown".into(),
            score: None,
            error: Some("No gates defined in frontmatter".into()),
            gates: BTreeMap::new(),
            sub_phases: BTreeMap::new(),
        };
    }

    // Determine locked sub-phases
    let mut locked_sps: HashSet<String> = HashSet::new();
    if let Some(dag) = dag {
        for sp in &spec.sub_phases {
            let fqid = format!("{phase_id}/{}", sp.id);
            if !dag.is_unlocked(&fqid, completed) {
                locked_sps.insert(sp.id.clone());
            }
        }
    }

    // Run all gates
    let mut gate_results: BTreeMap<String, GateRunResult> = BTreeMap::new();
    for gate in &spec.gates {
        let sp = gate.sub_phase.clone();
        let is_locked = sp.as_deref().map_or(false, |s| locked_sps.contains(s));
        let (status, detail) = if is_locked {
            let sp_id = sp.as_deref().unwrap_or("?");
            (
                "locked".to_owned(),
                format!("Sub-phase `{sp_id}` is locked — complete prerequisites first"),
            )
        } else {
            let r = run_gate_check(gate, phase_dir, root);
            (r.status.to_owned(), r.detail)
        };
        gate_results.insert(
            gate.id.clone(),
            GateRunResult {
                question: gate.question.clone(),
                severity: gate.severity,
                sub_phase: sp,
                status,
                detail,
            },
        );
    }

    // Build sub-phase results
    let mut sp_results: BTreeMap<String, SubPhaseRunResult> = BTreeMap::new();
    for sp in &spec.sub_phases {
        let sp_gates: BTreeMap<String, GateRunResult> = gate_results
            .iter()
            .filter(|(_, g)| g.sub_phase.as_deref() == Some(&sp.id))
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();

        let (sp_status, sp_score) = if locked_sps.contains(&sp.id) {
            ("locked".to_owned(), 0)
        } else {
            compute_sp_score_status(&sp_gates)
        };

        sp_results.insert(
            sp.id.clone(),
            SubPhaseRunResult {
                id: sp.id.clone(),
                label: sp.label.clone(),
                order: sp.order,
                status: sp_status,
                score: sp_score,
                gates: sp_gates,
            },
        );
    }

    let (phase_status, phase_score) = compute_phase_score_status(&gate_results);

    PhaseRunResult {
        phase_label: spec.phase_label.clone(),
        status: phase_status,
        score: Some(phase_score),
        error: None,
        gates: gate_results,
        sub_phases: sp_results,
    }
}

// ── YAML output ───────────────────────────────────────────────────────────────

fn write_yaml_output(all_results: &BTreeMap<String, PhaseRunResult>, root: &Path) {
    use serde_yaml::{Mapping, Value};

    fn str_val(s: &str) -> Value {
        Value::String(s.to_owned())
    }
    fn gate_to_yaml(g: &GateRunResult) -> Value {
        let mut m = Mapping::new();
        m.insert(str_val("status"), str_val(&g.status));
        m.insert(str_val("detail"), str_val(&g.detail));
        Value::Mapping(m)
    }

    let now = chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
    let mut root_map = Mapping::new();
    root_map.insert(str_val("last_run"), str_val(&now));

    let mut phases_map = Mapping::new();
    for (phase_name, result) in all_results {
        let mut phase_entry = Mapping::new();
        phase_entry.insert(str_val("status"), str_val(&result.status));
        match result.score {
            Some(s) => phase_entry.insert(str_val("score"), Value::Number(s.into())),
            None => phase_entry.insert(str_val("score"), Value::Null),
        };
        if let Some(err) = &result.error {
            phase_entry.insert(str_val("error"), str_val(err));
        }

        if !result.sub_phases.is_empty() {
            let mut sp_map = Mapping::new();
            let mut sorted_sps: Vec<_> = result.sub_phases.values().collect();
            sorted_sps.sort_by_key(|s| s.order);
            for sp in sorted_sps {
                let mut sp_entry = Mapping::new();
                sp_entry.insert(str_val("label"), str_val(&sp.label));
                sp_entry.insert(str_val("order"), Value::Number(sp.order.into()));
                sp_entry.insert(str_val("status"), str_val(&sp.status));
                sp_entry.insert(str_val("score"), Value::Number(sp.score.into()));
                let mut sp_gates_map = Mapping::new();
                for (gid, g) in &sp.gates {
                    sp_gates_map.insert(str_val(gid), gate_to_yaml(g));
                }
                sp_entry.insert(str_val("gates"), Value::Mapping(sp_gates_map));
                sp_map.insert(str_val(&sp.id), Value::Mapping(sp_entry));
            }
            phase_entry.insert(str_val("sub_phases"), Value::Mapping(sp_map));
        }

        let mut gates_map = Mapping::new();
        for (gid, g) in &result.gates {
            gates_map.insert(str_val(gid), gate_to_yaml(g));
        }
        phase_entry.insert(str_val("gates"), Value::Mapping(gates_map));
        phases_map.insert(str_val(phase_name), Value::Mapping(phase_entry));
    }
    root_map.insert(str_val("phases"), Value::Mapping(phases_map));

    let out = serde_yaml::to_string(&Value::Mapping(root_map))
        .unwrap_or_else(|e| format!("# YAML serialisation error: {e}\n"));
    let path = root.join("readiness.yaml");
    fs::write(&path, &out).expect("could not write readiness.yaml");
    eprintln!("\nReadiness state written to readiness.yaml");
}

// ── Markdown report ───────────────────────────────────────────────────────────

fn write_markdown_report(all_results: &BTreeMap<String, PhaseRunResult>, root: &Path) {
    let ts = chrono::Utc::now().format("%Y-%m-%d %H:%M UTC").to_string();
    let mut lines: Vec<String> = vec![
        "# Grimoire Readiness Report".into(),
        format!("\n_Generated: {ts}_\n"),
        "## Summary\n".into(),
        "| Phase | Status | Score |".into(),
        "|-------|--------|-------|".into(),
    ];

    for (phase_name, result) in all_results {
        let icon = status_icon(&result.status);
        let score_str = result.score.map(|s| format!("{s}%")).unwrap_or_else(|| "—".into());
        let label = &result.phase_label;
        lines.push(format!("| {label} | {icon} {} | {score_str} |", result.status.to_uppercase()));
        let _ = phase_name;
    }
    lines.push("\n---\n".into());

    for (_, result) in all_results {
        let label = &result.phase_label;
        let icon = status_icon(&result.status);
        let score_str = result.score.map(|s| format!("{s}%")).unwrap_or_else(|| "—".into());
        lines.push(format!("## {icon} {label} ({score_str})\n"));

        if let Some(err) = &result.error {
            lines.push(format!("> **Note:** {err}\n"));
            continue;
        }

        if !result.sub_phases.is_empty() {
            let mut sorted_sps: Vec<_> = result.sub_phases.values().collect();
            sorted_sps.sort_by_key(|s| s.order);
            for sp in sorted_sps {
                let sp_icon = sp_status_icon(&sp.status);
                let sp_score = format!("{}%", sp.score);
                lines.push(format!("### {sp_icon} {} ({sp_score})\n", sp.label));
                if sp.status == "locked" {
                    lines.push("> 🔒 **Locked** — complete prerequisite sub-phases first.\n".into());
                    continue;
                }
                for (gate_id, gr) in &sp.gates {
                    append_gate_line(&mut lines, gate_id, gr);
                }
                lines.push(String::new());
            }
        } else {
            for (gate_id, gr) in &result.gates {
                append_gate_line(&mut lines, gate_id, gr);
            }
        }
        lines.push(String::new());
    }

    let path = root.join("readiness-report.md");
    fs::write(&path, lines.join("\n")).expect("could not write readiness-report.md");
    eprintln!("Report written to readiness-report.md");
}

fn append_gate_line(lines: &mut Vec<String>, gate_id: &str, gr: &GateRunResult) {
    let icon = status_icon(&gr.status);
    let badge = if gr.severity == Severity::Required { "_(required)_" } else { "_(recommended)_" };
    lines.push(format!("- {icon} **{gate_id}** {badge}"));
    if !gr.question.is_empty() {
        lines.push(format!("  - _Q: {}_", gr.question));
    }
    lines.push(format!("  - {}", gr.detail));
}

// ── Terminal dashboard ────────────────────────────────────────────────────────

fn print_dashboard(all_results: &BTreeMap<String, PhaseRunResult>, use_color: bool) {
    println!();
    println!("{}", ansi(BOLD, "  GRIMOIRE READINESS DASHBOARD", use_color));
    println!("{}", ansi(DIM, &format!("  {}", "─".repeat(72)), use_color));
    println!("  {:<28}  {:<14}  {:>6}  Gates", "Phase", "Status", "Score");
    println!("{}", ansi(DIM, &format!("  {}", "─".repeat(72)), use_color));

    for (_, result) in all_results {
        let label = &result.phase_label;
        let score_str = result.score.map(|s| format!("{s:>3}%")).unwrap_or_else(|| "  N/A".into());
        let status_str = format_status(&result.status, use_color);

        // Gate summary
        let gate_summary = if let Some(err) = &result.error {
            ansi(DIM, err, use_color)
        } else {
            let mut green = 0usize;
            let mut yellow = 0usize;
            let mut red = 0usize;
            let mut locked = 0usize;
            for g in result.gates.values() {
                match g.status.as_str() {
                    "green" => green += 1,
                    "yellow" => yellow += 1,
                    "red" => red += 1,
                    "locked" => locked += 1,
                    _ => {}
                }
            }
            let mut parts = Vec::new();
            if green > 0 {
                parts.push(ansi(GREEN, &format!("✅ {green}"), use_color));
            }
            if yellow > 0 {
                parts.push(ansi(YELLOW, &format!("⚠️  {yellow}"), use_color));
            }
            if red > 0 {
                parts.push(ansi(RED, &format!("❌ {red}"), use_color));
            }
            if locked > 0 {
                parts.push(ansi(DIM, &format!("🔒 {locked}"), use_color));
            }
            parts.join("  ")
        };

        println!("  {label:<28}  {status_str:<23}  {score_str}  {gate_summary}");

        // Sub-phase breakdown
        if !result.sub_phases.is_empty() {
            let mut sorted_sps: Vec<_> = result.sub_phases.values().collect();
            sorted_sps.sort_by_key(|s| s.order);
            for sp in sorted_sps {
                let sp_icon = sp_status_icon(&sp.status);
                let sp_score = format!("{:>3}%", sp.score);
                let sp_detail = if sp.status == "locked" {
                    ansi(DIM, "locked", use_color)
                } else {
                    format!("{} gates", sp.gates.len())
                };
                println!("    {sp_icon} {:<24}  {sp_score}  {sp_detail}", sp.label);
            }
        }
    }

    println!("{}", ansi(DIM, &format!("  {}", "─".repeat(72)), use_color));

    // Overall summary
    let statuses: Vec<&str> = all_results
        .values()
        .filter(|r| r.status != "unknown")
        .map(|r| r.status.as_str())
        .collect();
    if !statuses.is_empty() {
        let overall = if statuses.iter().all(|s| *s == "green") {
            ansi(GREEN, "✅  All phases READY", use_color)
        } else if statuses.iter().any(|s| *s == "red") {
            ansi(RED, "❌  One or more phases BLOCKED", use_color)
        } else {
            ansi(YELLOW, "⚠️   Some phases need attention", use_color)
        };
        println!("\n  {overall}\n");
    }
}

// ── Main ──────────────────────────────────────────────────────────────────────

fn main() {
    let args = parse_args();
    let root = args.workspace.canonicalize().unwrap_or(args.workspace.clone());

    // Discover phase directories in fixed order
    let phase_dirs: Vec<(String, PathBuf)> = PHASE_DIRS
        .iter()
        .filter_map(|&name| {
            if let Some(ref prefix) = args.phase_prefix {
                if !name.starts_with(prefix.as_str()) {
                    return None;
                }
            }
            let d = root.join(name);
            if d.is_dir() { Some((name.to_owned(), d)) } else { None }
        })
        .collect();

    if phase_dirs.is_empty() {
        eprintln!("No matching phase directories found.");
        std::process::exit(1);
    }

    // Pre-parse all PhaseSpecs to build the DAG
    let mut phase_specs: HashMap<String, PhaseSpec> = HashMap::new();
    for (phase_name, phase_dir) in &phase_dirs {
        let gate_md = phase_dir.join("_gate.md");
        if let Some(spec) = parse_gate_md(phase_name, &gate_md) {
            if !spec.sub_phases.is_empty() {
                phase_specs.insert(phase_name.clone(), spec);
            }
        }
    }

    let dag = if phase_specs.is_empty() {
        None
    } else {
        match SubPhaseDAG::build(&phase_specs) {
            Ok(d) => Some(d),
            Err(e) => {
                eprintln!("Warning: DAG build failed: {e}");
                None
            }
        }
    };

    // First pass: run all phases
    let mut completed: HashSet<String> = HashSet::new();
    let mut all_results: BTreeMap<String, PhaseRunResult> = BTreeMap::new();
    for (phase_name, phase_dir) in &phase_dirs {
        let result = run_phase(phase_name, phase_dir, &root, dag.as_ref(), &completed);
        for (sp_id, sp) in &result.sub_phases {
            if sp.status == "complete" {
                completed.insert(format!("{phase_name}/{sp_id}"));
            }
        }
        all_results.insert(phase_name.clone(), result);
    }

    // Convergence loop: re-run phases with locked sub-phases until no new
    // sub-phases are unlocked.  The first pass only sees completions from
    // *earlier* phases; deeper dependency chains (e.g. core-identity →
    // relationships → cross-refs, depth 3) need additional iterations.
    if dag.is_some() {
        loop {
            let prev_completed_count = completed.len();
            for (phase_name, phase_dir) in &phase_dirs {
                let prev = &all_results[phase_name];
                let has_locked = prev.sub_phases.values().any(|sp| sp.status == "locked");
                if has_locked {
                    let result = run_phase(phase_name, phase_dir, &root, dag.as_ref(), &completed);
                    for (sp_id, sp) in &result.sub_phases {
                        if sp.status == "complete" {
                            completed.insert(format!("{phase_name}/{sp_id}"));
                        }
                    }
                    all_results.insert(phase_name.clone(), result);
                }
            }
            // Fixed point reached — no new completions
            if completed.len() == prev_completed_count {
                break;
            }
        }
    }

    // Output
    if !args.quiet {
        print_dashboard(&all_results, !args.no_color);
    }
    if !args.no_yaml {
        write_yaml_output(&all_results, &root);
    }
    if !args.no_report {
        write_markdown_report(&all_results, &root);
    }

    // Exit 1 if any required gate is red
    let has_failure = all_results.values().any(|r| {
        r.gates.values().any(|g| g.status == "red" && g.severity == Severity::Required)
    });
    std::process::exit(if has_failure { 1 } else { 0 });
}
