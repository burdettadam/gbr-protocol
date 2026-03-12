# CAP Narrative Profile Field Audit — Observable / Structural / Interpretive × Core / Narrative Profile

**Status:** Complete — all fields classified; all buckets implemented in CAP schemas, Rust types, converter, and PROFILE.md
**Date:** 2026-03-11  
**Purpose:** Classify every current CAP Narrative Profile v0.2.0 field by epistemic layer and destination (core protocol vs. narrative profile) to prepare for the extraction of a domain-agnostic `canonical-artifact-protocol`.

## Legend

**Epistemic Layer**
| Code | Meaning |
|------|---------|
| **O** | Observable — grounded directly in the artifact |
| **S** | Structural — how observables are organized |
| **I** | Interpretive — inferred meaning |
| **C** | Craft target — prescriptive authorial intent |
| **M** | Mixed — needs splitting (see Split Proposals) |

**Destination**
| Code | Meaning |
|------|---------|
| **core** | Belongs in the domain-agnostic semantic interaction protocol |
| **profile** | Belongs in the narrative domain profile |
| **split** | Concept generalizes to core; values/grammar are narrative-specific |

---

## 1. Entity Registry (`registry.json`)

### 1.1 Top-Level Fields

| # | Field Path | Type | Layer | Dest | Core Equivalent | Notes |
|---|-----------|------|-------|------|-----------------|-------|
| 1 | `book_id` | string | O | core | `artifact_id` | Identity. Domain-agnostic. |
| 2 | `title` | string | O | core | `artifact.metadata.title` | — |
| 3 | `author` | string | O | core | `artifact.metadata.author` | — |
| 4 | `characters` | map | S | profile | `artifact.entities` (typed) | "Character" is narrative vocabulary |
| 5 | `settings` | map | S | profile | `artifact.entities` (typed) | "Setting" is narrative vocabulary |
| 6 | `relationships` | array | S | split | `artifact.relationships` | Link concept is core; type enums are profile |
| 7 | `want_vocabulary` | map | S | profile | — | Narrative-specific controlled vocabulary |
| 8 | `narrator` | object | S | profile | — | Narrative-specific voice definition |

### 1.2 Character Entity Fields

| # | Field Path | Type | Layer | Dest | Core Equivalent | Notes |
|---|-----------|------|-------|------|-----------------|-------|
| 9 | `characters[].observables.id` | slug | O | core | `entity.entity_id` | — |
| 10 | `characters[].observables.name` | string | O | core | `entity.display_name` | — |
| 11 | `characters[].observables.slot` | string | O | profile | — | Cast template position; narrative-specific |
| 12 | `characters[].structure.role` | enum(17) | S | profile | `entity.entity_subtype` | Protagonist/antagonist are narrative roles |
| 13 | `characters[].structure.voice_signature` | object | S | profile | — | Prose voice fingerprint; narrative-specific |
| 14 | `characters[].structure.voice_signature.sentence_length_tendency` | enum(4) | S | profile | — | — |
| 15 | `characters[].structure.voice_signature.vocabulary_register` | enum(5) | S | profile | — | — |
| 16 | `characters[].structure.voice_signature.syntax_complexity` | enum(3) | S | profile | — | — |
| 17 | `characters[].structure.voice_signature.fid_markers` | string[] | S | profile | — | Free indirect discourse; narrative-specific |
| 18 | `characters[].structure.voice_signature.forbidden_words` | string[] | S | profile | — | — |
| 19 | `characters[].structure.voice_signature.signature_phrases` | string[] | S | profile | — | — |
| 20 | `characters[].structure.voice_embedding` | object | S | profile | — | ML embedding for voice continuity |
| 21 | `characters[].interpretations.archetype` | enum(21) | I | profile | `interpretation(type=archetype)` | Campbell/Vogler taxonomy |
| 22 | `characters[].interpretations.wound` | enum(23) | I | profile | `interpretation(type=wound)` | Weiland/Cron |
| 23 | `characters[].interpretations.alignment` | enum(9) | I | profile | `interpretation(type=alignment)` | 9-cell moral grid |
| 24 | `characters[].interpretations.drive_model` | enum(5) | I | profile | `interpretation(type=drive_model)` | Grimoire 5-drive model |
| 25 | `characters[].interpretations.arc_type` | enum(7) | I | profile | `interpretation(type=trajectory_shape)` | — |
| 26 | `characters[].interpretations.actant` | enum(6) | I | profile | `interpretation(type=actant)` | Greimas |
| 27 | `characters[].interpretations.ghost` | string | I | profile | `interpretation(type=backstory)` | Specific traumatic backstory |
| 28 | `characters[].interpretations.want` | string | I | profile | — | External conscious goal |
| 29 | `characters[].interpretations.need` | string | I | profile | — | Thematic truth needed |
| 30 | `characters[].interpretations.flaw` | string | I | profile | — | Behavioral manifestation of wound |

### 1.3 Setting Entity Fields

| # | Field Path | Type | Layer | Dest | Core Equivalent | Notes |
|---|-----------|------|-------|------|-----------------|-------|
| 31 | `settings[].observables.id` | slug | O | core | `entity.entity_id` | — |
| 32 | `settings[].observables.name` | string | O | core | `entity.display_name` | — |
| 33 | `settings[].structure.type` | enum(11) | S | profile | `entity.entity_subtype` | Setting types are narrative categories |
| 34 | `settings[].interpretations.general_vibe` | string | I | profile | `interpretation(type=atmosphere)` | — |
| 35 | `settings[].interpretations.sensory_signature` | string[3] | I | profile | — | Narrative-specific sensory detail |

### 1.4 Relationship Fields

| # | Field Path | Type | Layer | Dest | Core Equivalent | Notes |
|---|-----------|------|-------|------|-----------------|-------|
| 36 | `relationships[].observables.source` | slug | O | core | `relationship.source` | — |
| 37 | `relationships[].observables.target` | slug | O | core | `relationship.target` | — |
| 38 | `relationships[].structure.rel_type` | enum(18) | S | split | `relationship.relationship_type` | Concept core; values profile |
| 39 | `relationships[].interpretations.description` | string | I | profile | `interpretation(type=description)` | — |
| 40 | `relationships[].interpretations.dynamic_at_start` | enum(12) | I | profile | — | Narrative relational dynamics |
| 41 | `relationships[].interpretations.dynamic_at_end` | enum(12) | I | profile | — | — |
| 42 | `relationships[].interpretations.power_balance` | enum(5) | I | profile | — | — |

### 1.5 Narrator Object

| # | Field Path | Type | Layer | Dest | Core Equivalent | Notes |
|---|-----------|------|-------|------|-----------------|-------|
| 43 | `narrator.type` | enum(5) | S | profile | — | Genette/Stanzel narrator types |
| 44 | `narrator.voice_signature` | object | S | profile | — | Same as character voice_signature |
| 45 | `narrator.voice_embedding` | object | S | profile | — | — |
| 46 | `narrator.reliability` | enum(3) | I | profile | — | Booth |
| 47 | `narrator.distance` | enum(5) | I | profile | — | Gardner |

---

## 2. Story Architecture (`story_architecture.json`)

### 2.1 Top-Level Fields

| # | Field Path | Type | Layer | Dest | Core Equivalent | Notes |
|---|-----------|------|-------|------|-----------------|-------|
| 48 | `book_id` | string | O | core | `artifact_id` | — |
| 49 | `title` | string | O | core | `artifact.metadata.title` | — |
| 50 | `author` | string | O | core | `artifact.metadata.author` | — |

### 2.2 Observables

| # | Field Path | Type | Layer | Dest | Core Equivalent | Notes |
|---|-----------|------|-------|------|-----------------|-------|
| 51 | `observables.inciting_incident_chapter` | int | O | profile | — | Chapter numbering is narrative-specific |

### 2.3 Structure

| # | Field Path | Type | Layer | Dest | Core Equivalent | Notes |
|---|-----------|------|-------|------|-----------------|-------|
| 52 | `structure.genre` | enum(22)/obj | S | profile | — | Literary genre taxonomy |
| 53 | `structure.genre.primary` | enum(22) | S | profile | — | — |
| 54 | `structure.genre.secondary` | enum(22) | S | profile | — | — |
| 55 | `structure.genre.subgenre` | string | S | profile | — | — |
| 56 | `structure.collision_architecture.collision_type` | enum(9) | S | profile | — | Grimoire social circle collision |
| 57 | `structure.collision_architecture.collision_pattern` | enum | S | profile | — | — |
| 58 | `structure.inciting_incident.type` | enum | S | profile | — | Narrative inciting incident taxonomy |
| 59 | `structure.inciting_incident.description` | string | S | profile | — | — |
| 60 | `structure.act_count` | int 1–5 | S | profile | — | Act structure is narrative-specific |
| 61 | `structure.chapter_count` | int | S | profile | — | Chapter is narrative unit |
| 62 | `structure.word_count` | int | O | core | `artifact.metadata.size` | Can generalize (word count / LOC / etc.) |
| 63 | `structure.diegetic_level` | enum(3) | S | profile | — | Genette diegetic levels |
| 64 | `structure.has_frame_narrative` | bool | S | profile | — | Narrative framing device |
| 65 | `structure.actantial_map[]` | array | S | profile | — | Greimas actantial roles per character |
| 66 | `structure.actantial_map[].character_slug` | slug | S | profile | — | — |
| 67 | `structure.actantial_map[].character_role` | enum | S | profile | — | — |
| 68 | `structure.actantial_map[].actantial_role` | enum(6) | S | profile | — | — |

### 2.4 Interpretations

| # | Field Path | Type | Layer | Dest | Core Equivalent | Notes |
|---|-----------|------|-------|------|-----------------|-------|
| 69 | `interpretations.power_asymmetry` | enum | I | profile | — | — |
| 70 | `interpretations.antagonist.antagonist_type` | enum(8) | I | profile | — | Truby |
| 71 | `interpretations.antagonist.arc_type` | enum | I | profile | — | — |
| 72 | `interpretations.antagonist.opposition_level` | enum(5) | I | profile | — | Truby 5-level opposition |
| 73 | `interpretations.antagonist.thematic_mirror` | bool | I | profile | — | — |
| 74 | `interpretations.protagonist_arc.arc_direction` | enum | I | profile | — | — |
| 75 | `interpretations.protagonist_arc.drive_model` | enum(5) | I | profile | — | — |
| 76 | `interpretations.protagonist_arc.lie_believed` | string | I | profile | — | Weiland/Cron |
| 77 | `interpretations.protagonist_arc.truth_needed` | string | I | profile | — | — |
| 78 | `interpretations.protagonist_arc.want_need_alignment` | enum | I | profile | — | — |
| 79 | `interpretations.protagonist_arc.wound_slug` | slug | I | profile | — | — |
| 80 | `interpretations.transtextuality.intertexts[]` | array | I | profile | — | Genette transtextuality |
| 81 | `interpretations.transtextuality.intertexts[].source_text` | string | I | profile | — | — |
| 82 | `interpretations.transtextuality.intertexts[].relation_type` | enum(5) | I | profile | — | — |
| 83 | `interpretations.transtextuality.intertexts[].description` | string | I | profile | — | — |
| 84 | `interpretations.transtextuality.architext_genre` | string | I | profile | — | — |
| 85 | `interpretations.themes[]` | array | I | profile | — | — |
| 86 | `interpretations.themes[].theme` | string | I | profile | — | — |
| 87 | `interpretations.themes[].controlling_idea` | string | I | profile | — | McKee |
| 88 | `interpretations.themes[].thematic_question` | enum | I | profile | — | — |

---

## 3. Scene Card (`ch{NN}_s{NN}.json`)

### 3.1 Top-Level Fields

| # | Field Path | Type | Layer | Dest | Core Equivalent | Notes |
|---|-----------|------|-------|------|-----------------|-------|
| 89 | `scene_id` | slug | O | core | `unit.unit_id` | — |
| 90 | `book_id` | slug | O | core | `unit.artifact_id` | Ref back to parent artifact |
| 91 | `chapter` | int | S | profile | — | Narrative-specific ordering unit |
| 92 | `scene_index` | int | S | profile | — | Position within chapter |
| 93 | `scene_number_in_chapter` | int | S | profile | — | (Alias of scene_index) |

### 3.2 Observables

| # | Field Path | Type | Layer | Dest | Core Equivalent | Notes |
|---|-----------|------|-------|------|-----------------|-------|
| 94 | `observables.focalizer` | slug | O | profile | — | "Focalizer" is Genette narrative concept; core: `unit.observables.primary_participant` or via participant roles |
| 95 | `observables.event_type` | enum(11) | O | split | `unit.observables.event_type` | Concept generalizes; enum values are profile |
| 96 | `observables.participants` | slug[] | O | core | `unit.participants` | Entity references in the unit |
| 97 | `observables.diegetic_level` | enum(3) | O | profile | — | Genette; narrative-specific |
| 98 | `observables.narratee_type` | enum(4) | O | profile | — | Prince; narrative-specific |
| 99 | `observables.setting_instance` | object | O | profile | — | Scene-level setting instantiation |
| 100 | `observables.setting_instance.setting` | slug | O | profile | `unit.context.location` | Concept generalizes; "setting" is narrative |
| 101 | `observables.setting_instance.time_of_day` | enum(9) | O | profile | — | — |
| 102 | `observables.setting_instance.atmosphere` | enum(14) | O | profile | — | — |
| 103 | `observables.setting_instance.weather` | string | O | profile | — | — |
| 104 | `observables.setting_instance.lighting_source` | enum | O | profile | — | — |
| 105 | `observables.setting_instance.lighting_quality` | enum | O | profile | — | — |
| 106 | `observables.setting_instance.spatial_structure` | enum(9) | O | profile | — | Lotman/Tuan |
| 107 | `observables.setting_instance.territory_type` | enum(7) | O | profile | — | — |
| 108 | `observables.setting_instance.sensory_emphasis` | enum[] | O | profile | — | — |
| 109 | `observables.setting_instance.props[]` | object[] | O | profile | — | Physical objects in scene |
| 110 | `observables.setting_instance.motifs_present[]` | object[] | O | profile | — | Motif/symbol tracking |
| 111 | `observables.narrative_time` | object | O | profile | — | Genette temporal architecture |
| 112 | `observables.narrative_time.order` | enum(5) | O | profile | — | Genette order |
| 113 | `observables.narrative_time.analepsis_type` | enum | O | profile | — | — |
| 114 | `observables.narrative_time.prolepsis_type` | enum | O | profile | — | — |
| 115 | `observables.narrative_time.duration_mode` | enum(5) | O | profile | — | Genette duration |
| 116 | `observables.narrative_time.duration.value` | int | O | profile | — | — |
| 117 | `observables.narrative_time.duration.unit` | enum | O | profile | — | — |
| 118 | `observables.narrative_time.frequency` | enum(4) | O | profile | — | Genette frequency |
| 119 | `observables.narrative_time.iterative_pattern` | enum | O | profile | — | — |

### 3.3 Structure

| # | Field Path | Type | Layer | Dest | Core Equivalent | Notes |
|---|-----------|------|-------|------|-----------------|-------|
| 120 | `structure.beat` | enum(15) | S | profile | — | Narrative beat taxonomy (Hero's Journey etc.) |
| 121 | `structure.act` | int 1–5 | S | profile | — | Act structure |
| 122 | `structure.sequence` | enum | S | profile | — | Sequence type (Gulino) |
| 123 | `structure.arc_position` | float 0–1 | S | split | `unit.structure.position` | Concept of continuous position generalizes |
| 124 | `structure.scene_function` | enum(8) | S | profile | — | Seger scene function |
| 125 | `structure.turn` | object | S | split | `unit.transition` | Value change concept generalizes |
| 126 | `structure.turn.from` | enum(24) | S | profile | — | Scene polarity values are narrative |
| 127 | `structure.turn.to` | enum(24) | S | profile | — | — |
| 128 | `structure.causal_role` | enum(6) | S | split | `unit.structure.causal_role` | Concept generalizes; values may need extension |
| 129 | `structure.canonical_summary` | string/obj | S | split | `unit.structure.semantic_fingerprint` | Core concept; narrative grammar is profile |
| 130 | `structure.canonical_summary.want` | string | S | profile | — | Narrative-specific slot |
| 131 | `structure.canonical_summary.obstacle` | string | S | profile | — | — |
| 132 | `structure.canonical_summary.outcome` | enum(6) | S | profile | — | — |
| 133 | `structure.canonical_summary.delta` | string | S | split | `unit.transition.description` | "What changed" is core concept |
| 134 | `structure.canonical_summary.scene_polarity` | enum | S | profile | — | — |
| 135 | `structure.canonical_summary.scene_turns[]` | array | S | split | `unit.structure.steps[]` | Sub-unit decomposition is core concept |
| 136 | `structure.canonical_summary.scene_turns[].observables.turn_number` | int | O | core | `step.sequence_number` | — |
| 137 | `structure.canonical_summary.scene_turns[].observables.active_character` | slug | O | core | `step.agent` | — |
| 138 | `structure.canonical_summary.scene_turns[].observables.verb` | string | O | core | `step.action` | — |
| 139 | `structure.canonical_summary.scene_turns[].observables.target` | string | O | core | `step.target` | — |
| 140 | `structure.canonical_summary.scene_turns[].observables.event_type` | enum(11) | O | split | `step.event_type` | Concept core; values profile |
| 141 | `structure.canonical_summary.scene_turns[].observables.significance` | enum(2) | O | split | `step.significance` | Kernel/satellite (Chatman) concept may generalize |
| 142 | `structure.canonical_summary.scene_turns[].interpretations.emotional_state` | enum(30) | I | profile | — | — |
| 143 | `structure.canonical_summary.scene_turns[].interpretations.masked_emotion` | enum(30) | I | profile | — | — |
| 144 | `structure.canonical_summary.scene_turns[].interpretations.tactic` | enum(20) | I | profile | — | — |
| 145 | `structure.canonical_summary.embedded_analepsis` | object | S | profile | — | Genette analepsis |
| 146 | `structure.canonical_summary.pov_character` | slug | S | profile | — | — |
| 147 | `structure.canonical_summary.location` | string | S | profile | — | — |
| 148 | `structure.canonical_summary.time` | string | S | profile | — | — |
| 149 | `structure.canonical_summary.focalizer` | slug | S | profile | — | — |
| 150 | `structure.canonical_summary.chapter` | int | S | profile | — | — |
| 151 | `structure.canonical_summary.scene` | int | S | profile | — | — |
| 152 | `structure.chapter_position` | enum | S | profile | — | — |
| 153 | `structure.narrative_time` | object | S | profile | — | Note: narrative_time appears in both observables and structure depending on scene card version |
| 154 | `structure.scene_number_in_chapter` | int | S | profile | — | — |

### 3.4 Interpretations

| # | Field Path | Type | Layer | Dest | Core Equivalent | Notes |
|---|-----------|------|-------|------|-----------------|-------|
| 155 | `interpretations.pov` | enum(5) | I | profile | — | Genette/Stanzel |
| 156 | `interpretations.focalization` | enum(5) | I | profile | — | Genette focalization |
| 157 | `interpretations.psychic_distance` | int 1–5 | I | profile | — | Gardner scale |
| 158 | `interpretations.consciousness_mode` | enum(4) | I | profile | — | Cohn |
| 159 | `interpretations.narrator_reliability` | enum(6) | I | profile | — | Booth/Nünning |
| 160 | `interpretations.subtext` | object | I | profile | — | Grice/Hemingway |
| 161 | `interpretations.subtext.technique` | enum | I | profile | — | — |
| 162 | `interpretations.subtext.maxim_violated` | enum | I | profile | — | Grice |
| 163 | `interpretations.subtext.violation_type` | enum | I | profile | — | — |
| 164 | `interpretations.subtext.iceberg_category` | enum | I | profile | — | Hemingway |
| 165 | `interpretations.canonical_metrics.iceberg_proportion` | float 0–1 | I | profile | — | — |
| 166 | `interpretations.canonical_metrics.subtext_load` | float 0–1 | I | profile | — | — |
| 167 | `interpretations.stakes_domain` | string/enum[] | I | profile | — | — |

### 3.5 Craft Targets

| # | Field Path | Type | Layer | Dest | Core Equivalent | Notes |
|---|-----------|------|-------|------|-----------------|-------|
| 168 | `craft_targets.target_tension` | int 1–5 | C | profile | `unit.craft_targets.tension` | Prescriptive; narrative-specific. CAP Narrative Profile integer → profile float 0–1 |
| 169 | `craft_targets.target_pacing` | enum(5) | C | profile | `unit.craft_targets.pacing` | — |
| 170 | `craft_targets.tone` | enum(16) | C | profile | `unit.craft_targets.tone` | Was incorrectly mapped to `unit.interpretations.tone`; corrected 2026-03-11 |

### 3.6 Other Top-Level Scene Card Fields

| # | Field Path | Type | Layer | Dest | Core Equivalent | Notes |
|---|-----------|------|-------|------|-----------------|-------|
| 171 | `prose` | string | O | split | `unit.source_text` | The artifact text; core concept |
| 172 | `word_count` | int | O | core | `unit.metadata.size` | — |
| 173 | `motif_tags` | string[] | I | profile | — | Narrative motif tracking |
| 174 | `theory_notes` | object | I | profile | — | Free-form scholarly commentary |
| 175 | `character_states` | array | S | split | `unit.participant_states` | Concept generalizes; content is profile |

---

## 4. Character Scene State (embedded in Scene Cards)

### 4.1 Observables

| # | Field Path | Type | Layer | Dest | Core Equivalent | Notes |
|---|-----------|------|-------|------|-----------------|-------|
| 176 | `character_states[].observables.character` | slug | O | core | `participant_state.entity_ref` | Entity reference |
| 177 | `character_states[].observables.pov_role` | enum(3) | O | profile | — | Focalizer/participant/non_present is narrative |
| 178 | `character_states[].observables.posture` | enum | O | profile | — | Physical body state; narrative detail |
| 179 | `character_states[].observables.body_language` | enum[] | O | profile | — | — |
| 180 | `character_states[].observables.social_circles_active` | enum[] | O | profile | — | — |
| 181 | `character_states[].observables.fid_markers` | enum[] | O | profile | — | Free indirect discourse; narrative-specific |

### 4.2 Structure

| # | Field Path | Type | Layer | Dest | Core Equivalent | Notes |
|---|-----------|------|-------|------|-----------------|-------|
| 182 | `character_states[].structure.objective` | object | S | split | `participant_state.objective` | Goal concept generalizes |
| 183 | `character_states[].structure.objective.verb` | enum | S | split | `participant_state.objective.action` | Action verb generalizes |
| 184 | `character_states[].structure.objective.object_type` | enum(9) | S | profile | — | Narrative objective categories |
| 185 | `character_states[].structure.objective.target_role` | slug | S | split | `participant_state.objective.target` | — |
| 186 | `character_states[].structure.objective.constraint` | enum | S | profile | — | — |
| 187 | `character_states[].structure.tactic` | enum(20) | S | profile | — | Interpersonal tactic taxonomy |
| 188 | `character_states[].structure.tactic_shift` | enum(20) | S | profile | — | — |
| 189 | `character_states[].structure.obstacle` | object | S | split | `participant_state.obstacle` | Obstacle concept generalizes |
| 190 | `character_states[].structure.obstacle.type` | enum | S | profile | — | — |
| 191 | `character_states[].structure.obstacle.source_role` | slug | S | profile | — | — |
| 192 | `character_states[].structure.trigger_type` | enum(12) | S | profile | — | Psychological trigger taxonomy |
| 193 | `character_states[].structure.want_outcome` | enum(6) | S | profile | — | Granted/denied/deferred/pyrrhic |
| 194 | `character_states[].structure.arc_beat` | enum | S | profile | — | Personal arc position |
| 195 | `character_states[].structure.arc_direction` | enum(3) | S | profile | — | Advancing/regressing/stable |
| 196 | `character_states[].structure.wound_triggered` | bool | S | profile | — | Psychological wound activation |
| 197 | `character_states[].structure.knowledge_at_entry` | array | S | split | `participant_state.pre_state.knowledge` | Information state concept generalizes |
| 198 | `character_states[].structure.knowledge_at_entry[].domain` | enum(8) | S | profile | — | Knowledge domains are narrative |
| 199 | `character_states[].structure.knowledge_at_entry[].predicate` | enum(6) | S | split | — | Know/believe/suspect may generalize |
| 200 | `character_states[].structure.knowledge_at_entry[].about_role` | slug | S | core | — | Entity reference |
| 201 | `character_states[].structure.knowledge_at_entry[].certainty` | float 0–1 | S | core | — | Confidence generalizes |
| 202 | `character_states[].structure.knowledge_gaps` | array | S | split | `participant_state.pre_state.knowledge_gaps` | Information asymmetry generalizes |
| 203 | `character_states[].structure.knowledge_gained` | array | S | split | `participant_state.post_state.knowledge_gained` | Learning/state change generalizes |
| 204 | `character_states[].structure.relationships[]` | array | S | profile | — | Scene-level relational edges |
| 205 | `character_states[].structure.relationships[].target_role` | slug | S | profile | — | — |
| 206 | `character_states[].structure.relationships[].power_balance` | enum(5) | S | profile | — | — |
| 207 | `character_states[].structure.relationships[].power_source` | enum | S | profile | — | — |
| 208 | `character_states[].structure.relationships[].underlying_conflict` | enum | S | profile | — | — |
| 209 | `character_states[].structure.relationships[].wants_from_other` | enum | S | profile | — | — |
| 210 | `character_states[].structure.relationships[].perceives_other_as` | enum | S | profile | — | — |
| 211 | `character_states[].structure.relationships[].trigger_type` | enum(12) | S | profile | — | — |
| 212 | `character_states[].structure.relationships[].relationship_turn` | enum | S | profile | — | — |
| 213 | `character_states[].structure.psychic_distance_shifts[]` | array | S | profile | — | Gardner distance dynamics |
| 214 | `character_states[].structure.psychic_distance_shifts[].trigger` | enum | S | profile | — | — |
| 215 | `character_states[].structure.psychic_distance_shifts[].from_level` | int 1–5 | S | profile | — | — |
| 216 | `character_states[].structure.psychic_distance_shifts[].to_level` | int 1–5 | S | profile | — | — |

### 4.3 Interpretations

| # | Field Path | Type | Layer | Dest | Core Equivalent | Notes |
|---|-----------|------|-------|------|-----------------|-------|
| 217 | `character_states[].interpretations.emotional_state_entry` | object | I | profile | — | — |
| 218 | `character_states[].interpretations.emotional_state_entry.emotion` | enum(30) | I | profile | — | Plutchik/Ekman |
| 219 | `character_states[].interpretations.emotional_state_entry.intensity` | int 1–5 | I | profile | — | — |
| 220 | `character_states[].interpretations.emotional_state_entry.secondary_emotion` | enum(30) | I | profile | — | — |
| 221 | `character_states[].interpretations.emotional_state_entry.masked` | bool | I | profile | — | — |
| 222 | `character_states[].interpretations.emotional_state_exit` | object | I | profile | — | — |
| 223 | `character_states[].interpretations.emotional_state_exit.emotion` | enum(30) | I | profile | — | — |
| 224 | `character_states[].interpretations.emotional_state_exit.intensity` | int 1–5 | I | profile | — | — |
| 225 | `character_states[].interpretations.emotional_state_exit.secondary_emotion` | enum(30) | I | profile | — | — |
| 226 | `character_states[].interpretations.emotional_state_exit.masked` | bool | I | profile | — | — |
| 227 | `character_states[].interpretations.emotional_arc` | enum(9) | I | profile | — | — |
| 228 | `character_states[].interpretations.emotion` | enum(30) | I | profile | — | — |
| 229 | `character_states[].interpretations.masked_emotion` | enum(30) | I | profile | — | — |
| 230 | `character_states[].interpretations.psychic_distance` | int 1–5 | I | profile | — | Gardner |
| 231 | `character_states[].interpretations.consciousness_mode` | enum(4) | I | profile | — | Cohn |
| 232 | `character_states[].interpretations.social_mask` | enum | I | profile | — | — |
| 233 | `character_states[].interpretations.social_role` | enum | I | profile | — | — |
| 234 | `character_states[].interpretations.want_need_alignment` | enum | I | profile | — | — |
| 235 | `character_states[].interpretations.actantial_role` | enum(6) | I | profile | — | Greimas |
| 236 | `character_states[].interpretations.wound_category` | enum(23) | I | profile | — | — |
| 237 | `character_states[].interpretations.stakes` | object | I | profile | — | — |
| 238 | `character_states[].interpretations.stakes.personal` | enum | I | profile | — | — |
| 239 | `character_states[].interpretations.stakes.relational` | enum | I | profile | — | — |
| 240 | `character_states[].interpretations.stakes.level` | int 1–5 | I | profile | — | — |
| 241 | `character_states[].interpretations.arc_type` | enum(7) | I | profile | — | — |
| 242 | `character_states[].interpretations.drive_model` | enum(5) | I | profile | — | — |

---

## 5. Cross-Cutting Concepts

### 5.1 Interpreted Value Wrapper

| # | Concept | Layer | Dest | Core Equivalent | Notes |
|---|---------|-------|------|-----------------|-------|
| 243 | `interpreted_value` wrapper | I | **core** | `interpretation.provenance` | `{value, confidence, source}` is domain-agnostic provenance |
| 244 | `confidence` (0.0–1.0) | I | **core** | `interpretation.confidence` | — |
| 245 | `source` (human\|model\|inferred\|consensus) | I | **core** | `interpretation.source` | — |

### 5.2 Conformance Levels

| # | Concept | Layer | Dest | Core Equivalent | Notes |
|---|---------|-------|------|-----------------|-------|
| 246 | Level 1: Schema Conformant | S | **core** | Level 1: Schema Conformant | — |
| 247 | Level 2: Referentially Conformant | S | **core** | Level 2: Referentially Conformant | Entity refs resolve |
| 248 | Level 3: Round-Trip Conformant | S | **core** | Level 3: Round-Trip Conformant | Semantic fingerprint verifiable |

### 5.3 Validation Severity

| # | Concept | Layer | Dest | Notes |
|---|---------|-------|------|-------|
| 249 | ERROR | S | **core** | Must reject |
| 250 | WARNING | S | **core** | May have issue |
| 251 | INFO | S | **core** | Informational |

---

## 6. Summary Statistics

| Category | Total Fields | Core | Profile | Split |
|----------|-------------|------|---------|-------|
| Entity Registry | 47 | 7 | 36 | 4 |
| Story Architecture | 41 | 3 | 38 | 0 |
| Scene Card | 87 | 7 | 66 | 14 |
| Character Scene State | 67 | 3 | 56 | 8 |
| Cross-Cutting | 9 | 9 | 0 | 0 |
| **Total** | **251** | **29** | **196** | **26** |

**Key finding:** Only ~12% of current CAP Narrative Profile fields belong in the core. The vast majority (~78%) are narrative profile. ~10% need splitting — these are where the concept is domain-agnostic but the enum values or grammar are narrative-specific.

---

## 7. Split Proposals

For each field marked `split`, the concept generalizes to core but the specific values, grammar, or type options belong in the narrative profile.

### SP-01: `relationships[].structure.rel_type`

**Current:** Closed enum of 18 narrative relationship types (family_parent_child, romantic, mentor, rival, etc.)  
**Core concept:** Typed link between two entities — `relationship.relationship_type` as a string validated against the active profile's type registry.  
**Narrative profile:** Registers the 18 relationship type values.  
**Software profile:** Would register values like `dependency`, `api_consumer`, `data_flow`, `ownership`, `inheritance`.

### SP-02: `observables.event_type`

**Current:** Closed enum of 11 narrative event types (action, dialogue, internal_shift, revelation, confrontation, etc.)  
**Core concept:** `unit.observables.event_type` or `step.event_type` — a typed classification of what kind of interaction occurred.  
**Narrative profile:** Registers the 11 event type values.  
**Software profile:** Would register values like `request`, `response`, `query`, `mutation`, `error`, `timeout`, `authentication`.

### SP-03: `structure.arc_position`

**Current:** Float 0.0–1.0 representing position through the story arc.  
**Core concept:** `unit.structure.position` — normalized position within the artifact's primary sequence. Domain-agnostic.  
**No split needed on values — the float range is universal. The *interpretation* of positions (e.g., 0.25 = end of Act 1) is profile-level.

### SP-04: `structure.turn` (value change)

**Current:** Object with `from` and `to` fields using a 24-value scene_polarity enum.  
**Core concept:** `unit.transition` — a typed value change within the unit. The concept of "something changed direction" is domain-agnostic.  
**Narrative profile:** Registers the 24 polarity values.  
**Software profile:** Would register values like `healthy → degraded`, `authenticated → expired`, `available → rate_limited`.

### SP-05: `structure.canonical_summary`

**Current:** Structured object with narrative-specific grammar slots (want, obstacle, outcome, delta, scene_turns, pov_character, location).  
**Core concept:** `unit.structure.semantic_fingerprint` — a machine-verifiable serialization of the unit's essential semantic content, supporting the round-trip guarantee. The *concept* of a deterministic summary is the core's most important contribution.  
**Narrative profile:** Defines the narrative grammar template (POV_CHAR, EVENT_VERB, LOCATION, WANT, OUTCOME, etc.).  
**Software profile:** Would define its own grammar (e.g., AGENT → ACTION → TARGET; precondition → postcondition; error_class → recovery).  
**Core interface:** `semantic_fingerprint.render(unit, registry) → string` and `semantic_fingerprint.parse(string, registry) → unit`. Profiles implement these.

### SP-06: `structure.canonical_summary.delta`

**Current:** Single sentence stating what changed in the scene.  
**Core concept:** `unit.transition.description` — "what changed" is the universal core of any transformation unit. This is arguably the single most important field to generalize.  
**Both profiles produce deltas: "Nadia's armor took the first small crack" (narrative) / "The auth service now rejects tokens older than 30 minutes" (software).

### SP-07: `structure.canonical_summary.scene_turns[]`

**Current:** Array of sub-unit steps within a scene, each with observables (turn_number, active_character, verb, target, event_type, significance) and interpretations (emotional_state, masked_emotion, tactic).  
**Core concept:** `unit.structure.steps[]` — ordered sub-unit decomposition. Observable step fields (sequence_number, agent, action, target) generalize. Interpretive step fields (emotional_state, tactic) are profile.  
**Core step schema:** `{ sequence_number, agent, action, target, event_type, significance }`  
**Narrative profile extension:** adds `emotional_state`, `masked_emotion`, `tactic` to step interpretations.  
**Software profile extension:** would add `response_code`, `latency`, `error_class` to step observables.

### SP-08: `structure.canonical_summary.scene_turns[].observables.significance`

**Current:** Enum with 2 values: `kernel` (removal destroys causal chain) and `satellite` (expandable texture).  
**Core concept:** The kernel/satellite distinction (Chatman) maps to a general concept of **essential vs. supplementary steps**. In software: a request step is kernel; a logging step is satellite.  
**Core generalization:** Keep as `significance` with values `essential` / `supplementary`. Profiles may alias or extend.

### SP-09: `structure.causal_role`

**Current:** Enum of 6 values (setup, catalyst, complication, escalation, crisis, resolution).  
**Core concept:** A unit's function in the causal chain of the artifact. The concept is domain-agnostic. Some values may generalize directly (setup, resolution). Others may be narrative-specific.  
**Core:** Define `unit.structure.causal_role` with core values `setup`, `trigger`, `complication`, `resolution`, profile-extensible.  
**Narrative profile:** Adds `catalyst`, `escalation`, `crisis` or maps them to core values with narrative semantics.

### SP-10: `character_states[].structure.objective`

**Current:** Object with verb, object_type (9 narrative categories), target_role, constraint.  
**Core concept:** `participant_state.objective` — what an entity is trying to achieve in this unit. The concept of "an entity has a goal within a unit" is universal.  
**Core:** `{ action: string, target: entity_ref }` — minimal goal representation.  
**Narrative profile:** Extends with `object_type`, `constraint`, narrative-specific verb enum.  
**Software profile:** Would express objectives like `{ action: "authenticate", target: "user_session" }`.

### SP-11: `character_states[].structure.knowledge_*`

**Current:** Three arrays (knowledge_at_entry, knowledge_gaps, knowledge_gained) with items having domain (8 narrative values), predicate (6 values), about_role, certainty.  
**Core concept:** Information state tracking is genuinely domain-agnostic. In software: "service A knows service B's API key has been rotated" is the same pattern as "character A knows character B's secret."  
**Core:** `participant_state.information_state = { knows[], gaps[], gained[] }` with items `{ subject, predicate, about, certainty }`.  
**Narrative profile:** Registers knowledge domains (secrets, plans, relationships, etc.) and predicates (knows, believes, suspects, etc.).  
**Software profile:** Would register domains (configuration, credentials, state, capacity) and predicates (knows, assumes, monitors).

### SP-12: `character_states[].structure.obstacle`

**Current:** Object with type (enum) and source_role (slug).  
**Core concept:** What blocks an entity's objective. Universal in both narrative and software (a failing dependency blocks a service's goal).  
**Core:** `participant_state.obstacle = { type: string, source: entity_ref }` — profile validates type values.

### SP-13: `prose` (scene card top-level)

**Current:** The actual prose text of the scene passage.  
**Core concept:** `unit.source_text` — the raw artifact text from which the unit was derived. In software: the actual code block, config file, or API trace. Essential for round-trip verification.  
**No splitting needed — the field itself generalizes. Just rename to `source_text`.

### SP-14: `character_states` (array)

**Current:** Array of character scene states embedded in each scene card.  
**Core concept:** `unit.participant_states[]` — per-entity state snapshots within a unit. The concept of tracking how each participant enters and exits a unit is domain-agnostic.  
**Core:** Defines the array structure and entity reference. Profile defines the state schema (what gets tracked per participant).

---

## 8. Observations

### 8.1 The epistemic separation (ADR-006) was the right preparation

The observable/structure/interpretation split already established in v0.2.0 maps cleanly to the core protocol's needs. The new dimension is core vs. profile, which is orthogonal to the epistemic layers.

### 8.2 Narrative time is entirely profile

All Genette temporal architecture (order, duration, frequency, analepsis/prolepsis types) is narrative-specific. The core protocol needs only a generic `unit.temporal_context` (optional timestamp or ordering metadata), not a theory-grounded temporal model.

### 8.3 The story architecture document decomoses

`story_architecture.json` is really three things:
1. **Artifact metadata** (title, author, word_count) → core `artifact.metadata`
2. **Structural overview** (act_count, chapter_count, genre, collision_architecture) → narrative profile artifact-level config
3. **Interpretive overlay** (themes, protagonist_arc, antagonist, transtextuality) → narrative profile interpretations

In the new architecture, there is no single "story architecture" document. The artifact has metadata, profiles add structure, and interpretations are layered on.

### 8.4 Craft targets are a narrative innovation worth noting

The fourth epistemic section (`craft_targets`) — prescriptive authorial intent — does not belong in the core's three-section model (observables/structure/interpretations). The core documents that profiles MAY define additional epistemic sections beyond the three core sections, and `craft_targets` is the narrative profile's example of this.

**Implementation status:** `craft_targets` is now a registered fourth section on `unit.schema.json` and `SipUnit.craft_targets` (Rust). The narrative PROFILE.md §4.1 defines three fields: `tension`, `pacing`, `tone`. The converter (`grimoire-cap-convert`) emits all three. The fingerprint grammar uses `TONE=<tone>` from `unit.craft_targets.tone`.

### 8.5 Voice signature and voice embedding are production-layer

`voice_signature` and `voice_embedding` are tools for prose generation consistency. They blur the line between the analytical protocol and the production pipeline. In a strict separation, they belong in a narrative production adapter, not in the entity registry. For now, keeping them in the narrative profile is acceptable.

### 8.6 Knowledge tracking may be the strongest generalization candidate

The `knowledge_at_entry` / `knowledge_gaps` / `knowledge_gained` pattern maps remarkably well to software systems (what does service A know about service B's state?). This may deserve promotion to a core feature rather than remaining profile-specific. The belief/certainty model (`knows`, `believes`, `suspects` × `certainty: 0.0–1.0`) is a general epistemic state tracker.
