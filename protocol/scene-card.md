# Scene Card

A Scene Card is the atomic unit of the CAP Narrative Profile Protocol. It specifies everything needed to generate or extract a prose passage — structural position, narrative voice, craft targets, character states, and the canonical summary.

For normative field definitions, see [SPECIFICATION.md §7](../SPECIFICATION.md#7-scene-cards). For field-level design rationale, see the [architecture docs](../protocol/architecture.md) and [ADRs](../docs/decisions/).

---

## Design Principle

The Scene Card operationalizes the fabula/syuzhet distinction:

> *Fabula* (what happens) must be fully recoverable from *syuzhet* (how it is told), and vice versa.

Structured fields capture the fabula. The `prose` field (when present) holds the syuzhet. The `canonical_summary` is the deterministic bridge between them, enabling the round-trip guarantee.

Every field in a Scene Card represents a theoretical commitment about what constitutes a complete, agentic description of a scene. Fields are not arbitrary metadata — they are the minimal set of structured variables required to generate any prose passage that is consistent with the story's established facts.

---

## Epistemic Sections (v0.2.0)

Scene Cards are organized into four epistemic sections. See [ADR-006](../docs/decisions/ADR-006-observable-structure-interpretation.md) and [architecture.md](../protocol/architecture.md) for rationale.

### Identity (top-level)

| Field | Type | Required | Notes |
|-------|------|----------|-------|
| `scene_id` | slug | ✓ | Unique within corpus; `{book_id}_ch{N:02}_s{N:02}` |
| `book_id` | slug | ✓ | MUST match Entity Registry |
| `chapter` | integer ≥ 1 | ✓ | |
| `scene_index` | integer ≥ 1 | | Scene position within chapter |

### Observables

Facts directly grounded in the artifact. These fields MUST NOT use the `interpreted_value` wrapper.

| Field | Type | Required | Notes |
|-------|------|----------|-------|
| `focalizer` | slug | ✓ | MUST resolve to declared character |
| `event_type` | enum | | Primary story event |
| `participants` | array of slugs | | MUST resolve in registry |
| `diegetic_level` | enum | | Genette narrative level |
| `narratee_type` | enum | | Prince narratee type |
| `setting_instance` | object | | See Setting Instance below |

**Setting Instance** (nested in `observables`):

| Field | Type | Notes |
|-------|------|-------|
| `setting` | slug | MUST resolve to declared setting |
| `time_of_day` | enum | |
| `weather` | string | |
| `lighting_source` | string | |
| `atmosphere` | enum | `enums/setting.json#atmosphere` |

### Structure

How the narrative is organized. Fields MUST NOT use the `interpreted_value` wrapper.

| Field | Type | Required | Notes |
|-------|------|----------|-------|
| `beat` | enum | ✓ | Monomyth beat position |
| `act` | integer 1–5 | | Three-act minimum; five-act maximum |
| `sequence` | enum | | Gulino / Snyder sequence type |
| `arc_position` | float 0.0–1.0 | | Precise story position |
| `scene_function` | enum | | Structural work this scene performs |
| `turn` | object | | Scene MUST turn (McKee); value-sign change |
| `causal_role` | enum | | Scene's narrative function |
| `canonical_summary` | string | | Deterministic summary (see [canonical-summary.md](canonical-summary.md)) |
| `narrative_time.order` | enum | | Genette temporal order |
| `narrative_time.duration_mode` | enum | | Story time vs. discourse time ratio |
| `narrative_time.frequency` | enum | | Singulative / iterative / repetitive |
| `narrative_time.duration` | string | | Absolute story time covered |

### Interpretations

Inferred meaning. All fields MAY use the `interpreted_value` wrapper (`{ "value": <T>, "confidence": 0.0–1.0, "source": "<string>" }`).

| Field | Type | Required | Notes |
|-------|------|----------|-------|
| `pov` | enum | ✓ | Point of view type |
| `focalization` | enum | | Genette focalization mode |
| `psychic_distance` | integer 1–5 | | Gardner scale |
| `consciousness_mode` | enum | | Cohn's three modes |
| `narrator_reliability` | enum | | Booth/Nünning reliability classification |
| `subtext.iceberg_category` | enum | | Hemingway iceberg layer |
| `subtext.maxim_violated` | enum | | Gricean maxim |
| `canonical_metrics` | object | | Interpretive scores extracted from canonical summary |

### Craft Targets

Authorial dials — not observable from the text, not structural analysis.

| Field | Type | Notes |
|-------|------|-------|
| `target_tension` | integer 1–5 | 1 = low, 5 = maximum |
| `target_pacing` | enum | Target pacing mode |
| `tone` | enum | Narrator / author attitude |

### Character States

The `character_states` array embeds one [Character Scene State](character-state.md) per character present. The focalizer MUST have an entry with `observables.pov_role: "focalizer"`.

---

## Scene Turn Requirement

Every scene MUST contain a `structure.turn` object declaring the value-sign change:

```json
"structure": {
  "turn": {
    "from": "ignorance",
    "to": "knowledge"
  }
}
```

A scene that ends with the same value sign it began with is dramatically non-functional — it is passage, not scene.

---

## Minimal Valid Scene Card

```json
{
  "$schema": "https://gbr-protocol.dev/schemas/scene-card.schema.json",
  "scene_id": "pride_ch34_s01",
  "book_id": "pride_and_prejudice",
  "chapter": 34,
  "observables": {
    "focalizer": "elizabeth_bennet"
  },
  "structure": {
    "beat": "ordeal"
  },
  "interpretations": {
    "pov": "third_limited"
  },
  "character_states": [
    {
      "observables": {
        "character": "elizabeth_bennet",
        "pov_role": "focalizer"
      }
    }
  ]
}
```

See [`examples/`](../examples/) for complete scene card examples.
