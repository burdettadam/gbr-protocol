//! `grimoire-generate` — Story recipe generator. Rust port of `scripts/generate_story.py`.
//!
//! Chains all four phase engines:
//!   Phase 01 (concept)    → Genre + Plot Type + Trope
//!   Phase 02 (collision)  → Collision Pattern + Inciting Incident
//!   Phase 03 (characters) → Cast + Relationships
//!   Phase 05 (beats)      → Romance Beats + Trope Beats
//!
//! Then renders a Markdown story skeleton.
//!
//! Usage:
//!   grimoire-generate --genre romance [--seed 42] [--plot-type enemies_to_lovers]
//!   grimoire-generate --list-genres
//!   grimoire-generate --list-plot-types --genre romance

use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use grimoire_tooling::catalogs::{
    AlignmentSystemCatalog, ArchetypeCatalogEntry, CastSlot, CollisionPatternCatalogEntry,
    PlotTypeCatalogEntry, RomanceBeatEntry, RomanceBeatsCatalog,
    TropeCatalogEntry, WoundCatalogEntry,
};
use rand::{rngs::StdRng, seq::SliceRandom, Rng, SeedableRng};
use serde::Deserialize;

// ── CLI args ──────────────────────────────────────────────────────────────────

struct Args {
    workspace: PathBuf,
    genre: Option<String>,
    plot_type_slug: Option<String>,
    collision_slug: Option<String>,
    cast_size: Option<usize>,
    seed: Option<u64>,
    dry_run: bool,
    output: Option<PathBuf>,
    list_genres: bool,
    list_plot_types: bool,
}

fn parse_args() -> Args {
    let raw: Vec<String> = std::env::args().collect();
    let mut workspace = PathBuf::from(".");
    let mut genre = None;
    let mut plot_type_slug = None;
    let mut collision_slug = None;
    let mut cast_size = None;
    let mut seed = None;
    let mut dry_run = false;
    let mut output = None;
    let mut list_genres = false;
    let mut list_plot_types = false;

    let mut i = 1;
    while i < raw.len() {
        match raw[i].as_str() {
            "--workspace-path" => {
                i += 1;
                if i < raw.len() {
                    workspace = PathBuf::from(&raw[i]);
                }
            }
            "--genre" => {
                i += 1;
                if i < raw.len() {
                    genre = Some(raw[i].clone());
                }
            }
            "--plot-type" => {
                i += 1;
                if i < raw.len() {
                    plot_type_slug = Some(raw[i].clone());
                }
            }
            "--collision" => {
                i += 1;
                if i < raw.len() {
                    collision_slug = Some(raw[i].clone());
                }
            }
            "--cast-size" => {
                i += 1;
                if i < raw.len() {
                    cast_size = raw[i].parse().ok();
                }
            }
            "--seed" => {
                i += 1;
                if i < raw.len() {
                    seed = raw[i].parse().ok();
                }
            }
            "--dry-run" => dry_run = true,
            "--output" | "-o" => {
                i += 1;
                if i < raw.len() {
                    output = Some(PathBuf::from(&raw[i]));
                }
            }
            "--list-genres" => list_genres = true,
            "--list-plot-types" => list_plot_types = true,
            _ => {}
        }
        i += 1;
    }

    Args { workspace, genre, plot_type_slug, collision_slug, cast_size, seed, dry_run, output, list_genres, list_plot_types }
}

// ── Catalog loading helpers ───────────────────────────────────────────────────

fn read_yaml_str(path: &Path) -> Result<String, String> {
    fs::read_to_string(path).map_err(|e| format!("Cannot read `{}`: {e}", path.display()))
}

fn load_list<T>(path: &Path) -> Result<Vec<T>, String>
where
    T: serde::de::DeserializeOwned,
{
    let s = read_yaml_str(path)?;
    serde_yaml::from_str::<Vec<T>>(&s).map_err(|e| format!("YAML parse error in `{}`: {e}", path.display()))
}

fn load_doc<T>(path: &Path) -> Result<T, String>
where
    T: serde::de::DeserializeOwned,
{
    let s = read_yaml_str(path)?;
    serde_yaml::from_str::<T>(&s).map_err(|e| format!("YAML parse error in `{}`: {e}", path.display()))
}


// ── Inciting incident entry with actual YAML field names ──────────────────────
// The Rust `IncitingIncidentEntry` uses `compatible_plot_types` which does NOT
// match the YAML key `plot_type_affinity`. This local struct reflects reality.

#[derive(Debug, Deserialize)]
struct IncidentEntryFull {
    pub slug: String,
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub example: Option<String>,
    #[serde(default)]
    pub structural_function: Option<String>,
    #[serde(default)]
    pub collision_affinity: Vec<String>,
    #[serde(default)]
    pub plot_type_affinity: Vec<String>,
    #[serde(default)]
    pub genre_affinity: Vec<String>,
    #[serde(default)]
    pub generator_weight: Option<u32>,
}

#[derive(Debug, Deserialize)]
struct IncidentCategoryFull {
    #[serde(default)]
    pub incidents: Vec<IncidentEntryFull>,
}

#[derive(Debug, Deserialize)]
struct IncidentsCatalogFull {
    #[serde(default)]
    pub categories: HashMap<String, IncidentCategoryFull>,
}

// ── CharacterEntry (generated output type, not stored in catalogs.rs) ─────────

#[derive(Clone)]
struct CharacterEntry {
    label: String,
    slot: String,
    circle: String,
    membership: String,
    role: String,
    archetype_slug: String,
    archetype_name: String,
    wound_slug: String,
    wound_name: String,
    lie: String,
    need: String,
    want: String,
    alignment_slug: String,
    alignment_name: String,
    moral_rigidity_range: Vec<u8>,
}

// ── Phase 01: Concept ─────────────────────────────────────────────────────────

fn select_plot_type<'a>(
    genre: &str,
    rng: &mut StdRng,
    slug_override: Option<&str>,
    catalog: &'a [PlotTypeCatalogEntry],
) -> Result<&'a PlotTypeCatalogEntry, String> {
    if let Some(slug) = slug_override {
        return catalog
            .iter()
            .find(|pt| pt.slug == slug)
            .ok_or_else(|| format!("Unknown plot type '{slug}'"));
    }
    let candidates: Vec<_> = catalog
        .iter()
        .filter(|pt| pt.genre.to_string() == genre)
        .collect();
    if candidates.is_empty() {
        return Err(format!("No plot types for genre '{genre}'"));
    }
    Ok(candidates.choose(rng).unwrap())
}

fn select_trope<'a>(
    genre: &str,
    rng: &mut StdRng,
    plot_type_slug: &str,
    catalog: &'a [TropeCatalogEntry],
) -> Option<&'a TropeCatalogEntry> {
    if genre != "romance" {
        return None;
    }
    // Direct slug match first
    if let Some(t) = catalog.iter().find(|t| t.slug == plot_type_slug) {
        return Some(t);
    }
    // Fall back: collision_pattern_affinity or wound_affinity match
    let candidates: Vec<_> = catalog
        .iter()
        .filter(|t| {
            t.collision_pattern_affinity.iter().any(|a| a == plot_type_slug)
                || t.wound_affinity.iter().any(|a| a == plot_type_slug)
        })
        .collect();
    candidates.choose(rng).copied()
}

fn list_genres(catalog: &[PlotTypeCatalogEntry]) -> Vec<String> {
    let mut genres: Vec<String> = catalog
        .iter()
        .map(|pt| pt.genre.to_string())
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();
    genres.sort();
    genres
}

// ── Phase 02: Collision ───────────────────────────────────────────────────────

fn select_collision<'a>(
    rng: &mut StdRng,
    plot_type: &PlotTypeCatalogEntry,
    slug_override: Option<&str>,
    catalog: &'a [CollisionPatternCatalogEntry],
) -> Result<&'a CollisionPatternCatalogEntry, String> {
    if let Some(slug) = slug_override {
        return catalog
            .iter()
            .find(|cp| cp.slug == slug)
            .ok_or_else(|| format!("Unknown collision pattern '{slug}'"));
    }
    // Use required_collision_pattern from plot type
    if let Some(required) = &plot_type.required_collision_pattern {
        if let Some(cp) = catalog.iter().find(|cp| &cp.slug == required) {
            return Ok(cp);
        }
    }
    // Fall back to all patterns (no genre_affinity field in Rust struct, just pick randomly)
    catalog
        .choose(rng)
        .ok_or_else(|| "Collision catalog is empty".into())
}

fn select_inciting_incident<'a>(
    rng: &mut StdRng,
    genre: &str,
    collision_slug: &str,
    plot_type_slug: &str,
    catalog: &'a IncidentsCatalogFull,
) -> Option<&'a IncidentEntryFull> {
    let all: Vec<&IncidentEntryFull> =
        catalog.categories.values().flat_map(|cat| cat.incidents.iter()).collect();

    // Cascading filter: genre → collision → plot_type
    let by_genre: Vec<_> = all.iter().filter(|i| i.genre_affinity.iter().any(|g| g == genre)).copied().collect();
    let pool = if by_genre.is_empty() { all.clone() } else { by_genre };

    let by_collision: Vec<_> = pool.iter().filter(|i| i.collision_affinity.iter().any(|c| c == collision_slug)).copied().collect();
    let pool = if by_collision.is_empty() { pool } else { by_collision };

    let by_plot: Vec<_> = pool.iter().filter(|i| i.plot_type_affinity.iter().any(|p| p == plot_type_slug)).copied().collect();
    let pool = if by_plot.is_empty() { pool } else { by_plot };

    if pool.is_empty() {
        return None;
    }

    // Weighted selection
    let weights: Vec<u32> = pool.iter().map(|i| i.generator_weight.unwrap_or(1)).collect();
    let total: u32 = weights.iter().sum();
    if total == 0 {
        return pool.choose(rng).copied();
    }
    let mut r = (rand::Rng::gen_range(rng, 0..total)) as i64;
    for (item, &w) in pool.iter().zip(weights.iter()) {
        r -= w as i64;
        if r < 0 {
            return Some(item);
        }
    }
    pool.last().copied()
}

// ── Phase 03: Characters ──────────────────────────────────────────────────────

fn instantiate_character(
    rng: &mut StdRng,
    slot: &CastSlot,
    archetypes: &[ArchetypeCatalogEntry],
    wounds: &[WoundCatalogEntry],
    alignment_cells: &[grimoire_tooling::catalogs::AlignmentCellEntry],
) -> CharacterEntry {
    // Archetype — from slot constraint or random
    let archetype_slug = if !slot.archetype_constraint.is_empty() {
        slot.archetype_constraint.choose(rng).unwrap().clone()
    } else {
        archetypes.choose(rng).map(|a| a.slug.clone()).unwrap_or_default()
    };
    let archetype = archetypes.iter().find(|a| a.slug == archetype_slug);
    let archetype_name = archetype.map(|a| a.name.clone()).unwrap_or_else(|| archetype_slug.clone());

    // Wound — prefer archetype-affine wounds
    let affine_wounds: Vec<_> = wounds
        .iter()
        .filter(|w| w.archetype_affinity.iter().any(|a| a == &archetype_slug))
        .collect();
    let wound = if !affine_wounds.is_empty() {
        affine_wounds.choose(rng).copied()
    } else {
        wounds.choose(rng)
    };
    let wound_slug = wound.map(|w| w.slug.clone()).unwrap_or_default();
    let wound_name = wound.map(|w| w.name.clone()).unwrap_or_else(|| wound_slug.clone());
    let lie = wound.and_then(|w| w.lie_template.clone()).unwrap_or_default();
    let need = wound.and_then(|w| w.need_template.clone()).unwrap_or_default();
    let want = wound.and_then(|w| w.want_template.clone()).unwrap_or_default();

    // Alignment — prefer wound-affine alignment cell
    let tendencies = wound.map(|w| &w.alignment_tendency).cloned().unwrap_or_default();
    let alignment_cell = if !tendencies.is_empty() {
        let slug = tendencies.choose(rng).unwrap();
        alignment_cells.iter().find(|c| &c.slug == slug)
    } else {
        alignment_cells.choose(rng)
    };
    let alignment_cell = alignment_cell.or_else(|| alignment_cells.first());
    let alignment_slug = alignment_cell.map(|c| c.slug.clone()).unwrap_or_default();
    let alignment_name = alignment_cell.map(|c| c.label.clone()).unwrap_or_else(|| alignment_slug.clone());
    let moral_rigidity_range = alignment_cell.map(|c| c.moral_rigidity_range.clone()).unwrap_or_default();

    let slot_name = &slot.slot;
    let label = format!("[{}]", slot_name.replace('_', " ").split_whitespace()
        .map(|w| {
            let mut c = w.chars();
            match c.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
            }
        })
        .collect::<Vec<_>>()
        .join(" "));

    CharacterEntry {
        label,
        slot: slot_name.clone(),
        circle: slot.circle.clone().unwrap_or_else(|| "?".to_owned()),
        membership: slot.membership.clone().unwrap_or_else(|| "unknown".to_owned()),
        role: slot.role.clone().unwrap_or_else(|| "unknown".to_owned()),
        archetype_slug,
        archetype_name,
        wound_slug,
        wound_name,
        lie,
        need,
        want,
        alignment_slug,
        alignment_name,
        moral_rigidity_range,
    }
}

fn generate_cast(
    rng: &mut StdRng,
    collision: &CollisionPatternCatalogEntry,
    cast_size: Option<usize>,
    archetypes: &[ArchetypeCatalogEntry],
    wounds: &[WoundCatalogEntry],
    alignment_cells: &[grimoire_tooling::catalogs::AlignmentCellEntry],
) -> Vec<CharacterEntry> {
    let mut cast = Vec::new();
    for slot in &collision.required_cast_slots {
        let count = slot.min.unwrap_or(1) as usize;
        for _ in 0..count {
            cast.push(instantiate_character(rng, slot, archetypes, wounds, alignment_cells));
        }
    }

    // Pad to cast_size with supporting characters
    if let Some(target) = cast_size {
        const PADDING_ROLES: &[&str] = &["ally", "foil", "confidant"];
        let mut pad_idx = cast.len() + 1;
        while cast.len() < target {
            let role = PADDING_ROLES.choose(rng).unwrap();
            let circle = if rng.gen_bool(0.5) { "a" } else { "b" };
            let pad_slot = CastSlot {
                slot: format!("supporting_{pad_idx}"),
                circle: Some(circle.to_owned()),
                membership: Some("peripheral".to_owned()),
                archetype_constraint: vec![],
                role: Some((*role).to_owned()),
                min: Some(1),
                max: Some(1),
            };
            cast.push(instantiate_character(rng, &pad_slot, archetypes, wounds, alignment_cells));
            pad_idx += 1;
        }
    }

    cast
}

struct RelEntry {
    between: String,
    rel_type: String,
    dynamic: String,
    source: String,
}

fn generate_relationships(cast: &[CharacterEntry]) -> Vec<RelEntry> {
    let mut rels = Vec::new();

    // Auto-pair protagonist ↔ love_interest
    let protagonists: Vec<_> = cast.iter().filter(|c| c.role == "protagonist").collect();
    let love_interests: Vec<_> = cast.iter().filter(|c| c.role == "love_interest").collect();
    for p in &protagonists {
        for li in &love_interests {
            rels.push(RelEntry {
                between: format!("{} ↔ {}", p.label, li.label),
                rel_type: "romantic".into(),
                dynamic: "primary romantic arc".into(),
                source: "auto_paired".into(),
            });
        }
    }

    // Auto-pair protagonist ↔ antagonist
    let antagonists: Vec<_> = cast.iter().filter(|c| c.role == "antagonist").collect();
    for p in &protagonists {
        for ant in &antagonists {
            rels.push(RelEntry {
                between: format!("{} ↔ {}", p.label, ant.label),
                rel_type: "adversarial".into(),
                dynamic: "primary conflict".into(),
                source: "auto_paired".into(),
            });
        }
    }

    rels
}

// ── Phase 05: Beats ───────────────────────────────────────────────────────────

fn get_trope_beats<'a>(plot_type_slug: &str, tropes: &'a [TropeCatalogEntry]) -> Vec<&'a str> {
    tropes
        .iter()
        .find(|t| t.slug == plot_type_slug)
        .map(|t| t.required_tension_beats.iter().map(|s| s.as_str()).collect())
        .unwrap_or_default()
}

// ── Markdown renderer ─────────────────────────────────────────────────────────

struct StoryRecipe<'a> {
    genre: &'a str,
    seed: u64,
    plot_type: &'a PlotTypeCatalogEntry,
    collision: &'a CollisionPatternCatalogEntry,
    inciting: Option<&'a IncidentEntryFull>,
    cast: &'a [CharacterEntry],
    relationships: &'a [RelEntry],
    romance_beats: &'a [RomanceBeatEntry],
    trope_beats: Vec<&'a str>,
    trope: Option<&'a TropeCatalogEntry>,
}

fn render_markdown(recipe: &StoryRecipe<'_>) -> String {
    let mut lines: Vec<String> = Vec::new();
    let mut w = |s: String| lines.push(s);

    let pt = recipe.plot_type;
    let coll = recipe.collision;

    w(format!("# Story Recipe — {}", pt.name));
    w(String::new());
    w(format!("**Generated:** `seed={}`  ", recipe.seed));
    w(format!("**Genre:** {}  ", recipe.genre));
    w(format!("**Plot Type:** {} (`{}`)  ", pt.name, pt.slug));
    w(format!("**Collision Pattern:** {} (`{}`)  ", coll.name, coll.slug));
    if let Some(trope) = recipe.trope {
        w(format!("**Romance Trope:** {} (`{}`)  ", trope.name, trope.slug));
        if let Some(rp) = &trope.reader_promise {
            w(format!("**Reader Promise:** {rp}  "));
        }
    }
    w(String::new());

    // Logline
    if let Some(logline) = &pt.logline_template {
        w("## Logline Template".into());
        w(String::new());
        w(format!("> {logline}"));
        w(String::new());
    }

    // Inciting Incident
    if let Some(inc) = recipe.inciting {
        w("## Inciting Incident".into());
        w(String::new());
        w(format!("**Type:** {} (`{}`)  ", inc.name, inc.slug));
        if let Some(desc) = &inc.description {
            w(format!("**Description:** {desc}  "));
        }
        if let Some(ex) = &inc.example {
            w(format!("**Example:** *{ex}*  "));
        }
        if let Some(sf) = &inc.structural_function {
            w(format!("**Structural Function:** {sf}  "));
        }
        w("<!-- beat:inciting_incident -->".into());
        w(String::new());
    }

    // Collision Pattern Detail
    w("## Social Circle Collision".into());
    w(String::new());
    w(format!(
        "**Collision Type:** {}  ",
        coll.collision_type.as_deref().unwrap_or("unspecified")
    ));
    w(format!(
        "**Power Asymmetry:** {}  ",
        coll.power_asymmetry.as_deref().unwrap_or("unspecified")
    ));
    w(format!("<!-- collision_pattern:{} -->", coll.slug));
    w(String::new());

    if !coll.circle_a_type_constraint.is_empty() || !coll.circle_b_type_constraint.is_empty() {
        w("### Circle Constraints".into());
        w(String::new());
        w(format!("- **Circle A options:** {}", coll.circle_a_type_constraint.join(", ")));
        w(format!("- **Circle B options:** {}", coll.circle_b_type_constraint.join(", ")));
        w(String::new());
    }

    // Cast table
    w("## Cast".into());
    w(String::new());
    w("| # | Name | Slot | Circle | Membership | Role | Archetype | Wound | Lie | Alignment |".into());
    w("|---|------|------|--------|------------|------|-----------|-------|-----|-----------|".into());
    for (i, c) in recipe.cast.iter().enumerate() {
        w(format!(
            "| {} | {} | {} | {} | {} | {} | {} | {} | {} | {} |",
            i + 1,
            c.label,
            c.slot,
            c.circle,
            c.membership,
            c.role,
            c.archetype_name,
            c.wound_name,
            c.lie,
            c.alignment_name
        ));
    }
    w(String::new());

    // Character detail cards
    w("### Character Detail Cards".into());
    w(String::new());
    for c in recipe.cast.iter() {
        w(format!("#### {}", c.label));
        w(String::new());
        w(format!("- **Slot:** {}", c.slot));
        w(format!("- **Circle:** {} ({})", c.circle, c.membership));
        w(format!("- **Role:** {}", c.role));
        w(format!("- **Archetype:** {} <!-- archetype:{} -->", c.archetype_name, c.archetype_slug));
        w(format!("- **Wound:** {} <!-- wound:{} -->", c.wound_name, c.wound_slug));
        w(format!("- **Lie:** *\"{}\"*", c.lie));
        w(format!("- **Need:** *\"{}\"*", c.need));
        w(format!("- **Want:** *\"{}\"*", c.want));
        let mr_str = if c.moral_rigidity_range.len() >= 2 {
            format!(" (Moral Rigidity {}–{})", c.moral_rigidity_range[0], c.moral_rigidity_range[1])
        } else {
            String::new()
        };
        w(format!("- **Alignment:** {}{} <!-- alignment:{} -->", c.alignment_name, mr_str, c.alignment_slug));
        w(String::new());
    }

    // Relationships
    w("## Relationships".into());
    w(String::new());
    w("| Between | Type | Dynamic | Source |".into());
    w("|---------|------|---------|--------|".into());
    for r in recipe.relationships.iter() {
        w(format!("| {} | {} | {} | {} |", r.between, r.rel_type, r.dynamic, r.source));
    }
    w(String::new());

    // Romance Beats
    if !recipe.romance_beats.is_empty() {
        w("## Romance Arc Beats".into());
        w(String::new());
        w("| # | Beat | Position | Act | Tension | Tag |".into());
        w("|---|------|----------|-----|---------|-----|".into());
        for (i, b) in recipe.romance_beats.iter().enumerate() {
            let pos_str = if b.position_pct.len() >= 2 {
                format!("{}–{}%", b.position_pct[0], b.position_pct[1])
            } else {
                "?".into()
            };
            let t_str = if b.tension_level.len() >= 2 {
                format!("{}–{}", b.tension_level[0], b.tension_level[1])
            } else {
                "?".into()
            };
            w(format!(
                "| {} | {} | {} | {} | {} | `<!-- beat:{} -->` |",
                i + 1,
                b.name,
                pos_str,
                b.act,
                t_str,
                b.slug
            ));
        }
        w(String::new());
    }

    // Trope-specific beats
    if !recipe.trope_beats.is_empty() {
        w("### Trope-Specific Tension Beats".into());
        w(String::new());
        w(format!("*From `{}` trope requirements:*", pt.slug));
        w(String::new());
        for (i, tb) in recipe.trope_beats.iter().enumerate() {
            w(format!("{}. {tb}", i + 1));
        }
        w(String::new());
    }

    // Trope lie pairs + key mechanic
    if let Some(trope) = recipe.trope {
        if let Some(km) = &trope.key_tension_mechanic {
            w(format!("**Key Tension Mechanic:** {km}"));
            w(String::new());
        }
    }

    // Collision Escalation scaffolding
    w("## Collision Escalation Phases".into());
    w(String::new());
    w("| Phase | Description | Romance Beats | Your Notes |".into());
    w("|-------|-------------|---------------|------------|".into());
    w("| 1 — Setup | Circles are separate; status quo | Ordinary Worlds | |".into());
    w("| 2 — First Contact | Circles brush; representatives meet | The Meet, Push-Pull | |".into());
    w("| 3 — Friction | Values clash; loyalties tested | First Threshold, Deepening | |".into());
    w("| 4 — Escalation | Stakes rise; circles react | Midpoint Shift, Intimacy Deepens | |".into());
    w("| 5 — Crisis | Circles demand loyalty | Dark Moment | |".into());
    w("| 6 — Transformation | Characters choose; circles adapt | Realization, Grand Gesture | |".into());
    w("| 7 — Resolution | New equilibrium | Resolution (HEA/HFN) | |".into());
    w(String::new());

    // Structural summary
    if !pt.recommended_structures.is_empty() {
        w("## Recommended Structures".into());
        w(String::new());
        for s in &pt.recommended_structures {
            w(format!("- {s}"));
        }
        w(String::new());
    }
    if let Some(cc) = &pt.core_conflict_type {
        w(format!("**Core Conflict Type:** {cc}  "));
        w(String::new());
    }
    let cast_range = format!(
        "{} – {}",
        pt.min_cast_size.map(|n| n.to_string()).unwrap_or_else(|| "?".into()),
        pt.max_cast_size.map(|n| n.to_string()).unwrap_or_else(|| "?".into()),
    );
    w(format!("**Recommended Cast Size:** {cast_range}  "));
    w(String::new());

    // Footer
    w("---".into());
    w(String::new());
    w("## Next Steps".into());
    w(String::new());
    w("1. **Name your characters** — Replace the `[Slot Name]` placeholders".into());
    w("2. **Choose your circles** — Pick specific circle types from `02-collision/references/social-circle-types.yaml`".into());
    w("3. **Fill in `02-collision/social-circles.md`** — Instantiate your collision".into());
    w("4. **Fill in `03-characters/character-profile.md`** — One per character, using the archetype/wound/alignment above".into());
    w("5. **Fill in `03-characters/cast-overview.md`** — Paste the cast table above".into());
    w("6. **Draft your beat sheet** — Use `05-plot-and-structure/beat-sheet.md`".into());
    w("7. **Run gate checks** — `grimoire-gate-check`".into());
    w(String::new());
    w(format!("<!-- plot_type:{} -->", pt.slug));
    w(format!("<!-- genre:{} -->", recipe.genre));

    lines.join("\n") + "\n"
}

// ── Main ──────────────────────────────────────────────────────────────────────

fn main() {
    let args = parse_args();
    let root = args.workspace.canonicalize().unwrap_or_else(|_| args.workspace.clone());

    // ── Load catalogs ─────────────────────────────────────────────────────────
    macro_rules! catalog_path {
        ($rel:expr) => {
            root.join($rel)
        };
    }

    let plot_types: Vec<PlotTypeCatalogEntry> =
        load_list(&catalog_path!("01-concept/references/plot-types.yaml"))
            .unwrap_or_else(|e| { eprintln!("ERROR: {e}"); std::process::exit(1); });

    // ── List modes ────────────────────────────────────────────────────────────
    if args.list_genres {
        let genres = list_genres(&plot_types);
        println!("Available genres:");
        for g in &genres {
            let count = plot_types.iter().filter(|pt| pt.genre.to_string() == *g).count();
            println!("  {g} ({count} plot types)");
        }
        return;
    }

    if args.list_plot_types {
        let genre = match &args.genre {
            Some(g) => g.as_str(),
            None => { eprintln!("ERROR: --genre required with --list-plot-types"); std::process::exit(1); }
        };
        let pts: Vec<_> = plot_types.iter().filter(|pt| pt.genre.to_string() == genre).collect();
        if pts.is_empty() {
            println!("No plot types for genre '{genre}'");
            return;
        }
        println!("Plot types for genre '{genre}':");
        for pt in &pts {
            let cp = pt.required_collision_pattern.as_deref().unwrap_or("?");
            println!("  {:30} → collision: {cp}", pt.slug);
        }
        return;
    }

    // ── Generation mode ───────────────────────────────────────────────────────
    let genre = match &args.genre {
        Some(g) => g.as_str(),
        None => {
            eprintln!("ERROR: --genre is required");
            std::process::exit(1);
        }
    };

    let seed = args.seed.unwrap_or_else(|| {
        use std::time::{SystemTime, UNIX_EPOCH};
        SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_nanos() as u64 % 1_000_000).unwrap_or(0)
    });
    let mut rng = StdRng::seed_from_u64(seed);

    // Load all other catalogs
    let tropes: Vec<TropeCatalogEntry> =
        load_list(&catalog_path!("01-concept/references/romance-tropes.yaml"))
            .unwrap_or_else(|e| { eprintln!("WARNING: {e}"); vec![] });

    let collisions: Vec<CollisionPatternCatalogEntry> =
        load_list(&catalog_path!("02-collision/references/circle-collision-patterns.yaml"))
            .unwrap_or_else(|e| { eprintln!("ERROR: {e}"); std::process::exit(1); });

    let incidents_catalog: IncidentsCatalogFull =
        load_doc(&catalog_path!("02-collision/references/inciting-incidents.yaml"))
            .unwrap_or_else(|e| { eprintln!("WARNING: {e}"); IncidentsCatalogFull { categories: HashMap::new() } });

    let archetypes: Vec<ArchetypeCatalogEntry> =
        load_list(&catalog_path!("03-characters/references/character-archetypes.yaml"))
            .unwrap_or_else(|e| { eprintln!("ERROR: {e}"); std::process::exit(1); });

    let wounds: Vec<WoundCatalogEntry> =
        load_list(&catalog_path!("03-characters/references/character-wounds.yaml"))
            .unwrap_or_else(|e| { eprintln!("ERROR: {e}"); std::process::exit(1); });

    let alignment_catalog: AlignmentSystemCatalog =
        load_doc(&catalog_path!("03-characters/references/alignment-system.yaml"))
            .unwrap_or_else(|e| { eprintln!("ERROR: {e}"); std::process::exit(1); });
    let alignment_cells = &alignment_catalog.cells;

    let beats_catalog: Option<RomanceBeatsCatalog> = if genre == "romance" {
        load_doc(&catalog_path!("05-plot-and-structure/references/romance-beats.yaml")).ok()
    } else {
        None
    };

    // Phase 01: select plot type + trope
    let plot_type = select_plot_type(genre, &mut rng, args.plot_type_slug.as_deref(), &plot_types)
        .unwrap_or_else(|e| { eprintln!("ERROR: {e}"); std::process::exit(1); });
    let trope = select_trope(genre, &mut rng, &plot_type.slug, &tropes);

    // Phase 02: select collision + inciting incident
    let collision = select_collision(&mut rng, plot_type, args.collision_slug.as_deref(), &collisions)
        .unwrap_or_else(|e| { eprintln!("ERROR: {e}"); std::process::exit(1); });
    let inciting = select_inciting_incident(&mut rng, genre, &collision.slug, &plot_type.slug, &incidents_catalog);

    // Phase 03: cast + relationships
    let cast = generate_cast(&mut rng, collision, args.cast_size, &archetypes, &wounds, alignment_cells);
    let relationships = generate_relationships(&cast);

    // Phase 05: romance beats + trope beats
    let romance_beats: &[RomanceBeatEntry] = beats_catalog
        .as_ref()
        .map(|bc| bc.core_romance_arc.beats.as_slice())
        .unwrap_or(&[]);
    let trope_beats = get_trope_beats(&plot_type.slug, &tropes);

    // Dry-run mode
    if args.dry_run {
        println!("Seed:       {seed}");
        println!("Genre:      {genre}");
        println!("Plot Type:  {} ({})", plot_type.name, plot_type.slug);
        println!("Collision:  {} ({})", collision.name, collision.slug);
        if let Some(inc) = inciting {
            println!("Inciting:   {} ({})", inc.name, inc.slug);
        }
        println!("Cast Size:  {}", cast.len());
        for c in &cast {
            println!("  [{}] {} / {} / {}", c.slot, c.archetype_name, c.wound_name, c.alignment_name);
        }
        println!("Relationships: {}", relationships.len());
        if let Some(t) = trope {
            println!("Trope:      {}", t.name);
        }
        return;
    }

    // Render
    let recipe = StoryRecipe {
        genre,
        seed,
        plot_type,
        collision,
        inciting,
        cast: &cast,
        relationships: &relationships,
        romance_beats,
        trope_beats,
        trope,
    };
    let md = render_markdown(&recipe);

    match &args.output {
        Some(path) => {
            fs::write(path, &md).unwrap_or_else(|e| eprintln!("ERROR writing {}: {e}", path.display()));
            eprintln!("Wrote story recipe to {}", path.display());
        }
        None => print!("{md}"),
    }
}

