//! Gate system types — typed port of `grimoire/models.py` gate dataclasses.
//!
//! These model the readiness-check system defined in each phase's `_gate.md`.
//! All previously-Python dataclasses are now Rust structs with full serde
//! and JSON Schema support.

use std::collections::HashMap;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cap_narrative_types::enums::{CheckType, GateStatus, PhaseStatus, Severity, SubPhaseStatus};

// ── Sub-phase spec ────────────────────────────────────────────────────────────

/// Declaration of a sub-phase within a phase's `_gate.md` frontmatter.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub struct SubPhaseSpec {
    pub id: String,
    pub label: String,
    pub order: u32,
    /// Local sub-phase IDs or cross-phase FQIDs this sub-phase depends on
    pub depends_on: Vec<String>,
}

impl SubPhaseSpec {
    /// Fully-qualified ID, e.g. `"03-characters/core-identity"`.
    pub fn fqid(&self, phase_id: &str) -> String {
        format!("{}/{}", phase_id, self.id)
    }
}

// ── Gate spec ─────────────────────────────────────────────────────────────────

/// A single gate check as declared in `_gate.md` YAML.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct GateSpec {
    pub id: String,
    pub question: String,
    pub check_type: CheckType,
    pub severity: Severity,
    /// Sub-phase this gate belongs to (None = ungrouped)
    pub sub_phase: Option<String>,
    /// Path to the file being checked (relative to workspace root)
    pub target_file: Option<String>,
    /// For cross-ref checks: the source file containing the tags
    pub source_file: Option<String>,
    pub source_tag: Option<String>,
    pub target_tag: Option<String>,
    /// `placeholder_ratio` check: maximum allowed fraction 0.0–1.0
    pub max_placeholder_pct: Option<f32>,
    /// `word_count_min` check: minimum word count
    pub min_words: Option<u32>,
    /// `checkbox_completion` check: minimum fraction 0.0–1.0
    pub min_completion_pct: Option<f32>,
}

// ── Gate result ────────────────────────────────────────────────────────────────

/// The result of running a single gate check.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct GateResult {
    pub gate_id: String,
    pub question: String,
    pub severity: Severity,
    pub status: GateStatus,
    pub detail: Option<String>,
    pub sub_phase: Option<String>,
}

// ── Sub-phase result ───────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct SubPhaseResult {
    pub spec: SubPhaseSpec,
    pub status: SubPhaseStatus,
    /// Number of green gates (each required gate = 2 pts, recommended = 1 pt)
    pub score: u32,
    pub gates: HashMap<String, GateResult>,
}

// ── Phase spec ────────────────────────────────────────────────────────────────

/// Complete phase specification parsed from `_gate.md` frontmatter.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct PhaseSpec {
    pub phase_id: String,
    pub phase_label: String,
    pub sub_phases: Vec<SubPhaseSpec>,
    pub gates: Vec<GateSpec>,
}

impl PhaseSpec {
    /// Return the FQID for a given sub-phase local ID.
    pub fn fqid(&self, sub_phase_id: &str) -> String {
        format!("{}/{}", self.phase_id, sub_phase_id)
    }

    /// Find a gate by ID.
    pub fn gate(&self, gate_id: &str) -> Option<&GateSpec> {
        self.gates.iter().find(|g| g.id == gate_id)
    }
}

// ── Phase result ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct PhaseResult {
    pub phase_id: String,
    pub phase_label: String,
    pub status: PhaseStatus,
    pub score: u32,
    pub error: Option<String>,
    pub sub_phase_results: HashMap<String, SubPhaseResult>,
    pub gate_results: HashMap<String, GateResult>,
}

// ── Dependency edge ────────────────────────────────────────────────────────────

/// A directed dependency edge between two sub-phase FQIDs.
///
/// e.g. `"03-characters/core-identity" → "03-characters/relationships"`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema)]
pub struct DependencyEdge {
    pub source: String,
    pub target: String,
}

// ── Readiness summary ─────────────────────────────────────────────────────────

/// Top-level readiness output written to `readiness.yaml`.
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ReadinessSummary {
    pub phases: HashMap<String, PhaseResult>,
}

impl ReadinessSummary {
    /// All sub-phase FQIDs that are currently COMPLETE.
    pub fn completed_fqids(&self) -> std::collections::HashSet<String> {
        let mut set = std::collections::HashSet::new();
        for phase in self.phases.values() {
            for (fqid, result) in &phase.sub_phase_results {
                if result.status == SubPhaseStatus::Complete {
                    set.insert(fqid.clone());
                }
            }
        }
        set
    }
}
