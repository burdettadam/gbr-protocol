//! Strongly-typed annotation system.
//!
//! Grimoire uses inline `<!-- key:value -->` HTML comment annotations in
//! Markdown to tag prose passages with structured metadata.  This module
//! replaces the read-regex-parse-string pipeline with a typed `Annotation`
//! enum so that references, enums, and numeric ranges are all validated.
//!
//! **Placement rule**: an annotation at the start of a paragraph governs
//! that paragraph and holds until the next annotation of the same type
//! overrides it.
//!
//! **Annotation as input**: in the voice-fine-tuning pipeline, all annotations
//! on a passage are bundled into `SceneContext.annotations` so the model sees
//! what constraints were active when the author wrote each paragraph.

use std::collections::HashMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::enums::{
    Actant, Act, ConsciousnessMode, DiegeticLevel, GazeType, Genre,
    MotifStage, PactType, ParatextZone, PromisePhase, PromiseStatus,
    RevisionFlag, SpeechAct,
    // Theory-derived annotation keys (batch 1)
    AffectMode, BloomInfluenceMode, BurkeFormType, ComedyTheory, ComicMode,
    CulturalStructure, DisabilityRepMode, EmplotmentType, HermanRecoveryStage,
    IntertextualRelation, IronyType, MetaphorType, MimesisPhase, PentadElement,
    PentadRatio, QueerTimeMode, ReaderExperienceEffect, RevisionPassType,
    SerialArcType, SignType, SpatialPractice, SurvivranceMode,
    TemporalDuration, TemporalFrequency, TemporalOrder, TraumaMode,
    TransitivityProcess, YaNarratorType,
    // Theory-derived annotation keys (batch 2 — Phase IV/V)
    AbjectCategory, CognitiveNarrativeMode, DefamiliarizationMode,
    EcocriticalMode, ExperimentalNarrationMode, FeministNarrativeType,
    FreudianMechanism, ImageSchema, LacanRegister, MarxistNarrativeMode,
    PostcolonialMode, PosthumanMode, ProsodicElement, SemioticSquarePosition,
    SignifyingMode,
    // Theory-derived annotation keys (batch 3 — Phase V gaps)
    AutofictionMode, JenkinsTransmediaType, NarrativeEthicsMode,
    PerformativityMode, ProppFunction,
    // Genre reading contract
    GenreReadingMode,
    // Additional tagged theory enums (batch 4)
    AccessibilityRelation, AdaptationMode, GriceanMaxim, IntimateSpaceType,
    MultilingualStrategy, PanelTransitionType, StoryworkProtocol,
    TranslationStrategy, VisualVerbalRelation,
    // Narrative / prose annotation enums (batch 5 — scene, voice, style, craft)
    ArcPattern, ArcType, Complexity, ConflictType, DisplacementGap,
    DominantSense, EngagementMode, FocalizationType, KnowledgeScope,
    NarrativeDistance, NonResponseType, OpeningType, OutcomeType,
    PovType, ProfanityLevel, QuestionDelivery, ReliabilitySpectrum,
    SceneFunction, SceneType, SentenceArchitecture, SentenceLength,
    SentenceType, TimeOfDay, VocabularyLevel, VoiceStructure,
    Weather, WorldDeliveryMethod,
};
use crate::ontology::canonicalize_tag_key;

// ── Annotation channel ────────────────────────────────────────────────────────

/// Semantic channel for an annotation — determines which input channel the tag
/// contributes to when building `SceneContext` for training examples.
///
/// The two primary channels are kept **separate fields** in serialised training
/// data so a model can independently attend to *what is happening* (narrative
/// situation) vs. *how to render it* (prose technique).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AnnotationChannel {
    /// Entity references and narrative-structure metadata — describes
    /// *who/where/what is happening* in the story world.
    Context,
    /// Prose-technique instructions — describes *how to write* the passage
    /// (psychic distance, tension, rhythm, focalization mode, etc.).
    Craft,
    /// Scholarly and critical-theory framework tags — primarily for analysis
    /// workflows; controlled by [`TierConfig`](crate::training::TierConfig).
    Theory,
    /// Revision flags, source citations, and TTS/audiobook control tags.
    Governance,
}

// ── AnnotationKey trait ───────────────────────────────────────────────────────

/// Trait that marks an enum as a typed annotation key.
///
/// Any enum whose variants can appear as `<!-- key:value -->` annotation values
/// can implement this trait to record:
/// - `TAG_KEY` — the canonical Markdown annotation key string
/// - `CHANNEL` — which [`AnnotationChannel`] this annotation belongs to
///
/// The `to_pair()` convenience method derives a `(key, value)` string pair
/// suitable for the constraint-graph validation API.
///
/// ### Implementation
/// Use the [`impl_annotation_key!`] macro rather than writing the impl manually:
/// ```rust,ignore
/// impl_annotation_key!(PostcolonialMode,     "postcolonial",  Theory);
/// impl_annotation_key!(SentenceLength,       "sentence_length", Craft);
/// ```
pub trait AnnotationKey:
    std::fmt::Display
    + std::str::FromStr
    + Copy
    + Serialize
    + for<'de> Deserialize<'de>
    + schemars::JsonSchema
    + Send
    + Sync
    + 'static
{
    /// Canonical markdown annotation key, e.g. `"postcolonial"`, `"pov"`.
    const TAG_KEY: &'static str;
    /// Channel this annotation contributes to.
    const CHANNEL: AnnotationChannel;

    /// Returns a `(key, value)` pair for constraint-graph validation.
    fn to_pair(self) -> (String, String) {
        (Self::TAG_KEY.to_owned(), self.to_string())
    }
}

/// Implements [`AnnotationKey`] for a typed annotation enum.
///
/// Usage: `impl_annotation_key!(EnumType, "tag_key", ChannelVariant);`
macro_rules! impl_annotation_key {
    ($t:ty, $key:literal, $ch:ident) => {
        impl AnnotationKey for $t {
            const TAG_KEY: &'static str = $key;
            const CHANNEL: AnnotationChannel = AnnotationChannel::$ch;
        }
    };
}

// ── Entity reference ──────────────────────────────────────────────────────────

/// A validated slug reference to an entity of type `T`.
///
/// The slug is a `snake_case` string matching the entity's `id` field in
/// its registry.  Validation (checking the registry) happens at parse time
/// in the CLI tools; the type is a transparent wrapper at runtime.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub struct EntityRef {
    pub slug: String,
}

impl EntityRef {
    pub fn new(slug: impl Into<String>) -> Self {
        Self { slug: slug.into() }
    }
}

impl std::fmt::Display for EntityRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.slug)
    }
}

impl From<String> for EntityRef {
    fn from(s: String) -> Self {
        Self { slug: s }
    }
}

impl From<&str> for EntityRef {
    fn from(s: &str) -> Self {
        Self { slug: s.to_owned() }
    }
}

// ── Bounded integer types ─────────────────────────────────────────────────────

/// 1–10 tension score.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub struct Tension(u8);

impl Tension {
    pub fn new(value: u8) -> Result<Self, String> {
        if (1..=10).contains(&value) {
            Ok(Self(value))
        } else {
            Err(format!("Tension must be 1–10, got {value}"))
        }
    }
    pub fn value(self) -> u8 { self.0 }
}

// ── Source citation ────────────────────────────────────────────────────────────

/// A `<!-- source: ch3.p12 "…quote…" -->` citation used in analysis mode.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct SourceCitation {
    pub raw: String,
    pub chapter: Option<u32>,
    pub paragraph: Option<u32>,
    pub page: Option<u32>,
    pub quote: Option<String>,
}

/// Top-level document source declaration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct SourceTextMeta {
    pub title: String,
    pub author: String,
    pub edition: Option<String>,
    pub year: Option<u32>,
}

// ── Relationship tag data ─────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct RelationshipTag {
    pub from: EntityRef,
    pub to: EntityRef,
    pub rel_type: Option<String>,
}

// ── Scene compound tag ────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct SceneTagData {
    pub scene_id: EntityRef,
    pub beat: Option<EntityRef>,
    pub pov: Option<EntityRef>,
    pub setting: Option<EntityRef>,
}

// ── Promise tag ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct PromiseTagData {
    pub slug: EntityRef,
    pub phase: Option<PromisePhase>,
    pub status: Option<PromiseStatus>,
}

// ── Thread tag ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub enum ThreadId {
    Main,
    Subplot(String),
    FrameNarrator,
    EmbeddedStory,
    DualPov(String),
    Custom(String),
}

// ── Subplot compound tag ──────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct SubplotTagData {
    pub slug: EntityRef,
    pub characters: Vec<EntityRef>,
}

// ── The central Annotation type ───────────────────────────────────────────────

/// All 24+ annotation tag types from `references/gate-tag-vocabulary.md`,
/// now represented as a strongly-typed enum.
///
/// Each variant corresponds to one (or a family of) `<!-- key:value -->` tags.
/// Compound tags (e.g. `<!-- scene:slug beat:X pov:Y -->`) are parsed into
/// the appropriate structured variant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Annotation {
    // ── Entity references
    Character(EntityRef),
    Setting(EntityRef),
    Beat(EntityRef),
    Chapter { number: u32, title: Option<String> },

    // ── Relationship
    Relationship(RelationshipTag),

    // ── Scene compound
    Scene(SceneTagData),

    // ── Narrative thread
    Thread(ThreadId),

    // ── Subplot
    Subplot(SubplotTagData),

    // ── Structural
    DiegeticLevel(DiegeticLevel),
    Genre(Genre),
    Act(Act),

    // ── Character attributes (in-scene state)
    Archetype(EntityRef),  // slug validated against Archetype enum at load
    Role(EntityRef),
    Alignment(EntityRef),
    Wound(EntityRef),
    SocialCircle(EntityRef),
    PlotType(EntityRef),
    CollisionPattern(EntityRef),
    Trope(EntityRef),
    DriveModel(crate::enums::DriveModel),
    Actant(Actant),

    // ── Prose-level craft instructions (key input for voice fine-tuning)
    PsychicDistance(crate::enums::PsychicDistance),
    Consciousness(ConsciousnessMode),
    /// Free-text slug describing the unstated meaning active in this passage
    Subtext(String),
    Tension(Tension),
    SpeechAct(SpeechAct),
    Gaze(GazeType),
    Pact(PactType),

    // ── Motif
    Motif { slug: EntityRef, stage: Option<MotifStage> },

    // ── Hermeneutic code
    Promise(PromiseTagData),

    // ── Revision
    Flag(RevisionFlag),

    // ── Analysis mode
    Source(SourceCitation),
    SourceText(SourceTextMeta),

    // ── Paratext
    ParatextZone(ParatextZone),

    // ── TTS / audiobook
    Speaker { name: String },
    Tone(crate::enums::VoiceTone),
    Ambience(String),
    Music(String),
    Delivery(String),
    Sfx(String),

    // ── Genette narrative time
    TemporalOrder(TemporalOrder),
    TemporalDuration(TemporalDuration),
    TemporalFrequency(TemporalFrequency),

    // ── Reader experience (Sternberg / Iser)
    ReaderEffect(ReaderExperienceEffect),

    // ── Intertextuality (Genette / Bloom)
    Intertextual(IntertextualRelation),
    BloomMode(BloomInfluenceMode),

    // ── Metaphor and figurative language
    Metaphor(MetaphorType),

    // ── Comedy and irony
    Irony(IronyType),
    Comic(ComicMode),
    ComedyGrounds(ComedyTheory),

    // ── Trauma narratology
    TraumaMode(TraumaMode),
    RecoveryStage(HermanRecoveryStage),

    // ── Burke's Pentad
    PentadElement(PentadElement),
    PentadRatio(PentadRatio),
    BurkeForm(BurkeFormType),

    // ── Emplotment (Hayden White / Frye)
    Emplotment(EmplotmentType),

    // ── Ricoeur mimesis
    Mimesis(MimesisPhase),

    // ── Space (Bachelard / de Certeau)
    SpatialPractice(SpatialPractice),

    // ── Critical theory audit flags
    EthicsCategory(crate::enums::EthicsAuditCategory),
    CulturalStructure(CulturalStructure),
    SurvivranceMode(SurvivranceMode),
    QueerTime(QueerTimeMode),
    DisabilityRep(DisabilityRepMode),
    AffectMode(AffectMode),

    // ── Linguistics
    Transitivity(TransitivityProcess),
    JakobsonFn(crate::enums::JakobsonFunction),
    Sign(SignType),

    // ── YA narrative
    YaNarrator(YaNarratorType),

    // ── Craft tracking
    RevisionPass(RevisionPassType),
    SerialArc(SerialArcType),

    // ── Psychoanalytic narratology
    FreudianMechanism(FreudianMechanism),
    LacanRegister(LacanRegister),
    Abject(AbjectCategory),

    // ── Postcolonial
    PostcolonialMode(PostcolonialMode),

    // ── Ecocriticism
    EcocriticalMode(EcocriticalMode),

    // ── Embodied cognition
    ImageSchema(ImageSchema),

    // ── Prose linguistics
    Defamiliarization(DefamiliarizationMode),

    // ── Structuralist / semiotic
    SemioticPosition(SemioticSquarePosition),

    // ── Experimental narration
    ExperimentalMode(ExperimentalNarrationMode),

    // ── Posthumanism
    PosthumanMode(PosthumanMode),

    // ── Marxist narratology
    MarxistMode(MarxistNarrativeMode),

    // ── Feminist narratology
    FeministNarrative(FeministNarrativeType),

    // ── African-American narrative tradition
    Signifying(SignifyingMode),

    // ── Cognitive narratology
    CognitiveModeTag(CognitiveNarrativeMode),

    // ── Prosody
    ProsodicElement(ProsodicElement),

    // ── Propp morphological functions
    ProppFunction(ProppFunction),

    // ── Butler performativity
    Performativity(PerformativityMode),

    // ── Autofiction / life-writing mode
    AutofictionMode(AutofictionMode),

    // ── Narrative ethics
    NarrativeEthics(NarrativeEthicsMode),

    // ── Jenkins transmedia principles
    TransmediaType(JenkinsTransmediaType),

    // ── Doležel accessibility relation (possible-worlds)
    AccessibilityRelation(AccessibilityRelation),

    // ── Hutcheon / Stam adaptation mode
    Adaptation(AdaptationMode),

    // ── Grice cooperative maxim
    GriceanMaxim(GriceanMaxim),

    // ── Bachelard intimate space type
    IntimateSpace(IntimateSpaceType),

    // ── Multilingual rendering strategy
    Multilingual(MultilingualStrategy),

    // ── McCloud panel transition type
    PanelTransition(PanelTransitionType),

    // ── Archibald storywork protocol
    Storywork(StoryworkProtocol),

    // ── Venuti translation strategy
    TranslationStrategy(TranslationStrategy),

    // ── Chute visual-verbal relation
    VisualVerbal(VisualVerbalRelation),

    // ── Genre reading contract
    GenreReading(GenreReadingMode),

    // ── Scene-level structural tags
    SceneFunction(SceneFunction),
    SceneType(SceneType),
    Conflict(ConflictType),
    Arc(ArcType),
    ArcPattern(ArcPattern),
    Outcome(OutcomeType),

    // ── Environment / sensory
    DominantSense(DominantSense),
    TimeOfDay(TimeOfDay),
    Weather(Weather),
    WorldDelivery(WorldDeliveryMethod),

    // ── Narrator / focalization
    Pov(PovType),
    FocalizationType(FocalizationType),
    Knowledge(KnowledgeScope),
    Reliability(ReliabilitySpectrum),
    Engagement(EngagementMode),
    Distance(NarrativeDistance),
    Voice(VoiceStructure),

    // ── Prose style
    SentenceLength(SentenceLength),
    SentenceType(SentenceType),
    SentenceArch(SentenceArchitecture),
    Vocabulary(VocabularyLevel),
    Profanity(ProfanityLevel),

    // ── Narrative craft
    OpeningType(OpeningType),
    QuestionDelivery(QuestionDelivery),
    Complexity(Complexity),
    Displacement(DisplacementGap),
    NonResponse(NonResponseType),
}

impl Annotation {
    /// Classify this annotation into one of the four semantic channels.
    ///
    /// Used by the training-export pipeline to route tags into the
    /// `context_channel` vs `craft_channel` fields of a `ChannelledAnnotations`,
    /// and to apply [`TierConfig`](crate::training::TierConfig) tier filtering.
    pub fn channel(&self) -> AnnotationChannel {
        match self {
            // ── Context: entity references and narrative structure
            Annotation::Character(_)
            | Annotation::Setting(_)
            | Annotation::Beat(_)
            | Annotation::Chapter { .. }
            | Annotation::Relationship(_)
            | Annotation::Scene(_)
            | Annotation::Thread(_)
            | Annotation::Subplot(_)
            | Annotation::DiegeticLevel(_)
            | Annotation::Genre(_)
            | Annotation::Act(_)
            | Annotation::Archetype(_)
            | Annotation::Role(_)
            | Annotation::Alignment(_)
            | Annotation::Wound(_)
            | Annotation::SocialCircle(_)
            | Annotation::PlotType(_)
            | Annotation::CollisionPattern(_)
            | Annotation::Trope(_)
            | Annotation::DriveModel(_)
            | Annotation::Actant(_)
            | Annotation::Motif { .. }
            | Annotation::Promise(_)
            | Annotation::TemporalOrder(_)
            | Annotation::TemporalDuration(_)
            | Annotation::TemporalFrequency(_)
            | Annotation::SerialArc(_)
            | Annotation::SceneFunction(_)
            | Annotation::SceneType(_)
            | Annotation::Conflict(_)
            | Annotation::Arc(_)
            | Annotation::ArcPattern(_)
            | Annotation::Outcome(_)
            | Annotation::TimeOfDay(_)
            | Annotation::Weather(_)
            | Annotation::WorldDelivery(_)
            | Annotation::SpatialPractice(_)
            | Annotation::IntimateSpace(_) => AnnotationChannel::Context,

            // ── Craft: how to write the passage
            Annotation::PsychicDistance(_)
            | Annotation::Consciousness(_)
            | Annotation::Subtext(_)
            | Annotation::Tension(_)
            | Annotation::SpeechAct(_)
            | Annotation::Gaze(_)
            | Annotation::Pact(_)
            | Annotation::DominantSense(_)
            | Annotation::Pov(_)
            | Annotation::FocalizationType(_)
            | Annotation::Knowledge(_)
            | Annotation::Reliability(_)
            | Annotation::Engagement(_)
            | Annotation::Distance(_)
            | Annotation::Voice(_)
            | Annotation::SentenceLength(_)
            | Annotation::SentenceType(_)
            | Annotation::SentenceArch(_)
            | Annotation::Vocabulary(_)
            | Annotation::Profanity(_)
            | Annotation::OpeningType(_)
            | Annotation::QuestionDelivery(_)
            | Annotation::Complexity(_)
            | Annotation::Displacement(_)
            | Annotation::NonResponse(_)
            | Annotation::Metaphor(_)
            | Annotation::Irony(_)
            | Annotation::Comic(_)
            | Annotation::ComedyGrounds(_)
            | Annotation::ProsodicElement(_)
            | Annotation::ImageSchema(_)
            | Annotation::Defamiliarization(_)
            | Annotation::ReaderEffect(_)
            | Annotation::GenreReading(_) => AnnotationChannel::Craft,

            // ── Theory: scholarly / critical-theory framework tags
            Annotation::Intertextual(_)
            | Annotation::BloomMode(_)
            | Annotation::TraumaMode(_)
            | Annotation::RecoveryStage(_)
            | Annotation::PentadElement(_)
            | Annotation::PentadRatio(_)
            | Annotation::BurkeForm(_)
            | Annotation::Emplotment(_)
            | Annotation::Mimesis(_)
            | Annotation::EthicsCategory(_)
            | Annotation::CulturalStructure(_)
            | Annotation::SurvivranceMode(_)
            | Annotation::QueerTime(_)
            | Annotation::DisabilityRep(_)
            | Annotation::AffectMode(_)
            | Annotation::Transitivity(_)
            | Annotation::JakobsonFn(_)
            | Annotation::Sign(_)
            | Annotation::YaNarrator(_)
            | Annotation::FreudianMechanism(_)
            | Annotation::LacanRegister(_)
            | Annotation::Abject(_)
            | Annotation::PostcolonialMode(_)
            | Annotation::EcocriticalMode(_)
            | Annotation::SemioticPosition(_)
            | Annotation::ExperimentalMode(_)
            | Annotation::PosthumanMode(_)
            | Annotation::MarxistMode(_)
            | Annotation::FeministNarrative(_)
            | Annotation::Signifying(_)
            | Annotation::CognitiveModeTag(_)
            | Annotation::ProppFunction(_)
            | Annotation::Performativity(_)
            | Annotation::AutofictionMode(_)
            | Annotation::NarrativeEthics(_)
            | Annotation::TransmediaType(_)
            | Annotation::AccessibilityRelation(_)
            | Annotation::Adaptation(_)
            | Annotation::GriceanMaxim(_)
            | Annotation::Multilingual(_)
            | Annotation::PanelTransition(_)
            | Annotation::Storywork(_)
            | Annotation::TranslationStrategy(_)
            | Annotation::VisualVerbal(_) => AnnotationChannel::Theory,

            // ── Governance: flags, source, TTS
            Annotation::Flag(_)
            | Annotation::RevisionPass(_)
            | Annotation::Source(_)
            | Annotation::SourceText(_)
            | Annotation::ParatextZone(_)
            | Annotation::Speaker { .. }
            | Annotation::Tone(_)
            | Annotation::Ambience(_)
            | Annotation::Music(_)
            | Annotation::Delivery(_)
            | Annotation::Sfx(_) => AnnotationChannel::Governance,
        }
    }

    pub fn is_context(&self) -> bool {
        self.channel() == AnnotationChannel::Context
    }
    pub fn is_craft(&self) -> bool {
        self.channel() == AnnotationChannel::Craft
    }
    pub fn is_theory(&self) -> bool {
        self.channel() == AnnotationChannel::Theory
    }
    pub fn is_governance(&self) -> bool {
        self.channel() == AnnotationChannel::Governance
    }
}

// ── Paragraph-level annotation context ───────────────────────────────────────

/// The resolved annotation state at a specific paragraph boundary.
///
/// A `ParagraphAnnotations` captures all annotation values that are "active"
/// at the start of a paragraph (i.e. set by the most recent tag of each type
/// within that document position).  This is the structured representation
/// passed into `SceneContext.annotations` for training example construction.
///
/// Fields are grouped into three tiers:
/// - **Tier 1 (Core Craft)** — always present, minimum set for voice fine-tuning
/// - **Tier 2 (Narrative Structure)** — present when available
/// - **Tier 3 (Theory Enrichment)** — lives in `extra`; toggled by [`TierConfig`](crate::training::TierConfig)
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Default)]
pub struct ParagraphAnnotations {
    // ── Tier 1: Core Craft (always emit in training)
    pub psychic_distance: Option<crate::enums::PsychicDistance>,
    pub consciousness: Option<ConsciousnessMode>,
    pub subtext: Option<String>,
    pub tension: Option<Tension>,
    pub speech_act: Option<SpeechAct>,
    pub gaze: Option<GazeType>,
    pub dominant_sense: Option<crate::enums::DominantSense>,
    pub pov: Option<crate::enums::PovType>,
    pub focalization_type: Option<crate::enums::FocalizationType>,

    // ── Tier 2: Narrative Structure (emit when available)
    pub beat: Option<EntityRef>,
    pub motif: Option<EntityRef>,
    pub motif_stage: Option<MotifStage>,
    pub scene_function: Option<crate::enums::SceneFunction>,
    pub conflict: Option<crate::enums::ConflictType>,
    pub outcome: Option<crate::enums::OutcomeType>,
    pub temporal_order: Option<crate::enums::TemporalOrder>,
    pub reader_effect: Option<crate::enums::ReaderExperienceEffect>,
    pub arc_type: Option<crate::enums::ArcType>,

    // ── Governance
    pub flags: Vec<RevisionFlag>,
    pub speaker: Option<String>,
    pub tone: Option<crate::enums::VoiceTone>,

    // ── Tier 3 (Theory Enrichment) + any overflow
    pub extra: Vec<Annotation>,
}

impl ParagraphAnnotations {
    /// Build a `ParagraphAnnotations` from a slice of typed `Annotation`
    /// variants.  Known craft and structure tags are routed to their typed
    /// fields; all remaining annotations land in `extra`.
    pub fn from_annotations(anns: &[Annotation]) -> Self {
        let mut pa = ParagraphAnnotations::default();
        for ann in anns {
            match ann {
                Annotation::PsychicDistance(v) => pa.psychic_distance = Some(*v),
                Annotation::Consciousness(v) => pa.consciousness = Some(*v),
                Annotation::Subtext(v) => pa.subtext = Some(v.clone()),
                Annotation::Tension(v) => pa.tension = Some(*v),
                Annotation::SpeechAct(v) => pa.speech_act = Some(*v),
                Annotation::Gaze(v) => pa.gaze = Some(*v),
                Annotation::DominantSense(v) => pa.dominant_sense = Some(*v),
                Annotation::Pov(v) => pa.pov = Some(*v),
                Annotation::FocalizationType(v) => pa.focalization_type = Some(*v),
                Annotation::Beat(r) => pa.beat = Some(r.clone()),
                Annotation::Motif { slug, stage } => {
                    pa.motif = Some(slug.clone());
                    if let Some(s) = stage {
                        pa.motif_stage = Some(*s);
                    }
                }
                Annotation::SceneFunction(v) => pa.scene_function = Some(*v),
                Annotation::Conflict(v) => pa.conflict = Some(*v),
                Annotation::Outcome(v) => pa.outcome = Some(*v),
                Annotation::TemporalOrder(v) => pa.temporal_order = Some(*v),
                Annotation::ReaderEffect(v) => pa.reader_effect = Some(*v),
                Annotation::Arc(v) => pa.arc_type = Some(*v),
                Annotation::Flag(v) => pa.flags.push(*v),
                Annotation::Speaker { name } => pa.speaker = Some(name.clone()),
                Annotation::Tone(v) => pa.tone = Some(*v),
                other => pa.extra.push(other.clone()),
            }
        }
        pa
    }

    /// Flatten the typed fields into `(key, value)` string pairs suitable for
    /// constraint-graph validation via [`ConstraintGraph::validate`].
    pub fn to_constraint_pairs(&self) -> Vec<(String, String)> {
        let mut pairs: Vec<(String, String)> = Vec::new();
        if let Some(pd) = &self.psychic_distance {
            pairs.push(("psychic_distance".into(), pd.value().to_string()));
        }
        if let Some(c) = &self.consciousness {
            pairs.push(("consciousness".into(), c.to_string()));
        }
        if let Some(s) = &self.subtext {
            pairs.push(("subtext".into(), s.clone()));
        }
        if let Some(t) = &self.tension {
            pairs.push(("tension".into(), t.value().to_string()));
        }
        if let Some(sa) = &self.speech_act {
            pairs.push(("speech_act".into(), sa.to_string()));
        }
        if let Some(g) = &self.gaze {
            pairs.push(("gaze".into(), g.to_string()));
        }
        if let Some(d) = &self.dominant_sense {
            pairs.push(("dominant_sense".into(), d.to_string()));
        }
        if let Some(p) = &self.pov {
            pairs.push(("pov".into(), p.to_string()));
        }
        if let Some(f) = &self.focalization_type {
            pairs.push(("focalization_type".into(), f.to_string()));
        }
        if let Some(b) = &self.beat {
            pairs.push(("beat".into(), b.slug.clone()));
        }
        if let Some(sf) = &self.scene_function {
            pairs.push(("scene_function".into(), sf.to_string()));
        }
        if let Some(c) = &self.conflict {
            pairs.push(("conflict".into(), c.to_string()));
        }
        if let Some(o) = &self.outcome {
            pairs.push(("outcome".into(), o.to_string()));
        }
        if let Some(t) = &self.temporal_order {
            pairs.push(("temporal_order".into(), t.to_string()));
        }
        if let Some(r) = &self.reader_effect {
            pairs.push(("reader_effect".into(), r.to_string()));
        }
        if let Some(a) = &self.arc_type {
            pairs.push(("arc_type".into(), a.to_string()));
        }
        // Add pairs from extra (theory/governance channel annotations)
        for ann in &self.extra {
            if let Some((k, v)) = annotation_to_pair(ann) {
                pairs.push((k, v));
            }
        }
        pairs
    }

    /// Return only the context-channel annotations from `extra`.
    pub fn context_extras(&self) -> impl Iterator<Item = &Annotation> {
        self.extra.iter().filter(|a| a.is_context())
    }

    /// Return only the craft-channel annotations from `extra`.
    pub fn craft_extras(&self) -> impl Iterator<Item = &Annotation> {
        self.extra.iter().filter(|a| a.is_craft())
    }

    /// Return only the theory-channel annotations from `extra`.
    pub fn theory_extras(&self) -> impl Iterator<Item = &Annotation> {
        self.extra.iter().filter(|a| a.is_theory())
    }

    /// Collect all tier-3 (theory) annotations — i.e. extras that are
    /// in the Theory channel.
    pub fn theory_annotations(&self) -> Vec<&Annotation> {
        self.extra.iter().filter(|a| a.is_theory()).collect()
    }
}

// ── Sentence-level annotation context ────────────────────────────────────────

/// Annotation state at the grain of a single sentence.
///
/// Sentence-level tags are a focused subset of the [`Annotation`] vocabulary —
/// only craft properties that can meaningfully change sentence-by-sentence.
/// Many fields can be **automatically derived** from the surface text (sentence
/// length from word count; sentence type from syntax), so `auto_derived` tracks
/// whether the annotation was authored or inferred.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Default)]
pub struct SentenceAnnotations {
    /// 0-based index of this sentence within its paragraph.
    pub index: u32,

    // ── Syntactic structure
    pub sentence_length: Option<crate::enums::SentenceLength>,
    pub sentence_type: Option<crate::enums::SentenceType>,
    pub sentence_architecture: Option<crate::enums::SentenceArchitecture>,

    // ── Craft technique
    pub speech_act: Option<SpeechAct>,
    pub dominant_sense: Option<crate::enums::DominantSense>,
    pub image_schema: Option<crate::enums::ImageSchema>,
    pub prosodic_element: Option<crate::enums::ProsodicElement>,
    pub defamiliarization: Option<crate::enums::DefamiliarizationMode>,
    pub displacement_gap: Option<crate::enums::DisplacementGap>,
    pub gricean_maxim: Option<crate::enums::GriceanMaxim>,
    pub non_response: Option<crate::enums::NonResponseType>,
    pub metaphor: Option<crate::enums::MetaphorType>,
    pub irony: Option<crate::enums::IronyType>,

    /// True if this sentence is part of a dialogue exchange.
    pub contains_dialogue: bool,
    /// True when this annotation was automatically derived rather than
    /// hand-authored (allows filtering in training export).
    pub auto_derived: bool,
}

// ── Annotation parser (from raw tag strings) ──────────────────────────────────

/// Convert a single `Annotation` into a `(key, value)` string pair suitable
/// for constraint-graph validation.  Returns `None` for complex structural
/// Each simple single-value variant uses its `AnnotationKey::to_pair()` impl,
/// ensuring the emitted key always matches the key used by `parse_annotation_comment()`.
/// Complex multi-field variants (Motif, Chapter, Scene, Relationship, etc.) return `None`
/// — they are handled by `to_constraint_pairs()` on `ParagraphAnnotations`.
pub fn annotation_to_pair(ann: &Annotation) -> Option<(String, String)> {
    /// Calls `AnnotationKey::to_pair()` and wraps the result in `Some`.
    fn p<T: AnnotationKey>(v: T) -> Option<(String, String)> {
        Some(v.to_pair())
    }

    match ann {
        // ── Numeric newtypes — no AnnotationKey impl, value is a bounded integer
        Annotation::PsychicDistance(v) => Some(("psychic_distance".into(), v.value().to_string())),
        Annotation::Tension(v)         => Some(("tension".into(), v.value().to_string())),

        // ── Free-string variants
        Annotation::Subtext(s) => Some(("subtext".into(), s.clone())),

        // ── EntityRef slug variants
        Annotation::Beat(r)      => Some(("beat".into(),      r.slug.clone())),
        Annotation::Character(r) => Some(("character".into(), r.slug.clone())),
        Annotation::Setting(r)   => Some(("setting".into(),   r.slug.clone())),

        // ── Thread compound — flatten to a single string
        Annotation::Thread(t) => {
            let val = match t {
                ThreadId::Main           => "main".to_owned(),
                ThreadId::FrameNarrator  => "frame_narrator".to_owned(),
                ThreadId::EmbeddedStory  => "embedded_story".to_owned(),
                ThreadId::Subplot(v)     => format!("subplot_{v}"),
                ThreadId::DualPov(v)     => format!("dual_pov_{v}"),
                ThreadId::Custom(v)      => v.clone(),
            };
            Some(("thread".into(), val))
        }

        // ── SceneType variant — emit the parse key ("scene_type") for backward
        //    compatibility; the type alias SceneType = SceneFunction means the
        //    payload is a SceneFunction whose TAG_KEY is "scene_function".
        Annotation::SceneType(v) => Some(("scene_type".into(), v.to_string())),

        // ── All AnnotationKey-implementing enum variants ──────────────────────
        // Context channel
        Annotation::DiegeticLevel(v)     => p(*v),
        Annotation::TemporalOrder(v)     => p(*v),
        Annotation::TemporalDuration(v)  => p(*v),
        Annotation::TemporalFrequency(v) => p(*v),
        Annotation::SceneFunction(v)     => p(*v),
        Annotation::Conflict(v)          => p(*v),
        Annotation::Arc(v)               => p(*v),
        Annotation::ArcPattern(v)        => p(*v),
        Annotation::Outcome(v)           => p(*v),
        Annotation::DominantSense(v)     => p(*v),
        Annotation::TimeOfDay(v)         => p(*v),
        Annotation::Weather(v)           => p(*v),
        Annotation::WorldDelivery(v)     => p(*v),
        Annotation::SpatialPractice(v)   => p(*v),
        Annotation::IntimateSpace(v)     => p(*v),
        Annotation::SerialArc(v)         => p(*v),
        Annotation::DriveModel(v)        => p(*v),
        Annotation::Actant(v)            => p(*v),

        // Craft channel
        Annotation::Consciousness(v)     => p(*v),
        Annotation::SpeechAct(v)         => p(*v),
        Annotation::Gaze(v)              => p(*v),
        Annotation::Pact(v)              => p(*v),
        Annotation::Pov(v)               => p(*v),
        Annotation::FocalizationType(v)  => p(*v),
        Annotation::Knowledge(v)         => p(*v),
        Annotation::Reliability(v)       => p(*v),
        Annotation::Engagement(v)        => p(*v),
        Annotation::Distance(v)          => p(*v),
        Annotation::Voice(v)             => p(*v),
        Annotation::SentenceLength(v)    => p(*v),
        Annotation::SentenceType(v)      => p(*v),
        Annotation::SentenceArch(v)      => p(*v),
        Annotation::Vocabulary(v)        => p(*v),
        Annotation::Profanity(v)         => p(*v),
        Annotation::OpeningType(v)       => p(*v),
        Annotation::QuestionDelivery(v)  => p(*v),
        Annotation::Complexity(v)        => p(*v),
        Annotation::Displacement(v)      => p(*v),
        Annotation::NonResponse(v)       => p(*v),
        Annotation::Metaphor(v)          => p(*v),
        Annotation::Irony(v)             => p(*v),
        Annotation::Comic(v)             => p(*v),
        Annotation::ComedyGrounds(v)     => p(*v),
        Annotation::ProsodicElement(v)   => p(*v),
        Annotation::ImageSchema(v)       => p(*v),
        Annotation::Defamiliarization(v) => p(*v),
        Annotation::ReaderEffect(v)      => p(*v),
        Annotation::GenreReading(v)      => p(*v),
        Annotation::GriceanMaxim(v)      => p(*v),

        // Theory channel
        Annotation::Intertextual(v)          => p(*v),
        Annotation::BloomMode(v)             => p(*v),
        Annotation::TraumaMode(v)            => p(*v),
        Annotation::RecoveryStage(v)         => p(*v),
        Annotation::PentadElement(v)         => p(*v),
        Annotation::PentadRatio(v)           => p(*v),
        Annotation::BurkeForm(v)             => p(*v),
        Annotation::Emplotment(v)            => p(*v),
        Annotation::Mimesis(v)               => p(*v),
        Annotation::EthicsCategory(v)        => p(*v),
        Annotation::CulturalStructure(v)     => p(*v),
        Annotation::SurvivranceMode(v)       => p(*v),
        Annotation::QueerTime(v)             => p(*v),
        Annotation::DisabilityRep(v)         => p(*v),
        Annotation::AffectMode(v)            => p(*v),
        Annotation::Transitivity(v)          => p(*v),
        Annotation::JakobsonFn(v)            => p(*v),
        Annotation::Sign(v)                  => p(*v),
        Annotation::YaNarrator(v)            => p(*v),
        Annotation::FreudianMechanism(v)     => p(*v),
        Annotation::LacanRegister(v)         => p(*v),
        Annotation::Abject(v)               => p(*v),
        Annotation::PostcolonialMode(v)      => p(*v),
        Annotation::EcocriticalMode(v)       => p(*v),
        Annotation::SemioticPosition(v)      => p(*v),
        Annotation::ExperimentalMode(v)      => p(*v),
        Annotation::PosthumanMode(v)         => p(*v),
        Annotation::MarxistMode(v)           => p(*v),
        Annotation::FeministNarrative(v)     => p(*v),
        Annotation::Signifying(v)            => p(*v),
        Annotation::CognitiveModeTag(v)      => p(*v),
        Annotation::ProppFunction(v)         => p(*v),
        Annotation::Performativity(v)        => p(*v),
        Annotation::AutofictionMode(v)       => p(*v),
        Annotation::NarrativeEthics(v)       => p(*v),
        Annotation::TransmediaType(v)        => p(*v),
        Annotation::AccessibilityRelation(v) => p(*v),
        Annotation::Adaptation(v)            => p(*v),
        Annotation::Multilingual(v)          => p(*v),
        Annotation::PanelTransition(v)       => p(*v),
        Annotation::Storywork(v)             => p(*v),
        Annotation::TranslationStrategy(v)   => p(*v),
        Annotation::VisualVerbal(v)          => p(*v),

        // Governance channel
        Annotation::Flag(v)         => p(*v),
        Annotation::RevisionPass(v) => p(*v),
        Annotation::ParatextZone(v) => p(*v),

        // ── Complex / multi-field variants — not reducible to a single key:value pair.
        // These are handled structurally by `ParagraphAnnotations::to_constraint_pairs()`.
        _ => None,
    }
}

/// Parses a raw `<!-- key:value [key:value …] -->` string into zero or more
/// typed `Annotation` values.
///
/// Returns an error string for each tag value that fails to parse, so callers
/// can collect warnings without failing hard on unknown tags (forward compat).
pub fn parse_annotation_comment(
    raw: &str,
) -> (Vec<Annotation>, Vec<String>) {
    let mut annotations = Vec::new();
    let mut warnings = Vec::new();

    // Strip the HTML comment delimiters
    let inner = raw
        .trim_start_matches('<')
        .trim_start_matches('!')
        .trim_start_matches('-')
        .trim_start_matches('-')
        .trim_end_matches('-')
        .trim_end_matches('-')
        .trim_end_matches('>')
        .trim();

    // Parse space/comma-separated key:value pairs
    // A pair may contain a quoted value: key:"quoted value with spaces"
    let pairs = tokenize_pairs(inner);
    let mut kv: HashMap<String, String> = HashMap::new();
    for (k, v) in &pairs {
        kv.insert(k.clone(), v.clone());
    }

    macro_rules! parse_enum {
        ($key:literal, $variant:expr, $enum_type:ty) => {
            if let Some(v) = kv.get($key) {
                match v.parse::<$enum_type>() {
                    Ok(e) => annotations.push($variant(e)),
                    Err(_) => warnings.push(format!("unknown {}: {:?}", $key, v)),
                }
            }
        };
    }

    if let Some(slug) = kv.get("character") {
        annotations.push(Annotation::Character(EntityRef::new(slug)));
    }
    if let Some(slug) = kv.get("setting") {
        annotations.push(Annotation::Setting(EntityRef::new(slug)));
    }
    if let Some(slug) = kv.get("beat") {
        annotations.push(Annotation::Beat(EntityRef::new(slug)));
    }
    if let Some(slug) = kv.get("thread") {
        let thread_id = parse_thread_id(slug);
        annotations.push(Annotation::Thread(thread_id));
    }
    if let Some(slug) = kv.get("plot_type") {
        annotations.push(Annotation::PlotType(EntityRef::new(slug)));
    }
    if let Some(slug) = kv.get("collision_pattern") {
        annotations.push(Annotation::CollisionPattern(EntityRef::new(slug)));
    }
    if let Some(slug) = kv.get("trope") {
        annotations.push(Annotation::Trope(EntityRef::new(slug)));
    }

    parse_enum!("diegetic_level", Annotation::DiegeticLevel, DiegeticLevel);
    parse_enum!("consciousness", Annotation::Consciousness, ConsciousnessMode);
    parse_enum!("speech_act", Annotation::SpeechAct, SpeechAct);
    parse_enum!("gaze", Annotation::Gaze, GazeType);
    parse_enum!("pact", Annotation::Pact, PactType);
    parse_enum!("actant", Annotation::Actant, Actant);

    if let Some(v) = kv.get("psychic_distance") {
        match v.parse::<u8>() {
            Ok(n) => match crate::enums::PsychicDistance::new(n) {
                Ok(pd) => annotations.push(Annotation::PsychicDistance(pd)),
                Err(e) => warnings.push(e),
            },
            Err(_) => warnings.push(format!("invalid psychic_distance: {:?}", v)),
        }
    }
    if let Some(v) = kv.get("tension") {
        match v.parse::<u8>() {
            Ok(n) => match Tension::new(n) {
                Ok(t) => annotations.push(Annotation::Tension(t)),
                Err(e) => warnings.push(e),
            },
            Err(_) => warnings.push(format!("invalid tension: {:?}", v)),
        }
    }
    if let Some(v) = kv.get("subtext") {
        annotations.push(Annotation::Subtext(v.clone()));
    }
    if let Some(v) = kv.get("flag") {
        match v.parse::<RevisionFlag>() {
            Ok(f) => annotations.push(Annotation::Flag(f)),
            Err(_) => warnings.push(format!("unknown flag: {:?}", v)),
        }
    }
    if let Some(slug) = kv.get("motif") {
        let stage = kv.get("stage").and_then(|s| s.parse::<MotifStage>().ok());
        annotations.push(Annotation::Motif {
            slug: EntityRef::new(slug),
            stage,
        });
    }
    if let Some(v) = kv.get("chapter") {
        if let Ok(n) = v.parse::<u32>() {
            annotations.push(Annotation::Chapter { number: n, title: None });
        } else {
            warnings.push(format!("invalid chapter number: {:?}", v));
        }
    }
    if let Some(name) = kv.get("speaker") {
        annotations.push(Annotation::Speaker { name: name.clone() });
    }
    if let Some(v) = kv.get("tone") {
        match v.parse::<crate::enums::VoiceTone>() {
            Ok(t) => annotations.push(Annotation::Tone(t)),
            Err(_) => warnings.push(format!("unknown tone: {:?}", v)),
        }
    }
    if let Some(v) = kv.get("ambience") {
        annotations.push(Annotation::Ambience(v.clone()));
    }
    if let Some(v) = kv.get("music") {
        annotations.push(Annotation::Music(v.clone()));
    }
    if let Some(v) = kv.get("sfx") {
        annotations.push(Annotation::Sfx(v.clone()));
    }
    if let Some(v) = kv.get("delivery") {
        annotations.push(Annotation::Delivery(v.clone()));
    }
    if let Some(zone) = kv.get("paratext_zone") {
        match zone.parse::<ParatextZone>() {
            Ok(z) => annotations.push(Annotation::ParatextZone(z)),
            Err(_) => warnings.push(format!("unknown paratext_zone: {:?}", zone)),
        }
    }

    // ── Genette narrative time
    parse_enum!("temporal_order",    Annotation::TemporalOrder,    TemporalOrder);
    parse_enum!("temporal_duration", Annotation::TemporalDuration, TemporalDuration);
    parse_enum!("temporal_frequency",Annotation::TemporalFrequency,TemporalFrequency);

    // ── Reader experience
    parse_enum!("reader_effect", Annotation::ReaderEffect, ReaderExperienceEffect);

    // ── Intertextuality
    parse_enum!("intertextual",  Annotation::Intertextual, IntertextualRelation);
    parse_enum!("bloom_mode",    Annotation::BloomMode,    BloomInfluenceMode);

    // ── Metaphor
    parse_enum!("metaphor", Annotation::Metaphor, MetaphorType);

    // ── Comedy and irony
    parse_enum!("irony",         Annotation::Irony,        IronyType);
    parse_enum!("comic_mode",    Annotation::Comic,        ComicMode);
    parse_enum!("comedy_theory", Annotation::ComedyGrounds,ComedyTheory);

    // ── Trauma
    parse_enum!("trauma_mode",    Annotation::TraumaMode,    TraumaMode);
    parse_enum!("recovery_stage", Annotation::RecoveryStage, HermanRecoveryStage);

    // ── Burke's Pentad
    parse_enum!("pentad_element", Annotation::PentadElement, PentadElement);
    parse_enum!("pentad_ratio",   Annotation::PentadRatio,   PentadRatio);
    parse_enum!("burke_form",     Annotation::BurkeForm,     BurkeFormType);

    // ── Emplotment
    parse_enum!("emplotment", Annotation::Emplotment, EmplotmentType);

    // ── Ricoeur mimesis
    parse_enum!("mimesis", Annotation::Mimesis, MimesisPhase);

    // ── Space
    parse_enum!("spatial_practice", Annotation::SpatialPractice, SpatialPractice);

    // ── Critical theory
    parse_enum!("ethics",          Annotation::EthicsCategory,  crate::enums::EthicsAuditCategory);
    parse_enum!("cultural_structure", Annotation::CulturalStructure, CulturalStructure);
    parse_enum!("survivrance",     Annotation::SurvivranceMode, SurvivranceMode);
    parse_enum!("queer_time",      Annotation::QueerTime,       QueerTimeMode);
    parse_enum!("disability_rep",  Annotation::DisabilityRep,   DisabilityRepMode);
    parse_enum!("affect",          Annotation::AffectMode,      AffectMode);

    // ── Linguistics
    parse_enum!("transitivity", Annotation::Transitivity, TransitivityProcess);
    parse_enum!("jakobson",     Annotation::JakobsonFn,    crate::enums::JakobsonFunction);
    parse_enum!("sign_type",    Annotation::Sign,          SignType);

    // ── YA narrative
    parse_enum!("ya_narrator", Annotation::YaNarrator, YaNarratorType);

    // ── Craft tracking
    parse_enum!("revision_pass", Annotation::RevisionPass, RevisionPassType);
    parse_enum!("serial_arc",    Annotation::SerialArc,    SerialArcType);

    // ── Psychoanalytic
    parse_enum!("freudian",      Annotation::FreudianMechanism, FreudianMechanism);
    parse_enum!("lacan_register",Annotation::LacanRegister,    LacanRegister);
    parse_enum!("abject",        Annotation::Abject,           AbjectCategory);

    // ── Postcolonial
    parse_enum!("postcolonial",  Annotation::PostcolonialMode, PostcolonialMode);

    // ── Ecocriticism
    parse_enum!("ecocritical",   Annotation::EcocriticalMode,  EcocriticalMode);

    // ── Embodied cognition
    parse_enum!("image_schema",  Annotation::ImageSchema,      ImageSchema);

    // ── Prose linguistics
    parse_enum!("defamiliarize", Annotation::Defamiliarization, DefamiliarizationMode);

    // ── Structuralist
    parse_enum!("semiotic_pos",  Annotation::SemioticPosition,  SemioticSquarePosition);

    // ── Experimental narration
    parse_enum!("experimental",  Annotation::ExperimentalMode,  ExperimentalNarrationMode);

    // ── Posthumanism
    parse_enum!("posthuman",     Annotation::PosthumanMode,     PosthumanMode);

    // ── Marxist narratology
    parse_enum!("marxist_mode",  Annotation::MarxistMode,       MarxistNarrativeMode);

    // ── Feminist narratology
    parse_enum!("feminist",      Annotation::FeministNarrative, FeministNarrativeType);

    // ── African-American tradition
    parse_enum!("signifying",    Annotation::Signifying,        SignifyingMode);

    // ── Cognitive narratology
    parse_enum!("cognitive",     Annotation::CognitiveModeTag,  CognitiveNarrativeMode);

    // ── Prosody
    parse_enum!("prosodic",      Annotation::ProsodicElement,   ProsodicElement);

    // ── Propp morphological functions
    parse_enum!("propp",         Annotation::ProppFunction,     ProppFunction);

    // ── Butler performativity
    parse_enum!("performativity", Annotation::Performativity,   PerformativityMode);

    // ── Autofiction mode
    parse_enum!("autofiction",   Annotation::AutofictionMode,   AutofictionMode);

    // ── Narrative ethics
    parse_enum!("narrative_ethics", Annotation::NarrativeEthics, NarrativeEthicsMode);

    // ── Jenkins transmedia
    parse_enum!("transmedia",    Annotation::TransmediaType,    JenkinsTransmediaType);

    // ── Doležel accessibility relation
    parse_enum!("accessibility",    Annotation::AccessibilityRelation, AccessibilityRelation);

    // ── Hutcheon / Stam adaptation mode
    parse_enum!("adaptation",       Annotation::Adaptation,            AdaptationMode);

    // ── Grice cooperative maxim
    parse_enum!("gricean",          Annotation::GriceanMaxim,          GriceanMaxim);

    // ── Bachelard intimate space
    parse_enum!("intimate_space",   Annotation::IntimateSpace,         IntimateSpaceType);

    // ── Multilingual rendering strategy
    parse_enum!("multilingual",     Annotation::Multilingual,          MultilingualStrategy);

    // ── McCloud panel transition
    parse_enum!("panel_transition", Annotation::PanelTransition,       PanelTransitionType);

    // ── Archibald storywork protocol
    parse_enum!("storywork",        Annotation::Storywork,             StoryworkProtocol);

    // ── Venuti translation strategy
    parse_enum!("translation",      Annotation::TranslationStrategy,   TranslationStrategy);

    // ── Chute visual-verbal relation
    parse_enum!("visual_verbal",    Annotation::VisualVerbal,          VisualVerbalRelation);

    // ── Genre reading contract
    parse_enum!("genre_reading",    Annotation::GenreReading,     GenreReadingMode);

    // ── Scene-level structural
    parse_enum!("scene_function",   Annotation::SceneFunction,    SceneFunction);
    parse_enum!("scene_type",       Annotation::SceneType,        SceneType);
    parse_enum!("conflict",         Annotation::Conflict,         ConflictType);
    parse_enum!("arc",              Annotation::Arc,              ArcType);
    parse_enum!("arc_pattern",      Annotation::ArcPattern,       ArcPattern);
    parse_enum!("outcome",          Annotation::Outcome,          OutcomeType);

    // ── Environment / sensory
    parse_enum!("sense",            Annotation::DominantSense,    DominantSense);
    parse_enum!("time_of_day",      Annotation::TimeOfDay,        TimeOfDay);
    parse_enum!("weather",          Annotation::Weather,          Weather);
    parse_enum!("world_delivery",   Annotation::WorldDelivery,    WorldDeliveryMethod);

    // ── Narrator / focalization
    parse_enum!("pov",              Annotation::Pov,              PovType);
    parse_enum!("focalization_type",Annotation::FocalizationType, FocalizationType);
    parse_enum!("knowledge",        Annotation::Knowledge,        KnowledgeScope);
    parse_enum!("reliability",      Annotation::Reliability,      ReliabilitySpectrum);
    parse_enum!("engagement",       Annotation::Engagement,       EngagementMode);
    parse_enum!("narrative_distance",Annotation::Distance,        NarrativeDistance);
    parse_enum!("voice_structure",  Annotation::Voice,            VoiceStructure);

    // ── Prose style
    parse_enum!("sentence_length",  Annotation::SentenceLength,   SentenceLength);
    parse_enum!("sentence_type",    Annotation::SentenceType,     SentenceType);
    parse_enum!("sentence_arch",    Annotation::SentenceArch,     SentenceArchitecture);
    parse_enum!("vocabulary",       Annotation::Vocabulary,       VocabularyLevel);
    parse_enum!("profanity",        Annotation::Profanity,        ProfanityLevel);

    // ── Narrative craft
    parse_enum!("opening_type",     Annotation::OpeningType,      OpeningType);
    parse_enum!("question_delivery",Annotation::QuestionDelivery, QuestionDelivery);
    parse_enum!("complexity",       Annotation::Complexity,       Complexity);
    parse_enum!("displacement",     Annotation::Displacement,     DisplacementGap);
    parse_enum!("non_response",     Annotation::NonResponse,      NonResponseType);

    (annotations, warnings)
}

fn parse_thread_id(s: &str) -> ThreadId {
    match s {
        "main" => ThreadId::Main,
        "frame_narrator" => ThreadId::FrameNarrator,
        "embedded_story" => ThreadId::EmbeddedStory,
        other if other.starts_with("subplot_") => {
            ThreadId::Subplot(other.trim_start_matches("subplot_").to_owned())
        }
        other if other.starts_with("dual_pov_") => {
            ThreadId::DualPov(other.trim_start_matches("dual_pov_").to_owned())
        }
        other => ThreadId::Custom(other.to_owned()),
    }
}

/// Tokenize a flat string of `key:value` pairs, handling quoted values and
/// comma-separated multi-values.
fn tokenize_pairs(input: &str) -> Vec<(String, String)> {
    let mut pairs = Vec::new();
    let mut rest = input.trim();
    while !rest.is_empty() {
        // Find key (up to colon)
        let colon = match rest.find(':') {
            Some(i) => i,
            None => break,
        };
        let key = canonicalize_tag_key(rest[..colon].trim());
        rest = rest[colon + 1..].trim_start();

        // Value may be quoted
        let (value, consumed) = if rest.starts_with('"') {
            let end = rest[1..].find('"').unwrap_or(rest.len() - 1);
            let v = rest[1..end + 1].to_owned();
            (v, end + 2)
        } else {
            // Value ends at next whitespace or end
            let end = rest.find(|c: char| c.is_whitespace()).unwrap_or(rest.len());
            (rest[..end].to_owned(), end)
        };

        if !key.is_empty() && !value.is_empty() {
            pairs.push((key, value));
        }
        rest = rest[consumed..].trim_start();
    }
    pairs
}

// ── AnnotationKey impls ───────────────────────────────────────────────────────
//
// One impl_annotation_key! invocation per typed annotation enum.
// Grouped by channel to mirror the Annotation::channel() match arms.

// Context
impl_annotation_key!(crate::enums::TemporalOrder,        "temporal_order",    Context);
impl_annotation_key!(crate::enums::TemporalDuration,     "temporal_duration", Context);
impl_annotation_key!(crate::enums::TemporalFrequency,    "temporal_frequency",Context);
impl_annotation_key!(crate::enums::SceneFunction,        "scene_function",    Context);
impl_annotation_key!(crate::enums::ArcType,              "arc",               Context);
impl_annotation_key!(crate::enums::ArcPattern,           "arc_pattern",       Context);
impl_annotation_key!(crate::enums::ConflictType,         "conflict",          Context);
impl_annotation_key!(crate::enums::OutcomeType,          "outcome",           Context);
impl_annotation_key!(crate::enums::DominantSense,        "sense",             Context);
impl_annotation_key!(crate::enums::TimeOfDay,            "time_of_day",       Context);
impl_annotation_key!(crate::enums::Weather,              "weather",           Context);
impl_annotation_key!(crate::enums::WorldDeliveryMethod,  "world_delivery",    Context);
impl_annotation_key!(crate::enums::SpatialPractice,      "spatial_practice",  Context);
impl_annotation_key!(crate::enums::IntimateSpaceType,    "intimate_space",    Context);
impl_annotation_key!(DiegeticLevel,                      "diegetic_level",    Context);
impl_annotation_key!(crate::enums::SerialArcType,        "serial_arc",        Context);

// Craft
impl_annotation_key!(crate::enums::PovType,              "pov",               Craft);
impl_annotation_key!(crate::enums::FocalizationType,     "focalization_type", Craft);
impl_annotation_key!(crate::enums::KnowledgeScope,       "knowledge",         Craft);
impl_annotation_key!(crate::enums::ReliabilitySpectrum,  "reliability",       Craft);
impl_annotation_key!(crate::enums::EngagementMode,       "engagement",        Craft);
impl_annotation_key!(crate::enums::NarrativeDistance,    "narrative_distance",Craft);
impl_annotation_key!(crate::enums::VoiceStructure,       "voice_structure",   Craft);
impl_annotation_key!(crate::enums::SentenceLength,       "sentence_length",   Craft);
impl_annotation_key!(crate::enums::SentenceType,         "sentence_type",     Craft);
impl_annotation_key!(crate::enums::SentenceArchitecture, "sentence_arch",     Craft);
impl_annotation_key!(crate::enums::VocabularyLevel,      "vocabulary",        Craft);
impl_annotation_key!(crate::enums::ProfanityLevel,       "profanity",         Craft);
impl_annotation_key!(crate::enums::OpeningType,          "opening_type",      Craft);
impl_annotation_key!(crate::enums::QuestionDelivery,     "question_delivery", Craft);
impl_annotation_key!(crate::enums::Complexity,           "complexity",        Craft);
impl_annotation_key!(crate::enums::DisplacementGap,      "displacement",      Craft);
impl_annotation_key!(crate::enums::NonResponseType,      "non_response",      Craft);
impl_annotation_key!(crate::enums::MetaphorType,         "metaphor",          Craft);
impl_annotation_key!(crate::enums::IronyType,            "irony",             Craft);
impl_annotation_key!(crate::enums::ComicMode,            "comic_mode",        Craft);
impl_annotation_key!(crate::enums::ComedyTheory,         "comedy_theory",     Craft);
impl_annotation_key!(crate::enums::ProsodicElement,      "prosodic",          Craft);
impl_annotation_key!(crate::enums::ImageSchema,          "image_schema",      Craft);
impl_annotation_key!(crate::enums::DefamiliarizationMode,"defamiliarize",     Craft);
impl_annotation_key!(crate::enums::ReaderExperienceEffect,"reader_effect",    Craft);
impl_annotation_key!(crate::enums::GenreReadingMode,     "genre_reading",     Craft);
impl_annotation_key!(crate::enums::SpeechAct,            "speech_act",        Craft);
impl_annotation_key!(crate::enums::GriceanMaxim,         "gricean",           Craft);

// Theory
impl_annotation_key!(crate::enums::IntertextualRelation, "intertextual",      Theory);
impl_annotation_key!(crate::enums::BloomInfluenceMode,   "bloom_mode",        Theory);
impl_annotation_key!(crate::enums::TraumaMode,           "trauma_mode",       Theory);
impl_annotation_key!(crate::enums::HermanRecoveryStage,  "recovery_stage",    Theory);
impl_annotation_key!(crate::enums::PentadElement,        "pentad_element",    Theory);
impl_annotation_key!(crate::enums::PentadRatio,          "pentad_ratio",      Theory);
impl_annotation_key!(crate::enums::BurkeFormType,        "burke_form",        Theory);
impl_annotation_key!(crate::enums::EmplotmentType,       "emplotment",        Theory);
impl_annotation_key!(crate::enums::MimesisPhase,         "mimesis",           Theory);
impl_annotation_key!(crate::enums::CulturalStructure,    "cultural_structure",Theory);
impl_annotation_key!(crate::enums::SurvivranceMode,      "survivrance",       Theory);
impl_annotation_key!(crate::enums::QueerTimeMode,        "queer_time",        Theory);
impl_annotation_key!(crate::enums::DisabilityRepMode,    "disability_rep",    Theory);
impl_annotation_key!(crate::enums::AffectMode,           "affect",            Theory);
impl_annotation_key!(crate::enums::TransitivityProcess,  "transitivity",      Theory);
impl_annotation_key!(crate::enums::SignType,             "sign_type",         Theory);
impl_annotation_key!(crate::enums::YaNarratorType,       "ya_narrator",       Theory);
impl_annotation_key!(crate::enums::FreudianMechanism,    "freudian",          Theory);
impl_annotation_key!(crate::enums::LacanRegister,        "lacan_register",    Theory);
impl_annotation_key!(crate::enums::AbjectCategory,       "abject",            Theory);
impl_annotation_key!(crate::enums::PostcolonialMode,     "postcolonial",      Theory);
impl_annotation_key!(crate::enums::EcocriticalMode,      "ecocritical",       Theory);
impl_annotation_key!(crate::enums::SemioticSquarePosition,"semiotic_pos",     Theory);
impl_annotation_key!(crate::enums::ExperimentalNarrationMode,"experimental",  Theory);
impl_annotation_key!(crate::enums::PosthumanMode,        "posthuman",         Theory);
impl_annotation_key!(crate::enums::MarxistNarrativeMode, "marxist_mode",      Theory);
impl_annotation_key!(crate::enums::FeministNarrativeType,"feminist",          Theory);
impl_annotation_key!(crate::enums::SignifyingMode,       "signifying",        Theory);
impl_annotation_key!(crate::enums::CognitiveNarrativeMode,"cognitive",        Theory);
impl_annotation_key!(crate::enums::ProppFunction,        "propp",             Theory);
impl_annotation_key!(crate::enums::PerformativityMode,   "performativity",    Theory);
impl_annotation_key!(crate::enums::AutofictionMode,      "autofiction",       Theory);
impl_annotation_key!(crate::enums::NarrativeEthicsMode,  "narrative_ethics",  Theory);
impl_annotation_key!(crate::enums::JenkinsTransmediaType,"transmedia",        Theory);
impl_annotation_key!(crate::enums::AccessibilityRelation,"accessibility",     Theory);
impl_annotation_key!(crate::enums::AdaptationMode,       "adaptation",        Theory);
impl_annotation_key!(crate::enums::MultilingualStrategy, "multilingual",      Theory);
impl_annotation_key!(crate::enums::PanelTransitionType,  "panel_transition",  Theory);
impl_annotation_key!(crate::enums::StoryworkProtocol,    "storywork",         Theory);
impl_annotation_key!(crate::enums::TranslationStrategy,  "translation",       Theory);
impl_annotation_key!(crate::enums::VisualVerbalRelation, "visual_verbal",     Theory);

// Governance
impl_annotation_key!(crate::enums::RevisionFlag,         "flag",              Governance);
impl_annotation_key!(crate::enums::RevisionPassType,     "revision_pass",     Governance);
impl_annotation_key!(crate::enums::ParatextZone,         "paratext_zone",     Governance);

// Additional simple annotation enums not covered above
impl_annotation_key!(crate::enums::PactType,             "pact",              Craft);
impl_annotation_key!(GazeType,                           "gaze",              Craft);
impl_annotation_key!(crate::enums::EthicsAuditCategory,  "ethics",            Theory);
impl_annotation_key!(crate::enums::JakobsonFunction,     "jakobson",          Theory);
impl_annotation_key!(crate::enums::DriveModel,           "drive_model",       Context);
impl_annotation_key!(crate::enums::Actant,               "actant",            Context);
impl_annotation_key!(ConsciousnessMode,                  "consciousness",     Craft);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_psychic_distance() {
        let (anns, warns) = parse_annotation_comment("<!-- psychic_distance:3 -->");
        assert!(warns.is_empty());
        assert_eq!(anns.len(), 1);
        assert!(matches!(anns[0], Annotation::PsychicDistance(_)));
    }

    #[test]
    fn parse_compound_motif() {
        let (anns, warns) = parse_annotation_comment("<!-- motif:water stage:payoff -->");
        assert!(warns.is_empty());
        let motif = anns.iter().find(|a| matches!(a, Annotation::Motif { .. }));
        assert!(motif.is_some(), "should have parsed a motif annotation");
        if let Some(Annotation::Motif { slug, stage }) = motif {
            assert_eq!(slug.slug, "water");
            assert_eq!(*stage, Some(MotifStage::Payoff));
        }
    }

    #[test]
    fn parse_out_of_range_tension() {
        let (anns, warns) = parse_annotation_comment("<!-- tension:11 -->");
        assert!(anns.is_empty());
        assert!(!warns.is_empty());
    }

    #[test]
    fn parse_flag() {
        let (anns, warns) =
            parse_annotation_comment("<!-- flag:telling_not_showing -->");
        assert!(warns.is_empty());
        assert_eq!(anns.len(), 1);
        assert!(matches!(anns[0], Annotation::Flag(RevisionFlag::TellingNotShowing)));
    }
}
