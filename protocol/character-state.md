# Character Scene State

A Character Scene State describes one character's internal and relational state at the entry and exit boundaries of a single scene — their emotions, knowledge, objectives, tactics, and arc position.

Character Scene States are embedded within Scene Cards in the `character_states` array. They are not stored as separate files.

For normative field definitions, see [SPECIFICATION.md §8](../SPECIFICATION.md#8-character-scene-state).

---

## Design Principle

Character psychology in fiction is a state machine — each scene advances or arrests the state. The Character Scene State schema operationalizes two foundational claims:

1. **Character is action:** What a character *wants* in a scene (Stanislavski) determines what they *do*, which determines meaning.
2. **Character is consciousness:** What a character *knows, feels, and perceives* (Cohn) determines how the scene is rendered if they are the focalizer.

The schema bridges dramaturgical tradition (wanting/doing) and narratological tradition (knowing/perceiving) in a single structured object.

---

## Required Fields

| Field | Type | Notes |
|-------|------|-------|
| `character` | slug | MUST resolve to declared character in registry |
| `pov_role` | enum | `focalizer`, `participant`, or `non_present` |

---

## Emotional State

| Field | Type | Notes |
|-------|------|-------|
| `emotional_state_entry` | EmotionObject | Emotion at scene open |
| `emotional_state_exit` | EmotionObject | Emotion at scene close |
| `emotional_arc` | enum | Shape of emotional movement |

**EmotionObject:**

```json
{
  "emotion": "humiliation",
  "intensity": 4,
  "secondary_emotion": "anger",
  "masked": true
}
```

If `emotional_state_entry` equals `emotional_state_exit`, the scene has not performed emotional work for this character.

The `masked` boolean indicates whether the character is suppressing or performing a different emotion than they feel — this determines what the prose can access vs. what it must perform externally.

---

## Epistemic State

| Field | Type | Notes |
|-------|------|-------|
| `knowledge_at_entry` | array | What character knows at scene open |
| `knowledge_gaps` | array | What character does not know but should or wants to |
| `knowledge_gained` | array | What character learns during the scene |

**KnowledgeObject:**

```json
{
  "domain": "relationships",
  "predicate": "suspects",
  "about_role": "fitzwilliam_darcy",
  "certainty": 0.3
}
```

Information asymmetry is the primary mechanism of dramatic irony. The three-array structure tracks what changes in the character's epistemic state across the scene boundary.

---

## Action Grammar

| Field | Type | Notes |
|-------|------|-------|
| `objective` | ObjectiveObject | Character's scene-level want |
| `tactic` | enum | How they pursue it |
| `obstacle` | string | What blocks the objective |

**ObjectiveObject:**

```json
{
  "verb": "to refuse",
  "object_type": "commitment",
  "target_role": "fitzwilliam_darcy",
  "constraint": "Must not appear rude to the host"
}
```

The objective `verb` MUST be a transitive action verb directed at `target_role`. "To feel better" is not a valid objective. "To convince [target] to withdraw" is valid.

---

## Arc State

| Field | Type | Notes |
|-------|------|-------|
| `arc_beat` | enum | Position in character arc |
| `arc_direction` | string | `advancing`, `regressing`, or `stable` |
| `want_outcome` | enum | Outcome of the scene want |
| `wound_triggered` | boolean | Whether psychological wound was activated |

---

## Focalizer-Specific Fields

These fields apply only when `pov_role == "focalizer"`:

| Field | Type | Notes |
|-------|------|-------|
| `psychic_distance` | integer 1–5 | Gardner scale for this character's rendering |
| `psychic_distance_shifts` | array | Dynamic distance changes during scene |
| `consciousness_mode` | enum | Cohn mode for this character's interior |
| `fid_markers` | array of string | FID markers that SHOULD appear in prose |

---

## Example

```json
{
  "character": "elizabeth_bennet",
  "pov_role": "focalizer",
  "psychic_distance": 4,
  "consciousness_mode": "narrated_monologue",
  "emotional_state_entry": {
    "emotion": "shock",
    "intensity": 3,
    "secondary_emotion": "anger",
    "masked": false
  },
  "emotional_state_exit": {
    "emotion": "contempt",
    "intensity": 4,
    "masked": false
  },
  "objective": {
    "verb": "to refuse",
    "object_type": "commitment",
    "target_role": "fitzwilliam_darcy",
    "constraint": "Must maintain civility"
  },
  "tactic": "direct_confrontation",
  "want_outcome": "granted",
  "wound_triggered": true,
  "arc_direction": "advancing"
}
```
