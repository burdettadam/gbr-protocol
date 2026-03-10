# Grimoire Extraction Schemas

This directory contains JSON Schema files that mirror every Grimoire fill-in template. They serve two purposes:

1. **LLM structured output** — when an AI reads an existing book and fills in Grimoire templates as critique or analysis, these schemas define the expected output shape for each template.
2. **Validation** — `grimoire-validate` uses these schemas to check that filled-in templates have the correct field types, valid enum values, and (in analysis mode) source citations for every evidence-backed field.

---

## Directory Structure

```
schemas/
  _base.schema.json              ← shared $defs (enums, base types, source citation)
  README.md                      ← this file
  00-start-here/
    book-brief.schema.json
    author-journal.schema.json
  01-concept/
    premise-and-logline.schema.json
    theme-exploration.schema.json
    ...
  03-characters/
    core-identity/
      cast-overview.schema.json
      character-profile.schema.json
      character-arc.schema.json
    relationships/
      character-relationships.schema.json
    antagonist/
      antagonist-design.schema.json
    cross-refs/
      actantial-map.schema.json
  ...
```

The structure mirrors the template directory exactly. Each `.schema.json` lives beside (in the same relative path under `schemas/`) its corresponding fill-in template.

---

## Linking a Template to Its Schema

Every fill-in template declares its schema via a YAML frontmatter key:

```yaml
---
schema: schemas/01-concept/premise-and-logline.schema.json
---
```

Run `grimoire-schemas` to generate all schemas and inject this key automatically. Run `grimoire-schemas --force` to regenerate existing schemas.

---

## Base Schema (`_base.schema.json`)

All per-template schemas reference shared definitions from `_base.schema.json` via `$ref: "grimoire:base#/$defs/<name>"`. Key definitions:

| `$def` name | Type | Description |
|---|---|---|
| `labelValue` | `string` | A `**Label:** [placeholder]` fill-in field |
| `longText` | `string` | Multi-sentence freeform paragraph |
| `entityRef` | `string` | Reference to a declared character/setting/beat entity |
| `sourceCitation` | `string` | A passage citation (see below) |
| `sourceTextMeta` | `object` | Source text metadata (`title`, `author`, `edition`, `year`) |
| `checkboxGroup` | `array` | Multi-select checkbox group |
| `conditionalField` | `object` | Checkbox + detail fill-in (yes/no + explanation) |
| `arcTypeEnum` | `string` enum | Character arc types |
| `roleEnum` | `string` enum | Cast role types |
| `archetypeEnum` | `string` enum | Jungian/story archetypes |
| `alignmentEnum` | `string` enum | Moral alignment grid |
| `woundEnum` | `string` enum | Character wound catalog |
| `driveModelEnum` | `string` enum | 5 character drive models |
| `annotations` | `object` | Extracted `<!-- key:value -->` annotation tags |

---

## Custom Extension Keywords

These `x-` keywords appear in per-template schemas and are understood by `grimoire-validate`:

| Keyword | Values | Meaning |
|---|---|---|
| `x-source-required` | `true` / `false` | When `true`, this field should have a `<!-- source: ... -->` citation in analysis mode |
| `x-entity-ref` | `true` | This field references a declared Grimoire entity (character, setting, beat). Value should match a known entity slug. |
| `x-analysis-only` | `true` | This field is only relevant when using the template for book analysis, not authoring. |
| `x-todo` | `string` | Human review needed — type or constraints were auto-generated and should be verified. |
| `x-grimoire-template` | `string` | Relative path of the template file this schema was generated from. |

---

## Source Citation Convention

When using a Grimoire template to analyze an **existing book**, every filled field that draws on a specific passage should be annotated with a `<!-- source: ... -->` tag.

### Declaring the source text

Place once at the top of the document:

```markdown
<!-- source_text: title="Pride and Prejudice" author="Jane Austen" edition="Penguin 2003" year=1813 -->
```

### Citing a passage

Place immediately after the filled value:

```markdown
**Protagonist:** Elizabeth Bennet <!-- source: ch1.p3 "a girl of light and pleasing figure" -->
```

### Citation grammar

```
ch{N}[.p{N} | .pg{N}][" quote fragment"][, ch{N}...]
```

| Component | Syntax | Example |
|---|---|---|
| Chapter | `ch{N}` | `ch3` |
| Paragraph | `.p{N}` | `ch3.p12` |
| Page number | `.pg{N}` | `ch3.pg47` |
| Quote snippet | `"text"` | `ch3.p12 "door swung"` |
| Multiple locations | `,` | `ch3.p4, ch7.p18` |

### Validating citation coverage

```bash
grimoire-validate --require-sources 01-concept/premise-and-logline.md
```

This checks that fields marked `x-source-required: true` in the schema have corresponding `source:` tags.

---

## CLI Commands

```bash
# Generate / refresh all schemas and inject schema: frontmatter into templates
grimoire-schemas

# Generate only for one phase
grimoire-schemas --phase 01-concept

# Preview without writing files
grimoire-schemas --dry-run

# Regenerate existing schemas (overwrite)
grimoire-schemas --force

# Validate a filled-in template
grimoire-validate 01-concept/premise-and-logline.md

# Validate all templates with schemas
grimoire-validate --all

# Analysis mode: also require source citations
grimoire-validate --require-sources --all

# Verbose output (show info for passing templates too)
grimoire-validate --verbose 03-characters/core-identity/cast-overview.md
```

---

## Workflow: Analyzing an Existing Book

1. Copy the relevant fill-in templates for the aspects you want to analyze.
2. Add `<!-- source_text: title="..." author="..." -->` at the top of each.
3. Fill in each field, adding `<!-- source: chN.pN -->` citations after each value.
4. Run `grimoire-validate --require-sources <template.md>` to check coverage.
5. Use the filled schemas as structured input for LLM fine-tuning or critique workflows.

---

## Schema Generation Notes

Schemas are **auto-generated starting points**. After generation, review and refine:

- Fields marked `"x-todo": "review type"` — verify the inferred type is correct
- Add `"x-source-required": true` to fields that must be evidence-backed in analysis mode
- Add `"x-entity-ref": true` to fields that reference characters, settings, or beats
- Tighten enum constraints where the template uses a controlled vocabulary
- Add `"required": [...]` arrays to enforce mandatory fields in gate checks
