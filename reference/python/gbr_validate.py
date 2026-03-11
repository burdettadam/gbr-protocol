#!/usr/bin/env python3
"""
gbr_validate.py — GBR Protocol conformance validator.

Usage:
    python gbr_validate.py scene-card   path/to/scene.json
    python gbr_validate.py scene-card   path/to/scenes/ --registry registry.json
    python gbr_validate.py registry     path/to/registry.json
    python gbr_validate.py story        path/to/story-architecture.json
    python gbr_validate.py sip-artifact path/to/artifact.json [--level {1,2,3}]

Flags:
    --registry <path>   Registry JSON for Level 2 entity-ref checks (GBR only)
    --level {1,2,3}     Maximum conformance level to check (default: 1)
    --strict            Treat Level 3 advisory warnings as blocking failures

Conformance levels for sip-artifact:
    1  JSON Schema (artifact.schema.json + all SIP sub-schemas)
    2  Referential integrity (all entity_refs resolve to declared entity_ids)
    3  Semantic (step sequence_number ordering, protocol value)

Exit codes:
    0   all requested levels pass
    1   one or more conformance failures
    2   file or schema not found
"""

import argparse
import json
import pathlib
import sys

try:
    from jsonschema import Draft7Validator
except ImportError:
    sys.exit("Install jsonschema: pip install jsonschema")

try:
    from referencing import Registry, Resource
    from referencing.jsonschema import DRAFT7
    _HAS_REFERENCING = True
except ImportError:
    from jsonschema import RefResolver  # type: ignore[attr-defined]
    _HAS_REFERENCING = False

# Known base URL prefixes used by GBR schema $id fields.
# Pre-populating the resolver store with all prefix+filename combinations
# lets offline $ref resolution work regardless of which $id style a schema uses.
VERSION = "0.2.0"

_KNOWN_BASE_URLS = [
    "https://grimoire.dev/protocol/v1/",
    "https://gbr-protocol.dev/schemas/v0/",
]

# ── Constants ────────────────────────────────────────────────────────────────

REPO_ROOT = pathlib.Path(__file__).resolve().parent.parent.parent
SCHEMAS_DIR = REPO_ROOT / "schemas"
# SIP schemas live in the standalone sip-protocol repo (sibling of gbr-protocol).
# Fall back to the old in-tree location to keep older checkouts working.
_SIP_PROTOCOL_DIR = REPO_ROOT.parent / "sip-protocol"
SIP_SCHEMAS_DIR = (
    _SIP_PROTOCOL_DIR / "schemas"
    if (_SIP_PROTOCOL_DIR / "schemas").is_dir()
    else REPO_ROOT / "docs" / "sip" / "schemas"
)

SCHEMA_MAP = {
    "registry": "registry.schema.json",
    "scene-card": "scene-card.schema.json",
    "story": "story-architecture.schema.json",
    "character-state": "character-state.schema.json",
}

# Level 3: generic verbs whose presence in scene_turns indicates low specificity
GENERIC_VERBS = {"moves", "goes", "does", "makes", "says", "gets", "puts", "walks", "comes"}

# ── Helpers ──────────────────────────────────────────────────────────────────

def load_json(path: pathlib.Path) -> dict:
    """Load a JSON file; exit with code 2 on missing or parse error."""
    if not path.exists():
        _fatal(f"File not found: {path}", code=2)
    try:
        with open(path, encoding="utf-8") as f:
            return json.load(f)
    except json.JSONDecodeError as exc:
        _fatal(f"JSON parse error in {path}: {exc}", code=2)


def load_schema(doc_type: str) -> dict:
    """Load the JSON Schema for doc_type; exit with code 2 if missing."""
    schema_file = SCHEMAS_DIR / SCHEMA_MAP.get(doc_type, "")
    if not schema_file.exists():
        _fatal(
            f"Schema not found for type '{doc_type}': {schema_file}\n"
            f"  Schemas dir: {SCHEMAS_DIR}",
            code=2,
        )
    return load_json(schema_file)


def _fatal(msg: str, code: int = 1) -> None:
    print(f"ERROR: {msg}", file=sys.stderr)
    sys.exit(code)


# ── SIP schema loader ───────────────────────────────────────────────────────

def _build_sip_registry_or_resolver(
    schema_file: "pathlib.Path",
) -> "tuple[object, object] | tuple[None, object]":
    """Return (registry, validator) or raise on missing schema."""
    schema = load_json(schema_file)
    if _HAS_REFERENCING:
        resources: list[tuple[str, object]] = []
        for sf in SIP_SCHEMAS_DIR.glob("*.schema.json"):
            if not sf.is_file():
                continue
            try:
                s = json.loads(sf.read_text(encoding="utf-8"))
            except Exception:
                continue
            schema_id = s.get("$id", "")
            if schema_id:
                resources.append((schema_id, Resource.from_contents(s, default_specification=DRAFT7)))
            resources.append((sf.as_uri(), Resource.from_contents(s, default_specification=DRAFT7)))
            # Bare filename allows $ref: "entity.schema.json" (relative-style)
            resources.append((sf.name, Resource.from_contents(s, default_specification=DRAFT7)))
        registry = Registry().with_resources(resources)
        return Draft7Validator(schema, registry=registry)
    else:
        store: dict[str, dict] = {}
        for sf in SIP_SCHEMAS_DIR.glob("*.schema.json"):
            if not sf.is_file():
                continue
            try:
                s = json.loads(sf.read_text(encoding="utf-8"))
            except Exception:
                continue
            store[sf.as_uri()] = s
            schema_id = s.get("$id", "")
            if schema_id:
                store[schema_id] = s
            store[sf.name] = s
        resolver = RefResolver(base_uri=schema_file.as_uri(), referrer=schema, store=store)  # type: ignore[attr-defined]
        return Draft7Validator(schema, resolver=resolver)


# ── Validator classes ─────────────────────────────────────────────────────────

class GBRValidator:
    """Multi-level GBR conformance checker for a single document."""

    def __init__(self, doc: dict, doc_type: str, registry: dict | None = None):
        self.doc = doc
        self.doc_type = doc_type
        self.registry = registry
        self.failures: list[tuple[int, str]] = []   # (level, message)
        self.warnings: list[tuple[int, str]] = []   # (level, message)

    # ── Level 1 ─────────────────────────────────────────────────────────────

    def level1_schema(self) -> bool:
        """Validate doc against its JSON Schema (Draft 7)."""
        schema = load_schema(self.doc_type)

        if _HAS_REFERENCING:
            # Modern approach: referencing.Registry (jsonschema >= 4.18)
            resources: list[tuple[str, object]] = []
            for schema_file in SCHEMAS_DIR.glob("*.schema.json"):
                if not schema_file.is_file():
                    continue
                try:
                    s = json.loads(schema_file.read_text(encoding="utf-8"))
                except Exception:
                    continue
                schema_id = s.get("$id", "")
                if schema_id:
                    resources.append((schema_id, Resource.from_contents(s, default_specification=DRAFT7)))
                resources.append((schema_file.as_uri(), Resource.from_contents(s, default_specification=DRAFT7)))
                for base in _KNOWN_BASE_URLS:
                    key = base + schema_file.name
                    if key != schema_id:
                        resources.append((key, Resource.from_contents(s, default_specification=DRAFT7)))
            registry = Registry().with_resources(resources)
            validator = Draft7Validator(schema, registry=registry)
        else:
            # Legacy approach: RefResolver with a pre-populated store
            base_uri = (SCHEMAS_DIR / SCHEMA_MAP[self.doc_type]).as_uri()
            store: dict[str, dict] = {}
            for schema_file in SCHEMAS_DIR.glob("*.schema.json"):
                if not schema_file.is_file():
                    continue
                try:
                    s = json.loads(schema_file.read_text(encoding="utf-8"))
                except Exception:
                    continue
                store[schema_file.as_uri()] = s
                schema_id = s.get("$id", "")
                if schema_id:
                    store[schema_id] = s
                for base in _KNOWN_BASE_URLS:
                    store[base + schema_file.name] = s
            resolver = RefResolver(base_uri=base_uri, referrer=schema, store=store)
            validator = Draft7Validator(schema, resolver=resolver)

        errors: list[object] = []
        try:
            error_iter = validator.iter_errors(self.doc)
            while True:
                try:
                    err = next(error_iter)  # type: ignore[call-overload]
                    errors.append(err)
                except StopIteration:
                    break
                except Exception as ref_exc:
                    # Schema $ref resolution failed (e.g. pointer to missing $def,
                    # typically a schema-version mismatch).  Report as a warning
                    # and continue — we still surface whatever errors were found.
                    self.warnings.append((1, f"$ref resolution warning (schema version mismatch): {ref_exc}"))
                    break
        except Exception as exc:
            self.warnings.append((1, f"Schema validation could not run: {exc}"))
            return True  # treat as pass so Level 2+ checks still run

        errors = sorted(errors, key=lambda e: list(e.absolute_path))  # type: ignore[attr-defined]
        for err in errors:
            path_str = " → ".join(str(p) for p in err.absolute_path) or "(root)"  # type: ignore[attr-defined]
            self.failures.append((1, f"Schema: [{path_str}] {err.message}"))  # type: ignore[attr-defined]
        return len(self.failures) == 0

    # ── Level 2 ─────────────────────────────────────────────────────────────

    def level2_entity_refs(self) -> bool:
        """Resolve entity references against a loaded registry."""
        if self.registry is None:
            self.warnings.append((2, "No registry provided — Level 2 skipped"))
            return True

        reg_chars = set(self.registry.get("characters", {}).keys())
        reg_settings = set(self.registry.get("settings", {}).keys())
        ok = True

        if self.doc_type == "scene-card":
            ok = ok and self._check_scene_card_refs(reg_chars, reg_settings)
        elif self.doc_type == "character-state":
            ok = ok and self._check_character_state_refs(reg_chars)
        elif self.doc_type == "story":
            ok = ok and self._check_story_refs(reg_chars, reg_settings)

        return ok

    def _check_scene_card_refs(self, reg_chars: set, reg_settings: set) -> bool:
        ok = True
        obs = self.doc.get("observables", {})
        struct = self.doc.get("structure", {})

        # observables.focalizer
        focalizer = obs.get("focalizer")
        if focalizer and focalizer not in reg_chars:
            self.failures.append((2, f"Ref: 'observables.focalizer' → '{focalizer}' not in registry.characters"))
            ok = False

        # observables.setting_instance.location
        location = (obs.get("setting_instance") or {}).get("location")
        if location and location not in reg_settings:
            self.failures.append((2, f"Ref: 'observables.setting_instance.location' → '{location}' not in registry.settings"))
            ok = False

        # character_states[].observables.character
        for i, cs in enumerate(self.doc.get("character_states", [])):
            cid = (cs.get("observables") or {}).get("character")
            if cid and cid not in reg_chars:
                self.failures.append(
                    (2, f"Ref: 'character_states[{i}].observables.character' → '{cid}' not in registry.characters")
                )
                ok = False

        # structure.canonical_summary.scene_turns[].active_character
        turns = (struct.get("canonical_summary") or {}).get("scene_turns", [])
        for i, turn in enumerate(turns):
            ac = turn.get("active_character")
            if ac and ac not in reg_chars:
                self.failures.append(
                    (2, f"Ref: 'structure.canonical_summary.scene_turns[{i}].active_character' → '{ac}' not in registry.characters")
                )
                ok = False

        return ok

    def _check_character_state_refs(self, reg_chars: set) -> bool:
        ok = True
        cid = (self.doc.get("observables") or {}).get("character")
        if cid and cid not in reg_chars:
            self.failures.append((2, f"Ref: 'observables.character' → '{cid}' not in registry.characters"))
            ok = False
        return ok

    def _check_story_refs(self, reg_chars: set, reg_settings: set) -> bool:
        ok = True
        arc = (self.doc.get("interpretations") or {}).get("protagonist_arc", {})
        subject = arc.get("subject")
        if subject and subject not in reg_chars:
            self.failures.append((2, f"Ref: 'interpretations.protagonist_arc.subject' → '{subject}' not in registry.characters"))
            ok = False
        return ok

    # ── Level 3 ─────────────────────────────────────────────────────────────

    def level3_semantic(self) -> bool:
        """Advisory semantic-richness checks."""
        if self.doc_type != "scene-card":
            return True   # Level 3 only defined for scene cards

        ok = True
        struct = self.doc.get("structure", {})
        interps = self.doc.get("interpretations", {})
        summary = struct.get("canonical_summary") or {}
        if isinstance(summary, str):
            summary = {}  # prose-only summary — skip structural checks
        scene_id = self.doc.get("scene_id", "(unknown)")
        canonical_metrics = interps.get("canonical_metrics") or {}

        # delta length
        delta = summary.get("delta", "")
        if not delta or len(delta.strip()) < 20:
            self.warnings.append((3, f"delta is empty or too short (scene_id={scene_id})"))
            ok = False

        # iceberg_proportion range (now in interpretations.canonical_metrics)
        iceberg = canonical_metrics.get("iceberg_proportion")
        if iceberg is not None and not (0.3 <= iceberg <= 0.9):
            self.warnings.append(
                (3, f"iceberg_proportion={iceberg} outside recommended range [0.3, 0.9] (scene_id={scene_id})")
            )
            ok = False

        # generic verbs in structure.canonical_summary.scene_turns
        turns = summary.get("scene_turns", [])
        for i, turn in enumerate(turns):
            verb = (turn.get("verb") or "").lower()
            if verb in GENERIC_VERBS:
                self.warnings.append(
                    (3, f"generic verb '{verb}' in structure.canonical_summary.scene_turns[{i}] (scene_id={scene_id})")
                )
                ok = False

        # at least one kernel event (significance now in turn.interpretations)
        def _significance(turn: dict) -> str | None:
            turn_interps = turn.get("interpretations") or {}
            return turn_interps.get("significance") or turn.get("significance")

        has_kernel = any(_significance(t) == "kernel" for t in turns)
        if turns and not has_kernel:
            self.warnings.append((3, f"no kernel significance event in structure.canonical_summary.scene_turns (scene_id={scene_id})"))
            ok = False

        # masked_emotion ≠ emotional_state per turn (both now in turn.interpretations)
        for i, turn in enumerate(turns):
            turn_interps = turn.get("interpretations") or {}
            emotion = turn_interps.get("emotional_state") or turn.get("emotional_state")
            masked = turn_interps.get("masked_emotion") or turn.get("masked_emotion")
            if emotion and masked and emotion == masked:
                self.warnings.append(
                    (3, f"masked_emotion equals emotional_state ('{emotion}') in scene_turns[{i}] (scene_id={scene_id})")
                )
                ok = False

        return ok

    # ── Run ─────────────────────────────────────────────────────────────────

    def validate(self, level: int = 1, strict: bool = False) -> bool:
        """Run all checks up to *level*. Returns True iff conformant."""
        passed = True

        l1_ok = self.level1_schema()
        if not l1_ok:
            passed = False

        if level >= 2 and l1_ok:
            l2_ok = self.level2_entity_refs()
            if not l2_ok:
                passed = False

        if level >= 3 and l1_ok:
            l3_ok = self.level3_semantic()
            if not l3_ok and strict:
                passed = False

        return passed


# ── SIP Validator ───────────────────────────────────────────────────────────

class SIPValidator:
    """Multi-level conformance checker for a SIP artifact (sip-artifact type).

    Level 1 — JSON Schema: validates against sip-protocol/schemas/artifact.schema.json
               with all other SIP sub-schemas available for $ref resolution.
    Level 2 — Referential: all entity_ref values in units and relationships
               resolve to a declared entity_id in artifact.entities.
    Level 3 — Semantic: step sequence_numbers ascending + no duplicates per unit;
               protocol field must equal 'semantic-interaction-protocol'.
    """

    def __init__(self, doc: dict):
        self.doc = doc
        self.failures: list[tuple[int, str]] = []
        self.warnings: list[tuple[int, str]] = []

    # ── Level 1 ─────────────────────────────────────────────────────────────

    def level1_schema(self) -> bool:
        schema_file = SIP_SCHEMAS_DIR / "artifact.schema.json"
        if not schema_file.exists():
            _fatal(f"SIP schema not found: {schema_file}", code=2)

        try:
            json_validator = _build_sip_registry_or_resolver(schema_file)
        except Exception as exc:
            self.warnings.append((1, f"Could not build SIP validator: {exc}"))
            return True

        errors: list[object] = []
        try:
            error_iter = json_validator.iter_errors(self.doc)
            while True:
                try:
                    err = next(error_iter)  # type: ignore[call-overload]
                    errors.append(err)
                except StopIteration:
                    break
                except Exception as ref_exc:
                    self.warnings.append((1, f"$ref resolution warning: {ref_exc}"))
                    break
        except Exception as exc:
            self.warnings.append((1, f"Schema validation could not run: {exc}"))
            return True

        for err in sorted(errors, key=lambda e: list(e.absolute_path)):  # type: ignore[attr-defined]
            path_str = " → ".join(str(p) for p in err.absolute_path) or "(root)"  # type: ignore[attr-defined]
            self.failures.append((1, f"Schema: [{path_str}] {err.message}"))  # type: ignore[attr-defined]
        return len(self.failures) == 0

    # ── Level 2 ─────────────────────────────────────────────────────────────

    def level2_referential(self) -> bool:
        """Check all entity_ref values resolve to declared entity_ids."""
        ok = True
        declared_ids = {e.get("entity_id") for e in self.doc.get("entities", [])}

        def _check_ref(ref: object, path: str) -> None:
            nonlocal ok
            if ref and ref not in declared_ids:
                self.failures.append((2, f"Ref: '{path}' → '{ref}' not in artifact.entities"))
                ok = False

        for i, unit in enumerate(self.doc.get("units", [])):
            obs = unit.get("observables") or {}
            for j, participant in enumerate(obs.get("participants", [])):
                _check_ref(participant, f"units[{i}].observables.participants[{j}]")
            for j, ps in enumerate(unit.get("participant_states", [])):
                _check_ref(
                    ps.get("entity_ref"),
                    f"units[{i}].participant_states[{j}].entity_ref",
                )

        for i, rel in enumerate(self.doc.get("relationships", [])):
            _check_ref(rel.get("source"), f"relationships[{i}].source")
            _check_ref(rel.get("target"), f"relationships[{i}].target")

        return ok

    # ── Level 3 ─────────────────────────────────────────────────────────────

    def level3_semantic(self) -> bool:
        """Structural well-formedness: step ordering and protocol value."""
        ok = True

        protocol = self.doc.get("protocol", "")
        if protocol != "semantic-interaction-protocol":
            self.failures.append(
                (3, f"protocol must be 'semantic-interaction-protocol', got '{protocol}'")
            )
            ok = False

        for i, unit in enumerate(self.doc.get("units", [])):
            struct = unit.get("structure") or {}
            steps = struct.get("steps") or []
            seqs = [
                s["sequence_number"]
                for s in steps
                if isinstance(s.get("sequence_number"), int)
            ]
            if len(seqs) != len(set(seqs)):
                self.failures.append(
                    (3, f"units[{i}].structure.steps has duplicate sequence_numbers: {seqs}")
                )
                ok = False
            elif seqs and seqs != sorted(seqs):
                self.failures.append(
                    (3, f"units[{i}].structure.steps sequence_numbers not ascending: {seqs}")
                )
                ok = False

        return ok

    # ── Run ─────────────────────────────────────────────────────────────────

    def validate(self, level: int = 1, strict: bool = False) -> bool:
        """Run all checks up to *level*. Returns True iff conformant."""
        passed = True

        l1_ok = self.level1_schema()
        if not l1_ok:
            passed = False

        if level >= 2 and l1_ok:
            if not self.level2_referential():
                passed = False

        if level >= 3 and l1_ok:
            # Level 3 for SIP is always blocking (not advisory)
            if not self.level3_semantic():
                passed = False

        return passed


# ── Formatting ───────────────────────────────────────────────────────────────

def print_results(
    path: pathlib.Path,
    validator: GBRValidator,
    level: int,
    strict: bool,
    passed: bool,
) -> None:
    """Print conformance results for one document."""
    name = path.name

    # Level summaries
    # Level 3 may surface as failures (SIP, blocking) or warnings (GBR, advisory)
    l1_ok = not any(lvl == 1 for lvl, _ in validator.failures)
    l2_ok = not any(lvl == 2 for lvl, _ in validator.failures)
    l3_ok = not any(lvl == 3 for lvl, _ in validator.warnings) and not any(
        lvl == 3 for lvl, _ in validator.failures
    )

    tick = "✓"
    cross = "✗"
    warn = "⚠"
    skip = "–"
    lock = "🔒"

    def _symbol(ok: bool, requested: bool, advisory: bool = False) -> str:
        if not requested:
            return skip
        if ok:
            return tick
        return warn if advisory else cross

    l1_sym = _symbol(l1_ok, True)
    l2_sym = _symbol(l2_ok, level >= 2 and l1_ok)
    # L3 is advisory (GBR) unless L3 failures were added as blocking errors (SIP)
    l3_has_blocking = any(lvl == 3 for lvl, _ in validator.failures)
    l3_sym = _symbol(l3_ok, level >= 3 and l1_ok, advisory=not strict and not l3_has_blocking)
    if level >= 3 and not l1_ok:
        l3_sym = lock  # locked because L1 failed

    print(f"{l1_sym} Level 1 (schema)      — {name}")
    print(f"{l2_sym} Level 2 (entity refs) — {name}")
    print(f"{l3_sym} Level 3 (semantic)    — {name}")

    for lvl, msg in validator.failures:
        print(f"  {cross} {msg}")

    for lvl, msg in validator.warnings:
        label = "[blocking]" if strict else "[advisory]"
        print(f"  {warn} {label} {msg}")

    levels_requested = level
    levels_checked = [l1_ok]
    if level >= 2 and l1_ok:
        levels_checked.append(l2_ok)
    if level >= 3 and l1_ok:
        levels_checked.append(l3_ok)
    levels_passed = sum(levels_checked)
    print(f"\nPassed: {levels_passed}/{len(levels_checked)} level(s)")


# ── CLI ──────────────────────────────────────────────────────────────────────

def collect_paths(path_arg: str) -> list[pathlib.Path]:
    """Expand a file or directory argument to a list of .json paths."""
    p = pathlib.Path(path_arg)
    if p.is_file():
        return [p]
    if p.is_dir():
        return sorted(p.glob("*.json"))
    _fatal(f"Path not found: {p}", code=2)


def main() -> None:
    parser = argparse.ArgumentParser(
        prog="gbr_validate",
        description="GBR Protocol conformance validator",
        formatter_class=argparse.RawDescriptionHelpFormatter,
    )
    _all_doc_types = list(SCHEMA_MAP.keys()) + ["sip-artifact"]
    parser.add_argument(
        "doc_type",
        choices=_all_doc_types,
        help="Document type to validate",
    )
    parser.add_argument(
        "path",
        help="Path to a .json file or directory of .json files",
    )
    parser.add_argument(
        "--registry",
        metavar="PATH",
        help="Path to registry.json (required for Level 2 checks)",
    )
    parser.add_argument(
        "--level",
        type=int,
        choices=[1, 2, 3],
        default=1,
        help="Maximum conformance level (default: 1)",
    )
    parser.add_argument(
        "--strict",
        action="store_true",
        help="Treat Level 3 advisory warnings as blocking failures",
    )

    args = parser.parse_args()

    print(f"GBR Conformance Validator v{VERSION}")
    print("━" * 40)

    # Load registry once if provided
    registry = None
    if args.registry:
        registry = load_json(pathlib.Path(args.registry))

    paths = collect_paths(args.path)
    if not paths:
        _fatal(f"No .json files found at: {args.path}", code=2)

    overall_pass = True
    for doc_path in paths:
        doc = load_json(doc_path)
        if args.doc_type == "sip-artifact":
            validator: GBRValidator | SIPValidator = SIPValidator(doc)
        else:
            validator = GBRValidator(doc, args.doc_type, registry=registry)
        passed = validator.validate(level=args.level, strict=args.strict)
        if not passed:
            overall_pass = False
        print()
        print_results(doc_path, validator, args.level, args.strict, passed)
        if len(paths) > 1:
            print()

    sys.exit(0 if overall_pass else 1)


if __name__ == "__main__":
    main()
