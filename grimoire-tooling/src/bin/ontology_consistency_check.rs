//! `grimoire-ontology-consistency-check` — compare Rust enum vocabularies
//! against YAML catalog slug vocabularies for core ontology domains.
//!
//! Usage:
//!   grimoire-ontology-consistency-check [--workspace-path <path>] [--warn-only]

use std::{collections::HashSet, fs, path::PathBuf};

use grimoire_tooling::catalogs::{
    ArchetypeCatalogEntry, DriveCatalogEntry, RoleCatalogEntry, WoundCatalogEntry,
};
use grimoire_tooling::enums::{Archetype, DriveModel, Role, Wound};
use schemars::{schema_for, JsonSchema};
use schemars::schema::{RootSchema, Schema, SchemaObject};
use serde_json::Value;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let workspace = workspace_path(&args);
    let warn_only = args.iter().any(|a| a == "--warn-only");

    let mut has_drift = false;

    has_drift |= check_pair::<Archetype, ArchetypeCatalogEntry>(
        "archetype",
        &workspace.join("03-characters/references/character-archetypes.yaml"),
        |e| e.slug.clone(),
    );

    has_drift |= check_pair::<Wound, WoundCatalogEntry>(
        "wound",
        &workspace.join("03-characters/references/character-wounds.yaml"),
        |e| e.slug.clone(),
    );

    has_drift |= check_pair::<Role, RoleCatalogEntry>(
        "role",
        &workspace.join("03-characters/references/character-roles.yaml"),
        |e| e.slug.clone(),
    );

    has_drift |= check_pair::<DriveModel, DriveCatalogEntry>(
        "drive_model",
        &workspace.join("03-characters/references/character-drives.yaml"),
        |e| e.slug.clone(),
    );

    if has_drift {
        if warn_only {
            println!("\n⚠ warn-only mode enabled: ignoring consistency drift failure exit code");
            return;
        }
        std::process::exit(1);
    }

    println!("\n✓ No enum/catalog drift detected for checked domains.");
}

fn workspace_path(args: &[String]) -> PathBuf {
    if let Some(pos) = args.iter().position(|a| a == "--workspace-path") {
        if let Some(path) = args.get(pos + 1) {
            return PathBuf::from(path);
        }
    }
    PathBuf::from(".")
}

fn check_pair<E, C>(
    domain: &str,
    catalog_path: &std::path::Path,
    slug_of: fn(&C) -> String,
) -> bool
where
    E: JsonSchema,
    C: serde::de::DeserializeOwned,
{
    println!("\n[{domain}] {}", catalog_path.display());

    let enum_set = enum_values::<E>();
    if enum_set.is_empty() {
        println!("  ✗ enum schema is empty");
        return true;
    }

    let entries: Vec<C> = match load_yaml_list::<C>(catalog_path) {
        Ok(v) => v,
        Err(e) => {
            println!("  ✗ parse failed: {e}");
            return true;
        }
    };

    let catalog_set: HashSet<String> = entries.into_iter().map(|e| slug_of(&e)).collect();

    let tolerated_legacy = tolerated_legacy_enum_values(domain);

    let mut enum_missing_in_catalog: Vec<String> = enum_set
        .difference(&catalog_set)
        .filter(|v| !tolerated_legacy.contains(*v))
        .cloned()
        .collect();
    let mut catalog_missing_in_enum: Vec<String> = catalog_set.difference(&enum_set).cloned().collect();
    enum_missing_in_catalog.sort();
    catalog_missing_in_enum.sort();

    println!("  enum values   : {}", enum_set.len());
    println!("  catalog values: {}", catalog_set.len());
    if !tolerated_legacy.is_empty() {
        let mut legacy: Vec<_> = tolerated_legacy.iter().cloned().collect();
        legacy.sort();
        println!("  tolerated legacy enum values: {}", legacy.join(", "));
    }

    if enum_missing_in_catalog.is_empty() && catalog_missing_in_enum.is_empty() {
        println!("  ✓ aligned");
        return false;
    }

    if !enum_missing_in_catalog.is_empty() {
        println!("  ✗ in enum but missing in catalog ({}):", enum_missing_in_catalog.len());
        for v in &enum_missing_in_catalog {
            println!("    - {v}");
        }
    }

    if !catalog_missing_in_enum.is_empty() {
        println!("  ✗ in catalog but missing in enum ({}):", catalog_missing_in_enum.len());
        for v in &catalog_missing_in_enum {
            println!("    - {v}");
        }
    }

    true
}

fn tolerated_legacy_enum_values(domain: &str) -> HashSet<String> {
    match domain {
        "archetype" => vec![
            "lover",
            "sage",
            "innocent",
            "creator",
            "explorer",
            "magician",
            "jester",
            "outlaw",
        ]
        .into_iter()
        .map(|s| s.to_owned())
        .collect(),
        "wound" => vec![
            "abandonment",
            "betrayal",
            "guilt_and_failure",
            "trauma_and_abuse",
            "shame",
            "identity_rejection",
            "injustice",
            "neglect",
            "survivor_guilt",
            "displacement",
        ]
        .into_iter()
        .map(|s| s.to_owned())
        .collect(),
        _ => HashSet::new(),
    }
}

fn load_yaml_list<T: serde::de::DeserializeOwned>(path: &std::path::Path) -> Result<Vec<T>, String> {
    let content = fs::read_to_string(path).map_err(|e| e.to_string())?;

    if let Ok(v) = serde_yaml::from_str::<Vec<T>>(&content) {
        return Ok(v);
    }

    let mut all = Vec::new();
    for doc in serde_yaml::Deserializer::from_str(&content) {
        use serde::Deserialize as _;
        match Vec::<T>::deserialize(doc) {
            Ok(mut entries) => all.append(&mut entries),
            Err(e) => return Err(e.to_string()),
        }
    }
    Ok(all)
}

fn enum_values<E: JsonSchema>() -> HashSet<String> {
    let root = schema_for!(E);
    enum_values_from_root(&root)
}

fn enum_values_from_root(root: &RootSchema) -> HashSet<String> {
    let mut out = HashSet::new();
    collect_enum_values(&root.schema, root, &mut out);
    out
}

fn values_as_set(values: &[Value]) -> HashSet<String> {
    values
        .iter()
        .filter_map(|v| match v {
            Value::String(s) => Some(s.clone()),
            _ => None,
        })
        .collect()
}

fn collect_enum_values(schema_obj: &SchemaObject, root: &RootSchema, out: &mut HashSet<String>) {
    if let Some(values) = &schema_obj.enum_values {
        out.extend(values_as_set(values));
    }

    if let Some(v) = &schema_obj.const_value {
        if let Value::String(s) = v {
            out.insert(s.clone());
        }
    }

    if let Some(reference) = &schema_obj.reference {
        if let Some(def_name) = reference.strip_prefix("#/definitions/") {
            if let Some(def_schema) = root.definitions.get(def_name) {
                if let Schema::Object(obj) = def_schema {
                    collect_enum_values(obj, root, out);
                }
            }
        }
    }

    if let Some(sub) = &schema_obj.subschemas {
        if let Some(one_of) = &sub.one_of {
            collect_from_schema_vec(one_of, root, out);
        }
        if let Some(any_of) = &sub.any_of {
            collect_from_schema_vec(any_of, root, out);
        }
        if let Some(all_of) = &sub.all_of {
            collect_from_schema_vec(all_of, root, out);
        }
        if let Some(not) = &sub.not {
            collect_from_schema(not, root, out);
        }
        if let Some(if_schema) = &sub.if_schema {
            collect_from_schema(if_schema, root, out);
        }
        if let Some(then_schema) = &sub.then_schema {
            collect_from_schema(then_schema, root, out);
        }
        if let Some(else_schema) = &sub.else_schema {
            collect_from_schema(else_schema, root, out);
        }
    }
}

fn collect_from_schema_vec(schemas: &[Schema], root: &RootSchema, out: &mut HashSet<String>) {
    for schema in schemas {
        collect_from_schema(schema, root, out);
    }
}

fn collect_from_schema(schema: &Schema, root: &RootSchema, out: &mut HashSet<String>) {
    if let Schema::Object(obj) = schema {
        collect_enum_values(obj, root, out);
    }
}
