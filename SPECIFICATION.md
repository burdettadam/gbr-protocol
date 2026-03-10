# GBR Protocol Specification

**Version:** 0.1.0  
**Status:** Draft  
**Date:** 2026-03-09

---

## Abstract

The Grimoire Book Representation (GBR) Protocol is a formal standard for representing narrative fiction in a structured, machine-readable format. GBR defines document types, data schemas, enumeration vocabularies, and validation rules that enable bidirectional conversion between structured scene specifications and prose — the *lossless round-trip* guarantee.

---

## Table of Contents

1. [Overview](#1-overview)
2. [Terminology](#2-terminology)
3. [Core Design Principles](#3-core-design-principles)
4. [GBR Document Structure](#4-gbr-document-structure)
5. [Entity Registry](#5-entity-registry)
6. [Story Architecture](#6-story-architecture)
7. [Scene Cards](#7-scene-cards)
8. [Character Scene State](#8-character-scene-state)
9. [Canonical Summary](#9-canonical-summary)
10. [Validation Rules](#10-validation-rules)
11. [Serialization Formats](#11-serialization-formats)
12. [Conformance](#12-conformance)
13. [Versioning](#13-versioning)
14. [Security Considerations](#14-security-considerations)

---

## 1. Overview

### 1.1 Purpose

The GBR Protocol defines a machine-readable representation of narrative fiction that supports:

- Structured annotation of literary texts
- Automated validation of story-structure consistency
- Tool-assisted and AI-assisted writing workflows
- Deterministic bidirectional conversion between structured specifications and prose

### 1.2 Core Guarantee

The protocol's central guarantee is the **lossless round-trip**:

```
parse(render(semantic_structure)) == semantic_structure
```

Any prose passage generated from a GBR scene specification MUST be decomposable back into that exact specification. This is achieved through three mechanisms:

1. **Typed enumerations** — categorical fields use closed vocabularies; free text is not permitted where structure is possible
2. **Canonical summaries** — deterministic serialization bridges scene semantics and prose
3. **Entity registries** — every entity reference resolves to a declared, named entity

### 1.3 Scope

This specification defines the GBR data model, schemas, validation rules, and conformance requirements. It does not define:

- Prose generation algorithms or LLM fine-tuning methodology
- Author-facing workflow tools
- Research evaluation frameworks

### 1.4 Normative Language

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD", "SHOULD NOT", "RECOMMENDED", "MAY", and "OPTIONAL" in this document are to be interpreted as described in [RFC 2119](https://datatracker.ietf.org/doc/html/rfc2119).

---

## 2. Terminology

**Book ID (`book_id`)**  
A unique snake_case identifier that namespaces all GBR documents for a single book. Pattern: `^[a-z0-9_]+$`.

**Canonical Summary**  
A deterministic, grammar-fixed string that serializes the semantic content of a Scene Card. Defined in §9.

**Character Scene State**  
A document describing one character's internal and relational state at the entry and exit boundaries of a scene. Defined in §8.

**Entity**  
A named, declared object in an Entity Registry: a Character, Setting, Relationship, or Want entry.

**Entity Registry**  
A per-book controlled vocabulary declaring all named entities. Every entity reference in a GBR document MUST resolve against the Entity Registry. Defined in §5.

**Enumeration (Enum)**  
A closed vocabulary of typed string values. Defined in `enums/`. Fields typed as enums MUST only accept values from the declared vocabulary.

**Fabula**  
The chronological sequence of story events as they occur in the story world (what happens).

**Focalizer**  
The character through whose perception a scene is filtered. Referenced by slug.

**GBR Corpus**  
A complete set of GBR documents (registry, story architecture, scene cards) for a single book.

**Lossless Round-Trip**  
The property that `parse(render(x)) == x` for all valid GBR semantic structures. See §9.4.

**Scene Card**  
A document specifying everything needed to generate or extract a prose scene. The atomic unit of the protocol. Defined in §7.

**Slug**  
A stable, unique identifier in snake_case format matching `^[a-z0-9_]+$`. Used for all entity references.

**Story Architecture**  
A document encoding book-level structural metadata: genre, arc, antagonist, protagonist design. Defined in §6.

**Syuzhet**  
The narrative presentation order — how events are arranged and disclosed in the text (how it is told).

**Want Vocabulary**  
A per-book map from want slugs to human-readable labels used in canonical summary rendering.

---

## 3. Core Design Principles

### 3.1 Fabula/Syuzhet Separation

The GBR Protocol maintains a strict separation between *fabula* (what happens) and *syuzhet* (how it is told).

- Structured fields capture the fabula.
- The prose field holds the syuzhet.
- The Canonical Summary is the deterministic bridge between them.

No fabula information SHALL be stored only in prose fields. Every structurally significant story fact MUST have a corresponding structured field.

### 3.2 Typed Enumerations Over Free Text

Wherever a categorical value can be defined, GBR MUST use a typed enumeration rather than free text. Rationale: free-text fields cannot be validated, round-tripped deterministically, or compared programmatically.

Free-text fields (strings) are permitted only for:
- Proper names (`name`, `title`, `author`)
- Narratively specific content that is inherently unique (`ghost`, `want`, `need`, `flaw`)
- Sensory details and descriptive signatures

### 3.3 Registry-First Referential Integrity

Every entity reference in a GBR document MUST use a slug that resolves in the Entity Registry. Anonymous entities — entities described inline without a registry declaration — are NOT permitted. This ensures every reference is unambiguous and machine-resolvable.

### 3.4 Scene as Atomic Unit

The Scene Card is the atomic unit of the protocol. Scenes are the level at which dramatic value changes, character states transform, and canonical summaries are computed. Scenes MUST NOT be subdivided; prose passages SHOULD align to exactly one scene card.

### 3.5 Deterministic Canonical Summaries

The Canonical Summary is not a free-text description — it is a fixed-grammar string produced by a deterministic render function. A summary that cannot be produced by `render_summary()` is not a valid GBR canonical summary.

---

## 4. GBR Document Structure

### 4.1 Document Types

A GBR Corpus for a single book consists of four document types:

| Document Type | Cardinality | Description |
|---|---|---|
| Entity Registry | 1 per book | Declared entities (characters, settings, relationships) |
| Story Architecture | 1 per book | Book-level structure (genre, arc, protagonist) |
| Scene Card | 1 per scene | Per-scene specification |
| Character Scene State | 1 per character per scene | Per-character state at scene boundaries |

Character Scene States are embedded within Scene Cards rather than stored as separate files. See §7.

### 4.2 Corpus File Layout

A GBR-compliant book corpus MUST follow this directory structure:

```
{book_id}/
  registry.json              # Entity Registry (§5)
  story_architecture.json    # Story Architecture (§6)
  scenes/
    {book_id}_ch{N}_s{N}.json    # Scene Cards (§7), one per scene
```

File names for scene cards MUST follow the pattern `{book_id}_ch{N:02}_s{N:02}.json` where `N` is zero-padded to at least two digits.

### 4.3 Document Identification

Every GBR document MUST include a `book_id` field that matches the corpus directory name and the `book_id` in the Entity Registry for that corpus.

Scene Cards MUST include a `scene_id` field that is unique within the corpus. The RECOMMENDED format is `{book_id}_ch{N:02}_s{N:02}`.

### 4.4 Document Ordering

Scene Cards have no inherent file-system ordering requirement. Scene ordering is determined by the `chapter` and `scene_index` fields within each Scene Card, not by file name sort order.

---

## 5. Entity Registry

### 5.1 Purpose

The Entity Registry is the per-book controlled vocabulary. It MUST be created before any Scene Card or Story Architecture document for the same book. Every entity reference in these documents MUST resolve against the Entity Registry.

### 5.2 Required Fields

| Field | Type | Description |
|---|---|---|
| `book_id` | string (slug) | Unique book identifier; namespace for all slugs |
| `characters` | object (slug → Character) | All declared characters |
| `settings` | object (slug → Setting) | All declared settings |

### 5.3 Optional Fields

| Field | Type | Description |
|---|---|---|
| `title` | string | Human-readable book title |
| `author` | string | Author name |
| `relationships` | array of Relationship | Directed edges between characters |
| `want_vocabulary` | object (slug → string) | Canonical want labels for summary rendering |
| `narrator` | Narrator object | Narrative voice definition |

### 5.4 Character Entity

#### Required Fields

| Field | Type | Constraint |
|---|---|---|
| `id` | string | MUST match the parent map key; MUST match `^[a-z0-9_]+$` |
| `name` | string | Display name used in prose |

#### Optional Fields

| Field | Type | Enum Reference |
|---|---|---|
| `slot` | string | Cast template position |
| `archetype` | enum | `enums/character.json#archetype` |
| `wound` | enum | `enums/character.json#wound` |
| `alignment` | enum | `enums/character.json#alignment` |
| `role` | enum | `enums/character.json#role` |
| `drive_model` | enum | `enums/character.json#drive_model` |
| `arc_type` | enum | `enums/character.json#arc_type` |
| `actant` | enum | `enums/character.json#actant` |
| `ghost` | string | Specific traumatic backstory event |
| `want` | string | External goal (what they consciously pursue) |
| `need` | string | Thematic truth (what they actually need) |
| `flaw` | string | Behavioral manifestation of wound |
| `voice_signature` | object | Character voice fingerprint (§5.4.1) |

#### 5.4.1 Voice Signature

| Field | Type | Values |
|---|---|---|
| `sentence_length_tendency` | string | `short`, `medium`, `long`, `varied` |
| `vocabulary_register` | string | `colloquial`, `standard`, `formal`, `archaic`, `mixed` |
| `syntax_complexity` | string | `simple`, `moderate`, `complex` |
| `fid_markers` | array of string | FID marker type identifiers |
| `forbidden_words` | array of string | Words this character would not use |
| `signature_phrases` | array of string | Characteristic expressions |

### 5.5 Setting Entity

#### Required Fields

| Field | Type | Constraint |
|---|---|---|
| `id` | string | MUST match parent map key; MUST match `^[a-z0-9_]+$` |
| `name` | string | Display name used in prose |

#### Optional Fields

| Field | Type | Enum Reference |
|---|---|---|
| `type` | enum | `enums/setting.json#setting_type` |
| `general_vibe` | string | Atmosphere / emotional tone |
| `sensory_signature` | array of string | Three defining sensory details |

### 5.6 Relationship Entity

A Relationship is a directed edge between two characters.

#### Required Fields

| Field | Type | Constraint |
|---|---|---|
| `source` | string (slug) | MUST resolve to a declared character |
| `target` | string (slug) | MUST resolve to a declared character |
| `rel_type` | enum | `enums/relationship.json#relationship_type` |

#### Optional Fields

| Field | Type | Enum Reference |
|---|---|---|
| `description` | string | Human-readable relationship summary |
| `dynamic_at_start` | enum | `enums/relationship.json#relationship_dynamic` |
| `dynamic_at_end` | enum | `enums/relationship.json#relationship_dynamic` |
| `power_balance` | enum | `enums/relationship.json#power_balance` |

### 5.7 Want Vocabulary

The `want_vocabulary` object is a flat map from slug to display label:

```json
{
  "want_vocabulary": {
    "honest_respect": "honest respect",
    "social_approval": "social approval"
  }
}
```

Want slugs used in Character Scene State `objective` fields and Canonical Summary `character_want` fields MUST resolve against `want_vocabulary`.

---

## 6. Story Architecture

### 6.1 Purpose

The Story Architecture document encodes book-level structural metadata. It MUST be created once per book.

### 6.2 Required Fields

| Field | Type | Description |
|---|---|---|
| `book_id` | string (slug) | MUST match the Entity Registry |
| `genre` | object | Genre classification |
| `genre.primary` | enum | `enums/literary_theory.json#genre_type` |

### 6.3 Optional Fields

| Field | Type | Description |
|---|---|---|
| `title` / `author` | string | Book metadata |
| `genre.secondary` | enum | Secondary genre classification |
| `genre.subgenre` | string | Free-string subgenre specification |
| `collision_architecture` | object | Social collision structure (§6.4) |
| `inciting_incident` | object | Inciting incident (§6.5) |
| `antagonist` | object | Antagonist design (§6.6) |
| `protagonist_arc` | object | Protagonist arc design (§6.7) |
| `actantial_map` | object | Greimas actantial casting |
| `transtextual_references` | array | Intertextual relationships |
| `themes` | array of string | Core thematic statements |
| `controlling_idea` | string | McKee controlling idea |

### 6.4 Collision Architecture

| Field | Type | Enum Reference |
|---|---|---|
| `collision_type` | enum | `enums/literary_theory.json#collision_type` |
| `collision_pattern` | enum | `enums/literary_theory.json#collision_pattern` |
| `power_asymmetry` | enum | `enums/literary_theory.json#power_asymmetry_type` |

### 6.5 Inciting Incident

| Field | Type | Description |
|---|---|---|
| `type` | enum | `enums/scene_structure.json#inciting_incident_type` |
| `chapter` | integer | Chapter in which incident occurs |
| `description` | string | Brief description |

### 6.6 Antagonist

| Field | Type | Enum Reference |
|---|---|---|
| `antagonist_type` | enum | `enums/literary_theory.json#antagonist_type` |
| `arc_type` | enum | `enums/character.json#arc_type` |
| `opposition_level` | enum | `enums/literary_theory.json#opposition_level` |
| `thematic_mirror` | boolean | Whether antagonist mirrors protagonist thematically |

### 6.7 Protagonist Arc

| Field | Type | Enum Reference |
|---|---|---|
| `arc_direction` | enum | `enums/character.json#arc_type` |
| `drive_model` | enum | `enums/character.json#drive_model` |
| `lie_believed` | string | The false belief driving the protagonist's flaw |
| `truth_needed` | string | The thematic truth the protagonist must accept |

---

## 7. Scene Cards

### 7.1 Purpose

A Scene Card is the atomic unit of the GBR Protocol. It specifies everything needed to generate or extract a prose passage. Every declared scene MUST correspond to exactly one Scene Card.

### 7.2 Required Fields

| Field | Type | Constraint |
|---|---|---|
| `scene_id` | string (slug) | MUST be unique within the corpus |
| `book_id` | string (slug) | MUST match the Entity Registry |
| `chapter` | integer ≥ 1 | Chapter number |
| `beat` | enum | `enums/scene_structure.json#beat_type`; REQUIRED |
| `pov` | enum | `enums/narrative_voice.json#pov_type`; REQUIRED |
| `focalizer` | string (slug) | MUST resolve to a declared character |

### 7.3 Story Structure Fields

| Field | Type | Description |
|---|---|---|
| `scene_index` | integer ≥ 1 | Scene index within chapter |
| `act` | integer 1–5 | Act number |
| `sequence` | enum | `enums/scene_structure.json#sequence_type` |
| `arc_position` | float 0.0–1.0 | Position through story arc |
| `scene_function` | enum | `enums/scene_structure.json#scene_function` |
| `turn` | object | `{from, to}` using `enums/scene_structure.json#scene_polarity`; scene MUST turn |

### 7.4 Narrative Voice Fields

| Field | Type | Enum Reference |
|---|---|---|
| `focalization` | enum | `enums/narrative_voice.json#focalization_type` |
| `psychic_distance` | integer 1–5 | Gardner scale |
| `consciousness_mode` | enum | `enums/narrative_voice.json#consciousness_mode` |
| `diegetic_level` | enum | `enums/narrative_voice.json#diegetic_level` |
| `narratee_type` | enum | `enums/narrative_voice.json#narratee_type` |
| `narrator_reliability` | enum | `enums/narrative_voice.json#narrator_reliability_type` |

### 7.5 Narrative Time Fields

The `narrative_time` object operationalizes Genette's three temporal dimensions.

| Field | Type | Enum Reference |
|---|---|---|
| `narrative_time.order` | enum | `enums/narrative_time.json#narrative_order` |
| `narrative_time.duration_mode` | enum | `enums/narrative_time.json#duration_mode` |
| `narrative_time.frequency` | enum | `enums/narrative_time.json#frequency` |
| `narrative_time.duration` | string | Absolute story time covered (e.g., `"45 minutes"`) |

### 7.6 Craft Settings

| Field | Type | Constraint |
|---|---|---|
| `target_tension` | integer 1–5 | 1 = low suspense, 5 = maximum |
| `target_pacing` | enum | `enums/narrative_time.json#duration_mode` |
| `tone` | enum | `enums/narrative_voice.json#tone` |

### 7.7 Setting

| Field | Type | Constraint |
|---|---|---|
| `setting` | string (slug) | MUST resolve to a declared Setting |
| `setting_instance` | object | Scene-specific setting details (§7.7.1) |

#### 7.7.1 Setting Instance

| Field | Type | Enum Reference |
|---|---|---|
| `time_of_day` | enum | `enums/setting.json#time_of_day` |
| `weather` | string | Free description |
| `lighting_source` | string | Free description |
| `lighting_quality` | string | Free description |

### 7.8 Character States

The `character_states` array MUST contain one Character Scene State object (§8) for each character present in the scene. The focalizer MUST always have a corresponding Character Scene State.

```json
"character_states": [
  { "character": "elizabeth_bennet", "pov_role": "focalizer", ... },
  { "character": "fitzwilliam_darcy", "pov_role": "participant", ... }
]
```

### 7.9 Semantic Fields

| Field | Type | Description |
|---|---|---|
| `event_type` | enum | `enums/scene_structure.json#event_type`; primary story event |
| `participants` | array of slugs | Characters involved (MUST resolve in registry) |
| `canonical_summary` | string | Deterministic summary (§9); REQUIRED if scene has been rendered |
| `subtext` | object | Subtext annotation (§7.9.1) |

#### 7.9.1 Subtext Object

| Field | Type | Enum Reference |
|---|---|---|
| `iceberg_category` | enum | `enums/literary_theory.json#iceberg_category` |
| `maxim_violated` | enum | `enums/literary_theory.json#gricean_maxim` |
| `violation_type` | enum | `enums/literary_theory.json#violation_type` |

---

## 8. Character Scene State

### 8.1 Purpose

A Character Scene State describes one character's internal and relational state at the entry and exit boundaries of a scene — their emotions, knowledge, objectives, tactics, and arc position.

### 8.2 Required Fields

| Field | Type | Constraint |
|---|---|---|
| `character` | string (slug) | MUST resolve to a declared character |
| `pov_role` | enum | `enums/narrative_voice.json#pov_role_type` |

### 8.3 Emotional State

| Field | Type | Description |
|---|---|---|
| `emotional_state_entry` | EmotionObject | Emotion at scene entry |
| `emotional_state_exit` | EmotionObject | Emotion at scene exit |
| `emotional_arc` | enum | `enums/emotion_psychology.json#emotional_arc_type` |

**EmotionObject:**

| Field | Type | Constraint |
|---|---|---|
| `emotion` | enum | `enums/emotion_psychology.json#emotion` |
| `intensity` | integer 1–5 | Plutchik intensity scale |
| `secondary_emotion` | enum | `enums/emotion_psychology.json#emotion` (optional) |
| `masked` | boolean | Whether emotion is performed vs. felt |

### 8.4 Epistemic State

| Field | Type | Description |
|---|---|---|
| `knowledge_at_entry` | array of KnowledgeObject | What character knows at scene open |
| `knowledge_gaps` | array of KnowledgeObject | What character does not know |
| `knowledge_gained` | array of KnowledgeObject | What character learns during scene |

**KnowledgeObject:**

| Field | Type | Values |
|---|---|---|
| `domain` | string | `secrets`, `plans`, `relationships`, `identity`, `past`, `future`, `feelings`, `allegiances` |
| `predicate` | string | `knows`, `believes`, `suspects`, `denies`, `fears`, `desires` |
| `about_role` | string (slug) | Character the knowledge concerns |
| `certainty` | float 0.0–1.0 | Epistemic confidence |

### 8.5 Action Grammar

| Field | Type | Description |
|---|---|---|
| `objective` | ObjectiveObject | Character's scene objective |
| `tactic` | enum | `enums/emotion_psychology.json#tactic` |
| `obstacle` | string | What blocks the objective |
| `trigger_type` | enum | `enums/emotion_psychology.json#trigger_type` |

**ObjectiveObject:**

| Field | Type | Values |
|---|---|---|
| `verb` | string | Transitive action verb ("to convince", "to escape") |
| `object_type` | string | `information`, `commitment`, `submission`, `approval`, `resource`, `alliance`, `escape`, `forgiveness`, `recognition` |
| `target_role` | string (slug) | Character who holds what is wanted |
| `constraint` | string | What the character cannot do while pursuing objective |

### 8.6 Arc State

| Field | Type | Enum Reference |
|---|---|---|
| `arc_beat` | enum | `enums/character.json#arc_beat_type` |
| `arc_direction` | string | `advancing`, `regressing`, `stable` |
| `want_outcome` | enum | `enums/scene_structure.json#want_outcome` |
| `wound_triggered` | boolean | Whether psychological wound was activated |

### 8.7 Social State

| Field | Type | Enum Reference |
|---|---|---|
| `social_mask` | string | The public persona performed |
| `social_role` | enum | `enums/literary_theory.json#social_role_type` |
| `territory_type` | enum | `enums/setting.json#territory_type` |

---

## 9. Canonical Summary

### 9.1 Purpose

The Canonical Summary is a deterministic, fixed-grammar string that serializes scene semantics. It is the protocol's mechanism for ensuring lossless round-trip (§1.2).

### 9.2 Template

```
{POV_CHAR} {EVENT_VERB} {PARTICIPANTS} at {LOCATION}; 
wants {WANT_OBJECT} [{OUTCOME}]; 
stakes={STAKES}, atmosphere={ATMOSPHERE}, role={CAUSAL_ROLE}.
```

### 9.3 Slot Definitions

| Slot | Schema Field | Type | Render Rule |
|---|---|---|---|
| `{POV_CHAR}` | `focalizer` | slug | `registry.characters[slug].name` |
| `{EVENT_VERB}` | `event_type` | enum | `EVENT_VERBS[event_type]` (§9.5) |
| `{PARTICIPANTS}` | `participants[]` | slugs | Comma-joined display names |
| `{LOCATION}` | `setting` | slug | `registry.settings[slug].name` |
| `{WANT_OBJECT}` | `character_want` | slug | `registry.want_vocabulary[slug]` |
| `{OUTCOME}` | `want_outcome` | enum | `GRANTED`, `DENIED`, `DEFERRED`, `PYRRHIC` |
| `{STAKES}` | `stakes_domain` | enum | One of `enums/scene_structure.json#stakes_domain` |
| `{ATMOSPHERE}` | `atmosphere` | enum | One of `enums/setting.json#atmosphere` |
| `{CAUSAL_ROLE}` | `causal_role` | enum | One of `enums/scene_structure.json#causal_role` |

### 9.4 Round-Trip Contract

Two functions implement the canonical summary:

```
render_summary(semantic_dict, registry) → string
parse_summary(string, registry) → semantic_dict
```

The following invariant MUST hold for all valid GBR semantic structures:

```
parse_summary(render_summary(d, r), r) == d
```

A summary string that cannot be produced by `render_summary` is NOT a valid Canonical Summary and MUST be rejected by validators.

### 9.5 Event Type Verb Mapping

| Event Type | Verb Phrase |
|---|---|
| `arrival` | arrives with |
| `departure` | departs from |
| `confrontation` | confronts |
| `confession` | confesses to |
| `discovery` | discovers |
| `decision` | decides |
| `proposal` | proposes to |
| `refusal` | refuses |
| `acceptance` | accepts |
| `betrayal` | betrays |
| `reconciliation` | reconciles with |
| `revelation` | reveals to |
| `deception` | deceives |
| `seduction` | seduces |
| `negotiation` | negotiates with |
| `escape` | escapes from |
| `pursuit` | pursues |
| `rescue` | rescues |
| `loss` | loses |
| `transformation` | transforms at |

### 9.6 Validity Rules

A Canonical Summary is valid if and only if:

1. It was produced by `render_summary` — any manually written summary that produces a different output than `render_summary` on the same input is invalid.
2. Every slug in the summary resolves against the book's Entity Registry.
3. `parse_summary` applied to the string produces the original semantic structure.

---

## 10. Validation Rules

### 10.1 Schema Validation

All GBR documents MUST pass JSON Schema validation against the corresponding schema in `schemas/`:

| Document Type | Schema File |
|---|---|
| Entity Registry | `schemas/registry.schema.json` |
| Story Architecture | `schemas/story-architecture.schema.json` |
| Scene Card | `schemas/scene-card.schema.json` |
| Character Scene State | embedded in `schemas/scene-card.schema.json` |
| Enumerations | `schemas/enums.schema.json` |

### 10.2 Referential Integrity Rules

The following constraints MUST be satisfied:

1. **Character references:** Every character slug referenced in a Scene Card or Character Scene State MUST exist in `registry.characters`.
2. **Setting references:** Every setting slug referenced in a Scene Card MUST exist in `registry.settings`.
3. **Focalizer:** The `focalizer` slug in a Scene Card MUST exist in `registry.characters` and MUST have a corresponding entry in `character_states` with `pov_role == "focalizer"`.
4. **Want references:** Every want slug referenced in a Character Scene State `objective` or Canonical Summary MUST exist in `registry.want_vocabulary`.
5. **Uniqueness:** No two Scene Cards within a corpus MAY share the same `scene_id`.
6. **Book ID consistency:** The `book_id` in all documents in a corpus MUST be identical.

### 10.3 Enumeration Rules

1. All enum fields MUST contain a value declared in the corresponding `enums/` definition file.
2. Unknown enum values MUST cause validation failure.
3. Enum comparisons are case-sensitive.

### 10.4 Canonical Summary Rules

1. If a Scene Card contains a `canonical_summary`, it MUST pass round-trip validation (§9.4).
2. `canonical_summary` is REQUIRED for Scene Cards used as training data or for corpus-level analysis.
3. All entity names in the summary MUST correspond to names in the Entity Registry.

### 10.5 Validation Severity Levels

| Level | Meaning | Example |
|---|---|---|
| ERROR | Document is invalid; MUST be rejected | Missing required field |
| WARNING | Document may have an issue | `canonical_summary` absent |
| INFO | Informational observation | Unused want_vocabulary entry |

---

## 11. Serialization Formats

### 11.1 Primary Format: JSON

GBR documents MUST be serialized as [JSON](https://www.json.org/) (ECMA-404 / RFC 8259) unless an alternative format is explicitly negotiated.

Rules for JSON serialization:

1. Documents MUST use UTF-8 encoding.
2. Documents SHOULD use human-readable indentation (2 or 4 spaces).
3. Field ordering within objects is not significant.
4. `null` values for optional fields MAY be omitted entirely; omitting an optional field is equivalent to its being null.

### 11.2 $schema Field

GBR documents SHOULD include a `$schema` field referencing the applicable JSON Schema URI:

```json
{
  "$schema": "https://gbr-protocol.dev/schemas/registry.schema.json",
  "book_id": "..."
}
```

### 11.3 Alternative Formats

Implementations MAY support additional serialization formats (YAML, TOML, MessagePack) provided that the serialized data satisfies §10. The JSON representation is canonical; all other formats are projections.

---

## 12. Conformance

### 12.1 Conformance Levels

GBR defines three conformance levels. Each level is cumulative.

#### Level 1 — Schema Conformance

A document is Schema Conformant if:

- It passes JSON Schema validation against the applicable schema in `schemas/`.
- All required fields are present and correctly typed.
- All enum fields contain declared values.

#### Level 2 — Referential Conformance

A document is Referentially Conformant if:

- It satisfies Level 1.
- All entity references resolve against the Entity Registry (§10.2).
- `book_id` is consistent across all documents in the corpus.

#### Level 3 — Round-Trip Conformance

A document is Round-Trip Conformant if:

- It satisfies Level 2.
- All `canonical_summary` fields are present.
- All canonical summaries pass the round-trip invariant (§9.4).

### 12.2 Conformance Claims

Implementations claiming GBR conformance MUST specify which level they support and MUST pass the corresponding tests in `conformance/`.

### 12.3 Conformance Test Suite

The `conformance/` directory contains:

- `conformance/valid/` — documents that MUST pass all three conformance levels
- `conformance/invalid/` — documents with known defects; validators MUST reject each with the expected error category

---

## 13. Versioning

GBR follows [Semantic Versioning 2.0.0](https://semver.org/).

| Change Type | Version Component | Example Trigger |
|---|---|---|
| Breaking schema change | Major | Removing a required field; changing an enum value |
| New optional field or enum value | Minor | Adding a new optional scene card field |
| Clarification | Patch | Fixing a description; correcting a typo in a rule |

The current version is **GBR 0.1.0**. See [VERSIONING.md](VERSIONING.md) for the full versioning policy and [CHANGELOG.md](CHANGELOG.md) for the version history.

Documents MAY include a `gbr_version` field indicating the GBR specification version they were authored against.

---

## 14. Security Considerations

### 14.1 No Executable Content

GBR documents are data — they contain no executable code, scripts, or evaluation instructions. Implementations MUST NOT execute any content from GBR documents.

### 14.2 Entity Reference Validation

Implementations MUST validate entity references against the registry before resolving them. Unresolved references MUST produce a validation error and MUST NOT silently succeed or fall back to partial resolution.

### 14.3 Input Size

GBR documents MAY be arbitrarily large (a complete novel corpus may include hundreds of scene cards). Implementations SHOULD enforce configurable size limits on individual documents and SHOULD stream-parse large corpora rather than loading them entirely into memory.

### 14.4 Personally Identifiable Information

GBR documents may contain character names, biographical details, and psychological profiles that correspond to real people in autofiction or biographical narratives. Authors and systems handling such documents are responsible for compliance with applicable data protection regulations. The GBR Protocol does not mandate any specific PII handling; it notes the risk exists.

### 14.5 Schema Reference Security

If implementations fetch external `$schema` URIs, they MUST use HTTPS and SHOULD pin to a known good version. Implementations SHOULD support offline schema validation to avoid network-dependent security boundaries.

---

## Appendix A: Reference Implementation

The canonical reference implementation is:

- **Rust:** `reference/rust/` — the `gbr-types` crate provides typed structs, enum definitions, schema generation, and validation binaries
- **Python:** `reference/python/validate.py` — schema and referential conformance validation

Reference implementations are informative. The specification (this document) is the authoritative source; divergence between this document and a reference implementation MUST be resolved in favor of this document.

---

## Appendix B: Related Specifications

- [JSON Schema Draft 2020-12](https://json-schema.org/specification.html)
- [RFC 8259: The JavaScript Object Notation (JSON) Data Interchange Format](https://datatracker.ietf.org/doc/html/rfc8259)
- [RFC 2119: Key Words for Use in RFCs to Indicate Requirement Levels](https://datatracker.ietf.org/doc/html/rfc2119)
- [Semantic Versioning 2.0.0](https://semver.org/)

---

*GBR Protocol Specification — Version 0.1.0 — 2026-03-09*
