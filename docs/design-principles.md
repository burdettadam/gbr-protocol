# CAP Narrative Profile Protocol Design Principles

This document explains the reasoning behind the core design decisions of the CAP Narrative Profile Protocol. It is not normative — the SPECIFICATION.md governs. This document explains *why*.

---

## 1. The Scene as the Atomic Unit

**The Protocol is organized around scenes, not chapters or acts.**

A scene is the smallest unit of dramatic action with a self-contained shape: an observation base (who perceives), a want (what the focalizer is trying to achieve), at least one event, and an outcome. Every higher-level structure — chapters, acts, the whole book — is built from scenes.

This decision makes the protocol granular enough to be meaningful at the level where most narrative craft decisions are made, while remaining coarse enough to be tractable. Scene granularity is also the level at which:
- Character emotion and psychology can be precisely encoded
- Narrative temporality (order, duration, frequency) operates in fine-grained ways
- Polarity shifts — the primary engine of reader engagement — occur

The alternative (chapter-level granularity) would lose most of the structural information. The alternative (paragraph or sentence granularity) would create intractable annotation burdens without proportionate gain.

See also: ADR-001.

---

## 2. The Round-Trip Contract

**A comprehending reader with only the Canonical Summary must be able to reconstruct the scene's narrative content.**

This is the protocol's data integrity principle. It means:
- The Canonical Summary is not an abstract metadata label. It is the *minimum semantic content* of the scene.
- The summary must say who tried to do what, what happened, and what changed.
- Empty scene_turns arrays are never acceptable — if nothing happened, the scene should not exist.

The round-trip contract ensures that CAP Narrative Profile corpora are not lossy encodings. A scene that cannot be summarized in round-trip-compliant form is probably not a well-constructed scene. The protocol's requirements enforce craft discipline.

See also: SPECIFICATION.md §9, ADR-002, `protocol/canonical-summary.md`.

---

## 3. Typed Enumerations Over Free Text

**All categorical fields use closed, typed enumerations.**

This is the most controversial design decision, and the most deliberate.

The temptation is to make categorical fields free text — allow `pov: "close third"` or `pov: "limited third person"` or `pov: "intimate third"` instead of requiring `pov: third_limited`. But free text:
- Cannot be validated
- Cannot be compared programmatically
- Cannot be aggregated or analyzed
- Cannot be enforced as constraints in downstream systems
- Proliferates synonyms with no principled distinction

The cost is flexibility. A field like `beat_type` has 15 values; some stories will need nuances not captured by those values. The answer is not to abandon the enum — it is to use the closest existing value, file a proposal to extend the enum, and contribute to the shared vocabulary.

Every enum extension becomes part of the shared vocabulary for all CAP Narrative Profile users. Free text, by contrast, produces silos: each annotator's vocabulary is incompatible with every other.

See also: `enums/README.md`, ADR-003.

---

## 4. Fabula / Syuzhet Separation

**Story events and their narration are encoded separately.**

The `narrative_order` field on a Scene Card encodes the relationship between fabula time (when an event occurred in the story world) and syuzhet time (when it appears in the narration). A scene coded as `analepsis` is happening before the current narrative present even though it appears after in the text.

This distinction matters for:
- Understanding temporal structure across a corpus
- Analyzing how a narrative manages information revelation
- Encoding the difference between what the reader knows and when they learn it

Tools that don't make this distinction cannot answer questions like "how much of this story is told in chronological order?" or "when does this character learn this information relative to when it happened?"

See also: ADR-005, `enums/narrative_time.json`.

---

## 5. Entity Registry as Single Source of Truth

**All named entities (characters, settings, relationships) must be declared in the Entity Registry before being referenced.**

This rule enforces referential integrity across a corpus. A scene that references `character_ref: "elizabth"` (typo) fails validation if `elizabth` is not in the registry. The registry is checked first; everything else is downstream.

The alternatives:
- Declaring entities inline in scene cards — creates duplication and inconsistency
- Free-text character names — cannot be deduplicated (is "Liz" the same as "Elizabeth"?)
- No entity layer at all — makes cross-scene character tracking impossible

The registry also provides a place to store character properties that don't belong to any specific scene: archetype, wound, alignment, arc type, drive model. These are book-level properties that should not be repeated in every scene card.

---

## 6. Lossless Semantic Transport

**CAP Narrative Profile documents must preserve all narrative meaning through serialization and deserialization.**

JSON-LD or Profile URIs are not required, but the protocol commits to a consistent JSON encoding that round-trips through any standard JSON parser without loss. Required fields are required precisely because their absence makes the document lose information.

The `canonical_summary.delta` field is perhaps the most important instance of this principle: it requires encoding not just what happened but what *changed*. A scene that ends in the same state it began is not a scene — it is a description. The delta requirement enforces this.

---

## 7. Theory-Grounded Field Definitions

**Every field has a theoretical grounding drawn from narratology, literary theory, or cognitive science.**

CAP Narrative Profile is not an ad-hoc data format. Its vocabulary draws from:
- Genette (temporal fields, diegetic levels, focalization)
- Booth (narrator reliability, implied author)
- Cohn (consciousness modes)
- Gardner (psychic distance)
- Weiland/Cron (wound/misbelief/truth arc framework)
- Vogler/Campbell (archetype)
- Plutchik/Ekman (emotion)
- Truby (opposition levels)
- McKee/Field (scene polarity, beat structure)
- Hemingway (iceberg principle)
- Searle (speech act categories)
- Mulvey (gaze)

This theoretical grounding means:
- Fields have precise, well-understood meanings
- The vocabulary can be extended using the same theoretical frameworks
- The protocol is a contribution to the scholarly understanding of narrative structure, not just a private data format

---

## 8. CAP Narrative Profile Does Not Model the Text

**CAP Narrative Profile encodes narrative structure, not the text itself.**

The protocol stores *what happens* (canonical summary), *how it is told* (voice, consciousness mode, focalization), and *what it means* (polarity, stakes, motifs). It does not store the prose text.

This is a deliberate scope decision:
- Prose text is the source, not the artifact
- The protocol's purpose is structural analysis and round-trip verification
- Text storage is the problem of document management systems

A CAP Narrative Profile corpus can be linked to source text through `source_citation` annotations in the analysis mode, but raw text is never a required or stored field.
