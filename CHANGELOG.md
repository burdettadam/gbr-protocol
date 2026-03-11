# Changelog

All notable changes to the GBR Protocol are documented here.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).  
This project uses [Semantic Versioning](VERSIONING.md).

---

## [Unreleased]

### Changed — SIP Repo Split (Phase 5)

**SIP core extracted to standalone `sip-protocol` repository.**

The Semantic Interaction Protocol is now maintained in a separate [`sip-protocol`](https://github.com/adamburdett/sip-protocol) repo. GBR builds on SIP as a dependency:

- `docs/sip/` removed; all content lives in `sip-protocol`
- `PROFILE.md` promoted to repo root (was `docs/sip/profiles/narrative/PROFILE.md`)
- `reference/rust/src/sip/` removed; SIP Rust types now provided by the `sip-types` crate
- `gbr-types` `Cargo.toml` gains `sip-types = { path = "..." }` dependency
- `gbr-types` `lib.rs`: `pub mod sip;` replaced by `pub use sip_types as sip;` (zero downstream breakage — all `gbr_types::sip::*` paths unchanged)

---

### Added — SIP Extraction (Phases 1–4)

**Semantic Interaction Protocol specification** (`docs/sip/`)
- `docs/sip/SPECIFICATION.md` — full normative SIP spec v0.1.0 (12 sections + 3 appendices)
- `docs/sip/schemas/` — 12 JSON Schema files covering all SIP core objects: artifact, entity, unit, step, state, transition, relationship, participant-state, information-state, view, interpretation, \`_base\`
- `docs/sip/conformance/valid/` — 4 valid fixtures: minimal-artifact, multi-unit-artifact, full-narrative-artifact, full-software-artifact
- `docs/sip/conformance/invalid/` — 6 invalid fixtures: dangling-entity-ref, misordered-steps, missing-observables, missing-protocol, no-change-transition, wrong-protocol-value
- `docs/sip/profiles/narrative/PROFILE.md` — Narrative Profile v0.1.0 (Normative Draft); §§6–9 define: semantic fingerprint ABNF grammar, GBR v0.2.0 → SIP migration guide (7 field-mapping subsections), validation rules (L1/2/3 additions, significance mapping contract `kernel`→`essential`/`satellite`→`supplementary`), changelog

**SIP Rust types** (`reference/rust/src/sip/`, 11 modules)
- `artifact.rs`, `entity.rs`, `unit.rs`, `step.rs` (in unit), `state.rs`, `transition.rs`, `relationship.rs`, `participant_state.rs`, `interpretation.rs`, `view.rs`, `enums.rs`, `mod.rs`
- `SipArtifact`, `SipEntity`, `SipUnit`, `SipStep`, `SipStructure`, `SipObservables`, `SipState`, `SipTransition`, `SipRelationship`, `SipParticipantState`, `SipInformationState`, `Significance`, `CausalRole` types
- 11 unit tests in `mod.rs`: 5 round-trip fixture tests, 4 structural property tests, 2 protocol-value checks — all passing

**Python `SIPValidator` class** (`reference/python/gbr_validate.py`)
- Three-level validation for `sip-artifact` doc type: L1 JSON Schema, L2 entity-ref resolution, L3 step-ordering invariants
- All 10 SIP conformance fixtures produce correct pass/fail results

**CLI tooling** (`grimoire-tooling/`)
- `grimoire-sip-validate` binary — `--path`, `--level 1|2|3`, `--json` flags; validates any SIP artifact against all three conformance levels; all 10 fixtures correct
- `grimoire-sip-convert` binary — `--input <gbr.json>`, `--registry`, `--output`; converts GBR v0.2.0 scene cards to SIP narrative artifacts applying all PROFILE.md §7 field mappings; `kernel`→`essential` / `satellite`→`supplementary` significance translation

**SIP example corpus** (`examples/small-story/threshold/`)
- `ch01_s01.sip.json` — Status Quo + Inciting Incident (hand-authored, validated 3/3)
- `ch01_s02.sip.json` — Revelation; embedded analepsis via document proxy, non-present father as step agent, negative→positive value charge; validated 3/3
- `ch02_s01.sip.json` — Climax/Resolution; closed Booth dramatic irony encoded in unit interpretations, knowledge-gap final irony, positive→positive value charge; validated 3/3

**Design documentation** (`docs/`)
- `docs/decisions/ADR-007-sip-extraction.md` — architectural decision record for SIP extraction
- `docs/GBR_FIELD_AUDIT.md` — field-level audit mapping GBR v0.2.0 fields to SIP equivalents
- `docs/NARRATIVE_PROFILE_MAPPING.md` — detailed GBR→SIP field mapping tables
- `docs/CORE_ONTOLOGY_DRAFT.md` — draft core ontology for domain-agnostic SIP types

---

## [0.2.0] — 2026-03-10

**BREAKING** — Epistemic section restructuring (ADR-006).

### Changed

**Specification**
- `SPECIFICATION.md` v0.2.0 — all document types restructured with epistemic sections (observables / structure / interpretations); Scene Cards add a fourth `craft_targets` section; new §3.6 Principle of Epistemic Separation; updated validation rules (§9–§10)

**Protocol Documents** (`protocol/`)
- `architecture.md` — added "Epistemic Sections (v0.2.0)" section
- `scene-card.md` — field tables reorganized into four epistemic sections
- `character-state.md` — fields reorganized into three epistemic sections with nested examples
- `entity-registry.md` — character, setting, and relationship sub-objects reorganized into epistemic sub-sections
- `story-architecture.md` — flat field tables replaced with observables/structure/interpretations tables

**Schemas** (`schemas/`)
- `enums.schema.json` — added `interpreted_value` `$def` (oneOf wrapper for interpretation enums)
- `scene-card.schema.json` — top-level properties replaced by `observables`, `structure`, `interpretations`, `craft_targets`; scene_turn `$def` uses internal observables/interpretations split; `setting_instance` `$def` added
- `character-state.schema.json` — top-level required reduced to `["observables"]`; three nested sections; oneOf wrappers on all interpretation enum fields; `character_id`/`character_ref` aliases removed (canonical: `character`)
- `story-architecture.schema.json` — restructured into `observables` (`inciting_incident_chapter`), `structure` (`genre` required, structural fields), `interpretations` (thematic/analytical fields with oneOf wrappers)
- `registry.schema.json` — character/setting/relationship `$defs` restructured into nested observables/structure/interpretations

**Examples** (`examples/`)
- All 17 example JSON files converted from flat v0.1.0 format to nested v0.2.0 format
- `minimal/` — 4 files: story-architecture, registry, scene-card, character-state
- `small-story/threshold/` — 5 files: story-architecture, registry, ch01_s01, ch01_s02, ch02_s01
- `small-story/metamorphosis/` — 2 files: registry, scene card (ch01_s01)
- `edge-cases/` — 6 files: omniscient_multiple_focalizer, non_present_character, iterative_scene, flat_arc_focalizer, dual_pov_split, analepsis_within_analepsis

**Conformance Tests** (`conformance/`)
- All 8 conformance test files updated to v0.2.0 nested structure
- Expected error field paths updated (e.g. `pov` → `interpretations.pov`, `character_ref` → `observables.character`)

**Reference Implementation** (`reference/rust/`)
- `entities.rs` — `Character`, `Setting`, `Relationship` structs split into `*Observables`, `*Structure`, `*Interpretations` sub-structs; added `InterpretedValue<T>` enum wrapper

### Added

**Design Documentation** (`docs/`)
- `docs/decisions/ADR-006-observable-structure-interpretation.md` — architectural decision record for the epistemic section split
- `docs/terminology.md` — 6 new definitions: observable, structural field, interpretation, interpreted value, epistemic section, craft target

### Removed
- **Field aliases**: `character_id`, `character_ref` (use `character`); `focalization_type` (use `focalization`); `primary_emotion` (use `emotion`)

---

## [0.1.0] — 2026-03-09

Initial published version of the GBR (Grimoire Book Representation) Protocol.

### Added

**Specification**
- `SPECIFICATION.md` — 14-section normative specification with RFC 2119 language
- `VERSIONING.md` — SemVer policy document
- `CONTRIBUTING.md` — protocol-focused contribution guide

**Protocol Documents** (`protocol/`)
- `architecture.md` — four-layer data model and corpus layout
- `entity-registry.md` — Entity Registry document type reference
- `scene-card.md` — Scene Card field groups and the Scene Turn Requirement
- `story-architecture.md` — Story Architecture document type reference
- `character-state.md` — Character Scene State reference
- `canonical-summary.md` — Canonical Summary template and round-trip contract

**Schemas** (`schemas/`)
- `registry.schema.json` — Entity Registry JSON Schema (Draft-07)
- `scene-card.schema.json` — Scene Card JSON Schema (Draft-07)
- `story-architecture.schema.json` — Story Architecture JSON Schema (Draft-07)
- `character-state.schema.json` — Character Scene State JSON Schema (Draft-07)
- `enums.schema.json` — Enumeration definitions JSON Schema (Draft-07)

**Enumeration Vocabularies** (`enums/`)
- `character.json` — archetype (21), wound (23), alignment (9), role (17), drive_model (5), arc_type (7), actant (6)
- `narrative_voice.json` — pov_type (5), focalization_type (5), consciousness_mode (4), diegetic_level (3), narrator_reliability_type (6), narratee_type (4), pov_role_type (3), tone (16)
- `narrative_time.json` — narrative_order (5), duration_mode (5), frequency (4)
- `scene_structure.json` — beat_type (15), scene_function (8), scene_polarity (24), event_type (11), want_outcome (6), causal_role (6), stakes_domain (8)
- `emotion_psychology.json` — emotion (30), tactic (20), trigger_type (12), emotional_arc_type (9)
- `setting.json` — setting_type (11), time_of_day (9), spatial_structure (9), atmosphere (14), territory_type (7)
- `relationship.json` — relationship_type (18), relationship_dynamic (12), power_balance (5)
- `literary_theory.json` — genre_type (21), collision_type (9), antagonist_type (8), opposition_level (5), transtextuality_type (5), irony_type (7), gaze_type (6), speech_act_category (5), freudian_mechanism (8)

**Conformance Tests** (`conformance/`)
- `valid/minimal_registry.json` — smallest valid Entity Registry
- `valid/minimal_scene.json` — smallest valid Scene Card
- `valid/minimal_story.json` — smallest valid Story Architecture
- `valid/full_scene.json` — Scene Card with all major optional fields
- `invalid/missing_scene_id.json` + sidecar — Level 1 failure: missing required field
- `invalid/unknown_enum_value.json` + sidecar — Level 1 failure: invalid enum value
- `invalid/unresolved_entity_ref.json` + sidecar — Level 2 failure: unresolved entity reference
- `invalid/invalid_canonical_summary.json` + sidecar — Level 2 failure: empty scene_turns

**Examples** (`examples/`)
- `minimal/` — complete minimal corpus: registry, story-architecture, scene-card, character-state
- `small-story/metamorphosis/` — Kafka's *The Metamorphosis* scene and registry examples
- `edge-cases/` — iterative scene, dual POV split examples

**Design Documentation** (`docs/`)
- `terminology.md` — formal glossary of all GBR terms
- `design-principles.md` — rationale for core protocol decisions
- `decisions/ADR-001-scene-as-atomic-unit.md`
- `decisions/ADR-002-canonical-summary.md`
- `decisions/ADR-003-enum-based-semantics.md`
- `decisions/ADR-004-lossless-round-trip.md`
- `decisions/ADR-005-fabula-syuzhet-separation.md`

**Grimoire Template Schemas** (`template-schemas/`)
- All Grimoire phase-based extraction schemas (moved from `schemas/` to distinguish from protocol schemas)

---

## Pre-History

Earlier work on this repository was conducted under the `grimoire-types` Rust crate and `protocol/docs/` directory. That work informed the 0.1.0 release but was not semantically versioned. The original `protocol/docs/` content (THEORY.md, ENUMS.md, ROUND_TRIP.md, ENTITY_TYPES.md, architecture/) is retained for reference pending migration.
