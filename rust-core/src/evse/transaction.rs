use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveTransaction {
    pub transaction_id: String,
    pub seq_no: i32,
    pub start_timestamp: i64, // Sim tick
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TransactionManager {
    pub current_transaction: Option<ActiveTransaction>,
}

impl TransactionManager {
    pub fn new() -> Self {
        Self {
            current_transaction: None,
        }
    }

    pub fn start_transaction(
        &mut self,
        transaction_id: String,
        timestamp: i64,
    ) -> Result<(), &'static str> {
        if self.current_transaction.is_some() {
            return Err("Transaction already active");
        }
        self.current_transaction = Some(ActiveTransaction {
            transaction_id,
            seq_no: 0,
            start_timestamp: timestamp,
        });
        Ok(())
    }

    pub fn update_transaction(&mut self) -> Result<(), &'static str> {
        if let Some(tx) = &mut self.current_transaction {
            tx.seq_no += 1;
            Ok(())
        } else {
            Err("No active transaction")
        }
    }

    pub fn stop_transaction(&mut self) -> Result<(), &'static str> {
        if self.current_transaction.is_none() {
            return Err("No active transaction to stop");
        }
        self.current_transaction = None;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transaction_lifecycle() {
        let mut tm = TransactionManager::new();

        // Start
        assert!(tm.start_transaction("tx1".to_string(), 100).is_ok());
        assert!(tm.current_transaction.is_some());
        assert_eq!(
            tm.current_transaction.as_ref().unwrap().transaction_id,
            "tx1"
        );
        assert_eq!(tm.current_transaction.as_ref().unwrap().seq_no, 0);

        // Update
        assert!(tm.update_transaction().is_ok());
        assert_eq!(tm.current_transaction.as_ref().unwrap().seq_no, 1);

        // Stop
        assert!(tm.stop_transaction().is_ok());
        assert!(tm.current_transaction.is_none());
    }

    #[test]
    fn test_invariants() {
        let mut tm = TransactionManager::new();

        // Cannot update or stop if no tx
        assert!(tm.update_transaction().is_err());
        assert!(tm.stop_transaction().is_err());

        // Cannot start if already active
        tm.start_transaction("tx1".to_string(), 100).unwrap();
        assert!(tm.start_transaction("tx2".to_string(), 200).is_err());
    }
}
