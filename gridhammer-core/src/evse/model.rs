use super::fsm::transition;
use super::state::{EvseEvent, EvseState};
use super::transaction::TransactionManager;
use crate::ocpp::messages::{
    ChargingStateEnum, Transaction, TransactionEventEnum, TransactionEventRequest,
    TriggerReasonEnum, EVSE,
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
                if let Some(tx_id) = transaction_id.clone() {
                    self.transaction_manager
                        .start_transaction(tx_id.clone(), timestamp.timestamp())
                        .map_err(|e| e.to_string())?;

                    // Emit TransactionEvent(Started)
                    tx_event_req = Some(TransactionEventRequest {
                        event_type: TransactionEventEnum::Started,
                        timestamp,
                        trigger_reason: TriggerReasonEnum::RemoteStart, // Defaulting for now
                        seq_no: 0,
                        transaction_info: Transaction {
                            transaction_id: tx_id,
                            charging_state: Some(ChargingStateEnum::Charging),
                            time_spent_charging: Some(0),
                            stopped_reason: None,
                            remote_start_id: None,
                        },
                        evse: Some(EVSE {
                            id: self.id as i32,
                            connector_id: Some(1),
                        }),
                    });
                } else {
                    return Err("StartCharging event requires a transaction_id".to_string());
                }
            }
            EvseEvent::StopCharging => {
                // Capture current tx info before stopping
                if let Some(current_tx) = &self.transaction_manager.current_transaction {
                    let tx_id = current_tx.transaction_id.clone();
                    let seq_no = current_tx.seq_no + 1; // Increment for the final event

                    // Emit TransactionEvent(Ended)
                    tx_event_req = Some(TransactionEventRequest {
                        event_type: TransactionEventEnum::Ended,
                        timestamp,
                        trigger_reason: TriggerReasonEnum::RemoteStop, // Defaulting
                        seq_no,
                        transaction_info: Transaction {
                            transaction_id: tx_id,
                            charging_state: Some(ChargingStateEnum::Idle), // Stopped
                            time_spent_charging: Some(
                                (timestamp.timestamp() - current_tx.start_timestamp) as i32,
                            ),
                            stopped_reason: Some("RemoteStop".to_string()),
                            remote_start_id: None,
                        },
                        evse: Some(EVSE {
                            id: self.id as i32,
                            connector_id: Some(1),
                        }),
                    });
                }

                // Transitioning from Charging/Suspended to Available. Stop Transaction.
                self.transaction_manager
                    .stop_transaction()
                    .map_err(|e| e.to_string())?;
            }
            EvseEvent::Fault => {
                // Fault ends the transaction per invariant:
                // Charging -> Faulted must emit TransactionEvent(Ended, reason=Error)
                if let Some(current_tx) = &self.transaction_manager.current_transaction {
                    let tx_id = current_tx.transaction_id.clone();
                    let seq_no = current_tx.seq_no + 1;

                    // Emit TransactionEvent(Ended)
                    tx_event_req = Some(TransactionEventRequest {
                        event_type: TransactionEventEnum::Ended,
                        timestamp,
                        trigger_reason: TriggerReasonEnum::ChargingStateChanged,
                        seq_no,
                        transaction_info: Transaction {
                            transaction_id: tx_id,
                            charging_state: Some(ChargingStateEnum::SuspendedEVSE), // Fault implies suspension/end
                            time_spent_charging: Some(
                                (timestamp.timestamp() - current_tx.start_timestamp) as i32,
                            ),
                            stopped_reason: Some("Fault".to_string()),
                            remote_start_id: None,
                        },
                        evse: Some(EVSE {
                            id: self.id as i32,
                            connector_id: Some(1),
                        }),
                    });
                }

                // Stop Transaction
                self.transaction_manager
                    .stop_transaction()
                    .map_err(|e| e.to_string())?;
            }
            EvseEvent::PauseEV | EvseEvent::PauseEVSE | EvseEvent::ResumeCharging => {
                // Transaction continues, but state changes.
                if let Some(current_tx) = &self.transaction_manager.current_transaction {
                    let transaction_id = current_tx.transaction_id.clone();
                    let start_timestamp = current_tx.start_timestamp;
                    // Update the transaction sequence number
                    self.transaction_manager
                        .update_transaction()
                        .map_err(|e| e.to_string())?;
                    let seq_no = self
                        .transaction_manager
                        .current_transaction
                        .as_ref()
                        .unwrap()
                        .seq_no;

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

                    // Emit TransactionEvent(Updated)
                    tx_event_req = Some(TransactionEventRequest {
                        event_type: TransactionEventEnum::Updated,
                        timestamp,
                        trigger_reason: reason,
                        seq_no,
                        transaction_info: Transaction {
                            transaction_id,
                            charging_state: Some(charging_state),
                            time_spent_charging: Some(
                                (timestamp.timestamp() - start_timestamp) as i32,
                            ),
                            stopped_reason: None,
                            remote_start_id: None,
                        },
                        evse: Some(EVSE {
                            id: self.id as i32,
                            connector_id: Some(1),
                        }),
                    });
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
            Some(ChargingStateEnum::SuspendedEVSE)
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
