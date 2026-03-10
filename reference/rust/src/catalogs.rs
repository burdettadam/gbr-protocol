//! Typed YAML catalog entry shapes.
//!
//! Each YAML file under `*/references/` has an implicit schema.  Previously
//! these were loaded as `list[dict]` with no validation.  These structs make
//! every field explicit; unknown keys become deserialization errors.
//!
//! Catalog entries are loaded at startup into a `CatalogSet`, which is then
//! used to validate entity refs and populate `EntityRegistry`.
//!
//! **Design note**: The YAML catalog files use a `slug` field as the primary
//! identifier.  The slug encodes the enum variant (e.g. `"hero"` → `Archetype::Hero`).
//! Typed helpers like `.archetype()` parse the slug on demand rather than
//! requiring a redundant typed field in the YAML.

use std::collections::HashMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

use crate::enums::{Alignment, Archetype, DriveModel, Genre, Role, Wound};

// ── Archetype catalog (character-archetypes.yaml) ─────────────────────────────

/// Matches the structure of `03-characters/references/character-archetypes.yaml`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct ArchetypeCatalogEntry {
    pub slug: String,
    pub name: String,
    #[serde(default)] pub family: Option<String>,
    #[serde(default)] pub structural_function: Option<String>,
    #[serde(default)] pub core_desire: Option<String>,
    #[serde(default)] pub core_fear: Option<String>,
    #[serde(default)] pub story_role: Option<String>,
    #[serde(default)] pub typical_arc_pattern: Option<String>,
    #[serde(default)] pub shadow_form: Option<String>,
    #[serde(default)] pub strengths: Vec<String>,
    #[serde(default)] pub weaknesses: Vec<String>,
    #[serde(default)] pub common_surface_expressions: Vec<String>,
    #[serde(default)] pub romance_affinity: Vec<String>,
}

impl ArchetypeCatalogEntry {
    /// Parse the slug into a typed `Archetype` variant.
    pub fn archetype(&self) -> Option<Archetype> {
        self.slug.parse().ok()
    }
}

// ── Wound catalog (character-wounds.yaml) ─────────────────────────────────────

/// Matches the structure of `03-characters/references/character-wounds.yaml`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct WoundCatalogEntry {
    pub slug: String,
    pub name: String,
    #[serde(default)] pub category: Option<String>,
    #[serde(default)] pub description: Option<String>,
    #[serde(default)] pub lie_template: Option<String>,
    #[serde(default)] pub need_template: Option<String>,
    #[serde(default)] pub want_template: Option<String>,
    #[serde(default)] pub behavioral_patterns: Vec<String>,
    #[serde(default)] pub ghost_event_examples: Vec<String>,
    #[serde(default)] pub archetype_affinity: Vec<String>,
    #[serde(default)] pub alignment_tendency: Vec<String>,
    #[serde(default)] pub genre_affinity: Vec<String>,
    #[serde(default)] pub romance_tension: Option<String>,
}

impl WoundCatalogEntry {
    /// Parse the slug into a typed `Wound` variant.
    pub fn wound(&self) -> Option<Wound> {
        self.slug.parse().ok()
    }
}

// ── Alignment catalog (alignment-system.yaml) ─────────────────────────────────

/// Deprecated flat stub — the actual YAML uses `AlignmentSystemCatalog`.
/// Retained only for `CatalogSet.alignments` backward compat; prefer loading
/// via `AlignmentSystemCatalog` directly.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct AlignmentCatalogEntry {
    pub slug: String,
    /// The normalized enum value
    pub alignment: Alignment,
    /// Human-readable label (e.g. "Rule-bound Prosocial" for lawful_good)
    pub label: String,
    #[serde(default)] pub description: Option<String>,
    #[serde(default)] pub motivations: Vec<String>,
    #[serde(default)] pub shadow: Option<String>,
}

// ── Character role catalog (character-roles.yaml) ─────────────────────────────

/// Matches the structure of `03-characters/references/character-roles.yaml`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RoleCatalogEntry {
    pub slug: String,
    pub name: String,
    #[serde(default)] pub structural_function: Option<String>,
    #[serde(default)] pub max_per_story: Option<u32>,
    #[serde(default)] pub min_per_story: Option<u32>,
    #[serde(default)] pub required: Option<bool>,
    #[serde(default)] pub relationship_to_plot: Option<String>,
    #[serde(default)] pub typical_archetype_affinity: Vec<String>,
    #[serde(default)] pub notes: Option<String>,
}

impl RoleCatalogEntry {
    /// Parse the slug into a typed `Role` variant.
    pub fn role(&self) -> Option<Role> {
        self.slug.parse().ok()
    }
}

// ── Drive model catalog (character-drives.yaml) ───────────────────────────────

/// Matches the structure of `03-characters/references/character-drives.yaml`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct DriveCatalogEntry {
    pub slug: String,
    pub name: String,
    #[serde(default)] pub source_theory: Vec<String>,
    #[serde(default)] pub summary: Option<String>,
    #[serde(default)] pub fields: Vec<String>,
    #[serde(default)] pub arc_template: Option<String>,
    #[serde(default)] pub genre_affinity: Vec<String>,
    #[serde(default)] pub limitations: Option<String>,
    #[serde(default)] pub canonical_examples: Vec<String>,
}

impl DriveCatalogEntry {
    /// Parse the slug into a typed `DriveModel` variant.
    pub fn drive_model(&self) -> Option<DriveModel> {
        self.slug.parse().ok()
    }
}

// ── Plot type catalog (plot-types.yaml) ───────────────────────────────────────

/// Matches the structure of `01-concept/references/plot-types.yaml`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct PlotTypeCatalogEntry {
    pub slug: String,
    pub name: String,
    pub genre: Genre,
    #[serde(default)] pub logline_template: Option<String>,
    #[serde(default)] pub core_conflict_type: Option<String>,
    #[serde(default)] pub recommended_structures: Vec<String>,
    #[serde(default)] pub required_collision_pattern: Option<String>,
    #[serde(default)] pub min_cast_size: Option<u32>,
    #[serde(default)] pub max_cast_size: Option<u32>,
    /// `~` / null is valid in non-romance genres that don't use heat levels.
    #[serde(default)] pub heat_level_range: Option<Vec<JsonValue>>,
    #[serde(default)] pub description: Option<String>,
    #[serde(default)] pub reader_promise: Option<String>,
}

// ── Collision pattern catalog (circle-collision-patterns.yaml) ────────────────

/// A single cast-slot declaration inside a collision pattern.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CastSlot {
    pub slot: String,
    #[serde(default)] pub circle: Option<String>,
    #[serde(default)] pub membership: Option<String>,
    #[serde(default)] pub archetype_constraint: Vec<String>,
    #[serde(default)] pub role: Option<String>,
    #[serde(default)] pub min: Option<u32>,
    #[serde(default)] pub max: Option<u32>,
}

/// Matches the structure of `02-collision/references/circle-collision-patterns.yaml`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CollisionPatternCatalogEntry {
    pub slug: String,
    pub name: String,
    #[serde(default)] pub circle_a_type_constraint: Vec<String>,
    #[serde(default)] pub circle_b_type_constraint: Vec<String>,
    #[serde(default)] pub collision_type: Option<String>,
    #[serde(default)] pub power_asymmetry: Option<String>,
    #[serde(default)] pub required_cast_slots: Vec<CastSlot>,
    #[serde(default)] pub description: Option<String>,
    #[serde(default)] pub arc_polarity: Option<JsonValue>,
}

// ── Inciting incident catalog (inciting-incidents.yaml) ───────────────────────
//
// NOTE: This YAML file uses a top-level `categories:` mapping rather than a
// bare sequence.  Use `IncitingIncidentsCatalog` to load the whole file.

/// One incident entry nested inside a category.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct IncitingIncidentEntry {
    pub slug: String,
    pub name: String,
    #[serde(default)] pub description: Option<String>,
    #[serde(default)] pub collision_affinity: Vec<String>,
    #[serde(default)] pub compatible_plot_types: Vec<String>,
    #[serde(default)] pub compatible_collision_patterns: Vec<String>,
}

/// One category block containing multiple incidents.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct IncitingIncidentCategory {
    #[serde(default)] pub description: Option<String>,
    #[serde(default)] pub incidents: Vec<IncitingIncidentEntry>,
}

/// Top-level wrapper for `02-collision/references/inciting-incidents.yaml`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct IncitingIncidentsCatalog {
    #[serde(default)]
    pub categories: HashMap<String, IncitingIncidentCategory>,
}

impl IncitingIncidentsCatalog {
    /// Flatten all nested incidents into a single iterator.
    pub fn all_incidents(&self) -> impl Iterator<Item = &IncitingIncidentEntry> {
        self.categories.values().flat_map(|c| c.incidents.iter())
    }
}

// ── Social circle type catalog (social-circle-types.yaml) ─────────────────────

/// Matches the structure of `02-collision/references/social-circle-types.yaml`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct SocialCircleCatalogEntry {
    pub slug: String,
    pub name: String,
    #[serde(default)] pub category: Option<String>,
    #[serde(default)] pub type_label: Option<String>,
    #[serde(default)] pub typical_ideology: Option<String>,
    #[serde(default)] pub typical_core_values: Vec<String>,
    #[serde(default)] pub typical_taboos: Vec<String>,
    #[serde(default)] pub typical_power_level: Option<String>,
    #[serde(default)] pub capital_profile: Option<JsonValue>,
    #[serde(default)] pub collision_affinity: Vec<String>,
}

// ── Trope catalog (romance-tropes.yaml and genre-specific files) ──────────────

/// Matches the structure of `01-concept/references/romance-tropes.yaml`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct TropeCatalogEntry {
    pub slug: String,
    pub name: String,
    #[serde(default)] pub description: Option<String>,
    #[serde(default)] pub collision_pattern_affinity: Vec<String>,
    #[serde(default)] pub required_tension_beats: Vec<String>,
    #[serde(default)] pub wound_affinity: Vec<String>,
    #[serde(default)] pub key_tension_mechanic: Option<String>,
    #[serde(default)] pub heat_level_range: Vec<JsonValue>,
    #[serde(default)] pub reader_promise: Option<String>,
    #[serde(default)] pub subversions: Vec<String>,
    #[serde(default)] pub cliche_risks: Vec<String>,
    #[serde(default)] pub genre: Option<Genre>,
}

// ── Relationship role catalog (relationship-roles.yaml) ───────────────────────
//
// NOTE: The YAML file has a `dynamics:` top-level mapping appended after
// the list entries, making it structurally a mixed document.  The top-level
// document must be parsed as `RelationshipRolesCatalogDocument` rather than
// a bare `Vec<RelationshipRoleCatalogEntry>`.

/// One relationship type entry.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RelationshipRoleCatalogEntry {
    pub slug: String,
    pub name: String,
    #[serde(default)] pub description: Option<String>,
    #[serde(default)] pub structural_function: Option<String>,
    #[serde(default)] pub tension_source: Option<String>,
    #[serde(default)] pub subtypes: Vec<JsonValue>,
    #[serde(default)] pub arc_examples: Vec<JsonValue>,
    #[serde(default)] pub genre_affinity: Vec<String>,
}

/// Top-level wrapper for `relationship-roles.yaml`, which has optional
/// `dynamics:` and `principles:` sections after the list.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RelationshipRolesCatalogDocument {
    /// The primary list of relationship types.
    #[serde(default)]
    pub roles: Vec<RelationshipRoleCatalogEntry>,
    /// Optional dynamics section (power balance patterns, etc.).
    #[serde(default)]
    pub dynamics: Vec<JsonValue>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
struct RelationshipDynamicsWrapper {
    #[serde(default)]
    pub dynamics: Vec<JsonValue>,
}

/// Parse `relationship-roles.yaml` into canonical document shape.
///
/// Supports:
/// 1. Canonical mapping shape: `{ roles: [...], dynamics: [...] }`
/// 2. Legacy mixed shape: top-level role list followed by `dynamics:` mapping
///
/// Returns `(document, used_legacy_adapter)`.
pub fn parse_relationship_roles_catalog(
    content: &str,
) -> Result<(RelationshipRolesCatalogDocument, bool), String> {
    // Preferred canonical shape.
    if let Ok(doc) = serde_yaml::from_str::<RelationshipRolesCatalogDocument>(content) {
        if !doc.roles.is_empty() || !doc.dynamics.is_empty() {
            return Ok((doc, false));
        }
    }

    // Legacy shape adapter: split list and dynamics section.
    let needle = "\ndynamics:";
    let Some(pos) = content.find(needle) else {
        return Err("relationship-roles parse failed: no canonical mapping and no legacy dynamics section".to_owned());
    };

    let roles_part = &content[..pos];
    let dynamics_part = &content[pos + 1..]; // keep `dynamics:` at start

    let roles: Vec<RelationshipRoleCatalogEntry> = serde_yaml::from_str(roles_part)
        .map_err(|e| format!("relationship-roles legacy roles parse failed: {e}"))?;

    let dyns = serde_yaml::from_str::<RelationshipDynamicsWrapper>(dynamics_part)
        .map_err(|e| format!("relationship-roles legacy dynamics parse failed: {e}"))?;

    Ok((
        RelationshipRolesCatalogDocument {
            roles,
            dynamics: dyns.dynamics,
        },
        true,
    ))
}

// ── Shared helpers ───────────────────────────────────────────────────────────

/// A `{work: "...", note: "..."}` example entry reused across theory catalogs.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CatalogExample {
    pub work: String,
    #[serde(default)]
    pub note: Option<String>,
}

// ── Narrative-time modes catalog (narrative-time-modes.yaml) ─────────────────

/// Matches `05-plot-and-structure/references/narrative-time-modes.yaml`.
/// Covers Genette's three temporal axes: order, duration, frequency.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct NarrativeTimeModeEntry {
    pub slug: String,
    pub axis: String,
    pub name: String,
    #[serde(default)] pub description: Option<String>,
    #[serde(default)] pub craft_effect: Option<String>,
    #[serde(default)] pub when_to_use: Option<String>,
    #[serde(default)] pub risk: Option<String>,
    #[serde(default)] pub canonical_examples: Vec<CatalogExample>,
}

// ── Irony types catalog (irony-types.yaml) ───────────────────────────────────

/// Matches `07-drafting/references/irony-types.yaml`.
/// Booth *A Rhetoric of Irony* / Hutcheon *A Theory of Parody*.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct IronyTypeCatalogEntry {
    pub slug: String,
    pub name: String,
    pub definition: String,
    #[serde(default)] pub how_it_works: Option<String>,
    #[serde(default)] pub tone_range: Vec<String>,
    #[serde(default)] pub distinction: Option<String>,
    #[serde(default)] pub risks: Vec<String>,
    #[serde(default)] pub craft_notes: Vec<String>,
    #[serde(default)] pub craft_note: Option<String>,
    #[serde(default)] pub canonical_examples: Vec<CatalogExample>,
}

// ── Comic modes catalog (comic-modes.yaml) ───────────────────────────────────

/// Matches `07-drafting/references/comic-modes.yaml`.
/// Bergson, Morreall, Frye; covers both modes and comedy theories.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct ComicModeCatalogEntry {
    pub slug: String,
    pub category: String,
    pub name: String,
    #[serde(default)] pub definition: Option<String>,
    #[serde(default)] pub target: Option<String>,
    #[serde(default)] pub tone_range: Vec<String>,
    #[serde(default)] pub techniques: Vec<String>,
    #[serde(default)] pub risk: Option<String>,
    #[serde(default)] pub craft_note: Option<String>,
    // For comedy theory entries
    #[serde(default)] pub theorists: Vec<String>,
    #[serde(default)] pub narrative_application: Option<String>,
    #[serde(default)] pub craft_tool: Option<String>,
    #[serde(default)] pub bergson_addition: Option<String>,
    #[serde(default)] pub freud_addition: Option<String>,
    #[serde(default)] pub relationship_to_source: Option<String>,
    #[serde(default)] pub canonical_examples: Vec<CatalogExample>,
}

// ── Metaphor types catalog (metaphor-types.yaml) ─────────────────────────────

/// Matches `07-drafting/references/metaphor-types.yaml`.
/// Lakoff/Johnson, Richards, Black, Jakobson.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct MetaphorTypeCatalogEntry {
    pub slug: String,
    pub name: String,
    pub definition: String,
    #[serde(default)] pub theoretical_source: Option<String>,
    #[serde(default)] pub craft_function: Option<String>,
    #[serde(default)] pub detection_signal: Option<String>,
    #[serde(default)] pub craft_notes: Vec<String>,
    #[serde(default)] pub craft_note: Option<String>,
    #[serde(default)] pub how_it_works: Option<String>,
    #[serde(default)] pub risk: Option<String>,
    #[serde(default)] pub difference_from_metaphor: Option<String>,
    #[serde(default)] pub difference_from_synecdoche: Option<String>,
    #[serde(default)] pub relationship_to_source: Option<String>,
    #[serde(default)] pub canonical_examples: Vec<CatalogExample>,
}

// ── Trauma modes catalog (trauma-modes.yaml) ─────────────────────────────────

/// Matches `07-drafting/references/trauma-modes.yaml`.
/// Caruth, LaCapra, Herman, van der Kolk, Felman/Laub.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct TraumaModeEntry {
    pub slug: String,
    pub category: String,
    pub name: String,
    #[serde(default)] pub definition: Option<String>,
    #[serde(default)] pub narrative_applications: Vec<String>,
    #[serde(default)] pub psychic_distance_note: Option<String>,
    #[serde(default)] pub temporal_structures: Vec<String>,
    #[serde(default)] pub difference_from_working_through: Option<String>,
    #[serde(default)] pub difference_from_acting_out: Option<String>,
    #[serde(default)] pub craft_notes: Vec<String>,
    // For Herman recovery stage entries
    #[serde(default)] pub stage_number: Option<u32>,
    // For van der Kolk entry
    #[serde(default)] pub canonical_examples: Vec<CatalogExample>,
}

// ── Revision passes catalog (revision-passes.yaml) ───────────────────────────

/// A brief `{source: "...", note: "..."}` entry in revision-passes canonical_sources.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CanonicalSource {
    pub source: String,
    #[serde(default)] pub note: Option<String>,
}

/// Matches `08-revision/references/revision-passes.yaml`.
/// Murray, Elbow, Lamott six-pass system.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RevisionPassEntry {
    pub slug: String,
    pub pass_number: u32,
    pub name: String,
    pub focus: String,
    pub guiding_question: String,
    #[serde(default)] pub what_to_look_for: Vec<String>,
    #[serde(default)] pub tools: Vec<String>,
    #[serde(default)] pub do_not_yet: Option<String>,
    #[serde(default)] pub canonical_sources: Vec<CanonicalSource>,
    #[serde(default)] pub warning: Option<String>,
}

// ── Pentad elements catalog (pentad-elements.yaml) ────────────────────────────

/// Matches `07-drafting/references/pentad-elements.yaml`.
/// Burke *A Grammar of Motives* (1945).
/// Both `type: element` and `type: ratio` entries share this struct.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct PentadElementEntry {
    pub slug: String,
    /// "element" or "ratio"
    #[serde(rename = "type")]
    pub entry_type: String,
    pub name: String,
    #[serde(default)] pub burke_definition: Option<String>,
    #[serde(default)] pub core_question: Option<String>,
    #[serde(default)] pub narrative_function: Option<String>,
    #[serde(default)] pub character_design_questions: Vec<String>,
    #[serde(default)] pub craft_notes: Vec<String>,
    // Ratio-specific fields
    #[serde(default)] pub dominant_explanation: Option<String>,
    #[serde(default)] pub philosophy: Option<String>,
    #[serde(default)] pub character_type: Option<String>,
    #[serde(default)] pub narrative_mode: Option<String>,
    #[serde(default)] pub worldview: Option<String>,
    #[serde(default)] pub craft_application: Option<String>,
}

// ─── Theory catalogs (Phase II–V additions) ───────────────────────────────────

/// Entry in `07-drafting/references/speech-act-types.yaml`
/// Austin *How to Do Things with Words* (1962); Searle *Speech Acts* (1969)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct SpeechActEntry {
    pub slug: String,
    /// "austin_dimension" | "searle_category" | "pragmatic_extension"
    pub category: String,
    pub name: String,
    #[serde(default)] pub austin_dimension: Option<String>,
    #[serde(default)] pub definition: Option<String>,
    #[serde(default)] pub felicity_conditions: Vec<String>,
    #[serde(default)] pub subtypes: Vec<String>,
    #[serde(default)] pub narrative_applications: Vec<String>,
    #[serde(default)] pub craft_question: Option<String>,
    #[serde(default)] pub canonical_examples: Vec<CatalogExample>,
    // Searle-specific
    #[serde(default)] pub illocutionary_point: Option<String>,
    #[serde(default)] pub direction_of_fit: Option<String>,
    #[serde(default)] pub psychological_state: Option<String>,
    #[serde(default)] pub propositional_content: Option<String>,
    #[serde(default)] pub narrative_function: Option<String>,
    // Austin-specific
    #[serde(default)] pub act_type: Option<String>,
    #[serde(default)] pub force_markers: Vec<String>,
}

/// Entry in `07-drafting/references/focalization-modes.yaml`
/// Genette *Narrative Discourse* (1972); Bal *Narratology* (1985);
/// Gardner *The Art of Fiction* (1983); Cohn *Transparent Minds* (1978)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct FocalizationModeEntry {
    pub slug: String,
    /// "genette_focalization" | "bal_focalization" | "gardner_psychic_distance"
    /// | "cohn_consciousness"
    pub category: String,
    pub name: String,
    #[serde(default)] pub genette_formula: Option<String>,
    #[serde(default)] pub definition: Option<String>,
    #[serde(default)] pub craft_characteristics: Vec<String>,
    #[serde(default)] pub psychic_distance_range: Option<String>,
    #[serde(default)] pub risk: Option<String>,
    #[serde(default)] pub canonical_examples: Vec<CatalogExample>,
    // Gardner psychic distance
    #[serde(default)] pub level: Option<u8>,
    #[serde(default)] pub example_sentence: Option<String>,
    // Cohn consciousness modes
    #[serde(default)] pub narrator_type: Option<String>,
    #[serde(default)] pub technique_example: Option<String>,
    #[serde(default)] pub craft_note: Option<String>,
    // Bal
    #[serde(default)] pub distinction_from_narrator: Option<String>,
    #[serde(default)] pub internal_vs_external: Option<String>,
}

/// Entry in `01-concept/references/intertextual-relations.yaml`
/// Genette *Palimpsests / Paratexts* (1982/87); Bloom *Anxiety of Influence* (1973)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct IntertextualRelationEntry {
    pub slug: String,
    /// "genette_transtextuality" | "bloom_ratio"
    pub category: String,
    #[serde(default)] pub rank: Option<u8>,
    pub name: String,
    #[serde(default)] pub genette_definition: Option<String>,
    #[serde(default)] pub definition: Option<String>,
    #[serde(default)] pub subtypes: Vec<String>,
    #[serde(default)] pub craft_notes: Vec<String>,
    #[serde(default)] pub canonical_examples: Vec<CatalogExample>,
    // Bloom-specific
    #[serde(default)] pub greek_term: Option<String>,
    #[serde(default)] pub revisionary_ratio: Option<String>,
    #[serde(default)] pub narrative_manifestation: Option<String>,
}

/// Entry in `07-drafting/references/psychoanalytic-mechanisms.yaml`
/// Freud; Lacan *Écrits* (1966); Kristeva *Powers of Horror* (1980);
/// Mulvey "Visual Pleasure" (1975)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct PsychoanalyticMechanismEntry {
    pub slug: String,
    /// "freudian" | "lacan_register" | "kristeva" | "mulvey_gaze"
    pub category: String,
    #[serde(default)] pub mechanism_type: Option<String>,
    pub name: String,
    #[serde(default)] pub definition: Option<String>,
    #[serde(default)] pub narrative_applications: Vec<String>,
    #[serde(default)] pub detection_signal: Option<String>,
    #[serde(default)] pub craft_notes: Vec<String>,
    #[serde(default)] pub canonical_examples: Vec<CatalogExample>,
    // Lacan register fields
    #[serde(default)] pub register: Option<String>,
    #[serde(default)] pub lacanian_term: Option<String>,
    #[serde(default)] pub failure_mode: Option<String>,
    // Freud mechanism fields
    #[serde(default)] pub freudian_process: Option<String>,
    #[serde(default)] pub scene_audit_question: Option<String>,
}

/// Entry in `08-revision/references/postcolonial-modes.yaml`
/// Said *Orientalism* (1978); Bhabha *Location of Culture* (1994);
/// Spivak "Can the Subaltern Speak?" (1988); Fanon; Achebe; Ngũgĩ
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct PostcolonialModeEntry {
    pub slug: String,
    pub name: String,
    #[serde(default)] pub theorist: Option<String>,
    #[serde(default)] pub theorists: Vec<String>,
    #[serde(default)] pub definition: Option<String>,
    #[serde(default)] pub textual_markers: Vec<String>,
    #[serde(default)] pub narrative_audit_questions: Vec<String>,
    #[serde(default)] pub distinction_from_synthesis: Option<String>,
    #[serde(default)] pub narrative_applications: Vec<String>,
    #[serde(default)] pub craft_notes: Vec<String>,
    #[serde(default)] pub canonical_examples: Vec<CatalogExample>,
}

/// Entry in `05-plot-and-structure/references/emplotment-types.yaml`
/// Hayden White *Metahistory* (1973); Northrop Frye *Anatomy of Criticism* (1957)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct EmplotmentTypeEntry {
    pub slug: String,
    /// "white_emplotment" | "frye_mythos" | "white_extended"
    pub category: String,
    pub name: String,
    #[serde(default)] pub white_definition: Option<String>,
    #[serde(default)] pub frye_definition: Option<String>,
    #[serde(default)] pub hutcheon_definition: Option<String>,
    #[serde(default)] pub narrative_structure: Option<String>,
    #[serde(default)] pub ideological_tendency: Option<String>,
    #[serde(default)] pub craft_application: Option<String>,
    #[serde(default)] pub risk: Option<String>,
    #[serde(default)] pub canonical_examples: Vec<CatalogExample>,
    // Frye-specific
    #[serde(default)] pub season: Option<String>,
    #[serde(default)] pub frye_parallel: Option<String>,
    #[serde(default)] pub dramatic_structure: Vec<String>,
    #[serde(default)] pub narrative_correspondence: Option<String>,
    // Extended White
    #[serde(default)] pub relationship_to_white: Option<String>,
    #[serde(default)] pub craft_applications: Vec<String>,
    #[serde(default)] pub historiographic_use: Option<String>,
}

/// Helper: common_lie_pair sub-structure used in genre trope files
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, Default)]
pub struct CommonLiePair {
    #[serde(default)] pub protagonist: Option<String>,
    #[serde(default)] pub antagonist_force: Option<String>,
    #[serde(default)] pub antagonist: Option<String>,
}

/// Entry in `01-concept/references/{genre}-tropes.yaml`
/// Covers horror, scifi, fantasy, mystery, thriller, literary-fiction, historical-fiction.
/// Generic fields are shared across all 7 files; genre-specific fields are `Option`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct GenreTropeCatalogEntry {
    pub slug: String,
    pub name: String,
    #[serde(default)] pub description: Option<String>,
    #[serde(default)] pub collision_pattern_affinity: Vec<String>,
    #[serde(default)] pub required_tension_beats: Vec<String>,
    #[serde(default)] pub wound_affinity: Vec<String>,
    #[serde(default)] pub key_tension_mechanic: Option<String>,
    #[serde(default)] pub ticking_clock: Option<String>,
    #[serde(default)] pub common_lie_pair: Option<CommonLiePair>,
    #[serde(default)] pub reader_promise: Option<String>,
    #[serde(default)] pub genre_affinity: Vec<String>,
    // Horror-specific (Carroll)
    #[serde(default)] pub carroll_violation: Option<String>,
    // Sci-fi-specific (Suvin)
    #[serde(default)] pub novum: Option<String>,
    #[serde(default)] pub suvin_estrangement: Option<String>,
    // Fantasy-specific
    #[serde(default)] pub world_building_demand: Option<String>,
    // Mystery-specific
    #[serde(default)] pub fair_play_rule: Option<String>,
    // Literary-fiction-specific
    #[serde(default)] pub narrative_technique: Option<String>,
    #[serde(default)] pub exemplars: Vec<String>,
    // Historical-fiction-specific (Lukács)
    #[serde(default)] pub lukacs_principle: Option<String>,
}

// ── Phase V gap-fill theory catalog structs ──────────────────────────────────

/// Entry in `07-drafting/references/image-schemas.yaml`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct ImageSchemaEntry {
    pub slug: String,
    pub name: String,
    #[serde(default)] pub theorist: Option<String>,
    #[serde(default)] pub definition: Option<String>,
    #[serde(default)] pub core_structure: Option<String>,
    #[serde(default)] pub primary_mappings: Vec<String>,
    #[serde(default)] pub narrative_applications: Vec<String>,
    #[serde(default)] pub prose_technique: Option<String>,
    #[serde(default)] pub canonical_examples: Vec<CatalogExample>,
}

/// Entry in `07-drafting/references/subtext-modes.yaml`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct SubtextModeEntry {
    pub slug: String,
    #[serde(default)] pub category: Option<String>,
    pub name: String,
    #[serde(default)] pub grice_formulation: Option<String>,
    #[serde(default)] pub violation_produces: Option<String>,
    #[serde(default)] pub subtext_applications: Vec<String>,
    #[serde(default)] pub craft_question: Option<String>,
    #[serde(default)] pub signal: Option<String>,
    #[serde(default)] pub subtext_function: Option<String>,
    #[serde(default)] pub function: Option<String>,
    #[serde(default)] pub canonical_examples: Vec<CatalogExample>,
}

/// Entry in `07-drafting/references/adaptation-modes.yaml`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct AdaptationModeEntry {
    pub slug: String,
    #[serde(default)] pub category: Option<String>,
    pub name: String,
    #[serde(default)] pub hutcheon_definition: Option<String>,
    #[serde(default)] pub jenkins_definition: Option<String>,
    #[serde(default)] pub definition: Option<String>,
    #[serde(default)] pub canonical_media: Vec<String>,
    #[serde(default)] pub adaptation_notes: Vec<String>,
    #[serde(default)] pub craft_question: Option<String>,
    #[serde(default)] pub craft_implications: Vec<String>,
    #[serde(default)] pub tension_with: Option<String>,
    #[serde(default)] pub canonical_examples: Vec<CatalogExample>,
}

/// Entry in `07-drafting/references/autofiction-modes.yaml`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct AutofictionModeEntry {
    pub slug: String,
    #[serde(default)] pub category: Option<String>,
    pub name: String,
    #[serde(default)] pub lejeune_definition: Option<String>,
    #[serde(default)] pub doubrovsky_definition: Option<String>,
    #[serde(default)] pub ernaux_definition: Option<String>,
    #[serde(default)] pub barthes_definition: Option<String>,
    #[serde(default)] pub definition: Option<String>,
    #[serde(default)] pub pact_signals: Vec<String>,
    #[serde(default)] pub craft_implications: Vec<String>,
    #[serde(default)] pub risks: Vec<String>,
    #[serde(default)] pub canonical_examples: Vec<CatalogExample>,
}

/// Entry in `07-drafting/references/experimental-narration-modes.yaml`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct ExperimentalNarrationEntry {
    pub slug: String,
    #[serde(default)] pub category: Option<String>,
    pub name: String,
    #[serde(default)] pub booth_definition: Option<String>,
    #[serde(default)] pub waugh_definition: Option<String>,
    #[serde(default)] pub definition: Option<String>,
    #[serde(default)] pub unreliability_types: Vec<String>,
    #[serde(default)] pub metafictional_strategies: Vec<String>,
    #[serde(default)] pub functional_modes: Vec<String>,
    #[serde(default)] pub structural_types: Vec<String>,
    #[serde(default)] pub effects: Vec<String>,
    #[serde(default)] pub craft_notes: Vec<String>,
    #[serde(default)] pub craft_implications: Vec<String>,
    #[serde(default)] pub canonical_examples: Vec<CatalogExample>,
}

/// Entry in `07-drafting/references/philosophy-fiction-modes.yaml`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct PhilosophyFictionEntry {
    pub slug: String,
    #[serde(default)] pub category: Option<String>,
    pub name: String,
    #[serde(default)] pub ricoeur_definition: Option<String>,
    #[serde(default)] pub eco_definition: Option<String>,
    #[serde(default)] pub walton_definition: Option<String>,
    #[serde(default)] pub dolezal_definition: Option<String>,
    #[serde(default)] pub definition: Option<String>,
    #[serde(default)] pub craft_relevance: Vec<String>,
    #[serde(default)] pub make_believe_structure: Vec<String>,
    #[serde(default)] pub distinction: Option<String>,
    #[serde(default)] pub narrative_function: Option<String>,
    #[serde(default)] pub craft_question: Option<String>,
    #[serde(default)] pub craft_implications: Vec<String>,
    #[serde(default)] pub canonical_examples: Vec<CatalogExample>,
}

/// Entry in `07-drafting/references/verse-prosody.yaml`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct VerseProsodyEntry {
    pub slug: String,
    #[serde(default)] pub category: Option<String>,
    pub name: String,
    #[serde(default)] pub pattern: Option<String>,
    #[serde(default)] pub symbol: Option<String>,
    #[serde(default)] pub dominant_meter: Option<String>,
    #[serde(default)] pub craft_quality: Option<String>,
    #[serde(default)] pub fussell_note: Option<String>,
    #[serde(default)] pub definition: Option<String>,
    #[serde(default)] pub affective_function: Option<String>,
    #[serde(default)] pub craft_notes: Vec<String>,
    #[serde(default)] pub types: Vec<String>,
    #[serde(default)] pub forms: Vec<String>,
    #[serde(default)] pub canonical_examples: Vec<CatalogExample>,
}

/// Entry in `08-revision/references/ecocriticism-modes.yaml`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct EcocriticalModeEntry {
    pub slug: String,
    #[serde(default)] pub category: Option<String>,
    pub name: String,
    #[serde(default)] pub nixon_definition: Option<String>,
    #[serde(default)] pub morton_definition: Option<String>,
    #[serde(default)] pub buell_haraway_basis: Option<String>,
    #[serde(default)] pub definition: Option<String>,
    #[serde(default)] pub craft_challenge: Option<String>,
    #[serde(default)] pub narrative_strategies: Vec<String>,
    #[serde(default)] pub audit_questions: Vec<String>,
    #[serde(default)] pub craft_implications: Vec<String>,
    #[serde(default)] pub canonical_examples: Vec<CatalogExample>,
}

/// Entry in `08-revision/references/feminist-narrative-types.yaml`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct FeministNarrativeEntry {
    pub slug: String,
    #[serde(default)] pub category: Option<String>,
    pub name: String,
    #[serde(default)] pub duplessis_definition: Option<String>,
    #[serde(default)] pub lanser_definition: Option<String>,
    #[serde(default)] pub warhol_definition: Option<String>,
    #[serde(default)] pub fetterley_definition: Option<String>,
    #[serde(default)] pub miller_definition: Option<String>,
    #[serde(default)] pub definition: Option<String>,
    #[serde(default)] pub traditional_plot_types_resisted: Vec<String>,
    #[serde(default)] pub counter_plot_strategies: Vec<String>,
    #[serde(default)] pub audit_questions: Vec<String>,
    #[serde(default)] pub craft_notes: Vec<String>,
    #[serde(default)] pub craft_implications: Vec<String>,
    #[serde(default)] pub canonical_examples: Vec<CatalogExample>,
}

/// Entry in `08-revision/references/posthuman-modes.yaml`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct PosthumanModeEntry {
    pub slug: String,
    #[serde(default)] pub category: Option<String>,
    pub name: String,
    #[serde(default)] pub wolfe_definition: Option<String>,
    #[serde(default)] pub haraway_definition: Option<String>,
    #[serde(default)] pub hayles_definition: Option<String>,
    #[serde(default)] pub braidotti_definition: Option<String>,
    #[serde(default)] pub definition: Option<String>,
    #[serde(default)] pub contrast_with: Option<String>,
    #[serde(default)] pub craft_implications: Vec<String>,
    #[serde(default)] pub audit_questions: Vec<String>,
    #[serde(default)] pub philosophical_crux: Option<String>,
    #[serde(default)] pub canonical_examples: Vec<CatalogExample>,
}

/// Entry in `08-revision/references/queer-narrative-modes.yaml`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct QueerNarrativeModeEntry {
    pub slug: String,
    #[serde(default)] pub category: Option<String>,
    pub name: String,
    #[serde(default)] pub edelman_definition: Option<String>,
    #[serde(default)] pub halberstam_definition: Option<String>,
    #[serde(default)] pub munoz_definition: Option<String>,
    #[serde(default)] pub butler_definition: Option<String>,
    #[serde(default)] pub sedgwick_definition: Option<String>,
    #[serde(default)] pub definition: Option<String>,
    #[serde(default)] pub narrative_manifestations: Vec<String>,
    #[serde(default)] pub craft_implications: Vec<String>,
    #[serde(default)] pub queer_critique: Option<String>,
    #[serde(default)] pub audit_questions: Vec<String>,
    #[serde(default)] pub audit_question: Option<String>,
    #[serde(default)] pub canonical_examples: Vec<CatalogExample>,
    #[serde(default)] pub intersectional_instances: Vec<String>,
}

/// Entry in `08-revision/references/disability-rep-modes.yaml`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct DisabilityRepModeEntry {
    pub slug: String,
    #[serde(default)] pub category: Option<String>,
    pub name: String,
    #[serde(default)] pub mitchell_snyder_definition: Option<String>,
    #[serde(default)] pub garland_thomson_definition: Option<String>,
    #[serde(default)] pub siebers_definition: Option<String>,
    #[serde(default)] pub kafer_definition: Option<String>,
    #[serde(default)] pub definition: Option<String>,
    #[serde(default)] pub prosthesis_patterns: Vec<String>,
    #[serde(default)] pub audit_patterns: Vec<String>,
    #[serde(default)] pub audit_questions: Vec<String>,
    #[serde(default)] pub craft_implications: Vec<String>,
    #[serde(default)] pub canonical_examples: Vec<CatalogExample>,
    #[serde(default)] pub connection_to_reproductive_futurism: Option<String>,
}

/// Entry in `08-revision/references/marxist-narrative-modes.yaml`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct MarxistNarrativeModeEntry {
    pub slug: String,
    #[serde(default)] pub category: Option<String>,
    pub name: String,
    #[serde(default)] pub jameson_definition: Option<String>,
    #[serde(default)] pub goldmann_definition: Option<String>,
    #[serde(default)] pub eagleton_definition: Option<String>,
    #[serde(default)] pub lukacs_definition: Option<String>,
    #[serde(default)] pub williams_definition: Option<String>,
    #[serde(default)] pub definition: Option<String>,
    #[serde(default)] pub three_horizons: Vec<String>,
    #[serde(default)] pub late_lukacs_critique: Option<String>,
    #[serde(default)] pub narrative_function: Option<String>,
    #[serde(default)] pub craft_implications: Vec<String>,
    #[serde(default)] pub audit_questions: Vec<String>,
    #[serde(default)] pub canonical_examples: Vec<CatalogExample>,
}

/// Entry in `08-revision/references/indigenous-narrative-modes.yaml`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct IndigenousNarrativeModeEntry {
    pub slug: String,
    #[serde(default)] pub category: Option<String>,
    pub name: String,
    #[serde(default)] pub vizenor_definition: Option<String>,
    #[serde(default)] pub archibald_definition: Option<String>,
    #[serde(default)] pub womack_definition: Option<String>,
    #[serde(default)] pub silko_definition: Option<String>,
    #[serde(default)] pub definition: Option<String>,
    #[serde(default)] pub victimry_contrast: Option<String>,
    #[serde(default)] pub red_on_red: Option<String>,
    #[serde(default)] pub hurston_example: Option<String>,
    #[serde(default)] pub signifying_modes: Vec<String>,
    #[serde(default)] pub craft_implications: Vec<String>,
    #[serde(default)] pub audit_questions: Vec<String>,
    #[serde(default)] pub canonical_examples: Vec<CatalogExample>,
}

/// Entry in `08-revision/references/affect-modes.yaml`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct AffectModeEntry {
    pub slug: String,
    #[serde(default)] pub category: Option<String>,
    pub name: String,
    #[serde(default)] pub ahmed_definition: Option<String>,
    #[serde(default)] pub berlant_definition: Option<String>,
    #[serde(default)] pub berlant_freud_connection: Option<String>,
    #[serde(default)] pub definition: Option<String>,
    #[serde(default)] pub good_life_fantasies: Vec<String>,
    #[serde(default)] pub historical_context: Option<String>,
    #[serde(default)] pub craft_implications: Vec<String>,
    #[serde(default)] pub audit_questions: Vec<String>,
    #[serde(default)] pub canonical_examples: Vec<CatalogExample>,
}

/// Entry in `08-revision/references/narrative-ethics-modes.yaml`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct NarrativeEthicsModeEntry {
    pub slug: String,
    #[serde(default)] pub category: Option<String>,
    pub name: String,
    #[serde(default)] pub booth_definition: Option<String>,
    #[serde(default)] pub keen_definition: Option<String>,
    #[serde(default)] pub nussbaum_definition: Option<String>,
    #[serde(default)] pub definition: Option<String>,
    #[serde(default)] pub implied_author: Option<String>,
    #[serde(default)] pub risks: Vec<String>,
    #[serde(default)] pub craft_implications: Vec<String>,
    #[serde(default)] pub audit_questions: Vec<String>,
    #[serde(default)] pub audit_checklist: Vec<String>,
    #[serde(default)] pub mulvey_basis: Option<String>,
    #[serde(default)] pub canonical_examples: Vec<CatalogExample>,
}

/// Entry in `references/cognitive-narrative-modes.yaml`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CognitiveNarrativeModeEntry {
    pub slug: String,
    #[serde(default)] pub category: Option<String>,
    pub name: String,
    #[serde(default)] pub fludernik_definition: Option<String>,
    #[serde(default)] pub palmer_definition: Option<String>,
    #[serde(default)] pub hogan_definition: Option<String>,
    #[serde(default)] pub nunning_definition: Option<String>,
    #[serde(default)] pub definition: Option<String>,
    #[serde(default)] pub natural_narratology: Option<String>,
    #[serde(default)] pub intermental_thought: Option<String>,
    #[serde(default)] pub universal_plots: Vec<String>,
    #[serde(default)] pub craft_implications: Vec<String>,
    #[serde(default)] pub audit_questions: Vec<String>,
    #[serde(default)] pub canonical_examples: Vec<CatalogExample>,
}

/// Entry in `references/signifying-modes.yaml`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct SignifyingModeEntry {
    pub slug: String,
    #[serde(default)] pub category: Option<String>,
    pub name: String,
    #[serde(default)] pub gates_definition: Option<String>,
    #[serde(default)] pub baker_definition: Option<String>,
    #[serde(default)] pub morrison_definition: Option<String>,
    #[serde(default)] pub hurston_definition: Option<String>,
    #[serde(default)] pub gates_dubois_synthesis: Option<String>,
    #[serde(default)] pub definition: Option<String>,
    #[serde(default)] pub signifying_modes: Vec<String>,
    #[serde(default)] pub formal_features: Vec<String>,
    #[serde(default)] pub craft_implications: Vec<String>,
    #[serde(default)] pub audit_questions: Vec<String>,
    #[serde(default)] pub canonical_examples: Vec<CatalogExample>,
}

/// Entry in `04-world-building/references/spatial-modes.yaml`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct SpatialModeEntry {
    pub slug: String,
    #[serde(default)] pub category: Option<String>,
    pub name: String,
    #[serde(default)] pub bachelard_definition: Option<String>,
    #[serde(default)] pub de_certeau_definition: Option<String>,
    #[serde(default)] pub definition: Option<String>,
    #[serde(default)] pub phenomenological_quality: Option<String>,
    #[serde(default)] pub narrative_function: Option<String>,
    #[serde(default)] pub contrast_pair: Option<String>,
    #[serde(default)] pub psychoanalytic_resonance: Option<String>,
    #[serde(default)] pub craft_implications: Vec<String>,
    #[serde(default)] pub canonical_examples: Vec<CatalogExample>,
}

/// Entry in `05-plot-and-structure/references/propp-functions.yaml`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct ProppFunctionEntry {
    pub slug: String,
    pub number: serde_json::Value,   // can be int or string like "8a"
    #[serde(default)] pub category: Option<String>,
    pub name: String,
    pub propp_definition: String,
    pub narrative_function: String,
    #[serde(default)] pub note: Option<String>,
    #[serde(default)] pub canonical_examples: Vec<CatalogExample>,
}

/// Entry in `05-plot-and-structure/references/seriality-types.yaml`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct SerialityTypeEntry {
    pub slug: String,
    #[serde(default)] pub category: Option<String>,
    pub name: String,
    pub definition: String,
    #[serde(default)] pub craft_implications: Vec<String>,
    #[serde(default)] pub reader_contract: Option<String>,
    #[serde(default)] pub canonical_examples: Vec<CatalogExample>,
}

// ── Phase V round-2 theory catalog structs ─────────────────────────────────

/// Entry in `07-drafting/references/translation-modes.yaml`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct TranslationModeEntry {
    pub slug: String,
    #[serde(default)] pub category: Option<String>,
    pub name: String,
    #[serde(default)] pub venuti_definition: Option<String>,
    #[serde(default)] pub definition: Option<String>,
    #[serde(default)] pub ideological_risk: Option<String>,
    #[serde(default)] pub craft_implications: Vec<String>,
    #[serde(default)] pub audit_questions: Vec<String>,
    #[serde(default)] pub canonical_examples: Vec<CatalogExample>,
}

/// Entry in `07-drafting/references/graphic-narrative-modes.yaml`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct GraphicNarrativeModeEntry {
    pub slug: String,
    #[serde(default)] pub category: Option<String>,
    pub name: String,
    #[serde(default)] pub mccloud_definition: Option<String>,
    #[serde(default)] pub chute_definition: Option<String>,
    #[serde(default)] pub definition: Option<String>,
    #[serde(default)] pub closure_demand: Option<String>,
    #[serde(default)] pub temporal_effect: Option<String>,
    #[serde(default)] pub craft_implications: Vec<String>,
    #[serde(default)] pub canonical_examples: Vec<CatalogExample>,
}

/// Entry in `07-drafting/references/ya-narrative-modes.yaml`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct YaNarrativeModeEntry {
    pub slug: String,
    #[serde(default)] pub category: Option<String>,
    pub name: String,
    #[serde(default)] pub nikolajeva_definition: Option<String>,
    #[serde(default)] pub trites_definition: Option<String>,
    #[serde(default)] pub nodelman_definition: Option<String>,
    #[serde(default)] pub definition: Option<String>,
    #[serde(default)] pub aetonormativity_note: Option<String>,
    #[serde(default)] pub nodelman_note: Option<String>,
    #[serde(default)] pub trites_power_note: Option<String>,
    #[serde(default)] pub power_paradox: Option<String>,
    #[serde(default)] pub craft_implications: Vec<String>,
    #[serde(default)] pub audit_questions: Vec<String>,
    #[serde(default)] pub canonical_examples: Vec<CatalogExample>,
}

/// Entry in `09-polish-and-publish/references/paratext-zones.yaml`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct ParatextZoneEntry {
    pub slug: String,
    #[serde(default)] pub category: Option<String>,
    pub name: String,
    #[serde(default)] pub genette_definition: Option<String>,
    #[serde(default)] pub definition: Option<String>,
    #[serde(default)] pub design_considerations: Vec<String>,
    #[serde(default)] pub dedication_modes: Vec<String>,
    #[serde(default)] pub craft_implications: Vec<String>,
    #[serde(default)] pub audit_questions: Vec<String>,
    #[serde(default)] pub canonical_examples: Vec<CatalogExample>,
}

/// Entry in `references/semio-linguistic-functions.yaml`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct SemioLinguisticFunctionEntry {
    pub slug: String,
    #[serde(default)] pub category: Option<String>,
    pub name: String,
    #[serde(default)] pub jakobson_definition: Option<String>,
    #[serde(default)] pub halliday_definition: Option<String>,
    #[serde(default)] pub peirce_definition: Option<String>,
    #[serde(default)] pub definition: Option<String>,
    #[serde(default)] pub efferent_contrast: Option<String>,
    #[serde(default)] pub craft_implications: Vec<String>,
    #[serde(default)] pub audit_questions: Vec<String>,
    #[serde(default)] pub canonical_examples: Vec<CatalogExample>,
}

/// Entry in `01-concept/references/genre-reading-modes.yaml`
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct GenreReadingModeEntry {
    pub slug: String,
    #[serde(default)] pub category: Option<String>,
    pub name: String,
    #[serde(default)] pub derrida_definition: Option<String>,
    #[serde(default)] pub culler_definition: Option<String>,
    #[serde(default)] pub rosenblatt_definition: Option<String>,
    #[serde(default)] pub fish_definition: Option<String>,
    #[serde(default)] pub definition: Option<String>,
    #[serde(default)] pub efferent_contrast: Option<String>,
    #[serde(default)] pub craft_implications: Vec<String>,
    #[serde(default)] pub audit_questions: Vec<String>,
    #[serde(default)] pub canonical_examples: Vec<CatalogExample>,
}

// ── Alignment system catalog structs ─────────────────────────────────────────

/// A cell in the 9-cell alignment grid (`03-characters/references/alignment-system.yaml`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct AlignmentCellEntry {
    pub slug: String,
    pub order: String,
    pub ethics: String,
    pub label: String,
    #[serde(default)] pub dnd_equivalent: Option<String>,
    #[serde(default)] pub moral_rigidity_range: Vec<u8>,
    pub description: String,
    #[serde(default)] pub internal_conflict: Option<String>,
    #[serde(default)] pub archetype_affinity: Vec<String>,
    #[serde(default)] pub example: Option<String>,
    #[serde(default)] pub arc_pressure: Option<String>,
}

/// An arc pattern entry in `alignment-system.yaml`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct AlignmentArcPatternEntry {
    pub slug: String,
    pub name: String,
    pub direction: String,
    pub description: String,
    #[serde(default)] pub typical_trigger: Option<String>,
    #[serde(default)] pub genre_affinity: Vec<String>,
}

/// Top-level catalog for `03-characters/references/alignment-system.yaml`.
/// Contains `cells:` (the 9-cell grid) and `arc_patterns:`; the `axes:` section is skipped.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct AlignmentSystemCatalog {
    #[serde(default)] pub cells: Vec<AlignmentCellEntry>,
    #[serde(default)] pub arc_patterns: Vec<AlignmentArcPatternEntry>,
}

// ── Romance beats catalog structs ────────────────────────────────────────────

/// A single beat entry in `05-plot-and-structure/references/romance-beats.yaml`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RomanceBeatEntry {
    pub slug: String,
    pub name: String,
    #[serde(default)] pub position_pct: Vec<f32>,
    #[serde(default)] pub act: String,
    #[serde(default)] pub structural_alignment: Vec<String>,
    #[serde(default)] pub description: Option<String>,
    #[serde(default)] pub emotional_function: Option<String>,
    #[serde(default)] pub tension_level: Vec<u8>,
    #[serde(default)] pub required: bool,
    #[serde(default)] pub checklist: Vec<String>,
}

/// A named arc section containing a list of beats.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RomanceBeatArcSection {
    #[serde(default)] pub description: Option<String>,
    #[serde(default)] pub beats: Vec<RomanceBeatEntry>,
}

/// Top-level catalog for `05-plot-and-structure/references/romance-beats.yaml`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RomanceBeatsCatalog {
    pub core_romance_arc: RomanceBeatArcSection,
    #[serde(default)] pub supplementary_beats: Option<RomanceBeatArcSection>,
}

// ── Phase VI–VII theory catalog structs ──────────────────────────────────────

/// Entry in `07-drafting/references/dramatic-theory-modes.yaml`
/// Aristotle *Poetics* (c. 335 BCE); Stanislavski *An Actor Prepares* (1936);
/// Brecht *Brecht on Theatre* (ed. Willett, 1964); Artaud *The Theatre and Its Double* (1938);
/// Syd Field *Screenplay* (1979)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct DramaticTheoryModeEntry {
    pub slug: String,
    /// "aristotle" | "stanislavski" | "brecht" | "artaud" | "field"
    pub category: String,
    pub name: String,
    #[serde(default)] pub definition: Option<String>,
    // Aristotle
    #[serde(default)] pub related_concepts: Vec<String>,
    // Stanislavski
    #[serde(default)] pub craft_principles: Vec<String>,
    // Brecht
    #[serde(default)] pub fiction_equivalents: Vec<String>,
    // Artaud
    #[serde(default)] pub fiction_translations: Vec<String>,
    #[serde(default)] pub when_to_deploy: Vec<String>,
    // shared
    #[serde(default)] pub audit_questions: Vec<String>,
    #[serde(default)] pub canonical_examples: Vec<CatalogExample>,
}

/// Entry in `07-drafting/references/short-fiction-modes.yaml`
/// Poe "The Philosophy of Composition" (1846); Charles E. May *The Short Story* (1995);
/// Julio Cortázar "Some Aspects of the Short Story" (1963);
/// Shapard & Thomas *Sudden Fiction* (1986)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct ShortFictionModeEntry {
    pub slug: String,
    /// "poe" | "may" | "cortazar" | "flash" | "moore_williams" | "collection"
    pub category: String,
    pub name: String,
    #[serde(default)] pub definition: Option<String>,
    #[serde(default)] pub craft_principles: Vec<String>,
    /// Serialized as JSON — contains `"novel"` and `"short_story"` keys
    #[serde(default)] pub may_distinction: Option<JsonValue>,
    /// Flash fiction-specific: omission strategies
    #[serde(default)] pub collection_strategies: Vec<String>,
    #[serde(default)] pub audit_questions: Vec<String>,
    #[serde(default)] pub canonical_examples: Vec<CatalogExample>,
}

/// Entry in `07-drafting/references/nonfiction-narrative-modes.yaml`
/// Vivian Gornick *The Situation and the Story* (2001); John McPhee *Draft No. 4* (2017);
/// Mary Karr *The Art of Memoir* (2015); Joan Didion (essays/memoir);
/// Phillip Lopate *To Show and to Tell* (2013)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct NonfictionNarrativeModeEntry {
    pub slug: String,
    /// "gornick" | "mcphee" | "karr" | "didion" | "lopate"
    pub category: String,
    pub name: String,
    #[serde(default)] pub definition: Option<String>,
    #[serde(default)] pub craft_principles: Vec<String>,
    /// McPhee's six structural types
    #[serde(default)] pub structural_repertoire: Vec<String>,
    /// Karr's five voice principles
    #[serde(default)] pub voice_principles: Vec<String>,
    /// Didion's five techniques
    #[serde(default)] pub techniques: Vec<String>,
    #[serde(default)] pub audit_questions: Vec<String>,
    #[serde(default)] pub canonical_examples: Vec<CatalogExample>,
}

/// Entry in `08-revision/references/intersectional-modes.yaml`
/// Kimberlé Crenshaw (1989, 1991); Patricia Hill Collins *Black Feminist Thought* (2000);
/// bell hooks *Feminist Theory: From Margin to Center* (1984); Chimamanda Ngozi Adichie
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct IntersectionalModeEntry {
    pub slug: String,
    /// "crenshaw" | "collins" | "hooks" | "failure_patterns"
    pub category: String,
    pub name: String,
    #[serde(default)] pub definition: Option<String>,
    // Crenshaw
    #[serde(default)] pub narrative_applications: Vec<String>,
    // Collins
    #[serde(default)] pub four_domains: Vec<String>,
    // failure_patterns
    #[serde(default)] pub signs_in_fiction: Vec<String>,
    #[serde(default)] pub fix: Option<String>,
    // shared
    #[serde(default)] pub audit_questions: Vec<String>,
    #[serde(default)] pub canonical_examples: Vec<CatalogExample>,
}

/// Entry in `07-drafting/references/screenwriting-modes.yaml`
/// Syd Field *Screenplay* (1979); Michael Hauge *Writing Screenplays That Sell* (1988/2011);
/// David Bordwell *Narration in the Fiction Film* (1985); Robert McKee *Story* (1997);
/// Paul Joseph Gulino *Screenwriting: The Sequence Approach* (2004);
/// Linda Seger *Making a Good Script Great* (3rd ed. 2010)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct ScreenwritingModeEntry {
    pub slug: String,
    /// "field" | "hauge" | "bordwell" | "mckee" | "gulino" | "seger"
    pub category: String,
    pub name: String,
    #[serde(default)] pub definition: Option<String>,
    // Field
    #[serde(default)] pub structural_proportions: Vec<String>,
    #[serde(default)] pub fiction_translation: Vec<String>,
    // Hauge
    #[serde(default)] pub stages: Vec<String>,
    #[serde(default)] pub turning_points: Vec<String>,
    #[serde(default)] pub identity_markers: Vec<String>,
    #[serde(default)] pub essence_markers: Vec<String>,
    // Bordwell
    #[serde(default)] pub craft_implications: Vec<String>,
    #[serde(default)] pub scale: Vec<String>,
    /// bordwell_suppressive_narration "cheat test" questions
    #[serde(default)] pub cheat_test: Vec<String>,
    #[serde(default)] pub examples_in_fiction: Vec<String>,
    // McKee
    #[serde(default)] pub craft_principles: Vec<String>,
    #[serde(default)] pub value_examples: Vec<String>,
    #[serde(default)] pub charge_tracking: Vec<String>,
    #[serde(default)] pub gap_types: Vec<String>,
    #[serde(default)] pub archplot_markers: Vec<String>,
    #[serde(default)] pub miniplot_markers: Vec<String>,
    #[serde(default)] pub antiplot_markers: Vec<String>,
    #[serde(default)] pub hierarchy: Vec<String>,
    // Gulino
    #[serde(default)] pub eight_sequences: Vec<String>,
    // Seger
    #[serde(default)] pub function_types: Vec<String>,
    #[serde(default)] pub warning_signs: Vec<String>,
    #[serde(default)] pub five_stages: Vec<String>,
    #[serde(default)] pub thematic_mirror_principle: Vec<String>,
    #[serde(default)] pub subplot_functions: Vec<String>,
    // shared
    #[serde(default)] pub audit_questions: Vec<String>,
    #[serde(default)] pub canonical_examples: Vec<CatalogExample>,
}

// ── Full catalog set ──────────────────────────────────────────────────────────

/// All loaded YAML catalogs, indexed by slug.
///
/// Built once from the `*/references/` directories and passed to validation
/// pipelines.  Replacing the previous `list[dict]` approach means unknown
/// keys cause a deserialization error at load time rather than silently
/// disappearing into `extra: dict`.
#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
pub struct CatalogSet {
    pub archetypes: HashMap<String, ArchetypeCatalogEntry>,
    pub wounds: HashMap<String, WoundCatalogEntry>,
    pub alignments: HashMap<String, AlignmentCatalogEntry>,
    pub roles: HashMap<String, RoleCatalogEntry>,
    pub drive_models: HashMap<String, DriveCatalogEntry>,
    pub plot_types: HashMap<String, PlotTypeCatalogEntry>,
    pub collision_patterns: HashMap<String, CollisionPatternCatalogEntry>,
    pub social_circles: HashMap<String, SocialCircleCatalogEntry>,
    pub tropes: HashMap<String, TropeCatalogEntry>,
    pub relationship_roles: HashMap<String, RelationshipRoleCatalogEntry>,
    pub inciting_incidents: HashMap<String, IncitingIncidentEntry>,
}

impl CatalogSet {
    pub fn validate_plot_type_slug(&self, slug: &str) -> bool {
        self.plot_types.contains_key(slug)
    }

    pub fn validate_collision_pattern_slug(&self, slug: &str) -> bool {
        self.collision_patterns.contains_key(slug)
    }

    pub fn validate_trope_slug(&self, slug: &str) -> bool {
        self.tropes.contains_key(slug)
    }
}
