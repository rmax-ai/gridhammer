use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ChargingProfilePurpose {
    ChargingStationMaxProfile,
    TxDefaultProfile,
    TxProfile,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ChargingProfileKind {
    Absolute,
    Recurring,
    Relative,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RecurrencyKind {
    Daily,
    Weekly,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ChargingRateUnit {
    Watts,
    Amperes,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChargingSchedulePeriod {
    pub start_period: i32,
    pub limit: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_phases: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChargingSchedule {
    pub id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_schedule: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i32>,
    pub charging_rate_unit: ChargingRateUnit,
    pub charging_schedule_period: Vec<ChargingSchedulePeriod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_charging_rate: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ChargingProfile {
    pub id: i32,
    pub stack_level: i32,
    pub charging_profile_purpose: ChargingProfilePurpose,
    pub charging_profile_kind: ChargingProfileKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurrency_kind: Option<RecurrencyKind>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_from: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_to: Option<DateTime<Utc>>,
    pub charging_schedule: ChargingSchedule,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_charging_profile_serialization() {
        let profile = ChargingProfile {
            id: 1,
            stack_level: 0,
            charging_profile_purpose: ChargingProfilePurpose::TxProfile,
            charging_profile_kind: ChargingProfileKind::Absolute,
            recurrency_kind: None,
            valid_from: None,
            valid_to: None,
            charging_schedule: ChargingSchedule {
                id: 1,
                start_schedule: None,
                duration: Some(3600),
                charging_rate_unit: ChargingRateUnit::Watts,
                charging_schedule_period: vec![ChargingSchedulePeriod {
                    start_period: 0,
                    limit: 11000.0,
                    number_phases: Some(3),
                }],
                min_charging_rate: None,
            },
        };
        let json = serde_json::to_string(&profile).unwrap();
        let deserialized: ChargingProfile = serde_json::from_str(&json).unwrap();
        assert_eq!(profile, deserialized);
    }
}
