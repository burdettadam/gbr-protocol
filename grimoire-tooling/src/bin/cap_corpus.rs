/// `grimoire-cap-corpus` — Build a [`NarrativeCorpus`] from a GBR book directory.
///
/// Usage:
///   grimoire-cap-corpus --directory <book-dir> [--output <corpus.json>]
///
/// Expected directory layout:
///
///   <book-dir>/
///     registry.json           — entity registry (characters, settings, relationships)
///     story_architecture.json — story-level structural + interpretive data
///     scenes/                 — GBR scene-card JSON files (processed in name order)
///       01-*.json
///       02-*.json
///       ...
///
/// All scene files in `scenes/` are converted (in sorted filename order) and
/// assembled into a `NarrativeCorpus`. Shared entities are built from the
/// registry and declared once on the corpus. The story architecture is parsed
/// and attached as `story_architecture`. Cross-artifact relationships are
/// derived from registry relationship data where both source and target entities
/// are present in the shared entity list.
///
/// Exit codes:
///   0   success
///   1   conversion error
///   2   usage / file-not-found error

use std::collections::HashMap;
use std::path::PathBuf;

use cap_narrative_types::{
    corpus::{
        AntagonistDesign, BeatSequenceEntry, IncitingIncident, MotifEntry, NarrativeCorpus,
        ProtagonistArc, StoryArchitecture, ThemeClaim,
    },
    cap::{
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
    },
};
use serde_json::{json, Value};

// ── CLI ────────────────────────────────────────────────────────────────────

struct Args {
    directory: PathBuf,
    output: Option<PathBuf>,
}

fn parse_args() -> Args {
    let raw: Vec<String> = std::env::args().collect();
    let mut directory: Option<PathBuf> = None;
    let mut output: Option<PathBuf> = None;

    let mut i = 1;
    while i < raw.len() {
        match raw[i].as_str() {
            "--directory" => {
                i += 1;
                if i < raw.len() {
                    directory = Some(PathBuf::from(&raw[i]));
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
                    "Usage: grimoire-cap-corpus --directory <book-dir> [--output <corpus.json>]"
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

    let directory = directory.unwrap_or_else(|| {
        eprintln!("ERROR: --directory <book-dir> is required");
        std::process::exit(2);
    });

    Args { directory, output }
}

// ── Registry ──────────────────────────────────────────────────────────────────
//
// Duplicated from sip_convert.rs — single-file binaries keep tooling self-contained.
// Any changes here should mirror those in sip_convert.rs.

struct Registry {
    root: Value,
}

fn flatten_interpreted_values(v: &Value) -> Value {
    match v {
        Value::Object(m) => {
            let mut out = serde_json::Map::new();
            for (k, val) in m {
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
        if self.char_entry(id).is_some() { return "character".to_string(); }
        if self.setting_entry(id).is_some() { return "location".to_string(); }
        fallback.to_string()
    }

    fn char_structural_props(&self, id: &str) -> Option<Value> {
        self.char_entry(id)?.get("structure").cloned()
    }

    fn char_interpretations(&self, id: &str) -> Option<Value> {
        let interp = self.char_entry(id)?.get("interpretations")?;
        Some(flatten_interpreted_values(interp))
    }

    fn setting_structural_props(&self, id: &str) -> Option<Value> {
        self.setting_entry(id)?.get("structure").cloned()
    }

    fn setting_interpretations(&self, id: &str) -> Option<Value> {
        self.setting_entry(id)?.get("interpretations").cloned()
    }

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

    /// Build a `CapEntity` for every declared character and setting.
    fn build_all_shared_entities(&self) -> Vec<CapEntity> {
        let mut entities: Vec<CapEntity> = Vec::new();

        if let Some(chars) = self.root.get("characters").and_then(Value::as_object) {
            for (id, _) in chars {
                let obs_desc = self
                    .char_entry(id)
                    .and_then(|e| e.get("observables"))
                    .and_then(|o| o.get("slot"))
                    .map(|slot| json!({ "slot": slot }));
                entities.push(CapEntity {
                    entity_id: id.clone(),
                    entity_type: "character".to_string(),
                    display_name: self.display_name(id),
                    observable_descriptors: obs_desc,
                    structural_properties: self.char_structural_props(id),
                    interpretations: self.char_interpretations(id),
                });
            }
        }

        if let Some(settings) = self.root.get("settings").and_then(Value::as_object) {
            for (id, _) in settings {
                entities.push(CapEntity {
                    entity_id: id.clone(),
                    entity_type: "location".to_string(),
                    display_name: self.display_name(id),
                    observable_descriptors: None,
                    structural_properties: self.setting_structural_props(id),
                    interpretations: self.setting_interpretations(id),
                });
            }
        }

        entities
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────────

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
        "payoff" | "resolution" => Some(CausalRole::Known(KnownCausalRole::Resolution)),
        "bridge" => Some(CausalRole::Custom("bridge".to_owned())),
        other => Some(CausalRole::Custom(other.to_owned())),
    }
}

// ── Scene converter ───────────────────────────────────────────────────────────

fn convert_scene(gbr: &Value, reg: &Registry) -> Result<CapArtifact, String> {
    let artifact_id = str_field(gbr, "scene_id")
        .ok_or("missing required field 'scene_id'")?
        .to_string();

    let book_id = str_field(gbr, "book_id").map(String::from);
    let chapter = gbr.get("chapter").and_then(Value::as_u64);

    let mut meta_extra: HashMap<String, Value> = HashMap::new();
    if let Some(b) = &book_id { meta_extra.insert("book_id".into(), json!(b)); }
    if let Some(c) = chapter  { meta_extra.insert("chapter".into(), json!(c)); }
    meta_extra.insert("source_format".into(), json!("GBR v0.2.0"));
    meta_extra.insert("scene_id".into(), json!(&artifact_id));

    let metadata = Some(CapMetadata {
        title: None, author: None, owner: None, size: None,
        extra: meta_extra,
    });

    let obs = gbr.get("observables").unwrap_or(&Value::Null);
    let participants_raw = obs.get("participants").map(as_str_vec).unwrap_or_default();
    let setting_instance = obs.get("setting_instance").unwrap_or(&Value::Null);
    let setting_id = str_field(setting_instance, "setting").map(String::from);

    let mut entities: Vec<CapEntity> = Vec::new();
    for p in &participants_raw {
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

    let focalizer    = str_field(obs, "focalizer").map(String::from);
    let diegetic_level = str_field(obs, "diegetic_level").map(String::from);
    let time_of_day  = str_field(setting_instance, "time_of_day");
    let atmosphere   = str_field(setting_instance, "atmosphere");
    let spatial_struct = str_field(setting_instance, "spatial_structure");
    let narrative_time = obs.get("narrative_time").cloned();

    let mut ctx = serde_json::Map::new();
    if let Some(f)  = &focalizer      { ctx.insert("focalizer".into(), json!(f)); }
    if let Some(s)  = &setting_id     { ctx.insert("setting".into(), json!(s)); }
    if let Some(d)  = &diegetic_level { ctx.insert("diegetic_level".into(), json!(d)); }
    if let Some(t)  = time_of_day     { ctx.insert("time_of_day".into(), json!(t)); }
    if let Some(a)  = atmosphere      { ctx.insert("atmosphere".into(), json!(a)); }
    if let Some(ss) = spatial_struct  { ctx.insert("spatial_structure".into(), json!(ss)); }
    if let Some(nt) = narrative_time  { ctx.insert("narrative_time".into(), nt); }
    let context_val = if ctx.is_empty() { None } else { Some(Value::Object(ctx)) };

    let unit_observables = CapObservables {
        participants: participants_raw.clone(),
        context: context_val,
        event_type: None,
        source_text: None,
    };

    let structure = gbr.get("structure").unwrap_or(&Value::Null);
    let causal_role_raw = str_field(structure, "causal_role");
    let canonical_summary = structure.get("canonical_summary").unwrap_or(&Value::Null);

    let beat = str_field(structure, "beat");
    let scene_function = str_field(structure, "scene_function");
    let scene_number = structure.get("scene_number_in_chapter").and_then(Value::as_u64);

    let mut grouping = serde_json::Map::new();
    if let Some(b)  = beat          { grouping.insert("beat".into(), json!(b)); }
    if let Some(sf) = scene_function{ grouping.insert("scene_function".into(), json!(sf)); }
    if let Some(sn) = scene_number  { grouping.insert("scene_number_in_chapter".into(), json!(sn)); }
    let grouping_val = if grouping.is_empty() { None } else { Some(Value::Object(grouping)) };

    let scene_turns = canonical_summary
        .get("scene_turns").and_then(Value::as_array).cloned().unwrap_or_default();

    let steps: Vec<CapStep> = scene_turns
        .iter()
        .enumerate()
        .map(|(i, t)| {
            let turn_obs = t.get("observables").unwrap_or(&Value::Null);
            let seq = turn_obs.get("turn_number").and_then(Value::as_u64)
                .unwrap_or((i + 1) as u64) as u32;
            CapStep {
                sequence_number: seq,
                agent:        str_field(turn_obs, "active_character").unwrap_or("unknown").to_string(),
                action:       str_field(turn_obs, "verb").unwrap_or("acts").to_string(),
                target:       str_field(turn_obs, "target").map(String::from),
                event_type:   str_field(turn_obs, "event_type").map(String::from),
                significance: str_field(turn_obs, "significance").map(translate_significance),
                interpretations: t.get("interpretations").cloned(),
            }
        })
        .collect();

    let delta   = str_field(canonical_summary, "delta");
    let turn    = structure.get("turn").unwrap_or(&Value::Null);
    let turn_from = str_field(turn, "from");
    let turn_to   = str_field(turn, "to");

    let make_state = |v: &str| CapState {
        subject: None,
        state_type: "value_charge".to_string(),
        value: json!(v),
        evidence: None, provenance: None, confidence: None,
    };
    let transition_before = turn_from.map(make_state);
    let transition_after  = turn_to.map(make_state);

    let transition = delta.map(|d| CapTransition {
        subject: Some(artifact_id.clone()),
        before: transition_before,
        after: transition_after,
        trigger: None,
        description: d.to_string(),
        confidence: None,
        grounding: None,
    });

    let interp_gbr = gbr.get("interpretations").unwrap_or(&Value::Null);
    let craft      = gbr.get("craft_targets").unwrap_or(&Value::Null);

    let pov_raw  = str_field(interp_gbr, "pov");
    let tone_raw = str_field(craft, "tone");
    let arc_type = str_field(interp_gbr, "arc_type");

    // Compact fingerprint: AGENT verb [TARGET] | ROLE=… | …
    let fingerprint = (|| {
        let kernel = steps.iter()
            .find(|s| s.significance == Some(Significance::Essential))
            .or_else(|| steps.first())?;
        let agent = kernel.agent.to_uppercase();
        let mut fp = match &kernel.target {
            Some(t) => format!("{} {} {}", agent, kernel.action, t.to_uppercase()),
            None => format!("{} {}", agent, kernel.action),
        };
        if let Some(r) = causal_role_raw { fp.push_str(&format!(" | ROLE={r}")); }
        if let (Some(f), Some(t)) = (turn_from, turn_to) { fp.push_str(&format!(" | SHIFT={f}\u{2192}{t}")); }
        if let Some(b) = beat       { fp.push_str(&format!(" | BEAT={b}")); }
        if let Some(p) = pov_raw    { fp.push_str(&format!(" | POV={p}")); }
        if let Some(t) = tone_raw   { fp.push_str(&format!(" | TONE={t}")); }
        if let Some(a) = arc_type   { fp.push_str(&format!(" | ARC={a}")); }
        Some(fp)
    })().or_else(|| {
        focalizer.as_deref().map(|f| format!("{} acts", f.to_uppercase()))
    });

    let unit_structure = CapStructure {
        position: None,
        causal_role: causal_role_raw.and_then(translate_causal_role),
        grouping: grouping_val,
        steps,
        transition,
        semantic_fingerprint: fingerprint,
    };

    let mut unit_interp_map = serde_json::Map::new();
    for key in &["pov","focalization","consciousness_mode","psychic_distance","narrator_reliability","stakes_domain"] {
        if let Some(v) = interp_gbr.get(*key) { unit_interp_map.insert(key.to_string(), v.clone()); }
    }
    if let Some(cm) = interp_gbr.get("canonical_metrics") { unit_interp_map.insert("canonical_metrics".into(), cm.clone()); }
    if let Some(mt) = gbr.get("motif_tags")               { unit_interp_map.insert("motif_tags".into(), mt.clone()); }
    if let Some(tn) = gbr.get("theory_notes").filter(|v| !v.is_null()) { unit_interp_map.insert("theory_notes".into(), tn.clone()); }

    let want_s    = str_field(canonical_summary, "want");
    let obstacle_s = str_field(canonical_summary, "obstacle");
    let outcome_s  = str_field(canonical_summary, "outcome");
    if want_s.is_some() || obstacle_s.is_some() || outcome_s.is_some() {
        let mut cs = serde_json::Map::new();
        if let Some(w) = want_s     { cs.insert("want".into(), json!(w)); }
        if let Some(o) = obstacle_s { cs.insert("obstacle".into(), json!(o)); }
        if let Some(o) = outcome_s  { cs.insert("outcome".into(), json!(o)); }
        unit_interp_map.insert("canonical_summary".into(), Value::Object(cs));
    }
    let unit_interp = if unit_interp_map.is_empty() { None } else { Some(Value::Object(unit_interp_map)) };

    let mut craft_map = serde_json::Map::new();
    if let Some(t)  = str_field(craft, "tone")        { craft_map.insert("tone".into(), json!(t)); }
    if let Some(tt) = craft.get("target_tension")     { craft_map.insert("tension".into(), tt.clone()); }
    if let Some(tp) = str_field(craft, "target_pacing"){ craft_map.insert("pacing".into(), json!(tp)); }
    // prose_directives: forward the entire block as opaque JSON into craft_targets
    if let Some(pd) = gbr.get("prose_directives") {
        if !pd.is_null() {
            craft_map.insert("prose_directives".into(), pd.clone());
        }
    }
    let unit_craft = if craft_map.is_empty() { None } else { Some(Value::Object(craft_map)) };

    let char_states = gbr.get("character_states").and_then(Value::as_array).cloned().unwrap_or_default();
    let participant_states: Vec<CapParticipantState> = char_states
        .iter()
        .map(|cs| {
            let cs_obs    = cs.get("observables").unwrap_or(&Value::Null);
            let cs_struct = cs.get("structure").unwrap_or(&Value::Null);
            let entity_ref = str_field(cs_obs, "character").unwrap_or("unknown").to_string();

            let knows: Vec<InformationItem> = cs_struct.get("knowledge_at_entry")
                .and_then(Value::as_array)
                .map(|a| a.iter().filter_map(|item| {
                    let about = str_field(item, "fact")?;
                    Some(InformationItem { subject: entity_ref.clone(), predicate: "knows_that".to_string(), about: about.to_string(), certainty: None })
                }).collect())
                .unwrap_or_default();

            let gaps: Vec<InformationItem> = cs_struct.get("knowledge_gaps")
                .and_then(Value::as_array)
                .map(|a| a.iter().filter_map(|item| {
                    let about = str_field(item, "fact")?;
                    Some(InformationItem { subject: entity_ref.clone(), predicate: "does_not_know".to_string(), about: about.to_string(), certainty: None })
                }).collect())
                .unwrap_or_default();

            let info_state = if knows.is_empty() && gaps.is_empty() { None }
                else { Some(CapInformationState { knows, gaps, gained: Vec::new() }) };

            let pre_state = cs.get("interpretations")
                .and_then(|i| i.get("emotion")).and_then(Value::as_str)
                .map(|e| CapState { subject: None, state_type: "emotional".to_string(), value: json!(e), evidence: None, provenance: None, confidence: None });

            let objective = cs_struct.get("objective").and_then(|obj| {
                let action = str_field(obj, "action")?;
                Some(Objective { action: action.to_string(), target: str_field(obj, "target").map(String::from) })
            });
            let obstacle = cs_struct.get("objective").and_then(|obj| str_field(obj, "obstacle")).map(String::from);
            let ps_observables = str_field(cs_struct, "tactic").map(|t| json!({ "tactic": t }));

            CapParticipantState {
                entity_ref,
                role_in_unit: str_field(cs_obs, "pov_role").map(String::from),
                pre_state, post_state: None,
                objective, obstacle,
                information_state: info_state,
                observables: ps_observables,
                structure: None,
                interpretations: cs.get("interpretations").cloned(),
            }
        })
        .collect();

    let unit = CapUnit {
        unit_id: format!("{}_scene", artifact_id),
        artifact_id: artifact_id.clone(),
        unit_type: Some("scene".to_string()),
        sequence_index: 1,
        observables: unit_observables,
        structure: Some(unit_structure),
        interpretations: unit_interp,
        craft_targets: unit_craft,
        participant_states,
        metadata: None,
    };

    let artifact_interp = match (turn_from, turn_to) {
        (Some(open), Some(close)) => Some(ArtifactInterpretations::Object(json!({
            "value_charge": { "open": open, "close": close, "turn": format!("{open}\u{2192}{close}") }
        }))),
        _ => None,
    };

    let scene_entity_ids: std::collections::HashSet<&str> =
        entities.iter().map(|e| e.entity_id.as_str()).collect();
    let relationships: Vec<CapRelationship> = reg
        .build_relationships()
        .into_iter()
        .filter(|(src, tgt, _, _)| {
            scene_entity_ids.contains(src.as_str()) && scene_entity_ids.contains(tgt.as_str())
        })
        .map(|(source, target, relationship_type, interpretations)| CapRelationship {
            source, target, relationship_type, evidence: None, interpretations,
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

// ── Story architecture parser ─────────────────────────────────────────────────

fn parse_story_architecture(path: &PathBuf) -> Option<StoryArchitecture> {
    let text = std::fs::read_to_string(path).ok()?;
    let v: Value = serde_json::from_str(&text).ok()?;

    let structure = v.get("structure").unwrap_or(&Value::Null);
    let interps   = v.get("interpretations").unwrap_or(&Value::Null);

    let title      = str_field(&v, "title").or_else(|| str_field(structure, "title")).map(String::from);
    let genre      = str_field(structure, "genre").map(String::from);
    let genre_sec  = str_field(structure, "genre_secondary").map(String::from);
    let struct_type = str_field(structure, "structure_type").or_else(|| str_field(structure, "macro_structure")).map(String::from);
    let act_count  = structure.get("act_count").and_then(Value::as_u64).map(|n| n as u32);
    let chap_count = structure.get("chapter_count").and_then(Value::as_u64).map(|n| n as u32);

    // collision_type nested under structure.collision_architecture or structure directly
    let collision_type = structure
        .get("collision_architecture")
        .and_then(|ca| str_field(ca, "collision_type"))
        .or_else(|| str_field(structure, "collision_type"))
        .map(String::from);

    // inciting_incident
    let inciting_incident = structure.get("inciting_incident").map(|ii| IncitingIncident {
        chapter:       ii.get("chapter").and_then(Value::as_u64).map(|n| n as u32),
        incident_type: str_field(ii, "incident_type").map(String::from),
        description:   str_field(ii, "description").map(String::from),
    });

    // protagonist_arc
    let protagonist_arc = interps.get("protagonist_arc").map(|pa| ProtagonistArc {
        arc_direction:      str_field(pa, "arc_direction").map(String::from),
        drive_model:        str_field(pa, "drive_model").map(String::from),
        lie_believed:       str_field(pa, "lie_believed").map(String::from),
        truth_needed:       str_field(pa, "truth_needed").map(String::from),
        wound_slug:         str_field(pa, "wound_slug").map(String::from),
        want_need_alignment: str_field(pa, "want_need_alignment").map(String::from),
    });

    // antagonist
    let antagonist = interps.get("antagonist").map(|a| AntagonistDesign {
        entity_slug:     str_field(a, "entity_slug").map(String::from),
        antagonist_type: str_field(a, "antagonist_type").map(String::from),
        arc_type:        str_field(a, "arc_type").map(String::from),
        opposition_level: str_field(a, "opposition_level").map(String::from),
        thematic_mirror:  a.get("thematic_mirror").and_then(Value::as_bool),
    });

    // controlling_idea at top level or in interpretations
    let controlling_idea = str_field(&v, "controlling_idea")
        .or_else(|| str_field(interps, "controlling_idea"))
        .or_else(|| {
            interps.get("themes").and_then(Value::as_array).and_then(|t| {
                t.first().and_then(|th| str_field(th, "controlling_idea"))
            })
        })
        .map(String::from);

    let genre_promise = str_field(structure, "genre_promise")
        .or_else(|| str_field(&v, "genre_promise"))
        .map(String::from);

    let power_asymmetry = str_field(interps, "power_asymmetry")
        .or_else(|| str_field(structure, "power_asymmetry"))
        .map(String::from);

    // themes
    let themes: Vec<ThemeClaim> = interps.get("themes")
        .and_then(Value::as_array)
        .map(|arr| arr.iter().filter_map(|t| {
            let theme = str_field(t, "theme")?;
            Some(ThemeClaim { theme: theme.to_string(), controlling_idea: str_field(t, "controlling_idea").map(String::from) })
        }).collect())
        .unwrap_or_default();

    // beat_sequence
    let beat_sequence: Vec<BeatSequenceEntry> = v.get("beat_sequence")
        .and_then(Value::as_array)
        .map(|arr| arr.iter().filter_map(|b| {
            let beat = str_field(b, "beat")?;
            Some(BeatSequenceEntry {
                beat: beat.to_string(),
                chapter:     b.get("chapter").and_then(Value::as_u64).map(|n| n as u32),
                scene:       b.get("scene").and_then(Value::as_u64).map(|n| n as u32),
                description: str_field(b, "description").map(String::from),
            })
        }).collect())
        .unwrap_or_default();

    // motifs
    let motifs: Vec<MotifEntry> = v.get("motifs")
        .and_then(Value::as_array)
        .map(|arr| arr.iter().filter_map(|m| {
            let motif = str_field(m, "motif")?;
            Some(MotifEntry { motif: motif.to_string(), description: str_field(m, "description").map(String::from) })
        }).collect())
        .unwrap_or_default();

    let actantial_map = v.get("actantial_map").cloned()
        .or_else(|| interps.get("actantial_map").cloned());

    Some(StoryArchitecture {
        title,
        genre,
        genre_secondary: genre_sec,
        structure_type: struct_type,
        act_count,
        chapter_count: chap_count,
        collision_type,
        inciting_incident,
        protagonist_arc,
        antagonist,
        controlling_idea,
        genre_promise,
        power_asymmetry,
        themes,
        beat_sequence,
        motifs,
        actantial_map,
    })
}

// ── main ──────────────────────────────────────────────────────────────────────

fn main() {
    let args = parse_args();
    let dir = &args.directory;

    if !dir.is_dir() {
        eprintln!("ERROR: '{}' is not a directory", dir.display());
        std::process::exit(2);
    }

    // Derive corpus_id from directory basename
    let corpus_id = dir
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("corpus")
        .to_string();

    // Load registry (optional)
    let reg_path = dir.join("registry.json");
    let reg = if reg_path.exists() {
        eprintln!("Loading registry: {}", reg_path.display());
        Registry::load(&reg_path)
    } else {
        eprintln!("WARN: no registry.json found at '{}', entity data will be stubs", reg_path.display());
        Registry::empty()
    };

    // Load story architecture (optional) — accepts both naming conventions
    let arch_path = ["story_architecture.json", "story-architecture.json"]
        .iter()
        .map(|name| dir.join(name))
        .find(|p| p.exists());
    let story_architecture = match &arch_path {
        Some(p) => {
            eprintln!("Loading story architecture: {}", p.display());
            parse_story_architecture(p)
        }
        None => None,
    };

    // Build shared entities from registry
    let shared_entities = reg.build_all_shared_entities();
    eprintln!("Shared entities: {} declared", shared_entities.len());

    // Collect scene files from scenes/ subdirectory
    let scenes_dir = dir.join("scenes");
    let scene_files: Vec<PathBuf> = if scenes_dir.is_dir() {
        match std::fs::read_dir(&scenes_dir) {
            Ok(rd) => {
                let mut files: Vec<PathBuf> = rd
                    .filter_map(|e| e.ok())
                    .map(|e| e.path())
                    .filter(|p| p.extension().map_or(false, |ext| ext == "json"))
                    .collect();
                files.sort();
                files
            }
            Err(e) => {
                eprintln!("ERROR: could not read scenes dir '{}': {e}", scenes_dir.display());
                std::process::exit(2);
            }
        }
    } else {
        // Fall back to JSON files directly in the book directory (excluding known non-scene files)
        let skip_names = ["registry.json", "story_architecture.json", "story-architecture.json"];
        match std::fs::read_dir(dir) {
            Ok(rd) => {
                let mut files: Vec<PathBuf> = rd
                    .filter_map(|e| e.ok())
                    .map(|e| e.path())
                    .filter(|p| {
                        let name = p.file_name().and_then(|n| n.to_str()).unwrap_or("");
                        // Exclude already-converted .sip.json outputs and known non-scene files
                        p.extension().map_or(false, |ext| ext == "json")
                            && !name.ends_with(".sip.json")
                            && !skip_names.contains(&name)
                    })
                    .collect();
                files.sort();
                files
            }
            Err(e) => {
                eprintln!("ERROR: could not read directory '{}': {e}", dir.display());
                std::process::exit(2);
            }
        }
    };

    eprintln!("Found {} scene file(s)", scene_files.len());

    // Convert scenes
    let mut corpus = NarrativeCorpus::new(&corpus_id);
    corpus.story_architecture = story_architecture;
    for entity in shared_entities {
        corpus.declare_shared_entity(entity);
    }

    for path in &scene_files {
        let text = std::fs::read_to_string(path).unwrap_or_else(|e| {
            eprintln!("ERROR: could not read '{}': {e}", path.display());
            std::process::exit(2);
        });
        let gbr: Value = serde_json::from_str(&text).unwrap_or_else(|e| {
            eprintln!("ERROR: '{}' is not valid JSON: {e}", path.display());
            std::process::exit(2);
        });
        let artifact = convert_scene(&gbr, &reg).unwrap_or_else(|e| {
            eprintln!("ERROR: conversion failed for '{}': {e}", path.display());
            std::process::exit(1);
        });
        eprintln!("  converted: {}", artifact.artifact_id);
        corpus.add_artifact(artifact);
    }

    eprintln!(
        "Corpus '{}': {} artifacts, {} shared entities",
        corpus.corpus_id,
        corpus.artifacts.len(),
        corpus.shared_entities.len()
    );

    // Serialize
    let out = serde_json::to_string_pretty(&corpus).unwrap_or_else(|e| {
        eprintln!("ERROR: serialization failed: {e}");
        std::process::exit(1);
    });

    match &args.output {
        Some(path) => {
            std::fs::write(path, &out).unwrap_or_else(|e| {
                eprintln!("ERROR: could not write '{}': {e}", path.display());
                std::process::exit(1);
            });
            eprintln!("Wrote corpus to {}", path.display());
        }
        None => println!("{out}"),
    }
}
