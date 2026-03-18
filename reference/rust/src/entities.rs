//! Core declared entities and their registries.
//!
//! These are the named things that exist in the story world and are referenced
//! throughout phases via `<!-- key:slug -->` annotation tags.  Each entity
//! type has an explicit schema; there are no `extra: dict` catch-alls.
//!
//! **Entity registry** (`EntityRegistry`) is a typed, multi-kind store loaded
//! from the filled template files.  All `EntityRef` slugs are validated against
//! it at import time.

use std::collections::HashMap;

use indexmap::IndexMap;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::enums::{
    Act, Actant, Alignment, ArcType, Archetype, ConflictType,
    DominantSense, DriveModel, MotifStage, OutcomeType, Role, SceneFunction,
    SceneType, TimeOfDay, Weather, Wound,
};
use crate::tags::EntityRef;
use crate::voice::VoiceSignature;

// ── Interpreted value wrapper ────────────────────────────────────────────────

/// Wrapper for interpretation fields that carries optional confidence metadata.
///
/// Matches the v0.2.0 `interpreted_value` pattern in the JSON Schema:
/// a field can be either a bare value or an object with `value`, `confidence`,
/// and `source`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum InterpretedValue<T> {
    /// Simple value (no metadata)
    Plain(T),
    /// Value with confidence and source metadata
    WithMetadata {
        value: T,
        #[serde(skip_serializing_if = "Option::is_none")]
        confidence: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        source: Option<String>,
    },
}

impl<T> InterpretedValue<T> {
    /// Returns a reference to the inner value regardless of variant.
    pub fn value(&self) -> &T {
        match self {
            InterpretedValue::Plain(v) => v,
            InterpretedValue::WithMetadata { value, .. } => value,
        }
    }
}

// ── Character ─────────────────────────────────────────────────────────────────

/// Observable fields for a character entity (v0.2.0).
///
/// Facts directly recoverable from the text's surface.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CharacterObservables {
    /// Snake-case identifier used in `<!-- character:slug -->` tags
    pub id: String,
    pub name: String,
    pub slot: Option<String>,
}

/// Structural fields for a character entity (v0.2.0).
///
/// Authorial organizational choices rather than textual facts.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CharacterStructure {
    pub role: Option<Role>,
    /// Voice fingerprint — critical for LLM fine-tuning
    pub voice_signature: Option<VoiceSignature>,
}

/// Interpretation fields for a character entity (v0.2.0).
///
/// Analytical judgments that require inference or critical frameworks.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CharacterInterpretations {
    pub archetype: Option<InterpretedValue<Archetype>>,
    pub wound: Option<InterpretedValue<Wound>>,
    pub alignment: Option<InterpretedValue<Alignment>>,
    pub drive_model: Option<InterpretedValue<DriveModel>>,
    pub arc_type: Option<InterpretedValue<ArcType>>,
    /// Greimas actantial role in the story
    pub actant: Option<InterpretedValue<Actant>>,
    /// Free-text description of the character's ghost / internal wound origin
    pub ghost: Option<String>,
    /// Free-text description of the character's want (external goal)
    pub want: Option<String>,
    /// Free-text description of the character's need (thematic truth)
    pub need: Option<String>,
    /// Free-text description of the character's flaw
    pub flaw: Option<String>,
}

/// A declared character entity (Phase 03), v0.2.0 three-tier structure.
///
/// Fields are organized into observable (surface facts), structural
/// (authorial organization), and interpretive (analytical judgments)
/// sections per the epistemic-section architecture (ADR-006).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Character {
    pub observables: CharacterObservables,
    pub structure: CharacterStructure,
    pub interpretations: CharacterInterpretations,
}

impl Character {
    pub fn entity_ref(&self) -> EntityRef {
        EntityRef::new(&self.observables.id)
    }
}

// ── Relationship ──────────────────────────────────────────────────────────────

/// Observable fields for a relationship entity (v0.2.0).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RelationshipObservables {
    pub source: EntityRef,
    pub target: EntityRef,
}

/// Structural fields for a relationship entity (v0.2.0).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RelationshipStructure {
    pub rel_type: String,
}

/// Interpretation fields for a relationship entity (v0.2.0).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct RelationshipInterpretations {
    pub description: Option<String>,
    pub dynamic_at_start: Option<String>,
    pub dynamic_at_end: Option<String>,
    pub power_balance: Option<String>,
}

/// A declared relationship between two characters, v0.2.0 three-tier structure.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Relationship {
    pub observables: RelationshipObservables,
    pub structure: RelationshipStructure,
    pub interpretations: RelationshipInterpretations,
}

// ── Setting ───────────────────────────────────────────────────────────────────

/// Observable fields for a setting entity (v0.2.0).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct SettingObservables {
    pub id: String,
    pub name: String,
}

/// Structural fields for a setting entity (v0.2.0).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct SettingStructure {
    /// Setting category (e.g. interior_domestic, exterior_public)
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub setting_type: Option<String>,
}

/// Interpretation fields for a setting entity (v0.2.0).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct SettingInterpretations {
    pub general_vibe: Option<String>,
    pub sensory: Option<SensoryPalette>,
    /// Three go-to sensory details that define this place
    pub sensory_signature: Option<[String; 3]>,
}

/// A declared setting / location entity (Phase 04), v0.2.0 three-tier structure.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Setting {
    pub observables: SettingObservables,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub structure: Option<SettingStructure>,
    pub interpretations: SettingInterpretations,
}

impl Setting {
    pub fn entity_ref(&self) -> EntityRef {
        EntityRef::new(&self.observables.id)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct SensoryPalette {
    pub visual: Option<VisualDetails>,
    pub sounds: Option<SoundDetails>,
    pub smells: Option<SmellDetails>,
    pub textures: Option<TextureDetails>,
    pub tastes: Option<TasteDetails>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct VisualDetails {
    pub light_quality: Option<String>,
    pub dominant_colors: Vec<String>,
    pub movement: Option<String>,
    pub visual_textures: Option<String>,
    pub points_of_focus: Option<String>,
    pub contrast: Option<String>,
    pub by_time_of_day: IndexMap<TimeOfDay, String>,
    pub by_weather: IndexMap<Weather, String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct SoundDetails {
    pub ambient_soundscape: Option<String>,
    pub specific_sounds: Vec<String>,
    pub volume_and_rhythm: Option<String>,
    pub human_voices: Option<String>,
    pub silence_qualities: Option<String>,
    pub by_time_of_day: IndexMap<TimeOfDay, String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct SmellDetails {
    pub dominant_scent: Option<String>,
    pub layered_scents: Vec<String>,
    pub pleasant_vs_unpleasant: Option<String>,
    pub olfactory_associations: Option<String>,
    pub by_season_or_weather: IndexMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct TextureDetails {
    pub surfaces: Vec<String>,
    pub temperature: Option<String>,
    pub air_quality: Option<String>,
    pub typical_contact: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct TasteDetails {
    pub food_drink: Option<String>,
    pub air_taste: Option<String>,
    pub common_flavors: Option<String>,
}

// ── Beat ──────────────────────────────────────────────────────────────────────

/// A structural beat (Phase 05).  Replaces the Python `Beat` dataclass.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Beat {
    pub id: String,
    pub label: String,
    pub description: Option<String>,
    pub act: Option<Act>,
    pub order: u32,
    /// Approximate story position as a fraction 0.0–1.0
    pub story_position: Option<f32>,
    pub associated_threads: Vec<EntityRef>,
}

impl Beat {
    pub fn entity_ref(&self) -> EntityRef {
        EntityRef::new(&self.id)
    }
}

// ── Motif / Symbol ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct MotifAppearance {
    pub chapter: Option<String>,
    pub scene: Option<EntityRef>,
    pub context: String,
    pub meaning_shift: Option<String>,
}

/// A declared motif entity (Phase 04 motif tracker).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Motif {
    pub id: String,
    pub associated_theme: Option<String>,
    pub first_appearance: Option<MotifAppearance>,
    pub subsequent_appearances: Vec<MotifAppearance>,
    pub evolution: Option<String>,
    pub final_payoff: Option<String>,
    pub cliche_risk: Option<String>,
    pub current_stage: Option<MotifStage>,
}

impl Motif {
    pub fn entity_ref(&self) -> EntityRef {
        EntityRef::new(&self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Symbol {
    pub id: String,
    pub object_or_image: String,
    pub literal_meaning: Option<String>,
    pub symbolic_meaning: Option<String>,
    pub key_scenes: Vec<EntityRef>,
    pub when_established: Option<String>,
}

impl Symbol {
    pub fn entity_ref(&self) -> EntityRef {
        EntityRef::new(&self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Leitmotif {
    pub id: String,
    pub character: EntityRef,
    pub associated_element: String,
    pub introduced_in: Option<String>,
    pub appears_in: Vec<String>,
    pub emotional_function: Option<String>,
}

// ── Thread / Subplot ──────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Thread {
    pub id: String,
    pub label: String,
    pub characters: Vec<EntityRef>,
    pub description: Option<String>,
}

impl Thread {
    pub fn entity_ref(&self) -> EntityRef {
        EntityRef::new(&self.id)
    }
}

// ── Promise (hermeneutic code) ─────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Promise {
    pub id: String,
    pub question: String,
    pub phase: Option<crate::enums::PromisePhase>,
    pub status: crate::enums::PromiseStatus,
    pub planted_in: Option<EntityRef>,
    pub paid_off_in: Option<EntityRef>,
}

impl Promise {
    pub fn entity_ref(&self) -> EntityRef {
        EntityRef::new(&self.id)
    }
}

// ── Scene ─────────────────────────────────────────────────────────────────────

/// An individual scene card (Phase 06 scene template).
///
/// Previously untyped — only existed as Markdown.  Now every field from
/// `06-scenes/scene-template.md` has an explicit typed slot.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Scene {
    pub id: String,
    pub working_title: Option<String>,
    /// Numeric or string story position (act/chapter/percentage)
    pub story_position: Option<String>,
    pub pov_character: Option<EntityRef>,
    pub attending_characters: Vec<EntityRef>,
    pub setting: Option<EntityRef>,
    pub time_of_day: Option<String>,
    pub weather: Option<String>,
    // Scene proper
    pub goal: Option<String>,
    pub why_goal_matters: Option<String>,
    pub plan: Option<String>,
    pub opponent_or_obstacle: Option<String>,
    pub conflict_type: Vec<ConflictType>,
    pub escalation_beats: Vec<String>,
    pub dialogue_strategy: Option<String>,
    pub action_strategy: Option<String>,
    pub emotional_escalation: Option<String>,
    pub outcome_type: Option<OutcomeType>,
    pub what_changed: Option<SceneChanges>,
    pub new_information: Option<String>,
    pub plant_or_setup: Option<String>,
    // Sequel
    pub sequel: Option<SceneSequel>,
    // Sensory grounding
    pub dominant_sense: Option<DominantSense>,
    pub key_sensory_details: Vec<String>,
    pub emotional_weather: Option<String>,
    pub scene_unique_image: Option<String>,
    pub pacing_notes: Option<String>,
    // Meta
    pub target_word_count: Option<u32>,
    pub complexity: Option<crate::enums::Complexity>,
    pub priority: Option<crate::enums::Priority>,
    pub narrative_threads: Vec<EntityRef>,
    // Sequence context (set when scene is resolved within a sequence)
    pub sequence_id: Option<EntityRef>,
    pub scene_type: Option<SceneType>,
    pub tension_level: Option<u8>,
    /// Scene-level voice and style constraints for prose generation.
    /// Supplements the book-level voice contract with concrete mechanical rules
    /// that override model default voice. Designed to produce model-agnostic prose.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prose_directives: Option<ProseDirectives>,
}

// ── Prose Directives ──────────────────────────────────────────────────────────

/// Scene-level voice and style constraints for prose generation.
///
/// Supplements the book-level voice contract with concrete mechanical rules
/// that override model default voice. Each sub-section targets a specific
/// axis of prose control: sentence rhythm, word choice, figurative language
/// density, known anti-patterns to suppress, and scene-specific calibration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct ProseDirectives {
    /// Relative path to the book-level voice contract or prose style guide.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_contract_ref: Option<String>,
    /// Sentence-level rhythm and construction rules.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sentence_mechanics: Option<SentenceMechanics>,
    /// Word-level constraints on register, vocabulary, and sensory focus.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diction: Option<Diction>,
    /// Controls on figurative language density.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metaphor_budget: Option<MetaphorBudget>,
    /// Explicit model-default patterns to suppress.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub anti_patterns: Vec<AntiPattern>,
    /// Scene-specific prose ratio targets.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scene_calibration: Option<SceneCalibration>,
    /// Plain-language positive instructions for prose generation.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dos: Vec<String>,
    /// Plain-language prohibitions for prose generation.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub donts: Vec<String>,
    /// 3–5 sentences demonstrating the target voice.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub exemplar_lines: Vec<String>,
    /// 3–5 sentences demonstrating what to avoid.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub anti_exemplar_lines: Vec<String>,
}

/// Sentence-level rhythm and construction rules.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct SentenceMechanics {
    /// Target average sentence length (short / medium / medium-long / long / varied).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avg_sentence_length: Option<String>,
    /// Prose rhythm contract describing the expected sentence flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rhythm_pattern: Option<String>,
    /// Maximum sentences in a row starting with the same word/pronoun.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_consecutive_same_opener: Option<u8>,
    /// Ordered punctuation preference (e.g. semicolons, commas, em-dashes-selective).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub punctuation_preferences: Vec<String>,
    /// How freely sentence fragments may be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fragment_policy: Option<String>,
}

/// Word-level constraints on register, vocabulary, and sensory focus.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Diction {
    /// Target diction register (e.g. formal-literary-regency).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub register: Option<String>,
    /// Ordered sensory channels to emphasize.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sensory_priority: Vec<String>,
    /// Word families and diction categories to favor.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub embrace: Vec<String>,
    /// Word families and diction categories to suppress.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub avoid: Vec<String>,
}

/// Controls on figurative language density.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct MetaphorBudget {
    /// Overall metaphor density target (spare / moderate / rich).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub density: Option<String>,
    /// Maximum extended metaphorical conceits per scene.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_extended_conceits: Option<u8>,
    /// Figurative language types to avoid.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub prohibited_types: Vec<String>,
}

/// A named anti-pattern with detection description and correction instruction.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct AntiPattern {
    /// Short label for the anti-pattern (e.g. "aphoristic_excess").
    pub pattern: String,
    /// What the pattern looks like in generated prose.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Concrete correction rule for the model to follow.
    pub instruction: String,
}

/// Scene-specific prose ratio targets.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct SceneCalibration {
    /// Target ratio as "X:Y" (e.g. "40:60" = 40% dialogue, 60% interiority).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dialogue_interiority_ratio: Option<String>,
    /// Target show:tell ratio as "X:Y".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_tell_ratio: Option<String>,
    /// How the scene should close (e.g. "concrete-object", "action-beat").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scene_ending_type: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct SceneChanges {
    pub plot: Option<String>,
    pub character: Option<String>,
    pub stakes: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct SceneSequel {
    pub immediate_emotional_response: Option<String>,
    pub physical_reaction: Option<String>,
    pub processing_duration: Option<String>,
    pub the_problem: Option<String>,
    pub options: Vec<SequelOption>,
    pub why_difficult: Option<String>,
    pub internal_conflict: Option<String>,
    pub choice: Option<String>,
    pub why_this_choice: Option<String>,
    pub new_goal: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct SequelOption {
    pub label: String,
    pub description: Option<String>,
    pub consequence: Option<String>,
}

/// Complexity rating (re-export for use in scene fields).
pub use crate::enums::Complexity;

/// Priority (re-export for scene field use).
pub use crate::enums::Priority;

impl Scene {
    pub fn entity_ref(&self) -> EntityRef {
        EntityRef::new(&self.id)
    }
}

// ── Scene sequence ─────────────────────────────────────────────────────────────

/// A scene sequence (Phase 06 sequence planner).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct SceneSequence {
    pub id: String,
    pub name: String,
    pub story_position: Option<String>,
    pub function: Vec<SceneFunction>,
    pub central_question: Option<String>,
    pub tension_level: Option<crate::enums::TensionLevel>,
    pub arc_pattern: Option<crate::enums::ArcPattern>,
    /// Ordered list of scene IDs within this sequence
    pub scenes: Vec<SequenceScene>,
    // Analysis
    pub tension_progression: Option<String>,
    pub causality_chain: Option<String>,
    pub emotional_arc_start: Option<String>,
    pub emotional_arc_end: Option<String>,
    pub transition_in: Option<String>,
    pub transition_out: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct SequenceScene {
    pub scene_ref: EntityRef,
    pub scene_type: Option<SceneType>,
    pub goal: Option<String>,
    pub outcome: Option<String>,
    pub tension: Option<u8>,
    pub causality_from_previous: Option<String>,
    pub causality_to_next: Option<String>,
    pub key_function: Option<String>,
    pub threads: Vec<EntityRef>,
}

// ── Chapter ───────────────────────────────────────────────────────────────────

/// A chapter (Phase 07 active-drafting chapter-draft-template).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Chapter {
    pub id: String,
    pub number: u32,
    pub title: Option<String>,
    pub word_count_target: Option<u32>,
    pub pov_characters: Vec<EntityRef>,
    pub time_and_setting: Option<String>,
    pub story_position_pct: Option<f32>,
    pub act_section: Option<String>,
    pub beats_covered: Vec<EntityRef>,
    /// Scene breakdown within the chapter
    pub scenes: Vec<ChapterScene>,
    // Goals
    pub plot_goals: Vec<String>,
    pub character_goals: Vec<String>,
    pub information_goals: Vec<String>,
    pub thematic_goals: Vec<String>,
    // Constraints
    pub must_include: Vec<String>,
    pub must_avoid: Vec<String>,
    pub continuity_notes: Vec<String>,
    // Post-draft assessment
    pub assessment: Option<ChapterAssessment>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct ChapterScene {
    pub goal: String,
    pub conflict: Option<String>,
    pub outcome: Option<String>,
    pub transition: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct ChapterAssessment {
    pub word_count_actual: Option<u32>,
    pub opening_hook: Option<crate::enums::CraftRating>,
    pub ending: Option<crate::enums::CraftRating>,
    pub pacing: Option<crate::enums::CraftRating>,
    pub tension: Option<crate::enums::CraftRating>,
    pub dialogue: Option<crate::enums::CraftRating>,
    pub description: Option<crate::enums::CraftRating>,
    pub pov_consistency: Option<crate::enums::CraftRating>,
    pub style_compliance: Option<crate::enums::CraftRating>,
    pub problem_areas: Vec<ProblemArea>,
    pub discoveries: Option<String>,
    pub revision_flags: Vec<crate::enums::RevisionFlag>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct ProblemArea {
    pub lines: String,
    pub description: String,
}

// ── Entity Registry ───────────────────────────────────────────────────────────

/// The typed, multi-kind entity registry.
///
/// All entity refs are validated against this at import time.  The registry is
/// built once (from filled template files) and then passed into every engine
/// function and training example builder.
#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
pub struct EntityRegistry {
    pub characters: HashMap<String, Character>,
    pub settings: HashMap<String, Setting>,
    pub beats: HashMap<String, Beat>,
    pub motifs: HashMap<String, Motif>,
    pub symbols: HashMap<String, Symbol>,
    pub leitmotifs: HashMap<String, Leitmotif>,
    pub threads: HashMap<String, Thread>,
    pub promises: HashMap<String, Promise>,
    pub scenes: HashMap<String, Scene>,
    pub sequences: HashMap<String, SceneSequence>,
    pub chapters: HashMap<String, Chapter>,
}

impl EntityRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Validate an entity ref, returning `None` if the slug doesn't resolve.
    pub fn resolve_character(&self, r: &EntityRef) -> Option<&Character> {
        self.characters.get(&r.slug)
    }

    pub fn resolve_setting(&self, r: &EntityRef) -> Option<&Setting> {
        self.settings.get(&r.slug)
    }

    pub fn resolve_beat(&self, r: &EntityRef) -> Option<&Beat> {
        self.beats.get(&r.slug)
    }

    pub fn resolve_motif(&self, r: &EntityRef) -> Option<&Motif> {
        self.motifs.get(&r.slug)
    }

    pub fn resolve_leitmotif(&self, r: &EntityRef) -> Option<&Leitmotif> {
        self.leitmotifs.get(&r.slug)
    }

    pub fn resolve_scene(&self, r: &EntityRef) -> Option<&Scene> {
        self.scenes.get(&r.slug)
    }

    pub fn resolve_chapter(&self, r: &EntityRef) -> Option<&Chapter> {
        self.chapters.get(&r.slug)
    }

    /// Collect all dangling entity refs — refs that appear in fields but have
    /// no corresponding registered entity.  Returns a list of error strings.
    pub fn validate_refs(&self) -> Vec<String> {
        let mut errors = Vec::new();

        for scene in self.scenes.values() {
            if let Some(ref pov) = scene.pov_character {
                if self.resolve_character(pov).is_none() {
                    errors.push(format!(
                        "scene '{}': pov_character '{}' not in registry",
                        scene.id, pov.slug
                    ));
                }
            }
            for c in &scene.attending_characters {
                if self.resolve_character(c).is_none() {
                    errors.push(format!(
                        "scene '{}': attending character '{}' not in registry",
                        scene.id, c.slug
                    ));
                }
            }
            if let Some(ref s) = scene.setting {
                if self.resolve_setting(s).is_none() {
                    errors.push(format!(
                        "scene '{}': setting '{}' not in registry",
                        scene.id, s.slug
                    ));
                }
            }
        }

        for chapter in self.chapters.values() {
            for c in &chapter.pov_characters {
                if self.resolve_character(c).is_none() {
                    errors.push(format!(
                        "chapter '{}': pov character '{}' not in registry",
                        chapter.id, c.slug
                    ));
                }
            }
            for b in &chapter.beats_covered {
                if self.resolve_beat(b).is_none() {
                    errors.push(format!(
                        "chapter '{}': beat '{}' not in registry",
                        chapter.id, b.slug
                    ));
                }
            }
        }

        errors
    }
}
