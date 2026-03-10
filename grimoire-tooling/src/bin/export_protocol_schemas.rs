//! `grimoire-export-schemas` — write GBR protocol struct schemas to `schemas/generated/`.
//!
//! This binary calls [`gbr_types::generate_all_schemas`] to produce a JSON
//! object whose keys are type names and whose values are their JSON Schema
//! (Draft 7) representations, then writes one file per type into
//! `<workspace>/schemas/generated/`.
//!
//! These are **struct-level schemas** derived from the Rust type system.
//! They differ from the **document-level schemas** in `schemas/` (which are
//! hand-crafted to match the full GBR document format accepted by validators).
//!
//! Usage:
//!   grimoire-export-schemas [--workspace <path>] [--out <dir>] [--dry-run]
//!
//! Defaults:
//!   --workspace  Parent of `schemas/` inferred from the binary's own location
//!   --out        `<workspace>/schemas/generated/`
//!   --dry-run    Print what would be written without creating any files

use std::fs;
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let dry_run = args.contains(&"--dry-run".to_owned());

    let workspace = workspace_path(&args);
    let out_dir: PathBuf = flag_value(&args, "--out")
        .map(PathBuf::from)
        .unwrap_or_else(|| workspace.join("schemas").join("generated"));

    println!("grimoire-export-schemas v{}", env!("CARGO_PKG_VERSION"));
    println!("Output directory: {}", out_dir.display());
    if dry_run {
        println!("[dry-run] No files will be written.");
    }

    // Generate all struct-level schemas via gbr-types.
    let all_schemas = gbr_types::generate_all_schemas();

    let map = all_schemas
        .as_object()
        .expect("generate_all_schemas() must return a JSON object");

    if !dry_run {
        fs::create_dir_all(&out_dir).unwrap_or_else(|e| {
            eprintln!("ERROR: could not create output directory {}: {e}", out_dir.display());
            std::process::exit(1);
        });
    }

    let mut written = 0usize;
    for (type_name, schema_value) in map {
        let file_name = format!("{}.schema.json", to_snake_case(type_name));
        let out_path = out_dir.join(&file_name);

        let pretty = serde_json::to_string_pretty(schema_value)
            .unwrap_or_else(|e| panic!("Could not serialize schema for {type_name}: {e}"));

        if dry_run {
            println!("  [dry-run] would write {} ({} bytes)", file_name, pretty.len());
        } else {
            fs::write(&out_path, pretty.as_bytes()).unwrap_or_else(|e| {
                eprintln!("ERROR: could not write {}: {e}", out_path.display());
                std::process::exit(1);
            });
            println!("  ✓ {file_name}");
            written += 1;
        }
    }

    if dry_run {
        println!("\n[dry-run] Would write {} schema file(s).", map.len());
    } else {
        println!("\n✓ Wrote {written} schema file(s) to {}", out_dir.display());
    }
}

// ── Helpers ──────────────────────────────────────────────────────────────────

/// Convert `PascalCase` to `snake_case` for file naming.
fn to_snake_case(s: &str) -> String {
    let mut result = String::with_capacity(s.len() + 4);
    for (i, ch) in s.chars().enumerate() {
        if ch.is_uppercase() && i > 0 {
            result.push('_');
        }
        result.push(ch.to_ascii_lowercase());
    }
    result
}

/// Resolve the workspace root from `--workspace <path>` or by walking up from
/// the binary's location until we find a `Cargo.toml`.
fn workspace_path(args: &[String]) -> PathBuf {
    if let Some(p) = flag_value(args, "--workspace") {
        return PathBuf::from(p);
    }
    // Walk up from the binary's own directory.
    let mut dir = std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|d| d.to_path_buf()))
        .unwrap_or_else(|| PathBuf::from("."));
    loop {
        if dir.join("Cargo.toml").exists() && dir.join("schemas").exists() {
            return dir;
        }
        match dir.parent() {
            Some(p) => dir = p.to_path_buf(),
            None => {
                eprintln!("ERROR: could not locate workspace root (no Cargo.toml + schemas/ found)");
                eprintln!("       Use --workspace <path> to specify it explicitly.");
                std::process::exit(2);
            }
        }
    }
}

/// Extract the value of a `--flag <value>` pair from args.
fn flag_value<'a>(args: &'a [String], flag: &str) -> Option<&'a str> {
    args.windows(2)
        .find(|w| w[0] == flag)
        .map(|w| w[1].as_str())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snake_case_conversion() {
        assert_eq!(to_snake_case("Character"), "character");
        assert_eq!(to_snake_case("SceneCard"), "scene_card");
        assert_eq!(to_snake_case("VoiceContract"), "voice_contract");
        assert_eq!(to_snake_case("NarrativeVoice"), "narrative_voice");
    }
}
