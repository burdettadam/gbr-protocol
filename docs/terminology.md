# CAP Narrative Profile Protocol Terminology

This document provides formal definitions for all terms used in the CAP Narrative Profile Protocol specification. In any dispute over meaning, these definitions govern.

---

## A

**Analepsis**
A temporal shift in narration to events that occurred before the current story-time point; a flashback (Genette). See `narrative_order`.

**Antagonist**
The force opposing the protagonist's central want. May be a person, institution, society, nature, or the protagonist themselves. See `antagonist_type`.

**Archetype**
A psychological template characterizing a character's fundamental role and pattern (Campbell/Vogler). Distinct from `role`, which is functional. See `enums/character.json`.

**Atmosphere**
The dominant affective quality of a setting — the felt mood of the physical space. Distinct from `tone`, which is the narrator's attitude toward the subject. See `setting_type`.

---

## B

**Beat**
The macro structural position of a scene in the narrative arc (e.g., `inciting_incident`, `midpoint`, `climax`). A beat is different from a `scene_function`, which describes what the scene does, not where it falls. See `beat_type`.

**Beat Sequence**
The ordinal position of the scene within its chapter (integer ≥ 1). Distinct from chapter numbering.

**Book Corpus**
A complete CAP Narrative Profile dataset for a single narrative work. Layout: `{book_id}/registry.json`, `{book_id}/story_architecture.json`, `{book_id}/scenes/{scene_id}.json`. See `protocol/architecture.md`.

---

## C

**Craft Targets**
The epistemic section of a Scene Card containing prescriptive authorial intent — target tension, target pacing, tone. These are neither observations of the artifact, structural positions, nor inferences — they are desired effects. Only Scene Cards have a `craft_targets` section. See ADR-006.

**Canonical Metrics**
The interpretive measurements extracted from a scene's canonical summary — `iceberg_proportion`, `subtext_load`, and per-turn fields (`emotional_state`, `masked_emotion`, `tactic`, `significance`). Stored in `interpretations.canonical_metrics`, not inside the structural `canonical_summary` object. See ADR-006.

**Canonical Summary**
The structured, machine-readable scene summary embedded in every Scene Card. Implements the Round-Trip Requirement: a comprehending reader with only the Canonical Summary can reconstruct the scene's narrative content. Consists of: focalizer, want, scene_turns array, outcome, delta. See `protocol/canonical-summary.md`.

**Causal Role**
A scene's functional position in the chain of causes and effects: `setup`, `payoff`, `complication`, `resolution`, `standalone`, `simultaneous`. See `scene_structure.json`.

**Character Scene State**
The snapshot of one character's emotional, epistemic, and action state at or during a specific scene. Embedded in a Scene Card's `character_states` array. See `protocol/character-state.md`.

**Collision Architecture**
The structural pattern of the story's central conflict: the type of opposition (`collision_type`), the inciting incident, and the antagonist's properties. Stored in Story Architecture. See `protocol/story-architecture.md`.

**Conformance Level**
The tier of protocol compliance a system claims:
- **Level 1 (Schema Conformant):** Documents are valid against `schemas/*.schema.json`.
- **Level 2 (Semantically Conformant):** All entity references resolve; enum values are semantically appropriate; Scene Turn Requirement met.
- **Level 3 (Round-Trip Conformant):** Canonical Summaries enable full reconstruction of scene narrative content.

**Consciousness Mode**
Dorrit Cohn's taxonomy for how prose represents mental life: `psychonarration`, `quoted_monologue`, `narrated_monologue`, `mixed`. See `narrative_voice.json`.

---

## D

**Delta**
The final field in a Canonical Summary. A single sentence describing what changed — the net narrative or character effect of the scene. Required. Must state a change, not a condition.

**Diegetic Level**
Genette's three-level hierarchy of narration: `extradiegetic` (outside the story), `intradiegetic` (within the story), `metadiegetic` (a story-within-a-story). See `narrative_voice.json`.

**Drive Model**
The primary motivational framework characterizing a character's source of agency: `wound`, `desire`, `duty`, `perception`, `existential`. See `character.json`.

**Duration Mode**
The relationship between story time and discourse time in a scene: `scene`, `summary`, `ellipsis`, `pause`, `stretch` (Genette). See `narrative_time.json`.

---

## E

**Entity Registry**
The master reference document for all named entities in a book corpus. Every character, setting, and relationship must be declared here before being referenced in other CAP Narrative Profile documents. Required. See `protocol/entity-registry.md`.

**Entity Reference**
A string value in a CAP Narrative Profile document pointing to a key in the Entity Registry. Must resolve to a defined entity. Unresolved references are a Level 2 conformance failure.

---

## F

**Fabula**
The chronological story events in the order they actually occurred (Tomashevsky/Chatman). Distinct from `syuzhet`. The distinction between fabula and syuzhet is the foundational organizing principle of the CAP Narrative Profile Protocol's temporal encoding.

**Focalization**
Genette's concept: the perceptual filter through which events are presented. "Who perceives" (focalization) is distinct from "who speaks" (narration). The `focalizer` field on a Scene Card is the character whose perception filters the scene's events. See `focalization_type`.

**Focalizer**
The character whose perspective filters the narrated events in a given scene. Required on every Scene Card. Must resolve to a key in the Entity Registry.

**Free Indirect Discourse (FID)**
See `narrated_monologue`. The dominant consciousness mode in literary fiction; character thought rendered in third person but in the character's diction and perspective.

**Frequency**
The ratio of how many times an event occurs vs. how many times it is narrated: `singulative`, `iterative`, `repetitive`, `multiple_singulative` (Genette). See `narrative_time.json`.

---

## G

**CAP Narrative Profile**
Grimoire Book Representation. The protocol name.

**Genre**
The top-level narrative frame classification for the book (`genre_type`). Required on Story Architecture. Determines reader expectations and genre contract.

---

## I

**Iceberg Proportion**
A float (0.0–1.0) encoding how much of the scene's meaning is withheld from explicit narration (Hemingway's iceberg principle). 0.0 = fully explicit; 1.0 = fully implicit. Optional. Stored in `interpretations.canonical_metrics`.

**Interpretation**
The epistemic section of a CAP Narrative Profile document containing inferred meaning layered on top of observables and structure: motivations, emotional states, themes, subtext, literary-theoretical classifications. Interpretation fields MAY carry an `interpreted_value` wrapper for confidence and provenance. See ADR-006.

**Interpreted Value**
The optional metadata wrapper for interpretation fields. Either a plain value (e.g., `"humiliation"`) or a structured object: `{ "value": "humiliation", "confidence": 0.85, "source": "model" }`. The `source` enum is: `human`, `model`, `inferred`, `consensus`. Observable fields MUST NOT use this wrapper.

---

## K

**Kernel**
A narratively essential event — one whose removal would destroy the causal chain (Chatman). Kernels are the minimum content that must be represented to preserve the narrative.

**Knowledge Object**
A structured assertion encoding what a character knows or believes at scene time. Fields: `fact`, `is_true` (whether true in the story world), `source`, `confidence`. See `protocol/character-state.md`.

---

## M

**Masked Emotion**
The emotion a character displays or performs in the scene, which differs from their `primary_emotion` (what they actually feel). Captures subtext and suppression.

**Metadiegetic**
A story-within-a-story narration level; narration embedded inside the intradiegetic level (Genette).

**Motif**
A recurring element — image, object, phrase, or situation — that accumulates meaning through repetition across the narrative.

---

## N

**Narrative Order**
The temporal relationship between the order events occurred (fabula) and the order they are narrated (syuzhet): `chronological`, `analepsis`, `prolepsis`, `braided`, `in_medias_res`. See `narrative_time.json`.

**Narratee**
The fictional entity addressed by the narrator — distinct from the real reader (Prince). See `narratee_type` in `narrative_voice.json`.

**Narrator Reliability**
The degree to which the narrator's account can be trusted to align with the implied author's norms (Booth/Nünning). See `narrator_reliability_type`.

---

## O

**Observable**
The epistemic section of a CAP Narrative Profile document containing facts directly grounded in the artifact: named participants, quoted dialogue, explicit locations, visible actions, explicit objects and ordering markers. Observable fields are always certain and MUST NOT carry the `interpreted_value` metadata wrapper. See ADR-006.

**Objective Object**
A structured assertion encoding what a character wants to do (their action intention) in the scene. Fields: `action`, `target`, `obstacle`, `method`. See `protocol/character-state.md`.

**Outcome**
The result of the focalizer's scene-level want: `achieved`, `partially_achieved`, `failed`, `deferred`, `redirected`, `irrelevant`. Required in Canonical Summary. See `want_outcome`.

---

## P

**Polarity**
The direction of value-shift in a scene — whether the focalizer's circumstances move toward positive or negative (or more complex combinations). See `scene_polarity`.

**Primary Emotion**
The dominant emotion a character actually experiences (internally) during a scene. Distinguished from `masked_emotion` (the emotion they display). See `emotion` in `emotion_psychology.json`.

**Prolepsis**
A temporal shift in narration to events that will occur after the current story-time point; a flashforward (Genette).

**Protagonist Arc**
The character's arc trajectory and status stored in Story Architecture. Includes `arc_type`, `wound`, `misbelief`, `truth`, `ghost`, `want`, `need`. See `protocol/story-architecture.md`.

**Psychic Distance**
John Gardner's five-level scale (1–5) of felt closeness between reader and character consciousness. 1 = maximally distant; 5 = stream of consciousness. An integer field, not an enum. See `narrative_voice.json` for documentation.

---

## R

**Round-Trip Requirement**
The core data integrity constraint of CAP Narrative Profile: a comprehending reader presented only with the Canonical Summary MUST be able to reconstruct the scene's narrative content. The Canonical Summary is the protocol's guarantee of lossless semantic transport. See `protocol/canonical-summary.md`, SPECIFICATION.md §9.

**Relationship**
A directed edge in the Entity Registry between two characters. Has type, dynamic, and power balance. Direction matters: a relationship from A to B is not the same as B to A (power balance, dynamic may differ).

---

## S

**Satellite**
A narratively expandable but non-essential event — one whose removal does not destroy the causal chain (Chatman). Satellites provide texture, characterization, and thematic resonance.

**Scene Card**
The primary unit of scene-level CAP Narrative Profile data. Each scene card describes one scene of a narrative work, organized into four epistemic sections: `observables` (grounded facts), `structure` (organizational relationships), `interpretations` (inferred meaning), and `craft_targets` (authorial intent).

**Scene Function**
The primary dramatic purpose of a scene: `revelation`, `confrontation`, `decision`, `relationship_shift`, `world_building`, `setback`, `discovery`, `transit`. See `scene_structure.json`.

**Scene Turn**
One event in the Canonical Summary's `scene_turns` array. The minimum representation of one thing that happened. Fields: `event_type`, `agent`, `verb`, `patient`, `result`. The scene_turns array must not be empty.

**Scene Turn Requirement**
The rule that every Scene Card's `canonical_summary.scene_turns` array MUST contain at least one entry. See SPECIFICATION.md §7.4.

**Slug**
A `snake_case` unique identifier used as a key in the Entity Registry's maps. Character slugs are used everywhere a character is referenced (focalizer, character_states, etc.).

**Story Architecture**
The CAP Narrative Profile document encoding a book's macro-structural design: genre, collision architecture, protagonist arc, antagonist design, story premise. One per corpus. Organized into three epistemic sections: `observables`, `structure`, `interpretations`. See `protocol/story-architecture.md`.

**Structure (Epistemic Section)**
The epistemic section of a CAP Narrative Profile document containing how observables are organized in the canonical model: sequence, containment, adjacency, state transitions, dependency and causal links, groupings. The `canonical_summary` lives in this section. See ADR-006.

**Subtext Load**
An enum encoding how much of a scene's meaning operates below the surface: `explicit`, `moderate`, `high`, `dense`. Relates to `iceberg_proportion`.

**Syuzhet**
The order in which events are narrated — the discourse sequence, which may differ from the fabula chronology (Tomashevsky/Chatman). The Scene Card's `narrative_order` field encodes the syuzhet-fabula relationship of a scene.

---

## T

**Tactic**
The interpersonal strategy a character uses to pursue their want in the scene. Distinct from emotion (what they feel) and want (what they're after). See `tactic` in `emotion_psychology.json`.

**Tone**
The narrator's attitude toward the subject matter — distinct from atmosphere (setting quality) and voice (register). See `tone` in `narrative_voice.json`.

**Trigger Type**
The category of event or stimulus that triggers the character's significant emotional response in the scene. See `trigger_type` in `emotion_psychology.json`.

---

## V

**Verb**
In a Canonical Summary scene turn: the action word linking agent and patient. Must carry the specific narrative force of the event. Generic verbs (`does`, `goes`, `says`) are insufficient at conformance level 3.

---

## W

**Want**
The focalizer's scene-level goal — what they are actively trying to achieve in this specific scene. Distinct from `need` (what the character actually requires for growth). Appears in Character Scene State and in Canonical Summary.

**Want Vocabulary**
A book-level dictionary mapping want strings to semantic categories. Stored in Entity Registry. Optional but recommended for corpora with recurring want patterns.

**Wound**
A character's core psychological injury — the formative pain that drives their misbelief and shapes their behavior. See `wound` in `character.json`.
