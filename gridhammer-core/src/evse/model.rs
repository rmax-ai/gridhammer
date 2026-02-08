use super::fsm::transition;
use super::state::{EvseEvent, EvseState};
use super::transaction::TransactionManager;
use crate::ocpp::messages::{ChargingStateEnum, TransactionEventRequest, TriggerReasonEnum};
use crate::profiles::executor::{EnforcedLimit, Executor};
use crate::profiles::model::ChargingProfile as CoreChargingProfile;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Evse {
    pub id: u32,
    pub state: EvseState,
    pub transaction_manager: TransactionManager,
    pub charging_profiles: Vec<CoreChargingProfile>,
    pub current_limit: Option<EnforcedLimit>,
}

impl Evse {
    pub fn new(id: u32) -> Self {
        Self {
            id,
            state: EvseState::Available,
            transaction_manager: TransactionManager::new(),
            charging_profiles: Vec::new(),
            current_limit: None,
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
            EvseEvent::Unplug => {
                // Unplug ends the transaction if active (unexpected cable removal)
                if self.transaction_manager.current_transaction.is_some() {
                    let req = self
                        .transaction_manager
                        .stop_transaction(
                            timestamp,
                            self.id as i32,
                            TriggerReasonEnum::EVCommunicationLost,
                            Some("EVDisconnected".to_string()),
                        )
                        .map_err(|e| e.to_string())?;
                    tx_event_req = Some(req);
                }
            }
            // For updates, we might need a separate trigger or assume periodic updates
            // But here we are handling discrete events.
            _ => {}
        }

        self.state = new_state;
        Ok((new_state, tx_event_req))
    }

    pub fn set_charging_profile(&mut self, profile: CoreChargingProfile) {
        // Remove existing profile with same id or same stack_level/purpose collision
        self.charging_profiles.retain(|p| {
            let same_id = p.id == profile.id;
            let same_purpose_stack = p.charging_profile_purpose == profile.charging_profile_purpose
                && p.stack_level == profile.stack_level;
            !same_id && !same_purpose_stack
        });

        self.charging_profiles.push(profile);
    }

    pub fn tick_transaction(&mut self, current_time: DateTime<Utc>) -> Result<(), String> {
        let transaction_start = self
            .transaction_manager
            .current_transaction
            .as_ref()
            .map(|tx| tx.start_timestamp);

        let limit = Executor::resolve(
            self.charging_profiles.clone(),
            current_time,
            10000.0, // dummy requested_power
            10000.0, // dummy site_caps
            transaction_start,
        )
        .map_err(|e| e.to_string())?;

        self.current_limit = limit;
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
    fn test_unplug_should_end_transaction() {
        let mut evse = Evse::new(1);
        let ts = Utc.timestamp_opt(100, 0).unwrap();

        // Start Charging
        evse.handle_event(EvseEvent::PlugIn, ts, None).unwrap();
        evse.handle_event(EvseEvent::StartCharging, ts, Some("tx1".to_string()))
            .unwrap();
        assert_eq!(evse.state, EvseState::Charging);
        assert!(evse.transaction_manager.current_transaction.is_some());

        // Unplug (Unexpectedly removed cable)
        let (state, tx_req) = evse.handle_event(EvseEvent::Unplug, ts, None).unwrap();

        // Should be Available
        assert_eq!(state, EvseState::Available);

        // Transaction should be ENDED (or at least stopped)
        // Check if we got an Ended event
        let req = tx_req.expect("Should emit Ended event on Unplug");
        assert_eq!(req.event_type, TransactionEventEnum::Ended);
        assert_eq!(
            req.transaction_info.stopped_reason,
            Some("EVDisconnected".to_string())
        );

        // Transaction manager should be empty
        assert!(evse.transaction_manager.current_transaction.is_none());
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

    #[test]
    fn test_charging_profile_integration() {
        use crate::profiles::model::{
            ChargingProfileKind, ChargingProfilePurpose, ChargingRateUnit, ChargingSchedule,
            ChargingSchedulePeriod,
        };

        let mut evse = Evse::new(1);
        let now = Utc.timestamp_opt(100, 0).unwrap();

        // No profiles initially
        evse.tick_transaction(now).unwrap();
        assert_eq!(evse.current_limit, None);

        // Add a profile
        let profile = CoreChargingProfile {
            id: 1,
            stack_level: 1,
            charging_profile_purpose: ChargingProfilePurpose::TxProfile,
            charging_profile_kind: ChargingProfileKind::Absolute,
            recurrency_kind: None,
            valid_from: None,
            valid_to: None,
            charging_schedule: ChargingSchedule {
                id: 1,
                start_schedule: None,
                duration: None,
                charging_rate_unit: ChargingRateUnit::Watts,
                charging_schedule_period: vec![ChargingSchedulePeriod {
                    start_period: 0,
                    limit: 5000.0,
                    number_phases: None,
                }],
                min_charging_rate: None,
            },
        };

        evse.set_charging_profile(profile);
        evse.tick_transaction(now).unwrap();

        let limit = evse.current_limit.expect("Should have a limit");
        assert_eq!(limit.limit, 5000.0);
    }
}
