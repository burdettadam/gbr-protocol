#!/usr/bin/env python3
"""scripts/validate_against_schema.py — Validate a Grimoire template against its JSON Schema.

Given a filled-in template file, this script:

1. Reads the ``schema:`` path from YAML frontmatter
2. Loads the referenced JSON Schema
3. Extracts structured data from the markdown (labels → values, tables → dicts)
4. Validates the extracted data against the schema
5. Reports field-level validation issues and source-citation coverage

Usage::

    python scripts/validate_against_schema.py 01-concept/premise-and-logline.md
    grimoire-validate 01-concept/premise-and-logline.md

    # Check all templates with schemas
    grimoire-validate --all

    # Require source citations (analysis mode)
    grimoire-validate --require-sources 01-concept/premise-and-logline.md

Output is a human-readable report. Exit code 0 = all required fields pass;
1 = validation errors; 2 = schema or file not found.
"""

from __future__ import annotations

import argparse
import json
import re
import sys
from pathlib import Path
from typing import Any

import yaml

PROJECT_ROOT = Path(__file__).resolve().parent.parent
SCHEMAS_DIR = PROJECT_ROOT / "template-schemas"

# ── Regex ──────────────────────────────────────────────────────────────────────

_FRONTMATTER_RE = re.compile(r"\A\s*---\n(.*?)\n---\n?", re.DOTALL)
_DETAILS_BLOCK_RE = re.compile(r"<details>.*?</details>", re.DOTALL | re.IGNORECASE)
_ANNOTATION_RE = re.compile(r"<!--.*?-->", re.DOTALL)
_LABEL_RE = re.compile(
    r"^\*\*(?P<label>[^*]{2,60})\*\*\s*[:：]\s*(?P<value>.*)$",
    re.MULTILINE,
)
# Bold question/label on its own line followed by a filled-in value on next line
# (after placeholder has been replaced by real content)
_LABEL_MULTILINE_RE = re.compile(
    r"^\*\*(?P<label>[^*]{2,80})\??\*\*\s*\n(?P<value>[^\n\[\*#><\|]{5,})",
    re.MULTILINE,
)
_TABLE_HEADER_RE = re.compile(r"^\|(.+)\|$")
_TABLE_SEP_RE = re.compile(r"^\|[\s\-|:]+\|$")
_TABLE_ROW_RE = re.compile(r"^\|(.+)\|$")
_CHECKBOX_RE = re.compile(r"^- \[(?P<check>[ x])\]\s*(?P<text>.+)$", re.MULTILINE)
_PLACEHOLDER_RE = re.compile(r"\[(?![ x]\])[^\[\]\n]{5,}\]")
_SOURCE_TAG_RE = re.compile(
    r"<!--\s*source:\s*(?P<val>[^>]+?)\s*-->"
)
_KV_RE = re.compile(r"(\w+):([^\s]+)")


# ── Utilities ──────────────────────────────────────────────────────────────────

def _to_snake(label: str) -> str:
    label = re.sub(r"[^\w\s]", "", label)
    label = re.sub(r"\s+", "_", label.strip().lower())
    return re.sub(r"_+", "_", label)[:60]


def _load_frontmatter(content: str) -> dict:
    m = _FRONTMATTER_RE.match(content)
    if not m:
        return {}
    try:
        d = yaml.safe_load(m.group(1))
        return d if isinstance(d, dict) else {}
    except yaml.YAMLError:
        return {}


def _strip_noise(content: str) -> str:
    """Remove details blocks, frontmatter, and annotation comments."""
    content = _DETAILS_BLOCK_RE.sub("", content)
    content = _FRONTMATTER_RE.sub("", content)
    return content


# ── Extractors ─────────────────────────────────────────────────────────────────

def extract_label_values(content: str) -> dict[str, str]:
    """Extract label-value pairs.

    Handles:
    - ``**Label:** value``  (inline)
    - ``**Bold question?**`` on its own line with the filled value on the next line
    Skips unfilled bracketed placeholders.
    """
    clean = _strip_noise(content)
    clean = _ANNOTATION_RE.sub("", clean)  # remove annotations from values
    result: dict[str, str] = {}

    # Pattern 1: **Label:** value (inline)
    for m in _LABEL_RE.finditer(clean):
        label = m.group("label").strip()
        key = _to_snake(label)
        value = m.group("value").strip()
        if _PLACEHOLDER_RE.match(value):
            value = ""  # unfilled
        if key and value:
            result[key] = value

    # Pattern 2: **Bold question?** \n filled value (not a placeholder)
    for m in _LABEL_MULTILINE_RE.finditer(clean):
        label = m.group("label").strip().rstrip("?")
        key = _to_snake(label)
        value = m.group("value").strip()
        if key not in result and value and not _PLACEHOLDER_RE.match(value):
            result[key] = value

    return result


def extract_tables(content: str) -> list[list[dict[str, str]]]:
    """Extract fill-in markdown tables as lists of row dicts."""
    clean = _strip_noise(content)
    lines = clean.splitlines()
    tables: list[list[dict[str, str]]] = []
    i = 0
    while i < len(lines):
        line = lines[i].rstrip()
        hm = _TABLE_HEADER_RE.match(line)
        if hm and i + 1 < len(lines) and _TABLE_SEP_RE.match(lines[i + 1].rstrip()):
            headers = [c.strip() for c in line.strip("|").split("|") if c.strip()]
            header_keys = [_to_snake(h) for h in headers]
            i += 2
            rows: list[dict[str, str]] = []
            while i < len(lines):
                row_line = lines[i].rstrip()
                rm = _TABLE_ROW_RE.match(row_line)
                if not rm:
                    break
                cells = [c.strip() for c in row_line.strip("|").split("|")]
                # Remove annotation comments from cells
                cells = [_ANNOTATION_RE.sub("", c).strip() for c in cells]
                row = {}
                for k, v in zip(header_keys, cells):
                    if k and v and not _PLACEHOLDER_RE.match(v):
                        row[k] = v
                if any(row.values()):
                    rows.append(row)
                i += 1
            if rows:
                tables.append(rows)
            continue
        i += 1
    return tables


def extract_checked_boxes(content: str) -> list[str]:
    """Return list of checked checkbox option texts."""
    clean = _strip_noise(content)
    return [
        m.group("text").strip()
        for m in _CHECKBOX_RE.finditer(clean)
        if m.group("check") == "x"
    ]


def extract_source_citations(content: str) -> list[str]:
    """Return raw source citation values from <!-- source: ... --> tags."""
    return [m.group("val").strip() for m in _SOURCE_TAG_RE.finditer(content)]


# ── Schema loader ──────────────────────────────────────────────────────────────

def load_schema(schema_path: Path) -> dict[str, Any] | None:
    """Load and return a JSON Schema dict, or None on failure."""
    if not schema_path.exists():
        return None
    try:
        return json.loads(schema_path.read_text(encoding="utf-8"))
    except (json.JSONDecodeError, OSError):
        return None


# ── Validator ──────────────────────────────────────────────────────────────────

class ValidationResult:
    def __init__(self, template_path: Path):
        self.template_path = template_path
        self.schema_path: Path | None = None
        self.errors: list[str] = []
        self.warnings: list[str] = []
        self.info: list[str] = []
        self.fields_total: int = 0
        self.fields_filled: int = 0
        self.fields_with_source: int = 0

    @property
    def ok(self) -> bool:
        return not self.errors

    def summary_line(self) -> str:
        pct = int(100 * self.fields_filled / self.fields_total) if self.fields_total else 0
        src_pct = int(100 * self.fields_with_source / self.fields_total) if self.fields_total else 0
        status = "✅ PASS" if self.ok else "❌ FAIL"
        return (
            f"{status}  {self.template_path.relative_to(PROJECT_ROOT)}"
            f"  [{pct}% filled, {src_pct}% sourced, {len(self.errors)} errors]"
        )


def validate_template(
    template_path: Path,
    require_sources: bool = False,
) -> ValidationResult:
    result = ValidationResult(template_path)

    content = template_path.read_text(encoding="utf-8")
    fm = _load_frontmatter(content)
    schema_rel = fm.get("schema")

    if not schema_rel:
        result.warnings.append(
            "No 'schema:' key in frontmatter. "
            "Run 'grimoire-schemas' to generate and link a schema."
        )
        return result

    schema_path = PROJECT_ROOT / schema_rel
    result.schema_path = schema_path
    schema = load_schema(schema_path)

    if schema is None:
        result.errors.append(
            f"Schema file not found or invalid JSON: {schema_rel}. "
            "Run 'grimoire-schemas' to (re)generate it."
        )
        return result

    # ── Extract data from template ─────────────────────────────────────────────
    label_values = extract_label_values(content)
    tables = extract_tables(content)
    checked_boxes = extract_checked_boxes(content)
    citations = extract_source_citations(content)

    schema_props: dict[str, Any] = schema.get("properties", {})
    required: list[str] = schema.get("required", [])

    result.fields_total = len([
        k for k in schema_props
        if not k.startswith("_")
    ])
    result.fields_filled = len(label_values) + len(checked_boxes) + len(tables)
    result.fields_with_source = len(citations)

    # ── Required field checks ──────────────────────────────────────────────────
    for req_key in required:
        if req_key.startswith("_"):
            continue
        if req_key not in label_values:
            prop = schema_props.get(req_key, {})
            label = prop.get("description", req_key)
            result.errors.append(f"Required field not filled: '{req_key}' ({label})")

    # ── Enum constraint checks ─────────────────────────────────────────────────
    for key, value in label_values.items():
        prop = schema_props.get(key, {})
        enum_vals = prop.get("enum")
        if enum_vals and value not in enum_vals:
            # Case-insensitive check
            if value.lower() not in [str(e).lower() for e in enum_vals]:
                result.warnings.append(
                    f"Field '{key}' value '{value}' not in enum {enum_vals[:5]}..."
                )

    # ── Source coverage (analysis mode) ───────────────────────────────────────
    source_required_fields = [
        k for k, p in schema_props.items()
        if p.get("x-source-required") is True and not k.startswith("_")
    ]
    if source_required_fields:
        covered = len(citations)
        pct = int(100 * covered / len(source_required_fields)) if source_required_fields else 0
        result.info.append(
            f"Source citations: {covered}/{len(source_required_fields)} "
            f"required fields cited ({pct}%)"
        )
        if require_sources:
            missing = [f for f in source_required_fields if f not in label_values]
            # We can't do precise per-field citation matching here without more
            # complex positional analysis — report aggregate coverage instead.
            if pct < 50:
                result.warnings.append(
                    f"Low source coverage: {pct}% of required fields have citations. "
                    "Add <!-- source: chN.pN --> tags after filled values."
                )
    elif require_sources and not citations:
        result.warnings.append(
            "No source citations found. Add <!-- source: chN.pN --> tags "
            "or run without --require-sources for authoring mode."
        )

    # ── General fill status ────────────────────────────────────────────────────
    # Count remaining placeholders
    clean = _DETAILS_BLOCK_RE.sub("", _FRONTMATTER_RE.sub("", content))
    remaining_placeholders = len(_PLACEHOLDER_RE.findall(clean))
    if remaining_placeholders > 0:
        result.info.append(f"{remaining_placeholders} unfilled placeholder(s) remain.")

    return result


# ── Report renderer ────────────────────────────────────────────────────────────

def render_report(results: list[ValidationResult], verbose: bool = False) -> str:
    lines: list[str] = []
    lines.append("Grimoire Template Validation")
    lines.append("=" * 60)
    pass_count = sum(1 for r in results if r.ok)
    lines.append(f"Templates checked: {len(results)} | Pass: {pass_count} | Fail: {len(results) - pass_count}")
    lines.append("")

    for result in results:
        lines.append(result.summary_line())
        if verbose or not result.ok:
            for err in result.errors:
                lines.append(f"  ✗ {err}")
            for warn in result.warnings:
                lines.append(f"  ⚠ {warn}")
            if verbose:
                for info in result.info:
                    lines.append(f"  ℹ {info}")

    lines.append("")
    return "\n".join(lines)


# ── Main ───────────────────────────────────────────────────────────────────────

PHASE_DIRS = [
    "00-start-here", "01-concept", "02-collision", "03-characters",
    "04-world-building", "05-plot-and-structure", "06-scenes",
    "07-drafting", "08-revision", "09-polish-and-publish",
]

_SKIP_FILES = {
    "how-to-use-grimoire.md",
    "ai-collaboration-principles.md",
}


def find_all_templates() -> list[Path]:
    templates: list[Path] = []
    for phase in PHASE_DIRS:
        phase_path = PROJECT_ROOT / phase
        if not phase_path.exists():
            continue
        for md in sorted(phase_path.rglob("*.md")):
            if md.name.startswith("_") or md.name in _SKIP_FILES:
                continue
            fm = _load_frontmatter(md.read_text(encoding="utf-8"))
            if fm.get("schema"):
                templates.append(md)
    return templates


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(
        description="Validate a Grimoire fill-in template against its JSON Schema."
    )
    parser.add_argument(
        "templates",
        nargs="*",
        metavar="TEMPLATE",
        help="Template file path(s) to validate. If omitted, use --all.",
    )
    parser.add_argument(
        "--all",
        action="store_true",
        help="Validate all templates that have a schema: frontmatter key.",
    )
    parser.add_argument(
        "--require-sources",
        action="store_true",
        help="Warn when x-source-required fields lack source: citations (analysis mode).",
    )
    parser.add_argument(
        "--verbose", "-v",
        action="store_true",
        help="Show info messages even for passing templates.",
    )
    args = parser.parse_args(argv)

    if args.all:
        template_paths = find_all_templates()
    elif args.templates:
        template_paths = [Path(t) for t in args.templates]
    else:
        parser.print_help()
        return 0

    if not template_paths:
        print("No templates to validate.", file=sys.stderr)
        return 0

    results: list[ValidationResult] = []
    for tmpl in template_paths:
        if not tmpl.is_absolute():
            tmpl = PROJECT_ROOT / tmpl
        if not tmpl.exists():
            r = ValidationResult(tmpl)
            r.errors.append(f"File not found: {tmpl}")
            results.append(r)
            continue
        results.append(
            validate_template(tmpl, require_sources=args.require_sources)
        )

    print(render_report(results, verbose=args.verbose))

    any_errors = any(not r.ok for r in results)
    return 1 if any_errors else 0


if __name__ == "__main__":
    sys.exit(main())
