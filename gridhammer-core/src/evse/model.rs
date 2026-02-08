use super::fsm::transition;
use super::state::{EvseEvent, EvseState};
use super::transaction::TransactionManager;
use crate::ocpp::messages::{
    ChargingStateEnum, TransactionEventEnum, TransactionEventRequest, TriggerReasonEnum,
};
use chrono::{DateTime, Utc};
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
        timestamp: DateTime<Utc>,
        transaction_id: Option<String>,
    ) -> Result<(EvseState, Option<TransactionEventRequest>), String> {
        // 1. Check FSM transition
        let new_state = transition(self.state, event).map_err(|e| e.to_string())?;

        // 2. Handle Transaction Lifecycle based on event/state change
        let mut tx_event_req = None;

        match event {
            EvseEvent::StartCharging => {
                // Transitioning to Charging. Start Transaction.
                // We require a transaction_id for this event.
                if let Some(tx_id) = transaction_id {
                    let req = self
                        .transaction_manager
                        .start_transaction(
                            tx_id,
                            timestamp,
                            self.id as i32,
                            TriggerReasonEnum::RemoteStart, // Defaulting for now
                        )
                        .map_err(|e| e.to_string())?;

                    tx_event_req = Some(req);
                } else {
                    return Err("StartCharging event requires a transaction_id".to_string());
                }
            }
            EvseEvent::StopCharging => {
                // Transitioning from Charging/Suspended to Available. Stop Transaction.
                let req = self
                    .transaction_manager
                    .stop_transaction(
                        timestamp,
                        self.id as i32,
                        TriggerReasonEnum::RemoteStop,
                        Some("RemoteStop".to_string()),
                    )
                    .map_err(|e| e.to_string())?;
                tx_event_req = Some(req);
            }
            EvseEvent::Fault => {
                // Fault ends the transaction per invariant:
                // Charging -> Faulted must emit TransactionEvent(Ended, reason=Error)
                if self.transaction_manager.current_transaction.is_some() {
                    let req = self
                        .transaction_manager
                        .stop_transaction(
                            timestamp,
                            self.id as i32,
                            TriggerReasonEnum::ChargingStateChanged,
                            Some("Fault".to_string()),
                        )
                        .map_err(|e| e.to_string())?;
                    tx_event_req = Some(req);
                }
            }
            EvseEvent::PauseEV | EvseEvent::PauseEVSE | EvseEvent::ResumeCharging => {
                // Transaction continues, but state changes.
                if self.transaction_manager.current_transaction.is_some() {
                    let charging_state = match new_state {
                        EvseState::Charging => ChargingStateEnum::Charging,
                        EvseState::SuspendedEV => ChargingStateEnum::SuspendedEV,
                        EvseState::SuspendedEVSE => ChargingStateEnum::SuspendedEVSE,
                        _ => ChargingStateEnum::Idle,
                    };

                    let reason = match event {
                        EvseEvent::PauseEV | EvseEvent::PauseEVSE | EvseEvent::ResumeCharging => {
                            TriggerReasonEnum::ChargingStateChanged
                        }
                        _ => TriggerReasonEnum::Trigger,
                    };

                    let req = self
                        .transaction_manager
                        .update_transaction(timestamp, self.id as i32, reason, charging_state)
                        .map_err(|e| e.to_string())?;
                    tx_event_req = Some(req);
                }
            }
            // For updates, we might need a separate trigger or assume periodic updates
            // But here we are handling discrete events.
            _ => {}
        }

        // 3. Update state
        self.state = new_state;
        Ok((new_state, tx_event_req))
    }

    pub fn tick_transaction(&mut self) -> Result<(), String> {
        // This could be used to trigger periodic updates or check limits
        // For now, no automatic updates on tick unless required.
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ocpp::messages::TransactionEventEnum;
    use chrono::TimeZone;

    #[test]
    fn test_evse_start_stop_transaction() {
        let mut evse = Evse::new(1);
        let ts = Utc.timestamp_opt(100, 0).unwrap();

        // PlugIn -> Preparing
        let (state, tx_req) = evse.handle_event(EvseEvent::PlugIn, ts, None).unwrap();
        assert_eq!(state, EvseState::Preparing);
        assert!(tx_req.is_none());
        assert!(evse.transaction_manager.current_transaction.is_none());

        // StartCharging -> Charging (Starts Tx)
        let ts_start = Utc.timestamp_opt(110, 0).unwrap();
        let (state, tx_req) = evse
            .handle_event(EvseEvent::StartCharging, ts_start, Some("tx1".to_string()))
            .unwrap();
        assert_eq!(state, EvseState::Charging);
        assert!(evse.transaction_manager.current_transaction.is_some());

        let req = tx_req.unwrap();
        assert_eq!(req.event_type, TransactionEventEnum::Started);
        assert_eq!(req.transaction_info.transaction_id, "tx1");

        // StopCharging -> Available (Stops Tx)
        let ts_stop = Utc.timestamp_opt(120, 0).unwrap();
        let (state, tx_req) = evse
            .handle_event(EvseEvent::StopCharging, ts_stop, None)
            .unwrap();
        assert_eq!(state, EvseState::Available);
        assert!(evse.transaction_manager.current_transaction.is_none());

        let req = tx_req.unwrap();
        assert_eq!(req.event_type, TransactionEventEnum::Ended);
        assert_eq!(req.transaction_info.transaction_id, "tx1");
        assert_eq!(req.transaction_info.time_spent_charging, Some(10)); // 120 - 110
    }

    #[test]
    fn test_missing_tx_id_error() {
        let mut evse = Evse::new(1);
        let ts = Utc.timestamp_opt(100, 0).unwrap();
        evse.handle_event(EvseEvent::PlugIn, ts, None).unwrap();

        // StartCharging without tx_id should fail
        assert!(evse
            .handle_event(EvseEvent::StartCharging, ts, None)
            .is_err());
    }

    #[test]
    fn test_transaction_update_lifecycle() {
        let mut evse = Evse::new(1);
        let ts_start = Utc.timestamp_opt(100, 0).unwrap();

        // PlugIn -> Preparing
        evse.handle_event(EvseEvent::PlugIn, ts_start, None)
            .unwrap();

        // Start Transaction
        evse.handle_event(EvseEvent::StartCharging, ts_start, Some("tx1".to_string()))
            .unwrap();

        // Update 1: PauseEV -> SuspendedEV
        let ts_update1 = Utc.timestamp_opt(110, 0).unwrap();
        let (state, tx_req) = evse
            .handle_event(EvseEvent::PauseEV, ts_update1, None)
            .unwrap();
        assert_eq!(state, EvseState::SuspendedEV);
        let req = tx_req.unwrap();
        assert_eq!(req.event_type, TransactionEventEnum::Updated);
        assert_eq!(
            req.transaction_info.charging_state,
            Some(ChargingStateEnum::SuspendedEV)
        );
        assert_eq!(req.seq_no, 1);

        // Update 2: ResumeCharging -> Charging
        let ts_update2 = Utc.timestamp_opt(120, 0).unwrap();
        let (state, tx_req) = evse
            .handle_event(EvseEvent::ResumeCharging, ts_update2, None)
            .unwrap();
        assert_eq!(state, EvseState::Charging);
        let req = tx_req.unwrap();
        assert_eq!(req.event_type, TransactionEventEnum::Updated);
        assert_eq!(
            req.transaction_info.charging_state,
            Some(ChargingStateEnum::Charging)
        );
        assert_eq!(req.seq_no, 2);

        // Update 3: Fault -> Faulted (Ends Tx)
        let ts_update3 = Utc.timestamp_opt(130, 0).unwrap();
        let (state, tx_req) = evse
            .handle_event(EvseEvent::Fault, ts_update3, None)
            .unwrap();
        assert_eq!(state, EvseState::Faulted);
        let req = tx_req.unwrap();
        assert_eq!(req.event_type, TransactionEventEnum::Ended);
        assert_eq!(
            req.transaction_info.charging_state,
            Some(ChargingStateEnum::Idle)
        );
        assert_eq!(
            req.transaction_info.stopped_reason,
            Some("Fault".to_string())
        );
        assert_eq!(req.seq_no, 3);

        // Transaction should be cleared
        assert!(evse.transaction_manager.current_transaction.is_none());
    }
}
