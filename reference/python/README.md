# reference/python — GBR Protocol Validator

Python reference implementation of the GBR Protocol conformance validator.

## What this does

`gbr_validate.py` validates GBR JSON documents against:

1. **Level 1 — Schema validity** — the document conforms to the relevant JSON Schema in `schemas/`
2. **Level 2 — Referential integrity** — all entity references resolve to declared entities in a registry
3. **Level 3 — Semantic richness** — optional craft-level checks (non-empty delta, no generic verbs, iceberg proportion in range)

## Requirements

```bash
pip install jsonschema
```

## Usage

```bash
# Validate a single document (JSON Schema only)
python reference/python/gbr_validate.py scene-card book/ch01_s01.json

# Validate against schema + resolve entity refs using a registry
python reference/python/gbr_validate.py scene-card book/ch01_s01.json \
    --registry book/registry.json

# Full Level 3 check
python reference/python/gbr_validate.py scene-card book/ch01_s01.json \
    --registry book/registry.json --level 3

# Validate a directory of scene cards
python reference/python/gbr_validate.py scene-card book/scenes/ \
    --registry book/registry.json

# Validate a registry document
python reference/python/gbr_validate.py registry book/registry.json

# Validate a story architecture document
python reference/python/gbr_validate.py story book/story-architecture.json
```

## Output

```
GBR Conformance Validator v0.1.0
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
✓ Level 1 (schema)          — ch01_s01.json
✓ Level 2 (entity refs)     — ch01_s01.json
⚠ Level 3 (semantic)        — ch01_s01.json
  delta is empty (scene_id=ch01_s01)
  generic verb "moves" in turn 2 (scene_id=ch01_s01)

Passed: 2/3 levels
```

Exit code 0 = all requested levels pass; 1 = failures; 2 = file/schema not found.

## Document types

| Type argument | Schema used | Required fields |
|---------------|-------------|-----------------|
| `registry` | `schemas/registry.schema.json` | `book_id`, `characters`, `settings` |
| `scene-card` | `schemas/scene-card.schema.json` | `scene_id`, `book_id`, `chapter`, `pov`, `focalizer`, `canonical_summary` |
| `story` | `schemas/story-architecture.schema.json` | `book_id`, `genre` |
| `character-state` | `schemas/character-state.schema.json` | `scene_id`, `character_id` |

## Level 3 checks

Level 3 checks are advisory (non-blocking by default; use `--strict` to make them blocking):

- `delta` is non-empty and contains at least 20 characters
- `iceberg_proportion` is between 0.3 and 0.9
- No generic verbs (`moves`, `goes`, `does`, `makes`, `says`) in `scene_turns`
- `scene_turns` has at least one `kernel` significance event
- `masked_emotion` is not identical to `emotional_state` for the same turn

## Relationship to Rust reference implementation

The Rust crate (`reference/rust/`, `gbr-types`) is the authoritative type system and schema source. This Python validator uses the hand-crafted JSON Schemas in `schemas/` and is suitable for CI pipelines that do not have a Rust toolchain available.
