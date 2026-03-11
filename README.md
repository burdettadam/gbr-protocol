# GBR Protocol

**GBR Version:** 0.2.0 | **SIP Version:** 0.1.0 | **Status:** Draft

The **Grimoire Book Representation (GBR) Protocol** is a formal standard for representing narrative fiction in a structured, machine-readable format. GBR is built on top of the **Semantic Interaction Protocol (SIP)** — a domain-agnostic substrate for decomposing any artifact type into canonical, epistemically-separated structures.

---

## Architecture

```
┌─────────────────────────────────────────┐
│          GBR Narrative Profile           │  narrative-specific types, enums,
│             (PROFILE.md)                 │  fingerprint grammar, validation rules
├─────────────────────────────────────────┤
│    Semantic Interaction Protocol (SIP)   │  domain-agnostic: entities, units,
│        (sip-protocol repo)               │  steps, states, interpretations
├─────────────────────────────────────────┤
│           GBR Protocol (legacy)          │  scene cards, registries,
│              (SPECIFICATION.md)          │  story architecture, canonical summary
└─────────────────────────────────────────┘
```

New work targets SIP + the narrative profile. Existing GBR documents (v0.2.0) are the migration source; the converter binary produces SIP-native artifacts.

> **Repo split (v0.3.0):** SIP core has been extracted into the standalone [`sip-protocol`](https://github.com/adamburdett/sip-protocol) repository. The `gbr-types` Rust crate now depends on `sip-types` from that repo. The narrative profile document lives at [`PROFILE.md`](PROFILE.md).

---

## Key Principles

- **Lossless Round-Trip** — `parse(render(semantic_structure)) == semantic_structure`
- **Typed Enumerations** — no free text where structure is possible
- **Registry-First** — every entity reference resolves to a declared, named entity
- **Epistemic Separation** — observables, structure, and interpretations are distinct layers
- **Profile Extension** — SIP core is domain-agnostic; narrative semantics live in the profile

---

## Repository Structure

```
gbr-protocol/
  SPECIFICATION.md               # GBR normative specification (v0.2.0)
  PROFILE.md                     # GBR Narrative Profile (SIP profile document)
  VERSIONING.md                  # Versioning policy
  CHANGELOG.md                   # Release history
  protocol/                      # Per-section expansion of the GBR spec
  schemas/                       # GBR JSON Schemas
  enums/                         # Documented enumeration vocabularies
  examples/                      # Structured example corpus (GBR + SIP)
  conformance/                   # GBR validation test suite
  reference/                     # Reference implementations (Rust, Python)
    reference/rust/              # gbr-types crate (depends on sip-types)
    reference/python/            # Python conformance validator (GBR + SIP)
  grimoire-tooling/              # CLI binaries
  docs/                          # Design docs, ADRs, field audits
  template-schemas/              # Grimoire template extraction schemas
```

---

## GBR Specification

The canonical GBR normative document is [SPECIFICATION.md](SPECIFICATION.md).

It covers document types (Scene Card, Entity Registry, Story Architecture, Character State), epistemic section structure, validation rules, and the lossless round-trip guarantee.

---

## Semantic Interaction Protocol (SIP)

SIP is a domain-agnostic protocol for decomposing artifacts into canonical structures. It is the foundation GBR's narrative profile is built on.

- **Repository:** [`sip-protocol`](https://github.com/adamburdett/sip-protocol) — specification, schemas, conformance suite, and `sip-types` Rust crate
- **Narrative Profile:** [`PROFILE.md`](PROFILE.md) — registers narrative-specific types, the semantic fingerprint grammar, migration guide from GBR v0.2.0, and validation rules

---

## GBR Schemas

GBR document JSON Schemas are in [`schemas/`](schemas/):

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

- `examples/minimal/` — smallest valid GBR documents
- `examples/small-story/threshold/` — three-scene story in both GBR and SIP formats
  - `ch01_s01.json` / `ch01_s01.sip.json` — Status Quo + Inciting Incident
  - `ch01_s02.json` / `ch01_s02.sip.json` — Revelation (embedded analepsis via document proxy)
  - `ch02_s01.json` / `ch02_s01.sip.json` — Climax/Resolution (closed dramatic irony)
- `examples/edge-cases/` — nonlinear narrative, unreliable narrator, etc.

---

## Conformance Tests

**GBR** — [`conformance/`](conformance/):
- `conformance/valid/` — documents that MUST pass all validation
- `conformance/invalid/` — documents with known defects; each paired with an `.expected.json`

**SIP** — see [`sip-protocol/conformance/`](https://github.com/adamburdett/sip-protocol/tree/main/conformance) in the sip-protocol repo.

---

## Reference Implementations

### Rust — `gbr-types` crate ([`reference/rust/`](reference/rust/))

Typed structs for GBR document types. SIP core types are provided by the `sip-types` crate (from the sip-protocol repo) and re-exported as `gbr_types::sip::*` for backward compatibility. Includes unit tests covering round-trips and conformance invariants.

```bash
cargo test -p gbr-types
```

### Python — [`reference/python/gbr_validate.py`](reference/python/gbr_validate.py)

Conformance validator for both GBR documents and SIP artifacts. Three-level validation: schema (L1), entity-ref resolution (L2), semantic invariants (L3).

```bash
# Validate a GBR scene card
python3 reference/python/gbr_validate.py scene-card examples/small-story/threshold/ch01_s01.json --level 3

# Validate a SIP artifact
python3 reference/python/gbr_validate.py sip-artifact examples/small-story/threshold/ch01_s01.sip.json --level 3
```

### CLI Binaries ([`grimoire-tooling/`](grimoire-tooling/))

| Binary | Description |
|--------|-------------|
| `grimoire-sip-validate` | Three-level conformance checker for SIP artifacts (`--path`, `--level`, `--json`) |
| `grimoire-sip-convert` | Convert GBR v0.2.0 scene cards to SIP narrative artifacts (`--input`, `--registry`, `--output`) |
| `grimoire-gate-check` | Grimoire phase gate checker |
| `grimoire-export-training` | Export training data pipeline |

```bash
# Validate a SIP artifact
cargo build -p grimoire-tooling --bin grimoire-sip-validate
./target/debug/grimoire-sip-validate --path examples/small-story/threshold/ch01_s01.sip.json --level 3

# Convert a GBR scene card to SIP
./target/debug/grimoire-sip-convert --input examples/small-story/threshold/ch01_s01.json --output /tmp/out.sip.json
```

---

## Versioning

See [VERSIONING.md](VERSIONING.md). Current version: **GBR 0.2.0** / **SIP 0.1.0**.

---

## License

MIT — see [LICENSE](LICENSE)
