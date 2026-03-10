//! Unified ontology contract for Grimoire.
//!
//! This module defines the abstract data structure that all Grimoire artifacts
//! describe: entities, controlled vocabularies, annotation facts, constraints,
//! and their projections (Markdown/YAML/JSON Schema/Rust/Python).
//!
//! The goal is to provide a stable, Rust-first conceptual spine that can be
//! referenced by loaders, validators, adapters, and migration tooling.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Top-level bounded contexts in the Grimoire ontology.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum OntologyLayer {
    /// Readiness, gates, phase/sub-phase progression, dependency DAG.
    Governance,
    /// Story-domain entities and relationships (characters, scenes, beats, etc).
    NarrativeDomain,
    /// Theory and narratology concept layer (controlled semantics).
    InterpretiveSemantics,
    /// Tag-based, scoped declarations extracted from annotated text.
    AnnotationEvent,
    /// Canonical catalog entries loaded from YAML references.
    CatalogKnowledge,
    /// Project and character style constraints (voice contract).
    StyleVoice,
    /// Generated recipe/training artifacts for model-facing workflows.
    GenerationTraining,
    /// Schema/export/projection surface derived from canonical types.
    InterchangeSchema,
}

/// Core primitive category represented in the unified ontology.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum PrimitiveKind {
    Entity,
    Reference,
    VocabularyTerm,
    AnnotationFact,
    Constraint,
    Projection,
}

/// Supported representation surfaces for a canonical primitive.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ProjectionType {
    RustType,
    MarkdownTag,
    YamlCatalog,
    JsonSchema,
    PythonAdapter,
    CliOutput,
}

/// Deprecation level for a legacy tag key alias.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum DeprecationLevel {
    /// Preferred spelling and key.
    None,
    /// Legacy alias still accepted and silently canonicalized.
    Soft,
    /// Legacy alias accepted but should be migrated as soon as practical.
    Hard,
}

/// A canonical ontology primitive and where it lives.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct OntologyPrimitive {
    /// Stable identifier for this primitive, e.g. `entity.character`.
    pub id: String,
    /// Human-readable title.
    pub label: String,
    /// Ontology layer this primitive belongs to.
    pub layer: OntologyLayer,
    /// Primitive category.
    pub kind: PrimitiveKind,
    /// Canonical Rust symbol path, e.g. `entities::Character`.
    pub canonical_symbol: String,
    /// Optional behavioral constraints/invariants.
    pub invariants: Vec<String>,
    /// Known projections of the same concept.
    pub projections: Vec<PrimitiveProjection>,
}

/// Mapping from canonical primitive to one concrete projection surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct PrimitiveProjection {
    pub projection_type: ProjectionType,
    /// Symbol/path/key in the projection surface.
    pub target: String,
    /// Optional migration alias list for backwards compatibility.
    pub aliases: Vec<String>,
}

/// Canonical mapping entry for one annotation tag key.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct TagKeyMapping {
    /// Canonical key consumed by typed parsing.
    pub canonical_key: String,
    /// Legacy key alias, if any.
    pub alias_key: Option<String>,
    /// Which ontology layer this key primarily belongs to.
    pub layer: OntologyLayer,
    /// Which primitive kind this key contributes.
    pub kind: PrimitiveKind,
    /// Deprecation level for alias usage.
    pub deprecation: DeprecationLevel,
    /// Whether this key is expected to have a heading entry in the public
    /// gate-tag vocabulary document.
    pub expected_in_docs_heading: bool,
}

impl TagKeyMapping {
    pub fn canonical_only(
        canonical_key: &str,
        layer: OntologyLayer,
        kind: PrimitiveKind,
    ) -> Self {
        Self {
            canonical_key: canonical_key.to_owned(),
            alias_key: None,
            layer,
            kind,
            deprecation: DeprecationLevel::None,
            expected_in_docs_heading: true,
        }
    }

    pub fn alias(
        canonical_key: &str,
        alias_key: &str,
        layer: OntologyLayer,
        kind: PrimitiveKind,
        deprecation: DeprecationLevel,
    ) -> Self {
        Self {
            canonical_key: canonical_key.to_owned(),
            alias_key: Some(alias_key.to_owned()),
            layer,
            kind,
            deprecation,
            expected_in_docs_heading: false,
        }
    }
}

/// Full registry of canonical ontology primitives.
///
/// Covers all 11 entity types, the complete set of craft-instruction tag keys,
/// key interpretive-semantics vocabulary domains, and the training/generation
/// layer types.  Helper functions keep the list concise.
pub fn core_primitives() -> Vec<OntologyPrimitive> {
    // ── Builder helpers ───────────────────────────────────────────────────────

    fn entity(
        id: &str,
        label: &str,
        rust_path: &str,
        tag_key: &str,
        invariants: Vec<&str>,
    ) -> OntologyPrimitive {
        OntologyPrimitive {
            id: id.to_owned(),
            label: label.to_owned(),
            layer: OntologyLayer::NarrativeDomain,
            kind: PrimitiveKind::Entity,
            canonical_symbol: rust_path.to_owned(),
            invariants: invariants.into_iter().map(str::to_owned).collect(),
            projections: vec![
                PrimitiveProjection {
                    projection_type: ProjectionType::RustType,
                    target: format!("grimoire-types/src/entities.rs::{rust_path}"),
                    aliases: vec![],
                },
                PrimitiveProjection {
                    projection_type: ProjectionType::MarkdownTag,
                    target: format!("{tag_key}:<slug>"),
                    aliases: vec![],
                },
                PrimitiveProjection {
                    projection_type: ProjectionType::JsonSchema,
                    target: format!("entities.{rust_path}"),
                    aliases: vec![],
                },
            ],
        }
    }

    fn craft_tag(
        id: &str,
        label: &str,
        rust_path: &str,
        tag_key: &str,
        invariants: Vec<&str>,
    ) -> OntologyPrimitive {
        OntologyPrimitive {
            id: id.to_owned(),
            label: label.to_owned(),
            layer: OntologyLayer::StyleVoice,
            kind: PrimitiveKind::Constraint,
            canonical_symbol: rust_path.to_owned(),
            invariants: invariants.into_iter().map(str::to_owned).collect(),
            projections: vec![
                PrimitiveProjection {
                    projection_type: ProjectionType::RustType,
                    target: rust_path.to_owned(),
                    aliases: vec![],
                },
                PrimitiveProjection {
                    projection_type: ProjectionType::MarkdownTag,
                    target: format!("{tag_key}:<value>"),
                    aliases: vec![],
                },
            ],
        }
    }

    fn vocab_term(
        id: &str,
        label: &str,
        rust_path: &str,
        tag_key: &str,
        yaml_glob: &str,
    ) -> OntologyPrimitive {
        OntologyPrimitive {
            id: id.to_owned(),
            label: label.to_owned(),
            layer: OntologyLayer::InterpretiveSemantics,
            kind: PrimitiveKind::VocabularyTerm,
            canonical_symbol: rust_path.to_owned(),
            invariants: vec![
                "term is a controlled enum variant or catalog slug".to_owned(),
                "schema/catalog/tag representations must not drift".to_owned(),
            ],
            projections: vec![
                PrimitiveProjection {
                    projection_type: ProjectionType::RustType,
                    target: rust_path.to_owned(),
                    aliases: vec![],
                },
                PrimitiveProjection {
                    projection_type: ProjectionType::MarkdownTag,
                    target: format!("{tag_key}:<value>"),
                    aliases: vec![],
                },
                PrimitiveProjection {
                    projection_type: ProjectionType::YamlCatalog,
                    target: yaml_glob.to_owned(),
                    aliases: vec![],
                },
            ],
        }
    }

    fn generation_type(id: &str, label: &str, rust_path: &str) -> OntologyPrimitive {
        OntologyPrimitive {
            id: id.to_owned(),
            label: label.to_owned(),
            layer: OntologyLayer::GenerationTraining,
            kind: PrimitiveKind::Projection,
            canonical_symbol: rust_path.to_owned(),
            invariants: vec![],
            projections: vec![PrimitiveProjection {
                projection_type: ProjectionType::RustType,
                target: rust_path.to_owned(),
                aliases: vec![],
            }],
        }
    }

    // ── Entities (NarrativeDomain) ────────────────────────────────────────────

    let mut primitives = vec![
        entity(
            "entity.character", "Character entity",
            "Character", "character",
            vec!["id is a stable snake_case slug", "refs resolve through EntityRegistry"],
        ),
        entity(
            "entity.setting", "Setting / location entity",
            "Setting", "setting",
            vec!["id is a stable snake_case slug", "sensory_signature is three elements"],
        ),
        entity(
            "entity.beat", "Structural beat",
            "Beat", "beat",
            vec!["order is unique within the story", "story_position is 0.0–1.0"],
        ),
        entity(
            "entity.scene", "Scene card",
            "Scene", "scene",
            vec!["id is unique across all chapters", "tension_level is 1–10 when set"],
        ),
        entity(
            "entity.scene_sequence", "Scene sequence",
            "SceneSequence", "scene_sequence",
            vec!["scenes list is ordered by narrative position"],
        ),
        entity(
            "entity.chapter", "Chapter draft",
            "Chapter", "chapter",
            vec!["number is unique and monotonically increasing"],
        ),
        entity(
            "entity.motif", "Recurring motif",
            "Motif", "motif",
            vec!["first_appearance must precede subsequent_appearances"],
        ),
        entity(
            "entity.symbol", "Symbolic object / image",
            "Symbol", "symbol",
            vec!["object_or_image is a concrete noun phrase"],
        ),
        entity(
            "entity.leitmotif", "Character leitmotif",
            "Leitmotif", "leitmotif",
            vec!["character ref must resolve in EntityRegistry"],
        ),
        entity(
            "entity.thread", "Narrative thread / subplot",
            "Thread", "thread",
            vec!["characters list contains at least one ref"],
        ),
        entity(
            "entity.promise", "Narrative promise (hermeneutic code)",
            "Promise", "promise",
            vec!["planted_in and paid_off_in are EntityRefs to Scene or Chapter"],
        ),
    ];

    // ── Infrastructure (AnnotationEvent) ─────────────────────────────────────

    primitives.push(OntologyPrimitive {
        id: "reference.entity_ref".to_owned(),
        label: "Entity reference (slug)".to_owned(),
        layer: OntologyLayer::AnnotationEvent,
        kind: PrimitiveKind::Reference,
        canonical_symbol: "tags::EntityRef".to_owned(),
        invariants: vec![
            "slug is non-empty snake_case".to_owned(),
            "must resolve to declared entity in validated workflows".to_owned(),
        ],
        projections: vec![
            PrimitiveProjection {
                projection_type: ProjectionType::RustType,
                target: "grimoire-types/src/tags.rs::EntityRef".to_owned(),
                aliases: vec![],
            },
            PrimitiveProjection {
                projection_type: ProjectionType::JsonSchema,
                target: "schemas/_base.schema.json#/$defs/entityRef".to_owned(),
                aliases: vec![],
            },
        ],
    });

    primitives.push(OntologyPrimitive {
        id: "annotation.fact".to_owned(),
        label: "Typed annotation fact".to_owned(),
        layer: OntologyLayer::AnnotationEvent,
        kind: PrimitiveKind::AnnotationFact,
        canonical_symbol: "tags::Annotation".to_owned(),
        invariants: vec![
            "tag key/value pairs parse into typed variants".to_owned(),
            "unknown values produce warnings, not silent coercion".to_owned(),
            "channel() classifies every variant into Context/Craft/Theory/Governance".to_owned(),
        ],
        projections: vec![
            PrimitiveProjection {
                projection_type: ProjectionType::RustType,
                target: "grimoire-types/src/tags.rs::Annotation".to_owned(),
                aliases: vec![],
            },
            PrimitiveProjection {
                projection_type: ProjectionType::MarkdownTag,
                target: "references/gate-tag-vocabulary.md".to_owned(),
                aliases: vec![],
            },
        ],
    });

    primitives.push(OntologyPrimitive {
        id: "annotation.channel".to_owned(),
        label: "Annotation channel (Context/Craft/Theory/Governance)".to_owned(),
        layer: OntologyLayer::AnnotationEvent,
        kind: PrimitiveKind::Projection,
        canonical_symbol: "tags::AnnotationChannel".to_owned(),
        invariants: vec![
            "every Annotation variant maps to exactly one channel".to_owned(),
            "Context and Craft are the primary training input channels".to_owned(),
        ],
        projections: vec![PrimitiveProjection {
            projection_type: ProjectionType::RustType,
            target: "grimoire-types/src/tags.rs::AnnotationChannel".to_owned(),
            aliases: vec![],
        }],
    });

    primitives.push(OntologyPrimitive {
        id: "annotation.paragraph".to_owned(),
        label: "Paragraph-level annotation state".to_owned(),
        layer: OntologyLayer::AnnotationEvent,
        kind: PrimitiveKind::AnnotationFact,
        canonical_symbol: "tags::ParagraphAnnotations".to_owned(),
        invariants: vec![
            "Tier 1 fields are always populated when known".to_owned(),
            "extra holds overflow and Tier 3 theory annotations".to_owned(),
        ],
        projections: vec![PrimitiveProjection {
            projection_type: ProjectionType::RustType,
            target: "grimoire-types/src/tags.rs::ParagraphAnnotations".to_owned(),
            aliases: vec![],
        }],
    });

    primitives.push(OntologyPrimitive {
        id: "annotation.sentence".to_owned(),
        label: "Sentence-level annotation state".to_owned(),
        layer: OntologyLayer::AnnotationEvent,
        kind: PrimitiveKind::AnnotationFact,
        canonical_symbol: "tags::SentenceAnnotations".to_owned(),
        invariants: vec![
            "index is 0-based within the parent Paragraph".to_owned(),
            "auto_derived=true when inferred from surface text".to_owned(),
        ],
        projections: vec![PrimitiveProjection {
            projection_type: ProjectionType::RustType,
            target: "grimoire-types/src/tags.rs::SentenceAnnotations".to_owned(),
            aliases: vec![],
        }],
    });

    // ── Craft instruction tags (StyleVoice) ───────────────────────────────────

    primitives.extend([
        craft_tag(
            "craft.psychic_distance",
            "Psychic distance (Gardner 1–5)",
            "enums::PsychicDistance",
            "psychic_distance",
            vec!["value is 1–5 inclusive", "1 = deepest interiority; 5 = panoramic summary"],
        ),
        craft_tag(
            "craft.consciousness",
            "Consciousness mode (Cohn)",
            "enums::ConsciousnessMode",
            "consciousness",
            vec!["psychonarration | narrated_monologue | quoted_monologue"],
        ),
        craft_tag(
            "craft.tension",
            "Scene tension level (1–10)",
            "tags::Tension",
            "tension",
            vec!["bounded integer 1–10", "8+ indicates climactic tension"],
        ),
        craft_tag(
            "craft.subtext",
            "Active subtext description",
            "String",
            "subtext",
            vec!["free-text slug describing unstated meaning"],
        ),
        craft_tag(
            "craft.speech_act",
            "Speech act type (Searle / Austin)",
            "enums::SpeechAct",
            "speech_act",
            vec![
                "assertive | directive | commissive | expressive | declarative",
                "requires a character tag to identify the speaker",
            ],
        ),
        craft_tag(
            "craft.gaze",
            "Gaze type (Mulvey / Lacan)",
            "enums::GazeType",
            "gaze",
            vec!["male_gaze | female_gaze | queer_gaze | medical_gaze | colonial_gaze"],
        ),
        craft_tag(
            "craft.pov",
            "Point-of-view mode",
            "enums::PovType",
            "pov",
            vec!["first_person | second_person | third_person_limited | third_person_omniscient"],
        ),
        craft_tag(
            "craft.focalization_type",
            "Focalization type (Genette / Bal)",
            "enums::FocalizationType",
            "focalization_type",
            vec!["zero | internal | external | variable | multiple"],
        ),
        craft_tag(
            "craft.sentence_length",
            "Default sentence-length tendency",
            "enums::SentenceLength",
            "sentence_length",
            vec!["short | medium | long | varied"],
        ),
        craft_tag(
            "craft.sentence_type",
            "Sentence type (syntax)",
            "enums::SentenceType",
            "sentence_type",
            vec!["simple | compound | complex | compound_complex | fragment | minor"],
        ),
        craft_tag(
            "craft.dominant_sense",
            "Dominant sensory channel",
            "enums::DominantSense",
            "sense",
            vec!["visual | auditory | olfactory | tactile | gustatory | kinesthetic"],
        ),
    ]);

    // ── Vocabulary terms (InterpretiveSemantics) ──────────────────────────────

    primitives.extend([
        vocab_term(
            "vocab.archetype",
            "Character archetype (Campbell / Vogler)",
            "enums::Archetype",
            "archetype",
            "03-characters/references/character-archetypes.yaml",
        ),
        vocab_term(
            "vocab.wound",
            "Character psychological wound",
            "enums::Wound",
            "wound",
            "03-characters/references/character-wounds.yaml",
        ),
        vocab_term(
            "vocab.drive_model",
            "Character drive model",
            "enums::DriveModel",
            "drive_model",
            "03-characters/references/character-drives.yaml",
        ),
        vocab_term(
            "vocab.arc_type",
            "Character arc type",
            "enums::ArcType",
            "arc",
            "03-characters/references/character-arc-types.yaml",
        ),
        vocab_term(
            "vocab.actant",
            "Actantial role (Greimas)",
            "enums::Actant",
            "actant",
            "03-characters/cross-refs/actantial-map.md",
        ),
        vocab_term(
            "vocab.genre",
            "Story genre",
            "enums::Genre",
            "genre",
            "01-concept/references/genre-patterns.yaml",
        ),
        vocab_term(
            "vocab.plot_type",
            "Plot type",
            "enums::PlotType",
            "plot_type",
            "05-plot-and-structure/references/plot-types.yaml",
        ),
        vocab_term(
            "vocab.collision_pattern",
            "Collision / inciting-incident pattern",
            "enums::CollisionPattern",
            "collision_pattern",
            "02-collision/references/collision-patterns.yaml",
        ),
        vocab_term(
            "vocab.temporal_order",
            "Narrative temporal order (Genette)",
            "enums::TemporalOrder",
            "temporal_order",
            "05-plot-and-structure/references/narrative-time-reference.md",
        ),
        vocab_term(
            "vocab.reader_effect",
            "Reader experience effect (Sternberg / Iser)",
            "enums::ReaderExperienceEffect",
            "reader_effect",
            "05-plot-and-structure/reader-experience/reader-experience-map.md",
        ),
        vocab_term(
            "vocab.diegetic_level",
            "Diegetic level (Genette)",
            "enums::DiegeticLevel",
            "diegetic_level",
            "05-plot-and-structure/architecture/frame-narrative.md",
        ),
    ]);

    // ── Constraint graph ──────────────────────────────────────────────────────

    primitives.push(OntologyPrimitive {
        id: "constraint.tag_constraint".to_owned(),
        label: "Tag constraint (implies / excludes / requires / correlates)".to_owned(),
        layer: OntologyLayer::AnnotationEvent,
        kind: PrimitiveKind::Constraint,
        canonical_symbol: "constraints::TagConstraint".to_owned(),
        invariants: vec![
            "antecedent and consequent are TagPredicates over canonical tag keys".to_owned(),
            "Excludes and Requires constraints produce Error-severity violations".to_owned(),
            "Implies constraints produce Warning-severity violations".to_owned(),
        ],
        projections: vec![PrimitiveProjection {
            projection_type: ProjectionType::RustType,
            target: "grimoire-types/src/constraints.rs::TagConstraint".to_owned(),
            aliases: vec![],
        }],
    });

    // ── Generation / training layer ───────────────────────────────────────────

    primitives.extend([
        generation_type(
            "gen.tier_config",
            "Training export tier configuration",
            "training::TierConfig",
        ),
        generation_type(
            "gen.prose_intent",
            "Prose intent (tags → instruction intermediate layer)",
            "training::ProseIntent",
        ),
        generation_type(
            "gen.quality_target",
            "Single quality target dimension",
            "training::QualityTarget",
        ),
        generation_type(
            "gen.scene_context",
            "Full structured input for prose generation",
            "training::SceneContext",
        ),
        generation_type(
            "gen.prose_passage",
            "Authored prose output (target side of training pair)",
            "training::ProsePassage",
        ),
        generation_type(
            "gen.paragraph",
            "Paragraph with annotations and optional sentence breakdown",
            "training::Paragraph",
        ),
        generation_type(
            "gen.sentence",
            "Sentence with sentence-level annotations",
            "training::Sentence",
        ),
        generation_type(
            "gen.training_example",
            "Fine-tuning pair (SceneContext → ProsePassage)",
            "training::TrainingExample",
        ),
        generation_type(
            "gen.training_dataset",
            "Collection of training examples with statistics",
            "training::TrainingDataset",
        ),
    ]);

    // ── Voice contract layer ──────────────────────────────────────────────────

    primitives.push(OntologyPrimitive {
        id: "voice.contract".to_owned(),
        label: "Author voice contract (style fingerprint)".to_owned(),
        layer: OntologyLayer::StyleVoice,
        kind: PrimitiveKind::Constraint,
        canonical_symbol: "voice::VoiceContract".to_owned(),
        invariants: vec![
            "character_signatures keyed by character slug".to_owned(),
            "focalization config may override narrative_voice defaults per-scene".to_owned(),
        ],
        projections: vec![
            PrimitiveProjection {
                projection_type: ProjectionType::RustType,
                target: "grimoire-types/src/voice.rs::VoiceContract".to_owned(),
                aliases: vec![],
            },
            PrimitiveProjection {
                projection_type: ProjectionType::MarkdownTag,
                target: "07-drafting/voice-contract/".to_owned(),
                aliases: vec![],
            },
        ],
    });

    primitives.push(OntologyPrimitive {
        id: "voice.signature".to_owned(),
        label: "Per-character voice signature (FID / dialogue fingerprint)".to_owned(),
        layer: OntologyLayer::StyleVoice,
        kind: PrimitiveKind::Constraint,
        canonical_symbol: "voice::VoiceSignature".to_owned(),
        invariants: vec![
            "character_id matches a declared Character entity slug".to_owned(),
            "exemplar_passage is the key seed text for fine-tuning".to_owned(),
        ],
        projections: vec![PrimitiveProjection {
            projection_type: ProjectionType::RustType,
            target: "grimoire-types/src/voice.rs::VoiceSignature".to_owned(),
            aliases: vec![],
        }],
    });

    primitives
}

/// Canonical annotation tag keys and legacy aliases.
///
/// This list is the normative key-space contract for parser canonicalization
/// and drift validation against `references/gate-tag-vocabulary.md`.
pub fn tag_key_mappings() -> Vec<TagKeyMapping> {
    vec![
        // Canonical keys
        TagKeyMapping::canonical_only("character", OntologyLayer::NarrativeDomain, PrimitiveKind::Entity),
        TagKeyMapping::canonical_only("setting", OntologyLayer::NarrativeDomain, PrimitiveKind::Entity),
        TagKeyMapping::canonical_only("beat", OntologyLayer::NarrativeDomain, PrimitiveKind::Entity),
        TagKeyMapping::canonical_only("scene", OntologyLayer::NarrativeDomain, PrimitiveKind::Entity),
        TagKeyMapping::canonical_only("chapter", OntologyLayer::NarrativeDomain, PrimitiveKind::Entity),
        TagKeyMapping::canonical_only("thread", OntologyLayer::NarrativeDomain, PrimitiveKind::Entity),
        TagKeyMapping::canonical_only("subplot", OntologyLayer::NarrativeDomain, PrimitiveKind::Entity),
        TagKeyMapping::canonical_only("relationship", OntologyLayer::NarrativeDomain, PrimitiveKind::Entity),
        TagKeyMapping::canonical_only("motif", OntologyLayer::NarrativeDomain, PrimitiveKind::Entity),
        // internal compound attribute key for motif tags; not documented as a
        // top-level vocabulary heading.
        TagKeyMapping {
            canonical_key: "stage".to_owned(),
            alias_key: None,
            layer: OntologyLayer::NarrativeDomain,
            kind: PrimitiveKind::AnnotationFact,
            deprecation: DeprecationLevel::None,
            expected_in_docs_heading: false,
        },
        TagKeyMapping::canonical_only("promise", OntologyLayer::NarrativeDomain, PrimitiveKind::Entity),
        TagKeyMapping::canonical_only("genre", OntologyLayer::InterpretiveSemantics, PrimitiveKind::VocabularyTerm),
        TagKeyMapping::canonical_only("archetype", OntologyLayer::InterpretiveSemantics, PrimitiveKind::VocabularyTerm),
        TagKeyMapping::canonical_only("role", OntologyLayer::InterpretiveSemantics, PrimitiveKind::VocabularyTerm),
        TagKeyMapping::canonical_only("alignment", OntologyLayer::InterpretiveSemantics, PrimitiveKind::VocabularyTerm),
        TagKeyMapping::canonical_only("wound", OntologyLayer::InterpretiveSemantics, PrimitiveKind::VocabularyTerm),
        TagKeyMapping::canonical_only("social_circle", OntologyLayer::InterpretiveSemantics, PrimitiveKind::VocabularyTerm),
        TagKeyMapping::canonical_only("plot_type", OntologyLayer::InterpretiveSemantics, PrimitiveKind::VocabularyTerm),
        TagKeyMapping::canonical_only("collision_pattern", OntologyLayer::InterpretiveSemantics, PrimitiveKind::VocabularyTerm),
        TagKeyMapping::canonical_only("trope", OntologyLayer::InterpretiveSemantics, PrimitiveKind::VocabularyTerm),
        TagKeyMapping::canonical_only("diegetic_level", OntologyLayer::InterpretiveSemantics, PrimitiveKind::VocabularyTerm),
        TagKeyMapping::canonical_only("actant", OntologyLayer::InterpretiveSemantics, PrimitiveKind::VocabularyTerm),
        TagKeyMapping::canonical_only("drive_model", OntologyLayer::InterpretiveSemantics, PrimitiveKind::VocabularyTerm),
        TagKeyMapping::canonical_only("speech_act", OntologyLayer::InterpretiveSemantics, PrimitiveKind::VocabularyTerm),
        TagKeyMapping::canonical_only("gaze", OntologyLayer::InterpretiveSemantics, PrimitiveKind::VocabularyTerm),
        TagKeyMapping::canonical_only("pact", OntologyLayer::InterpretiveSemantics, PrimitiveKind::VocabularyTerm),
        TagKeyMapping::canonical_only("psychic_distance", OntologyLayer::StyleVoice, PrimitiveKind::Constraint),
        TagKeyMapping::canonical_only("consciousness", OntologyLayer::StyleVoice, PrimitiveKind::Constraint),
        TagKeyMapping::canonical_only("subtext", OntologyLayer::StyleVoice, PrimitiveKind::AnnotationFact),
        TagKeyMapping::canonical_only("tension", OntologyLayer::StyleVoice, PrimitiveKind::Constraint),
        TagKeyMapping::canonical_only("flag", OntologyLayer::Governance, PrimitiveKind::Constraint),
        TagKeyMapping::canonical_only("source", OntologyLayer::AnnotationEvent, PrimitiveKind::AnnotationFact),
        TagKeyMapping::canonical_only("source_text", OntologyLayer::AnnotationEvent, PrimitiveKind::AnnotationFact),
        TagKeyMapping::canonical_only("paratext_zone", OntologyLayer::InterchangeSchema, PrimitiveKind::Projection),
        // Common legacy aliases
        TagKeyMapping::alias(
            "character",
            "characters",
            OntologyLayer::NarrativeDomain,
            PrimitiveKind::Entity,
            DeprecationLevel::Soft,
        ),
        TagKeyMapping::alias(
            "scene",
            "scene_id",
            OntologyLayer::NarrativeDomain,
            PrimitiveKind::Entity,
            DeprecationLevel::Soft,
        ),
        TagKeyMapping::alias(
            "setting",
            "location",
            OntologyLayer::NarrativeDomain,
            PrimitiveKind::Entity,
            DeprecationLevel::Hard,
        ),
        TagKeyMapping::alias(
            "stage",
            "motif_stage",
            OntologyLayer::NarrativeDomain,
            PrimitiveKind::AnnotationFact,
            DeprecationLevel::Soft,
        ),
    ]
}

/// Canonicalize a raw tag key to the ontology-approved key.
pub fn canonicalize_tag_key(raw_key: &str) -> String {
    let key = raw_key.trim().to_lowercase();
    for mapping in tag_key_mappings() {
        if mapping.canonical_key == key {
            return mapping.canonical_key;
        }
        if mapping.alias_key.as_deref() == Some(key.as_str()) {
            return mapping.canonical_key;
        }
    }
    key
}

/// Canonical key set (no aliases), for drift checks.
pub fn canonical_tag_keys() -> Vec<String> {
    let mut keys: Vec<String> = tag_key_mappings()
        .into_iter()
        .filter(|m| m.alias_key.is_none())
        .map(|m| m.canonical_key)
        .collect();
    keys.sort();
    keys.dedup();
    keys
}

/// Canonical keys expected to appear as dedicated headings in
/// `references/gate-tag-vocabulary.md`.
pub fn canonical_documented_tag_keys() -> Vec<String> {
    let mut keys: Vec<String> = tag_key_mappings()
        .into_iter()
        .filter(|m| m.alias_key.is_none() && m.expected_in_docs_heading)
        .map(|m| m.canonical_key)
        .collect();
    keys.sort();
    keys.dedup();
    keys
}
