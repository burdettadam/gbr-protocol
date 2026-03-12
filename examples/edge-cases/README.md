# Edge Cases

This directory contains CAP Narrative Profile documents that test the protocol's behavior at the boundaries of its design. Each example is a valid CAP Narrative Profile document but exercises a non-standard or potentially ambiguous encoding scenario.

---

## Contents

| File | Edge Case | Theory Notes |
|------|-----------|--------------|
| `dual_pov_split.json` | Two scene cards for the same dramatic scene, each with a different focalizer (Rashomon structure) | Genette: same fabula event, two syuzhet renderings; Booth: competing reliabilities; see companion `dual_pov_split_b.json` (pending) |
| `analepsis_within_analepsis.json` | A scene that is itself an analepsis containing a memory — `narrative_order: analepsis` nested in an analepsis chapter | Genette: second-degree analepsis (analepsis₂ within analepsis₁); Bal: embedded focalization depth ≥ 2 |
| `flat_arc_focalizer.json` | A scene whose focalizer is a flat-arc character (no change arc); tests that `arc_type: flat` is still valid | Weiland: flat-arc protagonist as truth-teller; Cohn: narrated monologue with stable interior |
| `iterative_scene.json` | A scene encoding a repeating habitual pattern — `frequency: iterative` — rather than a singulative event | Genette: iterative frequency ("n times, narrated once"); duration_mode constraints differ from singulative |
| `non_present_character.json` | A scene where an important character is discussed but not physically present; tests `pov_role: non_present` | Booth: character as implied presence; encoding note: non_present characters may still have scene_turns via document-proxy (see threshold/ch01_s02.json) |
| `omniscient_multiple_focalizer.json` | A third-omniscient scene with access to multiple characters' interior states | Genette: zero focalization ("narrator knows more than any character"); Bal: external focalizer; Palmer: intermental thought |

---

## Usage

Edge cases are not conformance test documents — they are correct CAP Narrative Profile data that implementation libraries should handle without error. Validators MUST accept these documents at the appropriate conformance level.

```bash
# Validate all edge-case documents against Level 1 schema
for f in examples/edge-cases/*.json; do
    python reference/python/gbr_validate.py scene-card "$f"
done
```

---

## Notes on `dual_pov_split.json`

This file encodes the first of two companion documents for the same dramatic scene. The Rashomon pattern requires **two separate scene cards** with identical `chapter` and `scene_number_in_chapter` values but different `focalizer` values. A Level 2 validator should accept both as valid; a narrative-layer tool consuming both should detect the pairing and compare character epistemic states.

The companion (`dual_pov_split_b.json`) is pending and will complete the pair.

---

## Notes on `non_present_character.json`

A non-present character (`pov_role: non_present`) may still be encoded with scene turns when they act via a document proxy (letter, journal entry, recording). See [examples/small-story/threshold/ch01_s02.json](../small-story/threshold/ch01_s02.json) for a worked example where a deceased father's journal entry becomes his scene turn — encoded with `active_character: father` and the `embedded_analepsis` block at the `canonical_summary` level.

