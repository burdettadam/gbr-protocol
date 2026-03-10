# ADR-005: Fabula / Syuzhet Separation in Temporal Encoding

**Status:** Accepted  
**Date:** 2026-01-29  
**Authors:** GBR Protocol Working Group

---

## Context

Stories are told out of order. The events of *The Metamorphosis* occur in a single chronological sequence, but *Beloved* uses analepsis extensively. *Catch-22* fractures chronology. *Mrs. Dalloway* moves through time via consciousness.

A naive encoding would simply record scenes in the order they appear in the text. This approach loses the distinction between:
- **When something happened** (fabula time)
- **When it is narrated** (syuzhet time)

This distinction is foundational to narratology (Tomashevsky 1925, Chatman 1978, Genette 1972/1980). Without it:
- A flashback and a chronological scene are indistinguishable
- Cross-corpus analysis of narrative temporality is impossible
- The relationship between "when a character learns a fact" and "when the fact occurred" cannot be encoded

---

## Decision

**GBR Scene Cards encode both fabula and syuzhet time.** The `narrative_order` field on a Scene Card specifies the relationship between fabula and syuzhet for that scene. The `scene_id` and `chapter` / `beat_sequence` fields encode syuzhet position (where the scene appears in the text). The `canonical_summary` content (particularly the `want` and `scene_turns`) is understood to be expressed in fabula time (what happened then), even if the scene is narrated later.

The five `narrative_order` values are: `chronological`, `analepsis`, `prolepsis`, `braided`, `in_medias_res`.

---

## Consequences

**Positive:**
- Cross-corpus analysis of temporal structure is possible: "what proportion of this novel is told in analepsis?"
- Character psychology tracking distinguishes "when X happened to the character" from "when the reader learns about it"
- The distinction enables questions like "what does the reader know at each narrative moment?" (information asymmetry analysis)
- Aligns GBR with established narratological vocabulary

**Negative:**
- Annotators must make two judgments about time: where does this scene appear in the text (syuzhet), and when did it occur in the story world (encoded by the fabula time implied by `narrative_order` and the content)
- Complex temporal structures (braided timelines with embedded analepsis) may require careful annotation decisions
- Some annotators unfamiliar with the fabula/syuzhet distinction will need training

**Neutral:**
- The `duration_mode` and `frequency` fields add two more Genettian temporal dimensions on top of `narrative_order`
- All three temporal fields are optional (not required) — the protocol does not force temporal annotation for basic schema conformance, but semantic conformance benefits from it
- `enums/narrative_time.json` provides full definitions and the Genette citation

---

## Theoretical Basis

- Tomashevsky, Boris. "Thematics." *Russian Formalist Criticism* (1925): fabula vs. syuzhet
- Chatman, Seymour. *Story and Discourse* (1978): kernels and satellites, story vs. discourse
- Genette, Gérard. *Narrative Discourse* (1972/1980): order, duration, frequency
