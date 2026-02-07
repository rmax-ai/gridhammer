use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// TransactionEvent
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TransactionEventEnum {
    Started,
    Updated,
    Ended,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum TriggerReasonEnum {
    Authorized,
    CablePluggedIn,
    ChargingRateChanged,
    ChargingStateChanged,
    Deauthorized,
    EnergyLimitReached,
    EVCommunicationLost,
    EVConnectTimeout,
    MeterValueClock,
    MeterValuePeriodic,
    TimeLimitReached,
    Trigger,
    UnlockCommand,
    StopAuthorized,
    EVDeparted,
    EVDetected,
    RemoteStop,
    RemoteStart,
    AbnormalCondition,
    SignedDataReceived,
    ResetCommand,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ChargingStateEnum {
    Charging,
    EVConnected,
    SuspendedEV,
    SuspendedEVSE,
    Idle,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub transaction_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charging_state: Option<ChargingStateEnum>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_spent_charging: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stopped_reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_start_id: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EVSE {
    pub id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_id: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionEventRequest {
    pub event_type: TransactionEventEnum,
    pub timestamp: DateTime<Utc>,
    pub trigger_reason: TriggerReasonEnum,
    pub seq_no: i32,
    pub transaction_info: Transaction,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evse: Option<EVSE>,
    // TODO: meter_values, id_token
}

// ---------------------------------------------------------------------------
// StatusNotification
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ConnectorStatusEnum {
    Available,
    Occupied,
    Reserved,
    Unavailable,
    Faulted,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StatusNotificationRequest {
    pub timestamp: DateTime<Utc>,
    pub connector_status: ConnectorStatusEnum,
    pub evse_id: i32,
    pub connector_id: i32,
}
