//! `grimoire-types` — Backward-compatibility facade.
//!
//! This crate re-exports `gbr-types` (protocol-core) and `grimoire-tooling`
//! (authoring tools) under a single namespace, preserving the existing
//! `grimoire_types::` import paths used by downstream code.
//!
//! New code should depend directly on `gbr-types` or `grimoire-tooling`.

pub use gbr_types::catalogs;
pub use gbr_types::constraints;
pub use gbr_types::entities;
pub use gbr_types::enums;
pub use gbr_types::ontology;
pub use gbr_types::tags;
pub use gbr_types::voice;

pub use grimoire_tooling::dag;
pub use grimoire_tooling::gates;
pub use grimoire_tooling::recipe;
pub use grimoire_tooling::training;

pub use grimoire_tooling::generate_all_schemas;
