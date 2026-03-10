# Experiments

Empirical annotation studies and corpus analysis.

## Status

Placeholder. Content will be populated as annotation work begins.

## Planned Experiments

- **Frequency–Duration correlation study** — Does `frequency: iterative` reliably co-occur with `duration_mode: summary`?
- **Multi-annotator round-trip study** — Encode the same chapter independently; measure divergence in `scene_turns`, `polarity`, `delta`.
- **Flat arc focalizer study** — How do encoding strategies differ when the focalizer's arc_type is `flat`?
- **Genre signal study** — Which field combinations most reliably signal `genre_type` to a classifier?

## Structure

Each experiment lives in its own subdirectory:
```
experiments/
  exp-001-frequency-duration/
    README.md      # hypothesis, method, results
    data/          # sample encoded scenes
    analysis/      # code and output
```
