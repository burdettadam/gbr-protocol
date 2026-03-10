# grimoire-tooling

Grimoire authoring-system tooling, built on top of [gbr-types](../reference/rust/).

## What this crate is

`grimoire-tooling` provides the Grimoire writing system's authoring pipeline on top of the stable GBR Protocol types. It adds:

| Module | Contents |
|--------|----------|
| `training` | `SceneContext`, `ProsePassage`, `TrainingExample` — LLM fine-tuning data |
| `gates` | `PhaseSpec`, `GateSpec`, `GateResult` — readiness gate system |
| `dag` | `SubPhaseDAG` — Kahn topological sort for sub-phase dependencies |
| `recipe` | `StoryRecipe` — top-level pipeline output |

All `gbr-types` protocol-core modules are re-exported for convenience.

## Binaries

| Binary | Purpose |
|--------|---------|
| `grimoire-validate-catalogs` | Validate YAML catalogs against typed schemas |
| `grimoire-export-training` | Export training examples from annotated manuscripts |
| `grimoire-gate-check` | Run phase readiness gate checks |
| `grimoire-generate` | Generate story scaffolds from catalogs |
| `grimoire-ontology-drift-check` | Check for drift between code and annotation ontology |
| `grimoire-ontology-consistency-check` | Validate ontology internal consistency |
| `grimoire-export-owl` | Export ontology as OWL/RDF |

## Python bindings (optional)

Build with PyO3 enabled:

```bash
maturin develop --features python
```

This exposes key types to Python under the `grimoire_tooling` module name.

## Usage

```toml
[dependencies]
grimoire-tooling = { path = "grimoire-tooling" }
```

```rust
use grimoire_tooling::training::TrainingExample;
use grimoire_tooling::gates::GateResult;
// Protocol-core types re-exported:
use grimoire_tooling::enums::Archetype;
use grimoire_tooling::entities::Character;
```

## Backward compatibility

The `grimoire-types` crate is a thin re-export facade that maps `grimoire_types::` to this crate and `gbr-types`. Existing code using `grimoire_types::` continues to compile without changes.
