#![recursion_limit = "512"]
//! `cap-narrative-types` — Protocol-core types for the Canonical Artifact Protocol — Narrative Profile.
//!
//! This crate provides the stable, schema-level type system used by the CAP Narrative Profile.
//! It is the Rust reference implementation of the types described in `SPECIFICATION.md`.
//!
//! # Module overview
//!
//! | Module | Contents |
//! |--------|----------|
//! | [`enums`] | All closed enumerations (alignment, archetype, wound, POV, …) |
//! | [`entities`] | Core declared entities: Character, Setting, Beat, Scene, … |
//! | [`catalogs`] | YAML catalog entry shapes |
//! | [`ontology`] | Canonical tag-key ontology |
//! | [`tags`] | Typed annotation system (`<!-- key:value -->` → [`tags::Annotation`]) |
//! | [`voice`] | VoiceContract, VoiceSignature, FocalizationConfig, TtsVoiceProfile |
//! | [`constraints`] | Formal tag constraint graph |
//!
//! # Consumers
//!
//! External tools that only need the protocol type system should depend on this crate.
//! Tools that also need the Grimoire authoring pipeline (gate checks, training, story
//! generation) should depend on `grimoire-tooling`, which re-exports this crate.

pub mod catalogs;
pub mod constraints;
pub mod corpus;
pub mod entities;
pub mod enums;
pub mod ontology;
pub mod overlay;
pub mod tags;
pub mod validate_narrative;
pub mod views;
pub mod voice;

// CAP core types — re-exported from the standalone `cap-types` crate.
// All downstream `cap_narrative_types::cap::*` paths work unchanged.
pub use cap_types as cap;

// ── Schema generation ─────────────────────────────────────────────────────────

/// Generate the complete JSON Schema for all public types.
pub fn generate_all_schemas() -> serde_json::Value {
    use schemars::schema_for;
    serde_json::json!({
        "entities": {
            "Character": schema_for!(entities::Character),
            "Setting": schema_for!(entities::Setting),
            "Beat": schema_for!(entities::Beat),
            "Scene": schema_for!(entities::Scene),
            "SceneSequence": schema_for!(entities::SceneSequence),
            "Chapter": schema_for!(entities::Chapter),
            "Motif": schema_for!(entities::Motif),
            "Symbol": schema_for!(entities::Symbol),
            "Leitmotif": schema_for!(entities::Leitmotif),
            "Thread": schema_for!(entities::Thread),
            "Promise": schema_for!(entities::Promise),
        },
        "voice": {
            "VoiceContract": schema_for!(voice::VoiceContract),
            "VoiceSignature": schema_for!(voice::VoiceSignature),
            "NarrativeVoice": schema_for!(voice::NarrativeVoice),
            "FocalizationConfig": schema_for!(voice::FocalizationConfig),
        },
        "cap": {
            "CapArtifact": schema_for!(cap::CapArtifact),
            "CapEntity": schema_for!(cap::CapEntity),
            "CapUnit": schema_for!(cap::unit::CapUnit),
            "CapStep": schema_for!(cap::CapStep),
            "CapRelationship": schema_for!(cap::CapRelationship),
            "CapState": schema_for!(cap::CapState),
            "CapTransition": schema_for!(cap::CapTransition),
            "CapView": schema_for!(cap::CapView),
            "CapInterpretation": schema_for!(cap::CapInterpretation),
            "CapParticipantState": schema_for!(cap::CapParticipantState),
            "CapInformationState": schema_for!(cap::CapInformationState),
        },
    })
}
