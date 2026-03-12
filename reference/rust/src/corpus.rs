//! Multi-artifact corpus container for the GBR narrative profile.
//!
//! A [`NarrativeCorpus`] bundles multiple SIP narrative artifacts (one per
//! scene/chapter) together with shared entity definitions and cross-artifact
//! relationships, so that the full story can be processed without repeating
//! entity declarations in every artifact.
//!
//! # Design
//!
//! SIP is intentionally a single-artifact format; the corpus is a GBR-layer
//! concern only (see plan decision: "corpus in gbr-protocol only").
//!
//! ```rust,no_run
//! use cap_narrative_types::corpus::{NarrativeCorpus, CrossArtifactRelationship};
//! use cap_narrative_types::cap::CapArtifact;
//!
//! let mut corpus = NarrativeCorpus::new("threshold");
//! // corpus.add_artifact(scene_artifact);
//! // let entity = corpus.shared_entity("nadia");
//! ```

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::cap::{CapArtifact, CapEntity};

// ── Cross-artifact relationship ───────────────────────────────────────────────

/// A typed link between two units or entities in different artifacts.
///
/// Enables queries like "all scenes where this character arc beats occur"
/// or "all transitions involving this location".
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CrossArtifactRelationship {
    /// Source: `"<artifact_id>/<unit_id>"` or `"<artifact_id>/<entity_id>"`.
    pub from: String,
    /// Target: same format as `from`.
    pub to: String,
    /// Relationship type (e.g., `"continues"`, `"echoes"`, `"resolves"`).
    pub relationship_type: String,
    /// Optional free-text description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Profile-defined properties.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<Value>,
}

// ── Story architecture sub-types ──────────────────────────────────────────────

/// The inciting incident — the event that disrupts the story equilibrium.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, Default)]
pub struct IncitingIncident {
    /// Chapter in which the incident occurs (1-based).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chapter: Option<u32>,
    /// Structural type of the disruption (enum from `scene_structure.json`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incident_type: Option<String>,
    /// Brief human-readable description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Protagonist arc design at story level.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, Default)]
pub struct ProtagonistArc {
    /// Macro arc trajectory (e.g. `"positive_change"`, `"disillusionment"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arc_direction: Option<String>,
    /// Primary motivational system (`DriveModel` value).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drive_model: Option<String>,
    /// The false belief driving the protagonist's flaw.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lie_believed: Option<String>,
    /// The thematic truth the protagonist must accept to complete their arc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truth_needed: Option<String>,
    /// Slug referencing the protagonist's wound entity in the registry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wound_slug: Option<String>,
    /// Whether want and need are aligned or misaligned at story start.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub want_need_alignment: Option<String>,
}

/// Antagonist design at story level.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, Default)]
pub struct AntagonistDesign {
    /// Entity slug for the antagonist (may be an abstract force slug or `"self"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_slug: Option<String>,
    /// Antagonist typology (Truby taxonomy).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub antagonist_type: Option<String>,
    /// Does the antagonist undergo a change arc?
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arc_type: Option<String>,
    /// Structural depth of the antagonist's challenge (Truby opposition levels).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub opposition_level: Option<String>,
    /// Whether the antagonist mirrors the protagonist's want through opposite means.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thematic_mirror: Option<bool>,
}

/// A story-level thematic claim (McKee / Egri controlling idea).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct ThemeClaim {
    /// Theme label (e.g. `"grief"`, `"identity"`).
    pub theme: String,
    /// The controlling idea expressed by this theme.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controlling_idea: Option<String>,
}

/// A macro-arc beat positioned in the narrative sequence.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct BeatSequenceEntry {
    /// Beat label (e.g. `"status_quo"`, `"inciting_incident"`, `"climax"`).
    pub beat: String,
    /// Chapter number (1-based).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chapter: Option<u32>,
    /// Scene number within the chapter (1-based).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scene: Option<u32>,
    /// Brief description of what happens at this beat.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// A recurring motif tracked across the whole story.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct MotifEntry {
    /// Motif slug (e.g. `"the_key"`, `"silence"`).
    pub motif: String,
    /// Description of what the motif represents and where it appears.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

// ── Story architecture ─────────────────────────────────────────────────────────

/// Full typed narrative architecture for the whole story.
///
/// Maps from `story_architecture.json` — the GBR corpus-level document that
/// encodes genre contract, collision, protagonist/antagonist design, actantial
/// map, beat sequence, and motif vocabulary.
///
/// Per the GBR architecture doc, there is exactly one StoryArchitecture per corpus.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, Default)]
pub struct StoryArchitecture {
    /// Story title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    /// Primary genre (`horror`, `literary_fiction`, etc.).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genre: Option<String>,

    /// Secondary genre (genre hybridization).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genre_secondary: Option<String>,

    /// Macro-level story structure type (e.g. `"three_act"`, `"two_act"`, `"kishotenketsu"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub structure_type: Option<String>,

    /// Number of acts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub act_count: Option<u32>,

    /// Number of chapters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chapter_count: Option<u32>,

    /// The structural collision type (e.g. `"person_vs_self"`, `"person_vs_society"`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collision_type: Option<String>,

    /// The inciting incident.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inciting_incident: Option<IncitingIncident>,

    /// Protagonist arc design.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protagonist_arc: Option<ProtagonistArc>,

    /// Antagonist design.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub antagonist: Option<AntagonistDesign>,

    /// The controlling idea (Egri / McKee).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controlling_idea: Option<String>,

    /// The promise to the reader (genre contract).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genre_promise: Option<String>,

    /// Which social world has structural advantage.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub power_asymmetry: Option<String>,

    /// Story-level thematic claims.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub themes: Vec<ThemeClaim>,

    /// The macro-arc beat sequence — maps beat labels to scene positions.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub beat_sequence: Vec<BeatSequenceEntry>,

    /// Recurring motifs tracked across the whole story.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub motifs: Vec<MotifEntry>,

    /// Greimas actantial map (free-form; profile-defined structure).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actantial_map: Option<Value>,
}

// ── NarrativeCorpus ───────────────────────────────────────────────────────────

/// A collection of SIP narrative artifacts representing a complete story or book.
///
/// Entities that appear across multiple scenes are declared once in
/// `shared_entities`; individual artifacts may omit or repeat their entry.
/// Tools that consume a corpus should merge entity data from `shared_entities`
/// into each artifact's `entities` list when performing resolution.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct NarrativeCorpus {
    /// Machine-readable corpus identifier (e.g., book slug).
    pub corpus_id: String,

    /// Entities shared across the whole story (characters, major locations, …).
    ///
    /// These supplement — not replace — per-artifact entities. Resolvers should
    /// union the two sets with per-artifact data taking precedence.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub shared_entities: Vec<CapEntity>,

    /// All scene/chapter artifacts in story order.
    ///
    /// The order of entries in this list defines the canonical reading order.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub artifacts: Vec<CapArtifact>,

    /// Story-level structural metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub story_architecture: Option<StoryArchitecture>,

    /// Typed links between units or entities in different artifacts.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub cross_artifact_relationships: Vec<CrossArtifactRelationship>,
}

impl NarrativeCorpus {
    /// Create an empty corpus with the given identifier.
    pub fn new(corpus_id: impl Into<String>) -> Self {
        NarrativeCorpus {
            corpus_id: corpus_id.into(),
            shared_entities: Vec::new(),
            artifacts: Vec::new(),
            story_architecture: None,
            cross_artifact_relationships: Vec::new(),
        }
    }

    /// Append an artifact to the corpus (maintains reading order).
    pub fn add_artifact(&mut self, artifact: CapArtifact) {
        self.artifacts.push(artifact);
    }

    /// Declare a shared entity.  If an entity with the same `entity_id` already
    /// exists, the existing entry is replaced.
    pub fn declare_shared_entity(&mut self, entity: CapEntity) {
        if let Some(pos) = self
            .shared_entities
            .iter()
            .position(|e| e.entity_id == entity.entity_id)
        {
            self.shared_entities[pos] = entity;
        } else {
            self.shared_entities.push(entity);
        }
    }

    /// Look up a shared entity by ID.
    pub fn shared_entity(&self, entity_id: &str) -> Option<&CapEntity> {
        self.shared_entities
            .iter()
            .find(|e| e.entity_id == entity_id)
    }

    /// Return artifacts in corpus order (reading order).
    pub fn artifacts_in_order(&self) -> impl Iterator<Item = &CapArtifact> {
        self.artifacts.iter()
    }

    /// Return the total unit count across all artifacts.
    pub fn unit_count(&self) -> usize {
        self.artifacts.iter().map(|a| a.units.len()).sum()
    }

    /// Return all entity IDs referenced across all artifacts (deduplicated).
    pub fn all_entity_ids(&self) -> impl Iterator<Item = &str> {
        self.artifacts
            .iter()
            .flat_map(|a| a.entity_ids())
            .chain(self.shared_entities.iter().map(|e| e.entity_id.as_str()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_entity(id: &str, entity_type: &str) -> CapEntity {
        CapEntity {
            entity_id: id.to_owned(),
            entity_type: entity_type.to_owned(),
            display_name: id.to_owned(),
            observable_descriptors: None,
            structural_properties: None,
            interpretations: None,
        }
    }

    #[test]
    fn corpus_new_is_empty() {
        let corpus = NarrativeCorpus::new("my_book");
        assert_eq!(corpus.corpus_id, "my_book");
        assert!(corpus.artifacts.is_empty());
        assert!(corpus.shared_entities.is_empty());
    }

    #[test]
    fn declare_shared_entity_replaces_existing() {
        let mut corpus = NarrativeCorpus::new("book");
        corpus.declare_shared_entity(make_entity("nadia", "character"));
        corpus.declare_shared_entity(make_entity("nadia", "character")); // duplicate
        assert_eq!(corpus.shared_entities.len(), 1);
    }

    #[test]
    fn shared_entity_lookup() {
        let mut corpus = NarrativeCorpus::new("book");
        corpus.declare_shared_entity(make_entity("nadia", "character"));
        assert!(corpus.shared_entity("nadia").is_some());
        assert!(corpus.shared_entity("ghost").is_none());
    }

    #[test]
    fn unit_count_sums_across_artifacts() {
        let mut corpus = NarrativeCorpus::new("book");
        let raw1 = r#"{
            "protocol": "semantic-interaction-protocol",
            "protocol_version": "0.1.0",
            "profile": "narrative",
            "profile_version": "0.1.0",
            "artifact_id": "scene_1",
            "entities": [],
            "units": [
                {"unit_id": "u1", "artifact_id": "scene_1", "sequence_index": 1,
                 "observables": {"participants": []}},
                {"unit_id": "u2", "artifact_id": "scene_1", "sequence_index": 2,
                 "observables": {"participants": []}}
            ]
        }"#;
        let raw2 = r#"{
            "protocol": "semantic-interaction-protocol",
            "protocol_version": "0.1.0",
            "profile": "narrative",
            "profile_version": "0.1.0",
            "artifact_id": "scene_2",
            "entities": [],
            "units": [
                {"unit_id": "u1", "artifact_id": "scene_2", "sequence_index": 1,
                 "observables": {"participants": []}}
            ]
        }"#;
        corpus.add_artifact(serde_json::from_str(raw1).unwrap());
        corpus.add_artifact(serde_json::from_str(raw2).unwrap());
        assert_eq!(corpus.unit_count(), 3);
    }

    #[test]
    fn corpus_roundtrip() {
        let mut corpus = NarrativeCorpus::new("threshold");
        corpus.declare_shared_entity(make_entity("nadia", "character"));
        corpus.story_architecture = Some(StoryArchitecture {
            title: Some("Threshold".into()),
            genre: Some("literary_fiction".into()),
            controlling_idea: None,
            genre_promise: None,
            structure_type: None,
            ..Default::default()
        });
        let serialized = serde_json::to_string(&corpus).unwrap();
        let corpus2: NarrativeCorpus = serde_json::from_str(&serialized).unwrap();
        assert_eq!(corpus, corpus2);
    }
}
