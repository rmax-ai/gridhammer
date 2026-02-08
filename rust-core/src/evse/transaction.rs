use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TransactionState {
    Started,
    Updated,
    Ended,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: String,
    pub state: TransactionState,
    pub start_time: u64,
    pub end_time: Option<u64>,
    pub last_update_time: Option<u64>,
    pub evse_id: u32,
}

#[derive(Debug, Clone)]
pub enum TransactionEvent {
    Started { transaction: Transaction },
    Updated { transaction: Transaction },
    Ended { transaction: Transaction },
}

pub struct TransactionManager {
    active_transactions: HashMap<u32, Transaction>,
}

impl TransactionManager {
    pub fn new() -> Self {
        TransactionManager {
            active_transactions: HashMap::new(),
        }
    }

    pub fn start_transaction(
        &mut self,
        evse_id: u32,
        transaction_id: String,
        start_time: u64,
    ) -> Result<TransactionEvent, String> {
        if self.active_transactions.contains_key(&evse_id) {
            return Err("Cannot start transaction: one already active on EVSE".to_string());
        }
        let transaction = Transaction {
            id: transaction_id,
            state: TransactionState::Started,
            start_time,
            end_time: None,
            last_update_time: None,
            evse_id,
        };
        self.active_transactions
            .insert(evse_id, transaction.clone());
        Ok(TransactionEvent::Started { transaction })
    }

    pub fn update_transaction(
        &mut self,
        evse_id: u32,
        current_time: u64,
    ) -> Result<TransactionEvent, String> {
        if let Some(tx) = self.active_transactions.get_mut(&evse_id) {
            tx.state = TransactionState::Updated;
            tx.last_update_time = Some(current_time);
            Ok(TransactionEvent::Updated {
                transaction: tx.clone(),
            })
        } else {
            Err("Cannot update: no active transaction on EVSE".to_string())
        }
    }

    pub fn end_transaction(
        &mut self,
        evse_id: u32,
        end_time: u64,
    ) -> Result<TransactionEvent, String> {
        if let Some(mut tx) = self.active_transactions.remove(&evse_id) {
            tx.state = TransactionState::Ended;
            tx.end_time = Some(end_time);
            Ok(TransactionEvent::Ended { transaction: tx })
        } else {
            Err("Cannot end: no active transaction on EVSE".to_string())
        }
    }

    /// Check invariants: ensure no active transactions remain (call at end of simulation or per EVSE)
    pub fn check_no_orphans(&self) -> bool {
        self.active_transactions.is_empty()
    }

    /// Produce deterministic JSON lines for evse_states.jsonl (active transactions per EVSE)
    pub fn produce_evse_states_jsonl(&self) -> Vec<String> {
        let mut lines = Vec::new();
        for (&evse_id, tx) in &self.active_transactions {
            let entry = serde_json::json!({
                "evse_id": evse_id,
                "transaction": tx
            });
            lines.push(serde_json::to_string(&entry).unwrap());
        }
        lines.sort(); // deterministic order
        lines
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_start_transaction_success() {
        let mut manager = TransactionManager::new();
        let event = manager
            .start_transaction(1, "tx1".to_string(), 100)
            .unwrap();
        match event {
            TransactionEvent::Started { transaction } => {
                assert_eq!(transaction.id, "tx1");
                assert_eq!(transaction.state, TransactionState::Started);
                assert_eq!(transaction.evse_id, 1);
            }
            _ => panic!("Expected Started event"),
        }
        assert!(manager.active_transactions.contains_key(&1));
    }

    #[test]
    fn test_no_double_start() {
        let mut manager = TransactionManager::new();
        manager
            .start_transaction(1, "tx1".to_string(), 100)
            .unwrap();
        let result = manager.start_transaction(1, "tx2".to_string(), 200);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Cannot start transaction: one already active on EVSE"
        );
    }

    #[test]
    fn test_update_transaction_success() {
        let mut manager = TransactionManager::new();
        manager
            .start_transaction(1, "tx1".to_string(), 100)
            .unwrap();
        let event = manager.update_transaction(1, 150).unwrap();
        match event {
            TransactionEvent::Updated { transaction } => {
                assert_eq!(transaction.state, TransactionState::Updated);
            }
            _ => panic!("Expected Updated event"),
        }
    }

    #[test]
    fn test_update_no_active_transaction() {
        let mut manager = TransactionManager::new();
        let result = manager.update_transaction(1, 150);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Cannot update: no active transaction on EVSE"
        );
    }

    #[test]
    fn test_end_transaction_success() {
        let mut manager = TransactionManager::new();
        manager
            .start_transaction(1, "tx1".to_string(), 100)
            .unwrap();
        let event = manager.end_transaction(1, 200).unwrap();
        match event {
            TransactionEvent::Ended { transaction } => {
                assert_eq!(transaction.state, TransactionState::Ended);
                assert_eq!(transaction.end_time, Some(200));
            }
            _ => panic!("Expected Ended event"),
        }
        assert!(!manager.active_transactions.contains_key(&1));
    }

    #[test]
    fn test_no_double_end() {
        let mut manager = TransactionManager::new();
        manager
            .start_transaction(1, "tx1".to_string(), 100)
            .unwrap();
        manager.end_transaction(1, 200).unwrap();
        let result = manager.end_transaction(1, 300);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            "Cannot end: no active transaction on EVSE"
        );
    }

    #[test]
    fn test_check_no_orphans() {
        let mut manager = TransactionManager::new();
        assert!(manager.check_no_orphans());
        manager
            .start_transaction(1, "tx1".to_string(), 100)
            .unwrap();
        assert!(!manager.check_no_orphans());
        manager.end_transaction(1, 200).unwrap();
        assert!(manager.check_no_orphans());
    }

    #[test]
    fn test_produce_evse_states_jsonl() {
        let mut manager = TransactionManager::new();
        manager
            .start_transaction(2, "tx2".to_string(), 100)
            .unwrap();
        manager
            .start_transaction(1, "tx1".to_string(), 100)
            .unwrap();
        let lines = manager.produce_evse_states_jsonl();
        assert_eq!(lines.len(), 2);
        // Check deterministic order (sorted by evse_id)
        assert!(lines[0].contains(r#""evse_id":1"#));
        assert!(lines[1].contains(r#""evse_id":2"#));
    }
}
