/// `grimoire-export-training` — export LLM fine-tuning data from authored drafts.
///
/// Usage:
///   grimoire-export-training [--workspace-path <path>] [--output <file>]
///                            [--dry-run] [--min-words <n>] [--tier minimal|default|all]
///
/// Walks `07-drafting/active-drafting/` for chapter draft files, parses
/// annotation comments (`<!-- key:value -->`), pairs each prose paragraph
/// with its resolved annotation state, and emits a JSONL file of
/// `TrainingExample` records.
///
/// Each line in the output file is one Alpaca-format training pair:
///   `{ "id": "...", "instruction": "...", "input": { ... }, "output": "..." }`
///
/// This format is compatible with HuggingFace `datasets`, Axolotl, and most
/// PEFT fine-tuning pipelines without field remapping.

use std::fs;
use std::path::{Path, PathBuf};

use grimoire_tooling::{
    entities::Scene,
    tags::{parse_annotation_comment, ParagraphAnnotations},
    training::{
        Paragraph, ProseIntent, ProsePassage, SceneContext,
        TierConfig, TrainingExample, TrainingMeta, WorldTexture,
    },
    voice::{FocalizationConfig, VoiceContract},
};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let root = workspace_path(&args);
    let output = output_path(&args, &root);
    let dry_run = args.contains(&"--dry-run".to_owned());
    let min_words: u32 = flag_value(&args, "--min-words")
        .and_then(|s| s.parse().ok())
        .unwrap_or(30);
    let tier = flag_value(&args, "--tier").unwrap_or("default");
    let tier_config = match tier {
        "minimal" => TierConfig::minimal(),
        "all" => TierConfig::all(),
        _ => TierConfig::default(),
    };

    println!("Exporting training data from: {}", root.display());
    println!("Min words per paragraph: {min_words}  Tier: {tier}");
    if dry_run {
        println!("[dry-run] No file will be written.");
    }

    let drafts_dir = root.join("07-drafting/active-drafting");
    let examples = collect_examples(&drafts_dir, &root, min_words, tier_config);

    println!("Found {} training examples.", examples.len());
    let total_words: u32 = examples.iter().map(|e| e.output.word_count).sum();
    println!("Total words: {total_words}");

    if dry_run || examples.is_empty() {
        return;
    }

    let mut lines = String::new();
    for ex in &examples {
        match ex.to_jsonl() {
            Ok(line) => {
                lines.push_str(&line);
                lines.push('\n');
            }
            Err(e) => eprintln!("⚠ Serialisation error for {}: {e}", ex.id),
        }
    }
    fs::write(&output, &lines).expect("could not write JSONL output");
    println!("✓ Wrote {} examples to {}", examples.len(), output.display());
}

// ── Core pipeline ─────────────────────────────────────────────────────────────

/// Collect `TrainingExample` records from all chapter draft files in
/// `drafts_dir`.
///
/// For each Markdown file:
/// 1. Parse annotation comments (`<!-- key:value -->`) and prose paragraphs
///    into paired blocks.
/// 2. Build `ParagraphAnnotations` from each annotation block.
/// 3. Optionally auto-generate `ProseIntent` from the active Tier-1 tags.
/// 4. Wrap into a minimal `SceneContext` + `ProsePassage` → `TrainingExample`.
fn collect_examples(
    drafts_dir: &Path,
    workspace_root: &Path,
    min_words: u32,
    tier_config: TierConfig,
) -> Vec<TrainingExample> {
    let mut examples = Vec::new();

    if !drafts_dir.exists() {
        eprintln!("⚠ Draft directory not found: {}", drafts_dir.display());
        return examples;
    }

    let mut paths: Vec<PathBuf> = fs::read_dir(drafts_dir)
        .unwrap_or_else(|e| {
            eprintln!("⚠ Could not read draft directory: {e}");
            std::process::exit(1);
        })
        .flatten()
        .map(|e| e.path())
        .filter(|p| {
            p.extension().and_then(|e| e.to_str()) == Some("md")
                && !p
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .starts_with('_')
        })
        .collect();
    paths.sort();

    for path in &paths {
        let rel_path = path
            .strip_prefix(workspace_root)
            .unwrap_or(path)
            .to_string_lossy()
            .to_string();
        let stem = path
            .file_stem()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        let content = match fs::read_to_string(path) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("⚠ Could not read {}: {e}", path.display());
                continue;
            }
        };

        // -- Parse annotation–paragraph pairs from file
        let blocks = parse_annotated_paragraphs(&content, min_words);
        if blocks.is_empty() {
            continue;
        }

        // -- Build structured paragraphs
        let mut paragraphs: Vec<Paragraph> = Vec::new();
        let mut para_annotations: Vec<ParagraphAnnotations> = Vec::new();

        for (idx, (raw_anns, text)) in blocks.iter().enumerate() {
            let pa = ParagraphAnnotations::from_annotations(raw_anns);
            let intent = if tier_config.include_intent {
                ProseIntent::from_paragraph_annotations(&pa)
            } else {
                None
            };
            let wc = text.split_whitespace().count() as u32;
            let has_dialogue = text.contains('"')
                || text.contains('\u{201C}')
                || text.contains('\u{2018}');
            para_annotations.push(pa.clone());
            paragraphs.push(Paragraph {
                index: idx as u32,
                text: text.clone(),
                word_count: wc,
                annotations: pa,
                intent,
                contains_dialogue: has_dialogue,
                sentences: vec![],
            });
        }

        if paragraphs.is_empty() {
            continue;
        }

        // -- Build minimal SceneContext
        let scene = minimal_scene(&stem, &stem.replace('-', " "));
        let instruction = TrainingExample::default_instruction(&scene);
        let ctx = SceneContext {
            scene: scene.clone(),
            voice_contract: VoiceContract::default(),
            focalization: FocalizationConfig::default(),
            paragraph_annotations: para_annotations,
            world_texture: WorldTexture {
                setting: None,
                active_motifs: vec![],
                foregrounded_sensory: vec![],
            },
            character_states: vec![],
            story_position: None,
            act: None,
            beat_ref: None,
            chapter_ref: None,
            preceding_context: None,
            preceding_scene_summary: None,
            tier_config: tier_config.clone(),
        };

        // -- Build ProsePassage
        let full_text = paragraphs
            .iter()
            .map(|p| p.text.as_str())
            .collect::<Vec<_>>()
            .join("\n\n");
        let total_wc = paragraphs.iter().map(|p| p.word_count).sum();
        let passage = ProsePassage {
            text: full_text,
            paragraphs: paragraphs.clone(),
            dialogue_segments: vec![],
            word_count: total_wc,
            dominant_sense: None,
        };

        // -- Assemble TrainingExample
        let id = format!("{}_{}", stem, examples.len());
        let meta = TrainingMeta {
            chapter_id: Some(stem.clone()),
            chapter_number: None,
            scene_id: None,
            word_count: total_wc,
            revision_pass: 0,
            session_notes: None,
            post_hoc_flags: vec![],
            source_path: Some(rel_path),
        };
        examples.push(TrainingExample {
            id,
            instruction,
            context: ctx,
            output: passage,
            meta,
        });
    }

    examples
}

// ── Markdown parser ───────────────────────────────────────────────────────────

/// Parse a Markdown file into `(annotations, prose_text)` blocks.
///
/// Annotation comments (`<!-- key:value -->`) that appear before or adjacent
/// to a paragraph are associated with that paragraph.  The "sticky" rule: once
/// set, an annotation type holds until overridden by a later comment of the
/// same key.  However, for the training pipeline we use a simpler rule —
/// each block carries only the annotations that appeared immediately before it
/// (within the same blank-line-delimited section), which avoids false
/// "inheritance" across scene boundaries.
fn parse_annotated_paragraphs(
    content: &str,
    min_words: u32,
) -> Vec<(Vec<grimoire_tooling::tags::Annotation>, String)> {
    let mut blocks: Vec<(Vec<grimoire_tooling::tags::Annotation>, String)> = Vec::new();
    let mut in_frontmatter = false;
    let mut pending_anns: Vec<grimoire_tooling::tags::Annotation> = Vec::new();
    let mut para_buf = String::new();

    for line in content.lines() {
        let trimmed = line.trim();

        // YAML frontmatter
        if trimmed == "---" {
            in_frontmatter = !in_frontmatter;
            continue;
        }
        if in_frontmatter {
            continue;
        }

        // HTML annotation comment
        if trimmed.starts_with("<!--") && trimmed.ends_with("-->") {
            let (anns, _warns) = parse_annotation_comment(trimmed);
            pending_anns.extend(anns);
            continue;
        }

        // Markdown heading — treat as paragraph boundary
        if trimmed.starts_with('#') {
            flush_block(
                &mut para_buf,
                &mut pending_anns,
                &mut blocks,
                min_words,
            );
            continue;
        }

        // Blank line — paragraph boundary
        if trimmed.is_empty() {
            flush_block(
                &mut para_buf,
                &mut pending_anns,
                &mut blocks,
                min_words,
            );
            continue;
        }

        // Prose text
        if !para_buf.is_empty() {
            para_buf.push(' ');
        }
        para_buf.push_str(trimmed);
    }

    // Flush final paragraph
    flush_block(&mut para_buf, &mut pending_anns, &mut blocks, min_words);

    blocks
}

/// Push a completed paragraph block into `blocks`, clearing the buffers.
fn flush_block(
    para_buf: &mut String,
    pending_anns: &mut Vec<grimoire_tooling::tags::Annotation>,
    blocks: &mut Vec<(Vec<grimoire_tooling::tags::Annotation>, String)>,
    min_words: u32,
) {
    let text = para_buf.trim().to_owned();
    if !text.is_empty() && text.split_whitespace().count() as u32 >= min_words {
        blocks.push((std::mem::take(pending_anns), text));
    } else {
        pending_anns.clear();
    }
    para_buf.clear();
}

// ── Entity builders ───────────────────────────────────────────────────────────

/// Build a minimal `Scene` with only required fields set; everything optional
/// is `None` / `vec![]`.
fn minimal_scene(id: &str, title: &str) -> Scene {
    Scene {
        id: id.to_owned(),
        working_title: Some(title.to_owned()),
        story_position: None,
        pov_character: None,
        attending_characters: vec![],
        setting: None,
        time_of_day: None,
        weather: None,
        goal: None,
        why_goal_matters: None,
        plan: None,
        opponent_or_obstacle: None,
        conflict_type: vec![],
        escalation_beats: vec![],
        dialogue_strategy: None,
        action_strategy: None,
        emotional_escalation: None,
        outcome_type: None,
        what_changed: None,
        new_information: None,
        plant_or_setup: None,
        sequel: None,
        dominant_sense: None,
        key_sensory_details: vec![],
        emotional_weather: None,
        scene_unique_image: None,
        pacing_notes: None,
        target_word_count: None,
        complexity: None,
        priority: None,
        narrative_threads: vec![],
        sequence_id: None,
        scene_type: None,
        tension_level: None,
    }
}

// ── CLI helpers ───────────────────────────────────────────────────────────────

fn workspace_path(args: &[String]) -> PathBuf {
    if let Some(pos) = args.iter().position(|a| a == "--workspace-path") {
        PathBuf::from(&args[pos + 1])
    } else {
        PathBuf::from(".")
    }
}

fn output_path(args: &[String], root: &Path) -> PathBuf {
    if let Some(pos) = args.iter().position(|a| a == "--output") {
        PathBuf::from(&args[pos + 1])
    } else {
        root.join("training-data.jsonl")
    }
}

fn flag_value<'a>(args: &'a [String], flag: &str) -> Option<&'a str> {
    args.iter()
        .position(|a| a == flag)
        .and_then(|p| args.get(p + 1))
        .map(|s| s.as_str())
}
