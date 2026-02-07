use super::fsm::transition;
use super::state::{EvseEvent, EvseState};
use super::transaction::TransactionManager;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Evse {
    pub id: u32,
    pub state: EvseState,
    pub transaction_manager: TransactionManager,
}

impl Evse {
    pub fn new(id: u32) -> Self {
        Self {
            id,
            state: EvseState::Available,
            transaction_manager: TransactionManager::new(),
        }
    }

    pub fn handle_event(
        &mut self,
        event: EvseEvent,
        timestamp: i64,
        transaction_id: Option<String>,
    ) -> Result<EvseState, String> {
        // 1. Check FSM transition
        let new_state = transition(self.state, event).map_err(|e| e.to_string())?;

        // 2. Handle Transaction Lifecycle based on event/state change
        match event {
            EvseEvent::StartCharging => {
                // Transitioning to Charging. Start Transaction.
                // We require a transaction_id for this event.
                if let Some(tx_id) = transaction_id {
                    self.transaction_manager
                        .start_transaction(tx_id, timestamp)
                        .map_err(|e| e.to_string())?;
                } else {
                    return Err("StartCharging event requires a transaction_id".to_string());
                }
            }
            EvseEvent::StopCharging => {
                // Transitioning from Charging/Suspended to Available. Stop Transaction.
                self.transaction_manager
                    .stop_transaction()
                    .map_err(|e| e.to_string())?;
            }
            // For updates, we might need a separate trigger or assume periodic updates
            // But here we are handling discrete events.
            _ => {}
        }

        // 3. Update state
        self.state = new_state;
        Ok(new_state)
    }

    pub fn tick_transaction(&mut self) -> Result<(), String> {
        // This could be used to trigger periodic updates or check limits
        // For now, maybe just update the sequence number?
        // But TransactionManager::update_transaction() increments seq_no.
        // We might want to call this on MeterValues trigger or periodic timer.
        if self.transaction_manager.current_transaction.is_some() {
            self.transaction_manager
                .update_transaction()
                .map_err(|e| e.to_string())?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evse_start_stop_transaction() {
        let mut evse = Evse::new(1);

        // PlugIn -> Preparing
        assert_eq!(
            evse.handle_event(EvseEvent::PlugIn, 100, None).unwrap(),
            EvseState::Preparing
        );
        assert!(evse.transaction_manager.current_transaction.is_none());

        // StartCharging -> Charging (Starts Tx)
        assert_eq!(
            evse.handle_event(EvseEvent::StartCharging, 110, Some("tx1".to_string()))
                .unwrap(),
            EvseState::Charging
        );
        assert!(evse.transaction_manager.current_transaction.is_some());
        assert_eq!(
            evse.transaction_manager
                .current_transaction
                .as_ref()
                .unwrap()
                .transaction_id,
            "tx1"
        );

        // StopCharging -> Available (Stops Tx)
        assert_eq!(
            evse.handle_event(EvseEvent::StopCharging, 120, None)
                .unwrap(),
            EvseState::Available
        );
        assert!(evse.transaction_manager.current_transaction.is_none());
    }

    #[test]
    fn test_missing_tx_id_error() {
        let mut evse = Evse::new(1);
        evse.handle_event(EvseEvent::PlugIn, 100, None).unwrap();

        // StartCharging without tx_id should fail
        assert!(evse
            .handle_event(EvseEvent::StartCharging, 110, None)
            .is_err());
    }
}
