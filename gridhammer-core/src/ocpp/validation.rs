use crate::evse::state::EvseState;
use crate::ocpp::messages::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidationError {
    MissingField(String),
    InvalidValue(String),
    ConstraintViolation(String),
    IllegalStateTransition {
        message: String,
        current_state: String,
    },
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::MissingField(field) => write!(f, "Missing required field: {}", field),
            ValidationError::InvalidValue(msg) => write!(f, "Invalid value: {}", msg),
            ValidationError::ConstraintViolation(msg) => write!(f, "Constraint violation: {}", msg),
            ValidationError::IllegalStateTransition {
                message,
                current_state,
            } => write!(
                f,
                "Illegal state transition for {}: current state is {}",
                message, current_state
            ),
        }
    }
}

impl std::error::Error for ValidationError {}

// ---------------------------------------------------------------------------
// Incoming Message Validation (CSMS -> EVSE)
// ---------------------------------------------------------------------------

pub fn validate_set_charging_profile(
    req: &SetChargingProfileRequest,
) -> Result<(), ValidationError> {
    let profile = &req.charging_profile;

    if profile.stack_level < 0 {
        return Err(ValidationError::InvalidValue(format!(
            "Stack level cannot be negative: {}",
            profile.stack_level
        )));
    }

    for schedule in &profile.charging_schedule {
        if let Some(duration) = schedule.duration {
            if duration < 0 {
                return Err(ValidationError::InvalidValue(format!(
                    "Duration cannot be negative: {}",
                    duration
                )));
            }
        }

        if let Some(min_rate) = schedule.min_charging_rate {
            if min_rate < 0.0 {
                return Err(ValidationError::InvalidValue(format!(
                    "Min charging rate cannot be negative: {}",
                    min_rate
                )));
            }
        }

        for (idx, period) in schedule.charging_schedule_period.iter().enumerate() {
            if period.limit < 0.0 {
                return Err(ValidationError::InvalidValue(format!(
                    "Charging schedule period limit cannot be negative: {} (at index {})",
                    period.limit, idx
                )));
            }
            if period.start_period < 0 {
                return Err(ValidationError::InvalidValue(format!(
                    "Start period cannot be negative: {} (at index {})",
                    period.start_period, idx
                )));
            }
            if let Some(phases) = period.number_phases {
                if phases <= 0 {
                    return Err(ValidationError::InvalidValue(format!(
                        "Number of phases must be positive: {} (at index {})",
                        phases, idx
                    )));
                }
            }
        }

        // Ensure periods are sorted by start_period?
        // OCPP 2.1 doesn't explicitly strict sort, but it's implied.
        // Let's enforce it for sanity.
        let mut prev_start = -1;
        for (idx, period) in schedule.charging_schedule_period.iter().enumerate() {
            if period.start_period <= prev_start {
                return Err(ValidationError::ConstraintViolation(format!(
                      "Charging schedule periods must be sorted and unique. Index {} start {} <= prev {}", 
                      idx, period.start_period, prev_start
                  )));
            }
            prev_start = period.start_period;
        }
    }

    Ok(())
}

// ---------------------------------------------------------------------------
// Outgoing Message Validation (EVSE -> CSMS)
// ---------------------------------------------------------------------------

pub fn validate_transaction_event(
    req: &TransactionEventRequest,
    current_state: EvseState,
) -> Result<(), ValidationError> {
    // State-dependent legality
    match req.event_type {
        TransactionEventEnum::Started => {
            if matches!(current_state, EvseState::Faulted | EvseState::Unavailable) {
                return Err(ValidationError::IllegalStateTransition {
                    message: "TransactionEvent(Started)".to_string(),
                    current_state: format!("{:?}", current_state),
                });
            }
        }
        TransactionEventEnum::Ended => {
            // Cannot end if we are not in a transaction-like state?
            // But we might be in Available if we just ended it?
            // This depends on when this validation is called (pre or post state update).
            // Assuming Pre-send, Post-state-update (or Pre-state-update).
            // Let's assume this checks if the message makes sense *given the state of the EVSE at generation time*.

            // If we are sending Ended, we must have been in a transaction.
        }
        _ => {}
    }

    // Semantic checks
    if req.seq_no < 0 {
        return Err(ValidationError::InvalidValue(format!(
            "Sequence number cannot be negative: {}",
            req.seq_no
        )));
    }

    Ok(())
}

pub fn validate_meter_values(req: &MeterValuesRequest) -> Result<(), ValidationError> {
    if req.meter_value.is_empty() {
        return Err(ValidationError::MissingField(
            "meterValue list cannot be empty".to_string(),
        ));
    }

    for mv in &req.meter_value {
        if mv.sampled_value.is_empty() {
            return Err(ValidationError::MissingField(
                "sampledValue list cannot be empty".to_string(),
            ));
        }
        for sv in &mv.sampled_value {
            if let Some(_mult) = sv.unit_of_measure.as_ref().and_then(|u| u.multiplier) {
                // Check reasonable multiplier? (optional)
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_validate_set_charging_profile_valid() {
        let req = SetChargingProfileRequest {
            evse_id: 1,
            charging_profile: ChargingProfile {
                id: 1,
                stack_level: 1,
                charging_profile_purpose: ChargingProfilePurposeEnum::TxProfile,
                charging_profile_kind: ChargingProfileKindEnum::Absolute,
                recurrency_kind: None,
                valid_from: None,
                valid_to: None,
                transaction_id: None,
                charging_schedule: vec![ChargingSchedule {
                    id: 1,
                    start_schedule: None,
                    duration: None,
                    charging_rate_unit: ChargingRateUnitEnum::W,
                    min_charging_rate: None,
                    charging_schedule_period: vec![
                        ChargingSchedulePeriod {
                            start_period: 0,
                            limit: 1000.0,
                            number_phases: Some(3),
                            phase_to_use: None,
                        },
                        ChargingSchedulePeriod {
                            start_period: 3600,
                            limit: 0.0,
                            number_phases: None,
                            phase_to_use: None,
                        },
                    ],
                }],
            },
        };
        assert!(validate_set_charging_profile(&req).is_ok());
    }

    #[test]
    fn test_validate_set_charging_profile_negative_limit() {
        let req = SetChargingProfileRequest {
            evse_id: 1,
            charging_profile: ChargingProfile {
                id: 1,
                stack_level: 1,
                charging_profile_purpose: ChargingProfilePurposeEnum::TxProfile,
                charging_profile_kind: ChargingProfileKindEnum::Absolute,
                recurrency_kind: None,
                valid_from: None,
                valid_to: None,
                transaction_id: None,
                charging_schedule: vec![ChargingSchedule {
                    id: 1,
                    start_schedule: None,
                    duration: None,
                    charging_rate_unit: ChargingRateUnitEnum::W,
                    min_charging_rate: None,
                    charging_schedule_period: vec![ChargingSchedulePeriod {
                        start_period: 0,
                        limit: -100.0,
                        number_phases: None,
                        phase_to_use: None,
                    }],
                }],
            },
        };

        match validate_set_charging_profile(&req) {
            Err(ValidationError::InvalidValue(msg)) => assert!(msg.contains("negative")),
            _ => panic!("Expected InvalidValue error"),
        }
    }

    #[test]
    fn test_validate_transaction_event_started_while_faulted() {
        let req = TransactionEventRequest {
            event_type: TransactionEventEnum::Started,
            timestamp: Utc::now(),
            trigger_reason: TriggerReasonEnum::CablePluggedIn,
            seq_no: 1,
            transaction_info: Transaction {
                transaction_id: "123".to_string(),
                charging_state: None,
                time_spent_charging: None,
                stopped_reason: None,
                remote_start_id: None,
            },
            evse: None,
        };

        let err = validate_transaction_event(&req, EvseState::Faulted);
        assert!(matches!(
            err,
            Err(ValidationError::IllegalStateTransition { .. })
        ));
    }
}
