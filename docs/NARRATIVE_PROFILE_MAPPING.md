# Narrative Profile Mapping — CAP Narrative Profile v0.2.0 → CAP Core + Narrative Profile

**Status:** Draft  
**Date:** 2026-03-10  
**Purpose:** Show exactly how every current CAP Narrative Profile concept maps into the new layered architecture (Canonical Artifact Protocol core + narrative domain profile).

---

## 1. Document Type Mapping

| CAP Narrative Profile v0.2.0 Document | CAP Core Object | Narrative Profile Role | Notes |
|---------------------|-----------------|----------------------|-------|
| Entity Registry (`registry.json`) | `Artifact.entities` + `Artifact.relationships` | Entity declarations with narrative-specific type registries | Characters → entities of type `character`; settings → entities of type `location` |
| Story Architecture (`story_architecture.json`) | `Artifact.metadata` + `Artifact.views` + `Artifact.interpretations` | Narrative-specific artifact metadata, structural views, and interpretive overlays | Decomposes into three concerns; no single equivalent |
| Scene Card (`ch{NN}_s{NN}.json`) | `Unit` | Unit of type `scene` with narrative-specific observables, structure, and interpretations | 1:1 mapping; scene = unit |
| Character Scene State (embedded) | `ParticipantState` | Per-entity state within a unit, using narrative-specific state schema | Embedded within Unit, not separate |
| Canonical Summary | `Unit.structure.semantic_fingerprint` | Narrative-specific grammar for rendering/parsing | Concept is core; grammar is profile |
| Conformance Levels (1/2/3) | Conformance Levels (1/2/3) | Same contract, profile adds domain-specific rules at each level | — |

---

## 2. Term Translation Table

### 2.1 Core Concepts (terms that become core protocol objects)

| CAP Narrative Profile v0.2.0 Term | CAP Core Term | Type | Notes |
|-----------------|--------------|------|-------|
| Book / Corpus | **Artifact** | object | Top-level container |
| `book_id` | `artifact_id` | field | — |
| Entity Registry | `Artifact.entities` | field | — |
| Scene Card | **Unit** | object | Atomic transformation block |
| `scene_id` | `unit_id` | field | — |
| Character Scene State | **ParticipantState** | object | Per-entity state within unit |
| Relationship (edge) | **Relationship** | object | Directed link |
| `source` / `target` | `source` / `target` | field | — |
| Delta ("what changed") | **Transition.description** | field | Core concept |
| Canonical Summary | `semantic_fingerprint` | field | Concept core; grammar profile |
| Scene Turn | **Step** | object | Sub-unit decomposition |
| `turn_number` | `sequence_number` | field | — |
| `active_character` | `agent` | field | — |
| `verb` | `action` | field | — |
| `target` (turn) | `target` | field | — |
| `kernel` / `satellite` | `essential` / `supplementary` | value | Generalized significance |
| Observables section | Observables section | section | Same concept |
| Structure section | Structure section | section | Same concept |
| Interpretations section | Interpretations section | section | Same concept |
| `interpreted_value` wrapper | `interpreted_value` wrapper | pattern | `{value, confidence, source}` |
| `prose` | `source_text` | field | Raw artifact text |
| `word_count` | `metadata.size` | field | — |
| `participants` | `unit.observables.participants` | field | Entity references in unit |

### 2.2 Narrative Profile Terms (terms that become profile-specific)

| CAP Narrative Profile v0.2.0 Term | Narrative Profile Location | CAP Core Anchor | Notes |
|-----------------|---------------------------|-----------------|-------|
| Character | Entity (type: `character`) | `Entity.entity_type` | — |
| Setting | Entity (type: `location`) | `Entity.entity_type` | — |
| Narrator | Entity (type: `narrator`) or profile-level config | — | Voice definition |
| Scene | Unit (type: `scene`) | `Unit.unit_type` | — |
| Chapter | `Unit.structure.grouping.chapter` | — | Narrative ordering unit |
| Act | `Unit.structure.grouping.act` | — | — |
| Beat | `Unit.structure.beat` (profile field) | — | Hero's Journey / Snyder |
| Arc | View (type: `entity_trajectory`) | `View.view_type` | — |
| Arc Position | `Unit.structure.position` + profile interpretation | `Unit.structure.position` | Float generalizes; meaning is profile |
| Focalizer | `ParticipantState.role_in_unit = "focalizer"` | `ParticipantState.role_in_unit` | Genette concept |
| POV | `Unit.interpretations.pov` | — | Genette/Stanzel |
| Focalization | `Unit.interpretations.focalization` | — | Genette |
| Psychic Distance | `Unit.interpretations.psychic_distance` | — | Gardner |
| Consciousness Mode | `Unit.interpretations.consciousness_mode` | — | Cohn |
| Narrator Reliability | `Unit.interpretations.narrator_reliability` | — | Booth/Nünning |
| Diegetic Level | `Unit.observables.diegetic_level` | — | Genette |
| Narratee Type | `Unit.observables.narratee_type` | — | Prince |
| Narrative Time (all) | `Unit.observables.narrative_time` | — | Genette temporal architecture |
| Scene Function | `Unit.structure.scene_function` | — | Seger |
| Scene Polarity / Turn | Profile extension of `Transition` | `Unit.structure.transition` | 24 polarity values are profile |
| Causal Role | Profile extension of core values | `Unit.structure.causal_role` | Core has setup/trigger/complication/resolution |
| Craft Targets | Profile's 4th epistemic section | — | `craft_targets` is narrative-specific |
| Tone | `Unit.craft_targets.tone` | — | — |
| Target Tension | `Unit.craft_targets.target_tension` | — | — |
| Target Pacing | `Unit.craft_targets.target_pacing` | — | — |
| Voice Signature | `Entity.structural_properties.voice_signature` | — | Prose voice fingerprint |
| Voice Embedding | `Entity.structural_properties.voice_embedding` | — | ML continuity tracking |
| Motif Tags | `Unit.interpretations.motif_tags` | — | — |
| Theory Notes | `Unit.interpretations.theory_notes` | — | Scholarly commentary |
| Setting Instance | `Unit.observables.context` (profile-defined) | `Unit.observables.context` | — |
| Atmosphere | `Unit.observables.context.atmosphere` | — | — |
| Genre | `Artifact.structure.genre` (profile field) | — | — |
| Collision Architecture | `Artifact.structure.collision` (profile field) | — | Grimoire Phase 02 |
| Themes | `Artifact.interpretations.themes` | — | — |
| Controlling Idea | `Artifact.interpretations.controlling_idea` | — | McKee |
| Protagonist Arc | `Artifact.interpretations.protagonist_arc` | — | — |
| Antagonist | `Artifact.interpretations.antagonist` | — | Truby |
| Transtextuality | `Artifact.interpretations.transtextuality` | — | Genette |
| Actantial Map | `Artifact.structure.actantial_map` (profile field) | — | Greimas |
| Want Vocabulary | Profile-level controlled vocabulary | — | — |

### 2.3 Character-Specific Fields (all → Narrative Profile)

| CAP Narrative Profile Field | Profile Location | Notes |
|-----------|-----------------|-------|
| `role` | `Entity.structural_properties.role` | protagonist/antagonist/etc. |
| `archetype` | `Entity.interpretations.archetype` | Campbell/Vogler |
| `wound` | `Entity.interpretations.wound` | Weiland/Cron |
| `alignment` | `Entity.interpretations.alignment` | 9-cell grid |
| `drive_model` | `Entity.interpretations.drive_model` | 5-drive model |
| `arc_type` | `Entity.interpretations.arc_type` | — |
| `actant` | `Entity.interpretations.actant` | Greimas |
| `ghost` | `Entity.interpretations.ghost` | Backstory |
| `want` | `Entity.interpretations.want` | External goal |
| `need` | `Entity.interpretations.need` | Thematic truth |
| `flaw` | `Entity.interpretations.flaw` | Wound manifestation |
| `slot` | `Entity.observable_descriptors.slot` | Cast position |

### 2.4 Character Scene State Fields (all → Narrative Profile)

| CAP Narrative Profile Field | Profile Location | Core Anchor |
|-----------|-----------------|-------------|
| `character` | `ParticipantState.entity_ref` | core |
| `pov_role` | `ParticipantState.role_in_unit` | core (role concept), profile (values) |
| `posture` | `ParticipantState.observables.posture` | — |
| `body_language` | `ParticipantState.observables.body_language` | — |
| `fid_markers` | `ParticipantState.observables.fid_markers` | — |
| `objective` | `ParticipantState.objective` | core (concept), profile (object_type, constraint) |
| `tactic` | `ParticipantState.structure.tactic` | — |
| `obstacle` | `ParticipantState.obstacle` | core (concept), profile (type enum) |
| `trigger_type` | `ParticipantState.structure.trigger_type` | — |
| `want_outcome` | `ParticipantState.structure.want_outcome` | — |
| `arc_beat` | `ParticipantState.structure.arc_beat` | — |
| `arc_direction` | `ParticipantState.structure.arc_direction` | — |
| `wound_triggered` | `ParticipantState.structure.wound_triggered` | — |
| `knowledge_*` | `ParticipantState.information_state` | core (concept), profile (domains, predicates) |
| `emotional_state_*` | `ParticipantState.interpretations.emotional_state_*` | — |
| `emotional_arc` | `ParticipantState.interpretations.emotional_arc` | — |
| `emotion` / `masked_emotion` | `ParticipantState.interpretations.emotion` / `masked_emotion` | — |
| `psychic_distance` | `ParticipantState.interpretations.psychic_distance` | — |
| `consciousness_mode` | `ParticipantState.interpretations.consciousness_mode` | — |
| `social_mask` / `social_role` | `ParticipantState.interpretations.social_*` | — |
| `stakes` | `ParticipantState.interpretations.stakes` | — |

---

## 3. Enum Migration

### 3.1 Enums That Stay in Narrative Profile (all 7 current enum files)

| Enum File | Enum Name | Value Count | Profile Location |
|-----------|-----------|-------------|-----------------|
| `character.json` | `archetype` | 21 | Entity interpretation |
| | `wound` | 23 | Entity interpretation |
| | `alignment` | 9 | Entity interpretation |
| | `role` | 17 | Entity structure |
| | `drive_model` | 5 | Entity interpretation |
| | `arc_type` | 7 | Entity/Unit interpretation |
| | `actant` | 6 | Entity/Unit interpretation |
| `emotion_psychology.json` | `emotion` | 30 | ParticipantState interpretation |
| | `tactic` | 20 | ParticipantState structure |
| | `trigger_type` | 12 | ParticipantState structure |
| | `emotional_arc_type` | 9 | ParticipantState interpretation |
| `scene_structure.json` | `beat_type` | 15 | Unit structure |
| | `scene_function` | 8 | Unit structure |
| | `scene_polarity` | 24 | Unit transition |
| | `event_type` | 11 | Unit/Step observables |
| | `want_outcome` | 6 | ParticipantState structure |
| | `causal_role` | 6 | Unit structure (extends core) |
| | `stakes_domain` | 8 | Unit interpretation |
| `narrative_voice.json` | `pov_type` | 5 | Unit interpretation |
| | `focalization_type` | 5 | Unit interpretation |
| | `consciousness_mode` | 4 | Unit interpretation |
| | `diegetic_level` | 3 | Unit observables |
| | `narrator_reliability_type` | 6 | Unit interpretation |
| | `narratee_type` | 4 | Unit observables |
| | `pov_role_type` | 3 | ParticipantState structure |
| | `tone` | 16 | Unit craft_targets |
| `narrative_time.json` | `narrative_order` | 5 | Unit observables |
| | `duration_mode` | 5 | Unit observables |
| | `frequency` | 4 | Unit observables |
| `literary_theory.json` | `genre_type` | 22 | Artifact structure |
| | `collision_type` | 9 | Artifact structure |
| | `antagonist_type` | 8 | Artifact interpretation |
| | `opposition_level` | 5 | Artifact interpretation |
| | `transtextuality_type` | 5 | Artifact interpretation |
| | `irony_type` | 7 | Unit interpretation |
| | `gaze_type` | 6 | Unit interpretation |
| | `speech_act_category` | 5 | Unit interpretation |
| | `freudian_mechanism` | 8 | Unit interpretation |
| `setting.json` | `setting_type` | 11 | Entity structure |
| | `time_of_day` | 9 | Unit observables |
| | `spatial_structure` | 9 | Unit observables |
| | `atmosphere` | 14 | Unit observables |
| | `territory_type` | 7 | Unit observables |
| `relationship.json` | `relationship_type` | 18 | Relationship structure |
| | `relationship_dynamic` | 12 | Relationship interpretation |
| | `power_balance` | 5 | Relationship interpretation |

### 3.2 Core Protocol Values (new, not from CAP Narrative Profile)

| Core Field | Core Values | Profile Extension Point |
|-----------|-------------|------------------------|
| `causal_role` | `setup`, `trigger`, `complication`, `resolution` | Profiles add domain-specific values |
| `significance` | `essential`, `supplementary` | — |
| `provenance_source` | `human`, `model`, `inferred`, `consensus` | Profiles may add (e.g., `model:gpt-4`) |
| Conformance severity | `ERROR`, `WARNING`, `INFO` | — |

---

## 4. Story Architecture Decomposition

The current `story_architecture.json` does not map to a single core object. It decomposes:

| Story Architecture Section | CAP Destination | Content |
|---------------------------|-----------------|---------|
| `book_id`, `title`, `author` | `Artifact.metadata` | Identity |
| `structure.word_count` | `Artifact.metadata.size` | — |
| `structure.genre` | Profile: `Artifact.structure.genre` | — |
| `structure.act_count`, `chapter_count` | Profile: `Artifact.structure` | — |
| `structure.collision_architecture` | Profile: `Artifact.structure` | — |
| `structure.inciting_incident` | Profile: `Artifact.structure` | — |
| `structure.diegetic_level`, `has_frame_narrative` | Profile: `Artifact.structure` | — |
| `structure.actantial_map` | Profile: `Artifact.structure` | Greimas |
| `interpretations.power_asymmetry` | Profile: `Artifact.interpretations` | — |
| `interpretations.antagonist` | Profile: `Artifact.interpretations` | Truby |
| `interpretations.protagonist_arc` | Profile: `Artifact.interpretations` | — |
| `interpretations.transtextuality` | Profile: `Artifact.interpretations` | Genette |
| `interpretations.themes` | Profile: `Artifact.interpretations` | McKee |

---

## 5. What CAP Narrative Profile Loses / Gains

### Loses
- **Standalone document types**: No more separate `story_architecture.json` — its content distributes across artifact metadata, structure, and interpretations
- **Implicit narrative assumptions**: Fields like `focalizer` can no longer be required at the core level — they become profile requirements
- **Self-contained simplicity**: A CAP Narrative Profile scene card currently "just works" as a single JSON file. In CAP, it's a Unit that references an Artifact

### Gains
- **Formal separation of observation from inference**: Already started with ADR-006; now completed at the architectural level
- **Reusability**: The same engine validates narrative and software artifacts
- **Cleaner research framing**: "We built a general semantic decomposition protocol and validated it in the narrative domain" is a stronger paper than "we built a book format"
- **Profile composability**: Future domains (legal, medical, architectural) can be added without touching the core
- **Multi-view support**: Character arc views, chronological views, causal chain views become first-class objects instead of ad-hoc queries
- **Explicit provenance**: Every interpretation carries `source` and `confidence`, enabling multi-annotator studies
