# ADR-004: Lossless Round-Trip as the Core Integrity Guarantee

**Status:** Accepted  
**Date:** 2026-01-22  
**Authors:** GBR Protocol Working Group

---

## Context

The GBR Protocol must define what "valid data" means. A document can be schema-valid (passes JSON Schema validation) without being semantically useful. Consider:

```json
{
  "scene_id": "abc",
  "book_id": "xyz",
  "chapter": 1,
  "beat": "climax",
  "pov": "third_limited",
  "focalizer": "hero",
  "canonical_summary": {
    "chapter": 1, "scene": 1,
    "location": "A place",
    "time": "daytime",
    "focalizer": "hero",
    "want": "hero wants something",
    "scene_turns": [{"event_type": "action", "agent": "hero", "verb": "does", "patient": "something", "result": "a result"}],
    "outcome": "achieved",
    "delta": "something changed"
  }
}
```

This document passes schema validation. It is useless for any analytical or training purpose.

---

## Decision

**The GBR Protocol's Level 3 conformance (Round-Trip Conformant) requires that Canonical Summaries pass a "round-trip test": a comprehending reader with only the Canonical Summary must be able to reconstruct the scene's narrative content.**

This is not a mechanical test. It is a craft test. The Canonical Summary must:
1. Name the location and time precisely enough to anchor the scene
2. State the focalizer's want specifically (not `"hero wants something"`)
3. Include scene turns that record specific, named events with specific, meaningful verbs and results
4. State an outcome that is specific to this scene's want
5. Record a delta that states a specific change (not `"something changed"`)

---

## Consequences

**Positive:**
- Level 3 conformance is a meaningful quality bar, not just a technical check
- The Canonical Summary becomes a genuine narrative artifact, not a metadata label
- AI systems trained on Level 3 conformant data are trained on high-quality narrative data
- The delta field in particular enforces narrative efficiency: every scene must change something

**Negative:**
- Level 3 conformance cannot be fully automated — it requires human or carefully prompted AI judgment
- The distinction between "specific enough for round-trip" and "too vague" is a judgment call
- Annotators need training and examples to understand the standard

**Neutral:**
- The protocol defines three conformance levels; Level 1 is the minimum for schema testing; Level 3 is the goal for research corpora
- `conformance/valid/full_scene.json` demonstrates a Level 3 compliant Scene Card
- Generic verbs (`does`, `goes`, `says`, `something`) are reliable indicators of Level 3 non-compliance; they should trigger review

---

## Related

- ADR-002 (Canonical Summary design)
- SPECIFICATION.md §12 (Conformance)
- `protocol/canonical-summary.md` (verb guidance, template)
- `conformance/valid/full_scene.json` (example)
- `conformance/invalid/invalid_canonical_summary.json` (counter-example)
