/// `grimoire-validate-catalogs` — validate all YAML catalogs against typed schemas.
///
/// Usage:
///   grimoire-validate-catalogs [--workspace-path <path>] [--dump-schema]
///
/// Loads every `*/references/*.yaml` file in the workspace and deserializes
/// it against the appropriate typed catalog entry struct.  Any unknown keys
/// or type mismatches produce error output.
///
/// With `--dump-schema`, writes all generated JSON Schemas to `schemas/`
/// instead of running validation.

use std::fs;
use std::path::{Path, PathBuf};

use grimoire_tooling::catalogs::{
    AdaptationModeEntry, AffectModeEntry, ArchetypeCatalogEntry,
    AutofictionModeEntry, CognitiveNarrativeModeEntry, CollisionPatternCatalogEntry,
    ComicModeCatalogEntry, DisabilityRepModeEntry, DriveCatalogEntry,
    EcocriticalModeEntry, EmplotmentTypeEntry, ExperimentalNarrationEntry,
    FeministNarrativeEntry, FocalizationModeEntry, GenreTropeCatalogEntry,
    ImageSchemaEntry, IncitingIncidentsCatalog, IndigenousNarrativeModeEntry,
    IntertextualRelationEntry, IronyTypeCatalogEntry, MarxistNarrativeModeEntry,
    MetaphorTypeCatalogEntry, NarrativeEthicsModeEntry, NarrativeTimeModeEntry,
    PentadElementEntry, PhilosophyFictionEntry, PlotTypeCatalogEntry,
    PostcolonialModeEntry, PosthumanModeEntry, ProppFunctionEntry,
    PsychoanalyticMechanismEntry, QueerNarrativeModeEntry,
    RevisionPassEntry, RoleCatalogEntry, SerialityTypeEntry, SignifyingModeEntry,
    SocialCircleCatalogEntry, SpatialModeEntry, SpeechActEntry, SubtextModeEntry,
    TropeCatalogEntry, TraumaModeEntry, VerseProsodyEntry, WoundCatalogEntry,
    // Phase V round-2 additions
    GenreReadingModeEntry, GraphicNarrativeModeEntry, ParatextZoneEntry,
    SemioLinguisticFunctionEntry, TranslationModeEntry, YaNarrativeModeEntry,
    // Complex-structure catalogs
    AlignmentSystemCatalog, RomanceBeatsCatalog,
};

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.contains(&"--dump-schema".to_owned()) {
        dump_schemas(&workspace_path(&args));
        return;
    }

    let root = workspace_path(&args);
    let errors = validate_all_catalogs(&root);

    if errors.is_empty() {
        println!("✓ All catalogs valid.");
    } else {
        eprintln!("✗ {} catalog error(s):", errors.len());
        for e in &errors {
            eprintln!("  {e}");
        }
        std::process::exit(1);
    }
}

fn workspace_path(args: &[String]) -> PathBuf {
    if let Some(pos) = args.iter().position(|a| a == "--workspace-path") {
        PathBuf::from(&args[pos + 1])
    } else {
        PathBuf::from(".")
    }
}

fn validate_all_catalogs(root: &Path) -> Vec<String> {
    let mut errors = Vec::new();

    macro_rules! validate_list {
        ($path:expr, $entry_type:ty) => {{
            let full = root.join($path);
            if full.exists() {
                match load_yaml_list::<$entry_type>(&full) {
                    Ok(entries) => println!("  ✓ {} ({} entries)", $path, entries.len()),
                    Err(e) => errors.push(format!("{}: {}", $path, e)),
                }
            }
        }};
    }

    println!("Validating catalogs in: {}", root.display());

    // Simple list catalogs
    validate_list!("03-characters/references/character-archetypes.yaml", ArchetypeCatalogEntry);
    validate_list!("03-characters/references/character-wounds.yaml", WoundCatalogEntry);
    validate_list!("03-characters/references/character-roles.yaml", RoleCatalogEntry);
    validate_list!("03-characters/references/character-drives.yaml", DriveCatalogEntry);
    validate_list!("01-concept/references/plot-types.yaml", PlotTypeCatalogEntry);
    validate_list!("02-collision/references/circle-collision-patterns.yaml", CollisionPatternCatalogEntry);
    validate_list!("02-collision/references/social-circle-types.yaml", SocialCircleCatalogEntry);
    validate_list!("01-concept/references/romance-tropes.yaml", TropeCatalogEntry);

    // Theory catalogs — Genette, Burke, Booth/Hutcheon, Lakoff/Johnson, Trauma theorists, Murray/Lamott
    validate_list!("05-plot-and-structure/references/narrative-time-modes.yaml", NarrativeTimeModeEntry);
    validate_list!("07-drafting/references/irony-types.yaml", IronyTypeCatalogEntry);
    validate_list!("07-drafting/references/comic-modes.yaml", ComicModeCatalogEntry);
    validate_list!("07-drafting/references/metaphor-types.yaml", MetaphorTypeCatalogEntry);
    validate_list!("07-drafting/references/trauma-modes.yaml", TraumaModeEntry);
    validate_list!("07-drafting/references/pentad-elements.yaml", PentadElementEntry);
    validate_list!("08-revision/references/revision-passes.yaml", RevisionPassEntry);

    // Theory catalogs — Austin/Searle, Genette/Bal/Gardner/Cohn, Bloom, Freud/Lacan/Kristeva, Said/Spivak/Bhabha, White/Frye
    validate_list!("07-drafting/references/speech-act-types.yaml", SpeechActEntry);
    validate_list!("07-drafting/references/focalization-modes.yaml", FocalizationModeEntry);
    validate_list!("01-concept/references/intertextual-relations.yaml", IntertextualRelationEntry);
    validate_list!("07-drafting/references/psychoanalytic-mechanisms.yaml", PsychoanalyticMechanismEntry);
    validate_list!("08-revision/references/postcolonial-modes.yaml", PostcolonialModeEntry);
    validate_list!("05-plot-and-structure/references/emplotment-types.yaml", EmplotmentTypeEntry);

    // Theory catalogs — Phase V gap-fill (21 new catalogs)
    validate_list!("07-drafting/references/image-schemas.yaml", ImageSchemaEntry);
    validate_list!("07-drafting/references/subtext-modes.yaml", SubtextModeEntry);
    validate_list!("07-drafting/references/adaptation-modes.yaml", AdaptationModeEntry);
    validate_list!("07-drafting/references/autofiction-modes.yaml", AutofictionModeEntry);
    validate_list!("07-drafting/references/experimental-narration-modes.yaml", ExperimentalNarrationEntry);
    validate_list!("07-drafting/references/philosophy-fiction-modes.yaml", PhilosophyFictionEntry);
    validate_list!("07-drafting/references/verse-prosody.yaml", VerseProsodyEntry);
    validate_list!("08-revision/references/ecocriticism-modes.yaml", EcocriticalModeEntry);
    validate_list!("08-revision/references/feminist-narrative-types.yaml", FeministNarrativeEntry);
    validate_list!("08-revision/references/posthuman-modes.yaml", PosthumanModeEntry);
    validate_list!("08-revision/references/queer-narrative-modes.yaml", QueerNarrativeModeEntry);
    validate_list!("08-revision/references/disability-rep-modes.yaml", DisabilityRepModeEntry);
    validate_list!("08-revision/references/marxist-narrative-modes.yaml", MarxistNarrativeModeEntry);
    validate_list!("08-revision/references/indigenous-narrative-modes.yaml", IndigenousNarrativeModeEntry);
    validate_list!("08-revision/references/affect-modes.yaml", AffectModeEntry);
    validate_list!("08-revision/references/narrative-ethics-modes.yaml", NarrativeEthicsModeEntry);
    validate_list!("references/cognitive-narrative-modes.yaml", CognitiveNarrativeModeEntry);
    validate_list!("references/signifying-modes.yaml", SignifyingModeEntry);
    validate_list!("04-world-building/references/spatial-modes.yaml", SpatialModeEntry);
    validate_list!("05-plot-and-structure/references/propp-functions.yaml", ProppFunctionEntry);
    validate_list!("05-plot-and-structure/references/seriality-types.yaml", SerialityTypeEntry);

    // Theory catalogs — Phase V round-2 (6 new catalogs)
    validate_list!("07-drafting/references/translation-modes.yaml", TranslationModeEntry);
    validate_list!("07-drafting/references/graphic-narrative-modes.yaml", GraphicNarrativeModeEntry);
    validate_list!("07-drafting/references/ya-narrative-modes.yaml", YaNarrativeModeEntry);
    validate_list!("09-polish-and-publish/references/paratext-zones.yaml", ParatextZoneEntry);
    validate_list!("references/semio-linguistic-functions.yaml", SemioLinguisticFunctionEntry);
    validate_list!("01-concept/references/genre-reading-modes.yaml", GenreReadingModeEntry);

    // Genre trope catalogs
    validate_list!("01-concept/references/horror-tropes.yaml", GenreTropeCatalogEntry);
    validate_list!("01-concept/references/scifi-tropes.yaml", GenreTropeCatalogEntry);
    validate_list!("01-concept/references/fantasy-tropes.yaml", GenreTropeCatalogEntry);
    validate_list!("01-concept/references/mystery-tropes.yaml", GenreTropeCatalogEntry);
    validate_list!("01-concept/references/thriller-tropes.yaml", GenreTropeCatalogEntry);
    validate_list!("01-concept/references/literary-fiction-tropes.yaml", GenreTropeCatalogEntry);
    validate_list!("01-concept/references/historical-fiction-tropes.yaml", GenreTropeCatalogEntry);

    // inciting-incidents.yaml has a top-level `categories:` mapping (not a bare list)
    {
        let path = "02-collision/references/inciting-incidents.yaml";
        let full = root.join(path);
        if full.exists() {
            match load_yaml_value::<IncitingIncidentsCatalog>(&full) {
                Ok(catalog) => {
                    let n: usize = catalog.categories.values().map(|c| c.incidents.len()).sum();
                    println!("  ✓ {} ({} incidents across {} categories)", path, n, catalog.categories.len());
                }
                Err(e) => errors.push(format!("{}: {}", path, e)),
            }
        }
    }

    // relationship-roles.yaml: supports canonical mapping and legacy mixed
    // shape via typed adapter (canonicalized to RelationshipRolesCatalogDocument).
    {
        let path = "03-characters/references/relationship-roles.yaml";
        let full = root.join(path);
        if full.exists() {
            match fs::read_to_string(&full) {
                Err(e) => errors.push(format!("{}: {}", path, e)),
                Ok(content) => match grimoire_tooling::catalogs::parse_relationship_roles_catalog(&content) {
                    Ok((doc, used_legacy)) => {
                        if used_legacy {
                            println!(
                                "  \u{26a0} {} ({} roles, {} dynamics via legacy adapter)",
                                path,
                                doc.roles.len(),
                                doc.dynamics.len()
                            );
                        } else {
                            println!(
                                "  \u{2713} {} ({} roles, {} dynamics)",
                                path,
                                doc.roles.len(),
                                doc.dynamics.len()
                            );
                        }
                    }
                    Err(e) => errors.push(format!("{}: {}", path, e)),
                }
            }
        }
    }

    // alignment-system.yaml has axes + cells + arc_patterns (not a bare list)
    {
        let path = "03-characters/references/alignment-system.yaml";
        let full = root.join(path);
        if full.exists() {
            match load_yaml_value::<AlignmentSystemCatalog>(&full) {
                Ok(catalog) => println!(
                    "  ✓ {} ({} cells, {} arc patterns)",
                    path,
                    catalog.cells.len(),
                    catalog.arc_patterns.len()
                ),
                Err(e) => errors.push(format!("{}: {}", path, e)),
            }
        }
    }

    // romance-beats.yaml has core_romance_arc + supplementary_beats (not a bare list)
    {
        let path = "05-plot-and-structure/references/romance-beats.yaml";
        let full = root.join(path);
        if full.exists() {
            match load_yaml_value::<RomanceBeatsCatalog>(&full) {
                Ok(catalog) => {
                    let n = catalog.core_romance_arc.beats.len()
                        + catalog.supplementary_beats
                            .as_ref()
                            .map(|s| s.beats.len())
                            .unwrap_or(0);
                    println!("  ✓ {} ({} beats)", path, n);
                }
                Err(e) => errors.push(format!("{}: {}", path, e)),
            }
        }
    }

    errors
}

/// Load a YAML file as `Vec<T>` (sequence of entries).
/// Handles both single-document and multi-document (`---` separated) YAML files.
fn load_yaml_list<T: serde::de::DeserializeOwned>(path: &Path) -> Result<Vec<T>, String> {
    let content = fs::read_to_string(path).map_err(|e| e.to_string())?;

    // Try single-document parse first
    match serde_yaml::from_str::<Vec<T>>(&content) {
        Ok(v) => return Ok(v),
        Err(_) => {}
    }

    // Fall back to multi-document parse (files with `---` separators)
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

/// Load a YAML file as a typed value `T` (mapping or wrapper struct).
fn load_yaml_value<T: serde::de::DeserializeOwned>(path: &Path) -> Result<T, String> {
    let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
    serde_yaml::from_str(&content).map_err(|e| e.to_string())
}

fn dump_schemas(root: &Path) {
    let schemas = grimoire_tooling::generate_all_schemas();
    let schemas_dir = root.join("schemas");

    let pretty = serde_json::to_string_pretty(&schemas).expect("schema serialisation failed");

    let out = schemas_dir.join("_generated.schema.json");
    fs::create_dir_all(&schemas_dir).expect("could not create schemas/");
    fs::write(&out, &pretty).expect("could not write schema file");
    println!("✓ Wrote schemas to {}", out.display());
}

