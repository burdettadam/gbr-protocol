# GBR Protocol Versioning Policy

GBR uses [Semantic Versioning 2.0.0](https://semver.org/). The version string is `MAJOR.MINOR.PATCH`.

**Current version: GBR 0.1.0**

---

## Version Components

### MAJOR — Breaking Changes

A major version increment signals that existing GBR documents may not be valid against the new version without migration.

Breaking changes include:
- Removing a required field from any schema
- Renaming a required field
- Removing an enum value that is in use
- Changing the semantics of a required field in an incompatible way
- Restructuring the Canonical Summary format

The `$schema` URI includes the major version: `https://gbr-protocol.dev/schemas/v1/scene-card.schema.json`. Documents pinned to an older major version remain valid against that version's schemas.

### MINOR — Additive Changes

A minor version increment adds new capability without breaking existing documents.

Additive changes include:
- Adding a new optional field to a schema
- Adding a new enum value
- Adding a new document type
- Adding or clarifying conformance requirements that do not invalidate existing valid documents

### PATCH — Non-breaking Clarifications

A patch version increment fixes bugs or clarifies without changing behavior.

Patch changes include:
- Correcting errors in documentation
- Improving definition clarity without changing meaning
- Fixing typos in enum definitions
- Updating examples

---

## Pre-1.0 Policy

While the version is 0.x.y:
- MINOR increments may include breaking changes (with notice in CHANGELOG)
- Stability guarantees are best-effort only
- The protocol is in active design and may change significantly

The 1.0.0 release will signal that the protocol is stable and that the full SemVer guarantee applies.

---

## Version in Documents

GBR documents reference the protocol version via the `$schema` URI. The URI format is:

```
https://gbr-protocol.dev/schemas/v{MAJOR}/{document-type}.schema.json
```

During 0.x development, the `$schema` URI uses `v0`:

```
https://gbr-protocol.dev/schemas/v0/scene-card.schema.json
```

---

## Deprecation Process

Before removing a field or enum value:
1. Mark as deprecated in the documentation and schema (add `"deprecated": true`)
2. Issue a MINOR version increment announcing the deprecation
3. Wait at least one MINOR version before removing in a MAJOR version

---

## Changelog Requirement

Every version increment requires a CHANGELOG.md entry. The entry must include:
- Version number and date
- Category: Breaking / Added / Changed / Fixed / Deprecated / Removed
- Description of each change
- Migration notes for breaking changes
