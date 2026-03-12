# CAP Narrative Profile Protocol Architecture

This document describes the CAP Narrative Profile document model, layer relationships, and corpus layout. For normative rules, see [SPECIFICATION.md](../SPECIFICATION.md).

---

## Four-Layer Data Model

CAP Narrative Profile represents a book as four layered document types:

```
┌─────────────────────────────────────┐
│           Story Architecture         │  Book-level structure
└─────────────────────────────────────┘
┌─────────────────────────────────────┐
│           Entity Registry            │  Controlled vocabulary
└─────────────────────────────────────┘
┌─────────────────────────────────────┐
│           Scene Cards                │  Per-scene specifications
│   ┌───────────────────────────────┐ │
│   │     Character Scene States    │ │  Per-character per-scene state
│   └───────────────────────────────┘ │
└─────────────────────────────────────┘
```

### Layer 1: Entity Registry

The per-book controlled vocabulary. Declares all characters, settings, and relationships that may be referenced by any other document in the corpus.

**Why this layer exists:** Narrative is built from a finite set of entities acting within a relational network. Making that entity set explicit and machine-readable enables referential integrity, deterministic rendering, and canonical normalization.

### Layer 2: Story Architecture

Book-level structural metadata: genre, collision architecture, protagonist arc, antagonist design, actantial map. Applies to the whole book; there is exactly one per corpus.

**Why this layer exists:** Scene-level generation must be aware of the whole to be coherent. Genre sets reader expectations; the protagonist arc determines where each scene falls; the actantial map defines the story's structural grammar.

### Layer 3: Scene Cards

Per-scene specifications. Each Scene Card specifies structural position, narrative voice, craft targets, setting, character states, and the canonical summary. One per scene.

**Why scenes are atomic:** Scenes are the level at which dramatic value changes. Paragraphs are too small (sub-structural), chapters are too large (multi-event). The scene boundary is where character states transform and canonical summaries are computed.

### Character Scene States (embedded in Scene Cards)

Per-character-per-scene state snapshots, embedded in the `character_states` array of each Scene Card. Capture emotional, epistemic, and actional state at scene entry and exit.

**Why entry/exit dual state:** The delta between entry and exit state is what the scene *does*. A scene that leaves a character in the same state it found them has not performed dramatic work.

---

## Corpus Layout

```
{book_id}/
  registry.json                      # Entity Registry
  story_architecture.json            # Story Architecture
  scenes/
    {book_id}_ch01_s01.json          # Scene Card (chapter 1, scene 1)
    {book_id}_ch01_s02.json
    ...
```

Scene card file names MUST follow the pattern `{book_id}_ch{N:02}_s{N:02}.json`. Scene ordering within a chapter is determined by the `scene_index` field, not file name order.

---

## Dependency Order

Documents in a corpus have a strict dependency order:

1. Entity Registry MUST be created first — all other documents reference it
2. Story Architecture MAY be created after the registry
3. Scene Cards MUST be created after the registry; they reference it for entity resolution
4. Character Scene States are embedded within Scene Cards; they inherit scene card dependencies

Validators MUST load the Entity Registry before validating referential constraints in any other document.

---

## Epistemic Sections (v0.2.0)

Every CAP Narrative Profile document type internally separates its fields into up to four epistemic sections. This separation is structural — the JSON schema enforces it.

```
┌─────────────────────────────────────────────────┐
│  CAP Narrative Profile Document (any type)                        │
│                                                 │
│  ┌───────────────┐  ┌────────────────────────┐  │
│  │  observables   │  │  structure              │  │
│  │  (grounded     │  │  (how observables are   │  │
│  │   in artifact) │  │   organized)            │  │
│  └───────────────┘  └────────────────────────┘  │
│  ┌───────────────┐  ┌────────────────────────┐  │
│  │ interpretations│  │  craft_targets          │  │
│  │  (inferred     │  │  (authorial intent;     │  │
│  │   meaning)     │  │   Scene Cards only)     │  │
│  └───────────────┘  └────────────────────────┘  │
└─────────────────────────────────────────────────┘
```

### Observables

Facts directly grounded in the artifact. Named participants, quoted dialogue, explicit locations, visible actions, explicit objects and ordering markers.

Observable fields are always certain. They MUST NOT carry the `interpreted_value` metadata wrapper.

### Structure

How observables are organized in the canonical model. Sequence, containment, adjacency, state transitions, dependency links, causal links (when explicitly grounded), grouping into scenes/modules/etc.

The `canonical_summary` object lives in `structure` because it is the round-trip-critical bridge between fabula and syuzhet.

### Interpretations

Inferred meaning layered on top of observables and structure. Motivation, emotional state, theme, subtext, implied conflict class, literary-theoretical classifications.

Interpretation fields MAY carry an optional metadata wrapper: `{ "value": <T>, "confidence": 0.0–1.0, "source": "human" | "model" | "inferred" | "consensus" }`. Plain values (without the wrapper) are also accepted.

Interpretive metrics extracted from the canonical summary (`iceberg_proportion`, `subtext_load`, and per-turn fields like `emotional_state`, `masked_emotion`, `tactic`, `significance`) live in `interpretations.canonical_metrics`.

### Craft Targets

Prescriptive authorial intent — neither description nor inference, but desired effect. Present only on Scene Cards.

### Section applicability by document type

| Document Type | `observables` | `structure` | `interpretations` | `craft_targets` |
|---|---|---|---|---|
| Entity Registry | ✓ | ✓ | ✓ | — |
| Story Architecture | ✓ | ✓ | ✓ | — |
| Scene Card | ✓ | ✓ | ✓ | ✓ |
| Character Scene State | ✓ | ✓ | ✓ | — |

See [ADR-006](../docs/decisions/ADR-006-observable-structure-interpretation.md) for the full rationale.

---

## Design Rationale

See [docs/design-principles.md](../docs/design-principles.md) for the rationale behind each design decision, and [docs/decisions/](../docs/decisions/) for Architecture Decision Records.
