# ADR-001: Scene as the Atomic Unit of CAP Narrative Profile

**Status:** Accepted  
**Date:** 2026-01-15  
**Authors:** CAP Narrative Profile Protocol Working Group

---

## Context

The CAP Narrative Profile Protocol must choose a granularity at which to encode narrative structure. Candidates considered:

1. **Chapter-level granularity** — one document per chapter
2. **Scene-level granularity** — one document per dramatic scene
3. **Beat-level granularity** — one document per narrative beat (action, reaction, decision)
4. **Paragraph-level granularity** — one document per paragraph or sentence

The protocol is designed to support:
- Cross-corpus structural analysis
- Round-trip verification of narrative content
- Character psychology tracking
- Training data generation for narrative-aware AI systems

---

## Decision

**The scene is the atomic unit of CAP Narrative Profile.**

A scene is defined as: a continuous unit of dramatic action in a single location (or a continuous movement through space), through a single focalizer's perception, with a unified want, at least one event, and an outcome.

---

## Consequences

**Positive:**
- Scene granularity matches the level at which most narrative craft decisions are made
- Character emotion, psychology, and tactic can be precisely encoded per-scene
- Narrative temporality (order, duration, frequency) operates meaningfully at the scene level
- Polarity shifts — the primary engine of reader engagement — are scene-level phenomena
- The Canonical Summary is tractable at this granularity (not too long, not too short)

**Negative:**
- Annotating a full novel at scene granularity is labor-intensive (100–200+ scene cards per novel)
- Some narrative units don't fit the definition cleanly (long transitional passages; montage sequences)
- Scene boundaries are sometimes ambiguous; annotators must make judgment calls

**Neutral:**
- Chapter-level properties (number, beat position) are still captured in each Scene Card
- Act-level and whole-book structure is captured in Story Architecture — the Scene Card connects to it through the `beat` field

**Boundary cases:**
- Very short scenes (< 200 words) can still have a Scene Card; a minimal valid scene just needs: scene_id, book_id, chapter, beat, pov, focalizer, and canonical_summary with ≥ 1 scene turn
- Dual POV scenes (some characters alternate focalization within a single dramatic unit) should be split into separate Scene Cards with the same chapter/location if needed for analytical clarity
