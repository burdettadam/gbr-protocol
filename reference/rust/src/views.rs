//! Pre-computed views over a [`NarrativeCorpus`] or [`CapArtifact`].
//!
//! Views are derived, read-only projections that aggregate cross-artifact data.
//! They correspond to the four view types declared in `PROFILE.md §9`.
//!
//! # Usage
//!
//! ```rust,no_run
//! use cap_narrative_types::corpus::NarrativeCorpus;
//! use cap_narrative_types::views::{build_entity_trajectory_view, build_tension_curve_view, build_causal_chain_view};
//!
//! let corpus: NarrativeCorpus = unimplemented!();
//! let trajectory = build_entity_trajectory_view(&corpus, "nadia");
//! let tension    = build_tension_curve_view(&corpus);
//! let chain      = build_causal_chain_view(&corpus);
//! ```

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::corpus::NarrativeCorpus;

// ── View types ────────────────────────────────────────────────────────────────

/// A single data-point in an entity's arc across scenes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct EntityTrajectoryPoint {
    /// Artifact (scene) ID this point belongs to.
    pub artifact_id: String,
    /// Sequential index of this artifact in the corpus (0-based).
    pub corpus_index: usize,
    /// Emotional state at entry to this scene, if recorded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emotional_state: Option<String>,
    /// Objective action in this scene, if recorded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub objective: Option<String>,
    /// Whether the entity appears as focalizer in this scene.
    pub is_focalizer: bool,
    /// Participant interpretations (profile-defined, e.g. arc beat, wound).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interpretations: Option<Value>,
}

/// Projection of a single entity's arc across all corpus scenes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct EntityTrajectoryView {
    /// Profile type identifier.
    pub view_type: String,
    /// The entity slug this trajectory tracks.
    pub entity_id: String,
    /// One entry per artifact where the entity participates.
    pub points: Vec<EntityTrajectoryPoint>,
}

/// A single tension data-point in the story curve.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct TensionPoint {
    /// Artifact ID.
    pub artifact_id: String,
    /// Sequential index in corpus (0-based).
    pub corpus_index: usize,
    /// Numeric tension value 0.0 – 1.0.  Derived from `craft_targets.tension`
    /// if available; otherwise `None`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tension: Option<f64>,
    /// Beat label at this point (from `structure.grouping.beat`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beat: Option<String>,
    /// Value-charge transition at this artifact (e.g. `"negative→positive"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_shift: Option<String>,
}

/// The story's tension arc projected across all corpus artifacts.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct TensionCurveView {
    /// Profile type identifier.
    pub view_type: String,
    pub points: Vec<TensionPoint>,
}

/// A single node in the causal chain of a story.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CausalChainNode {
    /// Artifact ID.
    pub artifact_id: String,
    /// Sequential index in corpus (0-based).
    pub corpus_index: usize,
    /// Causal role at this node (e.g. `"setup"`, `"trigger"`, `"resolution"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub causal_role: Option<String>,
    /// Semantic fingerprint of the primary scene action.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    /// IDs of the corpus artifacts that this node logically follows from.
    /// Currently inferred as `[corpus_index - 1]` where causal_role is a
    /// follow-on role; left empty for setup/origin nodes.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub follows_from: Vec<String>,
}

/// The story's causal chain across all artifacts.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CausalChainView {
    /// Profile type identifier.
    pub view_type: String,
    pub nodes: Vec<CausalChainNode>,
}

// ── View builders ─────────────────────────────────────────────────────────────

/// Build an [`EntityTrajectoryView`] for `entity_id` across all corpus artifacts.
///
/// Scans every artifact's `participant_states` for entries that reference the
/// target entity, and records emotional state, objective, and focalizer flag.
pub fn build_entity_trajectory_view(
    corpus: &NarrativeCorpus,
    entity_id: &str,
) -> EntityTrajectoryView {
    let mut points = Vec::new();

    for (idx, artifact) in corpus.artifacts.iter().enumerate() {
        // Check if this entity participates in this artifact at all
        let participates = artifact.units.iter().any(|u| {
            u.observables.participants.iter().any(|p| p == entity_id)
        });
        if !participates {
            continue;
        }

        // Focalizer check
        let is_focalizer = artifact.units.iter().any(|u| {
            u.observables
                .context
                .as_ref()
                .and_then(|c| c.get("focalizer"))
                .and_then(|v| v.as_str())
                .map_or(false, |f| f == entity_id)
        });

        // Emotional state and objective from participant_states
        let mut emotional_state: Option<String> = None;
        let mut objective: Option<String> = None;
        let mut interp: Option<Value> = None;

        for unit in &artifact.units {
            for ps in &unit.participant_states {
                if ps.entity_ref != entity_id {
                    continue;
                }
                if emotional_state.is_none() {
                    emotional_state = ps
                        .pre_state
                        .as_ref()
                        .filter(|s| s.state_type == "emotional")
                        .and_then(|s| s.value.as_str())
                        .map(String::from);
                }
                if objective.is_none() {
                    objective = ps
                        .objective
                        .as_ref()
                        .map(|o| o.action.clone());
                }
                if interp.is_none() {
                    interp = ps.interpretations.clone();
                }
            }
        }

        points.push(EntityTrajectoryPoint {
            artifact_id: artifact.artifact_id.clone(),
            corpus_index: idx,
            emotional_state,
            objective,
            is_focalizer,
            interpretations: interp,
        });
    }

    EntityTrajectoryView {
        view_type: "entity_trajectory".to_string(),
        entity_id: entity_id.to_string(),
        points,
    }
}

/// Build a [`TensionCurveView`] across all corpus artifacts in reading order.
///
/// Tension value is extracted from `craft_targets.tension` if present.
/// Beat and value shift are extracted from `structure.grouping.beat` and
/// `artifact.interpretations.value_charge` respectively.
pub fn build_tension_curve_view(corpus: &NarrativeCorpus) -> TensionCurveView {
    let mut points = Vec::new();

    for (idx, artifact) in corpus.artifacts.iter().enumerate() {
        // Tension value from craft_targets
        let tension: Option<f64> = artifact.units.iter().find_map(|u| {
            u.craft_targets
                .as_ref()
                .and_then(|ct| ct.get("tension"))
                .and_then(|t| t.as_f64())
        });

        // Beat from structure.grouping
        let beat: Option<String> = artifact.units.iter().find_map(|u| {
            u.structure
                .as_ref()
                .and_then(|s| s.grouping.as_ref())
                .and_then(|g| g.get("beat"))
                .and_then(|v| v.as_str())
                .map(String::from)
        });

        // Value shift from artifact interpretations
        let value_shift: Option<String> = artifact
            .interpretations
            .as_ref()
            .and_then(|ai| match ai {
                crate::cap::artifact::ArtifactInterpretations::Object(v) => Some(v),
                _ => None,
            })
            .and_then(|v| v.get("value_charge"))
            .and_then(|vc| vc.get("turn"))
            .and_then(|t| t.as_str())
            .map(String::from);

        points.push(TensionPoint {
            artifact_id: artifact.artifact_id.clone(),
            corpus_index: idx,
            tension,
            beat,
            value_shift,
        });
    }

    TensionCurveView { view_type: "tension_curve".to_string(), points }
}

/// Build a [`CausalChainView`] across all corpus artifacts in reading order.
///
/// Each artifact becomes a node. The `causal_role` is read from the primary
/// unit's `structure.causal_role`. The `fingerprint` is the unit's
/// `semantic_fingerprint`. Causal edges are inferred sequentially: every
/// non-setup node is assumed to follow from the immediately preceding node
/// unless it is itself a `setup` or `trigger` (origin nodes).
pub fn build_causal_chain_view(corpus: &NarrativeCorpus) -> CausalChainView {
    // Roles that mark a scene as an "origin" (no predecessor edge)
    const ORIGIN_ROLES: &[&str] = &["setup"];

    let mut nodes: Vec<CausalChainNode> = corpus
        .artifacts
        .iter()
        .enumerate()
        .map(|(idx, artifact)| {
            let causal_role: Option<String> =
                artifact.units.iter().find_map(|u| {
                    u.structure.as_ref().and_then(|s| {
                        s.causal_role.as_ref().map(|cr| format!("{cr:?}").to_lowercase())
                    })
                });

            let fingerprint: Option<String> = artifact.units.iter().find_map(|u| {
                u.structure
                    .as_ref()
                    .and_then(|s| s.semantic_fingerprint.clone())
            });

            CausalChainNode {
                artifact_id: artifact.artifact_id.clone(),
                corpus_index: idx,
                causal_role,
                fingerprint,
                follows_from: Vec::new(), // filled below
            }
        })
        .collect();

    // Fill follows_from edges
    let artifact_ids: Vec<String> =
        corpus.artifacts.iter().map(|a| a.artifact_id.clone()).collect();
    for i in 1..nodes.len() {
        let is_origin = nodes[i]
            .causal_role
            .as_deref()
            .map_or(false, |r| ORIGIN_ROLES.contains(&r));
        if !is_origin {
            nodes[i].follows_from.push(artifact_ids[i - 1].clone());
        }
    }

    CausalChainView { view_type: "causal_chain".to_string(), nodes }
}
