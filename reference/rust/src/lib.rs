#![recursion_limit = "512"]
//! `gbr-types` — Protocol-core types for the Grimoire Book Representation (GBR) Protocol.
//!
//! This crate provides the stable, schema-level type system used by the GBR Protocol.
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
pub mod entities;
pub mod enums;
pub mod ontology;
pub mod tags;
pub mod voice;

// SIP core types — re-exported from the standalone `sip-types` crate.
// All downstream `gbr_types::sip::*` paths continue to work unchanged.
pub use sip_types as sip;

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
        "sip": {
            "SipArtifact": schema_for!(sip::SipArtifact),
            "SipEntity": schema_for!(sip::SipEntity),
            "SipUnit": schema_for!(sip::unit::SipUnit),
            "SipStep": schema_for!(sip::SipStep),
            "SipRelationship": schema_for!(sip::SipRelationship),
            "SipState": schema_for!(sip::SipState),
            "SipTransition": schema_for!(sip::SipTransition),
            "SipView": schema_for!(sip::SipView),
            "SipInterpretation": schema_for!(sip::SipInterpretation),
            "SipParticipantState": schema_for!(sip::SipParticipantState),
            "SipInformationState": schema_for!(sip::SipInformationState),
        },
    })
}
