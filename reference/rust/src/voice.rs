//! Voice contract and voice signature types.
//!
//! The `VoiceContract` encodes the author's stylistic choices from three
//! Phase 07 documents:
//! - `narrative-voice.md` — narrator identity, authority, reliability
//! - `prose-style-guide.md` — sentence craft, vocabulary, affect
//! - `focalization-guide.md` — Genette/Bal/Gardner/Cohn frameworks
//!
//! `VoiceSignature` is a **per-character** prose fingerprint — the key new
//! type that connects a `Character` entity to how their voice sounds in free
//! indirect discourse and dialogue. This is missing from the current system
//! and is critical for LLM voice fine-tuning.

use std::collections::HashMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::enums::{
    ConsciousnessMode, DiegeticLevel, EngagementMode, FocalizationType,
    GazeType, KnowledgeScope, NarrativeDistance, PactType, ProfanityLevel,
    PsychicDistance, ReliabilitySpectrum, SentenceLength, SentenceType,
    VocabularyLevel, VoiceStructure,
};
use crate::tags::EntityRef;

// ── Voice signature (per character) ──────────────────────────────────────────

/// The prose fingerprint for a single character.
///
/// Captures how *this character's* voice sounds when it bleeds into the
/// narrative — through free indirect discourse (FID), dialogue cadence,
/// recurring diction, and signature metaphors.  This is what a fine-tuned
/// LLM needs to distinguish voice between characters.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, Default)]
pub struct VoiceSignature {
    pub character_id: String,
    /// Words, phrases, or constructions that signal this character's FID
    pub fid_markers: Vec<String>,
    /// Characteristic verbal tics or speech patterns
    pub verbal_tics: Vec<String>,
    /// Default sentence length tendency
    pub sentence_length: Option<SentenceLength>,
    /// Domain of vocabulary (specialist diction, street slang, archaisms…)
    pub vocabulary_domain: Option<String>,
    /// Recurring metaphor or image clusters
    pub signature_metaphors: Vec<String>,
    /// Words or constructions this character would never use
    pub diction_exclusions: Vec<String>,
    /// Tonal register range: (minimum, maximum) on a free-text scale
    pub tonal_range: Option<(String, String)>,
    /// Typical psychic distance when this character is the focalizer
    pub default_psychic_distance: Option<PsychicDistance>,
    /// Consciousness mode used when rendering this character's interiority
    pub consciousness_mode: Option<ConsciousnessMode>,
    /// Free-text example of this character's most distinctive passage (seed
    /// text for fine-tuning)
    pub exemplar_passage: Option<String>,
}

// ── Per-character FID table entry ─────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct FidTableEntry {
    pub character: EntityRef,
    pub markers: Vec<String>,
    pub example: Option<String>,
}

// ── Tonal shift map ───────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct TonalShift {
    pub triggering_event: String,
    pub shift_description: String,
    pub prose_marker: Option<String>,
}

// ── Sentence type distribution ────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct SentenceTypeShare {
    pub sentence_type: SentenceType,
    /// Approximate percentage of sentences of this type (0–100)
    pub percentage: u8,
}

// ── Affect palette ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct AffectPaletteEntry {
    pub scene_type: String,
    pub target_affect: String,
    pub technique: String,
}

// ── Prose style guide ─────────────────────────────────────────────────────────

/// Sentence-level and paragraph-level craft specifications from
/// `07-drafting/voice-contract/prose-style-guide.md`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, Default)]
pub struct ProseStyleGuide {
    pub overall_voice_character: Option<String>,
    pub tonal_range: Option<String>,
    pub narrative_distance: Option<NarrativeDistance>,
    pub pov_type: Option<crate::enums::PovType>,
    pub tense: Option<String>,

    // Sentence craft
    pub default_sentence_length: Option<SentenceLength>,
    pub rhythm_pattern: Option<String>,
    pub paragraph_length_tendency: Option<String>,
    pub sentence_type_distribution: Vec<SentenceTypeShare>,
    pub default_sentence_construction: Option<SentenceType>,
    pub asyndeton_polysyndeton_policy: Option<String>,

    // Vocabulary
    pub vocabulary_level: Option<VocabularyLevel>,
    pub vocabulary_embrace: Vec<String>,
    pub vocabulary_avoid: Vec<String>,
    pub archaisms_policy: Option<String>,
    pub profanity: Option<ProfanityLevel>,

    // Affect and atmosphere
    pub affect_palette: Vec<AffectPaletteEntry>,
    pub dominant_minor_affect: Option<String>,
    pub never_feel_like: Vec<String>,
    pub primary_atmospheric_quality: Option<String>,
    pub atmosphere_techniques: Vec<String>,
    pub atmosphere_exception_zones: Vec<String>,

    // Oral performance
    pub sentence_architecture_default: Option<crate::enums::SentenceArchitecture>,
    pub max_pronoun_distance: Option<u8>,
}

// ── Narrative voice ────────────────────────────────────────────────────────────

/// Narrator-level configuration from
/// `07-drafting/voice-contract/narrative-voice.md`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, Default)]
pub struct NarrativeVoice {
    // Part 1 — narrator identity
    pub narrator_type: Option<String>,
    pub named_or_nameable: Option<bool>,
    pub temporal_position: Option<String>,
    pub existence_implies: Option<String>,

    // Part 2 — authority
    pub knowledge_scope: Option<KnowledgeScope>,
    pub how_claims_knowledge: Option<String>,
    pub suspicious_knowledge: Vec<String>,
    pub suspicious_gaps: Vec<String>,

    // Part 3 — reliability
    pub reliability: Option<ReliabilitySpectrum>,
    pub evidence_for_reliability: Vec<String>,
    pub gap_between_claims_and_evidence: Option<String>,
    pub resolvable: Option<bool>,

    // Part 4 — narrator–reader dynamic
    pub engagement_mode: Option<EngagementMode>,
    pub tonal_attitude: Option<String>,
    pub addresses_reader_directly: Option<bool>,
    pub assumed_reader_knowledge: Option<String>,

    // Part 5 — tonal range
    pub default_register: Option<String>,
    pub upper_range: Option<String>,
    pub lower_range: Option<String>,
    pub tonal_shifts: Vec<TonalShift>,
    pub violations_to_avoid: Vec<String>,

    // Part 6 — voice vs character FID (per-character table)
    pub narrator_signals: Vec<String>,
    pub character_fid_table: Vec<FidTableEntry>,

    // Part 7 — implied author
    pub implied_author_profile: Option<String>,
    pub matches_author_intention: Option<bool>,
}

// ── Focalization config ────────────────────────────────────────────────────────

/// Active focalization settings at document or scene level.
/// Drawn from `07-drafting/voice-contract/focalization-guide.md`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, Default)]
pub struct FocalizationConfig {
    pub focalization_type: Option<FocalizationType>,
    pub default_psychic_distance: Option<PsychicDistance>,
    pub consciousness_mode: Option<ConsciousnessMode>,
    pub diegetic_level: Option<DiegeticLevel>,
    pub voice_structure: Option<VoiceStructure>,
    pub pact_type: Option<PactType>,
    pub gaze_default: Option<GazeType>,
}

// ── Full voice contract ────────────────────────────────────────────────────────

/// The complete voice contract — bundles all three voice documents.
///
/// This is the "author fingerprint" object that gets serialized into every
/// `SceneContext` to give the fine-tuned LLM the stylistic constraints active
/// during authoring.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema, Default)]
pub struct VoiceContract {
    /// Project-level narrative voice specification
    pub narrative_voice: NarrativeVoice,
    /// Sentence-level and paragraph-level style guide
    pub prose_style: ProseStyleGuide,
    /// Focalization and POV configuration
    pub focalization: FocalizationConfig,
    /// Per-character voice signatures, keyed by character slug
    pub character_signatures: HashMap<String, VoiceSignature>,
}

impl VoiceContract {
    pub fn voice_signature(&self, character_slug: &str) -> Option<&VoiceSignature> {
        self.character_signatures.get(character_slug)
    }
}

// ── TTS voice profile ──────────────────────────────────────────────────────────

/// Audio voice profile for TTS/audiobook synthesis.
///
/// Mirrors the Python `VoiceProfile` dataclass from `scripts/voice_profiles.py`
/// but adds an explicit link to the `Character` entity.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct TtsVoiceProfile {
    /// Speaker directory name (maps to `voice-samples/<name>/`)
    pub name: String,
    /// Character entity this voice belongs to (optional — Narrator may not be a character)
    pub character_ref: Option<EntityRef>,
    pub language: String,
    /// Tone → relative path to reference WAV
    pub reference_wavs: HashMap<String, String>,
    pub fallback_tone: String,
    /// Path to a fine-tuned XTTS-v2 checkpoint, if one exists
    pub checkpoint: Option<String>,
}

impl Default for TtsVoiceProfile {
    fn default() -> Self {
        Self {
            name: String::new(),
            character_ref: None,
            language: "en".to_owned(),
            reference_wavs: HashMap::new(),
            fallback_tone: "neutral".to_owned(),
            checkpoint: None,
        }
    }
}

// ── Dialogue segment (typed port of parse_dialogue.py TextSegment) ─┅──────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct DialogueSegment {
    pub segment_type: crate::enums::SegmentType,
    pub text: String,
    pub speaker: String,
    pub voice: String,
    /// Speaker attribution confidence 0.0–1.0
    pub confidence: f32,
    pub line_number: u32,
    pub tone: crate::enums::VoiceTone,
    pub scene_index: u32,
    pub ambience: Option<String>,
    pub music: Option<String>,
    pub delivery: Option<String>,
    pub sfx: Option<String>,
    pub beat: Option<EntityRef>,
    pub start_time: f64,
    pub end_time: f64,
}
