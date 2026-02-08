use crate::ocpp::messages::{
    ChargingStateEnum, Transaction, TransactionEventEnum, TransactionEventRequest,
    TriggerReasonEnum, EVSE,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveTransaction {
    pub transaction_id: String,
    pub seq_no: i32,
    pub start_timestamp: DateTime<Utc>,
    pub charging_state: ChargingStateEnum,
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
        timestamp: DateTime<Utc>,
        evse_id: i32,
        trigger_reason: TriggerReasonEnum,
    ) -> Result<TransactionEventRequest, String> {
        if self.current_transaction.is_some() {
            return Err("Transaction already active".to_string());
        }

        let active_tx = ActiveTransaction {
            transaction_id: transaction_id.clone(),
            seq_no: 0,
            start_timestamp: timestamp,
            charging_state: ChargingStateEnum::Charging,
        };

        self.current_transaction = Some(active_tx);

        Ok(TransactionEventRequest {
            event_type: TransactionEventEnum::Started,
            timestamp,
            trigger_reason,
            seq_no: 0,
            transaction_info: Transaction {
                transaction_id,
                charging_state: Some(ChargingStateEnum::Charging),
                time_spent_charging: Some(0),
                stopped_reason: None,
                remote_start_id: None,
            },
            evse: Some(EVSE {
                id: evse_id,
                connector_id: Some(1), // Assume connector 1 for now
            }),
        })
    }

    pub fn update_transaction(
        &mut self,
        timestamp: DateTime<Utc>,
        evse_id: i32,
        trigger_reason: TriggerReasonEnum,
        new_charging_state: ChargingStateEnum,
    ) -> Result<TransactionEventRequest, String> {
        let tx = self
            .current_transaction
            .as_mut()
            .ok_or("No active transaction".to_string())?;

        tx.seq_no += 1;
        tx.charging_state = new_charging_state.clone();

        let time_spent = (timestamp - tx.start_timestamp).num_seconds() as i32;

        Ok(TransactionEventRequest {
            event_type: TransactionEventEnum::Updated,
            timestamp,
            trigger_reason,
            seq_no: tx.seq_no,
            transaction_info: Transaction {
                transaction_id: tx.transaction_id.clone(),
                charging_state: Some(new_charging_state),
                time_spent_charging: Some(time_spent),
                stopped_reason: None,
                remote_start_id: None,
            },
            evse: Some(EVSE {
                id: evse_id,
                connector_id: Some(1),
            }),
        })
    }

    pub fn stop_transaction(
        &mut self,
        timestamp: DateTime<Utc>,
        evse_id: i32,
        trigger_reason: TriggerReasonEnum,
        stopped_reason: Option<String>,
    ) -> Result<TransactionEventRequest, String> {
        let tx = self
            .current_transaction
            .take()
            .ok_or("No active transaction to stop".to_string())?;

        let time_spent = (timestamp - tx.start_timestamp).num_seconds() as i32;
        let seq_no = tx.seq_no + 1;

        Ok(TransactionEventRequest {
            event_type: TransactionEventEnum::Ended,
            timestamp,
            trigger_reason,
            seq_no,
            transaction_info: Transaction {
                transaction_id: tx.transaction_id,
                charging_state: Some(ChargingStateEnum::Idle), // Or omitted/relevant state
                time_spent_charging: Some(time_spent),
                stopped_reason: stopped_reason.or(Some("Local".to_string())),
                remote_start_id: None,
            },
            evse: Some(EVSE {
                id: evse_id,
                connector_id: Some(1),
            }),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_transaction_lifecycle() {
        let mut tm = TransactionManager::new();
        let start_time = Utc.timestamp_opt(1000, 0).unwrap();

        // Start
        let start_req = tm
            .start_transaction(
                "tx1".to_string(),
                start_time,
                1,
                TriggerReasonEnum::RemoteStart,
            )
            .unwrap();
        assert_eq!(start_req.event_type, TransactionEventEnum::Started);
        assert_eq!(start_req.transaction_info.transaction_id, "tx1");
        assert_eq!(start_req.seq_no, 0);

        // Update
        let update_time = Utc.timestamp_opt(1010, 0).unwrap();
        let update_req = tm
            .update_transaction(
                update_time,
                1,
                TriggerReasonEnum::ChargingStateChanged,
                ChargingStateEnum::SuspendedEV,
            )
            .unwrap();
        assert_eq!(update_req.event_type, TransactionEventEnum::Updated);
        assert_eq!(update_req.seq_no, 1);
        assert_eq!(update_req.transaction_info.time_spent_charging, Some(10));
        assert_eq!(
            update_req.transaction_info.charging_state,
            Some(ChargingStateEnum::SuspendedEV)
        );

        // Stop
        let stop_time = Utc.timestamp_opt(1020, 0).unwrap();
        let stop_req = tm
            .stop_transaction(
                stop_time,
                1,
                TriggerReasonEnum::RemoteStop,
                Some("RemoteStop".to_string()),
            )
            .unwrap();
        assert_eq!(stop_req.event_type, TransactionEventEnum::Ended);
        assert_eq!(stop_req.seq_no, 2);
        assert_eq!(stop_req.transaction_info.time_spent_charging, Some(20));
        assert!(tm.current_transaction.is_none());
    }

    #[test]
    fn test_invariants() {
        let mut tm = TransactionManager::new();
        let ts = Utc.timestamp_opt(100, 0).unwrap();

        // Cannot update or stop if no tx
        assert!(tm
            .update_transaction(
                ts,
                1,
                TriggerReasonEnum::Trigger,
                ChargingStateEnum::Charging
            )
            .is_err());
        assert!(tm
            .stop_transaction(ts, 1, TriggerReasonEnum::RemoteStop, None)
            .is_err());

        // Cannot start if already active
        tm.start_transaction("tx1".to_string(), ts, 1, TriggerReasonEnum::RemoteStart)
            .unwrap();
        assert!(tm
            .start_transaction("tx2".to_string(), ts, 1, TriggerReasonEnum::RemoteStart)
            .is_err());
    }
}
