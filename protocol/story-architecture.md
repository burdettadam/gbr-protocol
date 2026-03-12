# Story Architecture

The Story Architecture document encodes book-level structural metadata — the genre contract, collision architecture, inciting incident, antagonist design, protagonist arc, actantial map, and thematic framework. There is exactly one Story Architecture document per CAP Narrative Profile corpus.

For normative field definitions, see [SPECIFICATION.md §6](../SPECIFICATION.md#6-story-architecture).

---

## Design Principle

A story is a structured argument, not a sequence of events. The thematic claim (the "controlling idea") generates the story's moral and emotional logic, which in turn determines what genre, protagonist, antagonist, arc, and collision it requires.

The Story Architecture document makes this top-level design explicit so that scene-level generation can proceed with full awareness of the whole. Without it, scenes are locally coherent but globally arbitrary.

---

## Epistemic Sections (v0.2.0)

Story Architecture documents use the three-section epistemic structure. See [ADR-006](../docs/decisions/ADR-006-observable-structure-interpretation.md) for rationale.

### Observables

Facts directly grounded in the artifact.

| Field | Type | Required | Notes |
|-------|------|----------|-------|
| `book_id` | slug | ✓ | MUST match Entity Registry |
| `inciting_incident.chapter` | integer | | Chapter in which incident occurs |

### Structure

How the narrative is organized at book level.

| Field | Type | Required | Notes |
|-------|------|----------|-------|
| `genre.primary` | enum | ✓ | `enums/literary_theory.json#genre_type` |
| `genre.secondary` | enum | | Genre hybridization |
| `genre.subgenre` | string | | Free-string specificity (e.g., `"southern gothic"`) |
| `collision_type` | enum | | The structural conflict type |
| `collision_pattern` | enum | | The structural shape of the collision |
| `inciting_incident.type` | enum | | Structural nature of the disruption |
| `inciting_incident.description` | string | | Brief description |
| `actantial_map` | object | | Greimas role mapping (see below) |

### Interpretations

Inferred meaning. All fields MAY use the `interpreted_value` wrapper.

| Field | Type | Notes |
|-------|------|-------|
| `power_asymmetry` | enum | Which social world has structural advantage |
| `antagonist_type` | enum | Truby's antagonist taxonomy |
| `antagonist.arc_type` | enum | Does the antagonist change? |
| `antagonist.opposition_level` | enum | Structural depth of challenge |
| `antagonist.thematic_mirror` | boolean | Whether antagonist mirrors protagonist |
| `protagonist_arc.arc_direction` | enum | Macro arc trajectory |
| `protagonist_arc.drive_model` | enum | Primary motivational system |
| `protagonist_arc.lie_believed` | string | False belief driving flaw |
| `protagonist_arc.truth_needed` | string | Thematic truth to accept |
| `protagonist_arc.wound_slug` | slug | Character's wound in registry |

---

## Collision Architecture

The collision is the meeting of two incompatible social worlds that the protagonist must navigate. It is distinct from interpersonal conflict.

---

## Inciting Incident

The inciting incident is the plot event that radically upsets the balance of forces in the protagonist's life and launches the story's central action.

---

## Antagonist

The best antagonists are thematic mirrors of the protagonist — they want the same thing but pursue it through morally opposite means (Truby). When `thematic_mirror: true`, the antagonist's design is constrained by the protagonist's.

---

## Protagonist Arc

The tension between the lie believed and the truth needed drives the arc. Every scene's beat position should be legible against this framework.

---

## Actantial Map

Maps Entity Registry character slugs to Greimas actantial roles:

```json
"actantial_map": {
  "subject": "elizabeth_bennet",
  "object": "honest_marriage",
  "sender": "societal_pressure",
  "receiver": "elizabeth_bennet",
  "helper": "jane_bennet",
  "opponent": "fitzwilliam_darcy"
}
```

The actantial map reveals story grammar beneath genre surface. A character may appear in multiple actantial positions; the map records their *primary* function.

---

## Minimal Valid Story Architecture

```json
{
  "$schema": "https://gbr-protocol.dev/schemas/story-architecture.schema.json",
  "book_id": "pride_and_prejudice",
  "genre": {
    "primary": "romance"
  }
}
```

See [`examples/`](../examples/) for complete story architecture examples.
