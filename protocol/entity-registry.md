# Entity Registry

The Entity Registry is the per-book controlled vocabulary. It declares all named entities — characters, settings, relationships, and want labels — that may be referenced by any other CAP Narrative Profile document in the corpus.

For normative field definitions, see [SPECIFICATION.md §5](../SPECIFICATION.md#5-entity-registry). For field-level design rationale, see the [reference implementation entity types](../reference/rust/src/entities.rs).

---

## Design Principle

The registry operationalizes a claim from structuralist narratology: narrative is not a sum of free-floating events, but the combinatorial output of a finite set of entities acting within a relational network. Propp's 31 functions unfold from 7 roles; Greimas's narrative grammar reduces all narrative events to 6 actantial positions. The registry makes this underlying entity set explicit and machine-readable.

**Single source of truth:** Every entity in the story has exactly one declaration in the registry. All references use slugs that resolve to that declaration. This eliminates ambiguity — "Lena," "Dr. Marsh," and "Eleanor" are all the same person if they share a slug.

---

## Characters

Characters are stored as a map: `slug → Character`.

The map structure (rather than an array) enables O(1) lookup by slug. Every scene card references characters by slug; lookup is the protocol's primary integrity check.

#### Epistemic Sections (v0.2.0)

Character entries use the three-section epistemic structure. See [ADR-006](../docs/decisions/ADR-006-observable-structure-interpretation.md) for rationale.

- **Observables:** `id`, `name` (REQUIRED), `aliases`
- **Structure:** `role`, `voice_signature`
- **Interpretations:** `archetype`, `wound`, `alignment`, `drive_model`, `arc_type`, `actant`, `ghost`, `want`, `need`, `flaw`

### Required Fields

- `id` — snake_case slug; MUST match the parent map key (e.g., `"elizabeth_bennet"`)
- `name` — display name used in prose (e.g., `"Elizabeth Bennet"`)

### Enum Fields

All enum values reference [enums/character.json](../enums/character.json):

| Field | Description |
|-------|-------------|
| `archetype` | Campbell/Vogler archetype (21 variants) |
| `wound` | Core psychological wound (23 variants) |
| `alignment` | 9-cell moral alignment grid |
| `role` | Story function (17 variants) |
| `drive_model` | Motivational framework (5 variants) |
| `arc_type` | Character arc trajectory (7 variants) |
| `actant` | Greimas actantial position (6 variants) |

### Narrative-Specific String Fields

| Field | Description |
|-------|-------------|
| `ghost` | The specific traumatic backstory event (Truby's "ghost") |
| `want` | External goal — what the character consciously pursues |
| `need` | Thematic truth — what the character actually needs to become whole |
| `flaw` | Behavioral manifestation of the wound's misbelief (Aristotle's hamartia) |

The distinction between `want` and `need` is the engine of character arc. The gap between them is the story.

### Voice Signature

Optional character voice fingerprint for prose generation consistency:

```json
{
  "sentence_length_tendency": "varied",
  "vocabulary_register": "formal",
  "syntax_complexity": "complex",
  "fid_markers": ["exclamatory_syntax", "evaluative_language"],
  "forbidden_words": ["awesome"],
  "signature_phrases": ["I dare say", "Upon my word"]
}
```

---

## Settings

Settings are stored as a map: `slug → Setting`.

#### Epistemic Sections (v0.2.0)

- **Observables:** `id`, `name` (REQUIRED)
- **Structure:** `type`
- **Interpretations:** `general_vibe`, `sensory_signature`

### Required Fields

- `id` — snake_case slug; MUST match the parent map key
- `name` — display name used in prose

### Optional Fields

| Field | Description |
|-------|-------------|
| `type` | Setting category enum (`enums/setting.json#setting_type`) |
| `general_vibe` | Atmosphere / emotional tone |
| `sensory_signature` | Array of three defining sensory details |

Physical location is not decoration — it is semantically charged. Settings encode spatial oppositions (safe/dangerous, known/unknown, sacred/profane) that inform every scene that occurs within them.

---

## Relationships

Relationships are stored as an array of directed edge objects (not a map), because multiple relationship types can exist between the same pair of characters.

#### Epistemic Sections (v0.2.0)

- **Observables:** `source`, `target` (REQUIRED)
- **Structure:** `rel_type` (REQUIRED)
- **Interpretations:** `description`, `dynamic_at_start`, `dynamic_at_end`, `power_balance`

### Required Fields

- `source` — character slug (MUST resolve in registry)
- `target` — character slug (MUST resolve in registry)
- `rel_type` — relationship type enum (`enums/relationship.json#relationship_type`)

### Optional Fields

| Field | Description |
|-------|-------------|
| `description` | Human-readable summary |
| `dynamic_at_start` | Relationship dynamic at story start |
| `dynamic_at_end` | Relationship dynamic at story end |
| `power_balance` | Power asymmetry direction |

---

## Want Vocabulary

The `want_vocabulary` map provides canonical display labels for character want slugs used in canonical summaries:

```json
{
  "want_vocabulary": {
    "honest_respect": "honest respect",
    "social_approval": "social approval",
    "economic_independence": "economic independence"
  }
}
```

All want slugs referenced in Character Scene State `objective` fields or in canonical summaries MUST resolve against this map.

---

## Example

```json
{
  "$schema": "https://gbr-protocol.dev/schemas/registry.schema.json",
  "book_id": "pride_and_prejudice",
  "title": "Pride and Prejudice",
  "author": "Jane Austen",
  "characters": {
    "elizabeth_bennet": {
      "id": "elizabeth_bennet",
      "name": "Elizabeth Bennet",
      "archetype": "hero",
      "wound": "identity_rejection",
      "alignment": "chaotic_good",
      "role": "protagonist",
      "drive_model": "perception",
      "arc_type": "positive_change",
      "actant": "subject",
      "want": "To marry for love, not convenience",
      "need": "To see past her own prejudices"
    }
  },
  "settings": {
    "longbourn": {
      "id": "longbourn",
      "name": "Longbourn",
      "type": "estate_interior",
      "general_vibe": "Modest gentility under financial pressure"
    }
  },
  "want_vocabulary": {
    "honest_respect": "honest respect"
  }
}
```

See [`examples/`](../examples/) for complete registry examples.
