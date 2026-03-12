/// `grimoire-cap-validate` — three-level conformance checker for SIP artifacts.
///
/// Usage:
///   grimoire-cap-validate --path <file.json> [--level {1|2|3}] [--json]
///
/// Conformance levels:
///   1  Structural: JSON is well-formed and matches the CapArtifact type schema;
///      protocol field equals "semantic-interaction-protocol".
///   2  Referential: every entity_ref in units.observables.participants,
///      units.participant_states, and relationships resolves to a declared
///      entity_id in artifact.entities.
///   3  Semantic: step sequence_numbers within each unit are ascending and
///      contain no duplicates.
///
/// Exit codes:
///   0   all requested levels pass
///   1   one or more conformance failures
///   2   file not found or JSON parse error

use std::collections::HashSet;
use std::path::PathBuf;

use cap_narrative_types::cap::artifact::CapArtifact;

// ── Diagnostics ──────────────────────────────────────────────────────────────

#[derive(Debug)]
struct Diagnostic {
    level: u8,
    is_blocking: bool,
    path: String,
    message: String,
}

// ── CLI args ─────────────────────────────────────────────────────────────────

struct Args {
    path: PathBuf,
    level: u8,
    json_output: bool,
}

fn parse_args() -> Args {
    let raw: Vec<String> = std::env::args().collect();
    let mut path: Option<PathBuf> = None;
    let mut level: u8 = 3;
    let mut json_output = false;

    let mut i = 1;
    while i < raw.len() {
        match raw[i].as_str() {
            "--path" => {
                i += 1;
                path = Some(PathBuf::from(&raw[i]));
            }
            "--level" => {
                i += 1;
                level = raw[i].parse().unwrap_or_else(|_| {
                    eprintln!("ERROR: --level must be 1, 2, or 3");
                    std::process::exit(2);
                });
                if level < 1 || level > 3 {
                    eprintln!("ERROR: --level must be 1, 2, or 3");
                    std::process::exit(2);
                }
            }
            "--json" => json_output = true,
            "--help" | "-h" => {
                eprintln!(
                    "Usage: grimoire-cap-validate --path <file.json> [--level {{1|2|3}}] [--json]"
                );
                std::process::exit(0);
            }
            other => {
                eprintln!("ERROR: unknown flag '{other}'");
                std::process::exit(2);
            }
        }
        i += 1;
    }

    let path = path.unwrap_or_else(|| {
        eprintln!("ERROR: --path <file.json> is required");
        std::process::exit(2);
    });

    Args { path, level, json_output }
}

// ── Validation ────────────────────────────────────────────────────────────────

fn run_validation(artifact: &CapArtifact, level: u8) -> Vec<Diagnostic> {
    let mut diags: Vec<Diagnostic> = Vec::new();

    // ── Level 1 ──────────────────────────────────────────────────────────────
    // Structural validity is already guaranteed by successful deserialization
    // to CapArtifact.  Only the protocol field value is checked here.
    if !artifact.is_valid_protocol() {
        diags.push(Diagnostic {
            level: 1,
            is_blocking: true,
            path: "protocol".to_string(),
            message: format!(
                "must be 'semantic-interaction-protocol', got '{}'",
                artifact.protocol
            ),
        });
    }

    if level >= 2 {
        // ── Level 2: referential integrity ───────────────────────────────────
        let known_ids: HashSet<&str> = artifact.entity_ids().collect();

        for (ui, unit) in artifact.units.iter().enumerate() {
            for (pi, participant) in unit.observables.participants.iter().enumerate() {
                if !known_ids.contains(participant.as_str()) {
                    diags.push(Diagnostic {
                        level: 2,
                        is_blocking: true,
                        path: format!("units[{ui}].observables.participants[{pi}]"),
                        message: format!("entity_ref '{participant}' not in artifact.entities"),
                    });
                }
            }

            for (si, ps) in unit.participant_states.iter().enumerate() {
                if !known_ids.contains(ps.entity_ref.as_str()) {
                    diags.push(Diagnostic {
                        level: 2,
                        is_blocking: true,
                        path: format!("units[{ui}].participant_states[{si}].entity_ref"),
                        message: format!("entity_ref '{}' not in artifact.entities", ps.entity_ref),
                    });
                }
            }
        }

        for (ri, rel) in artifact.relationships.iter().enumerate() {
            if !known_ids.contains(rel.source.as_str()) {
                diags.push(Diagnostic {
                    level: 2,
                    is_blocking: true,
                    path: format!("relationships[{ri}].source"),
                    message: format!("entity_ref '{}' not in artifact.entities", rel.source),
                });
            }
            if !known_ids.contains(rel.target.as_str()) {
                diags.push(Diagnostic {
                    level: 2,
                    is_blocking: true,
                    path: format!("relationships[{ri}].target"),
                    message: format!("entity_ref '{}' not in artifact.entities", rel.target),
                });
            }
        }
    }

    if level >= 3 {
        // ── Level 3: step ordering ────────────────────────────────────────────
        for (ui, unit) in artifact.units.iter().enumerate() {
            let steps = unit
                .structure
                .as_ref()
                .map(|s| s.steps.as_slice())
                .unwrap_or(&[]);

            if steps.len() < 2 {
                continue;
            }

            let seqs: Vec<u32> = steps.iter().map(|s| s.sequence_number).collect();
            let mut seen: HashSet<u32> = HashSet::new();

            for (si, &seq) in seqs.iter().enumerate() {
                if !seen.insert(seq) {
                    diags.push(Diagnostic {
                        level: 3,
                        is_blocking: true,
                        path: format!("units[{ui}].structure.steps[{si}].sequence_number"),
                        message: format!("duplicate sequence_number {seq}"),
                    });
                }
            }

            for window in seqs.windows(2) {
                if window[0] >= window[1] {
                    diags.push(Diagnostic {
                        level: 3,
                        is_blocking: true,
                        path: format!("units[{ui}].structure.steps"),
                        message: format!("sequence_numbers not ascending: {seqs:?}"),
                    });
                    break; // one diagnostic per unit is enough
                }
            }
        }
    }

    diags
}

// ── Output ────────────────────────────────────────────────────────────────────

fn print_human(
    path: &PathBuf,
    diags: &[Diagnostic],
    level_requested: u8,
    _l1_available: bool,
) {
    let tick = "✓";
    let cross = "✗";
    let skip = "–";
    let lock = "🔒";

    let name = path.file_name().map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| path.display().to_string());

    let l1_ok = !diags.iter().any(|d| d.level == 1 && d.is_blocking);
    let l2_ok = !diags.iter().any(|d| d.level == 2 && d.is_blocking);
    let l3_ok = !diags.iter().any(|d| d.level == 3 && d.is_blocking);

    let l1_sym = if l1_ok { tick } else { cross };
    let l2_sym = if level_requested < 2 { skip } else if !l1_ok { lock } else if l2_ok { tick } else { cross };
    let l3_sym = if level_requested < 3 { skip } else if !l1_ok { lock } else if l3_ok { tick } else { cross };

    println!("{l1_sym} Level 1 (structural)   — {name}");
    println!("{l2_sym} Level 2 (referential)  — {name}");
    println!("{l3_sym} Level 3 (semantic)     — {name}");

    for d in diags {
        println!("  {cross} [L{}] {}: {}", d.level, d.path, d.message);
    }

    let checked: Vec<bool> = {
        let mut v = vec![l1_ok];
        // Only count deeper levels when L1 passed (mirrors Python validator behaviour)
        if level_requested >= 2 && l1_ok { v.push(l2_ok); }
        if level_requested >= 3 && l1_ok { v.push(l3_ok); }
        v
    };
    let passed = checked.iter().filter(|&&b| b).count();
    println!("\nPassed: {}/{} level(s)", passed, checked.len());
}

fn print_json(diags: &[Diagnostic], passed: bool) {
    let obj = serde_json::json!({
        "passed": passed,
        "diagnostics": diags.iter().map(|d| serde_json::json!({
            "level": d.level,
            "severity": if d.is_blocking { "ERROR" } else { "WARNING" },
            "path": d.path,
            "message": d.message,
        })).collect::<Vec<_>>()
    });
    println!("{}", serde_json::to_string_pretty(&obj).unwrap());
}

// ── Main ──────────────────────────────────────────────────────────────────────

fn main() {
    let args = parse_args();

    println!("SIP Validate v0.1.0  ({})", args.path.display());
    println!("{}", "━".repeat(40));

    // Read file
    let content = std::fs::read_to_string(&args.path).unwrap_or_else(|e| {
        eprintln!("ERROR: cannot read '{}': {e}", args.path.display());
        std::process::exit(2);
    });

    // Level 1: JSON parse + structural deserialization
    let parse_result: serde_json::Result<CapArtifact> = serde_json::from_str(&content);

    let (artifact, l1_parse_ok) = match parse_result {
        Ok(a) => (Some(a), true),
        Err(e) => {
            let diags = vec![Diagnostic {
                level: 1,
                is_blocking: true,
                path: "(root)".to_string(),
                message: format!("deserialization error: {e}"),
            }];
            if args.json_output {
                print_json(&diags, false);
            } else {
                print_human(&args.path, &diags, args.level, false);
            }
            std::process::exit(1);
        }
    };

    // Run remaining checks
    let diags = run_validation(artifact.as_ref().unwrap(), args.level);
    let passed = diags.iter().all(|d| !d.is_blocking);

    if args.json_output {
        print_json(&diags, passed);
    } else {
        let _ = l1_parse_ok;
        print_human(&args.path, &diags, args.level, l1_parse_ok);
    }

    std::process::exit(if passed { 0 } else { 1 });
}
