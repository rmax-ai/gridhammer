use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// TransactionEvent
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransactionEventEnum {
    Started,
    Updated,
    Ended,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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

// ---------------------------------------------------------------------------
// BootNotification
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BootReasonEnum {
    ApplicationReset,
    FirmwareUpdate,
    LocalReset,
    PowerUp,
    RemoteReset,
    ScheduledReset,
    Triggered,
    Unknown,
    Watchdog,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Modem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iccid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imsi: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChargingStation {
    pub model: String,
    pub vendor_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firmware_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modem: Option<Modem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BootNotificationRequest {
    pub reason: BootReasonEnum,
    pub charging_station: ChargingStation,
}

// ---------------------------------------------------------------------------
// MeterValues
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReadingContextEnum {
    #[serde(rename = "Interruption.Begin")]
    InterruptionBegin,
    #[serde(rename = "Interruption.End")]
    InterruptionEnd,
    Other,
    #[serde(rename = "Sample.Clock")]
    SampleClock,
    #[serde(rename = "Sample.Periodic")]
    SamplePeriodic,
    #[serde(rename = "Transaction.Begin")]
    TransactionBegin,
    #[serde(rename = "Transaction.End")]
    TransactionEnd,
    Trigger,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MeasurandEnum {
    #[serde(rename = "Current.Export")]
    CurrentExport,
    #[serde(rename = "Current.Import")]
    CurrentImport,
    #[serde(rename = "Current.Offered")]
    CurrentOffered,
    #[serde(rename = "Energy.Active.Export.Register")]
    EnergyActiveExportRegister,
    #[serde(rename = "Energy.Active.Import.Register")]
    EnergyActiveImportRegister,
    #[serde(rename = "Energy.Reactive.Export.Register")]
    EnergyReactiveExportRegister,
    #[serde(rename = "Energy.Reactive.Import.Register")]
    EnergyReactiveImportRegister,
    #[serde(rename = "Energy.Active.Export.Interval")]
    EnergyActiveExportInterval,
    #[serde(rename = "Energy.Active.Import.Interval")]
    EnergyActiveImportInterval,
    #[serde(rename = "Energy.Reactive.Export.Interval")]
    EnergyReactiveExportInterval,
    #[serde(rename = "Energy.Reactive.Import.Interval")]
    EnergyReactiveImportInterval,
    #[serde(rename = "Power.Active.Export")]
    PowerActiveExport,
    #[serde(rename = "Power.Active.Import")]
    PowerActiveImport,
    #[serde(rename = "Power.Factor")]
    PowerFactor,
    #[serde(rename = "Power.Offered")]
    PowerOffered,
    #[serde(rename = "Power.Reactive.Export")]
    PowerReactiveExport,
    #[serde(rename = "Power.Reactive.Import")]
    PowerReactiveImport,
    #[serde(rename = "SoC")]
    SoC,
    #[serde(rename = "Voltage")]
    Voltage,
    // Add others as needed
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PhaseEnum {
    L1,
    L2,
    L3,
    N,
    #[serde(rename = "L1-N")]
    L1N,
    #[serde(rename = "L1-L2")]
    L1L2,
    #[serde(rename = "L2-L3")]
    L2L3,
    #[serde(rename = "L3-L1")]
    L3L1,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LocationEnum {
    Body,
    Cable,
    EV,
    Inlet,
    Outlet,
    Upstream,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnitOfMeasure {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiplier: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SampledValue {
    pub value: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<ReadingContextEnum>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub measurand: Option<MeasurandEnum>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase: Option<PhaseEnum>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<LocationEnum>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_of_measure: Option<UnitOfMeasure>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeterValue {
    pub timestamp: DateTime<Utc>,
    pub sampled_value: Vec<SampledValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MeterValuesRequest {
    pub evse_id: i32,
    pub meter_value: Vec<MeterValue>,
}

// ---------------------------------------------------------------------------
// SetChargingProfile
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChargingProfilePurposeEnum {
    ChargingStationExternalConstraints,
    ChargingStationMaxProfile,
    TxDefaultProfile,
    TxProfile,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChargingProfileKindEnum {
    Absolute,
    Recurring,
    Relative,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RecurrencyKindEnum {
    Daily,
    Weekly,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChargingRateUnitEnum {
    W,
    A,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChargingSchedulePeriod {
    pub start_period: i32,
    pub limit: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_phases: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase_to_use: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChargingSchedule {
    pub id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_schedule: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    pub charging_rate_unit: ChargingRateUnitEnum,
    pub charging_schedule_period: Vec<ChargingSchedulePeriod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_charging_rate: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChargingProfile {
    pub id: i32,
    pub stack_level: i32,
    pub charging_profile_purpose: ChargingProfilePurposeEnum,
    pub charging_profile_kind: ChargingProfileKindEnum,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurrency_kind: Option<RecurrencyKindEnum>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_from: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_to: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
    pub charging_schedule: Vec<ChargingSchedule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetChargingProfileRequest {
    pub evse_id: i32,
    pub charging_profile: ChargingProfile,
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_boot_notification_request() {
        let req = BootNotificationRequest {
            reason: BootReasonEnum::PowerUp,
            charging_station: ChargingStation {
                model: "ModelX".to_string(),
                vendor_name: "VendorY".to_string(),
                serial_number: Some("12345".to_string()),
                firmware_version: Some("1.0.0".to_string()),
                modem: None,
            },
        };

        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"reason\":\"PowerUp\""));
        assert!(json.contains("\"model\":\"ModelX\""));
        assert!(json.contains("\"vendorName\":\"VendorY\""));
    }

    #[test]
    fn test_meter_values_request() {
        let req = MeterValuesRequest {
            evse_id: 1,
            meter_value: vec![MeterValue {
                timestamp: Utc.timestamp_opt(1672531200, 0).unwrap(),
                sampled_value: vec![SampledValue {
                    value: 1000.0,
                    context: Some(ReadingContextEnum::SamplePeriodic),
                    measurand: Some(MeasurandEnum::PowerActiveImport),
                    phase: None,
                    location: None,
                    unit_of_measure: Some(UnitOfMeasure {
                        unit: Some("W".to_string()),
                        multiplier: Some(0),
                    }),
                }],
            }],
        };

        let json = serde_json::to_string(&req).unwrap();
        assert!(json.contains("\"evseId\":1"));
        assert!(json.contains("\"measurand\":\"Power.Active.Import\""));
        assert!(json.contains("\"context\":\"Sample.Periodic\""));
    }

    #[test]
    fn test_set_charging_profile_request_deserialization() {
        let json = r#"{
            "evseId": 1,
            "chargingProfile": {
                "id": 100,
                "stackLevel": 1,
                "chargingProfilePurpose": "TxProfile",
                "chargingProfileKind": "Absolute",
                "chargingSchedule": [
                    {
                        "id": 101,
                        "chargingRateUnit": "W",
                        "chargingSchedulePeriod": [
                            {
                                "startPeriod": 0,
                                "limit": 11000.0
                            }
                        ]
                    }
                ]
            }
        }"#;

        let req: SetChargingProfileRequest = serde_json::from_str(json).unwrap();
        assert_eq!(req.evse_id, 1);
        assert_eq!(req.charging_profile.id, 100);
        assert_eq!(
            req.charging_profile.charging_schedule[0].charging_schedule_period[0].limit,
            11000.0
        );
    }
}
