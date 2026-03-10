//! `grimoire-ontology-drift-check` — compare canonical Rust tag keys with
//! documented tag keys in `references/gate-tag-vocabulary.md`.
//!
//! Usage:
//!   grimoire-ontology-drift-check [--workspace-path <path>] [--warn-only]

use std::{collections::HashSet, fs, path::PathBuf};

use grimoire_tooling::ontology::canonical_documented_tag_keys;
use regex::Regex;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let workspace = workspace_path(&args);
    let warn_only = args.iter().any(|a| a == "--warn-only");

    let vocab_path = workspace.join("references/gate-tag-vocabulary.md");
    let content = match fs::read_to_string(&vocab_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("✗ Could not read {}: {e}", vocab_path.display());
            if warn_only {
                eprintln!("⚠ warn-only mode enabled: ignoring failure exit code");
                return;
            }
            std::process::exit(1);
        }
    };

    let documented = documented_keys(&content);
    let canonical: HashSet<String> = canonical_documented_tag_keys().into_iter().collect();

    let mut missing_in_docs: Vec<String> = canonical.difference(&documented).cloned().collect();
    let mut missing_in_ontology: Vec<String> = documented.difference(&canonical).cloned().collect();
    missing_in_docs.sort();
    missing_in_ontology.sort();

    println!("Ontology drift check");
    println!("  canonical keys : {}", canonical.len());
    println!("  documented keys: {}", documented.len());

    if missing_in_docs.is_empty() && missing_in_ontology.is_empty() {
        println!("✓ No key drift detected.");
        return;
    }

    if !missing_in_docs.is_empty() {
        println!("\nKeys present in ontology but missing in docs ({}):", missing_in_docs.len());
        for k in &missing_in_docs {
            println!("  - {k}");
        }
    }

    if !missing_in_ontology.is_empty() {
        println!("\nKeys present in docs but missing in ontology ({}):", missing_in_ontology.len());
        for k in &missing_in_ontology {
            println!("  - {k}");
        }
    }

    if warn_only {
        println!("\n⚠ warn-only mode enabled: ignoring drift failure exit code");
        return;
    }
    std::process::exit(1);
}

fn workspace_path(args: &[String]) -> PathBuf {
    if let Some(pos) = args.iter().position(|a| a == "--workspace-path") {
        if let Some(path) = args.get(pos + 1) {
            return PathBuf::from(path);
        }
    }
    PathBuf::from(".")
}

fn documented_keys(content: &str) -> HashSet<String> {
    // Match section headers like: ## `source:` ... or ### `character:` ...
    let re = Regex::new(r"(?m)^#{2,}\s+`([a-zA-Z0-9_]+):`").expect("valid regex");
    re.captures_iter(content)
        .filter_map(|cap| cap.get(1).map(|m| m.as_str().to_lowercase()))
        .collect()
}
