# ADR-002: Canonical Summary as Round-Trip Guarantee

**Status:** Accepted  
**Date:** 2026-01-15  
**Authors:** CAP Narrative Profile Protocol Working Group

---

## Context

CAP Narrative Profile needs a mechanism to verify that scene data is semantically complete — that nothing essential has been lost in encoding. Without such a mechanism:
- A scene card could be technically valid (passes schema validation) while being semantically empty
- There would be no way to verify that a scene annotation actually captures narrative content
- Pipeline consumers could not determine whether a scene card was usable for recovery, analysis, or training

Approaches considered:
1. **No summary requirement** — trust annotators to supply complete data
2. **Free-text summary only** — a human-readable description alongside structured data
3. **Structured Canonical Summary with round-trip requirement** — a machine-readable scene representation sufficient to reconstruct narrative content

---

## Decision

**The Canonical Summary is a required, structured sub-document on every Scene Card. Its minimal structure (focalizer, want, scene_turns, outcome, delta) is sufficient to reconstruct the scene's narrative content. The Scene Turn Requirement specifies that scene_turns MUST NOT be empty.**

The round-trip contract is: *a comprehending reader presented only with the Canonical Summary can reconstruct the scene's narrative content.*

---

## Consequences

**Positive:**
- Mechanically verifiable completeness: an empty scene_turns array is a Level 1 schema violation
- The Canonical Summary functions as a scene-level lossless compression
- AI systems can use the Canonical Summary as a ground-truth scene representation for training or evaluation
- The round-trip requirement enforces craft discipline: any scene that cannot be summarized probably lacks the structural elements that make it a scene

**Negative:**
- The Canonical Summary must be authored by a human (or verified AI) per scene — it cannot be auto-generated from the other fields alone
- Annotating 150 scenes requires 150 Canonical Summaries; each takes 3–10 minutes to write carefully
- The required verb vocabulary in scene_turns requires training for annotators

**Neutral:**
- The Canonical Summary is in addition to (not instead of) the structured scene fields; both must be present
- The template for Canonical Summary is defined in `protocol/canonical-summary.md`
- The verb must carry specific narrative force; generic verbs are non-conformant at Level 3

---

## Alternatives Rejected

**Free-text summary only:** A free-text summary is not machine-verifiable and cannot be standardized across corpora. Two annotators summarizing the same scene would produce incomparable outputs.

**No summary:** Without a completeness mechanism, a scene card with all required fields populated but with semantically vacuous content would pass validation. The Canonical Summary is the protocol's answer to the question "is this actually good data?"
