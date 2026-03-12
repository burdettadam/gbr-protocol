/// `grimoire-cap-convert` — Convert GBR v0.2.0 scene cards to SIP narrative artifacts.
///
/// Usage:
///   grimoire-cap-convert --input <gbr-scene-card.json> [--registry <registry.json>] [--output <file.json>]
///
/// The converter applies the field mapping defined in `PROFILE.md §7`:
///
///   scene_id                          → artifact_id
///   observables.*                     → units[0].observables + entities[]
///   structure.canonical_summary.*     → units[0].structure.{steps, transition, grouping}
///   interpretations.*                 → units[0].interpretations
///   character_states[]                → units[0].participant_states[]
///   structure.turn                    → units[0].structure.transition.{before,after} + artifact.interpretations.value_charge
///
/// Significance mapping (PROFILE.md §8.4):
///   "kernel"    → "essential"
///   "satellite" → "supplementary"
///
/// Exit codes:
///   0   success
///   1   conversion error (malformed GBR input)
///   2   usage error (bad arguments, file not found)

use std::collections::HashMap;
use std::path::PathBuf;

use cap_narrative_types::cap::{
    artifact::{ArtifactInterpretations, CapArtifact, CapMetadata},
    entity::CapEntity,
    enums::{CausalRole, KnownCausalRole, Significance},
    participant_state::{
        InformationItem, Objective, CapInformationState, CapParticipantState,
    },
    relationship::CapRelationship,
    state::CapState,
    transition::CapTransition,
    unit::{CapObservables, CapStep, CapStructure, CapUnit},
};
use serde_json::{json, Value};

// ── CLI ────────────────────────────────────────────────────────────────────

struct Args {
    input: PathBuf,
    registry: Option<PathBuf>,
    output: Option<PathBuf>,
}

fn parse_args() -> Args {
    let raw: Vec<String> = std::env::args().collect();
    let mut input: Option<PathBuf> = None;
    let mut registry: Option<PathBuf> = None;
    let mut output: Option<PathBuf> = None;

    let mut i = 1;
    while i < raw.len() {
        match raw[i].as_str() {
            "--input" => {
                i += 1;
                if i < raw.len() {
                    input = Some(PathBuf::from(&raw[i]));
                }
            }
            "--registry" => {
                i += 1;
                if i < raw.len() {
                    registry = Some(PathBuf::from(&raw[i]));
                }
            }
            "--output" => {
                i += 1;
                if i < raw.len() {
                    output = Some(PathBuf::from(&raw[i]));
                }
            }
            "--help" | "-h" => {
                eprintln!(
                    "Usage: grimoire-cap-convert --input <gbr.json> [--registry <reg.json>] [--output <out.json>]"
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

    let input = input.unwrap_or_else(|| {
        eprintln!("ERROR: --input <file.json> is required");
        std::process::exit(2);
    });

    Args { input, registry, output }
}

// ── Registry ────────────────────────────────────────────────────────────────

/// Parses a GBR `registry.json` and provides typed accessors for entity and
/// relationship data.
///
/// GBR registry format:
/// ```json
/// { "book_id": "…", "characters": { "<slug>": { "observables": …, "structure": …, "interpretations": … } },
///   "settings": { "<slug>": { … } }, "relationships": [ { "observables": …, "structure": …, "interpretations": … } ] }
/// ```
struct Registry {
    root: Value,
}

/// Strip `{ "value": x, "confidence": … }` InterpretedValue wrappers from an
/// object, collapsing each wrapped field to its plain value.
fn flatten_interpreted_values(v: &Value) -> Value {
    match v {
        Value::Object(m) => {
            let mut out = serde_json::Map::new();
            for (k, val) in m {
                // If val is { "value": x, … }, extract x
                if let Some(inner) = val.as_object() {
                    if let Some(plain) = inner.get("value") {
                        out.insert(k.clone(), plain.clone());
                        continue;
                    }
                }
                out.insert(k.clone(), flatten_interpreted_values(val));
            }
            Value::Object(out)
        }
        _ => v.clone(),
    }
}

impl Registry {
    fn load(path: &PathBuf) -> Self {
        let text = std::fs::read_to_string(path).unwrap_or_else(|e| {
            eprintln!("ERROR: could not read registry '{}': {e}", path.display());
            std::process::exit(2);
        });
        let root: Value = serde_json::from_str(&text).unwrap_or_else(|e| {
            eprintln!("ERROR: could not parse registry '{}': {e}", path.display());
            std::process::exit(2);
        });
        Registry { root }
    }

    fn empty() -> Self {
        Registry { root: Value::Object(Default::default()) }
    }

    fn char_entry(&self, id: &str) -> Option<&Value> {
        self.root.get("characters")?.get(id)
    }

    fn setting_entry(&self, id: &str) -> Option<&Value> {
        self.root.get("settings")?.get(id)
    }

    fn display_name(&self, id: &str) -> String {
        let name = self
            .char_entry(id)
            .and_then(|e| e.get("observables"))
            .and_then(|o| o.get("name"))
            .and_then(Value::as_str)
            .or_else(|| {
                self.setting_entry(id)
                    .and_then(|e| e.get("observables"))
                    .and_then(|o| o.get("name"))
                    .and_then(Value::as_str)
            });
        name.map(String::from).unwrap_or_else(|| title_case(&id.replace('_', " ")))
    }

    fn entity_type(&self, id: &str, fallback: &str) -> String {
        if self.char_entry(id).is_some() {
            return "character".to_string();
        }
        if self.setting_entry(id).is_some() {
            return "location".to_string();
        }
        fallback.to_string()
    }

    /// Structural properties for a character (role, voice_signature).
    fn char_structural_props(&self, id: &str) -> Option<Value> {
        self.char_entry(id)?.get("structure").cloned()
    }

    /// Interpretations for a character, flattening any InterpretedValue wrappers.
    fn char_interpretations(&self, id: &str) -> Option<Value> {
        let interp = self.char_entry(id)?.get("interpretations")?;
        Some(flatten_interpreted_values(interp))
    }

    /// Structural properties for a setting (setting_type).
    fn setting_structural_props(&self, id: &str) -> Option<Value> {
        self.setting_entry(id)?.get("structure").cloned()
    }

    /// Interpretations for a setting.
    fn setting_interpretations(&self, id: &str) -> Option<Value> {
        self.setting_entry(id)?.get("interpretations").cloned()
    }

    /// All relationships as `(source, target, rel_type, interpretations)` tuples.
    fn build_relationships(&self) -> Vec<(String, String, String, Option<Value>)> {
        let rels = match self.root.get("relationships").and_then(Value::as_array) {
            Some(a) => a,
            None => return Vec::new(),
        };
        rels.iter()
            .filter_map(|r| {
                let obs = r.get("observables")?;
                let source = obs.get("source")?.as_str()?.to_string();
                let target = obs.get("target")?.as_str()?.to_string();
                let rel_type = r
                    .get("structure")
                    .and_then(|s| s.get("rel_type"))
                    .and_then(Value::as_str)
                    .unwrap_or("related_to")
                    .to_string();
                let interp = r.get("interpretations").cloned();
                Some((source, target, rel_type, interp))
            })
            .collect()
    }
}

// ── Helpers ──────────────────────────────────────────────────────────────────

fn str_field<'a>(v: &'a Value, key: &str) -> Option<&'a str> {
    v.get(key).and_then(Value::as_str)
}

fn as_str_vec(v: &Value) -> Vec<String> {
    v.as_array()
        .map(|a| a.iter().filter_map(Value::as_str).map(String::from).collect())
        .unwrap_or_default()
}

fn title_case(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

fn translate_significance(raw: &str) -> Significance {
    match raw {
        "kernel" | "essential" => Significance::Essential,
        _ => Significance::Supplementary,
    }
}

fn translate_causal_role(raw: &str) -> Option<CausalRole> {
    match raw {
        "setup" => Some(CausalRole::Known(KnownCausalRole::Setup)),
        "trigger" => Some(CausalRole::Known(KnownCausalRole::Trigger)),
        "complication" => Some(CausalRole::Known(KnownCausalRole::Complication)),
        // GBR "payoff" maps to SIP Resolution; "bridge" is a GBR-specific extension
        "payoff" | "resolution" => Some(CausalRole::Known(KnownCausalRole::Resolution)),
        "bridge" => Some(CausalRole::Custom("bridge".to_owned())),
        other => Some(CausalRole::Custom(other.to_owned())),
    }
}

// ── Semantic Fingerprint (PROFILE.md §6) ─────────────────────────────────────

/// Build a semantic fingerprint string per PROFILE.md §6.1–§6.3:
///
///   AGENT verb [TARGET] | ROLE=<causal_role> | SHIFT=<from>→<to> | BEAT=<beat>
///   [| POV=<pov>] [| TONE=<tone>] [| ARC=<arc_type>]
///
/// Returns `None` if no essential step is found and no fallback agent exists.
#[allow(clippy::too_many_arguments)]
fn build_semantic_fingerprint(
    focalizer: &Option<String>,
    steps: &[CapStep],
    causal_role: Option<&str>,
    turn_from: Option<&str>,
    turn_to: Option<&str>,
    beat: Option<&str>,
    pov: Option<&str>,
    tone: Option<&str>,
    arc_type: Option<&str>,
) -> Option<String> {
    // §6.2 rule 2: first essential step, else first step
    let kernel_step = steps
        .iter()
        .find(|s| s.significance == Some(Significance::Essential))
        .or_else(|| steps.first());

    let (agent_slug, verb, target) = match kernel_step {
        Some(s) => (s.agent.clone(), s.action.clone(), s.target.clone()),
        None => {
            // Use focalizer as fallback agent
            let agent = focalizer.as_deref()?.to_string();
            (agent, "acts".to_string(), None)
        }
    };

    let agent_upper = agent_slug.to_uppercase();
    let mut fp = match &target {
        Some(t) => format!("{} {} {}", agent_upper, verb, t.to_uppercase()),
        None => format!("{} {}", agent_upper, verb),
    };

    // Qualifiers (ordered: ROLE, SHIFT, BEAT, POV, TONE, ARC per §6.1)
    if let Some(role) = causal_role {
        fp.push_str(&format!(" | ROLE={role}"));
    }
    if let (Some(from), Some(to)) = (turn_from, turn_to) {
        fp.push_str(&format!(" | SHIFT={from}\u{2192}{to}"));
    }
    if let Some(b) = beat {
        fp.push_str(&format!(" | BEAT={b}"));
    }
    if let Some(p) = pov {
        fp.push_str(&format!(" | POV={p}"));
    }
    if let Some(t) = tone {
        fp.push_str(&format!(" | TONE={t}"));
    }
    if let Some(a) = arc_type {
        fp.push_str(&format!(" | ARC={a}"));
    }

    Some(fp)
}

// ── Conversion ───────────────────────────────────────────────────────────────

fn convert(gbr: &Value, reg: &Registry) -> Result<CapArtifact, String> {
    // ── §7.1: artifact-level fields ───────────────────────────────────────
    let artifact_id = str_field(gbr, "scene_id")
        .ok_or("missing required field 'scene_id'")?
        .to_string();

    let book_id = str_field(gbr, "book_id").map(String::from);
    let chapter = gbr.get("chapter").and_then(Value::as_u64);

    let mut meta_extra: HashMap<String, Value> = HashMap::new();
    if let Some(b) = &book_id {
        meta_extra.insert("book_id".into(), json!(b));
    }
    if let Some(c) = chapter {
        meta_extra.insert("chapter".into(), json!(c));
    }
    meta_extra.insert("source_format".into(), json!("GBR v0.2.0"));
    meta_extra.insert("scene_id".into(), json!(&artifact_id));

    let metadata = Some(CapMetadata {
        title: None,
        author: None,
        owner: None,
        size: None,
        extra: meta_extra,
    });

    // ── §7.2: entity construction ─────────────────────────────────────────
    let obs = gbr.get("observables").unwrap_or(&Value::Null);
    let participants_raw = obs
        .get("participants")
        .map(as_str_vec)
        .unwrap_or_default();
    let setting_instance = obs.get("setting_instance").unwrap_or(&Value::Null);
    let setting_id = str_field(setting_instance, "setting").map(String::from);

    let mut entities: Vec<CapEntity> = Vec::new();

    // Character entities from participants[]
    for p in &participants_raw {
        // Gather the slot field (if present) as observable_descriptors
        let obs_desc = reg
            .char_entry(p)
            .and_then(|e| e.get("observables"))
            .and_then(|o| o.get("slot"))
            .map(|slot| json!({ "slot": slot }));
        entities.push(CapEntity {
            entity_id: p.clone(),
            entity_type: reg.entity_type(p, "character"),
            display_name: reg.display_name(p),
            observable_descriptors: obs_desc,
            structural_properties: reg.char_structural_props(p),
            interpretations: reg.char_interpretations(p),
        });
    }

    // Location entity from setting_instance.setting
    if let Some(ref sid) = setting_id {
        if !entities.iter().any(|e| &e.entity_id == sid) {
            entities.push(CapEntity {
                entity_id: sid.clone(),
                entity_type: reg.entity_type(sid, "location"),
                display_name: reg.display_name(sid),
                observable_descriptors: None,
                structural_properties: reg.setting_structural_props(sid),
                interpretations: reg.setting_interpretations(sid),
            });
        }
    }

    // ── §7.3: unit observables ────────────────────────────────────────────
    let focalizer = str_field(obs, "focalizer").map(String::from);
    let diegetic_level = str_field(obs, "diegetic_level").map(String::from);
    let time_of_day = str_field(setting_instance, "time_of_day");
    let atmosphere = str_field(setting_instance, "atmosphere");
    let spatial_structure = str_field(setting_instance, "spatial_structure");
    let narrative_time = obs.get("narrative_time").cloned();

    let mut ctx = serde_json::Map::new();
    if let Some(f) = &focalizer {
        ctx.insert("focalizer".into(), json!(f));
    }
    if let Some(s) = &setting_id {
        ctx.insert("setting".into(), json!(s));
    }
    if let Some(d) = &diegetic_level {
        ctx.insert("diegetic_level".into(), json!(d));
    }
    if let Some(tod) = time_of_day {
        ctx.insert("time_of_day".into(), json!(tod));
    }
    if let Some(atm) = atmosphere {
        ctx.insert("atmosphere".into(), json!(atm));
    }
    if let Some(ss) = spatial_structure {
        ctx.insert("spatial_structure".into(), json!(ss));
    }
    if let Some(nt) = narrative_time {
        ctx.insert("narrative_time".into(), nt);
    }
    let context_val = if ctx.is_empty() { None } else { Some(Value::Object(ctx)) };

    let unit_observables = CapObservables {
        participants: participants_raw.clone(),
        context: context_val,
        event_type: None,
        source_text: None,
    };

    // ── §7.4: unit structure ──────────────────────────────────────────────
    let structure = gbr.get("structure").unwrap_or(&Value::Null);
    let causal_role_raw = str_field(structure, "causal_role");
    let canonical_summary = structure.get("canonical_summary").unwrap_or(&Value::Null);

    // §7.5: grouping
    let beat = str_field(structure, "beat");
    let scene_function = str_field(structure, "scene_function");
    let scene_number = structure
        .get("scene_number_in_chapter")
        .and_then(Value::as_u64);

    let mut grouping = serde_json::Map::new();
    if let Some(b) = beat {
        grouping.insert("beat".into(), json!(b));
    }
    if let Some(sf) = scene_function {
        grouping.insert("scene_function".into(), json!(sf));
    }
    if let Some(sn) = scene_number {
        grouping.insert("scene_number_in_chapter".into(), json!(sn));
    }
    let grouping_val = if grouping.is_empty() { None } else { Some(Value::Object(grouping)) };

    // §7.5: scene_turns → steps (significance kernel→essential, satellite→supplementary)
    let scene_turns = canonical_summary
        .get("scene_turns")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();

    let steps: Vec<CapStep> = scene_turns
        .iter()
        .enumerate()
        .map(|(i, t)| {
            let turn_obs = t.get("observables").unwrap_or(&Value::Null);
            let turn_interp = t.get("interpretations").cloned();
            let seq = turn_obs
                .get("turn_number")
                .and_then(Value::as_u64)
                .unwrap_or((i + 1) as u64) as u32;
            let agent = str_field(turn_obs, "active_character")
                .unwrap_or("unknown")
                .to_string();
            let action = str_field(turn_obs, "verb")
                .unwrap_or("acts")
                .to_string();
            let target = str_field(turn_obs, "target").map(String::from);
            let event_type = str_field(turn_obs, "event_type").map(String::from);
            let significance =
                str_field(turn_obs, "significance").map(translate_significance);
            CapStep {
                sequence_number: seq,
                agent,
                action,
                target,
                event_type,
                significance,
                interpretations: turn_interp,
            }
        })
        .collect();

    // §7.4: transition from delta + structure.turn → before/after (value_charge)
    let delta = str_field(canonical_summary, "delta");
    let turn = structure.get("turn").unwrap_or(&Value::Null);
    let turn_from = str_field(turn, "from");
    let turn_to = str_field(turn, "to");

    let transition_before = turn_from.map(|v| CapState {
        subject: None,
        state_type: "value_charge".to_string(),
        value: json!(v),
        evidence: None,
        provenance: None,
        confidence: None,
    });
    let transition_after = turn_to.map(|v| CapState {
        subject: None,
        state_type: "value_charge".to_string(),
        value: json!(v),
        evidence: None,
        provenance: None,
        confidence: None,
    });

    let transition = delta.map(|d| CapTransition {
        subject: Some(artifact_id.clone()),
        before: transition_before,
        after: transition_after,
        trigger: None,
        description: d.to_string(),
        confidence: None,
        grounding: None,
    });

    // ── §6: semantic fingerprint (PROFILE.md §6.1–§6.3) ──────────────────
    // (interp_gbr and craft are also used below for unit interpretations)
    let interp_gbr = gbr.get("interpretations").unwrap_or(&Value::Null);
    let craft = gbr.get("craft_targets").unwrap_or(&Value::Null);

    let pov_raw = str_field(interp_gbr, "pov");
    let tone_raw = str_field(craft, "tone");
    let arc_type_raw = str_field(interp_gbr, "arc_type");

    let fingerprint = build_semantic_fingerprint(
        &focalizer,
        &steps,
        causal_role_raw,
        turn_from,
        turn_to,
        beat,
        pov_raw,
        tone_raw,
        arc_type_raw,
    );

    let unit_structure = CapStructure {
        position: None,
        causal_role: causal_role_raw.and_then(translate_causal_role),
        grouping: grouping_val,
        steps,
        transition,
        semantic_fingerprint: fingerprint,
    };

    // ── §7.6 + unit interpretations ───────────────────────────────────────
    // (interp_gbr, craft already bound above for fingerprint)
    let motif_tags = gbr.get("motif_tags").cloned();

    let mut unit_interp_map = serde_json::Map::new();
    for key in &[
        "pov",
        "focalization",
        "consciousness_mode",
        "psychic_distance",
        "narrator_reliability",
        "stakes_domain",
    ] {
        if let Some(v) = interp_gbr.get(*key) {
            unit_interp_map.insert(key.to_string(), v.clone());
        }
    }
    if let Some(cm) = interp_gbr.get("canonical_metrics") {
        unit_interp_map.insert("canonical_metrics".into(), cm.clone());
    }
    if let Some(mt) = motif_tags {
        unit_interp_map.insert("motif_tags".into(), mt);
    }
    // canonical_summary (want/obstacle/outcome)
    let want = str_field(canonical_summary, "want");
    let obstacle = str_field(canonical_summary, "obstacle");
    let outcome = str_field(canonical_summary, "outcome");
    if want.is_some() || obstacle.is_some() || outcome.is_some() {
        let mut cs = serde_json::Map::new();
        if let Some(w) = want {
            cs.insert("want".into(), json!(w));
        }
        if let Some(o) = obstacle {
            cs.insert("obstacle".into(), json!(o));
        }
        if let Some(o) = outcome {
            cs.insert("outcome".into(), json!(o));
        }
        unit_interp_map.insert("canonical_summary".into(), Value::Object(cs));
    }
    // §7.7: theory_notes → interpretations.theory_notes (PROFILE.md §7.7)
    if let Some(tn) = gbr.get("theory_notes") {
        if !tn.is_null() {
            unit_interp_map.insert("theory_notes".into(), tn.clone());
        }
    }
    let unit_interp = if unit_interp_map.is_empty() {
        None
    } else {
        Some(Value::Object(unit_interp_map))
    };

    // ── §4: craft_targets — fourth epistemic section (prescriptive authorial intent) ──
    let mut craft_targets_map = serde_json::Map::new();
    if let Some(tone) = str_field(craft, "tone") {
        craft_targets_map.insert("tone".into(), json!(tone));
    }
    if let Some(tt) = craft.get("target_tension") {
        craft_targets_map.insert("tension".into(), tt.clone());
    }
    if let Some(tp) = str_field(craft, "target_pacing") {
        craft_targets_map.insert("pacing".into(), json!(tp));
    }
    let unit_craft_targets = if craft_targets_map.is_empty() {
        None
    } else {
        Some(Value::Object(craft_targets_map))
    };

    // ── §7.6: character_states → participant_states ───────────────────────
    let char_states = gbr
        .get("character_states")
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();

    let participant_states: Vec<CapParticipantState> = char_states
        .iter()
        .map(|cs| {
            let cs_obs = cs.get("observables").unwrap_or(&Value::Null);
            let cs_struct = cs.get("structure").unwrap_or(&Value::Null);
            let cs_interp = cs.get("interpretations").cloned();

            let entity_ref = str_field(cs_obs, "character")
                .unwrap_or("unknown")
                .to_string();
            let role_in_unit = str_field(cs_obs, "pov_role").map(String::from);

            // knowledge_at_entry → information_state.knows
            let knows: Vec<InformationItem> = cs_struct
                .get("knowledge_at_entry")
                .and_then(Value::as_array)
                .map(|arr| {
                    arr.iter()
                        .filter_map(|item| {
                            let about = str_field(item, "fact")?;
                            Some(InformationItem {
                                subject: entity_ref.clone(),
                                predicate: "knows_that".to_string(),
                                about: about.to_string(),
                                certainty: None,
                            })
                        })
                        .collect()
                })
                .unwrap_or_default();

            // knowledge_gaps → information_state.gaps
            let gaps: Vec<InformationItem> = cs_struct
                .get("knowledge_gaps")
                .and_then(Value::as_array)
                .map(|arr| {
                    arr.iter()
                        .filter_map(|item| {
                            let about = str_field(item, "fact")?;
                            Some(InformationItem {
                                subject: entity_ref.clone(),
                                predicate: "does_not_know".to_string(),
                                about: about.to_string(),
                                certainty: None,
                            })
                        })
                        .collect()
                })
                .unwrap_or_default();

            let info_state = if knows.is_empty() && gaps.is_empty() {
                None
            } else {
                Some(CapInformationState { knows, gaps, gained: Vec::new() })
            };

            // §7.6: emotion → pre_state (state_type: emotional)
            let pre_state = cs.get("interpretations")
                .and_then(|i| i.get("emotion"))
                .and_then(Value::as_str)
                .map(|e| CapState {
                    subject: None,
                    state_type: "emotional".to_string(),
                    value: json!(e),
                    evidence: None,
                    provenance: None,
                    confidence: None,
                });

            // §7.6: structure.objective → objective (Objective struct)
            let objective = cs_struct.get("objective").and_then(|obj| {
                let action = str_field(obj, "action")?;
                Some(Objective {
                    action: action.to_string(),
                    target: str_field(obj, "target").map(String::from),
                })
            });

            // §7.6: structure.objective.obstacle → obstacle string
            let obstacle = cs_struct
                .get("objective")
                .and_then(|obj| str_field(obj, "obstacle"))
                .map(String::from);

            // §7.6: structure.tactic → observables.tactic
            let ps_observables = str_field(cs_struct, "tactic").map(|t| json!({ "tactic": t }));

            CapParticipantState {
                entity_ref,
                role_in_unit,
                pre_state,
                post_state: None,
                objective,
                obstacle,
                information_state: info_state,
                observables: ps_observables,
                structure: None,
                interpretations: cs_interp,
            }
        })
        .collect();

    // ── build unit ────────────────────────────────────────────────────────
    let unit = CapUnit {
        unit_id: format!("{}_scene", artifact_id),
        artifact_id: artifact_id.clone(),
        unit_type: Some("scene".to_string()),
        sequence_index: 1,
        observables: unit_observables,
        structure: Some(unit_structure),
        interpretations: unit_interp,
        craft_targets: unit_craft_targets,
        participant_states,
        metadata: None,
    };

    // ── artifact-level interpretations (value_charge summary) ──────────────
    let artifact_interp = match (turn_from, turn_to) {
        (Some(open), Some(close)) => {
            let turn_str = format!("{open}\u{2192}{close}");
            Some(ArtifactInterpretations::Object(json!({
                "value_charge": {
                    "open": open,
                    "close": close,
                    "turn": turn_str
                }
            })))
        }
        _ => None,
    };

    // ── build relationships from registry ────────────────────────────────
    // Include registry relationships that connect entities present in this scene.
    let scene_entity_ids: std::collections::HashSet<&str> =
        entities.iter().map(|e| e.entity_id.as_str()).collect();
    let relationships: Vec<CapRelationship> = reg
        .build_relationships()
        .into_iter()
        .filter(|(src, tgt, _, _)| {
            scene_entity_ids.contains(src.as_str()) && scene_entity_ids.contains(tgt.as_str())
        })
        .map(|(source, target, relationship_type, interpretations)| CapRelationship {
            source,
            target,
            relationship_type,
            evidence: None,
            interpretations,
        })
        .collect();

    Ok(CapArtifact {
        protocol: "semantic-interaction-protocol".to_string(),
        protocol_version: "0.1.0".to_string(),
        profile: "narrative".to_string(),
        profile_version: "0.1.0".to_string(),
        artifact_id,
        metadata,
        entities,
        units: vec![unit],
        relationships,
        views: Vec::new(),
        interpretations: artifact_interp,
    })
}

// ── main ─────────────────────────────────────────────────────────────────────

fn main() {
    let args = parse_args();

    // Load GBR input
    let text = std::fs::read_to_string(&args.input).unwrap_or_else(|e| {
        eprintln!("ERROR: could not read '{}': {e}", args.input.display());
        std::process::exit(2);
    });
    let gbr: Value = serde_json::from_str(&text).unwrap_or_else(|e| {
        eprintln!("ERROR: '{}' is not valid JSON: {e}", args.input.display());
        std::process::exit(2);
    });

    // Load optional registry
    let reg = match &args.registry {
        Some(p) => Registry::load(p),
        None => Registry::empty(),
    };

    // Convert
    let artifact = convert(&gbr, &reg).unwrap_or_else(|e| {
        eprintln!("ERROR: conversion failed: {e}");
        std::process::exit(1);
    });

    // Serialize
    let out = serde_json::to_string_pretty(&artifact).unwrap_or_else(|e| {
        eprintln!("ERROR: serialization failed: {e}");
        std::process::exit(1);
    });

    // Write output
    match &args.output {
        Some(path) => {
            std::fs::write(path, &out).unwrap_or_else(|e| {
                eprintln!("ERROR: could not write '{}': {e}", path.display());
                std::process::exit(1);
            });
            eprintln!("Wrote {}", path.display());
        }
        None => println!("{out}"),
    }
}
