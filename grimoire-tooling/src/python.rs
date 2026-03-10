//! PyO3 Python bindings for `grimoire-types`.
//!
//! Exposes key types to Python so the existing engine scripts can adopt
//! the Rust type system incrementally.  Each binding provides:
//! - `__init__` via dataclass-style `#[new]`
//! - `to_json()` / `from_json()` for serde round-trip
//! - `to_dict()` (returns a Python dict for downstream YAML serialisation)
//!
//! Build with `maturin develop --features python` from the workspace root.

use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use pyo3::types::{PyDict, PyList};

// ── Registration ───────────────────────────────────────────────────────────────

pub fn register(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Enums
    m.add_class::<PyArchetype>()?;
    m.add_class::<PyWound>()?;
    m.add_class::<PyAlignment>()?;
    m.add_class::<PyRole>()?;
    m.add_class::<PyDriveModel>()?;
    m.add_class::<PyGateStatus>()?;
    m.add_class::<PySubPhaseStatus>()?;
    m.add_class::<PyPhaseStatus>()?;
    m.add_class::<PyRevisionFlag>()?;
    // Core types
    m.add_class::<PyEntityRef>()?;
    m.add_class::<PyAnnotation>()?;
    m.add_class::<PyCharacter>()?;
    m.add_class::<PyScene>()?;
    m.add_class::<PyVoiceContract>()?;
    m.add_class::<PyVoiceSignature>()?;
    m.add_class::<PyTrainingExample>()?;
    m.add_class::<PyTrainingDataset>()?;
    m.add_class::<PyGateSpec>()?;
    m.add_class::<PyPhaseSpec>()?;
    m.add_class::<PyGateResult>()?;
    m.add_class::<PyPhaseResult>()?;
    m.add_class::<PyStoryRecipe>()?;
    // Functions
    m.add_function(wrap_pyfunction!(parse_annotation, m)?)?;
    m.add_function(wrap_pyfunction!(dump_schemas, m)?)?;
    Ok(())
}

// ── Helper macro: JSON-based round-trip for any serde type ────────────────────

macro_rules! json_methods {
    ($py_type:ty, $inner:ty) => {
        #[pymethods]
        impl $py_type {
            fn to_json(&self) -> PyResult<String> {
                serde_json::to_string(&self.inner)
                    .map_err(|e| PyValueError::new_err(e.to_string()))
            }

            #[staticmethod]
            fn from_json(json: &str) -> PyResult<Self> {
                serde_json::from_str(json)
                    .map(|inner| Self { inner })
                    .map_err(|e| PyValueError::new_err(e.to_string()))
            }

            fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
                let json = serde_json::to_value(&self.inner)
                    .map_err(|e| PyValueError::new_err(e.to_string()))?;
                pythonize::pythonize(py, &json)
                    .map_err(|e| PyValueError::new_err(e.to_string()))
                    .and_then(|v| v.extract::<Bound<'py, PyDict>>())
            }

            fn __repr__(&self) -> String {
                serde_json::to_string(&self.inner)
                    .unwrap_or_else(|_| "<grimoire_types object>".to_owned())
            }
        }
    };
}

// ── Enum wrappers ─────────────────────────────────────────────────────────────

macro_rules! py_enum {
    ($py_name:ident, $inner:ty, $($variant:ident),+) => {
        #[pyclass]
        pub struct $py_name {
            pub inner: $inner,
        }

        #[pymethods]
        impl $py_name {
            fn __str__(&self) -> String {
                format!("{}", self.inner)
            }
            fn __repr__(&self) -> String {
                format!("{:?}", self.inner)
            }
            fn value(&self) -> String {
                serde_json::to_string(&self.inner)
                    .unwrap_or_default()
                    .trim_matches('"')
                    .to_owned()
            }
            #[staticmethod]
            fn from_str(s: &str) -> PyResult<Self> {
                s.parse::<$inner>()
                    .map(|inner| Self { inner })
                    .map_err(|_| PyValueError::new_err(format!("Invalid value: {:?}", s)))
            }
        }
    };
}

py_enum!(PyArchetype, gbr_types::enums::Archetype, Hero, Mentor, Trickster, Lover, Caregiver, Sage, Innocent, Rebel, Ruler, Creator, Explorer, Magician, Jester, Outlaw);
py_enum!(PyWound, gbr_types::enums::Wound, Abandonment, Betrayal, GuiltAndFailure, TraumaAndAbuse, Shame, Grief, TrustViolation, Powerlessness, IdentityRejection, Injustice, Neglect, SurvivorGuilt, Displacement);
py_enum!(PyAlignment, gbr_types::enums::Alignment, LawfulGood, NeutralGood, ChaoticGood, LawfulNeutral, TrueNeutral, ChaoticNeutral, LawfulEvil, NeutralEvil, ChaoticEvil);
py_enum!(PyRole, gbr_types::enums::Role, Protagonist, Deuteragonist, Antagonist, LoveInterest, Mentor, Confidant, Foil, Trickster, Guardian, Herald, Shapeshifter, Contagonist, WalkOn);
py_enum!(PyDriveModel, gbr_types::enums::DriveModel, Wound, Desire, Duty, Perception, Existential);
py_enum!(PyGateStatus, gbr_types::enums::GateStatus, Green, Yellow, Red, Locked, Unknown);
py_enum!(PySubPhaseStatus, gbr_types::enums::SubPhaseStatus, Locked, Ready, InProgress, Complete);
py_enum!(PyPhaseStatus, gbr_types::enums::PhaseStatus, Green, Yellow, Red, Unknown);
py_enum!(PyRevisionFlag, gbr_types::enums::RevisionFlag, TellingNotShowing, VoiceContractFail, PivotUnclear, SubtextMissing, PacingDrag, ContinuityBreak);

// ── EntityRef ─────────────────────────────────────────────────────────────────

#[pyclass]
pub struct PyEntityRef {
    inner: gbr_types::tags::EntityRef,
}

#[pymethods]
impl PyEntityRef {
    #[new]
    fn new(slug: &str) -> Self {
        Self { inner: gbr_types::tags::EntityRef::new(slug) }
    }
    #[getter]
    fn slug(&self) -> &str { &self.inner.slug }
    fn __str__(&self) -> &str { &self.inner.slug }
    fn __repr__(&self) -> String { format!("EntityRef('{}')", self.inner.slug) }
}

// ── Annotation ────────────────────────────────────────────────────────────────

#[pyclass]
pub struct PyAnnotation {
    inner: gbr_types::tags::Annotation,
}

#[pymethods]
impl PyAnnotation {
    fn to_json(&self) -> PyResult<String> {
        serde_json::to_string(&self.inner)
            .map_err(|e| PyValueError::new_err(e.to_string()))
    }
    fn __repr__(&self) -> String {
        serde_json::to_string(&self.inner).unwrap_or_default()
    }
}

// ── Character ─────────────────────────────────────────────────────────────────

#[pyclass]
pub struct PyCharacter { inner: gbr_types::entities::Character }
json_methods!(PyCharacter, gbr_types::entities::Character);

#[pymethods]
impl PyCharacter {
    #[new]
    fn new(id: &str, name: &str) -> Self {
        Self {
            inner: gbr_types::entities::Character {
                id: id.to_owned(),
                name: name.to_owned(),
                slot: None,
                archetype: None,
                wound: None,
                alignment: None,
                role: None,
                drive_model: None,
                arc_type: None,
                actant: None,
                ghost: None,
                want: None,
                need: None,
                flaw: None,
                voice_signature: None,
            },
        }
    }
    #[getter] fn id(&self) -> &str { &self.inner.id }
    #[getter] fn name(&self) -> &str { &self.inner.name }
}

// ── Scene ─────────────────────────────────────────────────────────────────────

#[pyclass]
pub struct PyScene { inner: gbr_types::entities::Scene }
json_methods!(PyScene, gbr_types::entities::Scene);

#[pymethods]
impl PyScene {
    #[new]
    fn new(id: &str) -> Self {
        Self {
            inner: gbr_types::entities::Scene {
                id: id.to_owned(),
                working_title: None,
                story_position: None,
                pov_character: None,
                attending_characters: Vec::new(),
                setting: None,
                time_of_day: None,
                weather: None,
                goal: None,
                why_goal_matters: None,
                plan: None,
                opponent_or_obstacle: None,
                conflict_type: Vec::new(),
                escalation_beats: Vec::new(),
                dialogue_strategy: None,
                action_strategy: None,
                emotional_escalation: None,
                outcome_type: None,
                what_changed: None,
                new_information: None,
                plant_or_setup: None,
                sequel: None,
                dominant_sense: None,
                key_sensory_details: Vec::new(),
                emotional_weather: None,
                scene_unique_image: None,
                pacing_notes: None,
                target_word_count: None,
                complexity: None,
                priority: None,
                narrative_threads: Vec::new(),
                sequence_id: None,
                scene_type: None,
                tension_level: None,
            },
        }
    }
    #[getter] fn id(&self) -> &str { &self.inner.id }
}

// ── VoiceSignature ────────────────────────────────────────────────────────────

#[pyclass]
pub struct PyVoiceSignature { inner: gbr_types::voice::VoiceSignature }
json_methods!(PyVoiceSignature, gbr_types::voice::VoiceSignature);

#[pymethods]
impl PyVoiceSignature {
    #[new]
    fn new(character_id: &str) -> Self {
        Self {
            inner: gbr_types::voice::VoiceSignature {
                character_id: character_id.to_owned(),
                ..Default::default()
            },
        }
    }
}

// ── VoiceContract ─────────────────────────────────────────────────────────────

#[pyclass]
pub struct PyVoiceContract { inner: gbr_types::voice::VoiceContract }
json_methods!(PyVoiceContract, gbr_types::voice::VoiceContract);

#[pymethods]
impl PyVoiceContract {
    #[new]
    fn new() -> Self {
        Self { inner: gbr_types::voice::VoiceContract::default() }
    }
}

// ── TrainingExample ───────────────────────────────────────────────────────────

#[pyclass]
pub struct PyTrainingExample { inner: crate::training::TrainingExample }
json_methods!(PyTrainingExample, crate::training::TrainingExample);

#[pymethods]
impl PyTrainingExample {
    fn to_jsonl(&self) -> PyResult<String> {
        self.inner.to_jsonl().map_err(|e| PyValueError::new_err(e.to_string()))
    }
    #[getter] fn id(&self) -> &str { &self.inner.id }
}

// ── TrainingDataset ───────────────────────────────────────────────────────────

#[pyclass]
pub struct PyTrainingDataset { inner: crate::training::TrainingDataset }

#[pymethods]
impl PyTrainingDataset {
    #[getter] fn name(&self) -> &str { &self.inner.name }
    #[getter] fn example_count(&self) -> usize { self.inner.example_count }
    #[getter] fn total_word_count(&self) -> u32 { self.inner.total_word_count }

    fn to_jsonl_string(&self) -> PyResult<String> {
        let mut out = String::new();
        for line in self.inner.to_jsonl_lines() {
            out.push_str(&line.map_err(|e| PyValueError::new_err(e.to_string()))?);
            out.push('\n');
        }
        Ok(out)
    }
}

// ── Gate types ────────────────────────────────────────────────────────────────

#[pyclass]
pub struct PyGateSpec { inner: crate::gates::GateSpec }
json_methods!(PyGateSpec, crate::gates::GateSpec);

#[pyclass]
pub struct PyGateResult { inner: crate::gates::GateResult }
json_methods!(PyGateResult, crate::gates::GateResult);

#[pyclass]
pub struct PyPhaseSpec { inner: crate::gates::PhaseSpec }
json_methods!(PyPhaseSpec, crate::gates::PhaseSpec);

#[pyclass]
pub struct PyPhaseResult { inner: crate::gates::PhaseResult }
json_methods!(PyPhaseResult, crate::gates::PhaseResult);

// ── StoryRecipe ───────────────────────────────────────────────────────────────

#[pyclass]
pub struct PyStoryRecipe { inner: crate::recipe::StoryRecipe }
json_methods!(PyStoryRecipe, crate::recipe::StoryRecipe);

#[pymethods]
impl PyStoryRecipe {
    #[getter] fn seed(&self) -> u64 { self.inner.seed }
}

// ── Module-level functions ─────────────────────────────────────────────────────

/// Parse a `<!-- key:value -->` annotation comment string into a list of
/// `PyAnnotation` objects.  Returns `(annotations, warnings)`.
#[pyfunction]
fn parse_annotation(
    py: Python<'_>,
    raw: &str,
) -> PyResult<(Vec<PyAnnotation>, Vec<String>)> {
    let (anns, warns) = gbr_types::tags::parse_annotation_comment(raw);
    let py_anns = anns.into_iter().map(|a| PyAnnotation { inner: a }).collect();
    Ok((py_anns, warns))
}

/// Dump all JSON Schemas as a JSON string.
/// Use from Python: `json.loads(grimoire_types.dump_schemas())`
#[pyfunction]
fn dump_schemas() -> PyResult<String> {
    serde_json::to_string_pretty(&crate::generate_all_schemas())
        .map_err(|e| PyValueError::new_err(e.to_string()))
}
