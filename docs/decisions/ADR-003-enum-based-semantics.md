# ADR-003: Typed Enumerations for All Categorical Fields

**Status:** Accepted  
**Date:** 2026-01-22  
**Authors:** GBR Protocol Working Group

---

## Context

The GBR Protocol includes many categorical fields: point of view, beat type, emotion, archetype, scene function, atmospheric quality, and dozens more. For each field, the protocol must choose between:

1. **Free text** — annotators write natural language values
2. **Suggested vocabulary** — values are recommended but not enforced
3. **Closed enumeration** — only values in the defined enum list are valid

The key tradeoffs are:
- **Expressivity vs. comparability** — free text can express anything; enums can be compared programmatically
- **Annotator flexibility vs. analytical tractability** — free text is easy to write; enums are easy to analyze
- **Growth vs. stability** — free text grows naturally; enums require governance

---

## Decision

**All categorical fields in GBR use closed, typed enumerations. Free text is not permitted for enum fields. All enum values use snake_case.**

The enum vocabulary is documented in `enums/` and formalized in `schemas/enums.schema.json`. The authoritative runtime source is `reference/rust/src/enums.rs`.

---

## Consequences

**Positive:**
- All categorical values can be validated at parse time
- Cross-corpus analysis is possible: aggregate "how many scenes have `beat: climax`?" across any GBR corpus
- Enum values appear verbatim in Canonical Summaries, creating a shared vocabulary for the round-trip bridge
- Downstream consumers (AI training pipelines, visualization tools, search indices) can treat enum fields as structured data without natural language parsing
- Annotation quality is checkable: an unknown enum value is a schema error, not a semantic debate

**Negative:**
- Annotators cannot express distinctions not captured in the existing enums
- Some narrative phenomena defy clean categorization; annotators must choose the "closest" value
- Enum governance requires a process (GitHub issues + ADR) for each addition
- Early enum choices may prove wrong; changing an enum value is a breaking change

**Neutral:**
- The `enums/README.md` documents the governance process for proposing new values
- Literary theory provides a principled basis for enum vocabulary — the protocol draws from Genette, Booth, Cohn, Gardner, Vogler, Plutchik, Truby, McKee, and others
- The enum list is intentionally large to minimize "closest value" compromises

---

## Governance

Proposing a new enum value requires opening a GitHub issue with:
1. The proposed `value` (snake_case)
2. A one-sentence `definition`
3. An `example_usage`
4. Justification of why no existing value suffices

Accepted proposals are implemented in the Rust source (`enums.rs`), documented in the relevant JSON file (`enums/`), and formalized in the schema. This is a minor version increment (0.x.0 → 0.(x+1).0).

Removing an enum value is a major version change (0.x.0 → 1.0.0) and requires a deprecation period.
