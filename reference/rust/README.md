# gbr-types

Protocol-core Rust types for the [Grimoire Book Representation (GBR) Protocol](../../SPECIFICATION.md).

## What this crate is

`gbr-types` is the **Rust reference implementation** of the GBR Protocol's type system. It provides stable, schema-level types that correspond directly to the constructs defined in the GBR specification.

| Module | Contents |
|--------|----------|
| `enums` | All closed enumerations: archetype, wound, POV, focalization, emotion, … |
| `entities` | Core declared entities: Character, Setting, Beat, Scene, Chapter, … |
| `catalogs` | YAML catalog entry shapes |
| `ontology` | Canonical tag-key ontology and canonicalization |
| `tags` | Typed annotation system (`<!-- key:value -->` → `Annotation`) |
| `voice` | VoiceContract, VoiceSignature, FocalizationConfig, NarrativeVoice |
| `constraints` | Formal tag constraint graph (implies/excludes/requires/correlates) |

## Who should use this crate

- **External tools** that only need the protocol type system (parsers, validators, converters)
- **LLM fine-tuning pipelines** that need typed scene and entity representations
- **JSON Schema generation** via `generate_all_schemas()`

For the full Grimoire authoring stack (gate checks, story generation, training data pipeline), depend on `grimoire-tooling` instead — it re-exports this crate.

## Usage

```toml
[dependencies]
gbr-types = { path = "reference/rust" }  # or via crates.io when published
```

```rust
use gbr_types::enums::Archetype;
use gbr_types::entities::Character;
use gbr_types::tags::Annotation;
```

## Relationship to the JSON enums

The `enums/` directory at the repo root contains JSON representations of every enum in this crate. The Rust enums are the **authoritative source** — the JSON files are the documentation layer for non-Rust consumers.

## Schema generation

```rust
let schemas = gbr_types::generate_all_schemas();
```

This produces a JSON object with one key per type group (`"entities"`, `"voice"`, …), each value being a JSON Schema Draft-07 object.
