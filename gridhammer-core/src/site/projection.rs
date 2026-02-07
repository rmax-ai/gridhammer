use super::topology::SiteTopology;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ViolationType {
    FeederCapacityExceeded {
        feeder_id: String,
        limit: f64,
        actual: f64,
    },
    TransformerCapacityExceeded {
        limit: f64,
        actual: f64,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Violation {
    pub violation_type: ViolationType,
    pub affected_evses: Vec<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProjectionEffect {
    // A hardware fault (breaker trip)
    Faulted { evse_id: i32, reason: String },
    // A local controller limiting power
    UpdateChargingProfile { evse_id: i32, limit_watts: f64 },
    // EVSE goes to Suspended state
    SuspendEVSE { evse_id: i32 },
}

pub fn check_constraints(
    topology: &SiteTopology,
    evse_loads: &HashMap<i32, f64>,
) -> Vec<Violation> {
    let mut violations = Vec::new();
    let mut site_total_load = 0.0;
    let mut feeder_loads: HashMap<String, f64> = HashMap::new();
    let mut feeder_evses: HashMap<String, Vec<i32>> = HashMap::new();

    // Initialize feeder loads
    for feeder_id in topology.feeders.keys() {
        feeder_loads.insert(feeder_id.clone(), 0.0);
        feeder_evses.insert(feeder_id.clone(), Vec::new());
    }

    // Sort EVSEs for deterministic iteration
    let mut all_evses: Vec<i32> = evse_loads.keys().cloned().collect();
    all_evses.sort();

    // Sum up loads
    for &evse_id in &all_evses {
        if let Some(&load) = evse_loads.get(&evse_id) {
            if load < 0.0 {
                continue;
            } // Ignore negative load for capacity check

            site_total_load += load;

            if let Some(feeder) = topology.get_feeder_for_evse(evse_id) {
                let load_entry = feeder_loads.entry(feeder.id.clone()).or_insert(0.0);
                *load_entry += load;

                let evse_entry = feeder_evses.entry(feeder.id.clone()).or_default();
                evse_entry.push(evse_id);
            }
        }
    }

    // Sort feeders for deterministic iteration
    let mut sorted_feeder_ids: Vec<String> = feeder_loads.keys().cloned().collect();
    sorted_feeder_ids.sort();

    // Check Feeder capacities
    for feeder_id in sorted_feeder_ids {
        if let Some(&total_load) = feeder_loads.get(&feeder_id) {
            if let Some(feeder) = topology.feeders.get(&feeder_id) {
                if total_load > feeder.capacity {
                    violations.push(Violation {
                        violation_type: ViolationType::FeederCapacityExceeded {
                            feeder_id: feeder_id.clone(),
                            limit: feeder.capacity,
                            actual: total_load,
                        },
                        affected_evses: feeder_evses.get(&feeder_id).cloned().unwrap_or_default(),
                    });
                }
            }
        }
    }

    // Check Site Transformer capacity
    if site_total_load > topology.transformer_capacity {
        violations.push(Violation {
            violation_type: ViolationType::TransformerCapacityExceeded {
                limit: topology.transformer_capacity,
                actual: site_total_load,
            },
            affected_evses: all_evses,
        });
    }

    violations
}

/// Maps violations to concrete effects on EVSEs.
/// Currently implements a "hard breaker trip" policy:
/// - If a limit is exceeded, ALL affected EVSEs are faulted.
pub fn project_effects(violations: &[Violation]) -> Vec<ProjectionEffect> {
    let mut effects = Vec::new();

    for violation in violations {
        match &violation.violation_type {
            ViolationType::FeederCapacityExceeded {
                feeder_id,
                limit,
                actual,
            } => {
                let reason = format!("Feeder {} overloaded: {} > {}", feeder_id, actual, limit);
                for &evse_id in &violation.affected_evses {
                    effects.push(ProjectionEffect::Faulted {
                        evse_id,
                        reason: reason.clone(),
                    });
                }
            }
            ViolationType::TransformerCapacityExceeded { limit, actual } => {
                let reason = format!("Transformer overloaded: {} > {}", actual, limit);
                for &evse_id in &violation.affected_evses {
                    effects.push(ProjectionEffect::Faulted {
                        evse_id,
                        reason: reason.clone(),
                    });
                }
            }
        }
    }

    // Deduplicate effects?
    // If an EVSE is faulted by Feeder AND Transformer, we probably only need one Fault.
    // For now, let's keep it simple.

    effects
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::site::topology::Feeder;

    fn create_test_topology() -> SiteTopology {
        let mut topology = SiteTopology::new(10000.0);
        let feeder1 = Feeder {
            id: "feeder1".to_string(),
            capacity: 5000.0,
            evse_ids: vec![1, 2],
        };
        let feeder2 = Feeder {
            id: "feeder2".to_string(),
            capacity: 5000.0,
            evse_ids: vec![3, 4],
        };
        topology.feeders.insert("feeder1".to_string(), feeder1);
        topology.feeders.insert("feeder2".to_string(), feeder2);
        topology.validate().expect("Topology should be valid");
        topology
    }

    #[test]
    fn test_no_violations() {
        let topology = create_test_topology();
        let mut loads = HashMap::new();
        loads.insert(1, 2000.0);
        loads.insert(2, 2000.0); // Feeder1: 4000 < 5000
        loads.insert(3, 2000.0);
        loads.insert(4, 2000.0); // Feeder2: 4000 < 5000
                                 // Total: 8000 < 10000

        let violations = check_constraints(&topology, &loads);
        assert!(violations.is_empty());
    }

    #[test]
    fn test_feeder_violation() {
        let topology = create_test_topology();
        let mut loads = HashMap::new();
        loads.insert(1, 3000.0);
        loads.insert(2, 3000.0); // Feeder1: 6000 > 5000!
        loads.insert(3, 1000.0);

        let violations = check_constraints(&topology, &loads);
        assert_eq!(violations.len(), 1);

        match &violations[0].violation_type {
            ViolationType::FeederCapacityExceeded {
                feeder_id,
                limit,
                actual,
            } => {
                assert_eq!(feeder_id, "feeder1");
                assert_eq!(*limit, 5000.0);
                assert_eq!(*actual, 6000.0);
            }
            _ => panic!("Wrong violation type"),
        }

        let effects = project_effects(&violations);
        // Should fault EVSE 1 and 2
        assert_eq!(effects.len(), 2);
        // We don't enforce order of effects here unless we sort them.
        let faulted_ids: Vec<i32> = effects
            .iter()
            .map(|e| match e {
                ProjectionEffect::Faulted { evse_id, .. } => *evse_id,
                _ => 0,
            })
            .collect();
        assert!(faulted_ids.contains(&1));
        assert!(faulted_ids.contains(&2));
    }

    #[test]
    fn test_transformer_violation() {
        let topology = create_test_topology();
        let mut loads = HashMap::new();
        loads.insert(1, 4000.0);
        loads.insert(2, 0.0);
        loads.insert(3, 4000.0);
        loads.insert(4, 3000.0);
        // Feeder1: 4000 < 5000
        // Feeder2: 7000 > 5000 (Also a feeder violation!)
        // Total: 11000 > 10000 (Transformer violation!)

        let violations = check_constraints(&topology, &loads);
        assert!(violations.len() >= 2); // At least Feeder2 and Transformer

        let effects = project_effects(&violations);
        assert!(!effects.is_empty());
    }

    #[test]
    fn test_determinism_ordering() {
        // Run checks multiple times and ensure output order is identical
        let topology = create_test_topology();
        let mut loads = HashMap::new();
        // Insert in random order (HashMap does this anyway)
        loads.insert(1, 4000.0);
        loads.insert(2, 4000.0);
        loads.insert(3, 4000.0);
        loads.insert(4, 4000.0);
        // Feeder1: 8000 > 5000
        // Feeder2: 8000 > 5000
        // Total: 16000 > 10000

        let run1 = check_constraints(&topology, &loads);
        let effects1 = project_effects(&run1);

        let run2 = check_constraints(&topology, &loads);
        let effects2 = project_effects(&run2);

        assert_eq!(run1.len(), run2.len());
        assert_eq!(effects1.len(), effects2.len());

        // Check exact equality of Effects (which implements PartialEq)
        for (i, e1) in effects1.iter().enumerate() {
            let e2 = &effects2[i];
            assert_eq!(e1, e2, "Mismatch at index {}", i);
        }

        // Logic:
        // 1. Iterate sorted feeders. feeder1, feeder2.
        //    Violations pushed: Feeder1, then Feeder2.
        // 2. Check Transformer.
        //    Violations pushed: Transformer.
        //
        // So expected order of violations: Feeder1, Feeder2, Transformer.

        assert_eq!(run1.len(), 3);

        match &run1[0].violation_type {
            ViolationType::FeederCapacityExceeded { feeder_id, .. } => {
                assert_eq!(feeder_id, "feeder1")
            }
            _ => panic!("Expected feeder1 first"),
        }
        match &run1[1].violation_type {
            ViolationType::FeederCapacityExceeded { feeder_id, .. } => {
                assert_eq!(feeder_id, "feeder2")
            }
            _ => panic!("Expected feeder2 second"),
        }
        match &run1[2].violation_type {
            ViolationType::TransformerCapacityExceeded { .. } => {}
            _ => panic!("Expected transformer last"),
        }
    }
}
