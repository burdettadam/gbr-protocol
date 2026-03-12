#![recursion_limit = "512"]
//! `grimoire-tooling` — Grimoire authoring-system tooling built on the CAP Narrative Profile.
//!
//! This crate provides the Grimoire writing system's authoring tools on top of
//! the stable `cap-narrative-types` protocol layer. It adds: gate system, sub-phase DAG,
//! LLM training pipeline, story recipe, and optional PyO3 Python bindings.
//!
//! # Module overview
//!
//! | Module | Contents |
//! |--------|----------|
//! | [`training`] | SceneContext, ProsePassage, TrainingExample for LLM fine-tuning |
//! | [`gates`] | Gate system structs (PhaseSpec, GateSpec, GateResult, …) |
//! | [`dag`] | Sub-phase dependency DAG (Kahn topological sort) |
//! | [`recipe`] | StoryRecipe — top-level pipeline output |
//!
//! # Re-exports from cap-narrative-types
//!
//! All protocol-core modules are re-exported for convenience:
//! `catalogs`, `constraints`, `entities`, `enums`, `ontology`, `tags`, `voice`.
//!
//! # Feature flags
//!
//! - `python` — enables PyO3 bindings; build with `maturin build --features python`

// ── Protocol-core re-exports ─────────────────────────────────────────────────

pub use cap_narrative_types::catalogs;
pub use cap_narrative_types::constraints;
pub use cap_narrative_types::entities;
pub use cap_narrative_types::enums;
pub use cap_narrative_types::ontology;
pub use cap_narrative_types::tags;
pub use cap_narrative_types::voice;

// ── Tooling modules ───────────────────────────────────────────────────────────

pub mod dag;
pub mod gates;
pub mod recipe;
pub mod training;

// ── Python bindings (opt-in) ──────────────────────────────────────────────────

#[cfg(feature = "python")]
mod python;

#[cfg(feature = "python")]
use pyo3::prelude::*;

#[cfg(feature = "python")]
#[pymodule]
fn grimoire_tooling(m: &Bound<'_, PyModule>) -> PyResult<()> {
    python::register(m)
}

// ── Schema generation ─────────────────────────────────────────────────────────

/// Generate the complete JSON Schema for all public types (protocol-core + tooling).
pub fn generate_all_schemas() -> serde_json::Value {
    use schemars::schema_for;
    let mut schemas = cap_narrative_types::generate_all_schemas();
    let obj = schemas.as_object_mut().unwrap();
    obj.insert("training".to_string(), serde_json::json!({
        "SceneContext": schema_for!(training::SceneContext),
        "TrainingExample": schema_for!(training::TrainingExample),
        "ProsePassage": schema_for!(training::ProsePassage),
        "TrainingDataset": schema_for!(training::TrainingDataset),
        "TierConfig": schema_for!(training::TierConfig),
        "ProseIntent": schema_for!(training::ProseIntent),
        "NarrativeFunction": schema_for!(training::NarrativeFunction),
        "Paragraph": schema_for!(training::Paragraph),
        "Sentence": schema_for!(training::Sentence),
    }));
    schemas
}
