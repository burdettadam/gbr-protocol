# Scene Card

A Scene Card is the atomic unit of the GBR Protocol. It specifies everything needed to generate or extract a prose passage — structural position, narrative voice, craft targets, character states, and the canonical summary.

For normative field definitions, see [SPECIFICATION.md §7](../SPECIFICATION.md#7-scene-cards). For field-level design rationale, see the [architecture docs](../protocol/architecture.md) and [ADRs](../docs/decisions/).

---

## Design Principle

The Scene Card operationalizes the fabula/syuzhet distinction:

> *Fabula* (what happens) must be fully recoverable from *syuzhet* (how it is told), and vice versa.

Structured fields capture the fabula. The `prose` field (when present) holds the syuzhet. The `canonical_summary` is the deterministic bridge between them, enabling the round-trip guarantee.

Every field in a Scene Card represents a theoretical commitment about what constitutes a complete, agentic description of a scene. Fields are not arbitrary metadata — they are the minimal set of structured variables required to generate any prose passage that is consistent with the story's established facts.

---

## Field Groups

### Identity

| Field | Type | Required | Notes |
|-------|------|----------|-------|
| `scene_id` | slug | ✓ | Unique within corpus; `{book_id}_ch{N:02}_s{N:02}` |
| `book_id` | slug | ✓ | MUST match Entity Registry |
| `chapter` | integer ≥ 1 | ✓ | |
| `scene_index` | integer ≥ 1 | | Scene position within chapter |

### Story Structure

| Field | Type | Required | Notes |
|-------|------|----------|-------|
| `act` | integer 1–5 | | Three-act minimum; five-act maximum |
| `sequence` | enum | | Gulino / Snyder sequence type |
| `beat` | enum | ✓ | Monomyth beat position |
| `arc_position` | float 0.0–1.0 | | Precise story position |
| `scene_function` | enum | | Structural work this scene performs |
| `turn.from` / `turn.to` | enum pair | | Scene MUST turn (McKee); value-sign change |

### Narrative Voice

| Field | Type | Required | Notes |
|-------|------|----------|-------|
| `pov` | enum | ✓ | Point of view type |
| `focalization` | enum | | Genette focalization mode |
| `focalizer` | slug | ✓ | MUST resolve to declared character |
| `psychic_distance` | integer 1–5 | | Gardner scale |
| `consciousness_mode` | enum | | Cohn's three modes |
| `diegetic_level` | enum | | Genette narrative level |
| `narratee_type` | enum | | Prince narratee type |
| `narrator_reliability` | enum | | Booth/Nünning reliability classification |

### Narrative Time

| Field | Type | Notes |
|-------|------|-------|
| `narrative_time.order` | enum | Genette temporal order |
| `narrative_time.duration_mode` | enum | Story time vs. discourse time ratio |
| `narrative_time.frequency` | enum | Singulative / iterative / repetitive |
| `narrative_time.duration` | string | Absolute story time covered |

### Craft Settings

| Field | Type | Notes |
|-------|------|-------|
| `target_tension` | integer 1–5 | 1 = low, 5 = maximum |
| `target_pacing` | enum | Target pacing mode |
| `tone` | enum | Narrator / author attitude |

### Setting

| Field | Type | Notes |
|-------|------|-------|
| `setting` | slug | MUST resolve to declared setting |
| `setting_instance.time_of_day` | enum | |
| `setting_instance.weather` | string | |
| `setting_instance.lighting_source` | string | |

### Character States

The `character_states` array embeds one [Character Scene State](character-state.md) per character present. The focalizer MUST have an entry with `pov_role: "focalizer"`.

### Semantics

| Field | Type | Notes |
|-------|------|-------|
| `event_type` | enum | Primary story event |
| `participants` | array of slugs | MUST resolve in registry |
| `canonical_summary` | string | Deterministic summary (see [canonical-summary.md](canonical-summary.md)) |
| `subtext.iceberg_category` | enum | Hemingway iceberg layer |
| `subtext.maxim_violated` | enum | Gricean maxim |

---

## Scene Turn Requirement

Every scene MUST contain a `turn` object declaring the value-sign change:

```json
"turn": {
  "from": "ignorance",
  "to": "knowledge"
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
  "beat": "ordeal",
  "pov": "third_limited",
  "focalizer": "elizabeth_bennet",
  "character_states": [
    {
      "character": "elizabeth_bennet",
      "pov_role": "focalizer"
    }
  ]
}
```

See [`examples/`](../examples/) for complete scene card examples.
