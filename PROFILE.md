# Narrative Profile — Skeleton

> **Profile Identifier:** `narrative`
> **Profile Version:** `0.1.0`
> **SIP Core Version:** `≥ 0.1.0`
> **Status:** Normative Draft

This document defines the **narrative** domain profile for the Semantic Interaction Protocol (SIP). It registers the types, enums, required fields, and validation rules necessary to annotate prose fiction (and, by extension, other literary works).

This skeleton declares the profile structure and type registries. A full normative profile specification will follow in a later phase.

---

## 1. Profile Declaration

A narrative artifact declares this profile in its top-level header:

```json
{
  "protocol": "semantic-interaction-protocol",
  "protocol_version": "0.1.0",
  "profile": "narrative",
  "profile_version": "0.1.0"
}
```

---

## 2. Type Registries

Per SIP §6.2, the narrative profile registers the following values for each core type field.

### 2.1 Entity Types

| Value | Description |
|-------|-------------|
| `character` | A person or anthropomorphized agent |
| `location` | A named place or setting |
| `object` | A significant physical or conceptual item |
| `group` | A collective (family, organization, faction) |
| `narrator` | A narrating voice that is not a character |

**Source enum:** `character.json → role`, `setting.json → setting_type`

### 2.2 Unit Types

| Value | Description |
|-------|-------------|
| `scene` | A continuous dramatic action in one setting |
| `beat` | A single dramatic value-change within a scene |
| `chapter_segment` | A structural division within a chapter |

### 2.3 Relationship Types

Registered from `relationship.json → relationship_type`:

| Value | Description |
|-------|-------------|
| `family_parent_child` | Parent-child bond |
| `family_sibling` | Sibling bond |
| `romantic` | Romantic relationship |
| `mentor` | Mentor-mentee relationship |
| `rival` | Rivalry / opposition |
| `ally` | Alliance / friendship |
| `employer_employee` | Employment / fealty |
| `foil` | Thematic mirror / contrast |
| *(+ remaining values from `relationship.json`)* | |

### 2.4 Interpretation Types

| Value | Source Enum | Description |
|-------|------------|-------------|
| `motivation` | — | Character's driving desire |
| `emotion` | `emotion_psychology.json → emotion` | Emotional state |
| `theme` | — | Thematic assertion |
| `conflict_role` | — | Role in central conflict |
| `archetype` | `character.json → archetype` | Jungian / structural archetype |
| `wound` | `character.json → wound` | Psychological wound |
| `arc_type` | `character.json → arc_type` | Character arc trajectory |
| `drive_model` | `character.json → drive_model` | Drive model (5 types) |
| `actant` | `character.json → actant` | Greimas actantial role |
| `irony_type` | `literary_theory.json → irony_type` | Type of irony present |
| `gaze_type` | `literary_theory.json → gaze_type` | Gaze framing |
| `speech_act_category` | `literary_theory.json → speech_act_category` | Austin/Searle classification |
| `freudian_mechanism` | `literary_theory.json → freudian_mechanism` | Psychoanalytic defense mechanism |
| `antagonist_type` | `literary_theory.json → antagonist_type` | Antagonist classification |
| `opposition_level` | `literary_theory.json → opposition_level` | Truby level of opposition |
| `transtextuality_type` | `literary_theory.json → transtextuality_type` | Genette transtextual relation |
| `stakes_domain` | `scene_structure.json → stakes_domain` | What is at risk |

### 2.5 Event Types

Registered from `scene_structure.json → event_type`:

| Value | Description |
|-------|-------------|
| `action` | Physical action |
| `dialogue` | Speech between characters |
| `internal_shift` | Internal thought or realization |
| `revelation` | Information disclosed to reader/character |
| `description` | Setting or character description |
| `flashback` | Analeptic event |
| `flash_forward` | Proleptic event |
| *(+ remaining values from `scene_structure.json`)* | |

### 2.6 State Types

| Value | Source | Description |
|-------|--------|-------------|
| `emotional` | `emotion_psychology.json → emotion` | Emotional condition |
| `relational` | — | Relationship standing |
| `psychological` | — | Internal psychological state |
| `social` | — | Social role or status |
| `physical` | — | Physical condition |

### 2.7 Causal Role Extensions

The narrative profile extends the SIP core causal roles (`setup`, `trigger`, `complication`, `resolution`) with:

| Value | Source | Description |
|-------|--------|-------------|
| `catalyst` | `scene_structure.json → causal_role` | Inciting force |
| `escalation` | `scene_structure.json → causal_role` | Raising stakes |
| `crisis` | `scene_structure.json → causal_role` | Moment of greatest tension |
| `climax` | — | Peak of dramatic action |
| `denouement` | — | Unwinding after resolution |

---

## 3. Additional Required Fields

Per SIP §6.3, the narrative profile declares these additional requirements on core objects.

### 3.1 Entity (when `entity_type: character`)

| Field Path | Requirement |
|------------|-------------|
| `structural_properties.role` | **REQUIRED** — protagonist, antagonist, supporting, etc. |

### 3.2 Unit (when `unit_type: scene`)

| Field Path | Requirement |
|------------|-------------|
| `observables.context.focalizer` | **REQUIRED** — entity_ref of the perceiving character |
| `observables.context.pov` | **REQUIRED** — point of view type |

### 3.3 Unit (all types)

| Field Path | Requirement |
|------------|-------------|
| `structure.beat` | RECOMMENDED — macro arc position |

---

## 4. Additional Epistemic Sections

### 4.1 `craft_targets`

The narrative profile defines a fourth epistemic section on Units. Craft targets are prescriptive authorial intent — what the author is aiming for. They are always intentional and do not carry the `interpreted_value` wrapper.

| Property | Description |
|----------|-------------|
| **Name** | `craft_targets` |
| **Purpose** | Prescriptive authorial intent — what the author is aiming for in this unit |
| **interpreted_value wrapper** | Not used (values are goals, not inferred claims) |
| **Validation** | `tension`, when present, MUST be a float between 0.0 and 1.0 |

**Registered craft_targets fields:**

| Field | Type | Source Enum | Description |
|-------|------|-------------|-------------|
| `tension` | float 0.0–1.0 | — | Target narrative tension level (1 = low, 5 = maximum when integer; or 0.0–1.0 normalized) |
| `pacing` | string | `scene_structure.json → pacing` | Target pacing mode for this unit |
| `tone` | string | `narrative_voice.json → tone` | Authorial/narrator attitude |

**Conversion note:** When converting from GBR v0.2.0, `craft_targets.target_tension` maps to `craft_targets.tension`, `craft_targets.target_pacing` maps to `craft_targets.pacing`, and `craft_targets.tone` stays at `craft_targets.tone`.

---

## 5. Narrative Voice Observables

The narrative profile adds these fields to `unit.observables.context`:

| Field | Type | Source Enum | Description |
|-------|------|------------|-------------|
| `focalizer` | entity_ref | — | Who perceives |
| `pov` | string | `narrative_voice.json → pov_type` | Point of view |
| `diegetic_level` | string | `narrative_voice.json → diegetic_level` | Genette diegetic level |
| `narrative_order` | string | `narrative_time.json → narrative_order` | Temporal order |
| `duration_mode` | string | `narrative_time.json → duration_mode` | Genette duration |
| `frequency` | string | `narrative_time.json → frequency` | Genette frequency |
| `setting` | entity_ref | — | Location entity |
| `time_of_day` | string | `setting.json → time_of_day` | When the unit occurs |
| `atmosphere` | string | `setting.json → atmosphere` | Environmental mood |
| `spatial_structure` | string | `setting.json → spatial_structure` | Spatial configuration |

---

## 6. Semantic Fingerprint Grammar

The narrative profile defines a machine-verifiable grammar for `unit.structure.semantic_fingerprint`. The fingerprint is a single-line string encoding the unit's structural essence in a form suitable for embedding, clustering, and retrieval.

### 6.1 Grammar (ABNF)

```
fingerprint     = agent-clause *( SP "|" SP qualifier )

agent-clause    = UPPER(agent) SP verb SP [UPPER(target)]

qualifier       = key "=" value
key             = "ROLE" / "SHIFT" / "BEAT" / "POV" / "TONE" / "ARC"

; ROLE    — causal_role from SIP §5.3
; SHIFT   — before_state "→" after_state  (abbreviated state type values)
; BEAT    — narrative macro-arc position (see §2 beat enum)
; POV     — point-of-view type
; TONE    — authorial tone (from narrative_voice.json)
; ARC     — character arc moment (e.g., "crack", "pivot", "break")

UPPER(x)        = entity-slug rendered in ALL_CAPS
verb            = action string from the unit's kernel step (present tense, snake_case)
value           = 1*( ALPHA / DIGIT / "-" / "→" / "_" )
```

### 6.2 Rendering Rules

1. **`UPPER(agent)`**: the `entity_id` of the agent who performs the kernel step, uppercased.
2. **`verb`**: the `action` field of the unit's first `significance: essential` step in `structure.steps`. If multiple essential steps exist, use the first. If no steps exist, use the unit's `observables.event_type`.
3. **`UPPER(target)`**: optional. The `target` of the kernel step, uppercased if it is an entity ref. Omit if the action has no target or the target is an abstract phrase.
4. **Qualifiers** (all optional, ordered as shown):
   - `ROLE=<causal_role>` — from `unit.structure.causal_role`
   - `SHIFT=<before>→<after>` — abbreviated state values from `structure.transition.before` and `.after`
   - `BEAT=<beat>` — from `unit.structure.grouping.beat`
   - `POV=<pov>` — from `unit.observables.context.pov`
   - `TONE=<tone>` — from `unit.craft_targets.tone`
   - `ARC=<arc_moment>` — a single descriptor of where on the character arc this unit falls

### 6.3 Examples

```
NADIA arrives CHILDHOOD_HOME | ROLE=setup | SHIFT=intact→cracked | BEAT=status_quo
NADIA finds JOURNAL_OPEN_ON_DESK | ROLE=trigger | SHIFT=resolve→grief | BEAT=revelation
NADIA calls ESTATE_AGENT | ROLE=payoff | SHIFT=grief→grief | BEAT=climax | ARC=pivot
```

### 6.4 Parsing

Tools MAY parse the fingerprint by splitting on ` | ` and further splitting each qualifier on `=`. The agent-clause is always the first token before the first `|`. The `SHIFT` value is split on `→`.

### 6.5 Validation

- A fingerprint MUST contain exactly one agent-clause.
- All qualifier keys MUST come from the registered set in §6.1.
- The agent (pre-UPPER) SHOULD resolve to a declared entity_id.
- The `SHIFT` before and after values SHOULD be non-empty strings.

---

## 7. GBR v0.2.0 → SIP Narrative Migration Guide

The following table maps GBR v0.2.0 scene-card fields to their SIP narrative profile equivalents. This is the authoritative mapping used by `grimoire-sip-convert`.

### 7.1 Artifact-level fields

| GBR v0.2.0 field | SIP narrative field | Notes |
|------------------|---------------------|-------|
| `scene_id` | `artifact_id` | The SIP artifact wraps one scene |
| `book_id` | `metadata.book_id` | Preserved in free-form metadata |
| `chapter` | `metadata.chapter` | |
| *(implied)* | `protocol` = `"semantic-interaction-protocol"` | Added by converter |
| *(implied)* | `profile` = `"narrative"` | Added by converter |

### 7.2 Entity construction

GBR scene cards do not declare entities; they reference them from a registry. The converter SHOULD:

1. Create a `character` entity for every slug in `observables.participants` and `character_states[*].observables.character`, merging duplicates.
2. Create a `location` entity for `observables.setting_instance.setting`.
3. If a registry JSON is available, populate `display_name`, `structural_properties.role`, and `interpretations.*` from the registry's `characters` and `settings` maps.
4. If no registry is available, leave fields other than `entity_id` and `entity_type` empty.

### 7.3 Unit observables

| GBR v0.2.0 field | SIP narrative field |
|------------------|---------------------|
| `observables.focalizer` | `units[0].observables.context.focalizer` |
| `observables.participants` | `units[0].observables.participants` |
| `observables.diegetic_level` | `units[0].observables.context.diegetic_level` |
| `observables.setting_instance.setting` | `units[0].observables.context.setting` |
| `observables.setting_instance.time_of_day` | `units[0].observables.context.time_of_day` |
| `observables.setting_instance.atmosphere` | `units[0].observables.context.atmosphere` |
| `observables.narrative_time.order` | `units[0].observables.context.narrative_time.order` |
| `observables.narrative_time.duration_mode` | `units[0].observables.context.narrative_time.duration_mode` |
| `observables.narrative_time.frequency` | `units[0].observables.context.narrative_time.frequency` |

### 7.4 Unit structure

| GBR v0.2.0 field | SIP narrative field |
|------------------|---------------------|
| `structure.causal_role` | `units[0].structure.causal_role` |
| `structure.beat` | `units[0].structure.grouping.beat` |
| `structure.scene_function` | `units[0].structure.grouping.scene_function` |
| `structure.scene_number_in_chapter` | `units[0].structure.grouping.scene_number_in_chapter` |
| `structure.canonical_summary.delta` | `units[0].structure.transition.description` |
| `structure.canonical_summary.want` | `units[0].interpretations.canonical_summary.want` |
| `structure.canonical_summary.obstacle` | `units[0].interpretations.canonical_summary.obstacle` |
| `structure.canonical_summary.outcome` | `units[0].interpretations.canonical_summary.outcome` |
| `structure.turn.from` | `units[0].structure.transition.before.value` (with `state_type: "value_charge"`) |
| `structure.turn.to` | `units[0].structure.transition.after.value` |

### 7.5 Scene turns → Steps

Each entry in `structure.canonical_summary.scene_turns[]` maps to one entry in `units[0].structure.steps[]`:

| GBR scene_turn field | SIP step field |
|----------------------|----------------|
| `observables.turn_number` | `sequence_number` |
| `observables.active_character` | `agent` |
| `observables.verb` | `action` |
| `observables.target` | `target` |
| `observables.event_type` | `event_type` |
| `observables.significance` | `significance` (`kernel` → `essential`, `satellite` → `supplementary`) |
| `interpretations.*` | `interpretations` (preserved as-is) |

**Significance mapping:** GBR uses `kernel` / `satellite`; SIP uses `essential` / `supplementary`. The converter MUST translate these values.

### 7.6 Character states → Participant states

Each entry in `character_states[]` maps to one entry in `units[0].participant_states[]`:

| GBR character_state field | SIP participant_state field |
|---------------------------|----------------------------|
| `observables.character` | `entity_ref` |
| `observables.pov_role` | `role_in_unit` |
| `structure.objective` | `objective` |
| `structure.tactic` | `observables.tactic` |
| `structure.knowledge_at_entry[]` | `information_state.knows[]` (predicate: `"knows_that"`) |
| `structure.knowledge_gaps[]` | `information_state.gaps[]` (predicate: `"does_not_know"`) |
| `interpretations.emotion` | `pre_state.value` (state_type: `"emotional"`) |
| `interpretations.arc_type` | `interpretations.arc_type` |
| `interpretations.drive_model` | `interpretations.drive_model` |
| `interpretations.masked_emotion` | `interpretations.masked_emotion` |

### 7.7 Top-level interpretations → Unit interpretations

| GBR v0.2.0 field | SIP narrative field |
|------------------|---------------------|
| `interpretations.pov` | `units[0].interpretations.pov` |
| `interpretations.focalization` | `units[0].interpretations.focalization` |
| `interpretations.consciousness_mode` | `units[0].interpretations.consciousness_mode` |
| `interpretations.psychic_distance` | `units[0].interpretations.psychic_distance` |
| `interpretations.narrator_reliability` | `units[0].interpretations.narrator_reliability` |
| `interpretations.stakes_domain` | `units[0].interpretations.stakes_domain` |
| `interpretations.canonical_metrics` | `units[0].interpretations.canonical_metrics` |
| `craft_targets.tone` | `units[0].craft_targets.tone` |
| `craft_targets.target_tension` | `units[0].craft_targets.tension` |
| `craft_targets.target_pacing` | `units[0].craft_targets.pacing` |
| `motif_tags` | `units[0].interpretations.motif_tags` |
| `theory_notes` | `units[0].interpretations.theory_notes` |

---

## 8. Validation Rules

The narrative profile adds the following rules on top of the SIP base conformance levels.

### 8.1 Level 1 additions (Schema)

These rules are enforced by the profile JSON Schema extension (to be added in a future phase as `docs/sip/profiles/narrative/schemas/`). Until then they are normative prose:

| Object | Rule |
|--------|------|
| Unit (scene) | `observables.context.focalizer` MUST be present and non-empty |
| Unit (scene) | `observables.context.pov` MUST be present and non-empty |
| Step | `agent` MUST be a non-empty string |

### 8.2 Level 2 additions (Referential)

| Check | Description |
|-------|-------------|
| `context.focalizer` | MUST resolve to a declared `entity_id` of `entity_type: character` |
| `context.setting` | MUST resolve to a declared `entity_id` of `entity_type: location` |
| `step.agent` | SHOULD resolve to a declared `entity_id`; warning if it does not |

### 8.3 Level 3 additions (Semantic)

| Check | Description |
|-------|-------------|
| Essential step count | Each unit SHOULD have at least one step with `significance: essential` |
| Transition consistency | `transition.before` and `transition.after` SHOULD differ in value |
| Fingerprint agent | The agent extracted from `semantic_fingerprint` SHOULD match at least one step agent |

### 8.4 Significance mapping contract

When converting from GBR v0.2.0, the following significance values MUST be translated:

| GBR v0.2.0 | SIP narrative |
|------------|--------------|
| `kernel` | `essential` |
| `satellite` | `supplementary` |

---

## 9. Changelog

| Version | Date | Changes |
|---------|------|---------|
| 0.1.0 | 2026-03 | Initial normative draft: type registries, field vocabulary, semantic fingerprint grammar (§6), GBR v0.2.0 migration guide (§7), validation rules (§8) |

Placeholder format:

```
AGENT(entity_ref) ACTION TARGET | ROLE=causal_role | SHIFT=before_emotion→after_emotion
```

---

## 7. Canonical Views

Per SIP §6.7, the narrative profile declares these canonical view types:

| View Type | Description |
|-----------|-------------|
| `entity_trajectory` | Character arc: one entity's state across all units |
| `chronological` | Units in story-time order (may differ from discourse order) |
| `causal_chain` | Units connected by causal_role progression |
| `tension_curve` | craft_targets.tension plotted across unit sequence |

---

## 8. Enum Governance

All narrative enums are versioned independently from SIP core, per SIP §6.5.

Current enum files and their version:

| File | Version | Value Count |
|------|---------|-------------|
| `character.json` | 0.2.0 | 88 |
| `emotion_psychology.json` | 0.2.0 | 71 |
| `scene_structure.json` | 0.2.0 | 78 |
| `narrative_voice.json` | 0.2.0 | 46 |
| `narrative_time.json` | 0.2.0 | 14 |
| `literary_theory.json` | 0.2.0 | 75 |
| `setting.json` | 0.2.0 | 50 |
| `relationship.json` | 0.2.0 | 35 |

Adding a value → minor bump. Removing/renaming → major bump.

---

## 9. Domain-Specific Validation Rules

### Level 1 (Schema)
- Profile schemas extend core schemas with narrative-specific constraints.

### Level 2 (Referential)
- Every `focalizer` ref MUST resolve to an entity with `entity_type: character`.
- Every `setting` ref MUST resolve to an entity with `entity_type: location`.
- Every `participants[]` entry MUST have a corresponding `participant_states[]` entry when participant_states is present.

### Level 3 (Round-Trip)
- Semantic fingerprint round-trip invariant MUST hold.
- `craft_targets.tension` values MUST be reproducible from the canonical data.

---

## Appendix A: Mapping from GBR v0.2.0

This profile is the direct successor to GBR v0.2.0's narrative-specific fields. The mapping table in SIP SPECIFICATION.md Appendix C documents the correspondence.

Key transformations:
- `story_architecture.json` → distributes across `Artifact.metadata`, profile-level `Artifact.structure`, and `Artifact.interpretations`
- `scene_card.json` → becomes a `Unit` with `unit_type: scene`
- Character-level JSON → becomes `Entity` with `entity_type: character` + profile extensions
- All enums remain in the profile; none migrate to SIP core

---

## Appendix B: Future Work

1. **Full normative profile specification** — field-level schemas, complete enum registries, validation rule formalization.
2. **Semantic fingerprint grammar** — formal BNF/PEG grammar with reference implementation.
3. **Profile JSON Schemas** — overlay schemas that extend core schemas with narrative constraints.
4. **Migration tooling** — GBR v0.2.0 → SIP narrative profile adapter.
