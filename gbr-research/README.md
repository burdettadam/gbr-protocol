# gbr-research

Research and theory layer for the [GBR Protocol](https://github.com/your-org/gbr-protocol).

## What is this?

`gbr-research` contains work that is **related to but not part of** the normative GBR Protocol specification:

| Directory | Contents |
|-----------|----------|
| `theory/` | Literary-theoretical foundations for protocol constructs |
| `training/` | LLM fine-tuning pipelines and annotation tooling |
| `experiments/` | Annotation experiments and corpus studies |
| `papers/` | Academic writing derived from corpus analysis |

## Relationship to gbr-protocol

```
gbr-protocol/          ← normative specification (schemas, enums, conformance)
gbr-research/          ← scientific and research layer (this repo)
  theory/              ← why constructs exist (theoretical grounding)
  training/            ← how models learn from encoded corpora
  experiments/         ← empirical annotation studies
  papers/              ← academic output
```

The protocol is the **contract** — stable, versioned, governed.  
The research layer is the **laboratory** — exploratory, iterative, unpublished.

## Theory

`theory/THEORY.md` maps every GBR Protocol construct to its scholarly source (Genette, Cohn, Truby, Greimas, Mulvey, Searle, etc.). It is the primary document for understanding *why* a field exists, what tradition it derives from, and what adjacent concepts were considered and rejected.

## Training

`training/` will contain:
- Typed scene annotation pipelines (driven by `grimoire-types`' training.rs types)
- Fine-tuning dataset generation scripts
- Annotation quality metrics and inter-annotator agreement tools

## Experiments

`experiments/` will contain:
- Corpus annotation studies (e.g., "how often is `frequency: iterative` combined with `duration_mode: summary`?")
- Comparative encoding studies (same novel encoded by multiple annotators)
- Round-trip fidelity experiments (prose reconstruction from canonical summary)

## Papers

`papers/` will contain academic writing derived from corpus and experimental work.

## Contributing

Research contributions do not require the same governance process as protocol changes. Open a PR with your work in the appropriate subdirectory. For work that *implies a protocol change* (a new field, a new enum value), open an issue in `gbr-protocol` first.
