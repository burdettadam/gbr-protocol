//! All enumerations used across the Grimoire data model.
//!
//! Enums are derived from three sources (previously out of sync):
//! - `grimoire/models.py` — Python enums
//! - `schemas/_base.schema.json` — JSON Schema enum definitions
//! - YAML catalogs under each phase's `references/` directory
//!
//! Rust's exhaustive pattern matching enforces consistency at compile time.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

// ── Gate system ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum Severity {
    Required,
    Recommended,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum GateStatus {
    Green,
    Yellow,
    Red,
    Locked,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum SubPhaseStatus {
    Locked,
    Ready,
    InProgress,
    Complete,
}

/// Phase-level readiness status — unified with `GateStatus`.
///
/// A phase can be `Green`, `Yellow`, `Red`, or `Unknown`; it never uses the
/// `Locked` variant (only sub-phases lock). This alias ensures one canonical
/// status type for both gate-level and phase-level results.
pub type PhaseStatus = GateStatus;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum CheckType {
    FileExists,
    PlaceholderRatio,
    CheckboxCompletion,
    WordCountMin,
    TagCrossRef,
    EntityCoverage,
    SchemaValid,
    SourceCoverage,
}

// ── Character ────────────────────────────────────────────────────────────────

/// Fourteen canonical archetypes drawn from the character-archetypes YAML catalog.
/// Previously the JSON Schema had 14 entries and the YAML had ~13 — now unified.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum Archetype {
    Hero,
    Mentor,
    ThresholdGuardian,
    Herald,
    Shapeshifter,
    Shadow,
    Ally,
    Trickster,
    Everyman,
    Lover,
    Caregiver,
    Sage,
    Innocent,
    Rebel,
    Ruler,
    Outcast,
    Creator,
    Explorer,
    Magician,
    Jester,
    Outlaw,
}

/// Coarse wound category — the 13 legacy umbrella terms.
///
/// Use `Wound::category()` to map any specific `Wound` variant to its
/// broad `WoundCategory`.  This enum exists so catalog-level grouping and
/// UI filtering can work at a higher level of abstraction than the full
/// 23-variant `Wound` list.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum WoundCategory {
    Abandonment,
    Betrayal,
    GuiltAndFailure,
    TraumaAndAbuse,
    Shame,
    Grief,
    TrustViolation,
    Powerlessness,
    IdentityRejection,
    Injustice,
    Neglect,
    SurvivorGuilt,
    Displacement,
}

/// Twenty-three wound variants: 13 legacy umbrella terms plus 10 catalog-native
/// fine-grained subtypes.  Use `Wound::category()` to get the umbrella group.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum Wound {
    // Legacy umbrella terms (kept for backward compatibility)
    /// Parental or caregiver abandonment
    Abandonment,
    /// Betrayal by a trusted person
    Betrayal,
    /// Failure to protect someone dependent
    GuiltAndFailure,
    /// Violation of bodily or psychological safety
    TraumaAndAbuse,
    /// Persistent devaluation of self-worth
    Shame,
    /// Repeated or profound loss
    Grief,
    /// Fundamental mistrust of others' motives
    TrustViolation,
    /// Powerlessness in the face of authority or circumstance
    Powerlessness,
    /// Rejection of core identity
    IdentityRejection,
    /// Unfair treatment without recourse
    Injustice,
    /// Insufficient nurturing or care
    Neglect,
    /// Survivor's guilt or complicity in harm to others
    SurvivorGuilt,
    /// Isolation — belonging nowhere
    Displacement,

    // Canonical catalog-native terms
    ParentalAbandonment,
    RomanticAbandonment,
    SocialExile,
    InstitutionalBetrayal,
    PublicHumiliation,
    IdentityShame,
    LossOfPurpose,
    CatastrophicFailure,
    MoralCompromise,
    Smothering,
}

impl Wound {
    /// Maps any `Wound` variant to its broad `WoundCategory`.
    ///
    /// Umbrella wounds (the 13 legacy terms) map directly to themselves.
    /// Catalog-native wounds (the 10 fine-grained subtypes) map to their
    /// parent umbrella category.
    pub fn category(self) -> WoundCategory {
        match self {
            Wound::Abandonment           => WoundCategory::Abandonment,
            Wound::Betrayal              => WoundCategory::Betrayal,
            Wound::GuiltAndFailure       => WoundCategory::GuiltAndFailure,
            Wound::TraumaAndAbuse        => WoundCategory::TraumaAndAbuse,
            Wound::Shame                 => WoundCategory::Shame,
            Wound::Grief                 => WoundCategory::Grief,
            Wound::TrustViolation        => WoundCategory::TrustViolation,
            Wound::Powerlessness         => WoundCategory::Powerlessness,
            Wound::IdentityRejection     => WoundCategory::IdentityRejection,
            Wound::Injustice             => WoundCategory::Injustice,
            Wound::Neglect               => WoundCategory::Neglect,
            Wound::SurvivorGuilt         => WoundCategory::SurvivorGuilt,
            Wound::Displacement          => WoundCategory::Displacement,
            // Catalog-native → parent umbrella
            Wound::ParentalAbandonment   => WoundCategory::Abandonment,
            Wound::RomanticAbandonment   => WoundCategory::Abandonment,
            Wound::SocialExile           => WoundCategory::Displacement,
            Wound::InstitutionalBetrayal => WoundCategory::Betrayal,
            Wound::PublicHumiliation     => WoundCategory::Shame,
            Wound::IdentityShame         => WoundCategory::Shame,
            Wound::LossOfPurpose         => WoundCategory::GuiltAndFailure,
            Wound::CatastrophicFailure   => WoundCategory::GuiltAndFailure,
            Wound::MoralCompromise       => WoundCategory::GuiltAndFailure,
            Wound::Smothering            => WoundCategory::Powerlessness,
        }
    }
}

/// Nine-cell moral alignment grid.
/// Schema had D&D names (`lawful_good`); YAML catalog used custom names (`rule_bound_prosocial`).
/// These variants use a neutral slug that maps to both; display strings in catalog data.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum Alignment {
    LawfulGood,
    NeutralGood,
    ChaoticGood,
    LawfulNeutral,
    TrueNeutral,
    ChaoticNeutral,
    LawfulEvil,
    NeutralEvil,
    ChaoticEvil,
}

/// Story roles — ~30 roles from character-roles.yaml, distilled into key categories.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum Role {
    Protagonist,
    Deuteragonist,
    Antagonist,
    LoveInterest,
    Mentor,
    Confidant,
    Ally,
    Foil,
    Catalyst,
    BridgeCharacter,
    GhostCharacter,
    Trickster,
    Guardian,
    Herald,
    Shapeshifter,
    Contagonist,
    WalkOn,
}

/// Five drive models from character-drives.yaml.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum DriveModel {
    /// Character acts from an unhealed wound
    Wound,
    /// Character acts toward a conscious want
    Desire,
    /// Character acts from obligation (honor, loyalty, law)
    Duty,
    /// Character acts from a distorted worldview
    Perception,
    /// Character acts from confrontation with mortality / meaninglessness
    Existential,
}

/// Character arc trajectory.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum ArcType {
    PositiveChange,
    NegativeFall,
    Flat,
    Minor,
    Disillusionment,
    Corruption,
    None,
}

// ── Narrative structure ───────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum ConflictType {
    PersonVsPerson,
    PersonVsSelf,
    PersonVsEnvironment,
    PersonVsSociety,
    PersonVsTechnology,
    PersonVsNature,
    Multiple,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum OutcomeType {
    FullDisaster,
    PartialDisaster,
    Dilemma,
    SuccessWithComplication,
    FullSuccess,
}

/// Scene sequence arc pattern.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum ArcPattern {
    SteadyBuild,
    PeaksAndValleys,
    Plateau,
    Spike,
    Descent,
}

/// Function of a scene within a sequence.
///
/// `Anchor` (formerly only in `SceneType`) is included here: a scene that
/// establishes context without advancing the conflict — the "steady ground"
/// before movement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum SceneFunction {
    /// Establishes context, character state, or setting — no conflict advance.
    Anchor,
    Setup,
    Complication,
    Escalation,
    Revelation,
    Climax,
    Resolution,
    Transition,
}

/// Scene type — now a type alias for `SceneFunction`.
///
/// The five former `SceneType` variants (`Anchor`, `Escalation`, `Revelation`,
/// `Climax`, `Resolution`) are all present in `SceneFunction`. Prefer
/// `SceneFunction` in new code; this alias preserves backward compatibility for
/// existing entity fields and serialized data.
pub type SceneType = SceneFunction;

/// Complexity rating for a scene.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum Complexity {
    Simple,
    Moderate,
    Complex,
}

/// Scene planning priority.
///
/// **Two scales are present for historical reasons:**
/// - Qualitative (scene narrative weight): `Critical`, `Important`, `Supporting`
/// - Generic (effort/order): `Low`, `Medium`, `High`
///
/// Prefer the qualitative scale (`Critical`/`Important`/`Supporting`) for scene
/// priority fields; the generic scale (`Low`/`Medium`/`High`) is kept for
/// backward compatibility with any stored data that uses it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum Priority {
    /// Pivotal scene — must land perfectly; cannot be cut.
    Critical,
    /// Scene materially advances plot or character arc.
    Important,
    /// Scene adds texture or support; could be trimmed.
    Supporting,
    /// Alias for `Supporting` (generic scale — prefer qualitative).
    Low,
    /// Alias for `Important` (generic scale — prefer qualitative).
    Medium,
    /// Alias for `Critical` (generic scale — prefer qualitative).
    High,
}

/// Tension level coarsened.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum TensionLevel {
    Low,
    Moderate,
    High,
    Peak,
}

/// Opening hook type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum OpeningType {
    InMediasRes,
    CharacteristicMoment,
    IncitingImage,
    VoiceForward,
    WorldImmersion,
    FramingDevice,
}

/// Narrative world delivery method.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum WorldDeliveryMethod {
    Immersive,
    Explanatory,
    Incremental,
    Contrast,
}

/// How the central narrative question is delivered.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum QuestionDelivery {
    Explicit,
    Implicit,
    Atmospheric,
    Character,
}

// ── Perspective & voice ───────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum PovType {
    FirstPerson,
    SecondPerson,
    ThirdPersonLimited,
    ThirdPersonMultiple,
    ThirdPersonOmniscient,
    ThirdPersonObjective,
    Epistolary,
}

/// Genette focalization spectrum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum FocalizationType {
    /// Narrator knows more than any character (zero restriction)
    Zero,
    /// Committed to one character's perspective throughout
    InternalFixed,
    /// Shifts between characters (chapter by chapter)
    InternalVariable,
    /// Same events through multiple characters
    InternalMultiple,
    /// Narrator knows less than characters (behaviourist)
    External,
}

/// Gardner's psychic distance scale (1 = very close, 5 = very distant).
///
/// Stored as a newtype so it can participate in serde and schemars while
/// encoding the 1–5 constraint at the type level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub struct PsychicDistance(u8);

impl PsychicDistance {
    pub fn new(value: u8) -> Result<Self, String> {
        if (1..=5).contains(&value) {
            Ok(Self(value))
        } else {
            Err(format!("PsychicDistance must be 1–5, got {value}"))
        }
    }

    pub fn value(self) -> u8 {
        self.0
    }
}

/// Dorrit Cohn's three modes of representing consciousness (third-person).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum ConsciousnessMode {
    /// Narrator describes the character's mind in narrator's language
    PsychoNarration,
    /// Free indirect discourse — narrator/character voices merge
    NarratedMonologue,
    /// Quoted interior monologue — typographically marked
    QuotedMonologue,
}

/// Bakhtin voice structure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum VoiceStructure {
    Monologic,
    Dialogic,
    Polyphonic,
}

/// Genette diegetic levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum DiegeticLevel {
    /// Narrator exists outside the story world
    Extradiegetic,
    /// Narrator exists within the story world
    Intradiegetic,
    /// Story within a story
    Metadiegetic,
}

/// Narrator knowledge scope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum KnowledgeScope {
    Omniscient,
    Limited,
    Objective,
    Varies,
}

/// Narrator reliability spectrum (Booth/Nünning).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum ReliabilitySpectrum {
    FullyReliable,
    MostlyReliable,
    Unreliable,
    RadicallyUnreliable,
}

/// Warhol engaging/distancing narrative mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum EngagementMode {
    Engaging,
    Distancing,
    Oscillating,
}

/// Narrative distance (prose style register).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum NarrativeDistance {
    Intimate,
    Medium,
    Distant,
}

/// Bordwell narration type (*Narration in the Fiction Film*, 1985).
///
/// Classifies how much information a narrative withholds or reveals relative
/// to character knowledge — a more precise alternative to `KnowledgeScope`
/// for analytical work.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum BordwellNarrationType {
    /// Narration bound to one character's knowledge; reader learns when they do
    Restricted,
    /// Narration ranges freely across characters and locations; may know more than any character
    Unrestricted,
    /// Narration withholds information the POV character possesses; reader knows less than character
    Suppressive,
    /// Narration foregrounds a formal parameter over story logic; style is itself the subject
    Parametric,
}

/// Bordwell self-consciousness scale (covert → metafictional).
///
/// Captures how overtly the narrative acknowledges its own act of narrating.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum NarrationSelfConsciousness {
    /// Invisible style — the narration pretends not to exist
    Covert,
    /// Slight signals, teases, irony — narration hints at its own presence
    ModeratelyCommunicative,
    /// Direct address to reader; metafictional commentary
    OvertlySelfConscious,
    /// The fiction acknowledges its own fictionality as its subject
    Metafictional,
}

// ── Prose style ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum SentenceLength {
    Short,
    Medium,
    Long,
    Varied,
}

/// Fish/Tufte sentence type taxonomy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum SentenceType {
    Additive,
    Subordinative,
    Periodic,
    Cumulative,
    Appositive,
    AbsolutePhrase,
    Fragment,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum VocabularyLevel {
    Plain,
    Moderate,
    Elevated,
    Specialized,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum ProfanityLevel {
    None,
    Mild,
    Moderate,
    Unrestricted,
}

// ── Sensory / world ───────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum DominantSense {
    Visual,
    Auditory,
    Tactile,
    Olfactory,
    Gustatory,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum TimeOfDay {
    Dawn,
    Morning,
    Midday,
    Afternoon,
    Dusk,
    Evening,
    Night,
    Midnight,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum Weather {
    Clear,
    Rain,
    Storm,
    Fog,
    Snow,
    Wind,
    Overcast,
}

/// Motif stage in the narrative arc.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum MotifStage {
    Introduction,
    Development,
    Payoff,
}

// ── Scene-centric training enums (v3.0) ──────────────────────────────────────

/// Core emotional states for character scene state tracking.
/// Aligned with `enums.schema.json#/$defs/emotion`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum Emotion {
    // Positive
    Joy, Contentment, Hope, Pride, Love, Gratitude, Relief,
    // Anger cluster
    Anger, Frustration, Resentment, Contempt, Disgust,
    // Fear cluster
    Fear, Anxiety, Dread, Panic,
    // Sadness cluster
    Sadness, Grief, Despair, Melancholy, Loneliness,
    // Shame cluster
    Shame, Guilt, Embarrassment, Humiliation,
    // Surprise cluster
    Surprise, Shock, Confusion, Disbelief,
    // Interest cluster
    Curiosity, Interest, Anticipation,
    // Neutral cluster
    Boredom, Apathy, Resignation,
    // Jealousy
    Jealousy, Envy,
    // Meta states
    Neutral, Conflicted, Mixed,
}

/// Scene tone/emotional register.
/// Aligned with `enums.schema.json#/$defs/tone`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum Tone {
    // Tender
    Tender, Intimate, Warm, Affectionate,
    // Humorous
    Playful, Humorous, Ironic, Satirical, Sardonic,
    // Tense
    Tense, Anxious, Ominous, Menacing, Threatening,
    // Confrontational
    Confrontational, Hostile, Aggressive,
    // Sad
    Melancholic, Elegiac, Mournful, Somber,
    // Optimistic
    Hopeful, Optimistic, Triumphant,
    // Urgent
    Desperate, Frantic, Chaotic,
    // Calm
    Calm, Peaceful, Contemplative, Reflective,
    // Formal
    Formal, Ceremonial, Dignified,
    // Casual
    Casual, Everyday, Mundane,
    // Mysterious
    Mysterious, Eerie, Uncanny,
    // Romantic
    Romantic, Passionate, Erotic,
    // Clinical
    Clinical, Detached, ToneNeutral,
}

/// What gives one character power over another.
/// Aligned with `enums.schema.json#/$defs/power_source`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum PowerSource {
    Economic,
    SocialRank,
    ProfessionalAuthority,
    Physical,
    KnowledgeSecret,
    KnowledgeExpertise,
    MoralAuthority,
    EmotionalLeverage,
    AgeSeniority,
    Charisma,
    Institutional,
    Legal,
    Familial,
    None,
}

/// What one character wants from another.
/// Aligned with `enums.schema.json#/$defs/want_type`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum WantType {
    // Approval
    Approval, Acceptance, Validation, Respect,
    // Connection
    WantLove, Affection, Intimacy, Connection,
    // Information
    Information, Truth, Honesty, Explanation,
    // Acknowledgment
    Acknowledgment, Apology, Justice, Revenge,
    // Distance
    Distance, Escape, Freedom, Independence,
    // Protection
    Protection, Safety, Security,
    // Control
    Control, Obedience, Submission,
    // Support
    Help, Support, Cooperation,
    // Reconciliation
    Forgiveness, Reconciliation,
    // Silence
    Silence, Discretion,
}

/// The deeper issue beneath surface disagreements.
/// Aligned with `enums.schema.json#/$defs/underlying_conflict`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum UnderlyingConflict {
    RespectVsContempt,
    TrustVsSuspicion,
    ControlVsAutonomy,
    WorthVsInadequacy,
    BelongingVsRejection,
    TruthVsDeception,
    DutyVsDesire,
    LoyaltyVsSelfInterest,
    PrideVsHumility,
    ClassPrejudice,
    CulturalClash,
    GenerationalGap,
    PastInjury,
    UnspokenHistory,
    CompetingValues,
    IdentityConflict,
    PowerStruggle,
    ResourceConflict,
    ConflictNone,
}

/// Category of what prevents character from achieving objective.
/// Aligned with `enums.schema.json#/$defs/obstacle_type`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum ObstacleType {
    OpposingCharacter,
    SocialNorms,
    PhysicalBarrier,
    TimePressure,
    MissingInformation,
    InternalResistance,
    CompetingObligation,
    ResourceLack,
    SkillLack,
    AuthorityFigure,
    InstitutionalObstacle,
    Environmental,
    EmotionalBlock,
    ObstacleNone,
}

/// How a character pursues their objective.
/// Aligned with `enums.schema.json#/$defs/tactic`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum Tactic {
    // Direct
    DirectRequest, Demand, Persuasion, Manipulation,
    // Charm
    Charm, Flattery, Seduction, Bribery,
    // Threat
    Threat, Intimidation, Aggression,
    // Avoidance
    Deflection, Avoidance, Withdrawal, TacticSilence,
    // Honesty
    TacticHonesty, Confession, Vulnerability,
    // Deception
    Deception, Misdirection, Omission,
    // Appeal
    AppealToEmotion, AppealToLogic, AppealToAuthority,
    // Negotiation
    Bargaining, Compromise, TacticSubmission,
    // Defiance
    Defiance, Mockery, Sarcasm,
    // Observation
    PassiveObservation, InformationGathering,
}

/// Markers of free indirect discourse.
/// Aligned with `enums.schema.json#/$defs/fid_marker`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum FidMarker {
    ExclamatorySyntax,
    CharacterVocabulary,
    UnattributedQuestions,
    PresentTenseFeelings,
    DeicticShifts,
    EvaluativeLanguage,
    ColloquialRegister,
    TagQuestions,
}

/// How Gricean maxim violations manifest in dialogue.
/// Aligned with `enums.schema.json#/$defs/violation_signature`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum ViolationSignature {
    OverExplanation,
    UnderExplanation,
    SingleSyllableShutdown,
    TopicChange,
    AnsweringDifferentQuestion,
    ExcessiveDetail,
    VaguenessFromPreciseSpeaker,
    FormalityShift,
    RepetitionUnderPressure,
    FalsePleasantry,
}

/// Social role character inhabits in scene.
/// Aligned with `enums.schema.json#/$defs/social_role`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum SocialRole {
    SocialAuthorityFigure,
    Subordinate,
    EqualPeer,
    SocialRival,
    SocialMentor,
    Student,
    Protector,
    SocialDependent,
    Host,
    Guest,
    SocialStranger,
    SocialIntimate,
    Professional,
    Client,
    Servant,
    Master,
    Elder,
    Youth,
    SocialParent,
    Child,
    Sibling,
    Suitor,
    Beloved,
    ExPartner,
    Spouse,
    SocialAlly,
    Adversary,
    NeutralParty,
}

// ── Setting instance enums (v3.0) ─────────────────────────────────────────────

/// Category of setting location.
/// Aligned with `enums.schema.json#/$defs/setting_type`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum SettingType {
    EstateInterior, EstateExterior, ManorHouse, Cottage,
    PublicBuilding, ReligiousBuilding, GovernmentBuilding,
    Commercial, InnTavern, Market,
    DomesticModest, DomesticGrand, DomesticPoor,
    Transport, Carriage, Ship, OnFoot,
    NaturalLandscape, Garden, Forest, Field, WaterBody,
    UrbanStreet, UrbanSquare, UrbanAlley,
    Military, Prison, Hospital, School,
}

/// Specific room type within a building.
/// Aligned with `enums.schema.json#/$defs/room_type`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum RoomType {
    DrawingRoom, SittingRoom, Parlor, Library, Study,
    DiningRoom, Ballroom, Hall, Gallery,
    Bedroom, DressingRoom, Nursery,
    Kitchen, ServantsQuarters, Cellar, Attic,
    EntranceHall, Corridor, Stairway,
    GardenRoom, Conservatory, Terrace,
    Office, ShopFloor, Workshop,
    RoomNone,
}

/// Qualitative aspect of the space.
/// Aligned with `enums.schema.json#/$defs/location_qualifier`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum LocationQualifier {
    Grand, Modest, Shabby, Intimate, Vast,
    Private, Public, SemiPublic,
    Transitional, Utilitarian, Ceremonial, Sacred,
    Familiar, Foreign, Contested,
}

/// Atmospheric weather conditions.
/// Aligned with `enums.schema.json#/$defs/weather_condition`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum WeatherCondition {
    WxClear, PartlyCloudy, WxOvercast, Threatening,
    RainLight, RainHeavy, RainStarting, RainEnding,
    SnowLight, SnowHeavy, Sleet, Hail,
    WxFog, Mist, Haze,
    WxStorm, Thunderstorm, WindLight, WindStrong, Gale,
    HeatOppressive, ColdBitter, Humid, Dry,
    SeasonalTypical, Unseasonable, WeatherNone,
}

/// Primary light source.
/// Aligned with `enums.schema.json#/$defs/lighting_source`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum LightingSource {
    SunlightDirect, SunlightDiffused, SunlightFiltered,
    Moonlight, Starlight,
    CandleSingle, CandlesMultiple, Chandelier,
    Fireplace, Torch, Lantern, LampOil,
    GasLight, Electric,
    LightNone, LightMixed,
}

/// Quality of illumination.
/// Aligned with `enums.schema.json#/$defs/lighting_quality`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum LightingQuality {
    Bright, WellLit, Adequate, Dim, Dark, PitchBlack,
    Harsh, Soft, LightWarm, LightCold,
    Flickering, Steady, Shifting,
    Dappled, Shadowed, Contrasting,
}

/// Thematic motif category.
/// Aligned with `enums.schema.json#/$defs/motif_category`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum MotifCategory {
    // Confinement
    Entrapment, MotifFreedom, Confinement, MotifEscape,
    // Vision
    Sight, Blindness, Perception, Revelation,
    // Movement
    Journey, Arrival, Departure, Threshold,
    // Nature vs civilization
    Nature, Civilization, Wilderness, MotifGarden,
    // Time
    MotifTime, Decay, Growth, Season,
    // Reflection
    Mirror, Reflection, Doubling, Shadow,
    // Elements
    Water, Fire, Earth, Air,
    // Light/dark
    Light, Darkness, MotifDawn, MotifDusk,
    // Identity
    Mask, Performance, Authenticity,
    // Knowledge
    Letter, Book, Document, Secret,
    // Sustenance
    Food, Hunger, Feast, Poison,
    // Body
    Blood, Wound, Healing,
    // Sound
    Music, MotifSilence, Noise,
}

/// Category of physical object in scene.
/// Aligned with `enums.schema.json#/$defs/prop_category`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum PropCategory {
    // Furniture
    FurnitureSeating, FurnitureTable, FurnitureStorage, FurnitureBed,
    // Documents
    DocumentLetter, DocumentBook, DocumentLegal, DocumentNews,
    // Clothing
    ClothingWorn, ClothingRemoved, Accessory, Jewelry,
    // Food/drink
    PropFood, Drink, DiningImplement,
    // Personal
    PersonalItem, Keepsake, Gift,
    // Architectural
    ArchitecturalDoor, ArchitecturalWindow, ArchitecturalFireplace, ArchitecturalStair,
    // Nature
    NaturePlant, NatureAnimal, NatureWeatherRelated,
    // Weapons
    WeaponBlade, WeaponFirearm, WeaponImprovised,
    // Objects
    PropLightSource, Timepiece, PropMirror,
    MusicalInstrument, ArtObject, ReligiousObject,
    Container, Vehicle, Tool,
    SymbolicObject,
}

/// Character's physical position in space.
/// Aligned with `enums.schema.json#/$defs/position_type`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum PositionType {
    Seated, Standing, Pacing, Lying, Kneeling,
    AtThreshold, AtWindow, AtFireplace, AtTable,
    CenterRoom, Corner, Background, Foreground,
    BlockingExit, NearExit, Isolated, Clustered,
}

/// Type of physical movement.
/// Aligned with `enums.schema.json#/$defs/movement_type`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum MovementType {
    Enters, Exits, Approaches, Retreats, Circles,
    Rises, Sits, TurnsToward, TurnsAway,
    CrossesTo, RemainsStill, Gestures,
    TouchesObject, HandlesProp, DiscardsProp,
    TouchesCharacter, WithdrawsContact,
    MovePaces, Freezes, Collapses,
}

/// Category of sensory detail.
/// Aligned with `enums.schema.json#/$defs/sensory_type`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum SensoryType {
    // Visual
    VisualLight, VisualColor, VisualMovement, VisualTexture,
    // Auditory
    AuditoryVoice, AuditoryMusic, AuditoryNature, AuditoryMechanical, AuditorySilence,
    // Tactile
    TactileTemperature, TactileTexture, TactilePressure, TactileMoisture,
    // Olfactory
    OlfactoryPleasant, OlfactoryUnpleasant, OlfactoryFood, OlfactoryNature, OlfactoryHuman,
    // Gustatory
    GustatorySweet, GustatoryBitter, GustatorySavory, GustatorySour,
}

// ── Scene context enums (v3.0) ────────────────────────────────────────────────

/// Thread type for narrative tracking.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum ThreadType {
    Plot,
    Relationship,
    Thematic,
    Mystery,
    CharacterArc,
}

/// Thread status in the narrative.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum ThreadStatus {
    Introduced,
    Developing,
    Escalating,
    Pivoting,
    Resolving,
    Dormant,
}

/// Motif deployment stage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum MotifDeployment {
    MotifIntroduced,
    Reinforced,
    Varied,
    Inverted,
    Culminating,
}

/// Narrative time order (Genette).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum NarrativeTimeOrder {
    NtChronological,
    NtAnalepsis,
    NtProlepsis,
    Braided,
}

/// Scene chapter position.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum ChapterPosition {
    Opening,
    Early,
    Middle,
    Late,
    Closing,
    EntireChapter,
}

/// Character arc beat position.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum PersonalArcBeat {
    Ordinary,
    Disruption,
    Refusal,
    Threshold,
    Tests,
    Approach,
    Ordeal,
    Reward,
    RoadBack,
    DarkNight,
    Resurrection,
    Elixir,
}

/// Knowledge domain for character state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum KnowledgeDomain {
    CharacterTrait,
    KdRelationship,
    PastEvent,
    KdSecret,
    SocialStatus,
    Plan,
    Belief,
    Capability,
}

/// Certainty level for knowledge items.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum KnowledgeCertainty {
    Witnessed,
    ToldDirectly,
    Inferred,
    Rumored,
    Assumed,
    Suspected,
}

/// Character role in scene (for role-based references).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum SceneRole {
    SceneProtagonist,
    SceneDeuteragonist,
    SceneAntagonist,
    SceneLoveInterest,
    SceneAlly,
    SceneRival,
    SceneMentor,
    SceneAuthority,
    SceneDependent,
    FamilyMember,
    Confidant,
    Foil,
    Catalyst,
    Stranger,
    Narrator,
    AbsentParty,
    OtherPresent,
    #[serde(rename = "self")]
    #[strum(serialize = "self")]
    RoleSelf,
    ThisChar,
    RoleNone,
}

/// Scene polarity for turn tracking (dramatic direction).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum ScenePolarity {
    Hope, Fear, Trust, Suspicion,
    Ignorance, Knowledge, Connection, Isolation,
    Control, Chaos, Safety, Danger,
    Certainty, Doubt, Love, Hate,
    Calm, Agitation, Power, Powerlessness,
    Belonging, Alienation, Clarity, Confusion,
}

/// Thematic question being explored.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum ThematicQuestion {
    CanLoveOvercomeObstacles,
    IsPersonalChangePossible,
    DoesTruthMatterMoreThanPeace,
    CanWoundsHeal,
    IsJusticeAttainable,
    DoesSocialClassDetermineFate,
    CanPrideBeOvercome,
    DoesVirtueGetRewarded,
    IsForgivenessPossible,
    CanDutyCoexistWithDesire,
    DoesLoyaltyDemandSacrifice,
    IsHappinessAChoice,
    CanThePastBeEscaped,
    DoesPowerCorrupt,
    IsAuthenticityWorthTheCost,
    CanLoveSurviveDeception,
    IsRedemptionEarnedOrGiven,
    DoesKnowledgeBringHappiness,
}

/// Speech register (formality level in dialogue).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum SpeechRegister {
    FormalElevated,
    FormalStandard,
    PoliteNeutral,
    CasualFamiliar,
    IntimatePrivate,
    ArchaicPeriod,
    DialectRegional,
    SlangSubcultural,
    ProfessionalTechnical,
    CeremonialRitual,
}

/// Narrative voice register (stylistic register).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum NarrativeVoiceRegister {
    Archaic,
    PeriodAppropriate,
    Contemporary,
    FormalLiterary,
    Conversational,
    Lyrical,
    Journalistic,
    Academic,
    Vernacular,
}

// ── Speech act theory (Searle) ────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum SpeechAct {
    /// Describes a state of affairs (assert, conclude, deny)
    Assertive,
    /// Attempts to get the hearer to do something (request, order, plead)
    Directive,
    /// Commits the speaker to a future action (promise, threaten, offer)
    Commissive,
    /// Expresses psychological states (apologise, thank, congratulate)
    Expressive,
    /// Changes a state in the world (declare war, pronounce married)
    Declaration,
}

// ── Gaze (Mulvey) ────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum GazeType {
    Active,
    Passive,
    Power,
    Counter,
    Internalized,
}

// ── Lejeune pact ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum PactType {
    Autobiographical,
    Fictional,
    Autofictional,
}

// ── Greimas actantial model ───────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum Actant {
    Subject,
    Object,
    Sender,
    Receiver,
    Helper,
    Opponent,
}

// ── Dialogue / subtext ────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum GriceanMaxim {
    Quantity,
    Quality,
    Relation,
    Manner,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum DisplacementGap {
    Small,
    Medium,
    Large,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum NonResponseType {
    Deflection,
    Silence,
    TopicChange,
    OverAnswer,
}

// ── Revision flags ─────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum RevisionFlag {
    TellingNotShowing,
    VoiceContractFail,
    PivotUnclear,
    SubtextMissing,
    PacingDrag,
    ContinuityBreak,
}

// ── Audiobook / TTS ───────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum SegmentType {
    Narration,
    Dialogue,
    AiSpeech,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum VoiceTone {
    Neutral,
    Urgent,
    Tense,
    Tender,
    Formal,
}

/// Audio priority setting for the oral-performance design.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum AudioPriority {
    Primary,
    Secondary,
    Equal,
}

// ── Ong's oral architecture (oral-performance-design) ─────────────────────────

/// Coarsened sentence architecture for oral/audiobook performance design.
///
/// This is a projection of `SentenceType` onto three oral-delivery shapes.
/// Use `SentenceType::architecture()` or `SentenceArchitecture::from(st)` to
/// derive this value from a fine-grained `SentenceType` rather than setting
/// it independently.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum SentenceArchitecture {
    Additive,
    Subordinative,
    /// All sentence types that mix or transcend the additive/subordinative split.
    Mixed,
}

impl SentenceType {
    /// Returns the coarsened oral-architecture category for this sentence type.
    ///
    /// `Additive` and `Subordinative` map directly.  All other sentence types
    /// (`Periodic`, `Cumulative`, `Appositive`, `AbsolutePhrase`, `Fragment`)
    /// map to `Mixed` because they don't fit cleanly into either oral pattern.
    pub fn architecture(self) -> SentenceArchitecture {
        match self {
            SentenceType::Additive      => SentenceArchitecture::Additive,
            SentenceType::Subordinative => SentenceArchitecture::Subordinative,
            _                           => SentenceArchitecture::Mixed,
        }
    }
}

impl From<SentenceType> for SentenceArchitecture {
    fn from(st: SentenceType) -> Self {
        st.architecture()
    }
}

// ── Paratext zones (Genette Seuils) ──────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum ParatextZone {
    Peritext,
    Title,
    Subtitle,
    Epigraph,
    Dedication,
    ChapterTitles,
    AuthorsNote,
    Footnote,
    Glossary,
    Appendix,
    Epitext,
}

// ── Session log ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum SessionStatus {
    DraftingNew,
    Revising,
    Outlining,
    Research,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum SessionDifficulty {
    Easy,
    Moderate,
    Hard,
}

// ── Craft quality rating (post-draft assessment) ─────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum CraftRating {
    Strong,
    Adequate,
    Weak,
}

// ── Promise tracking (hermeneutic code) ──────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum PromiseStatus {
    Open,
    Developing,
    Answered,
    Abandoned,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum PromisePhase {
    Plant,
    Reminder,
    Payoff,
}

// ── Beat act position ─────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum Act {
    ActOne,
    ActTwo,
    ActTwoA,
    ActTwoB,
    ActThree,
}

// ── Genette: narrative time ────────────────────────────────────────────────────

/// Genette *Narrative Discourse* (1972): temporal ORDER — the relation between
/// story-time sequence and discourse-time sequence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum TemporalOrder {
    /// Default: story sequence == discourse sequence.
    Chronological,
    /// Flashback — discourse moves backward in story time.
    Analepsis,
    /// Flash-forward — discourse anticipates future events.
    Prolepsis,
    /// Story begins at a point of action; prior context delivered through analepsis.
    InMediasRes,
}

/// Genette: temporal DURATION — the ratio of narrative time to story time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum TemporalDuration {
    /// Gap — time passes with zero narrative space (chapter breaks, omitted events).
    Ellipsis,
    /// Events compressed; narrative time < story time.
    Summary,
    /// Roughly equal pacing: narrative time ≈ story time (dialogue, action).
    Scene,
    /// Slow-down; narrative time > story time (stream of consciousness, lyric).
    Stretch,
    /// Story time halts entirely; static description or meditation.
    Pause,
}

/// Genette: temporal FREQUENCY — how many times an event is narrated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum TemporalFrequency {
    /// One event told once. Default.
    Singulative,
    /// One narration represents a class of repeated events ("every morning she…").
    Iterative,
    /// The same event narrated more than once, from different angles (*Rashomon*, *As I Lay Dying*).
    Repetitive,
    /// The same *type* of event told each time it occurs, building an escalating series.
    CumulativeSingulative,
}

// ── Sternberg: reader experience effects ──────────────────────────────────────

/// Sternberg *Expositional Modes and Temporal Ordering in Fiction* (1978) /
/// Iser *The Act of Reading* (1978): the primary effects of narrative information management.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum ReaderExperienceEffect {
    /// Reader doesn't know what happened in the past — withheld backstory.
    Curiosity,
    /// Reader doesn't know what will happen — uncertain future outcome.
    Suspense,
    /// Reader held a confident belief that turns out to be wrong.
    Surprise,
    /// Reader actively fills interpretive gaps (Iser indeterminacy).
    GapFilling,
    /// Reader knows something a character doesn't — structural dramatic irony.
    DramaticIrony,
}

// ── Genette: transtextuality ───────────────────────────────────────────────────

/// Genette *Palimpsests* (1982): the five types of transtextual relation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum IntertextualRelation {
    /// Co-presence: quotation, allusion, plagiarism.
    Intertextuality,
    /// Threshold relation: titles, prefaces, epigraphs, blurbs.
    Paratextuality,
    /// Commentary: a text commenting on another (criticism, gloss, review).
    Metatextuality,
    /// Transformation: hypertext derived from a hypotext (parody, pastiche, adaptation).
    Hypertextuality,
    /// Taxonomic: unmarked architectural relation to genre and archetype.
    Architextuality,
}

/// Bloom *The Anxiety of Influence* (1973): six revisionary ratios by which a
/// later writer wrestles free from a precursor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum BloomInfluenceMode {
    /// Swerve — creative misreading that opens a new poetic space.
    Clinamen,
    /// Antithetical completion — extending or "completing" the precursor's work.
    Tessera,
    /// Discontinuity — emptying out the precursor's claim on the sublime.
    Kenosis,
    /// Counter-sublime — demonizing the precursor's strength as a defense.
    Daemonization,
    /// Purgation — self-curtailment to separate the self from the precursor.
    Askesis,
    /// Return — the later poem seems to have been anticipated by the precursor.
    Apophrades,
}

// ── Metaphor and figurative language ──────────────────────────────────────────

/// Figurative language type; anchors to Lakoff/Johnson, Richards, and Black
/// frameworks in `07-drafting/theory-reference/metaphor-and-figurative-language.md`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum MetaphorType {
    /// Lakoff/Johnson: one conceptual domain understood through another (ARGUMENT IS WAR).
    Conceptual,
    /// Fossilized; figurative origin no longer recognized ("leg" of a table).
    Dead,
    /// Two incompatible metaphors conflated in the same phrase.
    Mixed,
    /// Sustained and developed at significant length.
    Extended,
    /// Elaborate, often paradoxical extended metaphor (Donne, Herbert).
    Conceit,
    /// Explicit comparison using "like" or "as".
    Simile,
    /// Part stands for whole, or whole for part.
    Synecdoche,
    /// Adjacent concept substituted for the intended referent (crown for royalty).
    Metonymy,
    /// Abstract or non-human entity given human qualities.
    Personification,
    /// Speaker addresses an absent, dead, or non-human entity.
    Apostrophe,
}

// ── Comedy and irony ───────────────────────────────────────────────────────────

/// Booth/Hutcheon irony taxonomy (v3.0.7 aligned)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum IronyType {
    /// Saying the opposite of what you mean; sarcasm, understatement, litotes
    Verbal,
    /// Events contradict reasonable expectations (the fire station burns down)
    Situational,
    /// Audience knows something a character doesn't (Shakespeare, most thrillers)
    Dramatic,
    /// The text's structure undermines or ironizes its apparent argument
    Structural,
    /// Fate or the universe mocks human aspiration; cosmic scale
    Cosmic,
    /// Feigning ignorance to expose another's foolishness (Columbo, Socrates)
    Socratic,
    /// Author breaks the fictional frame to comment on the narrative's artificiality
    Romantic,
    /// No irony marked
    None,
}

/// Comic mode — which register of humor defines a story or passage (v3.0.7 aligned)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum ComicMode {
    /// Critiques institutions, ideologies, or human folly from a superior external position
    Satire,
    /// Exposes the gap between social performance and private reality (Austen, Wilde)
    Manners,
    /// Physical comedy, improbable situations, escalating chaos (Wodehouse, Feydeau)
    Farce,
    /// Humor drawn from dark, painful, or taboo material
    DarkComedy,
    /// The world itself is irrational; logic fails (Beckett, Ionesco)
    Absurdist,
    /// An outsider/rogue moves through society, exposing its contradictions
    Picaresque,
    /// Love story where humor arises from misunderstanding, denial, and social performance
    RomanticComedy,
    /// Humor delivered without any signal that it's funny
    Deadpan,
    /// No comic mode marked
    None,
}

// ── Affect theory ──────────────────────────────────────────────────────────────

/// Ahmed/Berlant affect theory: textual affect operating in scene (v3.0.8)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum TextualAffectType {
    /// Attachment to object/fantasy that actively impedes flourishing (Berlant)
    CruelOptimism,
    /// Emotion adheres to objects, accretes over time, circulates (Ahmed)
    AffectiveStickiness,
    /// Minor affects — envy, anxiety, irritation — vs. grand passions (Ngai)
    UglyFeelings,
    /// Attrition of life under late capitalism; ordinary crisis (Berlant)
    SlowDeath,
    /// Suspension of agency; stuck between old and new forms (Berlant)
    Impasse,
    /// Loss of bearings, spatial/temporal confusion (Ahmed)
    Disorientation,
    /// No textual affect type marked
    None,
}

// ── Unnatural narratology ──────────────────────────────────────────────────────

/// Richardson unnatural narratology: anti-mimetic techniques (v3.0.8)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum AntiMimeticTechnique {
    /// Dead, unborn, or omniscient first-person narrator (impossible epistemic access)
    ImpossibleNarrator,
    /// Events are negated or un-happen after being narrated
    Denarration,
    /// Logical impossibilities coexist in the storyworld
    ContradictoryStoryworld,
    /// Time runs backward, scrambled, or acausal
    AntiTemporal,
    /// Multiple incompatible versions of events presented
    Permutation,
    /// "You" as protagonist creates defamiliarization
    SecondPersonAntiMimetic,
    /// No anti-mimetic technique
    None,
}

// ── Event significance ─────────────────────────────────────────────────────────

/// Chatman kernel/satellite distinction for event significance (v3.0.8)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum EventSignificance {
    /// Plot-changing event; cannot be removed without altering story logic
    Kernel,
    /// Texture/elaboration; can be removed without changing plot
    Satellite,
}

// ── Prince narratee theory ─────────────────────────────────────────────────────

/// Prince narratee types (v3.0.9)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum NarrateeType {
    /// Named or directly addressed within the text
    Explicit,
    /// Constructed by narration but not named
    Implicit,
    /// Narratee as character with reactions/responses
    Dramatized,
    /// Unmarked, universal reader position
    ZeroDegree,
    /// No narratee marked
    None,
}

// ── Burke dramatism ────────────────────────────────────────────────────────────

/// Burke Pentad focus (v3.0.9)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum PentadFocus {
    /// What happened — the action itself dominates
    Act,
    /// Where/when — setting as determinant
    Scene,
    /// Who did it — character agency dominates
    Agent,
    /// How/by what means — method, instrument
    Agency,
    /// Why — motivation, intention, goal
    Purpose,
}

/// Which theory of comedy *grounds* the humor — diagnostic, not mutually exclusive.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum ComedyTheory {
    /// Laughter from violated expectations (Kant, Schopenhauer, Kierkegaard).
    Incongruity,
    /// Laughter from superiority over a comic target (Hobbes, Bergson's "rigidity").
    Superiority,
    /// Laughter releases psychic tension; taboo content in safe form (Freud, Spencer).
    Relief,
}

// ── Trauma narratology ─────────────────────────────────────────────────────────

/// LaCapra *Writing History, Writing Trauma* (2001) / Caruth (1996) /
/// Felman & Laub (1992): trauma as narrative form.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum TraumaMode {
    /// Trauma returns belatedly, fragmented and compulsive — not integrated (Caruth).
    Belatedness,
    /// Compulsive repetition without integration; the wound governs behavior (LaCapra).
    ActingOut,
    /// Conscious integration; the character begins to narrate and contextualize (LaCapra).
    WorkingThrough,
    /// Ethically attuned bearing of witness to another's trauma (Felman/Laub).
    Witnessing,
    /// Passive consumption of another's trauma — risks exploitation (Felman/Laub).
    Spectating,
}

/// Herman *Trauma and Recovery* (1992): the three-stage recovery arc.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum HermanRecoveryStage {
    /// Establishing physical and psychological safety; stabilizing the survivor.
    Safety,
    /// Remembrance and mourning — constructing the narrative of the traumatic event.
    RemembranceAndMourning,
    /// Rebuilding a new life and a new relationship with the world.
    Reconnection,
}

// ── Burke's Dramatistic Pentad ─────────────────────────────────────────────────

/// Burke *A Grammar of Motives* (1945): the five constitutive elements.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum PentadElement {
    /// What was done — the event, deed, or change.
    Act,
    /// Where/when it happened — context, environment, conditions.
    Scene,
    /// Who did it — the motive-bearer.
    Agent,
    /// How it was done — the means, instrument, or method.
    Agency,
    /// Why it was done — the value or goal at stake.
    Purpose,
}

/// The *dominant ratio* in a character's implicit philosophy of action — which
/// pentadic element they privilege as the primary explanatory frame.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum PentadRatio {
    SceneAct,
    SceneAgent,
    SceneAgency,
    ScenePurpose,
    ActAgent,
    ActAgency,
    ActPurpose,
    AgentAgency,
    AgentPurpose,
    AgencyPurpose,
}

/// Burke's four form types: how narrative creates and fulfills expectations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum BurkeFormType {
    /// Creates and fulfills desire in sequence (syllogistic or qualitative progression).
    Progressive,
    /// The same gesture repeated with variations.
    Repetitive,
    /// Satisfies expectations the genre or form itself has already established.
    Conventional,
    /// Local devices: antithesis, chiasmus, alliteration — not architectural.
    Minor,
}

// ── Hayden White / Frye: emplotment ───────────────────────────────────────────

/// Hayden White *Metahistory* (1973) / Frye *Anatomy of Criticism* (1957):
/// the four emplotment types (mythoi) available to narrative.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum EmplotmentType {
    /// Protagonist triumphs over the world; quest, redemption, wish-fulfillment.
    Romance,
    /// Protagonist falls; fatal flaw, inexorable consequence, catharsis.
    Tragedy,
    /// Hero integrates with society; reconciliation, renewal, festive ending.
    Comedy,
    /// Ironic distance on the other three; skepticism, ambiguity, deflation.
    Satire,
}

// ── Ricoeur: mimesis phases ────────────────────────────────────────────────────

/// Ricoeur *Time and Narrative* vol. 1 (1984): the three-phase mimesis cycle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum MimesisPhase {
    /// Pre-understanding of human action; the symbolic field the author draws on.
    Prefiguration,
    /// The narrative text itself; emplotment that mediates reality and imagination.
    Configuration,
    /// The reader's reception; the text realized in lived experience and action.
    Refiguration,
}

// ── Doležel: possible-worlds accessibility ────────────────────────────────────

/// Doležel *Heterocosmica* (1998): accessibility relations that govern which
/// possible states can coexist within a fictional universe.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum AccessibilityRelation {
    /// Laws of nature and logical possibility ("can this happen in this world?").
    Alethic,
    /// What characters know, believe, or are deceived about.
    Epistemic,
    /// What is permitted, forbidden, or obligated in the storyworld.
    Deontic,
    /// What is good, bad, beautiful, or worthy of praise.
    Axiological,
}

// ── Translation (Venuti) ───────────────────────────────────────────────────────

/// Venuti *The Translator's Invisibility* (1995): the core translation tension.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum TranslationStrategy {
    /// Smooths foreign elements; reads as if originally in the target language.
    Domestication,
    /// Preserves foreign texture; the translation feels like it comes from elsewhere.
    Foreignization,
}

/// Craft-level rendering strategy for a multilingual character's non-dominant language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum MultilingualStrategy {
    /// Reproduce the original language verbatim; assumes bilingual reader.
    NativeScript,
    /// Translate directly; linguistic difference is invisible.
    Translation,
    /// Reproduce with a gloss in brackets or italics immediately after.
    Gloss,
    /// Phonetic transcription conveys sound without intelligibility.
    Phonetic,
    /// Italicize to signal foreignness without translating.
    Italics,
}

// ── McCloud: graphic narrative ────────────────────────────────────────────────

/// McCloud *Understanding Comics* (1993): six panel-to-panel transition types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum PanelTransitionType {
    /// Two panels show a single moment from slightly different angles.
    MomentToMoment,
    /// A single subject in continuous action across two panels.
    ActionToAction,
    /// Different subjects within the same scene.
    SubjectToSubject,
    /// A shot change to a different scene, location, or time.
    SceneToScene,
    /// Different aspects of a place or mood; no advancing action.
    AspectToAspect,
    /// No logical relationship — pure stylistic or conceptual jump.
    NonSequitur,
}

/// Chute *Graphic Women* (2010): modes of visual-verbal relationship in
/// graphic narrative.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum VisualVerbalRelation {
    /// Words and pictures say essentially the same thing.
    Parallel,
    /// Each reinforces the other; together they say more.
    Additive,
    /// Words and pictures work in interdependence; neither alone conveys the full meaning.
    Interdependent,
    /// Words and pictures contradict or ironize each other.
    Contrapuntal,
}

// ── YA / children's fiction (Nikolajeva) ──────────────────────────────────────

/// Nikolajeva *The Rhetoric of Character in Children's Literature* (2002):
/// narrator types in YA and children's fiction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum YaNarratorType {
    /// Child narrator with limited understanding; creates dramatic irony for adult readers.
    Naive,
    /// Narrator knows more than they reveal; controlled ironic distance.
    Sophisticated,
    /// Adult narrator looking back at childhood; temporal gap creates reflection.
    Retrospective,
    /// Multiple child voices with no authoritative adult perspective imposed.
    Polyphonic,
}

// ── Indigenous narratology (Vizenor / Archibald) ──────────────────────────────

/// Vizenor *Manifest Manners* (1994): active presence vs. colonial construction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum SurvivranceMode {
    /// Active presence; stories that resist absence, dominance, and tragedy narratives.
    Survivance,
    /// Passive colonial construction: Indigenous subject as victim, absence, or pathology.
    Victimry,
}

/// Vizenor *Manifest Manners* (1994): extended survivance vocabulary.
/// Reference: `08-revision/critical-theory/indigenous-narratology.md`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum SurvivanceMode {
    /// Active presence; stories that resist absence, dominance, and tragedy narratives.
    Survivance,
    /// Passive colonial construction: Indigenous subject as victim, absence, or pathology.
    Victimry,
    /// Radical irony, indeterminacy, transformation; refusal of fixed identity.
    TricksterDiscourse,
    /// Colonial simulations of Indianness — not living self-representation.
    DeadVoices,
}

/// Archibald *Indigenous Storywork* (2008): Seven Principles of ethical
/// engagement with Indigenous stories.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum StoryworkProtocol {
    Respect,
    Responsibility,
    Reciprocity,
    Reverence,
    Holism,
    Interrelatedness,
    Synergy,
}

// ── Williams: structures of feeling ───────────────────────────────────────────

/// Raymond Williams *Marxism and Literature* (1977): cultural formation types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum CulturalStructure {
    /// The hegemonic; the established order that currently feels like common sense.
    Dominant,
    /// What is beginning to emerge in opposition to the dominant.
    Emergent,
    /// What was once dominant but is now archaic, vestigial, or nostalgic.
    Residual,
}

// ── Bachelard / de Certeau: space ─────────────────────────────────────────────

/// Bachelard *The Poetics of Space* (1958): intimate phenomenological space types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum IntimateSpaceType {
    /// Height, overview, aspiration — freedom from the weight of what lies below.
    Attic,
    /// Depth, the unconscious, the hidden — the irrational below the threshold.
    Cellar,
    /// Warmth, intimacy, the smallest habitable circle of home.
    Nest,
    /// Retreat, self-enclosure, solitude — smallest possible dwelling.
    Shell,
    /// Rest, refuge — the sheltered angle; cosiness in confinement.
    Corner,
    /// Container of secrets — memory storage space.
    Drawer,
    /// Container of treasures — the repository of depths.
    Chest,
    /// Liminal space — the point of passage between domains.
    Threshold,
}

/// De Certeau *The Practice of Everyday Life* (1980): enacted spatial opposites.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum SpatialPractice {
    /// Institutional / power-structural: plans, property, surveyed place — from above.
    Strategy,
    /// Marginal / improvisatory: moving through spaces without owning them — from below.
    Tactic,
}

// ── Revision: six-pass system ─────────────────────────────────────────────────

/// Murray / Elbow / Lamott six-pass revision framework.
/// Each pass targets a distinct level of the manuscript.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum RevisionPassType {
    /// Architecture: structure, sequence, act turns, sub-plot integration.
    Structural,
    /// Character consistency: voice, arc, motivation, relationship dynamics.
    Character,
    /// Sentence-level voice: rhythm, diction, psychic distance consistency.
    Voice,
    /// Line-by-line: cutting, verb strength, clarity, subtext craft.
    Line,
    /// Grammar, punctuation, usage.
    Copyedit,
    /// Final read for typos and typesetting issues before publication.
    Proofread,
}

// ── Seriality ──────────────────────────────────────────────────────────────────

/// How a book's arc relates to the larger series structure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum SerialArcType {
    /// Each book fully resolves; no series-level arc dependency.
    Standalone,
    /// Books are self-contained AND contribute to a series arc.
    StandaloneWithArc,
    /// Books are episodes playable in any order.
    Episodic,
    /// Books require reading in order; one long arc with a planned resolution.
    ClosedSeries,
    /// Series arc continues indefinitely; no planned endpoint.
    OpenSeries,
}

// ── Narrative ethics audit ─────────────────────────────────────────────────────

/// Ethical audit categories for the revision phase.
/// Grounds to Booth, Keen, Nussbaum, Spivak in `08-revision/critical-theory/`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum EthicsAuditCategory {
    /// Who is represented, how, and whether their humanity is fully rendered.
    Representation,
    /// Whose perspective structures the visual field; complicity in objectification?
    GazeAnalysis,
    /// Does the text invite empathy, and with whom?
    Empathy,
    /// Does the text invite the reader to collude with problematic attitudes?
    Complicity,
    /// Who has power and who doesn't — examined critically or reinforced?
    PowerDynamics,
    /// Booth's test: what does the narrative's ethical argument coerce from the reader?
    BoothCoduction,
}

// ── Semiotics (Peirce) ────────────────────────────────────────────────────────

/// Peirce's three sign types (*Collected Papers*).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum SignType {
    /// Sign that resembles what it represents (photograph, map, onomatopoeia).
    Icon,
    /// Sign caused by or indexically connected to its referent (smoke → fire, fever → illness).
    Index,
    /// Sign with arbitrary, conventional relationship to its referent (most words).
    Symbol,
}

// ── Jakobson: communicative functions ─────────────────────────────────────────

/// Jakobson *Closing Statement: Linguistics and Poetics* (1960):
/// the six functions of any communicative act.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum JakobsonFunction {
    /// Context-oriented: conveying information about the world.
    Referential,
    /// Sender-oriented: expressing attitude, emotion, subjective stance.
    Emotive,
    /// Receiver-oriented: commanding, requesting, persuading.
    Conative,
    /// Channel-oriented: checking or maintaining the communication channel ("hello?").
    Phatic,
    /// Code-oriented: discussing the language or code being used.
    Metalingual,
    /// Message-oriented: aesthetic self-focus; the poetic function.
    Poetic,
}

// ── Halliday: transitivity processes ──────────────────────────────────────────

/// Halliday's systemic functional grammar: the six process types.
/// Who does what to whom — the grammar of agency and representation in a sentence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum TransitivityProcess {
    /// Doing, happening, creating — physical action in the world.
    Material,
    /// Sensing, feeling, perceiving — inner mental experience.
    Mental,
    /// Being, having, attributing — identification or classification.
    Relational,
    /// Saying, telling, communicating — speech or text production.
    Verbal,
    /// Physiological and behavioral processes (sigh, cough, dream, tremble).
    Behavioral,
    /// Intransitive existence statements ("There was a woman…").
    Existential,
}

// ── Hutcheon: adaptation modes ────────────────────────────────────────────────

/// Hutcheon *A Theory of Adaptation* (2006): how an audience engages adapted narrative.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum AdaptationMode {
    /// Novels, poetry — narrative mediated through language alone.
    Telling,
    /// Film, theatre, opera — narrative shown through performance.
    Showing,
    /// Video games, interactive fiction — narrative navigated through participation.
    Interacting,
}

// ── Queer temporality ──────────────────────────────────────────────────────────

/// Halberstam *In a Queer Time and Place* (2005) / Edelman *No Future* (2004) /
/// Muñoz *Cruising Utopia* (2009).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum QueerTimeMode {
    /// The hegemonic temporal model: organized around reproduction, the child-future.
    ReproductiveFuturism,
    /// Halberstam: alternative temporalities around different life-markers (subculture, illness).
    QueerTemporality,
    /// Muñoz: utopian hope in the not-yet-here; glimpsed in collective performance.
    QueerFuturity,
}

/// Sedgwick/Rich/Butler/Edelman/Halberstam: combined queer narrative modes for audit.
/// Reference: `08-revision/critical-theory/queer-narratology.md`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum QueerMode {
    /// Sedgwick: narrative organized around the epistemology of the closet — concealment/revelation.
    ClosetEpistemology,
    /// Rich: the institution that makes heterosexuality appear natural and mandatory.
    CompulsoryHeterosexuality,
    /// Edelman: the hegemonic temporal model organized around reproduction & the child-future.
    ReproductiveFuturism,
    /// Halberstam: alternative temporalities around different life-markers (subculture, illness).
    QueerTemporality,
    /// Muñoz: utopian hope in the not-yet-here; glimpsed in collective performance.
    QueerFuturity,
}

// ── Disability representation ──────────────────────────────────────────────────

/// Garland-Thomson, Mitchell & Snyder, Siebers, Kafer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum DisabilityRepMode {
    /// Mitchell & Snyder: disability as narrative device without full subjectivity.
    NarrativeProsthesis,
    /// Siebers: disability as social/political position, not deviation from a norm.
    ComplexEmbodiment,
    /// Kafer: refusal of the cure narrative; openness to non-normative futures.
    CripFuturity,
    /// Garland-Thomson: the normate gaze that renders disability as spectacle.
    NormateLens,
}

// ── Affect theory (Ahmed / Berlant) ───────────────────────────────────────────

/// Ahmed *The Cultural Politics of Emotion* (2004) /
/// Berlant *Cruel Optimism* (2011).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum AffectMode {
    /// Ahmed: emotions that accumulate and adhere to objects through repetitive contact.
    Sticky,
    /// Berlant: attachment to an object whose attainment would damage the subject.
    CruelOptimism,
    /// Berlant: suspended between hope and its failure; temporal stasis.
    Impasse,
    /// Berlant/Nixon: cumulative, slow harm that does not register as acute crisis.
    SlowDeath,
    /// Ahmed: how bodies and subjects orient toward (or away from) objects and worlds.
    Orientation,
}

// ── Psychoanalytic Narratology (Freud / Lacan / Kristeva) ─────────────────────

/// Freudian mechanisms as applied to narrative production and character psychology.
/// Source: Freud *The Interpretation of Dreams* (1900), *Beyond the Pleasure Principle* (1920).
/// Reference: `07-drafting/theory-reference/psychoanalytic-narratology.md`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum FreudianMechanism {
    /// Merging of two (or more) ideas into a single composite image; dream-work logic.
    Condensation,
    /// An idea's affective charge transferred to a different, less threatening object.
    Displacement,
    /// The return of something familiar that has become estranged — *unheimlich*.
    Uncanny,
    /// Compulsive re-enactment of a traumatic situation without resolution.
    RepetitionCompulsion,
    /// Attributing one's own unacceptable impulse to another person.
    Projection,
    /// Constructing logical justification for an unconsciously motivated act.
    Rationalization,
    /// Redirecting an unacceptable impulse toward a socially sanctioned outlet.
    Sublimation,
    /// Banishment of a thought from consciousness; the engine of the symptom.
    Repression,
}

/// Lacan's three registers — the structural coordinates of psychic life.
/// Source: Lacan seminars; overview in *Écrits* (1966).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum LacanRegister {
    /// The pre-linguistic, unmediated, traumatic core that resists symbolization.
    Real,
    /// The domain of language, law, the Other; narrative and social identity.
    Symbolic,
    /// The domain of image, identification, misrecognition; the ego's mirror stage.
    Imaginary,
}

/// Kristeva's categories of the abject — that which threatens the boundary of self.
/// Source: Kristeva *Powers of Horror* (1980).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum AbjectCategory {
    /// Bodily waste and excretion that must be expelled to maintain selfhood.
    Corporeal,
    /// The maternal body that must be repudiated for the subject to individuate.
    Maternal,
    /// Ritual pollution and taboo objects; the sacred-in-reverse.
    ReligiousDefilement,
    /// The corpse — the ultimate abject, the body that was self but is now other.
    Death,
    /// Social or cultural categories that disturb identity and stable meaning.
    IdentityThreat,
}

// ── Postcolonial Narratology (Said / Spivak / Bhabha) ─────────────────────────

/// Postcolonial narrative modes and critical categories.
/// Sources: Said *Orientalism* (1978), Spivak *Can the Subaltern Speak?* (1988),
/// Bhabha *The Location of Culture* (1994), Fanon *Black Skin, White Masks* (1952).
/// Reference: `08-revision/critical-theory/postcolonial-narratology.md`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum PostcolonialMode {
    /// Said: the West's systematic representation of the East as exotic, static, inferior.
    Orientalism,
    /// Bhabha: cultural mixing that produces something new, irreducible to either source.
    Hybridity,
    /// Bhabha: the space of cultural translation where meaning is negotiated between cultures.
    ThirdSpace,
    /// Bhabha: colonial subject imitates colonizer — close but never identical; uncanny to power.
    Mimicry,
    /// Du Bois / Fanon: the divided consciousness of those who see themselves through the colonizer's eyes.
    DoubleConsciousness,
    /// Spivak: the subaltern cannot speak in structures that systematically exclude their voice.
    SubalternSilence,
    /// Active resistance to colonial epistemology and institutional structures.
    Decolonial,
    /// Bhabha: the colonizer's ambivalence — desire and dread toward the colonial other.
    Ambivalence,
    /// Achebe: narrative dehumanization through systematic reduction of a people to background.
    StructuralDehumanization,
    /// Ngũgĩ: the politics of which language carries authority and whose culture is validated.
    LanguagePolitics,
}

// ── Ecocriticism (Buell / Morton / Nixon / Haraway / Heise) ───────────────────

/// Ecocritical modes for representing environment and ecological crisis.
/// Sources: Buell *The Environmental Imagination* (1995), Morton *Ecology Without Nature* (2007),
/// Nixon *Slow Violence* (2011), Haraway *Staying with the Trouble* (2016).
/// Reference: `08-revision/critical-theory/ecocriticism.md`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum EcocriticalMode {
    /// Traditional nature as innocent refuge from civilization; idealized wilderness.
    Pastoral,
    /// Pastoral exposed as ideology — the violence beneath the natural idyll.
    AntiPastoral,
    /// The aestheticization of environmental catastrophe; sublime horror of collapse.
    ToxicSublime,
    /// Nixon: dispersed, attritional violence that accumulates invisibly over time — pollution, climate, deforestation.
    SlowViolence,
    /// Morton: ecology without a concept of pristine nature; the mesh, the strange stranger.
    DarkEcology,
    /// Haraway: webs of species relation; humans embedded in multi-species assemblages.
    Multispecies,
    /// Heise: scale problem — attachment to local place vs. sense of planetary belonging.
    SenseOfPlace,
    /// Heise: cognitivized world-citizenship; global risk as narrative subject.
    SenseOfPlanet,
    /// Environmental threat as eschatological; narrative shaped by extinction or collapse.
    Apocalyptic,
}

// ── Embodied Cognition (Johnson / Merleau-Ponty / Sheets-Johnstone) ───────────

/// Image schemas — recurrent kinesthetic-sensory gestalts that structure abstract thought.
/// Source: Johnson *The Body in the Mind* (1987); Lakoff & Johnson *Philosophy in the Flesh* (1999).
/// Reference: `07-drafting/theory-reference/embodied-narration.md`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum ImageSchema {
    /// In–out; bounded region with interior, exterior, and boundary.
    Containment,
    /// Oriented movement toward a goal: source, path, destination.
    SourcePathGoal,
    /// Application of energy; resistance and overcoming.
    Force,
    /// Equilibrium of counteracting forces.
    Balance,
    /// Physical connection establishing relationship.
    Link,
    /// Centralpoint surrounded by peripheral region.
    CenterPeriphery,
    /// Vertical orientation; up as positive, down as negative in most cultures.
    UpDown,
    /// Recurring process that returns to origin.
    Cycle,
    /// Obstacle preventing motion along a path.
    Blockage,
    /// The same entity in different conditions (caterpillar → butterfly structurally).
    Transformation,
}

// ── Prose Linguistics (Shklovsky / Leech-Short / Halliday extension) ──────────

/// Defamiliarization and foregrounding modes.
/// Sources: Shklovsky *Art as Technique* (1917); Leech & Short *Style in Fiction* (2007).
/// Reference: `07-drafting/theory-reference/prose-linguistics.md`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum DefamiliarizationMode {
    /// Shklovsky: perception dulled by habit — what was once strange is now invisible.
    Automatization,
    /// Shklovsky: art makes the familiar strange again; restores perception of form.
    Ostranenie,
    /// Leech/Short: deviation from a linguistic or literary norm (grammar, vocabulary, syntax).
    Deviation,
    /// Leech/Short: regular repetition of a pattern at any linguistic level.
    Parallelism,
    /// Making the reader perceive the technique itself — the brush strokes visible.
    ExposedConstruction,
    /// The deliberate breaking of established automatized pattern.
    PatternBreak,
}

// ── Structuralist / Semiotic (Greimas / Saussure / Propp) ─────────────────────

/// Greimas's semiotic square — the four positions of the logical square.
/// Source: Greimas *Structural Semantics* (1966).
/// Reference: `01-concept/references/semiotic-square-reference.md`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum SemioticSquarePosition {
    /// S1: the primary term (e.g. Life, Freedom, Good).
    S1,
    /// S2: the contrary of S1 (e.g. Death, Constraint, Evil).
    S2,
    /// Not-S2: the contradictory of S2; implies S1 but is not identical to it.
    NotS2,
    /// Not-S1: the contradictory of S1; implies S2 but is not identical to it.
    NotS1,
    /// Complex term: both S1 and S2 simultaneously held.
    Complex,
    /// Neutral term: neither S1 nor S2 (the unmarked middle).
    Neutral,
}

// ── Experimental Narration (Booth / Waugh / McHale) ───────────────────────────

/// Experimental narration modes — deviations from conventionally transparent narration.
/// Sources: Booth *The Rhetoric of Fiction* (1961), Waugh *Metafiction* (1984),
/// McHale *Postmodernist Fiction* (1987).
/// Reference: `07-drafting/theory-reference/experimental-narration.md`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum ExperimentalNarrationMode {
    /// Booth: narrator whose testimony is systematically undermined by the implied author.
    UnreliableNarrator,
    /// Waugh/McHale: fiction that foregrounds its own constructedness; the frame breaks.
    Metafictional,
    /// Narrative without stable chronology; mosaic, shattered, or collaged structure.
    Fragmented,
    /// Second-person address ('you') implicates the reader as protagonist.
    SecondPerson,
    /// Oulipo tradition: writing produced or constrained by a formal rule.
    ConstraintBased,
    /// Bakhtin: multiple independent consciousnesses; no single authorial perspective dominates.
    Polyphonic,
    /// Aarseth: text requiring non-trivial traversal effort; ergodic / interactive.
    Ergodic,
    /// Nabokov/Borges: the character becomes aware they are in a narrative.
    Metaleptic,
}

// ── Posthumanism and Nonhuman Narrative (Wolfe / Braidotti / Hayles) ──────────

/// Posthumanist narrative modes — how narrative frames nonhuman, animal, machine subjects.
/// Sources: Wolfe *Animal Rites* (2003), Braidotti *The Posthuman* (2013),
/// Hayles *How We Became Posthuman* (1999).
/// Reference: `08-revision/critical-theory/posthumanism-and-nonhuman.md`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum PosthumanMode {
    /// Wolfe: approaching animal experience through its own Umwelt — non-humanizing.
    AnimalAlterity,
    /// Braidotti: process of becoming-animal; the human boundary is crossed imaginatively.
    BecomingAnimal,
    /// Braidotti: the human-machine assemblage; cyborg consciousness.
    BecomingMachine,
    /// Braidotti: zoe (bare life) as the force that exceeds the human subject.
    ZoeCentered,
    /// Hayles: consciousness as pattern of information rather than embodied continuity.
    EmbodiedInformation,
    /// Clark: mind extended into tools, environment, others; cognition beyond the skull.
    ExtendedCognition,
    /// Distributed and collective nonhuman agency — the network, the swarm, the ecosystem.
    NonhumanAgency,
}

// ── Marxist / Class Narratology (Jameson / Goldmann / Williams / Lukács / Eagleton)

/// Marxist and class-based narrative analysis modes.
/// Sources: Jameson *The Political Unconscious* (1981), Williams *Marxism and Literature* (1977),
/// Lukács *Studies in European Realism* (1950), Eagleton *Criticism and Ideology* (1976).
/// Reference: `08-revision/critical-theory/class-and-marxist-narratology.md`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum MarxistNarrativeMode {
    /// Jameson: the ideological horizon that cannot be thought directly — only symptomatically.
    PoliticalUnconscious,
    /// Goldmann: homological relationship between narrative structure and social class's world-view.
    Homology,
    /// Williams: the dominant form of culture — what the current order reproduces.
    DominantStructure,
    /// Williams: surviving formations from a previous social order; not fully residual.
    ResidualStructure,
    /// Williams: formation not yet dominant; harbinger of a new cultural logic.
    EmergentStructure,
    /// Marx/Lukács: social relations between people misapprehended as relations between things.
    Reification,
    /// Eagleton: literary text as site of ideological production, not merely reflection.
    IdeologicalProduction,
    /// Lukács: the bourgeois novel form as historically conditioned by capital's rise.
    RealismAsBourgeoisForm,
}

impl MarxistNarrativeMode {
    /// Returns the Williams `CulturalStructure` equivalent for the three
    /// structure-of-feeling variants, or `None` for Jameson/Goldmann/Lukács modes.
    ///
    /// This formalises the overlap between the standalone `cultural_structure`
    /// annotation tag and the `marxist_mode` tag: the same Williams concept appears
    /// in both contexts and both are valid \u2014 `cultural_structure` for standalone
    /// thematic annotation, `marxist_mode` for full Marxist meta-analysis.
    pub fn as_cultural_structure(self) -> Option<CulturalStructure> {
        match self {
            MarxistNarrativeMode::DominantStructure  => Some(CulturalStructure::Dominant),
            MarxistNarrativeMode::ResidualStructure  => Some(CulturalStructure::Residual),
            MarxistNarrativeMode::EmergentStructure  => Some(CulturalStructure::Emergent),
            _ => None,
        }
    }
}

impl From<CulturalStructure> for MarxistNarrativeMode {
    /// Lifts a Williams `CulturalStructure` into the `MarxistNarrativeMode` context
    /// for cross-phase constraint validation.
    fn from(c: CulturalStructure) -> Self {
        match c {
            CulturalStructure::Dominant => MarxistNarrativeMode::DominantStructure,
            CulturalStructure::Residual => MarxistNarrativeMode::ResidualStructure,
            CulturalStructure::Emergent => MarxistNarrativeMode::EmergentStructure,
        }
    }
}

// ── Feminist Narratology (Lanser / DuPlessis / Warhol / Fetterley / Miller) ───

/// Feminist narrative categories.
/// Sources: Lanser *Fictions of Authority* (1992), DuPlessis *Writing Beyond the Ending* (1985),
/// Warhol *Gendered Interventions* (1989), Fetterley *The Resisting Reader* (1978).
/// Reference: `08-revision/critical-theory/feminist-narratology.md`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum FeministNarrativeType {
    /// Warhol: narrator who directly addresses the narratee to encourage emotional identification.
    EngagingNarrator,
    /// Warhol: narrator who creates ironic/critical distance, refusing sentimental complicity.
    DistancingNarrator,
    /// DuPlessis: the nineteenth-century convention that resolves women's stories through marriage or death.
    MarriagePlot,
    /// DuPlessis: narrative strategies that write beyond the marriage/death ending for women.
    WritingBeyondTheEnding,
    /// Fetterley: the process by which female readers are trained to identify with male values against their own interests.
    Immasculation,
    /// Miller: conventions that govern what is 'plausible' for female characters — often ideologically determined.
    PlausibilityConvention,
    /// Lanser: narrative authority as gendered — female narrators face specific legitimation problems.
    AuthorityAndGender,
    /// Women's cultural tradition as distinct resource distinct from the dominant tradition.
    WomensCulture,
}

// ── African-American Narrative Tradition (Gates / Baker / Morrison / Hurston) ─

/// Signifying and related modes from the African-American literary tradition.
/// Sources: Gates *The Signifying Monkey* (1988), Baker *Blues, Ideology and Afro-American Literature* (1984),
/// Morrison *Playing in the Dark* (1992), Hurston *Their Eyes Were Watching God* (1937).
/// Reference: `references/african-american-narrative-tradition.md`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum SignifyingMode {
    /// Gates: repetition with a difference — texts revise and signify on prior texts.
    RepetitionWithDifference,
    /// Gates: indirect, tonal signification through implication and double-voiced discourse.
    SignifyinTonal,
    /// Gates: direct signification — explicit revision of a prior text or tradition.
    SignifyinDirect,
    /// Gates: metaphoric indirection; the signifier slides under the surface of the signified.
    MetaphoricIndirection,
    /// Imitation of another's style, but with parodic or critical intent.
    Pastiche,
    /// Hurston: vernacular voice as the primary medium; the language of community speaks through the text.
    SpeakerlyText,
    /// Baker: the blues matrix — music as cultural epistemology; narrative shaped by blues formal logic.
    BluesAesthetic,
    /// Morrison: the presence of Africanist imagery and shadow in 'white' American literary texts.
    AfricanistPresence,
}

// ── Cognitive Narratology (Fludernik / Palmer / Hogan / Nünning) ──────────────

/// Cognitive narrative modes — how minds, comprehension, and emotion operate in narrative.
/// Sources: Fludernik *Towards a Natural Narratology* (1996), Palmer *Fictional Minds* (2004),
/// Hogan *The Mind and Its Stories* (2003), Nünning *Unreliable Narration* (1999).
/// Reference: `references/cognitive-narratology-reference.md`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum CognitiveNarrativeMode {
    /// Fludernik: narrative as the representation of conscious human experience; experientiality as the core criterion.
    Experientiality,
    /// Fludernik: texts conforming to natural (oral, conversational) narrative schemas.
    NaturalNarrative,
    /// Palmer: the continuing, developing consciousness of fictional characters as primary narrative engine.
    ContinuingConsciousness,
    /// Palmer: the shared, group, collective minds of characters — social mind in action.
    IntermentalThought,
    /// Hogan: universal emotional story structures (heroic, romantic, sacrificial) driven by affect.
    AffectiveScript,
    /// Nünning: unreliability detected through frame comparison between narrator and implied-author frames.
    FrameBasedUnreliability,
}

// ── Prosody and Verse Form ────────────────────────────────────────────────────

/// Prosodic and verse form elements — for verse novel and prose-poetry work.
/// Sources: Fussell *Poetic Meter and Poetic Form* (1965), Hartman *Free Verse* (1980),
/// Attridge *The Rhythms of English Poetry* (1982).
/// Reference: `07-drafting/theory-reference/verse-novel-and-prose-poetry.md`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum ProsodicElement {
    /// Recurring pattern of stressed and unstressed syllables.
    Meter,
    /// Repetition of terminal sounds; masculine, feminine, slant, eye.
    Rhyme,
    /// The point where a line of verse ends — a unit of meaning independent of syntax.
    LineBreak,
    /// Continuation of a syntactic unit across a line boundary; tension between syntax and line.
    Enjambment,
    /// A pause within a line, often marked by punctuation; breath and rhythm.
    Caesura,
    /// Visual arrangement of space on the page as a meaning-bearing element.
    WhiteSpace,
    /// Repetition of initial consonant sounds.
    Alliteration,
    /// Repetition of vowel sounds within nearby words.
    Assonance,
    /// A grouping of lines forming a unit; the stanza as formal container.
    StanzaForm,
    /// The overall rhythmic movement without fixed meter; breath, breath-group.
    FreeVerseRhythm,
}

// ── Propp: morphological functions ────────────────────────────────────────────

/// Propp *Morphology of the Folktale* (1928/1968) — the 31 narrative functions
/// condensed into a type system for structural annotation.  Not every story uses
/// all 31; the sequence is invariable when functions are present.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum ProppFunction {
    /// I — A member of the family absents themselves from home.
    Absentation,
    /// II — A prohibition or rule is established.
    Interdiction,
    /// III — The prohibition is violated.
    Violation,
    /// IV — Villain reconnoitres the hero's family.
    Reconnaissance,
    /// V — Villain receives information about the hero.
    Delivery,
    /// VI — Villain attempts to deceive the hero.
    Trickery,
    /// VII — Hero is deceived; unwitting complicity.
    Complicity,
    /// VIII — Villain causes harm or injury; the lack is established.
    VillainyCausedLack,
    /// IX — Misfortune is made known; hero is dispatched.
    Mediation,
    /// X — Hero agrees to counter the villainy or seeks to remedy the lack.
    CounterAction,
    /// XI — Hero leaves home.
    Departure,
    /// XII — Hero is tested by a donor figure.
    TestingDonor,
    /// XIII — Hero reacts to the test (often correctly).
    ReactionToTest,
    /// XIV — Hero acquires a magical agent or helper.
    AcquisitionOfAgent,
    /// XV — Hero is led to the object of the search.
    Guidance,
    /// XVI — Hero and villain join in direct struggle.
    Struggle,
    /// XVII — Hero is branded or marked.
    Branding,
    /// XVIII — Villain is defeated.
    VillainDefeated,
    /// XIX — Initial misfortune or lack is resolved.
    Liquidation,
    /// XX — Hero returns home.
    ReturnHome,
    /// XXI — Hero is pursued by villain or antagonist force.
    Pursuit,
    /// XXII — Hero is rescued from pursuit.
    Rescue,
    /// XXIII — Hero returns home unrecognized.
    UnrecognizedArrival,
    /// XXIV — False hero presents unfounded claims.
    UnfoundedClaims,
    /// XXV — A difficult task is proposed to the hero.
    DifficultTask,
    /// XXVI — The task is accomplished.
    TaskAccomplished,
    /// XXVII — Hero is recognized (branding, object).
    Recognition,
    /// XXVIII — False hero or villain is exposed.
    Exposure,
    /// XXIX — Hero is given a new appearance or transfigured.
    Transfiguration,
    /// XXX — Villain is punished.
    Punishment,
    /// XXXI — Hero is married and/or ascends the throne; reward.
    Wedding,
}

// ── Butler: performativity modes ──────────────────────────────────────────────

/// Butler *Gender Trouble* (1990), *Bodies That Matter* (1993).
/// Muñoz *Disidentifications* (1999).
/// Covers the mechanism by which gender and identity are
/// produced through repeated citation of norms — and subverted.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum PerformativityMode {
    /// Performativity as the sedimented citational chain that produces the effect of identity.
    Citationality,
    /// Parodic over-citation that exposes the constructedness of gender norms.
    DragPerformance,
    /// The performative concealment of identity to pass within a normative category.
    Passing,
    /// Performance achieves desired self-categorization; the norm is successfully inhabited.
    FelicitousPerformative,
    /// Failed performance; a gap between the citation and its recognition.
    Misfire,
    /// Repetition with a subversive difference — the norm is cited but displaces it.
    SubversiveRepetition,
    /// The re-inscription of a norm through repeated uncritical performance.
    HeteronormCitation,
    /// Muñoz: working with/against/through dominant culture rather than rejection or assimilation.
    Disidentification,
}

// ── Autofiction self-presentation modes ───────────────────────────────────────

/// Lejeune *On Autobiography* (1975); Doubrovsky (autofiction, 1977);
/// Ernaux (collective impersonal, 1974–); Knausgård (disclosure ethics, 2009–);
/// Barthes *Roland Barthes par Roland Barthes* (1975).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum AutofictionMode {
    /// Lejeune: author = narrator = character; identity declared in pact with reader.
    AutobiographicalPact,
    /// Doubrovsky: fictionalized autobiography; the 'I' is simultaneously real and literary construct.
    AutofictionProper,
    /// Ernaux: impersonal first person — 'I' as representative of a collective, not unique self.
    CollectiveImpersonal,
    /// Barthes: self rendered as textual system, indexed fragments, dispersal of authorial unity.
    SelfAsText,
    /// Lyric-essay hybrid: 'I' organized by meditation, digression, association over plot.
    LyricEssayMode,
    /// Knausgård line: total disclosure, no transformation of shame/intimacy into aesthetic distance.
    DisclosureEthicMode,
    /// Productive ambiguity about which contract governs — is this autobiography or fiction?
    PactAmbiguity,
}

// ── Narrative ethics modes ─────────────────────────────────────────────────────

/// Booth *The Company We Keep* (1988); Keen *Empathy and the Novel* (2007);
/// Nussbaum *Love's Knowledge* (1990); Spivak (representation ethics, 1988).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum NarrativeEthicsMode {
    /// Booth: the cumulative experience of the implied author as moral agent; long effect of the text.
    Coduction,
    /// Keen: author-reader empathic exchange — author designs readerly identification.
    AuthorEmpathyDesign,
    /// Keen: empathy attached to individual character rather than author strategy.
    CharacterEmpathy,
    /// Keen: broadcast empathy — universal appeal to human sameness.
    BroadcastEmpathy,
    /// Keen: bounded empathy — empathy restricted by in-group/out-group recognition.
    BoundedEmpathy,
    /// Keen: strategic empathy — deliberate authorial deployment to political end.
    StrategicEmpathy,
    /// Nussbaum: fiction as moral imagination, a rehearsal of ethical perception.
    MoralLaboratory,
    /// Text aestheticizes harm — narrative form implicates reader in what it depicts.
    Complicity,
    /// Text uses narrative to critique the autonomy of values it otherwise endorses.
    CritiqueOfAutonomy,
}

// ── Jenkins: transmedia principles ────────────────────────────────────────────

/// Jenkins *Convergence Culture* (2006), *Transmedia Storytelling* (2009 online essay).
/// The seven core principles of transmedia storytelling plus related concepts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum JenkinsTransmediaType {
    /// Each medium makes its own unique contribution; the whole is irreducible to parts.
    Spreadability,
    /// Depth: allows fans to drill into the world beyond the surface story.
    Drillability,
    /// Sequential cause-and-effect canon preserved across media.
    Continuity,
    /// Non-canonical alternative versions; 'what if?' explorations.
    Multiplicity,
    /// The experience of being transported into the story world — immersive design.
    Immersion,
    /// Allowing story elements to migrate into daily life as real objects or practices.
    Extractability,
    /// Expansion of the world's geography, history, and population across texts.
    WorldBuilding,
    /// The story continues across instalments; the expanded seriality principle.
    Seriality,
    /// Different characters or perspectives on the same events across media.
    Subjectivity,
    /// Audience participation in the construction of the story world.
    Performance,
}

// ── Genre reading contract (Derrida / Culler / Rosenblatt / Fish) ────────────
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum GenreReadingMode {
    /// Derrida: every text participates in genres without belonging to any single genre.
    LawOfGenre,
    /// Derrida: texts carry the traces of genres they don't fully inhabit.
    GenreContamination,
    /// Culler: the learned set of conventions a reader brings to a text as literature.
    LiteraryCompetence,
    /// Culler: the process by which readers fit puzzling texts into recognizable frames.
    Naturalization,
    /// Rosenblatt: stance focused on the lived-through experience of the text.
    AestheticReading,
    /// Rosenblatt: stance focused on extracting information to be carried away.
    EfferentReading,
    /// Fish: meanings produced by communities of readers sharing interpretive conventions.
    InterpretiveCommunity,
}

// ── Genre ─────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum Genre {
    Romance,
    Horror,
    SciFi,
    HistoricalFiction,
    Fantasy,
    Mystery,
    /// Alias "literary" is used in older YAML catalog entries.
    #[serde(alias = "literary")]
    LiteraryFiction,
    Thriller,
    YoungAdult,
    GraphicNarrative,
    VerseNovel,
}

// ════════════════════════════════════════════════════════════════════════════
// V3.0.2 TRAINING SCHEMA ENUMS - Centralized vocabularies for scene annotation
// ════════════════════════════════════════════════════════════════════════════

// ── Character Perception & Relationship ──────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum PerceptionOfOther {
    ExpectsApproval,
    ExpectsRejection,
    ExpectsHostility,
    ExpectsIndifference,
    ExpectsSupport,
    ExpectsBetrayal,
    ExpectsHonesty,
    ExpectsDeception,
    ExpectsGratitude,
    ExpectsBlame,
    Uncertain,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum TriggerType {
    PastInjuryMentioned,
    Dismissal,
    Condescension,
    Accusation,
    Comparison,
    NameInvoked,
    PhysicalProximity,
    ToneOfVoice,
    SpecificWord,
    TriggerNone,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum RelationshipTurn {
    TurnNone,
    Deterioration,
    Improvement,
    Revelation,
    Rupture,
    Reconciliation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum EmotionalArc {
    Change,
    Stasis,
    Escalation,
    DeEscalation,
    Realization,
    Reversal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum PowerBalance {
    Equal,
    ThisCharHasPower,
    OtherCharHasPower,
    Contested,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum RelationshipDistanceLevel {
    Intimate,
    Personal,
    Social,
    Public,
    Distant,
    Hostile,
}

// ── Character Objective ──────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum ObjectiveVerb {
    Obtain,
    Persuade,
    Escape,
    Protect,
    Confront,
    Conceal,
    Confess,
    Decide,
    Impress,
    Survive,
    Connect,
    Separate,
    Refuse,
    Accept,
    Reveal,
    Learn,
    Prevent,
    Maintain,
    ObjectiveNone,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum ObjectiveObject {
    Information,
    Secret,
    Relationship,
    Proposal,
    Accusation,
    Position,
    Resource,
    Safety,
    Dignity,
    Truth,
    Alliance,
    Freedom,
    Approval,
    Control,
    ReconciliationObj,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum ObjectiveConstraint {
    Openly,
    Covertly,
    MaintainingDignity,
    WithoutConflict,
    Permanently,
    Temporarily,
    Conditionally,
    ConstraintNone,
}

// ── Character Physical State ─────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum BodyLanguage {
    EyeContact,
    AvoidsEyeContact,
    ClenchedFists,
    CrossedArms,
    OpenPosture,
    TurnedAway,
    LeaningIn,
    LeaningBack,
    Fidgeting,
    Stillness,
    Trembling,
    Tears,
    ForcedSmile,
    GenuineSmile,
    Frown,
    BlankExpression,
    Nodding,
    ShakingHead,
    TouchingFace,
    TouchingOther,
    MaintainingDistance,
    ClosingDistance,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum Posture {
    Seated,
    Standing,
    Pacing,
    Lying,
    Kneeling,
    Crouching,
    Leaning,
    Frozen,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum PovRole {
    Focal,
    Present,
    Mentioned,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum PsychicDistanceLevel {
    DistantNarrator,
    CloseNarrator,
    NeutralFilter,
    DeepFilter,
    StreamOfConsciousness,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum PsychicDistanceTrigger {
    DialogueStart,
    RevelationMoment,
    DecisionPoint,
    EmotionalPeak,
    ActionSequence,
    Reflection,
    SceneEnd,
}

// ── Stakes ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum PersonalStakes {
    Identity,
    SelfRespect,
    SafetyStake,
    FreedomStake,
    Livelihood,
    Reputation,
    RelationshipStake,
    Goal,
    SecretStake,
    Belief,
    StakesNone,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum RelationalStakes {
    TrustStake,
    IntimacyStake,
    StatusStake,
    ConnectionStake,
    AllianceStake,
    RuptureStake,
    RelationStakesNone,
}

// ── Social Circles ───────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum SocialCircle {
    Family,
    Professional,
    Community,
    Underground,
    Romantic,
    Religious,
    Political,
    Artistic,
    CircleNone,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum SocialMask {
    Authentic,
    Performing,
    Concealing,
    CodeSwitching,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum DominantSocialCircle {
    FamilyDom,
    ProfessionalDom,
    SocialDom,
    IntimateDom,
    PublicDom,
    PrivateDom,
    InstitutionalDom,
    UndergroundDom,
    MarginalDom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum CircleIntersection {
    IntersectionNone,
    FamilyProfessional,
    FamilySocial,
    ProfessionalSocial,
    IntimateFamily,
    PublicPrivate,
    MultiCircle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum TerritoryType {
    HomeGround,
    SharedSpace,
    RivalTerritory,
    NeutralGround,
    ContestedTerritory,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum CharacterTerritory {
    Home,
    Foreign,
    Neutral,
    ContestedChar,
}

// ── Continuity ───────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum TemporalGapType {
    GapNone,
    BriefEllipsis,
    ExtendedEllipsis,
    ChapterBreak,
    SectionBreak,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum LocationChangeType {
    Same,
    Adjacent,
    Nearby,
    DistantLoc,
    VeryDistant,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum CharacterArrival {
    StartsPresent,
    EntersEarly,
    EntersMid,
    EntersLate,
    NotPresent,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum CharacterDeparture {
    StaysThroughout,
    ExitsEarly,
    ExitsMid,
    ExitsLate,
    NotPresentDep,
}

// ── Reader Experience (Sternberg) ────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum InformationGapType {
    Temporary,
    Permanent,
    Ambiguous,
    FalseGap,
    InfoGapNone,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum InformationAsymmetry {
    ReaderKnowsMore,
    ReaderKnowsLess,
    EqualKnowledge,
    ReaderSuspects,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum GapDomain {
    Plot,
    CharacterMotive,
    Backstory,
    Future,
    Reliability,
    DomainNone,
}

// ── Threads & Callbacks ──────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum CallbackThreadType {
    PlotCallback,
    DialogueCallback,
    ImageCallback,
    MotifCallback,
    CharacterTraitCallback,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum CallbackEffect {
    Echo,
    Contrast,
    EscalationEffect,
    Payoff,
    Reframe,
}

// ── Motif ────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum MotifType {
    Visual,
    Auditory,
    Tactile,
    Linguistic,
    Behavioral,
    Spatial,
    Temporal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum ObjectiveCorrelativeType {
    NaturalPhenomenon,
    PhysicalObject,
    SettingDetail,
    Action,
    Sound,
    WeatherCorr,
    CorrelativeNone,
}

// ── Thematic ─────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum ControllingIdeaPosition {
    Supports,
    Challenges,
    Complicates,
    Tests,
    PositionNeutral,
}

// ── Setting ──────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum Season {
    Spring,
    Summer,
    Autumn,
    Winter,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum SpatialStructure {
    Enclosed,
    Threshold,
    Open,
    Liminal,
    Vertical,
    Horizontal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum PropFunction {
    SetDressing,
    ActionObject,
    Symbolic,
    ChekhovGun,
    Continuity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum SensoryProminence {
    Background,
    Mentioned,
    Emphasized,
    Dominant,
}

// ── Narrative Time ───────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum AnalepsisType {
    External,
    Internal,
    Complete,
    Partial,
    AnalepsisNone,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum ProlepisType {
    Announcing,
    Completing,
    ExternalProl,
    InternalProl,
    ProlepisNone,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum IterativePattern {
    Daily,
    Weekly,
    Seasonal,
    Habitual,
    PatternNone,
}

// ── Subtext ──────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum SubtextTechnique {
    GriceanViolation,
    Deflection,
    LoadedSilence,
    Iceberg,
    PinterPause,
    ApparentIrrelevance,
    TechniqueNone,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum IcebergCategory {
    PastTrauma,
    SecretLove,
    HiddenResentment,
    Guilt,
    FearIceberg,
    Desire,
    FamilySecret,
    Betrayal,
    Loss,
    Ambition,
    IcebergNone,
}

// ── Beat & Sequence (Snyder) ─────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum BeatType {
    Hook,
    Setup,
    Catalyst,
    Debate,
    Threshold,
    Tests,
    Approach,
    Ordeal,
    Reward,
    RoadBack,
    Resurrection,
    Return,
    Crisis,
    Climax,
    Resolution,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum SequenceType {
    SetupSeq,
    CatalystSeq,
    DebateSeq,
    BreakIntoTwo,
    FunAndGames,
    Midpoint,
    BadGuysCloseIn,
    AllIsLost,
    DarkNight,
    BreakIntoThree,
    Finale,
    FinalImage,
}

// ── Knowledge ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum KnowledgePredicate {
    Loves,
    Hates,
    Married,
    Engaged,
    Separated,
    Divorced,
    RelatedTo,
    Betrayed,
    Helped,
    Harmed,
    Saved,
    Abandoned,
    Rejected,
    Forgave,
    LiedTo,
    Deceived,
    Manipulated,
    ToldTruth,
    IsWealthy,
    IsPoor,
    IsPowerful,
    IsVulnerable,
    IsHiding,
    IsPresent,
    KnowsSecret,
    HasDebt,
    OwesFavor,
    BrokePromise,
    KeptPromise,
    Caused,
    Prevented,
    Witnessed,
    Planned,
    Intends,
    Owns,
    Wants,
    Fears,
}

// ── Prose-Level Features (v3.0.3) ────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum GrammaticalSentenceType {
    Declarative,
    Interrogative,
    Imperative,
    Exclamatory,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum SentenceStructure {
    Fragment,
    Simple,
    Compound,
    Complex,
    CompoundComplex,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum SentenceLengthCategory {
    Fragment,
    Short,
    Medium,
    Long,
    Marathon,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum ProseRhythm {
    Staccato,
    Flowing,
    Varied,
    Periodic,
    Telegraphic,
    Cumulative,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum DictionLevel {
    Monosyllabic,
    Plain,
    Conversational,
    Formal,
    Literary,
    Ornate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum Concreteness {
    Abstract,
    General,
    Concrete,
    Sensory,
    Visceral,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum FigureType {
    Metaphor,
    Simile,
    Personification,
    Metonymy,
    Synecdoche,
    Hyperbole,
    Litotes,
    Oxymoron,
    Antithesis,
    Zeugma,
    Chiasmus,
    Anaphora,
    Anadiplosis,
    Epistrophe,
    Asyndeton,
    Polysyndeton,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum DefamiliarizationLevel {
    Automatized,
    Mild,
    Moderate,
    Strong,
    Radical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum SoundDevice {
    Alliteration,
    Assonance,
    Consonance,
    Onomatopoeia,
    Euphony,
    Cacophony,
    Sibilance,
    Plosives,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum ProcessType {
    Material,
    Mental,
    Verbal,
    Relational,
    Behavioral,
    Existential,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum AgentEncoding {
    ClearAgent,
    ObscuredAgent,
    AgentlessPassive,
    Nominalization,
    Ambient,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum AttributionStyle {
    Said,
    SaidVariant,
    ActionBeat,
    None,
    Mixed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum DiscourseMode {
    DirectSpeech,
    IndirectSpeech,
    FreeIndirectDiscourse,
    NarratedSpeech,
    SpeechSummary,
    ThoughtReport,
    DirectThought,
    FreeIndirectThought,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum ForegroundingType {
    ExternalDeviation,
    InternalDeviation,
    Parallelism,
    CohesionPattern,
    SemanticSaturation,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum ProsePacing {
    Compressed,
    Standard,
    Expanded,
    Dilated,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum VocabularySource {
    CharacterOwned,
    NarratorOwned,
    PeriodAppropriate,
    Mixed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum ClausePosition {
    MainInitial,
    MainMedial,
    MainFinal,
    Embedded,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum ParagraphFunction {
    SceneAction,
    DialogueExchange,
    Interiority,
    Description,
    Exposition,
    Transition,
    Summary,
    Reflection,
}

// ── Story Architecture (v3.0.4) ──────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum GenreType {
    Romance,
    Literary,
    Thriller,
    Mystery,
    Fantasy,
    ScienceFiction,
    Horror,
    Historical,
    Contemporary,
    YoungAdult,
    MiddleGrade,
    Memoir,
    Western,
    Gothic,
    Comedy,
    Tragedy,
    Satire,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum CollisionPattern {
    RivalCirclesEqual,
    ClassCrossing,
    InsiderOutsider,
    ForbiddenProximity,
    ForcedProximity,
    SameCircleDisruption,
    InstitutionalHierarchy,
    PerformanceForAudience,
    InstitutionalMerger,
    ReturningInsider,
    TemperamentCollision,
    TriangulatedCircles,
    HiddenWorld,
    CompetitiveArena,
    GenerationalDivide,
    InvestigatorEntersClosedWorld,
    SealedCommunity,
    LoneFigureVsInstitution,
    UnderworldOverworld,
    CatAndMouseCircles,
    Infiltration,
    CivilianEntersInstitutional,
    WhistleblowerVsInstitution,
    FugitiveAndPursuers,
    HostageBoundary,
    PsychologicalBoundary,
    FellowshipVsDarkPower,
    WorldCrossingLiteral,
    CommonerEntersPowerStructure,
    CourtFactions,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum CollisionType {
    WorldCrossing,
    PowerAsymmetry,
    ResourceCompetition,
    LoyaltyConflict,
    ValueClash,
    IdentityThreat,
    ForcedAlliance,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum IncitingIncidentType {
    StrangerArrives,
    ProtagonistDisplaced,
    ReturnOfThePast,
    ForcedDeparture,
    AssignedTogether,
    ArrangementProposed,
    StrandedOrTrapped,
    DiscoveryRevelation,
    InheritanceBequest,
    ChallengeIssued,
    CrimeWitnessed,
    WrongAccused,
    LossOrDeath,
    BetrayalDiscovered,
    SecretExposed,
    OpportunityOffered,
    ThresholdCrossed,
    CallRefusedThenForced,
}

// ── Character Architecture (v3.0.4) ──────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum AntagonistType {
    Character,
    Institutional,
    Natural,
    Internal,
    Societal,
    Cosmic,
    Layered,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum AntagonistArcType {
    FlatEvil,
    TragicDescent,
    Revelation,
    Redemption,
    Parallel,
    Escalation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum OppositionLevel {
    Physical,
    Intellectual,
    Emotional,
    Moral,
    Thematic,
}

// ── World-Building Theory (v3.0.4) ───────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum BoundaryType {
    Physical,
    Social,
    Institutional,
    Psychological,
    Temporal,
    Ontological,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum SemanticZone {
    Safety,
    Danger,
    Past,
    Future,
    Known,
    Unknown,
    Power,
    Powerlessness,
    Freedom,
    Constraint,
    Sacred,
    Profane,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum ChronotopeType {
    Road,
    ThresholdChronotope,
    Salon,
    Castle,
    ProvincialTown,
    Idyll,
    AdventureTime,
    EverydayTime,
    CrisisTime,
    BiographicalTime,
}

// ── Speech Act Theory (v3.0.4) ───────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum SpeechActCategory {
    Assertive,
    Directive,
    Commissive,
    Expressive,
    Declaration,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum IllocutionaryForce {
    Assert,
    Claim,
    Report,
    Describe,
    Insist,
    Deny,
    Order,
    Command,
    Request,
    Ask,
    Beg,
    Plead,
    Invite,
    Suggest,
    Promise,
    Vow,
    Pledge,
    Threaten,
    Offer,
    Guarantee,
    Refuse,
    Thank,
    Apologize,
    Congratulate,
    Condole,
    Welcome,
    Deplore,
    Declare,
    Pronounce,
    Name,
    Appoint,
    Fire,
    Adjourn,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum ArcDirection {
    Positive,
    Negative,
    Flat,
    Disillusionment,
    FallThenRise,
    RiseThenFall,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum PowerAsymmetryType {
    Equal,
    DominantA,
    DominantB,
    Contested,
    Shifting,
}

// ============================================================================
// v3.0.6: Narrator Reliability, Trauma, Metafiction, Knowledge Gaps
// ============================================================================

/// Booth/Nünning narrator reliability types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum NarratorReliabilityType {
    Reliable,
    FactuallyUnreliable,
    InterpretivelyUnreliable,
    EvaluativelyUnreliable,
    SelfDeceptive,
    CulturallyContingent,
}

/// Sternberg: what creates the information gap
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum KnowledgeGapSource {
    WithheldPast,
    UncertainFuture,
    SuppressedByFocalizer,
    UnreliableSource,
    LimitedPerception,
    DeliberateOmission,
}

/// Caruth/Herman/LaCapra trauma representation modes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum TraumaRepresentationMode {
    Integrated,
    Fragmented,
    Belated,
    ActingOut,
    WorkingThrough,
    Dissociated,
    Somatized,
}

/// Van der Kolk embodied trauma responses
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum SomaticTraumaResponse {
    Startle,
    Freeze,
    Dissociation,
    Hypervigilance,
    Numbness,
    FlashbackBody,
    Trembling,
    Nausea,
    ConstrictedBreathing,
    None,
}

/// Waugh/McHale metafictional techniques
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum MetafictionalTechnique {
    DirectAddress,
    AuthorialIntrusion,
    MiseEnAbyme,
    FrameBreaking,
    SelfReference,
    ExposedDevice,
    AlternativeEndings,
    ReaderAsCharacter,
    None,
}

/// Max Black interaction theory: metaphor quality levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum MetaphorQuality {
    Dead,
    Conventional,
    Active,
    Resonant,
    Mixed,
}

/// Truby want vs. need alignment
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum WantNeedAlignment {
    Aligned,
    Opposed,
    UnknownNeed,
    FalseWant,
    Evolving,
}

// v3.0.7: Actantial, Transtextuality (IronyType, GazeType, ComicMode already exist above)

/// Greimas actantial model roles
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum ActantialRole {
    Subject,
    Object,
    Sender,
    Receiver,
    Helper,
    Opponent,
}

/// Genette transtextuality types (v3.0.7 - complements IntertextualRelation with None variant)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Display, EnumString)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum TranstextualityType {
    Intertextuality,
    Paratextuality,
    Metatextuality,
    Hypertextuality,
    Architextuality,
    None,
}
