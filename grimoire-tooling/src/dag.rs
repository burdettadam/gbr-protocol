//! Sub-phase dependency DAG — typed port of `grimoire/dag.py`.
//!
//! Implements Kahn's algorithm for topological ordering and provides
//! lock/unlock/readiness queries over the sub-phase graph.

use std::collections::{HashMap, HashSet, VecDeque};

use thiserror::Error;

use gbr_types::enums::{GateStatus, SubPhaseStatus};
use crate::gates::{GateResult, PhaseSpec};

// ── Error ─────────────────────────────────────────────────────────────────────

#[derive(Debug, Error)]
pub enum DagError {
    #[error("cycle detected involving sub-phase '{0}'")]
    CycleDetected(String),
    #[error("unknown sub-phase FQID '{0}'")]
    UnknownFqid(String),
}

// ── Sub-phase DAG ─────────────────────────────────────────────────────────────

/// Directed acyclic graph of sub-phase dependencies.
///
/// Nodes are fully-qualified sub-phase IDs (`"phase_id/sub_phase_id"`).
/// Edges are `depends_on` relationships declared in `_gate.md` frontmatter.
#[allow(dead_code)] // nodes/successors used by build(); predecessors/topo_order drive public API
pub struct SubPhaseDAG {
    /// All registered FQID nodes
    nodes: HashSet<String>,
    /// Adjacency list: source → set of targets it must precede
    successors: HashMap<String, HashSet<String>>,
    /// Inverse: target → set of predecessors it requires
    predecessors: HashMap<String, HashSet<String>>,
    /// Topological order of all nodes (Kahn's algorithm)
    topo_order: Vec<String>,
}

impl SubPhaseDAG {
    /// Build a DAG from a collection of phase specs.
    pub fn build(phases: &HashMap<String, PhaseSpec>) -> Result<Self, DagError> {
        let mut nodes: HashSet<String> = HashSet::new();
        let mut successors: HashMap<String, HashSet<String>> = HashMap::new();
        let mut predecessors: HashMap<String, HashSet<String>> = HashMap::new();

        // Register all nodes
        for (phase_id, spec) in phases {
            for sp in &spec.sub_phases {
                let fqid = sp.fqid(phase_id);
                nodes.insert(fqid.clone());
                successors.entry(fqid.clone()).or_default();
                predecessors.entry(fqid).or_default();
            }
            // Phases with no sub-phases get a synthetic node
            if spec.sub_phases.is_empty() {
                nodes.insert(phase_id.clone());
                successors.entry(phase_id.clone()).or_default();
                predecessors.entry(phase_id.clone()).or_default();
            }
        }

        // Register edges
        for (phase_id, spec) in phases {
            for sp in &spec.sub_phases {
                let target_fqid = sp.fqid(phase_id);
                for dep in &sp.depends_on {
                    // `dep` may be a local ID (within the same phase) or a FQID
                    let source_fqid = if dep.contains('/') {
                        dep.clone()
                    } else {
                        format!("{}/{}", phase_id, dep)
                    };
                    successors
                        .entry(source_fqid.clone())
                        .or_default()
                        .insert(target_fqid.clone());
                    predecessors
                        .entry(target_fqid.clone())
                        .or_default()
                        .insert(source_fqid);
                }
            }
        }

        let topo_order = Self::kahn_topo(&nodes, &successors, &predecessors)?;

        Ok(Self { nodes, successors, predecessors, topo_order })
    }

    /// Whether a sub-phase is unlocked given the set of completed FQIDs.
    pub fn is_unlocked(&self, fqid: &str, completed: &HashSet<String>) -> bool {
        self.predecessors
            .get(fqid)
            .map(|preds| preds.iter().all(|p| completed.contains(p)))
            .unwrap_or(true) // unknown nodes are considered unlocked
    }

    /// Sub-phases that are unlocked but not yet in `completed`, in
    /// topological order.
    pub fn next_actionable(&self, completed: &HashSet<String>) -> Vec<String> {
        self.topo_order
            .iter()
            .filter(|fqid| !completed.contains(*fqid) && self.is_unlocked(fqid, completed))
            .cloned()
            .collect()
    }

    /// The ordered chain of blocking nodes preventing `fqid` from starting.
    pub fn blocking_path(&self, fqid: &str, completed: &HashSet<String>) -> Vec<String> {
        let mut path = Vec::new();
        let mut to_visit: Vec<String> = self
            .predecessors
            .get(fqid)
            .map(|p| p.iter().cloned().collect())
            .unwrap_or_default();
        let mut visited = HashSet::new();
        while let Some(node) = to_visit.pop() {
            if visited.contains(&node) { continue; }
            visited.insert(node.clone());
            if !completed.contains(&node) {
                path.push(node.clone());
                if let Some(preds) = self.predecessors.get(&node) {
                    to_visit.extend(preds.iter().cloned());
                }
            }
        }
        path
    }

    /// Full topological ordering of all nodes.
    pub fn topological_order(&self) -> &[String] {
        &self.topo_order
    }

    /// Compute the sub-phase status for a given FQID.
    pub fn compute_status(
        &self,
        fqid: &str,
        gate_results: &HashMap<String, GateResult>,
        completed: &HashSet<String>,
    ) -> SubPhaseStatus {
        if !self.is_unlocked(fqid, completed) {
            return SubPhaseStatus::Locked;
        }
        // Collect gate results for this sub-phase
        let phase_gates: Vec<&GateResult> = gate_results
            .values()
            .filter(|g| g.sub_phase.as_deref() == Some(fqid))
            .collect();
        if phase_gates.is_empty() {
            return SubPhaseStatus::Ready;
        }
        let all_green = phase_gates
            .iter()
            .all(|g| g.status == GateStatus::Green);
        let any_green = phase_gates
            .iter()
            .any(|g| g.status == GateStatus::Green || g.status == GateStatus::Yellow);
        if all_green {
            SubPhaseStatus::Complete
        } else if any_green {
            SubPhaseStatus::InProgress
        } else {
            SubPhaseStatus::Ready
        }
    }

    // ── Kahn's algorithm ──────────────────────────────────────────────────────

    fn kahn_topo(
        nodes: &HashSet<String>,
        successors: &HashMap<String, HashSet<String>>,
        predecessors: &HashMap<String, HashSet<String>>,
    ) -> Result<Vec<String>, DagError> {
        let mut in_degree: HashMap<String, usize> = nodes
            .iter()
            .map(|n| {
                let deg = predecessors.get(n).map(|p| p.len()).unwrap_or(0);
                (n.clone(), deg)
            })
            .collect();

        // Start with zero-in-degree nodes; sort for determinism
        let mut queue: VecDeque<String> = {
            let mut roots: Vec<String> = in_degree
                .iter()
                .filter(|(_, &d)| d == 0)
                .map(|(n, _)| n.clone())
                .collect();
            roots.sort();
            roots.into()
        };

        let mut order = Vec::new();
        while let Some(node) = queue.pop_front() {
            order.push(node.clone());
            if let Some(succ) = successors.get(&node) {
                let mut succ_sorted: Vec<String> = succ.iter().cloned().collect();
                succ_sorted.sort();
                for s in succ_sorted {
                    if let Some(d) = in_degree.get_mut(&s) {
                        *d -= 1;
                        if *d == 0 {
                            queue.push_back(s);
                        }
                    }
                }
            }
        }

        if order.len() != nodes.len() {
            // Cycle — find a node that never made it out
            let remaining: Vec<String> = nodes
                .iter()
                .filter(|n| !order.contains(n))
                .cloned()
                .collect();
            return Err(DagError::CycleDetected(remaining[0].clone()));
        }

        Ok(order)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_phase(id: &str, sub_phases: Vec<(&str, u32, Vec<&str>)>) -> PhaseSpec {
        crate::gates::PhaseSpec {
            phase_id: id.to_owned(),
            phase_label: id.to_owned(),
            sub_phases: sub_phases
                .into_iter()
                .map(|(sid, order, deps)| crate::gates::SubPhaseSpec {
                    id: sid.to_owned(),
                    label: sid.to_owned(),
                    order,
                    depends_on: deps.into_iter().map(String::from).collect(),
                })
                .collect(),
            gates: Vec::new(),
        }
    }

    #[test]
    fn linear_chain_unlocks_in_order() {
        let mut phases = HashMap::new();
        phases.insert(
            "03-characters".to_owned(),
            make_phase(
                "03-characters",
                vec![
                    ("core-identity", 0, vec![]),
                    ("relationships", 1, vec!["core-identity"]),
                    ("cross-refs", 2, vec!["relationships"]),
                ],
            ),
        );
        let dag = SubPhaseDAG::build(&phases).unwrap();
        let completed = HashSet::new();
        let next = dag.next_actionable(&completed);
        assert_eq!(next, vec!["03-characters/core-identity"]);
    }

    #[test]
    fn cycle_detected() {
        let mut phases = HashMap::new();
        phases.insert(
            "test".to_owned(),
            make_phase(
                "test",
                vec![
                    ("a", 0, vec!["b"]),
                    ("b", 1, vec!["a"]),
                ],
            ),
        );
        assert!(SubPhaseDAG::build(&phases).is_err());
    }
}
