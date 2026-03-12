# ADR-006: Observable / Structure / Interpretation Separation

**Status:** Accepted  
**Date:** 2026-03-10  
**Authors:** CAP Narrative Profile Protocol Working Group

---

## Context

CAP Narrative Profile v0.1.0 documents are flat: a Scene Card places `participants` (an observable fact), `beat` (a structural position), `narrator_reliability` (an interpretive judgment), and `target_tension` (a prescriptive craft target) as sibling fields in the same JSON object. The field group names in the spec ("Identity," "Story Structure," "Narrative Voice," "Semantics") hint at a classification, but these groups are purely documentary — the schema does not enforce them, and they do not separate observable facts from inferred meaning.

This creates three concrete problems:

1. **Epistemic hygiene.** A consumer cannot distinguish what is grounded in the artifact (characters named in the text, quoted dialogue, explicit locations) from what is an analyst's or model's inferred claim (emotional state attribution, thematic significance, subtext category). A reader who trusts `participants: ["nadia", "father"]` the same way they trust `narrator_reliability: "factually_unreliable"` conflates direct evidence with theoretical judgment.

2. **Downstream filtering.** ML training pipelines, validation tools, and visualization dashboards have different needs. A training pipeline may want only observables for grounded summarization. A revision dashboard may want only interpretations. A structure analyzer may want beat + causal_role but not tone. With flat documents, every consumer must hard-code which field names belong to which epistemic category.

3. **Confidence stratification.** Observable facts (who is present, what was said) do not need confidence scores — they are grounded. Interpretive fields (emotional state, thematic meaning) can be uncertain, provisional, or contested. A flat schema cannot express "this field is always certain" vs. "this field is an inference that may carry metadata."

Character Scene State has the same problem: `character` (an identity fact) sits beside `objective` (an action grammar), beside `emotional_arc` (an interpretive classification), beside `consciousness_mode` (a rendering instruction).

---

## Decision

**Every CAP Narrative Profile document type uses up to four formally separated top-level sections:**

| Section | Definition | Confidence | Example |
|---------|-----------|------------|---------|
| **`observables`** | Facts directly grounded in the artifact: named entities, quoted content, explicit spatial/temporal markers, visible actions | Always certain; no metadata wrapper | `participants`, `setting`, `prose` |
| **`structure`** | How observables are organized in the canonical model: sequence, containment, adjacency, state transitions, dependency and causal links, groupings | Certain when derived from observables; interpretive when inferred | `beat`, `turn`, `narrative_time`, `canonical_summary` |
| **`interpretations`** | Inferred meaning layered on top of observables and structure: motivations, emotional states, themes, subtext, literary-theoretical classifications | May carry optional `{ value, confidence, source }` metadata | `pov`, `narrator_reliability`, `emotional_arc` |
| **`craft_targets`** | Prescriptive authorial intent — neither description nor inference, but desired effect | Always intentional; no metadata wrapper | `target_tension`, `target_pacing`, `tone` |

**Rules:**

1. `observables` and `structure` are present on all four document types.
2. `interpretations` is present on all four document types.
3. `craft_targets` is present only on Scene Cards (the only document type with prescriptive authoring fields).
4. Observable fields MUST NOT use the `interpreted_value` wrapper. They are grounded facts.
5. Interpretation fields MAY use the `interpreted_value` wrapper: either a plain value or `{ "value": <T>, "confidence": float 0.0–1.0, "source": enum }`.
6. `source` enum values: `human`, `model`, `inferred`, `consensus`.
7. If `source` is `"model"`, a `confidence` value SHOULD be present.

**Canonical Summary handling (Option C):** The `canonical_summary` object stays in `structure` because it is the round-trip-critical structural bridge between fabula and syuzhet. Interpretive metrics (`iceberg_proportion`, `subtext_load`) and per-turn interpretive fields (`emotional_state`, `masked_emotion`, `tactic`, `significance`) are extracted to `interpretations.canonical_metrics`. Each `scene_turn` within the canonical summary uses lightweight internal `observables` / `interpretations` sub-objects.

**Alias cleanup:** v0.2.0 eliminates field aliases: `character_id`/`character_ref` → `character`; `focalization_type` → `focalization`; `primary_emotion` → `emotion`.

---

## Schema Shape

### Interpreted Value wrapper (`$defs.interpreted_value`)

```json
{
  "oneOf": [
    { "$comment": "Plain value — type varies per field" },
    {
      "type": "object",
      "properties": {
        "value": { "$comment": "Same type as the plain form" },
        "confidence": { "type": "number", "minimum": 0.0, "maximum": 1.0 },
        "source": { "enum": ["human", "model", "inferred", "consensus"] }
      },
      "required": ["value"]
    }
  ]
}
```

### Document shape (Scene Card example)

```json
{
  "observables": {
    "scene_id": "threshold_ch01_s01",
    "book_id": "threshold",
    "chapter": 1,
    "participants": ["nadia"],
    "setting": "childhood_home",
    "setting_instance": { "time_of_day": "morning" }
  },
  "structure": {
    "beat": "status_quo",
    "scene_function": "world_building",
    "narrative_time": { "order": "chronological", "duration_mode": "scene" },
    "turn": { "from": "ignorance", "to": "knowledge" },
    "canonical_summary": { "..." : "..." }
  },
  "interpretations": {
    "pov": "third_limited",
    "focalization": "internal_fixed",
    "consciousness_mode": "narrated_monologue",
    "narrator_reliability": {
      "value": "factually_unreliable",
      "confidence": 0.9,
      "source": "human"
    },
    "subtext": { "iceberg_category": "withheld_emotion" },
    "canonical_metrics": {
      "iceberg_proportion": 0.65,
      "subtext_load": 0.6
    }
  },
  "craft_targets": {
    "target_tension": 3,
    "target_pacing": "measured",
    "tone": "clinical"
  }
}
```

---

## Consequences

**Positive:**
- Consumers can filter by epistemic tier without hard-coding field lists
- Training pipelines can extract observables-only corpora for grounded summarization
- Interpretation fields can carry provenance and confidence without polluting grounded facts
- The schema self-documents the epistemic status of every field
- Alias cleanup reduces surface area and eliminates legacy ambiguity

**Negative:**
- Breaking change — all existing corpora must be migrated
- JSON nesting depth increases by one level (`.observables.scene_id` instead of `.scene_id`)
- The canonical summary straddles structure and interpretation, requiring the Option C split
- Downstream tooling must update all field path references

**Neutral:**
- `craft_targets` as a fourth section (rather than folding into interpretations) is a modeling choice — reasonable people could disagree, but prescriptive intent is categorically distinct from both observation and inference
- Some fields are borderline (e.g., `focalization` could be argued as structural rather than interpretive) — the classification is a principled default, not a theorem

---

## Alternatives Considered

1. **Keep flat, add `_tier` suffix to field names** (e.g., `pov_interpretation`, `beat_structure`). Rejected: verbose, error-prone, not machine-filterable without regex.
2. **Tag fields with metadata in schema only (no JSON structure change).** Rejected: consumers still need runtime filtering; schema annotations don't help at query time.
3. **Two tiers (grounded / inferred) instead of four.** Rejected: conflates structure with observables and craft targets with interpretations, losing the middle categories that matter most.

---

## References

- ADR-001 (Scene as Atomic Unit)
- ADR-003 (Enum-Based Semantics)
- ADR-005 (Fabula/Syuzhet Separation)
- SPECIFICATION.md §7–§8 (Scene Card, Character Scene State)
