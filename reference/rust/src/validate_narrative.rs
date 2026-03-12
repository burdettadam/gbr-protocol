//! Narrative profile validator — implements [`ProfileValidator`] for the GBR
//! narrative profile.
//!
//! Adds profile-specific L1, L2, and L3 checks on top of
//! `cap_types::validate_core()`.
//!
//! # Usage
//!
//! ```rust,no_run
//! use cap_narrative_types::cap::{CapArtifact, ConformanceLevel, validate_core};
//! use cap_narrative_types::validate_narrative::{NarrativeValidator, validate_corpus};
//! use cap_narrative_types::cap::ProfileValidator;
//!
//! let artifact: CapArtifact = unimplemented!();
//! let mut issues = validate_core(&artifact, ConformanceLevel::Referential);
//! issues.extend(NarrativeValidator.validate(&artifact, ConformanceLevel::Referential));
//! ```

use crate::corpus::NarrativeCorpus;
use crate::cap::{
    enums::{ConformanceLevel, ValidationSeverity},
    validate::{ProfileValidator, ValidationIssue},
    CapArtifact,
};

/// Stateless narrative profile validator.
///
/// Construct with `NarrativeValidator` (unit struct) and call
/// `ProfileValidator::validate()`.
pub struct NarrativeValidator;

impl ProfileValidator for NarrativeValidator {
    fn validate(&self, artifact: &CapArtifact, level: ConformanceLevel) -> Vec<ValidationIssue> {
        let mut issues = Vec::new();

        for (ui, unit) in artifact.units.iter().enumerate() {
            let upath = format!("units[{ui}]");

            // ── L1: focalizer and pov are required (PROFILE.md §8.1) ──────
            let context = unit
                .observables
                .context
                .as_ref()
                .and_then(|v| v.as_object());

            let focalizer = context
                .and_then(|c| c.get("focalizer"))
                .and_then(|v| v.as_str());
            let pov = context
                .and_then(|c| c.get("pov"))
                .and_then(|v| v.as_str());

            if focalizer.map_or(true, |s| s.is_empty()) {
                issues.push(ValidationIssue {
                    severity: ValidationSeverity::Error,
                    code: "NL1:FOCALIZER_MISSING".into(),
                    message: "narrative unit requires observables.context.focalizer".into(),
                    path: format!("{upath}.observables.context.focalizer"),
                });
            }

            if pov.map_or(true, |s| s.is_empty()) {
                issues.push(ValidationIssue {
                    severity: ValidationSeverity::Error,
                    code: "NL1:POV_MISSING".into(),
                    message: "narrative unit requires observables.context.pov".into(),
                    path: format!("{upath}.observables.context.pov"),
                });
            }

            // ── L2: focalizer resolves to a declared character entity ─────
            if level >= ConformanceLevel::Referential {
                if let Some(focal) = focalizer {
                    let resolves = artifact.entities.iter().any(|e| {
                        e.entity_id == focal && e.entity_type == "character"
                    });
                    if !resolves {
                        // Degrade to warning if entity exists but is the wrong type
                        let entity_exists = artifact.entities.iter().any(|e| e.entity_id == focal);
                        let sev = if entity_exists {
                            ValidationSeverity::Warning
                        } else {
                            ValidationSeverity::Error
                        };
                        issues.push(ValidationIssue {
                            severity: sev,
                            code: "NL2:FOCALIZER_NOT_CHARACTER".into(),
                            message: format!(
                                "focalizer {:?} does not resolve to a declared character entity",
                                focal
                            ),
                            path: format!("{upath}.observables.context.focalizer"),
                        });
                    }
                }

                // setting must resolve to a declared location entity
                if let Some(setting) = context.and_then(|c| c.get("setting")).and_then(|v| v.as_str()) {
                    let resolves = artifact.entities.iter().any(|e| {
                        e.entity_id == setting && e.entity_type == "location"
                    });
                    if !resolves {
                        let entity_exists = artifact.entities.iter().any(|e| e.entity_id == setting);
                        let sev = if entity_exists {
                            ValidationSeverity::Warning
                        } else {
                            ValidationSeverity::Error
                        };
                        issues.push(ValidationIssue {
                            severity: sev,
                            code: "NL2:SETTING_NOT_LOCATION".into(),
                            message: format!(
                                "setting {:?} does not resolve to a declared location entity",
                                setting
                            ),
                            path: format!("{upath}.observables.context.setting"),
                        });
                    }
                }
            }

            // ── L3: semantic checks ───────────────────────────────────────
            if level >= ConformanceLevel::RoundTrip {
                // At least one essential step
                if let Some(structure) = &unit.structure {
                    let has_essential = structure.steps.iter().any(|s| {
                        matches!(
                            s.significance,
                            Some(crate::cap::enums::Significance::Essential)
                        )
                    });
                    if !structure.steps.is_empty() && !has_essential {
                        issues.push(ValidationIssue {
                            severity: ValidationSeverity::Warning,
                            code: "NL3:NO_ESSENTIAL_STEP".into(),
                            message: "narrative unit should have at least one step with significance: essential".into(),
                            path: format!("{upath}.structure.steps"),
                        });
                    }

                    // Transition consistency: before and after should differ
                    if let Some(transition) = &structure.transition {
                        if let (Some(before), Some(after)) =
                            (&transition.before, &transition.after)
                        {
                            if before.value == after.value {
                                issues.push(ValidationIssue {
                                    severity: ValidationSeverity::Warning,
                                    code: "NL3:TRANSITION_UNCHANGED".into(),
                                    message: "transition.before and transition.after have the same value — no change recorded".into(),
                                    path: format!("{upath}.structure.transition"),
                                });
                            }
                        }
                    }

                    // Fingerprint agent should match at least one step agent
                    if let Some(fp) = &structure.semantic_fingerprint {
                        let fp_agent_upper = fp
                            .split_whitespace()
                            .next()
                            .unwrap_or("")
                            .to_lowercase();
                        if !fp_agent_upper.is_empty() {
                            let agent_match = structure.steps.iter().any(|s| {
                                s.agent.to_lowercase() == fp_agent_upper
                            });
                            if !agent_match && !structure.steps.is_empty() {
                                issues.push(ValidationIssue {
                                    severity: ValidationSeverity::Warning,
                                    code: "NL3:FINGERPRINT_AGENT_MISMATCH".into(),
                                    message: format!(
                                        "fingerprint agent {:?} does not match any step agent",
                                        fp_agent_upper
                                    ),
                                    path: format!("{upath}.structure.semantic_fingerprint"),
                                });
                            }
                        }
                    }
                }
            }
        }

        issues
    }

    fn validate_state_value(
        &self,
        state_type: &str,
        value: &serde_json::Value,
        path: &str,
    ) -> Vec<ValidationIssue> {
        let mut issues = Vec::new();

        match state_type {
            // emotional state must be a non-empty string
            "emotional" => {
                if !value.is_string() || value.as_str().map_or(true, |s| s.is_empty()) {
                    issues.push(ValidationIssue {
                        severity: ValidationSeverity::Error,
                        code: "NL1:EMOTIONAL_STATE_TYPE".into(),
                        message: "state_type 'emotional' requires a non-empty string value".into(),
                        path: path.to_owned(),
                    });
                }
            }
            // value_charge must be a non-empty string (e.g. "positive", "negative", "mixed")
            "value_charge" => {
                if !value.is_string() || value.as_str().map_or(true, |s| s.is_empty()) {
                    issues.push(ValidationIssue {
                        severity: ValidationSeverity::Error,
                        code: "NL1:VALUE_CHARGE_TYPE".into(),
                        message: "state_type 'value_charge' requires a non-empty string value".into(),
                        path: path.to_owned(),
                    });
                }
            }
            _ => {}
        }

        issues
    }
}

// ── Corpus-level validation ───────────────────────────────────────────────────

/// Validate a [`NarrativeCorpus`] for cross-artifact consistency.
///
/// Checks:
/// - **NCC1** — protagonist / antagonist entity slugs declared in
///   `story_architecture` resolve to entries in `shared_entities`.
/// - **NCC2** — consecutive artifacts share compatible participant states:
///   the `post_state` of scene N matches the `pre_state` of scene N+1 for
///   the same participant (warning only, since post_state is often missing).
///
/// Returns a list of [`ValidationIssue`] entries. An empty list means the
/// corpus passes all corpus-level checks at the requested `level`.
pub fn validate_corpus(
    corpus: &NarrativeCorpus,
    level: ConformanceLevel,
) -> Vec<ValidationIssue> {
    let mut issues = Vec::new();

    // ── NCC1: architecture slug resolution ───────────────────────────────
    if level >= ConformanceLevel::Referential {
        if let Some(arch) = &corpus.story_architecture {
            let known_slugs: std::collections::HashSet<&str> =
                corpus.shared_entities.iter().map(|e| e.entity_id.as_str()).collect();

            // Protagonist arc: wound_slug
            if let Some(arc) = &arch.protagonist_arc {
                if let Some(slug) = &arc.wound_slug {
                    if !known_slugs.contains(slug.as_str()) {
                        issues.push(ValidationIssue {
                            severity: ValidationSeverity::Warning,
                            code: "NCC1:WOUND_SLUG_UNRESOLVED".into(),
                            message: format!(
                                "story_architecture.protagonist_arc.wound_slug {:?} not found in shared_entities",
                                slug
                            ),
                            path: "story_architecture.protagonist_arc.wound_slug".into(),
                        });
                    }
                }
            }

            // Antagonist entity_slug
            if let Some(ant) = &arch.antagonist {
                if let Some(slug) = &ant.entity_slug {
                    // Abstract force slugs like "self" are allowed — only warn for non-trivial slugs
                    if slug != "self"
                        && slug != "society"
                        && !known_slugs.contains(slug.as_str())
                    {
                        issues.push(ValidationIssue {
                            severity: ValidationSeverity::Warning,
                            code: "NCC1:ANTAGONIST_SLUG_UNRESOLVED".into(),
                            message: format!(
                                "story_architecture.antagonist.entity_slug {:?} not found in shared_entities",
                                slug
                            ),
                            path: "story_architecture.antagonist.entity_slug".into(),
                        });
                    }
                }
            }
        }
    }

    // ── NCC2: consecutive participant-state continuity ───────────────────
    if level >= ConformanceLevel::RoundTrip {
        // Build per-entity post_state map for each artifact
        // post_state[artifact_idx][entity_id] = state_value string
        let state_snapshots: Vec<
            std::collections::HashMap<&str, &serde_json::Value>,
        > = corpus
            .artifacts
            .iter()
            .map(|artifact| {
                let mut map = std::collections::HashMap::new();
                for unit in &artifact.units {
                    for ps in &unit.participant_states {
                        if let Some(post) = &ps.post_state {
                            map.insert(ps.entity_ref.as_str(), &post.value);
                        }
                    }
                }
                map
            })
            .collect();

        for i in 1..corpus.artifacts.len() {
            let prev_post = &state_snapshots[i - 1];
            let curr_artifact = &corpus.artifacts[i];

            for unit in &curr_artifact.units {
                for ps in &unit.participant_states {
                    if let Some(pre) = &ps.pre_state {
                        if let Some(prev_val) = prev_post.get(ps.entity_ref.as_str()) {
                            if *prev_val != &pre.value {
                                issues.push(ValidationIssue {
                                    severity: ValidationSeverity::Warning,
                                    code: "NCC2:STATE_CONTINUITY_GAP".into(),
                                    message: format!(
                                        "participant '{}': post_state in '{}' ({}) does not match pre_state in '{}' ({})",
                                        ps.entity_ref,
                                        corpus.artifacts[i - 1].artifact_id,
                                        prev_val,
                                        curr_artifact.artifact_id,
                                        pre.value
                                    ),
                                    path: format!(
                                        "artifacts[{}].participant_states[entity={}].pre_state",
                                        i,
                                        ps.entity_ref
                                    ),
                                });
                            }
                        }
                    }
                }
            }
        }
    }

    issues
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cap::{validate_core, ConformanceLevel, CapArtifact};
    use serde_json::json;

    fn make_minimal_narrative(with_context: Option<serde_json::Value>) -> CapArtifact {
        let ctx = with_context.unwrap_or_else(|| {
            json!({ "focalizer": "nadia", "pov": "third_person_limited" })
        });
        let raw = json!({
            "protocol": "canonical-artifact-protocol",
            "protocol_version": "1.0.0",
            "profile": "narrative",
            "profile_version": "1.0.0",
            "artifact_id": "test_scene",
            "entities": [
                { "entity_id": "nadia", "entity_type": "character", "display_name": "Nadia" },
                { "entity_id": "house", "entity_type": "location", "display_name": "The House" }
            ],
            "units": [{
                "unit_id": "u1",
                "artifact_id": "test_scene",
                "sequence_index": 1,
                "observables": {
                    "participants": ["nadia"],
                    "context": ctx
                }
            }]
        });
        serde_json::from_value(raw).expect("test artifact parse failed")
    }

    #[test]
    fn valid_narrative_no_errors() {
        let artifact = make_minimal_narrative(None);
        let mut issues = validate_core(&artifact, ConformanceLevel::Referential);
        issues.extend(NarrativeValidator.validate(&artifact, ConformanceLevel::Referential));
        let errors: Vec<_> = issues
            .iter()
            .filter(|i| i.severity == ValidationSeverity::Error)
            .collect();
        assert!(errors.is_empty(), "unexpected errors: {errors:#?}");
    }

    #[test]
    fn missing_focalizer_detected() {
        let artifact = make_minimal_narrative(Some(json!({ "pov": "third_person_limited" })));
        let issues = NarrativeValidator.validate(&artifact, ConformanceLevel::Schema);
        assert!(
            issues.iter().any(|i| i.code == "NL1:FOCALIZER_MISSING"),
            "expected NL1:FOCALIZER_MISSING"
        );
    }

    #[test]
    fn missing_pov_detected() {
        let artifact = make_minimal_narrative(Some(json!({ "focalizer": "nadia" })));
        let issues = NarrativeValidator.validate(&artifact, ConformanceLevel::Schema);
        assert!(
            issues.iter().any(|i| i.code == "NL1:POV_MISSING"),
            "expected NL1:POV_MISSING"
        );
    }

    #[test]
    fn state_value_type_validation() {
        let issues = NarrativeValidator.validate_state_value(
            "emotional",
            &json!(123),
            "test.state",
        );
        assert!(
            issues.iter().any(|i| i.code == "NL1:EMOTIONAL_STATE_TYPE"),
            "expected NL1:EMOTIONAL_STATE_TYPE"
        );
    }
}
