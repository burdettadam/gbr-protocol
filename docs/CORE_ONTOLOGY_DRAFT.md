# Core Ontology Draft — Canonical Artifact Protocol v0.1.0

**Status:** Draft  
**Date:** 2026-03-10  
**Purpose:** Define the minimum domain-agnostic ontology for decomposing artifacts into canonical representations, supporting round-trip transformation and layered interpretation.

---

## 1. Problem Statement

Complex artifacts — novels, codebases, architectural designs, legal documents — resist machine understanding because they mix observable facts, structural organization, and interpretive meaning into a single undifferentiated surface. Current decomposition approaches are either domain-locked (tied to one artifact type) or too abstract to validate (no round-trip guarantee).

The Canonical Artifact Protocol (CAP) defines a general-purpose canonical representation that:

1. Separates what is observed from what is inferred
2. Enables domain-specific richness through a profile extension mechanism
3. Guarantees round-trip verifiability at each epistemic layer
4. Supports multiple projections (views) over a single canonical representation

---

## 2. Core Principles

### Principle 1: Canonical representation is separate from source rendering

The protocol represents the *structure and meaning* of an artifact, not its surface form. Prose is not the canonical form of a novel. Code is not the canonical form of a system. The canonical representation is a structured decomposition from which the source can be reconstructed and against which reconstruction can be verified.

### Principle 2: Observable structure is separate from interpretation

Every object in the protocol distinguishes between:
- **Observables** — facts grounded directly in the artifact, verifiable by inspection
- **Structure** — how observables are organized, derivable with certainty from the artifact
- **Interpretations** — inferred meaning layered on top, carrying provenance and confidence

This three-layer epistemic model is mandatory for all core objects. Domain profiles MAY define additional epistemic sections (e.g., prescriptive intent).

### Principle 3: Core protocol is domain-agnostic

No core object, field, or type references a specific domain. Terms like "character," "scene," "chapter," "function," "module," or "endpoint" never appear in the core. Domain-specific concepts live in profiles.

### Principle 4: Domain richness lives in profiles, not the core

The core is deliberately minimal. All domain-specific entity types, unit types, relationship types, interpretation types, enum values, validation rules, and rendering grammars are defined by domain profiles. The core provides the extension mechanism; profiles provide the vocabulary.

### Principle 5: Multiple views may be derived from one canonical artifact

A single canonical representation supports multiple projections: chronological view, dependency graph, entity trajectory, deployment view. Views are first-class objects, not ad-hoc queries. Profiles define which views are canonical for their domain.

### Principle 6: Round-trip comparison occurs by layer

Fidelity is measured separately at each epistemic layer:
- **Observable fidelity** — are the same facts present?
- **Structural fidelity** — is the organization preserved?
- **Interpretive fidelity** — do the inferences align?

This prevents "lossless" from becoming a single vague claim. A transformation may preserve observables perfectly while diverging on interpretation — and that is a meaningful, measurable distinction.

---

## 3. Core Objects

### 3.1 Artifact

The top-level container: the whole thing being represented.

An Artifact is a complete canonical decomposition of one source work. A novel, a microservice system, a legal contract — each becomes one Artifact.

#### Fields

| Field | Type | Required | Section | Description |
|-------|------|----------|---------|-------------|
| `artifact_id` | slug | **yes** | identity | Unique identifier |
| `protocol_version` | semver | **yes** | identity | CAP version this artifact conforms to |
| `profile` | string | **yes** | identity | Domain profile identifier (e.g., `narrative`, `software`) |
| `profile_version` | semver | **yes** | identity | Profile version |
| `metadata` | object | no | observables | Domain-agnostic metadata (title, author/owner, size, creation date) |
| `entities` | Entity[] | **yes** | structure | All persistent entities declared in this artifact |
| `units` | Unit[] | **yes** | structure | Ordered decomposition of the artifact into atomic units |
| `relationships` | Relationship[] | no | structure | Cross-entity and cross-unit links |
| `views` | View[] | no | structure | Named projections over the canonical data |
| `interpretations` | Interpretation[] | no | interpretations | Artifact-level inferences (themes, architectural style, etc.) |

#### Validation Rules

- `artifact_id` MUST be a non-empty slug (`^[a-z0-9_]+$`)
- Every `Entity`, `Unit`, `Relationship`, and `View` within an Artifact MUST reference only entities and units declared within the same Artifact
- `profile` MUST resolve to a registered domain profile

#### Examples

**Narrative:**
```json
{
  "artifact_id": "threshold",
  "protocol_version": "0.1.0",
  "profile": "narrative",
  "profile_version": "1.0.0",
  "metadata": { "title": "Threshold", "author": "Example Author", "size": 45000 }
}
```

**Software:**
```json
{
  "artifact_id": "auth_system",
  "protocol_version": "0.1.0",
  "profile": "software",
  "profile_version": "1.0.0",
  "metadata": { "title": "Authentication Subsystem", "owner": "platform-team", "size": 12400 }
}
```

---

### 3.2 Entity

A persistent participant or object of interest that exists across multiple units.

Entities are declared once in the artifact and referenced by ID elsewhere. Their book-level (or system-level) properties live in the entity declaration; their per-unit state lives in `participant_states` within each Unit.

#### Fields

| Field | Type | Required | Section | Description |
|-------|------|----------|---------|-------------|
| `entity_id` | slug | **yes** | identity | Unique within artifact |
| `entity_type` | string | **yes** | structure | Profile-defined type (e.g., `character`, `component`) |
| `display_name` | string | **yes** | observables | Human-readable name |
| `observable_descriptors` | object | no | observables | Profile-defined observable properties |
| `structural_properties` | object | no | structure | Profile-defined structural properties |
| `interpretations` | object | no | interpretations | Profile-defined interpretive properties |

#### Validation Rules

- `entity_id` MUST be unique within the artifact
- `entity_type` MUST be a value registered by the active profile
- All entity references (`entity_ref`) elsewhere in the artifact MUST resolve to a declared entity

#### Examples

**Narrative — Character:**
```json
{
  "entity_id": "nadia",
  "entity_type": "character",
  "display_name": "Nadia Vance",
  "observable_descriptors": { "slot": "protagonist" },
  "structural_properties": { "role": "protagonist", "voice_signature": { "..." : "..." } },
  "interpretations": { "archetype": "explorer", "wound": "grief", "arc_type": "positive_change" }
}
```

**Software — Component:**
```json
{
  "entity_id": "auth_service",
  "entity_type": "service",
  "display_name": "Authentication Service",
  "observable_descriptors": { "language": "go", "entry_point": "cmd/auth/main.go" },
  "structural_properties": { "layer": "infrastructure", "protocol": "grpc" },
  "interpretations": { "responsibility": "identity_verification", "pattern": "gateway" }
}
```

---

### 3.3 Unit

An atomic transformation or interaction block — the smallest meaningful chunk of the artifact that contains a complete interaction cycle (pre-state → action → post-state).

Units are the atomic level of the protocol. They correspond to "scenes" in narrative, "interactions" in software, "clauses" in legal documents, etc.

#### Fields

| Field | Type | Required | Section | Description |
|-------|------|----------|---------|-------------|
| `unit_id` | slug | **yes** | identity | Unique within artifact |
| `artifact_id` | slug | **yes** | identity | Back-reference to parent artifact |
| `unit_type` | string | no | structure | Profile-defined type (e.g., `scene`, `interaction`) |
| `sequence_index` | integer | **yes** | structure | Position in artifact's primary ordering |
| **Observables** | | | | |
| `observables.participants` | entity_ref[] | **yes** | observables | Entities involved in this unit |
| `observables.context` | object | no | observables | Profile-defined contextual observables (location, time, environment) |
| `observables.event_type` | string | no | observables | Profile-defined event classification |
| `observables.source_text` | string | no | observables | Raw artifact text this unit was derived from |
| **Structure** | | | | |
| `structure.position` | float 0.0–1.0 | no | structure | Normalized position in artifact's primary sequence |
| `structure.causal_role` | string | no | structure | Function in causal chain (profile-extensible; core values: `setup`, `trigger`, `complication`, `resolution`) |
| `structure.steps` | Step[] | no | structure | Ordered sub-unit decomposition (see §3.3.1) |
| `structure.transition` | Transition | no | structure | Value change within this unit (see §3.6) |
| `structure.semantic_fingerprint` | object/string | no | structure | Machine-verifiable semantic summary (profile defines grammar) |
| `structure.grouping` | object | no | structure | Profile-defined grouping metadata (chapter, act, module, package) |
| **Interpretations** | | | | |
| `interpretations` | object | no | interpretations | Profile-defined interpretive properties |
| **Participant States** | | | | |
| `participant_states` | ParticipantState[] | no | structure | Per-entity state at unit boundaries |
| **Metadata** | | | | |
| `metadata` | object | no | — | Size, word count, LOC, etc. |

#### 3.3.1 Step (sub-unit)

An ordered atomic action within a Unit.

| Field | Type | Required | Section | Description |
|-------|------|----------|---------|-------------|
| `sequence_number` | integer | **yes** | structure | Order within the unit |
| `agent` | entity_ref | **yes** | observables | Entity performing the action |
| `action` | string | **yes** | observables | Verb or action identifier |
| `target` | string | no | observables | What the action is directed at |
| `event_type` | string | no | observables | Profile-defined event classification |
| `significance` | string | no | structure | `essential` or `supplementary` (generalizes kernel/satellite) |
| `interpretations` | object | no | interpretations | Profile-defined step-level interpretations |

#### Validation Rules

- `unit_id` MUST be unique within the artifact
- Every entity_ref in `participants` MUST resolve to a declared entity
- `steps` MUST be ordered by `sequence_number`
- If `semantic_fingerprint` is present, the profile MUST define a `render`/`parse` contract for round-trip verification

#### Examples

**Narrative — Scene:**
```json
{
  "unit_id": "threshold_ch01_s01",
  "artifact_id": "threshold",
  "unit_type": "scene",
  "sequence_index": 1,
  "observables": {
    "participants": ["nadia"],
    "context": {
      "setting": "childhood_home",
      "time_of_day": "morning",
      "atmosphere": "uncanny"
    }
  },
  "structure": {
    "position": 0.0,
    "causal_role": "setup",
    "transition": {
      "description": "Nadia's armor — the fiction that this is a logistical task — took the first small crack."
    }
  }
}
```

**Software — Interaction:**
```json
{
  "unit_id": "auth_login_flow",
  "artifact_id": "auth_system",
  "unit_type": "interaction",
  "sequence_index": 1,
  "observables": {
    "participants": ["api_gateway", "auth_service", "user_db"],
    "context": {
      "protocol": "https",
      "endpoint": "/api/v1/login"
    }
  },
  "structure": {
    "causal_role": "trigger",
    "transition": {
      "description": "User transitions from unauthenticated to authenticated with a session token."
    }
  }
}
```

---

### 3.4 Relationship

A typed, directed link between two entities, two units, or an entity and a unit.

#### Fields

| Field | Type | Required | Section | Description |
|-------|------|----------|---------|-------------|
| `source` | ref | **yes** | observables | Entity or unit ID |
| `target` | ref | **yes** | observables | Entity or unit ID |
| `relationship_type` | string | **yes** | structure | Profile-defined type |
| `evidence` | string | no | observables | Observable basis for the relationship |
| `interpretations` | object | no | interpretations | Profile-defined interpretive properties |

#### Validation Rules

- `source` and `target` MUST resolve to declared entities or units
- `relationship_type` MUST be registered by the active profile

#### Examples

**Narrative:**
```json
{
  "source": "nadia",
  "target": "father",
  "relationship_type": "family_parent_child",
  "evidence": "Registry declares parent-child bond; confirmed by prose references to 'her father's house'",
  "interpretations": {
    "dynamic": "distant",
    "power_balance": "target_dominant"
  }
}
```

**Software:**
```json
{
  "source": "api_gateway",
  "target": "auth_service",
  "relationship_type": "dependency",
  "evidence": "import statement in gateway/auth_client.go",
  "interpretations": {
    "coupling": "tight",
    "criticality": "high"
  }
}
```

---

### 3.5 State

A named condition attached to an entity, group, or the artifact as a whole, at a specific point.

#### Fields

| Field | Type | Required | Section | Description |
|-------|------|----------|---------|-------------|
| `subject` | ref | **yes** | observables | What this state describes (entity or unit ref) |
| `state_type` | string | **yes** | structure | Profile-defined state category |
| `value` | any | **yes** | — | The state value (type depends on `state_type`) |
| `evidence` | string | no | observables | Observable basis |
| `provenance` | string | no | interpretations | `human`, `model`, `inferred`, `consensus`, or arbitrary source string |
| `confidence` | float 0.0–1.0 | no | interpretations | Certainty of this state assignment |

#### Examples

**Narrative:**
```json
{
  "subject": "nadia",
  "state_type": "emotional",
  "value": "grief",
  "evidence": "Narrator describes 'four months of postponement'",
  "provenance": "human",
  "confidence": 0.9
}
```

**Software:**
```json
{
  "subject": "auth_service",
  "state_type": "health",
  "value": "degraded",
  "evidence": "Error rate > 5% in last 5 minutes (monitoring dashboard)",
  "provenance": "inferred",
  "confidence": 0.95
}
```

---

### 3.6 Transition

The change from one state to another, representing the transformation that occurs within or across units.

#### Fields

| Field | Type | Required | Section | Description |
|-------|------|----------|---------|-------------|
| `subject` | ref | no | observables | Entity or unit undergoing the transition |
| `before` | State | no | structure | Pre-transition state |
| `after` | State | no | structure | Post-transition state |
| `trigger` | string | no | structure | What caused the change |
| `description` | string | **yes** | structure | Human-readable statement of what changed (the "delta") |
| `confidence` | float 0.0–1.0 | no | interpretations | Certainty that this transition occurred |
| `grounding` | string | no | observables | Evidence from the source artifact |

#### Validation Rules

- `description` MUST state a *change*, not a *condition*. "X happened" is invalid; "X changed from A to B" or "X was established" is valid.

#### Examples

**Narrative:**
```json
{
  "subject": "nadia",
  "before": { "state_type": "defense", "value": "intact" },
  "after": { "state_type": "defense", "value": "cracked" },
  "trigger": "The house is smaller than she remembered",
  "description": "Nadia's armor — the fiction that this is a logistical task — took the first small crack.",
  "grounding": "She stops in the kitchen doorway without knowing why."
}
```

**Software:**
```json
{
  "subject": "user_session",
  "before": { "state_type": "authentication", "value": "unauthenticated" },
  "after": { "state_type": "authentication", "value": "authenticated" },
  "trigger": "Valid credentials submitted to /api/v1/login",
  "description": "User transitioned from unauthenticated to authenticated with a 30-minute session token."
}
```

---

### 3.7 View

A named projection over the canonical artifact — a way of reading the same data for a specific purpose.

Views do not add information; they select and organize existing information for a particular lens. One artifact may have many views.

#### Fields

| Field | Type | Required | Section | Description |
|-------|------|----------|---------|-------------|
| `view_id` | slug | **yes** | identity | Unique within artifact |
| `view_type` | string | **yes** | structure | Profile-defined view type |
| `description` | string | no | structure | What this view shows |
| `scope` | ref[] | no | structure | Which entities/units are included (default: all) |
| `ordering` | string | no | structure | How items are ordered (e.g., `chronological`, `dependency`, `causal`) |
| `data` | object | no | structure | View-specific computed data |

#### Examples

**Narrative — Character Arc View:**
```json
{
  "view_id": "nadia_arc",
  "view_type": "entity_trajectory",
  "description": "Nadia's emotional and psychological arc across all scenes",
  "scope": ["nadia"],
  "ordering": "chronological",
  "data": {
    "trajectory_points": [
      { "unit": "threshold_ch01_s01", "state": "armor_intact", "direction": "stable" },
      { "unit": "threshold_ch01_s02", "state": "armor_cracking", "direction": "advancing" }
    ]
  }
}
```

**Software — Dependency Graph View:**
```json
{
  "view_id": "service_dependencies",
  "view_type": "dependency_graph",
  "description": "Runtime service dependency graph",
  "ordering": "dependency",
  "data": {
    "edges": [
      { "source": "api_gateway", "target": "auth_service", "type": "synchronous" },
      { "source": "auth_service", "target": "user_db", "type": "synchronous" }
    ]
  }
}
```

---

### 3.8 Interpretation

A structured inference attached to any object in the protocol. Interpretations are always separate from observables and always carry provenance.

#### Fields

| Field | Type | Required | Section | Description |
|-------|------|----------|---------|-------------|
| `target_ref` | ref | **yes** | — | What this interpretation is about (entity, unit, relationship, or artifact ref) |
| `interpretation_type` | string | **yes** | — | Profile-defined type (e.g., `motivation`, `architectural_intent`) |
| `value` | any | **yes** | — | The interpretive claim |
| `confidence` | float 0.0–1.0 | no | — | Certainty |
| `rationale` | string | no | — | Why this interpretation was made |
| `evidence_refs` | ref[] | no | — | References to observable data supporting this interpretation |
| `source` | string | no | — | Provenance: `human`, `model`, `inferred`, `consensus`, or arbitrary string (e.g., `model:gpt-4`, `human:editor`) |

#### Validation Rules

- `target_ref` MUST resolve to a declared object in the artifact
- `interpretation_type` MUST be registered by the active profile
- `confidence` defaults to 1.0 if omitted (assumed certain)

#### Examples

**Narrative:**
```json
{
  "target_ref": "nadia",
  "interpretation_type": "motivation",
  "value": "Avoidance of grief through compulsive forward motion",
  "confidence": 0.85,
  "rationale": "Nadia's cataloguing behavior and clinical tone mask emotional processing",
  "evidence_refs": ["threshold_ch01_s01.steps[3]"],
  "source": "human"
}
```

**Software:**
```json
{
  "target_ref": "auth_service",
  "interpretation_type": "architectural_intent",
  "value": "Single responsibility: all authentication logic centralized",
  "confidence": 0.9,
  "rationale": "No other service handles credential validation",
  "evidence_refs": ["auth_login_flow"],
  "source": "inferred"
}
```

---

### 3.9 ParticipantState

Per-entity state snapshot within a specific Unit. Tracks how an entity enters, acts within, and exits a unit.

#### Fields

| Field | Type | Required | Section | Description |
|-------|------|----------|---------|-------------|
| `entity_ref` | slug | **yes** | observables | Which entity |
| `role_in_unit` | string | no | structure | Profile-defined role (e.g., `focalizer`, `initiator`, `responder`) |
| `pre_state` | object | no | structure | Entity state at unit entry (profile-defined) |
| `post_state` | object | no | structure | Entity state at unit exit (profile-defined) |
| `objective` | object | no | structure | What this entity is trying to achieve in this unit |
| `objective.action` | string | no | structure | Goal verb/action |
| `objective.target` | ref | no | structure | Goal target |
| `obstacle` | object | no | structure | What blocks the objective |
| `information_state` | object | no | structure | Knowledge tracking (see §3.9.1) |
| `observables` | object | no | observables | Profile-defined observable properties (posture, body language, etc.) |
| `interpretations` | object | no | interpretations | Profile-defined interpretive properties (emotion, motivation, etc.) |

#### 3.9.1 Information State

Tracks what an entity knows, does not know, and learns within a unit.

| Field | Type | Description |
|-------|------|-------------|
| `knows` | InformationItem[] | Facts known at entry |
| `gaps` | InformationItem[] | Facts not known at entry |
| `gained` | InformationItem[] | Facts learned during unit |

**InformationItem:**

| Field | Type | Description |
|-------|------|-------------|
| `subject` | string | What the information is about (profile-defined domain) |
| `predicate` | string | Epistemic relation (profile-defined; common: `knows`, `believes`, `suspects`, `assumes`) |
| `about` | ref | Entity the information concerns |
| `certainty` | float 0.0–1.0 | How certain the knowledge is |

---

## 4. Profile Extension Mechanism

A domain profile extends the core protocol by declaring:

### 4.1 Type Registries

Each profile MUST register allowed values for:

| Registry | Core Field | Example (Narrative) | Example (Software) |
|----------|-----------|---------------------|---------------------|
| Entity Types | `entity.entity_type` | `character`, `location`, `object`, `group` | `service`, `component`, `module`, `data_store`, `interface` |
| Unit Types | `unit.unit_type` | `scene`, `beat`, `chapter_segment` | `interaction`, `process`, `transaction` |
| Relationship Types | `relationship.relationship_type` | `family_parent_child`, `romantic`, `mentor`, `rival` | `dependency`, `api_consumer`, `data_flow`, `ownership` |
| Interpretation Types | `interpretation.interpretation_type` | `motivation`, `emotion`, `theme`, `conflict_role` | `responsibility`, `pattern`, `trust_boundary`, `architectural_style` |
| Event Types | `unit.observables.event_type` / `step.event_type` | `action`, `dialogue`, `internal_shift`, `revelation` | `request`, `response`, `query`, `mutation`, `error` |
| State Types | `state.state_type` | `emotional`, `relational`, `psychological` | `health`, `authentication`, `capacity`, `configuration` |
| Causal Roles | `unit.structure.causal_role` | `catalyst`, `escalation`, `crisis` (extends core) | `initialization`, `retry`, `fallback` (extends core) |

### 4.2 Required Fields

Profiles MAY declare additional required fields on core objects. Example:

```yaml
# Narrative profile — additional requirements
entity:
  when_type: character
  required:
    - structural_properties.role  # protagonist/antagonist/etc.
unit:
  when_type: scene
  required:
    - observables.context.focalizer  # who perceives
    - structure.beat                  # macro arc position
```

### 4.3 Additional Epistemic Sections

Profiles MAY define epistemic sections beyond the core three (observables/structure/interpretations). Each additional section MUST document:
- Name and purpose
- Whether fields require the interpreted_value wrapper
- Validation rules

Example: The narrative profile defines `craft_targets` (prescriptive authorial intent — target tension, pacing, tone).

### 4.4 Semantic Fingerprint Grammar

Each profile MUST define a `render` and `parse` contract for `unit.structure.semantic_fingerprint`:
- `render(unit, entities) → string` — deterministic serialization
- `parse(string, entities) → unit_fragment` — reverse mapping
- Round-trip invariant: `parse(render(u, e), e) == u` for the covered fields

### 4.5 Enum Governance

Profile enums are versioned independently from the core protocol. Adding a value is a minor version bump; removing or renaming is a major version bump. The core protocol defines the governance rules; profiles own their enum registries.

### 4.6 Domain-Specific Validation Rules

Profiles MAY declare additional validation rules beyond core schema validation. These participate in the conformance level system:
- **Level 1** (Schema): Core + profile schema validation
- **Level 2** (Referential): All entity/unit references resolve; profile-specific cross-reference rules pass
- **Level 3** (Round-Trip): Semantic fingerprint round-trip invariant holds; profile-specific semantic constraints pass

### 4.7 Canonical Views

Profiles SHOULD declare which view types are canonical for their domain:

| Profile | Canonical Views |
|---------|----------------|
| Narrative | `entity_trajectory` (character arc), `chronological`, `causal_chain` |
| Software | `dependency_graph`, `sequence_diagram`, `deployment_view` |

---

## 5. Serialization

### 5.1 Primary Format

JSON. All core objects serialize as JSON objects. Field names use `snake_case`.

### 5.2 Corpus Layout

A CAP corpus is a directory containing:

```
{artifact_id}/
  artifact.json          # Artifact metadata + entity declarations
  units/
    {unit_id}.json       # One file per unit
  views/
    {view_id}.json       # One file per view (optional)
```

Profiles MAY define alternative layouts (e.g., the narrative profile's current layout of `registry.json` + `story_architecture.json` + `scenes/`).

### 5.3 References

All inter-object references use slugs (`^[a-z0-9_]+$`). References are local to the artifact — no cross-artifact references in v0.1.0.

---

## 6. Conformance

### 6.1 Levels

| Level | Name | Requirements |
|-------|------|-------------|
| 1 | Schema Conformant | Passes core JSON Schema validation; all required fields present; all type-registered values valid |
| 2 | Referentially Conformant | Level 1 + all entity/unit references resolve within the artifact; profile-specific cross-reference rules pass |
| 3 | Round-Trip Conformant | Level 2 + semantic fingerprint passes `parse(render(x)) == x`; profile-specific semantic constraints pass |

### 6.2 Severity

| Level | Meaning |
|-------|---------|
| ERROR | Document MUST be rejected |
| WARNING | Document may have issues; SHOULD be reviewed |
| INFO | Informational; no action required |

---

## 7. Adapter Interfaces

Adapters transform between real artifacts and the canonical representation. The core defines the interface contracts; profiles implement them.

### 7.1 Ingest Adapter

```
ingest(source_artifact, profile, config) → Artifact
```

Transforms a raw artifact (prose text, codebase, API trace) into a canonical CAP Artifact.

### 7.2 Render Adapter

```
render(artifact, view_type, config) → rendered_output
```

Produces a domain-specific rendering from a canonical Artifact (prose, code skeleton, documentation, diagram).

### 7.3 Diff Interface

```
diff(artifact_a, artifact_b, layer) → DiffResult
```

Compares two artifacts at a specific epistemic layer:
- `layer: observables` — are the same facts present?
- `layer: structure` — is the organization preserved?
- `layer: interpretations` — do the inferences align?

### 7.4 Validate Interface

```
validate(artifact, level) → ValidationResult[]
```

Validates an artifact at the specified conformance level.

---

## 8. Normative Requirements

The key words "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD", "SHOULD NOT", "RECOMMENDED", "MAY", and "OPTIONAL" in this document are to be interpreted as described in RFC 2119.

1. Every object in a CAP artifact MUST organize its fields into the three core epistemic sections (observables, structure, interpretations) where applicable.
2. Observable fields MUST NOT carry the interpreted_value wrapper.
3. Interpretation fields MAY carry the interpreted_value wrapper (`{value, confidence, source}`).
4. All entity and unit references MUST use slugs that resolve within the parent artifact.
5. Profile-defined enum values MUST NOT appear in core objects without a profile being declared.
6. The `transition.description` field (delta) MUST state a change, not a condition.
7. If a `semantic_fingerprint` is provided, the active profile MUST implement the `render`/`parse` round-trip contract.
8. Conformance level claims MUST be validated — a document MUST NOT claim Level 3 without passing the round-trip test.
