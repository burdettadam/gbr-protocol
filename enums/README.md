# Enumeration Vocabularies

This directory contains authoritative, documented definitions for all typed enumerations in the CAP Narrative Profile Protocol.

---

## Philosophy

The CAP Narrative Profile Protocol uses closed, typed enumerations for every categorical field. Free text is not permitted where structure is possible.

**Why enums matter:**
- Free-text fields cannot be validated, compared programmatically, or round-tripped deterministically.
- Typed enums make every categorical assertion machine-verifiable.
- Enum values appear verbatim in Canonical Summaries, making them the vocabulary of the round-trip bridge.

---

## File Contents

Each file covers one semantic domain:

| File | Domain |
|------|--------|
| `character.json` | Character archetypes, wounds, alignments, roles, drive models, arc types, actant roles |
| `narrative_voice.json` | POV types, focalization, consciousness modes, psychic distance, diegetic levels, narrator types |
| `narrative_time.json` | Temporal order, duration modes, frequency types |
| `scene_structure.json` | Beat types, scene functions, polarity, event types, want outcomes, causal roles, stakes |
| `emotion_psychology.json` | Emotion types, tactic types, trigger types, emotional arc shapes |
| `setting.json` | Setting types, time of day, spatial structure, atmosphere, territory types |
| `relationship.json` | Relationship types, dynamics, power balance |
| `literary_theory.json` | Theory-grounded enums: genre, collision, antagonist, speech acts, gaze, irony, intertextuality |

---

## Entry Format

Every enum value entry uses this schema:

```json
{
  "value": "the_enum_value",
  "definition": "One sentence defining what this value means in the protocol.",
  "example_usage": "Brief note on when or how this value is applied.",
  "group": "optional grouping label within the domain"
}
```

---

## Conventions

- All enum values use `snake_case`.
- Comparisons are case-sensitive.
- No free-text substitutions are permitted for enum fields.
- To propose a new enum value, open a GitHub issue with: value, definition, example usage, and justification for why existing values are insufficient. See [CONTRIBUTING.md](../CONTRIBUTING.md).

---

## Canonical Source

The authoritative runtime source is the Rust implementation in `reference/rust/src/enums.rs`. The JSON files in this directory are the human-readable documentation layer. In any discrepancy, the Rust source is correct and the JSON documentation should be updated.

The JSON Schema representations of all enums are in `schemas/enums.schema.json`.
