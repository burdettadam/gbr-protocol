# GBR Protocol

**Version:** 0.1.0 | **Status:** Draft

The Grimoire Book Representation (GBR) Protocol is a formal standard for representing narrative fiction in a structured, machine-readable format.

---

## Key Principles

- **Lossless Round-Trip** — `parse(render(semantic_structure)) == semantic_structure`
- **Typed Enumerations** — no free text where structure is possible; all categorical fields use closed vocabularies
- **Registry-First** — every entity reference resolves to a declared, named entity
- **Scene as Atomic Unit** — the Scene Card is the fundamental representation unit

---

## Repository Structure

```
gbr-protocol/
  SPECIFICATION.md          # Canonical normative specification
  VERSIONING.md             # Versioning policy
  CHANGELOG.md              # Release history
  protocol/                 # Per-section expansion of the spec
  schemas/                  # JSON Schema (auto-generated from Rust)
  enums/                    # Documented enumeration vocabularies
  examples/                 # Structured example corpus
  conformance/              # Validation test suite (valid/ and invalid/)
  reference/                # Reference implementations (Rust, Python)
  template-schemas/         # Grimoire template extraction schemas
  docs/                     # Design principles and Architecture Decision Records
```

---

## Specification

The canonical normative document is [SPECIFICATION.md](SPECIFICATION.md).

It answers:
1. What is GBR?
2. How is GBR structured?
3. What files and fields are valid?
4. How is validity tested?
5. What does compliance mean?
6. How does versioning work?

---

## Schemas

Protocol JSON Schemas are in [`schemas/`](schemas/). There are two layers:

| Layer | Location | Source |
|-------|----------|--------|
| **Document schemas** | `schemas/` | Hand-crafted; define the full GBR document format accepted by validators and CI |
| **Struct schemas** | `schemas/generated/` | Machine-generated from Rust types via `grimoire-export-schemas`; regenerate with `cargo run --bin grimoire-export-schemas` |

Document schemas currently in `schemas/`:

| Schema | Document Type |
|--------|---------------|
| `schemas/registry.schema.json` | Entity Registry |
| `schemas/scene-card.schema.json` | Scene Card |
| `schemas/story-architecture.schema.json` | Story Architecture |
| `schemas/character-state.schema.json` | Character Scene State |
| `schemas/enums.schema.json` | Enumeration vocabularies |

> **Note:** The document schemas in `schemas/` currently reflect an earlier (v3) format of the Grimoire training data layout. They will be updated to match the GBR 0.1.0 document format (as used by `examples/small-story/threshold/` and `conformance/`) in a forthcoming schema migration. Until then, Level 1 schema validation against the document schemas will report expected mismatches for GBR 0.1.0-format documents.

---

## Enumerations

Documented enum definitions are in [`enums/`](enums/). Each file covers a domain and includes a `definition` and `example_usage` for every value.

---

## Examples

Structured example documents are in [`examples/`](examples/):

- `examples/minimal/` — smallest valid GBR documents
- `examples/small-story/` — multi-scene story examples
- `examples/edge-cases/` — nonlinear narrative, unreliable narrator, etc.

---

## Conformance Tests

The [`conformance/`](conformance/) directory contains:

- `conformance/valid/` — documents that MUST pass all validation
- `conformance/invalid/` — documents with known defects; each paired with an `.expected.json`

---

## Reference Implementations

- **Rust:** [`reference/rust/`](reference/rust/) — `gbr-types` crate; typed structs, enum definitions, schema generation
- **Python:** [`reference/python/`](reference/python/) — schema and referential conformance validator

```bash
# Validate a registry
jsonschema -i book/registry.json schemas/registry.schema.json

# Validate a scene card
jsonschema -i book/scenes/ch01_s01.json schemas/scene-card.schema.json
```

---

## Versioning

See [VERSIONING.md](VERSIONING.md). Current version: **GBR 0.1.0**.

---

## License

MIT — see [LICENSE](LICENSE)
