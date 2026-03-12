# ADR-007: Extract Domain-Agnostic Canonical Artifact Protocol from CAP Narrative Profile

**Status:** Accepted  
**Date:** 2026-03-11  
**Authors:** CAP Narrative Profile Protocol Working Group

---

## Context

CAP Narrative Profile v0.2.0 is a well-specified protocol for decomposing narrative artifacts into canonical representations with epistemic separation (ADR-006), lossless round-trip guarantees (ADR-004), and typed enumerations (ADR-003). The protocol works: it has five JSON schemas, seven enum files, a three-level conformance model, reference implementations in both Python and Rust, and validated conformance test suites.

However, a systematic audit of every CAP Narrative Profile field (251 fields across four document types) reveals a structural imbalance:

| Category | Fields | % of Total |
|----------|--------|-----------|
| Domain-agnostic (core) | 29 | 12% |
| Narrative-specific (profile) | 196 | 78% |
| Mixed (needs splitting) | 26 | 10% |

Only 12% of the protocol's surface area is truly domain-agnostic. The remaining 88% is narrative-specific vocabulary (character archetypes, focalization modes, Genette temporal models, scene polarity enums) embedded directly at the protocol level. This has three consequences:

1. **Research framing limitation.** "Built a book annotation format" is a narrower contribution than "built a general semantic decomposition protocol, validated in the narrative domain." The core insight — that observable facts, structural organization, and interpretive meaning should be formally separated and independently verifiable — applies to any complex artifact: codebases, legal documents, architectural designs, clinical records.

2. **Reuse barrier.** Applying the same decomposition to a software system requires either (a) abusing narrative-specific fields (`focalizer` as "primary observer," `beat` as "lifecycle stage") or (b) forking the entire protocol. Neither option is viable.

3. **Coupled evolution.** Adding a new narrative concept (e.g., a drafting theory framework) requires a protocol-level version bump even though the core data model hasn't changed. Core and domain vocabulary evolve at different rates but are version-locked.

The epistemic separation established in ADR-006 (observables/structure/interpretations) already provides the architectural spine for a domain-agnostic core. The audit confirms that the concepts of entities, units, relationships, states, transitions, views, and interpretations-with-provenance are all domain-agnostic. What is domain-specific is the *vocabulary* that fills those structures: the entity types, unit types, relationship types, event types, state types, interpretation types, and enum values.

A toy software interaction (API authentication flow: gateway → auth service → database) was successfully encoded using the same core object model with zero narrative-specific fields — demonstrating that the generalization is real, not hypothetical.

---

## Decision

**Extract a domain-agnostic Canonical Artifact Protocol (CAP) as a separate specification layer. Redefine CAP Narrative Profile as a narrative profile built on top of CAP.**

The architecture becomes:

```
┌──────────────────────────────────────────┐
│ CAP Core (domain-agnostic)               │
│   8 core objects + profile extension     │
│   mechanism + conformance model          │
├──────────────────────────────────────────┤
│ Narrative    │ Software   │ [Future      │
│ Profile      │ Profile    │  Profiles]   │
│ (≈ CAP Narrative Profile)      │ (sketch)   │              │
└──────────────────────────────────────────┘
```

### Core Objects (8 + 2 sub-objects)

1. **Artifact** — top-level container (novel, codebase, contract)
2. **Entity** — persistent participant (character, service, party)
3. **Unit** — atomic transformation block (scene, interaction, clause)
4. **Relationship** — typed directed link between entities/units
5. **State** — point-in-time condition
6. **Transition** — value change (before → after + trigger + delta)
7. **View** — named projection over canonical data
8. **Interpretation** — structured inference with provenance

Sub-objects: **Step** (sub-unit action), **InformationState** (epistemic tracking).

### Profile Extension Mechanism

Profiles declare:
- **Type registries** (7 categories: entity, unit, relationship, interpretation, event, state, causal role)
- **Additional required fields** (conditional on type)
- **Additional epistemic sections** (e.g., narrative's `craft_targets`)
- **Semantic fingerprint grammar** (render/parse contract)
- **Domain-specific validation rules** (layered on conformance)
- **Canonical view types**

### Key Design Choices

- Core schemas use `additionalProperties: true` at profile extension points — profile-specific fields pass core validation; profiles add constraints
- The three-layer epistemic model (observables/structure/interpretations) is core-mandatory; additional sections are profile-optional
- The `interpreted_value` wrapper (`{value, confidence, source}`) is a core concept
- `semantic_fingerprint` is a core structure field; the grammar is profile-defined
- Core conformance levels (Schema → Referential → Round-Trip) carry forward from CAP Narrative Profile, generalized
- Profile enums version independently from core protocol

---

## Consequences

**Positive:**

- The core protocol has a broader research contribution: general semantic decomposition with layered verifiability
- Software, legal, architectural, and other domains can build profiles without narrative-specific overhead
- CAP Narrative Profile's existing 196 narrative-specific fields are preserved exactly — they migrate to a narrative profile, not discarded
- The 14 split proposals (SP-01 through SP-14) from the Phase 1 audit document exactly where each mixed field goes
- Tooling contracts (ingest, render, diff, validate) become domain-agnostic — a single validator engine serves all profiles
- Independent versioning: narrative vocabulary can evolve (new emotion labels, new focalization types) without core protocol version bumps

**Negative:**

- Additional conceptual overhead: two specs instead of one, two version numbers, profile registration mechanism
- Existing CAP Narrative Profile tooling (Rust types, Python validator, conformance tests) must be restructured — all current field paths change from flat to profile-scoped
- Migration cost for existing corpora (threshold examples, conformance fixtures)
- Risk of over-abstraction: the core must be validated against at least two real domains before declaring stability

**Mitigated:**

- Migration is scoped and phased: Phase 2 (spec + schemas), Phase 3 (profile mechanics), Phase 4 (narrative profile), Phase 5 (tooling), Phase 6 (repo split)
- The software toy example validates the core against a second domain immediately
- Existing CAP Narrative Profile content is preserved — this is a restructuring, not a rewrite

---

## Alternatives Considered

1. **Keep CAP Narrative Profile narrative-specific; extract patterns as a "style guide."** Rejected: does not enable actual reuse; other domains would fork rather than extend.

2. **Parameterize CAP Narrative Profile directly (`domain: narrative|software|...`).** Rejected: leads to a monolithic schema where every domain's fields coexist; validation becomes combinatorial; no clear extension mechanism.

3. **Build from scratch with no CAP Narrative Profile foundation.** Rejected: CAP Narrative Profile's 251-field audit, three-level conformance model, canonical summary architecture, and epistemic separation represent 6+ months of validated design work. Extraction preserves this investment.

---

## References

- ADR-006 (Observable / Structure / Interpretation Separation) — epistemic architecture foundation
- ADR-004 (Lossless Round-Trip) — round-trip guarantee generalizes to core
- ADR-002 (Canonical Summary) — becomes semantic fingerprint with profile-defined grammar
- ADR-003 (Enum-Based Semantics) — enum governance model extends to profile registries
- Phase 1 Deliverables:
  - `docs/GBR_FIELD_AUDIT.md` — 251-field classification (core/profile/split)
  - `docs/CORE_ONTOLOGY_DRAFT.md` — 8 core objects defined with examples
  - `docs/NARRATIVE_PROFILE_MAPPING.md` — complete translation table
  - `docs/examples/narrative_new_core.json` — narrative scene in CAP format
  - `docs/examples/software_toy.json` — software interaction generality proof
