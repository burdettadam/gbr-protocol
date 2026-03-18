//! Training example construction for voice fine-tuning.
//!
//! The central design decision: **annotations are inputs, not outputs**.
//! The LLM is trained to produce prose that satisfies the annotation
//! instructions given the full scene context.  The annotation tags that
//! were present when the author wrote a paragraph tell the model what
//! constraints were active — psychic distance level, motif to weave in,
//! tension target, active beat, etc.
//!
//! Training pair:
//! ```text
//! Input:  SceneContext  (structured scene planning data + voice contract
//!                        + resolved annotations as explicit fields)
//! Output: ProsePassage  (the authored text, optionally with paragraph
//!                        boundaries preserved)
//! ```

use std::collections::HashMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cap_narrative_types::entities::{Character, ProseDirectives, Scene, Setting, Motif};
use cap_narrative_types::enums::{Act, DominantSense, MotifStage, RevisionFlag};
use cap_narrative_types::tags::{AnnotationChannel, EntityRef, ParagraphAnnotations, SentenceAnnotations};
use cap_narrative_types::voice::{DialogueSegment, FocalizationConfig, VoiceContract, VoiceSignature};

// ── Tier configuration ────────────────────────────────────────────────────────

/// Controls which annotation tiers are included in training-example export.
///
/// The three-tier system lets experiments isolate different levels of
/// annotation richness without rebuilding the underlying data.
///
/// - **Tier 1** (core craft): always on — minimum set for voice fine-tuning.
/// - **Tier 2** (narrative structure): beat, motif, conflict, arc, temporal
///   order, reader effect — structural signals.
/// - **Tier 3** (theory enrichment): all theory-derived tags (Freudian
///   mechanism, postcolonial mode, etc.); enables literary-analysis tasks.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct TierConfig {
    /// Emit Tier 1 core craft tags (always recommended).
    pub tier1_core_craft: bool,
    /// Emit Tier 2 narrative-structure tags.
    pub tier2_narrative_structure: bool,
    /// Emit Tier 3 theory-enrichment tags.
    pub tier3_theory: bool,
    /// Emit sentence-level annotations within each paragraph.
    pub sentence_level: bool,
    /// Include the `ProseIntent` intermediate layer in the serialised output.
    pub include_intent: bool,
    /// Keep governance annotations (flags, source, TTS) in the output.
    pub include_governance: bool,
}

impl Default for TierConfig {
    fn default() -> Self {
        Self {
            tier1_core_craft: true,
            tier2_narrative_structure: true,
            tier3_theory: false,
            sentence_level: false,
            include_intent: true,
            include_governance: false,
        }
    }
}

impl TierConfig {
    /// A configuration for high-richness academic / analysis export.
    pub fn all() -> Self {
        Self {
            tier1_core_craft: true,
            tier2_narrative_structure: true,
            tier3_theory: true,
            sentence_level: true,
            include_intent: true,
            include_governance: true,
        }
    }

    /// Minimal config — Tier 1 only, no sentence level, no theory.
    pub fn minimal() -> Self {
        Self {
            tier1_core_craft: true,
            tier2_narrative_structure: false,
            tier3_theory: false,
            sentence_level: false,
            include_intent: false,
            include_governance: false,
        }
    }

    /// Whether the given channel should be included under this config.
    pub fn includes_channel(&self, channel: AnnotationChannel) -> bool {
        match channel {
            AnnotationChannel::Craft | AnnotationChannel::Context => self.tier1_core_craft,
            AnnotationChannel::Theory => self.tier3_theory,
            AnnotationChannel::Governance => self.include_governance,
        }
    }
}

// ── Prose intent (intermediate layer) ────────────────────────────────────────

/// A concrete target quality dimension for a prose unit.
///
/// Expresses a single annotation constraint as a named (dimension, value) pair
/// in natural-language-friendly terms, e.g.
/// `{ dimension: "psychic_distance", target: "1" }`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct QualityTarget {
    /// Annotation key / dimension name (matches the tag vocabulary key).
    pub dimension: String,
    /// Human-readable target value as a string.
    pub target: String,
    /// True if this target was automatically derived from tags (not authored).
    pub auto_derived: bool,
}

/// The narrative function a prose unit is serving.
///
/// Drawn from the active beat, scene function annotation, or inferred from
/// context.  Serves as a high-level label for the prose-intent description.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum NarrativeFunction {
    /// Establishing a scene or character state.
    Establishing,
    /// Escalating conflict or tension.
    Escalation,
    /// A moment of revelation or recognition.
    Revelation,
    /// Interior processing after a scene event (Swain's Sequel).
    InternalProcessing,
    /// Two characters in direct opposition.
    Confrontation,
    /// A moment of decision / commitment to a new goal.
    Decision,
    /// Delivering a plant or setup for later payoff.
    PlantOrSetup,
    /// Paying off an earlier plant.
    Payoff,
    /// Atmospheric scene-setting without forward narrative motion.
    Atmospheric,
    /// Transition between scenes or story beats.
    Transition,
    /// Dialogue exchange carrying subtext.
    SubtextDialogue,
    /// Character interiority / consciousness rendering.
    Interiority,
    /// Free-text narrative function not covered above.
    Custom(String),
}

/// The intermediate layer between annotation tags and output prose.
///
/// A `ProseIntent` translates the active tag-set into a natural-language
/// micro-instruction describing *what the prose unit should accomplish*.
/// This gives the fine-tuned model both structured signals (via tags) and a
/// semantic summary (via intent), improving generalisation.
///
/// Intent descriptions can be:
/// - **Template-generated**: built automatically from tag combinations at
///   export time.
/// - **Author-supplied**: written by hand when the template is insufficient.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct ProseIntent {
    /// Natural-language description of what this text unit should accomplish.
    pub instruction: String,
    /// Structured breakdown of the active quality targets.
    pub target_qualities: Vec<QualityTarget>,
    /// The narrative function this unit is fulfilling.
    pub function: Option<NarrativeFunction>,
    /// True when the instruction was auto-generated from tags rather than
    /// hand-written by the author.
    pub auto_generated: bool,
}

impl ProseIntent {
    /// Build a template-generated intent from the active tier-1 craft tags on
    /// a paragraph.  Returns `None` when there are no relevant tags.
    pub fn from_paragraph_annotations(ann: &ParagraphAnnotations) -> Option<Self> {
        let mut targets = Vec::new();
        let mut parts: Vec<String> = Vec::new();

        if let Some(pd) = &ann.psychic_distance {
            let pd_u8: u8 = pd.value();
            targets.push(QualityTarget {
                dimension: "psychic_distance".into(),
                target: pd_u8.to_string(),
                auto_derived: true,
            });
            let label = match pd_u8 {
                1 => "close interiority",
                2 => "near-close narration",
                3 => "mid-range narration",
                4 => "distant narration",
                5 => "panoramic/summary narration",
                _ => "narration",
            };
            parts.push(format!("Write in {label}"));
        }

        if let Some(c) = &ann.consciousness {
            targets.push(QualityTarget {
                dimension: "consciousness".into(),
                target: c.to_string(),
                auto_derived: true,
            });
            parts.push(format!("using {} mode", c));
        }

        if let Some(t) = &ann.tension {
            targets.push(QualityTarget {
                dimension: "tension".into(),
                target: t.value().to_string(),
                auto_derived: true,
            });
            let level = if t.value() >= 8 {
                "high tension"
            } else if t.value() >= 5 {
                "moderate tension"
            } else {
                "low tension"
            };
            parts.push(format!("at {level}"));
        }

        if let Some(s) = &ann.subtext {
            targets.push(QualityTarget {
                dimension: "subtext".into(),
                target: s.clone(),
                auto_derived: true,
            });
            parts.push(format!("with subtext: {s}"));
        }

        if parts.is_empty() {
            return None;
        }

        Some(Self {
            instruction: parts.join(", ") + ".",
            target_qualities: targets,
            function: None,
            auto_generated: true,
        })
    }
}



/// Snapshot of a character's narrative state at the start of a scene.
///
/// This lets the model understand where each character *is* in their arc —
/// not just their general biographical profile — which is crucial for
/// voice consistency across a full draft.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct CharacterState {
    pub character_ref: EntityRef,
    /// Static profile (archetype, wound, alignment, etc.)
    pub profile: Character,
    /// Current position in the character's arc (0.0 = start, 1.0 = end)
    pub arc_position: Option<f32>,
    /// Dominant emotional state at the start of this scene
    pub emotional_state: Option<String>,
    /// Active wound behaviour being triggered by this scene
    pub active_wound_behaviour: Option<String>,
    /// Relationship status with every other attending character
    pub relationship_states: HashMap<String, String>,
    /// Which drive is currently dominant for this character
    pub dominant_drive: Option<cap_narrative_types::enums::DriveModel>,
    /// This character's voice signature (copied from VoiceContract for convenience)
    pub voice_signature: Option<VoiceSignature>,
}

// ── World texture ─────────────────────────────────────────────────────────────

/// Snapshot of relevant world-building context active at scene time.
///
/// Drawn from Phase 04 templates so the model has sensory grounding.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct WorldTexture {
    pub setting: Option<Setting>,
    /// Active motifs in this scene (slug → current stage)
    pub active_motifs: Vec<ActiveMotif>,
    /// Any setting-specific sensory details worth foregrounding
    pub foregrounded_sensory: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct ActiveMotif {
    pub motif: Motif,
    pub current_stage: MotifStage,
    pub instruction: Option<String>,
}

// ── Scene context (full structured input) ────────────────────────────────────

/// The complete structured input for prose generation.
///
/// This is what gets serialised as the `input` side of a `TrainingExample`.
/// Every piece of planning information the author had access to when writing
/// the prose is captured here.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct SceneContext {
    // ── Scene planning data
    pub scene: Scene,

    // ── Voice contract (author fingerprint)
    pub voice_contract: VoiceContract,

    // ── Active focalization settings for this scene
    /// Note: can override VoiceContract defaults for this specific scene
    pub focalization: FocalizationConfig,

    // ── Active annotation instructions
    /// Paragraph-level annotation state resolved from the Markdown at the
    /// start of each prose block.  Ordered to match the paragraph sequence
    /// in the output ProsePassage.
    pub paragraph_annotations: Vec<ParagraphAnnotations>,

    // ── World texture
    pub world_texture: WorldTexture,

    // ── Character states
    pub character_states: Vec<CharacterState>,

    // ── Narrative macro-context
    /// Story position as a fraction 0.0–1.0
    pub story_position: Option<f32>,
    pub act: Option<Act>,
    /// The structural beat this scene is fulfilling
    pub beat_ref: Option<EntityRef>,
    /// Chapter reference
    pub chapter_ref: Option<EntityRef>,
    /// The N preceding prose paragraphs (continuity window)
    pub preceding_context: Option<String>,
    /// Summary of events in the preceding scene
    pub preceding_scene_summary: Option<String>,

    // ── Training export configuration
    /// Controls which annotation tiers and granularity levels are emitted
    /// when this context is serialised into a training example.  Defaults to
    /// [`TierConfig::default`] (Tier 1 + Tier 2, no sentence level, no theory).
    pub tier_config: TierConfig,

    // ── Prose directives (scene-card level voice override)
    /// Scene-level voice and style constraints loaded from the GBR scene card's
    /// `prose_directives` section. Overrides model default voice with concrete
    /// mechanical rules (sentence mechanics, anti-patterns, diction, etc.).
    /// When present, this takes precedence over `scene.prose_directives`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prose_directives: Option<ProseDirectives>,
}

// ── Sentence unit ────────────────────────────────────────────────────────────

/// A single sentence with its resolved annotation context.
///
/// Sentence-level granularity is the finest unit in the multi-level hierarchy:
/// Scene → Paragraph → Sentence.  It is optional — controlled by
/// [`TierConfig::sentence_level`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Sentence {
    /// 0-based index within the parent paragraph.
    pub index: u32,
    pub text: String,
    pub word_count: u32,
    /// Sentence-grain annotation state (craft techniques at this grain).
    pub annotations: SentenceAnnotations,
    /// Prose intent for this sentence (present when
    /// [`TierConfig::include_intent`] is active).
    pub intent: Option<ProseIntent>,
}

// ── Prose paragraph unit ─────────────────────────────────────────────────────

/// A single paragraph with its resolved annotation context.
///
/// The `sentences` field is populated only when
/// [`TierConfig::sentence_level`] is enabled.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct Paragraph {
    pub index: u32,
    pub text: String,
    pub word_count: u32,
    /// Annotation state active at the start of this paragraph.
    pub annotations: ParagraphAnnotations,
    /// Prose intent describing what this paragraph should accomplish.
    /// Present when [`TierConfig::include_intent`] is active.
    pub intent: Option<ProseIntent>,
    /// Whether this paragraph contains or is part of a dialogue exchange.
    pub contains_dialogue: bool,
    /// Sentence-level breakdown.  Populated only when
    /// [`TierConfig::sentence_level`] is enabled.
    pub sentences: Vec<Sentence>,
}

// ── Prose passage (output) ────────────────────────────────────────────────────

/// The authored prose output — the target side of a `TrainingExample`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct ProsePassage {
    /// Full raw text (the plain prose, no annotation comments)
    pub text: String,
    /// Paragraphs with their resolved annotation state preserved
    pub paragraphs: Vec<Paragraph>,
    /// Extracted dialogue segments (for audiobook pipeline)
    pub dialogue_segments: Vec<DialogueSegment>,
    pub word_count: u32,
    pub dominant_sense: Option<DominantSense>,
}

// ── Training metadata ─────────────────────────────────────────────────────────

/// Metadata attached to a training example.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct TrainingMeta {
    pub chapter_id: Option<String>,
    pub chapter_number: Option<u32>,
    pub scene_id: Option<String>,
    pub word_count: u32,
    /// Revision pass that produced this draft (0 = first draft)
    pub revision_pass: u8,
    /// Author session notes that accompanied this draft
    pub session_notes: Option<String>,
    /// Any revision flags that were later identified on this passage
    pub post_hoc_flags: Vec<RevisionFlag>,
    /// Source document path relative to workspace root
    pub source_path: Option<String>,
}

// ── Training example ──────────────────────────────────────────────────────────

/// A single fine-tuning data point.
///
/// Serialise to JSONL via `serde_json` for ingestion into HuggingFace
/// `datasets`, Axolotl, or any standard LLM fine-tuning pipeline.
///
/// The canonical field names align with Alpaca/ShareGPT conventions so
/// the same JSONL can be used without field remapping:
/// - `instruction` = natural-language task description
/// - `input` = structured context (JSON)
/// - `output` = the authored prose
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct TrainingExample {
    /// Stable unique ID: `{chapter_id}_{scene_id}_{para_range}`
    pub id: String,
    /// Task description for the LLM
    pub instruction: String,
    /// Full structured scene context as the model input
    pub context: SceneContext,
    /// The authored prose output
    pub output: ProsePassage,
    /// Metadata (not fed to model, used for filtering/analysis)
    pub meta: TrainingMeta,
}

impl TrainingExample {
    /// Serialise this example as a single JSONL line (Alpaca-compatible).
    pub fn to_jsonl(&self) -> serde_json::Result<String> {
        #[derive(Serialize)]
        struct AlpacaLine<'a> {
            id: &'a str,
            instruction: &'a str,
            input: serde_json::Value,
            output: &'a str,
        }
        let input_json = serde_json::to_value(&self.context)?;
        let line = AlpacaLine {
            id: &self.id,
            instruction: &self.instruction,
            input: input_json,
            output: &self.output.text,
        };
        serde_json::to_string(&line)
    }

    /// Default instruction string used when building training examples from
    /// the workspace.  Can be overridden per example.
    pub fn default_instruction(scene: &Scene) -> String {
        let title = scene.working_title.as_deref().unwrap_or("Untitled scene");
        format!(
            "Write a prose passage for the scene '{title}'. \
             Follow the voice contract, honour all active annotation \
             constraints in the context, and write in the author's \
             established style."
        )
    }
}

// ── Training dataset ──────────────────────────────────────────────────────────

/// A collection of training examples with basic statistics.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TrainingDataset {
    pub name: String,
    pub examples: Vec<TrainingExample>,
    pub total_word_count: u32,
    pub example_count: usize,
    pub chapters_covered: Vec<String>,
    pub scenes_covered: Vec<String>,
}

impl TrainingDataset {
    pub fn new(name: impl Into<String>, examples: Vec<TrainingExample>) -> Self {
        let total_word_count = examples.iter().map(|e| e.output.word_count).sum();
        let example_count = examples.len();
        let chapters_covered: Vec<String> = examples
            .iter()
            .filter_map(|e| e.meta.chapter_id.clone())
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();
        let scenes_covered: Vec<String> = examples
            .iter()
            .filter_map(|e| e.meta.scene_id.clone())
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();
        Self {
            name: name.into(),
            examples,
            total_word_count,
            example_count,
            chapters_covered,
            scenes_covered,
        }
    }

    /// Iterate over JSONL lines for writing to a `.jsonl` file.
    pub fn to_jsonl_lines(&self) -> impl Iterator<Item = serde_json::Result<String>> + '_ {
        self.examples.iter().map(|e| e.to_jsonl())
    }
}
