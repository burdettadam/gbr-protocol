//! Tag constraint graph — formal relationships between annotation tags.
//!
//! The constraint graph encodes three kinds of relationships:
//!
//! | Kind | Meaning |
//! |------|---------|
//! | `Implies`    | If tag A is active, tag B is expected (soft recommendation) |
//! | `Excludes`   | Tags A and B cannot both be active (logical contradiction) |
//! | `Requires`   | Tag A cannot appear without tag B (hard dependency) |
//! | `Correlates` | A and B statistically co-occur (advisory, not validated) |
//!
//! The graph is used in two ways:
//! 1. **Validation** at annotation parse time: `ConstraintGraph::validate` returns
//!    `Vec<ConstraintViolation>` so the CLI can surface errors/warnings.
//! 2. **Training augmentation**: when an implication is known but the implied
//!    tag is absent, an export pipeline can infer it (marked `auto_derived`).
//!
//! ## Seed constraints (20 core rules)
//!
//! Built into [`ConstraintGraph::default`].  Additional project-specific
//! constraints can be pushed onto the graph at runtime.

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::tags::{annotation_to_pair, ParagraphAnnotations};

// ── Tag predicate ─────────────────────────────────────────────────────────────

/// A predicate that matches a tag by key and optionally by value.
///
/// `TagPredicate { key: "psychic_distance", value: None }` matches any
/// `psychic_distance` tag; `{ key: "consciousness", value: Some("quoted_monologue") }`
/// matches only that exact value.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct TagPredicate {
    /// Canonical tag key (from the tag vocabulary).
    pub key: String,
    /// Optional value constraint.  `None` = any value of this key.
    pub value: Option<String>,
}

impl TagPredicate {
    pub fn key(key: impl Into<String>) -> Self {
        Self { key: key.into(), value: None }
    }

    pub fn exact(key: impl Into<String>, value: impl Into<String>) -> Self {
        Self { key: key.into(), value: Some(value.into()) }
    }

    /// Returns `true` when this predicate matches the given (key, value) pair.
    pub fn matches(&self, tag_key: &str, tag_value: &str) -> bool {
        if self.key != tag_key {
            return false;
        }
        match &self.value {
            Some(v) => v == tag_value,
            None => true,
        }
    }
}

// ── Constraint kind ────────────────────────────────────────────────────────────

/// The semantic relationship expressed by a `TagConstraint`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ConstraintKind {
    /// When the antecedent is active, the consequent is expected to be set.
    /// Violation = warning only.
    Implies,
    /// The antecedent and consequent are logically incompatible.
    /// Violation = error.
    Excludes,
    /// The antecedent cannot appear without the consequent also being set.
    /// Violation = error.
    Requires,
    /// The two predicates frequently co-occur (advisory, never an error).
    Correlates,
}

// ── Constraint severity ───────────────────────────────────────────────────────

/// How severely a constraint violation should be treated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ConstraintSeverity {
    /// Hard error — the annotation set is logically inconsistent.
    Error,
    /// Advisory warning — the combination is suspicious but valid.
    Warning,
    /// Informational — worth noting for analysis, not a problem.
    Info,
}

// ── Constraint ────────────────────────────────────────────────────────────────

/// A single formal relationship between two tag predicates.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct TagConstraint {
    /// Stable identifier (snake_case).
    pub id: String,
    pub kind: ConstraintKind,
    /// The tag that triggers the constraint check.
    pub antecedent: TagPredicate,
    /// The tag that is implied / excluded / required.
    pub consequent: TagPredicate,
    pub severity: ConstraintSeverity,
    /// Human-readable explanation of why this constraint exists.
    pub rationale: String,
    /// Theoretical provenance (e.g. "Gardner psychic distance scale").
    pub theory_source: Option<String>,
}

impl TagConstraint {
    fn implies(
        id: &str,
        ant_key: &str,
        ant_val: Option<&str>,
        con_key: &str,
        con_val: Option<&str>,
        rationale: &str,
        theory_source: Option<&str>,
    ) -> Self {
        Self {
            id: id.into(),
            kind: ConstraintKind::Implies,
            antecedent: TagPredicate {
                key: ant_key.into(),
                value: ant_val.map(Into::into),
            },
            consequent: TagPredicate {
                key: con_key.into(),
                value: con_val.map(Into::into),
            },
            severity: ConstraintSeverity::Warning,
            rationale: rationale.into(),
            theory_source: theory_source.map(Into::into),
        }
    }

    fn excludes(
        id: &str,
        ant_key: &str,
        ant_val: Option<&str>,
        con_key: &str,
        con_val: Option<&str>,
        rationale: &str,
        theory_source: Option<&str>,
    ) -> Self {
        Self {
            id: id.into(),
            kind: ConstraintKind::Excludes,
            antecedent: TagPredicate {
                key: ant_key.into(),
                value: ant_val.map(Into::into),
            },
            consequent: TagPredicate {
                key: con_key.into(),
                value: con_val.map(Into::into),
            },
            severity: ConstraintSeverity::Error,
            rationale: rationale.into(),
            theory_source: theory_source.map(Into::into),
        }
    }

    fn requires(
        id: &str,
        ant_key: &str,
        ant_val: Option<&str>,
        con_key: &str,
        con_val: Option<&str>,
        rationale: &str,
        theory_source: Option<&str>,
    ) -> Self {
        Self {
            id: id.into(),
            kind: ConstraintKind::Requires,
            antecedent: TagPredicate {
                key: ant_key.into(),
                value: ant_val.map(Into::into),
            },
            consequent: TagPredicate {
                key: con_key.into(),
                value: con_val.map(Into::into),
            },
            severity: ConstraintSeverity::Error,
            rationale: rationale.into(),
            theory_source: theory_source.map(Into::into),
        }
    }

    fn correlates(
        id: &str,
        ant_key: &str,
        ant_val: Option<&str>,
        con_key: &str,
        con_val: Option<&str>,
        rationale: &str,
    ) -> Self {
        Self {
            id: id.into(),
            kind: ConstraintKind::Correlates,
            antecedent: TagPredicate {
                key: ant_key.into(),
                value: ant_val.map(Into::into),
            },
            consequent: TagPredicate {
                key: con_key.into(),
                value: con_val.map(Into::into),
            },
            severity: ConstraintSeverity::Info,
            rationale: rationale.into(),
            theory_source: None,
        }
    }
}

// ── Constraint violation ──────────────────────────────────────────────────────

/// A constraint violation found during `ConstraintGraph::validate`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct ConstraintViolation {
    /// The constraint that was violated.
    pub constraint_id: String,
    pub kind: ConstraintKind,
    pub severity: ConstraintSeverity,
    /// The active (key, value) pair that triggered the check.
    pub triggering_tag: (String, String),
    /// The missing or conflicting (key, value) pair.
    pub related_tag: (String, String),
    pub message: String,
}

// ── Constraint graph ──────────────────────────────────────────────────────────

/// The full set of tag constraints, with validation logic.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ConstraintGraph {
    pub constraints: Vec<TagConstraint>,
}

impl Default for ConstraintGraph {
    fn default() -> Self {
        Self { constraints: seed_constraints() }
    }
}

impl ConstraintGraph {
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a custom constraint to the graph.
    pub fn push(&mut self, c: TagConstraint) {
        self.constraints.push(c);
    }

    /// Validate a set of (key, value) annotation pairs.
    ///
    /// Returns a list of violations.  An empty list means the annotation set
    /// is consistent with all registered constraints.
    ///
    /// `annotations` is expected to be a flat list of `(canonical_key, value)`
    /// pairs as emitted by the annotation parser.
    pub fn validate<'a>(
        &self,
        annotations: impl IntoIterator<Item = (&'a str, &'a str)>,
    ) -> Vec<ConstraintViolation> {
        let pairs: Vec<(&str, &str)> = annotations.into_iter().collect();
        let mut violations = Vec::new();

        for constraint in &self.constraints {
            // Find all triggering pairs
            for (key, value) in &pairs {
                if !constraint.antecedent.matches(key, value) {
                    continue;
                }
                // Antecedent is active — check the consequent
                match constraint.kind {
                    ConstraintKind::Implies => {
                        let consequent_active = pairs
                            .iter()
                            .any(|(k, v)| constraint.consequent.matches(k, v));
                        if !consequent_active {
                            violations.push(ConstraintViolation {
                                constraint_id: constraint.id.clone(),
                                kind: constraint.kind,
                                severity: constraint.severity,
                                triggering_tag: (key.to_string(), value.to_string()),
                                related_tag: (
                                    constraint.consequent.key.clone(),
                                    constraint.consequent.value.clone().unwrap_or_default(),
                                ),
                                message: format!(
                                    "Tag '{}:{}' implies '{}{}' should also be set — {}",
                                    key,
                                    value,
                                    constraint.consequent.key,
                                    constraint
                                        .consequent
                                        .value
                                        .as_deref()
                                        .map(|v| format!(":{v}"))
                                        .unwrap_or_default(),
                                    constraint.rationale
                                ),
                            });
                        }
                    }
                    ConstraintKind::Excludes => {
                        for (k2, v2) in &pairs {
                            if constraint.consequent.matches(k2, v2) {
                                violations.push(ConstraintViolation {
                                    constraint_id: constraint.id.clone(),
                                    kind: constraint.kind,
                                    severity: constraint.severity,
                                    triggering_tag: (key.to_string(), value.to_string()),
                                    related_tag: (k2.to_string(), v2.to_string()),
                                    message: format!(
                                        "Tags '{}:{}' and '{}:{}' are incompatible — {}",
                                        key, value, k2, v2, constraint.rationale
                                    ),
                                });
                            }
                        }
                    }
                    ConstraintKind::Requires => {
                        let dependency_met = pairs
                            .iter()
                            .any(|(k, v)| constraint.consequent.matches(k, v));
                        if !dependency_met {
                            violations.push(ConstraintViolation {
                                constraint_id: constraint.id.clone(),
                                kind: constraint.kind,
                                severity: constraint.severity,
                                triggering_tag: (key.to_string(), value.to_string()),
                                related_tag: (
                                    constraint.consequent.key.clone(),
                                    constraint.consequent.value.clone().unwrap_or_default(),
                                ),
                                message: format!(
                                    "Tag '{}:{}' requires '{}{}' to also be set — {}",
                                    key,
                                    value,
                                    constraint.consequent.key,
                                    constraint
                                        .consequent
                                        .value
                                        .as_deref()
                                        .map(|v| format!(":{v}"))
                                        .unwrap_or_default(),
                                    constraint.rationale
                                ),
                            });
                        }
                    }
                    // Correlates is advisory — never generates a violation.
                    ConstraintKind::Correlates => {}
                }
            }
        }

        violations
    }

    /// Return only `Error`-severity violations from a validation run.
    pub fn errors<'a>(
        &self,
        annotations: impl IntoIterator<Item = (&'a str, &'a str)>,
    ) -> Vec<ConstraintViolation> {
        self.validate(annotations)
            .into_iter()
            .filter(|v| v.severity == ConstraintSeverity::Error)
            .collect()
    }

    /// Return only `Warning`-severity violations from a validation run.
    pub fn warnings<'a>(
        &self,
        annotations: impl IntoIterator<Item = (&'a str, &'a str)>,
    ) -> Vec<ConstraintViolation> {
        self.validate(annotations)
            .into_iter()
            .filter(|v| v.severity == ConstraintSeverity::Warning)
            .collect()
    }

    /// Lookup a constraint by its stable ID.
    pub fn get(&self, id: &str) -> Option<&TagConstraint> {
        self.constraints.iter().find(|c| c.id == id)
    }

    /// Validate a single [`ParagraphAnnotations`] struct.
    ///
    /// Extracts the typed fields and any `extra` annotations into constraint
    /// pairs, then runs them through the graph.  This is the primary
    /// entry-point for per-paragraph consistency checking during draft review
    /// and training-example validation.
    pub fn validate_paragraph(&self, ann: &ParagraphAnnotations) -> Vec<ConstraintViolation> {
        let pairs = ann.to_constraint_pairs();
        let pair_refs: Vec<(&str, &str)> = pairs
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect();
        self.validate(pair_refs)
    }

    /// Validate all paragraphs in a `SceneContext.paragraph_annotations` slice
    /// and return every violation across all paragraphs, tagged with the
    /// paragraph index for reporting.
    ///
    /// The returned violations are identical to those from `validate_paragraph`
    /// but the `constraint_id` is prefixed with `"para_{N}/"` so callers can
    /// identify which paragraph triggered each one.
    pub fn validate_scene_annotations(
        &self,
        paragraphs: &[ParagraphAnnotations],
    ) -> Vec<ConstraintViolation> {
        let mut all = Vec::new();
        for (i, para) in paragraphs.iter().enumerate() {
            let mut vs = self.validate_paragraph(para);
            for v in &mut vs {
                v.constraint_id = format!("para_{i}/{}", v.constraint_id);
            }
            all.extend(vs);
        }
        all
    }

    /// Utility: extract `(key, value)` pairs from a single [`Annotation`]
    /// using the [`annotation_to_pair`] helper.  Returns `None` for complex
    /// multi-field variants.
    pub fn pairs_from_extra<'a>(
        extra: impl IntoIterator<Item = &'a crate::tags::Annotation>,
    ) -> Vec<(String, String)> {
        extra.into_iter().filter_map(annotation_to_pair).collect()
    }
}

// ── Seed constraints ──────────────────────────────────────────────────────────

/// The 20 core constraints built into every `ConstraintGraph`.
///
/// These are grounded in named literary theories (cited in `theory_source`).
fn seed_constraints() -> Vec<TagConstraint> {
    vec![
        // ── Psychic distance × Consciousness (Gardner scale / Cohn modes)
        TagConstraint::implies(
            "pd1_implies_consciousness",
            "psychic_distance", Some("1"),
            "consciousness", None,
            "Psychic distance 1 (deepest interiority) should specify a Cohn \
             consciousness mode (quoted_monologue or narrated_monologue).",
            Some("Gardner 1983 psychic distance scale; Cohn 1978"),
        ),
        TagConstraint::excludes(
            "pd5_excludes_quoted_monologue",
            "psychic_distance", Some("5"),
            "consciousness", Some("quoted_monologue"),
            "Psychic distance 5 (panoramic/summary) is incompatible with \
             quoted_monologue (direct interior speech — a close interiority marker).",
            Some("Gardner 1983; Cohn 1978 Transparent Minds"),
        ),
        TagConstraint::excludes(
            "pd5_excludes_narrated_monologue",
            "psychic_distance", Some("5"),
            "consciousness", Some("narrated_monologue"),
            "Psychic distance 5 does not support narrated monologue (deep FID). \
             Use psychonarration instead.",
            Some("Gardner 1983; Cohn 1978"),
        ),
        TagConstraint::implies(
            "pd1_implies_pov",
            "psychic_distance", Some("1"),
            "pov", None,
            "Deep interiority (PD 1) requires an explicit POV declaration.",
            Some("Gardner 1983"),
        ),

        // ── Tension × Sentence length (craft correlation)
        TagConstraint::correlates(
            "high_tension_correlates_short_sentences",
            "tension", None,
            "sentence_length", Some("short"),
            "High tension tends to correlate with shorter sentence rhythms.",
        ),

        // ── Speech act requires a character speaker
        TagConstraint::requires(
            "speech_act_requires_character",
            "speech_act", None,
            "character", None,
            "A speech_act annotation implies a character is performing the act; \
             an active character tag should be present.",
            Some("Searle 1969 Speech Acts; Austin 1962 How to Do Things with Words"),
        ),

        // ── Gaze × Feminist (audit pairing)
        TagConstraint::implies(
            "male_gaze_implies_feminist_audit",
            "gaze", Some("male_gaze"),
            "feminist", None,
            "Deploying the male gaze warrants an explicit feminist narratology \
             audit annotation to signal intentional critical engagement.",
            Some("Mulvey 1975; Lanser 1992"),
        ),

        // ── Diegetic level × Thread
        TagConstraint::implies(
            "metadiegetic_implies_embedded_thread",
            "diegetic_level", Some("metadiegetic"),
            "thread", None,
            "A metadiegetic level (embedded story within story) should reference \
             the embedded thread via a thread tag.",
            Some("Genette 1980 Narrative Discourse"),
        ),

        // ── Autofiction pact × POV
        TagConstraint::implies(
            "autofiction_implies_first_person",
            "pact", Some("autofictional"),
            "pov", Some("first_person"),
            "Autofictional pact (Doubrovsky/Lejeune) conventionally uses first-person narration.",
            Some("Lejeune 1989; Doubrovsky 1977"),
        ),
        TagConstraint::excludes(
            "autobiographical_pact_excludes_third_person",
            "pact", Some("autobiographical"),
            "pov", Some("third_person_omniscient"),
            "Third-person omniscient narration breaks the autobiographical pact \
             (Lejeune name test).",
            Some("Lejeune 1989 On Autobiography"),
        ),

        // ── Experimental × POV
        TagConstraint::implies(
            "second_person_experimental_implies_pov",
            "experimental", Some("second_person"),
            "pov", Some("second_person"),
            "Second-person experimental narration should declare second_person POV.",
            Some("McHale 1987 Postmodernist Fiction"),
        ),

        // ── Trauma × Recovery stage dependency
        TagConstraint::implies(
            "trauma_implies_recovery_stage",
            "trauma_mode", None,
            "recovery_stage", None,
            "Trauma mode annotations are more useful when paired with a Herman \
             recovery stage for clinical-narratological completeness.",
            Some("Herman 1997 Trauma and Recovery"),
        ),

        // ── Propp function requires beat context
        TagConstraint::requires(
            "propp_requires_beat",
            "propp", None,
            "beat", None,
            "Propp morphological function annotations should be anchored to a \
             structural beat for cross-reference validation.",
            Some("Propp 1928 Morphology of the Folktale"),
        ),

        // ── Temporal analepsis requires beat (flashback anchoring)
        TagConstraint::implies(
            "analepsis_implies_beat",
            "temporal_order", Some("analepsis"),
            "beat", None,
            "Analepsis (flashback) should reference the beat or scene it returns \
             to, for narrative time tracking.",
            Some("Genette 1980 §1 order"),
        ),

        // ── Psychoanalytic gaze requires character
        TagConstraint::requires(
            "gaze_requires_character",
            "gaze", None,
            "character", None,
            "Gaze annotations (Mulvey's apparatus) require an active character \
             context — whose gaze is operating?",
            Some("Mulvey 1975; Lacan mirror stage"),
        ),

        // ── Actant requires character or setting reference
        TagConstraint::requires(
            "actant_requires_entity",
            "actant", None,
            "character", None,
            "Actantial role (Greimas) should be paired with its character entity \
             reference for cross-phase validation.",
            Some("Greimas 1966 Structural Semantics"),
        ),

        // ── Consciousness mode × POV consistency
        TagConstraint::excludes(
            "third_person_omniscient_excludes_quoted_monologue",
            "pov", Some("third_person_omniscient"),
            "consciousness", Some("quoted_monologue"),
            "Omniscient narration rarely uses quoted interior monologue \
             (Cohn's mode 1 — that is a first-person or close-third technique).",
            Some("Cohn 1978; Stanzel 1984"),
        ),

        // ── Reliability × Evidence
        TagConstraint::implies(
            "unreliable_narrator_implies_knowledge_scope",
            "reliability", Some("unreliable"),
            "knowledge", None,
            "An unreliable narrator tag should be paired with a knowledge scope \
             annotation declaring the source of unreliability.",
            Some("Booth 1961 Rhetoric of Fiction; Nünning 1999"),
        ),

        // ── Postcolonial mode → representation audit
        TagConstraint::implies(
            "postcolonial_implies_ethics_audit",
            "postcolonial", None,
            "ethics", None,
            "Postcolonial mode annotations should pair with a narrative ethics \
             audit tag when deploying frameworks that carry representation stakes.",
            Some("Said 1978; Spivak 1988; Achebe 1977"),
        ),

        // ── Visual-verbal × graphic narrative requirement
        TagConstraint::requires(
            "visual_verbal_requires_panel_transition",
            "visual_verbal", None,
            "panel_transition", None,
            "Visual-verbal relation (Chute) is specific to graphic narrative and \
             should pair with a McCloud panel transition type.",
            Some("McCloud 1993 Understanding Comics; Chute 2016"),
        ),
    ]
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn pairs<'a>(raw: &'a [(&'a str, &'a str)]) -> Vec<(&'a str, &'a str)> {
        raw.to_vec()
    }

    #[test]
    fn pd5_excludes_quoted_monologue_fires() {
        let graph = ConstraintGraph::default();
        let tags = pairs(&[("psychic_distance", "5"), ("consciousness", "quoted_monologue")]);
        let viols = graph.errors(tags.iter().copied());
        assert!(
            viols.iter().any(|v| v.constraint_id == "pd5_excludes_quoted_monologue"),
            "expected pd5_excludes_quoted_monologue violation"
        );
    }

    #[test]
    fn pd5_and_psychonarration_is_clean() {
        let graph = ConstraintGraph::default();
        let tags = pairs(&[("psychic_distance", "5"), ("consciousness", "psychonarration")]);
        let errors = graph.errors(tags.iter().copied());
        assert!(errors.is_empty(), "pd5 + psychonarration should produce no errors");
    }

    #[test]
    fn speech_act_without_character_is_error() {
        let graph = ConstraintGraph::default();
        let tags = pairs(&[("speech_act", "directive")]);
        let errors = graph.errors(tags.iter().copied());
        assert!(
            errors.iter().any(|v| v.constraint_id == "speech_act_requires_character"),
            "speech_act without character should be an error"
        );
    }

    #[test]
    fn speech_act_with_character_is_clean() {
        let graph = ConstraintGraph::default();
        let tags = pairs(&[("speech_act", "directive"), ("character", "alice")]);
        let errors = graph.errors(tags.iter().copied());
        assert!(errors.is_empty());
    }

    #[test]
    fn autofiction_implies_first_person_warning() {
        let graph = ConstraintGraph::default();
        let tags = pairs(&[("pact", "autofictional")]);
        let warns = graph.warnings(tags.iter().copied());
        assert!(
            warns.iter().any(|v| v.constraint_id == "autofiction_implies_first_person"),
            "autofiction without pov should warn"
        );
    }

    #[test]
    fn pd1_with_correct_consciousness_no_pd5_violation() {
        let graph = ConstraintGraph::default();
        let tags = pairs(&[("psychic_distance", "1"), ("consciousness", "narrated_monologue")]);
        let errors = graph.errors(tags.iter().copied());
        // pd5 excludes should NOT fire for pd1
        assert!(
            !errors.iter().any(|v| v.constraint_id.starts_with("pd5")),
            "pd5 constraints should not fire for pd1"
        );
    }

    #[test]
    fn custom_constraint_push() {
        let mut graph = ConstraintGraph::new();
        graph.push(TagConstraint {
            id: "custom_test".into(),
            kind: ConstraintKind::Excludes,
            antecedent: TagPredicate::exact("pov", "first_person"),
            consequent: TagPredicate::exact("pov", "third_person_omniscient"),
            severity: ConstraintSeverity::Error,
            rationale: "Cannot have two POV modes simultaneously.".into(),
            theory_source: None,
        });
        let tags = pairs(&[("pov", "first_person"), ("pov", "third_person_omniscient")]);
        let errors = graph.errors(tags.iter().copied());
        assert!(errors.iter().any(|v| v.constraint_id == "custom_test"));
    }
}
