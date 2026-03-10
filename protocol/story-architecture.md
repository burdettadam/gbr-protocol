# Story Architecture

The Story Architecture document encodes book-level structural metadata — the genre contract, collision architecture, inciting incident, antagonist design, protagonist arc, actantial map, and thematic framework. There is exactly one Story Architecture document per GBR corpus.

For normative field definitions, see [SPECIFICATION.md §6](../SPECIFICATION.md#6-story-architecture).

---

## Design Principle

A story is a structured argument, not a sequence of events. The thematic claim (the "controlling idea") generates the story's moral and emotional logic, which in turn determines what genre, protagonist, antagonist, arc, and collision it requires.

The Story Architecture document makes this top-level design explicit so that scene-level generation can proceed with full awareness of the whole. Without it, scenes are locally coherent but globally arbitrary.

---

## Required Fields

| Field | Type | Notes |
|-------|------|-------|
| `book_id` | slug | MUST match Entity Registry |
| `genre.primary` | enum | `enums/literary_theory.json#genre_type` |

---

## Genre

| Field | Type | Notes |
|-------|------|-------|
| `genre.primary` | enum | Required; primary genre classification |
| `genre.secondary` | enum | Optional; genre hybridization |
| `genre.subgenre` | string | Optional; free-string specificity (e.g., `"southern gothic"`) |

Genre is the reader's contract — it sets expectations of form, content, affect, and resolution. Secondary genre formalizes Derrida's contamination principle: texts always participate in more than one genre.

---

## Collision Architecture

The collision is the meeting of two incompatible social worlds that the protagonist must navigate. It is distinct from interpersonal conflict.

| Field | Type | Notes |
|-------|------|-------|
| `collision_type` | enum | The structural conflict type |
| `collision_pattern` | enum | The structural shape of the collision |
| `power_asymmetry` | enum | Which social world has structural advantage |

---

## Inciting Incident

| Field | Type | Notes |
|-------|------|-------|
| `type` | enum | Structural nature of the disruption |
| `chapter` | integer | Chapter in which incident occurs |
| `description` | string | Brief description |

The inciting incident is the plot event that radically upsets the balance of forces in the protagonist's life and launches the story's central action.

---

## Antagonist

| Field | Type | Notes |
|-------|------|-------|
| `antagonist_type` | enum | Truby's antagonist taxonomy |
| `arc_type` | enum | Does the antagonist change? |
| `opposition_level` | enum | Structural depth of challenge (surface → existential) |
| `thematic_mirror` | boolean | Whether antagonist mirrors protagonist thematically |

The best antagonists are thematic mirrors of the protagonist — they want the same thing but pursue it through morally opposite means (Truby). When `thematic_mirror: true`, the antagonist's design is constrained by the protagonist's.

---

## Protagonist Arc

| Field | Type | Notes |
|-------|------|-------|
| `arc_direction` | enum | Macro arc trajectory across the whole book |
| `drive_model` | enum | Primary motivational system |
| `lie_believed` | string | The false belief driving the protagonist's flaw |
| `truth_needed` | string | The thematic truth the protagonist must accept |
| `wound_slug` | string (slug) | References the character's wound in the registry |

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
