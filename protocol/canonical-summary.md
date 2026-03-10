# Canonical Summary

The Canonical Summary is the protocol's mechanism for ensuring lossless round-trip between semantic structure and prose. It is a deterministic, fixed-grammar string that serializes the semantic content of a Scene Card.

For normative rules, see [SPECIFICATION.md §9](../SPECIFICATION.md#9-canonical-summary).

---

## The Problem

Human-authored prose summaries are ambiguous by construction:

```
# Human prose summary (NOT valid in GBR)
"Elizabeth refuses Darcy's proposal. The atmosphere is tense and the
confrontation exposes their mutual misunderstanding."
```

This cannot be reliably parsed back to structure:
- Event type: `confrontation`? `refusal`? `revelation`?
- Want outcome: `DENIED`? `PYRRHIC`?
- Causal role: `PIVOTS` or `ESCALATES`?

**A summary that cannot be parsed back to its structured source is not a GBR canonical summary.**

---

## The Template

```
{POV_CHAR} {EVENT_VERB} {PARTICIPANTS} at {LOCATION}; 
wants {WANT_OBJECT} [{OUTCOME}]; 
stakes={STAKES}, atmosphere={ATMOSPHERE}, role={CAUSAL_ROLE}.
```

### Slot Definitions

| Slot | Schema Field | Render Rule |
|------|-------------|-------------|
| `{POV_CHAR}` | `focalizer` | `registry.characters[slug].name` |
| `{EVENT_VERB}` | `event_type` | `EVENT_VERBS[event_type]` |
| `{PARTICIPANTS}` | `participants[]` | Comma-joined display names |
| `{LOCATION}` | `setting` | `registry.settings[slug].name` |
| `{WANT_OBJECT}` | `character_want` | `registry.want_vocabulary[slug]` |
| `{OUTCOME}` | `want_outcome` | `GRANTED`, `DENIED`, `DEFERRED`, `PYRRHIC` |
| `{STAKES}` | `stakes_domain` | Enum value |
| `{ATMOSPHERE}` | `atmosphere` | Enum value |
| `{CAUSAL_ROLE}` | `causal_role` | `ESTABLISHES`, `ESCALATES`, `PIVOTS`, `RESOLVES` |

---

## Example

**Semantic Structure:**
```json
{
  "focalizer": "elizabeth_bennet",
  "event_type": "refusal",
  "participants": ["fitzwilliam_darcy"],
  "setting": "hunsford_parsonage",
  "character_want": "honest_respect",
  "want_outcome": "denied",
  "stakes_domain": "social",
  "atmosphere": "tense",
  "causal_role": "pivots"
}
```

**Canonical Summary:**
```
Elizabeth Bennet refuses Fitzwilliam Darcy at Hunsford Parsonage; 
wants honest respect [DENIED]; stakes=social, atmosphere=tense, role=PIVOTS.
```

---

## Round-Trip Contract

Two functions implement the canonical summary:

```
render_summary(semantic_dict, registry) → string
parse_summary(string, registry) → semantic_dict
```

The invariant that MUST hold:

```
parse_summary(render_summary(d, r), r) == d
```

---

## Event Type Verb Mapping

| `event_type` | Verb Phrase |
|---|---|
| `arrival` | arrives with |
| `departure` | departs from |
| `confrontation` | confronts |
| `confession` | confesses to |
| `discovery` | discovers |
| `decision` | decides |
| `proposal` | proposes to |
| `refusal` | refuses |
| `acceptance` | accepts |
| `betrayal` | betrays |
| `reconciliation` | reconciles with |
| `revelation` | reveals to |
| `deception` | deceives |
| `seduction` | seduces |
| `negotiation` | negotiates with |
| `escape` | escapes from |
| `pursuit` | pursues |
| `rescue` | rescues |
| `loss` | loses |
| `transformation` | transforms at |

---

## Validity Rules

A Canonical Summary is valid if and only if:

1. It was produced by `render_summary` — manually written summaries that would produce a different output are invalid.
2. Every slug resolves against the book's Entity Registry.
3. `parse_summary` applied to the string returns the original semantic structure.

If a `canonical_summary` field is present in a Scene Card, it MUST satisfy all three conditions. Validators MUST reject Scene Cards with invalid canonical summaries.

---

## What Canonical Summaries Are Not

A canonical summary is **not** a prose synopsis, **not** a chapter summary, and **not** a description of the scene for human readers. It is a serialized semantic fingerprint whose purpose is machine verification. Its grammar is deliberately mechanical and non-literary.
