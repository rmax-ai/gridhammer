use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Feeder {
    pub id: String,
    pub capacity: f64,
    pub evse_ids: Vec<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiteTopology {
    pub transformer_capacity: f64,
    pub feeders: HashMap<String, Feeder>,
    #[serde(skip)]
    evse_to_feeder: HashMap<i32, String>,
}

impl SiteTopology {
    pub fn new(transformer_capacity: f64) -> Self {
        Self {
            transformer_capacity,
            feeders: HashMap::new(),
            evse_to_feeder: HashMap::new(),
        }
    }

    pub fn validate(&mut self) -> Result<(), String> {
        if self.transformer_capacity < 0.0 {
            return Err("Transformer capacity must be non-negative".to_string());
        }

        self.evse_to_feeder.clear();
        let mut all_evse = HashSet::new();

        for feeder in self.feeders.values() {
            if feeder.capacity < 0.0 {
                return Err(format!(
                    "Feeder {} capacity must be non-negative",
                    feeder.id
                ));
            }
            for &evse_id in &feeder.evse_ids {
                if !all_evse.insert(evse_id) {
                    return Err(format!(
                        "EVSE ID {} is assigned to multiple feeders",
                        evse_id
                    ));
                }
                self.evse_to_feeder.insert(evse_id, feeder.id.clone());
            }
        }

        Ok(())
    }

    pub fn get_feeder_for_evse(&self, evse_id: i32) -> Option<&Feeder> {
        self.evse_to_feeder
            .get(&evse_id)
            .and_then(|fid| self.feeders.get(fid))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_topology() {
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

        assert!(topology.validate().is_ok());
        assert_eq!(topology.get_feeder_for_evse(1).unwrap().id, "feeder1");
        assert_eq!(topology.get_feeder_for_evse(3).unwrap().id, "feeder2");
    }

    #[test]
    fn test_duplicate_evse_id() {
        let mut topology = SiteTopology::new(10000.0);
        let feeder1 = Feeder {
            id: "feeder1".to_string(),
            capacity: 5000.0,
            evse_ids: vec![1, 2],
        };
        let feeder2 = Feeder {
            id: "feeder2".to_string(),
            capacity: 5000.0,
            evse_ids: vec![2, 3], // duplicate 2
        };
        topology.feeders.insert("feeder1".to_string(), feeder1);
        topology.feeders.insert("feeder2".to_string(), feeder2);

        assert!(topology.validate().is_err());
    }

    #[test]
    fn test_negative_capacity() {
        let mut topology = SiteTopology::new(-1000.0);
        let feeder = Feeder {
            id: "feeder1".to_string(),
            capacity: 5000.0,
            evse_ids: vec![1],
        };
        topology.feeders.insert("feeder1".to_string(), feeder);

        assert!(topology.validate().is_err());
    }

    #[test]
    fn test_negative_feeder_capacity() {
        let mut topology = SiteTopology::new(10000.0);
        let feeder = Feeder {
            id: "feeder1".to_string(),
            capacity: -5000.0,
            evse_ids: vec![1],
        };
        topology.feeders.insert("feeder1".to_string(), feeder);

        assert!(topology.validate().is_err());
    }
}
