# CAP Narrative Profile Protocol

**CAP Narrative Profile Version:** 0.2.0 | **CAP Version:** 0.1.0 | **Status:** Draft

The **Canonical Artifact Protocol — Narrative Profile Protocol** is a formal standard for representing narrative fiction in a structured, machine-readable format. CAP Narrative Profile is built on top of the **Canonical Artifact Protocol (CAP)** — a domain-agnostic substrate for decomposing any artifact type into canonical, epistemically-separated structures.

---

## Architecture

```
┌─────────────────────────────────────────┐
│          CAP Narrative Profile Narrative Profile           │  narrative-specific types, enums,
│             (PROFILE.md)                 │  fingerprint grammar, validation rules
├─────────────────────────────────────────┤
│    Canonical Artifact Protocol (CAP)   │  domain-agnostic: entities, units,
│        (cap-protocol repo)               │  steps, states, interpretations
├─────────────────────────────────────────┤
│           CAP Narrative Profile Protocol (legacy)          │  scene cards, registries,
│              (SPECIFICATION.md)          │  story architecture, canonical summary
└─────────────────────────────────────────┘
```

New work targets CAP + the narrative profile. Existing CAP Narrative Profile documents (v0.2.0) are the migration source; the converter binary produces CAP-native artifacts.

> **Repo split (v0.3.0):** CAP core has been extracted into the standalone [`cap-protocol`](https://github.com/adamburdett/cap-protocol) repository. The `cap-narrative-types` Rust crate now depends on `cap-types` from that repo. The narrative profile document lives at [`PROFILE.md`](PROFILE.md).

---

## Key Principles

- **Lossless Round-Trip** — `parse(render(semantic_structure)) == semantic_structure`
- **Typed Enumerations** — no free text where structure is possible
- **Registry-First** — every entity reference resolves to a declared, named entity
- **Epistemic Separation** — observables, structure, and interpretations are distinct layers
- **Profile Extension** — CAP core is domain-agnostic; narrative semantics live in the profile

---

## Repository Structure

```
cap-narrative-profile/
  SPECIFICATION.md               # CAP Narrative Profile normative specification (v0.2.0)
  PROFILE.md                     # CAP Narrative Profile Narrative Profile (CAP profile document)
  VERSIONING.md                  # Versioning policy
  CHANGELOG.md                   # Release history
  protocol/                      # Per-section expansion of the CAP Narrative Profile spec
  schemas/                       # CAP Narrative Profile JSON Schemas
  enums/                         # Documented enumeration vocabularies
  examples/                      # Structured example corpus (CAP Narrative Profile + CAP)
  conformance/                   # CAP Narrative Profile validation test suite
  reference/                     # Reference implementations (Rust, Python)
    reference/rust/              # cap-narrative-types crate (depends on cap-types)
    reference/python/            # Python conformance validator (CAP Narrative Profile + CAP)
  grimoire-tooling/              # CLI binaries
  docs/                          # Design docs, ADRs, field audits
  template-schemas/              # Grimoire template extraction schemas
```

---

## CAP Narrative Profile Specification

The canonical CAP Narrative Profile normative document is [SPECIFICATION.md](SPECIFICATION.md).

It covers document types (Scene Card, Entity Registry, Story Architecture, Character State), epistemic section structure, validation rules, and the lossless round-trip guarantee.

---

## Canonical Artifact Protocol (CAP)

CAP is a domain-agnostic protocol for decomposing artifacts into canonical structures. It is the foundation CAP Narrative Profile's narrative profile is built on.

- **Repository:** [`cap-protocol`](https://github.com/adamburdett/cap-protocol) — specification, schemas, conformance suite, and `cap-types` Rust crate
- **Narrative Profile:** [`PROFILE.md`](PROFILE.md) — registers narrative-specific types, the semantic fingerprint grammar, migration guide from CAP Narrative Profile v0.2.0, and validation rules

---

## CAP Narrative Profile Schemas

CAP Narrative Profile document JSON Schemas are in [`schemas/`](schemas/):

| Schema | Document Type |
|--------|---------------|
| `schemas/registry.schema.json` | Entity Registry |
| `schemas/scene-card.schema.json` | Scene Card |
| `schemas/story-architecture.schema.json` | Story Architecture |
| `schemas/character-state.schema.json` | Character Scene State |
| `schemas/enums.schema.json` | Enumeration vocabularies |

Struct schemas auto-generated from Rust types are in `schemas/generated/` (regenerate with `cargo run --bin grimoire-export-schemas`).

---

## Enumerations

Documented enum definitions are in [`enums/`](enums/). Each file covers a domain and includes a `definition` and `example_usage` for every value.

---

## Examples

Structured example documents are in [`examples/`](examples/):

- `examples/minimal/` — smallest valid CAP Narrative Profile documents
- `examples/small-story/threshold/` — three-scene story in both CAP Narrative Profile and CAP formats
  - `ch01_s01.json` / `ch01_s01.sip.json` — Status Quo + Inciting Incident
  - `ch01_s02.json` / `ch01_s02.sip.json` — Revelation (embedded analepsis via document proxy)
  - `ch02_s01.json` / `ch02_s01.sip.json` — Climax/Resolution (closed dramatic irony)
- `examples/edge-cases/` — nonlinear narrative, unreliable narrator, etc.

---

## Conformance Tests

**CAP Narrative Profile** — [`conformance/`](conformance/):
- `conformance/valid/` — documents that MUST pass all validation
- `conformance/invalid/` — documents with known defects; each paired with an `.expected.json`

**CAP** — see [`cap-protocol/conformance/`](https://github.com/adamburdett/cap-protocol/tree/main/conformance) in the cap-protocol repo.

---

## Reference Implementations

### Rust — `cap-narrative-types` crate ([`reference/rust/`](reference/rust/))

Typed structs for CAP Narrative Profile document types. CAP core types are provided by the `cap-types` crate (from the cap-protocol repo) and re-exported as `cap_narrative_types::sip::*` for backward compatibility. Includes unit tests covering round-trips and conformance invariants.

```bash
cargo test -p cap-narrative-types
```

### Python — [`reference/python/gbr_validate.py`](reference/python/gbr_validate.py)

Conformance validator for both CAP Narrative Profile documents and CAP artifacts. Three-level validation: schema (L1), entity-ref resolution (L2), semantic invariants (L3).

```bash
# Validate a CAP Narrative Profile scene card
python3 reference/python/gbr_validate.py scene-card examples/small-story/threshold/ch01_s01.json --level 3

# Validate a CAP artifact
python3 reference/python/gbr_validate.py sip-artifact examples/small-story/threshold/ch01_s01.sip.json --level 3
```

### CLI Binaries ([`grimoire-tooling/`](grimoire-tooling/))

| Binary | Description |
|--------|-------------|
| `grimoire-cap-validate` | Three-level conformance checker for CAP artifacts (`--path`, `--level`, `--json`) |
| `grimoire-cap-convert` | Convert CAP Narrative Profile v0.2.0 scene cards to CAP narrative artifacts (`--input`, `--registry`, `--output`) |
| `grimoire-gate-check` | Grimoire phase gate checker |
| `grimoire-export-training` | Export training data pipeline |

```bash
# Validate a CAP artifact
cargo build -p grimoire-tooling --bin grimoire-cap-validate
./target/debug/grimoire-cap-validate --path examples/small-story/threshold/ch01_s01.sip.json --level 3

# Convert a CAP Narrative Profile scene card to CAP
./target/debug/grimoire-cap-convert --input examples/small-story/threshold/ch01_s01.json --output /tmp/out.sip.json
```

---

## Versioning

See [VERSIONING.md](VERSIONING.md). Current version: **CAP Narrative Profile 0.2.0** / **CAP 0.1.0**.

---

## License

MIT — see [LICENSE](LICENSE)
