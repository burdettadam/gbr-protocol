/// `grimoire-sip-convert` — Convert GBR v0.2.0 scene cards to SIP narrative artifacts.
///
/// Usage:
///   grimoire-sip-convert --input <gbr-scene-card.json> [--registry <registry.json>] [--output <file.json>]
///
/// The converter applies the field mapping defined in `PROFILE.md §7`:
///
///   scene_id                          → artifact_id
///   observables.*                     → units[0].observables + entities[]
///   structure.canonical_summary.*     → units[0].structure.{steps, transition, grouping}
///   interpretations.*                 → units[0].interpretations
///   character_states[]                → units[0].participant_states[]
///   structure.turn                    → artifact.interpretations.value_charge
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

use gbr_types::sip::{
    artifact::{ArtifactInterpretations, SipArtifact, SipMetadata},
    entity::SipEntity,
    enums::{CausalRole, Significance},
    participant_state::{InformationItem, SipInformationState, SipParticipantState},
    transition::SipTransition,
    unit::{SipObservables, SipStep, SipStructure, SipUnit},
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
                    "Usage: grimoire-sip-convert --input <gbr.json> [--registry <reg.json>] [--output <out.json>]"
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

/// Optional JSON object mapping `entity_id → { display_name, entity_type }`.
struct Registry {
    inner: HashMap<String, Value>,
}

impl Registry {
    fn load(path: &PathBuf) -> Self {
        let text = std::fs::read_to_string(path).unwrap_or_else(|e| {
            eprintln!("ERROR: could not read registry '{}': {e}", path.display());
            std::process::exit(2);
        });
        let v: Value = serde_json::from_str(&text).unwrap_or_else(|e| {
            eprintln!("ERROR: could not parse registry '{}': {e}", path.display());
            std::process::exit(2);
        });
        let inner = v
            .as_object()
            .map(|m| m.iter().map(|(k, v)| (k.clone(), v.clone())).collect())
            .unwrap_or_default();
        Registry { inner }
    }

    fn empty() -> Self {
        Registry { inner: HashMap::new() }
    }

    fn display_name(&self, id: &str) -> String {
        self.inner
            .get(id)
            .and_then(|e| e.get("display_name"))
            .and_then(Value::as_str)
            .map(String::from)
            .unwrap_or_else(|| title_case(&id.replace('_', " ")))
    }

    fn entity_type(&self, id: &str, fallback: &str) -> String {
        self.inner
            .get(id)
            .and_then(|e| e.get("entity_type"))
            .and_then(Value::as_str)
            .map(String::from)
            .unwrap_or_else(|| fallback.to_string())
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
        "setup" => Some(CausalRole::Setup),
        "trigger" => Some(CausalRole::Trigger),
        "complication" => Some(CausalRole::Complication),
        // GBR "payoff" maps to SIP Resolution; "bridge" has no SIP core equivalent
        "payoff" | "resolution" => Some(CausalRole::Resolution),
        "bridge" => Some(CausalRole::Other),
        _ => None,
    }
}

// ── Conversion ───────────────────────────────────────────────────────────────

fn convert(gbr: &Value, reg: &Registry) -> Result<SipArtifact, String> {
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

    let metadata = Some(SipMetadata {
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

    let mut entities: Vec<SipEntity> = Vec::new();

    // Character entities from participants[]
    for p in &participants_raw {
        entities.push(SipEntity {
            entity_id: p.clone(),
            entity_type: reg.entity_type(p, "character"),
            display_name: reg.display_name(p),
            observable_descriptors: None,
            structural_properties: None,
            interpretations: None,
        });
    }

    // Location entity from setting_instance.setting
    if let Some(ref sid) = setting_id {
        if !entities.iter().any(|e| &e.entity_id == sid) {
            entities.push(SipEntity {
                entity_id: sid.clone(),
                entity_type: reg.entity_type(sid, "location"),
                display_name: reg.display_name(sid),
                observable_descriptors: None,
                structural_properties: None,
                interpretations: None,
            });
        }
    }

    // ── §7.3: unit observables ────────────────────────────────────────────
    let focalizer = str_field(obs, "focalizer").map(String::from);
    let diegetic_level = str_field(obs, "diegetic_level").map(String::from);
    let time_of_day = str_field(setting_instance, "time_of_day");
    let atmosphere = str_field(setting_instance, "atmosphere");
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
    if let Some(nt) = narrative_time {
        ctx.insert("narrative_time".into(), nt);
    }
    let context_val = if ctx.is_empty() { None } else { Some(Value::Object(ctx)) };

    let unit_observables = SipObservables {
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

    let steps: Vec<SipStep> = scene_turns
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
            SipStep {
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

    // §7.5: transition from delta
    let delta = str_field(canonical_summary, "delta");
    let transition = delta.map(|d| SipTransition {
        subject: Some(artifact_id.clone()),
        before: None,
        after: None,
        trigger: None,
        description: d.to_string(),
        confidence: None,
        grounding: None,
    });

    let unit_structure = SipStructure {
        position: None,
        causal_role: causal_role_raw.and_then(translate_causal_role),
        grouping: grouping_val,
        steps,
        transition,
        semantic_fingerprint: None,
    };

    // ── §7.6 + unit interpretations ───────────────────────────────────────
    let interp_gbr = gbr.get("interpretations").unwrap_or(&Value::Null);
    let craft = gbr.get("craft_targets").unwrap_or(&Value::Null);
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

    let participant_states: Vec<SipParticipantState> = char_states
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
                Some(SipInformationState { knows, gaps, gained: Vec::new() })
            };

            SipParticipantState {
                entity_ref,
                role_in_unit,
                pre_state: None,
                post_state: None,
                objective: None,
                obstacle: None,
                information_state: info_state,
                observables: None,
                structure: None,
                interpretations: cs_interp,
            }
        })
        .collect();

    // ── build unit ────────────────────────────────────────────────────────
    let unit = SipUnit {
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

    // ── §7.1: artifact-level interpretations (value_charge) ───────────────
    let turn = structure.get("turn").unwrap_or(&Value::Null);
    let turn_from = str_field(turn, "from");
    let turn_to = str_field(turn, "to");
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

    Ok(SipArtifact {
        protocol: "semantic-interaction-protocol".to_string(),
        protocol_version: "0.1.0".to_string(),
        profile: "narrative".to_string(),
        profile_version: "0.1.0".to_string(),
        artifact_id,
        metadata,
        entities,
        units: vec![unit],
        relationships: Vec::new(),
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
