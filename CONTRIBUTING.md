# Contributing to GBR Protocol

The GBR (Grimoire Book Representation) Protocol is an open engineering standard. Contributions are welcome in four categories: enum proposals, specification clarifications, schema improvements, and conformance test cases.

---

## What Belongs Here

This repository contains:
- The normative specification (`SPECIFICATION.md`)
- Protocol JSON Schemas (`schemas/`)
- Enumeration vocabularies (`enums/`)
- Conformance tests (`conformance/`)
- Protocol documentation (`protocol/`, `docs/`)
- Examples (`examples/`)

**What does not belong here:**
- Grimoire writing workflow templates → `grimoire` repository
- Literary theory research → `gbr-research` repository
- LLM training pipelines → `gbr-research` repository
- Author-facing tooling → `grimoire` repository

---

## Contribution Types

### 1. Propose a New Enum Value

Open a GitHub issue with the label `enum-proposal`. Include:

```
Value: snake_case_value
Domain: (which enums/*.json file)
Definition: One sentence defining this value precisely.
Example usage: When or how this value is applied.
Justification: Why no existing value in the domain is sufficient.
```

Accepted proposals are implemented in `reference/rust/src/enums.rs`, documented in the JSON file, and formalized in `schemas/enums.schema.json`. This triggers a MINOR version increment.

Removing an enum value is a MAJOR version change and requires a deprecation period (see `VERSIONING.md`).

### 2. Propose a Specification Clarification

Open a GitHub issue with the label `spec-clarification`. Include:
- The section of `SPECIFICATION.md` that is unclear
- The ambiguity or gap you have identified
- Your proposed clarification or additional normative text
- Any existing GBR documents that are affected

Clarifications that don't change validator behavior are PATCH increments. Clarifications that change what valid/invalid means are MINOR or MAJOR.

### 3. Propose a Schema Change

Open a GitHub issue with the label `schema-change`. Include:
- The schema file (`schemas/*.schema.json`) to be modified
- The proposed change (add field, change type, add constraint)
- Whether the change is breaking (MAJOR) or additive (MINOR)
- Example valid documents under the new schema

Schema changes must be reflected in: the JSON Schema file, the Rust source (`reference/rust/src/`), the conformance tests, and the relevant protocol docs.

### 4. Add a Conformance Test

Open a pull request directly. Conformance tests live in `conformance/valid/` or `conformance/invalid/`. Each invalid test requires a paired `.expected.json` sidecar with fields:

```json
{
  "error_type": "missing_required_field | invalid_enum_value | unresolved_entity_reference | empty_scene_turns | ...",
  "field": "the.field.path",
  "document_type": "scene_card | registry | story_architecture | character_state",
  "conformance_level": 1,
  "message": "Human-readable explanation.",
  "spec_reference": "SPECIFICATION.md §N.M"
}
```

---

## Pull Request Process

1. Fork the repository
2. Create a branch named `type/short-description` (e.g., `enum/add-ambivalent-tone`, `spec/clarify-round-trip`)
3. Make changes
4. Update `CHANGELOG.md` under `[Unreleased]`
5. Ensure `conformance/valid/` tests pass against any schema you have changed
6. Open a pull request with a description that includes: what changed, why, and which conformance level is affected

---

## Decision Records

Significant design decisions are recorded as Architecture Decision Records in `docs/decisions/`. If your proposal would create a new design precedent or reverse an existing ADR, you should propose a new ADR as part of your contribution. Use the existing ADR files as templates.

---

## Style

- Enum values: `snake_case`
- JSON: 2-space indent, no trailing commas
- Markdown: ATX headers (`##`), no HTML, no emoji in specification docs
- Rust: standard `rustfmt` formatting

---

## Questions

Open a GitHub Discussion for questions about the protocol's design intent or how to encode a specific narrative scenario.

