//! Top-level StoryRecipe — the complete pipeline output.
//!
//! Replaces the Python `StoryRecipe` dataclass.  All `extra: dict` fields
//! have been eliminated; every known attribute is an explicit typed field.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cap_narrative_types::entities::{Character, Relationship};
use cap_narrative_types::enums::Genre;

// ── Engine output types ───────────────────────────────────────────────────────

/// Selected plot type (output of Phase 01 concept_engine).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct PlotType {
    pub slug: String,
    pub name: String,
    pub genre: Genre,
    pub logline_template: Option<String>,
    pub required_collision_pattern: Option<String>,
    pub beat_labels: Vec<String>,
    pub description: Option<String>,
    pub reader_promise: Option<String>,
}

/// Selected trope (output of Phase 01 concept_engine).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Trope {
    pub slug: String,
    pub name: String,
    pub reader_promise: Option<String>,
    pub description: Option<String>,
    pub subversions: Vec<String>,
    pub cliche_risks: Vec<String>,
}

/// Selected collision pattern (output of Phase 02 collision_engine).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CollisionPattern {
    pub slug: String,
    pub name: String,
    pub description: Option<String>,
    pub cast_slots: Vec<String>,
    pub inherent_tension: Option<String>,
}

/// Selected inciting incident (output of Phase 02 collision_engine).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct IncitingIncident {
    pub slug: String,
    pub name: String,
    pub description: Option<String>,
}

/// A single structural beat in the generated beat sheet.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct GeneratedBeat {
    pub label: String,
    pub description: Option<String>,
    pub act: Option<cap_narrative_types::enums::Act>,
    pub order: u32,
}

// ── Story recipe ──────────────────────────────────────────────────────────────

/// Complete pipeline output from `grimoire-generate`.
///
/// Replaces the Python `StoryRecipe` dataclass.  The seed is stored so
/// the same recipe can be reproduced deterministically.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct StoryRecipe {
    /// RNG seed used for this generation
    pub seed: u64,
    pub genre: Genre,
    pub plot_type: PlotType,
    pub trope: Option<Trope>,
    pub collision: CollisionPattern,
    pub inciting_incident: IncitingIncident,
    pub cast: Vec<Character>,
    pub relationships: Vec<Relationship>,
    pub beats: Vec<GeneratedBeat>,
}
