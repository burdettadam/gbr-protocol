# Training

LLM fine-tuning pipelines and annotation tooling for GBR-encoded corpora.

## Status

Placeholder. Content will be populated as the training pipeline matures.

## Planned Contents

- `pipeline/` — Scene annotation → training example conversion scripts
- `formats/` — Output format specs for common fine-tuning frameworks (OpenAI, HuggingFace, Axolotl)
- `quality/` — Inter-annotator agreement tools and annotation rubrics
- `datasets/` — Pointers to released datasets (actual data lives outside this repo)

## Types

The typed data models for training are defined in `grimoire-types/src/training.rs` in the `gbr-protocol` repo. Those types are the source of truth for what a training example looks like.
