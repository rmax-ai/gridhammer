use super::model::{
    ChargingProfile, ChargingProfileKind, ChargingProfilePurpose, ChargingRateUnit, RecurrencyKind,
};
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EnforcementReason {
    ProfileLimit,
    StationLimit,
    InternalConstraint,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EnforcedLimit {
    pub limit: f64,
    pub unit: ChargingRateUnit,
    pub reason: EnforcementReason,
    pub should_suspend: bool,
}

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum ExecutorError {
    #[error("Limit is NaN")]
    LimitIsNan,
}

pub struct Executor;

impl Executor {
    pub fn resolve(
        profiles: Vec<ChargingProfile>,
        current_time: DateTime<Utc>,
        requested_power: f64,
        site_caps: f64,
        transaction_start: Option<DateTime<Utc>>,
    ) -> Result<Option<EnforcedLimit>, ExecutorError> {
        // Filter valid profiles
        let valid_profiles: Vec<ChargingProfile> = profiles
            .into_iter()
            .filter(|p| {
                p.valid_from.is_none_or(|from| current_time >= from)
                    && p.valid_to.is_none_or(|to| current_time <= to)
            })
            .collect();

        // Group by purpose
        let mut purpose_map: HashMap<ChargingProfilePurpose, Vec<ChargingProfile>> = HashMap::new();
        for profile in valid_profiles {
            purpose_map
                .entry(profile.charging_profile_purpose.clone())
                .or_default()
                .push(profile);
        }

        // Select the best profile for each purpose (highest stack_level, then highest id)
        let mut selected_profiles: HashMap<ChargingProfilePurpose, ChargingProfile> =
            HashMap::new();
        for (purpose, mut profs) in purpose_map {
            profs.sort_by(|a, b| b.stack_level.cmp(&a.stack_level).then(b.id.cmp(&a.id)));
            if let Some(best) = profs.into_iter().next() {
                selected_profiles.insert(purpose, best);
            }
        }

        // Get planned limit from TxProfile or TxDefaultProfile
        let planned_profile = selected_profiles
            .get(&ChargingProfilePurpose::TxProfile)
            .or_else(|| selected_profiles.get(&ChargingProfilePurpose::TxDefaultProfile));

        let planned = planned_profile.map(|profile| {
            let (val, unit) = evaluate_profile(profile, current_time, transaction_start);
            (val, unit, profile.id)
        });

        // Get limit from ChargingStationMaxProfile
        let limit_profile =
            selected_profiles.get(&ChargingProfilePurpose::ChargingStationMaxProfile);
        let limit = limit_profile.map(|profile| {
            let (val, unit) = evaluate_profile(profile, current_time, transaction_start);
            (val, unit, profile.id)
        });

        // Combine: min of planned and limit
        let result = match (planned, limit) {
            (Some((p_val, p_unit, _)), Some((l_val, l_unit, _))) => {
                if p_unit == l_unit {
                    if p_val < l_val {
                        Some(EnforcedLimit {
                            limit: p_val,
                            unit: p_unit,
                            reason: EnforcementReason::ProfileLimit,
                            should_suspend: false,
                        })
                    } else {
                        Some(EnforcedLimit {
                            limit: l_val,
                            unit: l_unit,
                            reason: EnforcementReason::StationLimit,
                            should_suspend: false,
                        })
                    }
                } else {
                    // Special case: Zero Power Semantics.
                    if p_val <= 0.0 {
                        Some(EnforcedLimit {
                            limit: p_val,
                            unit: p_unit,
                            reason: EnforcementReason::ProfileLimit,
                            should_suspend: false,
                        })
                    } else if l_val <= 0.0 {
                        Some(EnforcedLimit {
                            limit: l_val,
                            unit: l_unit,
                            reason: EnforcementReason::StationLimit,
                            should_suspend: false,
                        })
                    } else {
                        // Prefer StationMax as safety net
                        Some(EnforcedLimit {
                            limit: l_val,
                            unit: l_unit,
                            reason: EnforcementReason::StationLimit,
                            should_suspend: false,
                        })
                    }
                }
            }
            (Some((p_val, p_unit, _)), None) => Some(EnforcedLimit {
                limit: p_val,
                unit: p_unit,
                reason: EnforcementReason::ProfileLimit,
                should_suspend: false,
            }),
            (None, Some((l_val, l_unit, _))) => Some(EnforcedLimit {
                limit: l_val,
                unit: l_unit,
                reason: EnforcementReason::StationLimit,
                should_suspend: false,
            }),
            (None, None) => None,
        };

        // Apply requested_power and site_caps
        let final_enforced = if let Some(mut e) = result {
            if e.limit.is_nan() {
                return Err(ExecutorError::LimitIsNan);
            }
            e.limit = e.limit.min(requested_power).min(site_caps);
            Some(e)
        } else {
            None
        };

        // Enforce invariants
        if let Some(mut enforced) = final_enforced {
            if enforced.limit <= 0.0 {
                enforced.limit = 0.0;
                enforced.should_suspend = true;
            }
            Ok(Some(enforced))
        } else {
            Ok(None)
        }
    }
}

fn evaluate_profile(
    profile: &ChargingProfile,
    current_time: DateTime<Utc>,
    transaction_start: Option<DateTime<Utc>>,
) -> (f64, ChargingRateUnit) {
    let schedule = &profile.charging_schedule;
    let effective_start = match profile.charging_profile_kind {
        ChargingProfileKind::Absolute => schedule.start_schedule.unwrap_or(current_time),
        ChargingProfileKind::Recurring => {
            let base = schedule.start_schedule.unwrap_or(current_time);
            let time_since_base = current_time - base;
            let period_seconds = match profile.recurrency_kind {
                Some(RecurrencyKind::Daily) => 86400,
                Some(RecurrencyKind::Weekly) => 604800,
                None => 86400, // default to daily
            };
            let cycles = time_since_base.num_seconds() / period_seconds;
            base + Duration::seconds(cycles * period_seconds)
        }
        ChargingProfileKind::Relative => transaction_start.unwrap_or(current_time),
    };

    // Calculate elapsed seconds from effective_start
    let elapsed = (current_time - effective_start).num_seconds() as i32;

    // Find the current limit from periods (step function)
    let mut current_limit = 0.0;
    for period in &schedule.charging_schedule_period {
        if period.start_period <= elapsed {
            current_limit = period.limit;
        } else {
            break;
        }
    }

    (current_limit, schedule.charging_rate_unit.clone())
}

#[cfg(test)]
mod tests {
    use super::super::model::ChargingSchedulePeriod;
    use super::*;
    use chrono::TimeZone;

    fn create_profile(
        id: i32,
        stack_level: i32,
        purpose: ChargingProfilePurpose,
        kind: ChargingProfileKind,
        periods: Vec<ChargingSchedulePeriod>,
        start_schedule: Option<DateTime<Utc>>,
        unit: ChargingRateUnit,
    ) -> ChargingProfile {
        ChargingProfile {
            id,
            stack_level,
            charging_profile_purpose: purpose,
            charging_profile_kind: kind,
            recurrency_kind: None,
            valid_from: None,
            valid_to: None,
            charging_schedule: super::super::model::ChargingSchedule {
                id: 1,
                start_schedule,
                duration: None,
                charging_rate_unit: unit,
                charging_schedule_period: periods,
                min_charging_rate: None,
            },
        }
    }

    #[test]
    fn test_basic_selection_by_stack_level() {
        let current_time = Utc.with_ymd_and_hms(2023, 1, 1, 12, 0, 0).unwrap();
        let profiles = vec![
            create_profile(
                1,
                1,
                ChargingProfilePurpose::TxProfile,
                ChargingProfileKind::Absolute,
                vec![ChargingSchedulePeriod {
                    start_period: 0,
                    limit: 1000.0,
                    number_phases: None,
                }],
                Some(current_time),
                ChargingRateUnit::Watts,
            ),
            create_profile(
                2,
                2,
                ChargingProfilePurpose::TxProfile,
                ChargingProfileKind::Absolute,
                vec![ChargingSchedulePeriod {
                    start_period: 0,
                    limit: 2000.0,
                    number_phases: None,
                }],
                Some(current_time),
                ChargingRateUnit::Watts,
            ),
        ];
        let result = Executor::resolve(profiles, current_time, 10000.0, 10000.0, None).unwrap();
        assert_eq!(
            result,
            Some(EnforcedLimit {
                limit: 2000.0,
                unit: ChargingRateUnit::Watts,
                reason: EnforcementReason::ProfileLimit,
                should_suspend: false,
            })
        );
    }

    #[test]
    fn test_time_checking_valid_invalid() {
        let current_time = Utc.with_ymd_and_hms(2023, 1, 1, 12, 0, 0).unwrap();
        let valid_from = Some(Utc.with_ymd_and_hms(2023, 1, 1, 10, 0, 0).unwrap());
        let valid_to = Some(Utc.with_ymd_and_hms(2023, 1, 1, 14, 0, 0).unwrap());
        let invalid_to = Some(Utc.with_ymd_and_hms(2023, 1, 1, 11, 0, 0).unwrap());
        let mut profile = create_profile(
            1,
            1,
            ChargingProfilePurpose::TxProfile,
            ChargingProfileKind::Absolute,
            vec![ChargingSchedulePeriod {
                start_period: 0,
                limit: 1000.0,
                number_phases: None,
            }],
            Some(current_time),
            ChargingRateUnit::Watts,
        );
        profile.valid_from = valid_from;
        profile.valid_to = valid_to;
        let result =
            Executor::resolve(vec![profile.clone()], current_time, 10000.0, 10000.0, None).unwrap();
        assert_eq!(
            result,
            Some(EnforcedLimit {
                limit: 1000.0,
                unit: ChargingRateUnit::Watts,
                reason: EnforcementReason::ProfileLimit,
                should_suspend: false,
            })
        );

        profile.valid_to = invalid_to;
        let result =
            Executor::resolve(vec![profile], current_time, 10000.0, 10000.0, None).unwrap();
        assert_eq!(result, None);
    }

    #[test]
    fn test_recurring_profile_daily() {
        let base_time = Utc.with_ymd_and_hms(2023, 1, 1, 0, 0, 0).unwrap();
        let current_time = base_time + Duration::hours(25); // next day, 1 hour in
        let mut profile = create_profile(
            1,
            1,
            ChargingProfilePurpose::TxProfile,
            ChargingProfileKind::Recurring,
            vec![
                ChargingSchedulePeriod {
                    start_period: 0,
                    limit: 1000.0,
                    number_phases: None,
                },
                ChargingSchedulePeriod {
                    start_period: 3600,
                    limit: 2000.0,
                    number_phases: None,
                },
            ],
            Some(base_time),
            ChargingRateUnit::Watts,
        );
        profile.recurrency_kind = Some(RecurrencyKind::Daily);
        let result =
            Executor::resolve(vec![profile], current_time, 10000.0, 10000.0, None).unwrap();
        assert_eq!(
            result,
            Some(EnforcedLimit {
                limit: 2000.0,
                unit: ChargingRateUnit::Watts,
                reason: EnforcementReason::ProfileLimit,
                should_suspend: false,
            })
        );
    }

    #[test]
    fn test_relative_profile() {
        let transaction_start = Some(Utc.with_ymd_and_hms(2023, 1, 1, 10, 0, 0).unwrap());
        let current_time = transaction_start.unwrap() + Duration::hours(1);
        let profile = create_profile(
            1,
            1,
            ChargingProfilePurpose::TxProfile,
            ChargingProfileKind::Relative,
            vec![
                ChargingSchedulePeriod {
                    start_period: 0,
                    limit: 1000.0,
                    number_phases: None,
                },
                ChargingSchedulePeriod {
                    start_period: 1800,
                    limit: 2000.0,
                    number_phases: None,
                },
            ],
            None,
            ChargingRateUnit::Watts,
        );
        let result = Executor::resolve(
            vec![profile],
            current_time,
            10000.0,
            10000.0,
            transaction_start,
        )
        .unwrap();
        assert_eq!(
            result,
            Some(EnforcedLimit {
                limit: 2000.0,
                unit: ChargingRateUnit::Watts,
                reason: EnforcementReason::ProfileLimit,
                should_suspend: false,
            })
        );
    }

    #[test]
    fn test_combination_tx_and_station_max() {
        let current_time = Utc.with_ymd_and_hms(2023, 1, 1, 12, 0, 0).unwrap();
        let tx_profile = create_profile(
            1,
            1,
            ChargingProfilePurpose::TxProfile,
            ChargingProfileKind::Absolute,
            vec![ChargingSchedulePeriod {
                start_period: 0,
                limit: 3000.0,
                number_phases: None,
            }],
            Some(current_time),
            ChargingRateUnit::Watts,
        );
        let max_profile = create_profile(
            2,
            1,
            ChargingProfilePurpose::ChargingStationMaxProfile,
            ChargingProfileKind::Absolute,
            vec![ChargingSchedulePeriod {
                start_period: 0,
                limit: 2000.0,
                number_phases: None,
            }],
            Some(current_time),
            ChargingRateUnit::Watts,
        );
        let result = Executor::resolve(
            vec![tx_profile, max_profile],
            current_time,
            10000.0,
            10000.0,
            None,
        )
        .unwrap();
        assert_eq!(
            result,
            Some(EnforcedLimit {
                limit: 2000.0,
                unit: ChargingRateUnit::Watts,
                reason: EnforcementReason::StationLimit,
                should_suspend: false,
            })
        );
    }

    #[test]
    fn test_negative_clamping() {
        let current_time = Utc.with_ymd_and_hms(2023, 1, 1, 12, 0, 0).unwrap();
        let profile = create_profile(
            1,
            1,
            ChargingProfilePurpose::TxProfile,
            ChargingProfileKind::Absolute,
            vec![ChargingSchedulePeriod {
                start_period: 0,
                limit: -100.0,
                number_phases: None,
            }],
            Some(current_time),
            ChargingRateUnit::Watts,
        );
        let result =
            Executor::resolve(vec![profile], current_time, 10000.0, 10000.0, None).unwrap();
        // Should be clamped to 0.0, but still attributed to ID 1
        assert_eq!(
            result,
            Some(EnforcedLimit {
                limit: 0.0,
                unit: ChargingRateUnit::Watts,
                reason: EnforcementReason::ProfileLimit,
                should_suspend: true,
            })
        );
    }

    #[test]
    fn test_nan_error() {
        let current_time = Utc.with_ymd_and_hms(2023, 1, 1, 12, 0, 0).unwrap();
        let profile = create_profile(
            1,
            1,
            ChargingProfilePurpose::TxProfile,
            ChargingProfileKind::Absolute,
            vec![ChargingSchedulePeriod {
                start_period: 0,
                limit: f64::NAN,
                number_phases: None,
            }],
            Some(current_time),
            ChargingRateUnit::Watts,
        );
        let result = Executor::resolve(vec![profile], current_time, 10000.0, 10000.0, None);
        assert_eq!(result, Err(ExecutorError::LimitIsNan));
    }

    #[test]
    fn test_overlapping_profiles_priority() {
        let current_time = Utc.with_ymd_and_hms(2023, 1, 1, 12, 0, 0).unwrap();
        // Lower stack level
        let p1 = create_profile(
            1,
            1,
            ChargingProfilePurpose::TxProfile,
            ChargingProfileKind::Absolute,
            vec![ChargingSchedulePeriod {
                start_period: 0,
                limit: 1000.0,
                number_phases: None,
            }],
            Some(current_time),
            ChargingRateUnit::Watts,
        );
        // Higher stack level
        let p2 = create_profile(
            2,
            2,
            ChargingProfilePurpose::TxProfile,
            ChargingProfileKind::Absolute,
            vec![ChargingSchedulePeriod {
                start_period: 0,
                limit: 2000.0,
                number_phases: None,
            }],
            Some(current_time),
            ChargingRateUnit::Watts,
        );
        // Same stack level as p2, but higher ID
        let p3 = create_profile(
            3,
            2,
            ChargingProfilePurpose::TxProfile,
            ChargingProfileKind::Absolute,
            vec![ChargingSchedulePeriod {
                start_period: 0,
                limit: 3000.0,
                number_phases: None,
            }],
            Some(current_time),
            ChargingRateUnit::Watts,
        );

        // p2 beats p1 (stack level)
        let result = Executor::resolve(
            vec![p1.clone(), p2.clone()],
            current_time,
            10000.0,
            10000.0,
            None,
        )
        .unwrap();
        assert_eq!(
            result,
            Some(EnforcedLimit {
                limit: 2000.0,
                unit: ChargingRateUnit::Watts,
                reason: EnforcementReason::ProfileLimit,
                should_suspend: false,
            })
        );

        // p3 beats p2 (same stack level, higher ID)
        let result = Executor::resolve(
            vec![p2.clone(), p3.clone()],
            current_time,
            10000.0,
            10000.0,
            None,
        )
        .unwrap();
        assert_eq!(
            result,
            Some(EnforcedLimit {
                limit: 3000.0,
                unit: ChargingRateUnit::Watts,
                reason: EnforcementReason::ProfileLimit,
                should_suspend: false,
            })
        );
    }

    #[test]
    fn test_profile_churn() {
        // Simulate time passing and profiles expiring/activating
        let start_time = Utc.with_ymd_and_hms(2023, 1, 1, 12, 0, 0).unwrap();

        let mut p1 = create_profile(
            1,
            1,
            ChargingProfilePurpose::TxProfile,
            ChargingProfileKind::Absolute,
            vec![ChargingSchedulePeriod {
                start_period: 0,
                limit: 1000.0,
                number_phases: None,
            }],
            Some(start_time),
            ChargingRateUnit::Watts,
        );
        p1.valid_to = Some(start_time + Duration::minutes(30));

        let mut p2 = create_profile(
            2,
            1,
            ChargingProfilePurpose::TxProfile,
            ChargingProfileKind::Absolute,
            vec![ChargingSchedulePeriod {
                start_period: 0,
                limit: 2000.0,
                number_phases: None,
            }],
            Some(start_time),
            ChargingRateUnit::Watts,
        );
        p2.valid_from = Some(start_time + Duration::minutes(30));

        let profiles = vec![p1, p2];

        // At T=0, p1 is active
        let res1 = Executor::resolve(profiles.clone(), start_time, 10000.0, 10000.0, None).unwrap();
        assert_eq!(res1.unwrap().limit, 1000.0);

        // At T=29, p1 is still active
        let res2 = Executor::resolve(
            profiles.clone(),
            start_time + Duration::minutes(29),
            10000.0,
            10000.0,
            None,
        )
        .unwrap();
        assert_eq!(res2.unwrap().limit, 1000.0);

        // At T=30, p1 expires (inclusive) but p2 activates (inclusive). p2.id > p1.id, so p2 wins.
        let res3 = Executor::resolve(
            profiles.clone(),
            start_time + Duration::minutes(30),
            10000.0,
            10000.0,
            None,
        )
        .unwrap();
        assert_eq!(res3.unwrap().limit, 2000.0);

        // At T=31, p1 is definitely invalid
        let res4 = Executor::resolve(
            profiles.clone(),
            start_time + Duration::minutes(31),
            10000.0,
            10000.0,
            None,
        )
        .unwrap();
        assert_eq!(res4.unwrap().limit, 2000.0);
    }

    #[test]
    fn test_zero_power_suspend() {
        let current_time = Utc.with_ymd_and_hms(2023, 1, 1, 12, 0, 0).unwrap();
        let p1 = create_profile(
            1,
            1,
            ChargingProfilePurpose::TxProfile,
            ChargingProfileKind::Absolute,
            vec![ChargingSchedulePeriod {
                start_period: 0,
                limit: 0.0,
                number_phases: None,
            }],
            Some(current_time),
            ChargingRateUnit::Watts,
        );

        let result = Executor::resolve(vec![p1], current_time, 10000.0, 10000.0, None).unwrap();
        assert_eq!(
            result,
            Some(EnforcedLimit {
                limit: 0.0,
                unit: ChargingRateUnit::Watts,
                reason: EnforcementReason::ProfileLimit,
                should_suspend: true,
            })
        );
    }

    #[test]
    fn test_unit_mismatch_zero_logic() {
        let current_time = Utc.with_ymd_and_hms(2023, 1, 1, 12, 0, 0).unwrap();

        // TxProfile says STOP (0 Amps)
        let tx_profile = create_profile(
            1,
            1,
            ChargingProfilePurpose::TxProfile,
            ChargingProfileKind::Absolute,
            vec![ChargingSchedulePeriod {
                start_period: 0,
                limit: 0.0,
                number_phases: None,
            }],
            Some(current_time),
            ChargingRateUnit::Amperes,
        );

        // StationMax says 10kW
        let max_profile = create_profile(
            2,
            1,
            ChargingProfilePurpose::ChargingStationMaxProfile,
            ChargingProfileKind::Absolute,
            vec![ChargingSchedulePeriod {
                start_period: 0,
                limit: 10000.0,
                number_phases: None,
            }],
            Some(current_time),
            ChargingRateUnit::Watts,
        );

        let result = Executor::resolve(
            vec![tx_profile, max_profile],
            current_time,
            10000.0,
            10000.0,
            None,
        )
        .unwrap();

        // We EXPECT it to respect 0.0 Amps (Suspend)
        assert_eq!(
            result,
            Some(EnforcedLimit {
                limit: 0.0,
                unit: ChargingRateUnit::Amperes,
                reason: EnforcementReason::ProfileLimit,
                should_suspend: true,
            })
        );
    }

    #[test]
    fn test_tx_default_profile_fallback() {
        let current_time = Utc.with_ymd_and_hms(2023, 1, 1, 12, 0, 0).unwrap();

        // TxDefaultProfile
        let default_profile = create_profile(
            1,
            1,
            ChargingProfilePurpose::TxDefaultProfile,
            ChargingProfileKind::Absolute,
            vec![ChargingSchedulePeriod {
                start_period: 0,
                limit: 5000.0,
                number_phases: None,
            }],
            Some(current_time),
            ChargingRateUnit::Watts,
        );

        // Case 1: Only Default Profile
        let result = Executor::resolve(
            vec![default_profile.clone()],
            current_time,
            10000.0,
            10000.0,
            None,
        )
        .unwrap();
        assert_eq!(
            result,
            Some(EnforcedLimit {
                limit: 5000.0,
                unit: ChargingRateUnit::Watts,
                reason: EnforcementReason::ProfileLimit,
                should_suspend: false,
            })
        );

        // Case 2: TxProfile overrides Default Profile
        let tx_profile = create_profile(
            2,
            1,
            ChargingProfilePurpose::TxProfile,
            ChargingProfileKind::Absolute,
            vec![ChargingSchedulePeriod {
                start_period: 0,
                limit: 2000.0,
                number_phases: None,
            }],
            Some(current_time),
            ChargingRateUnit::Watts,
        );

        let result = Executor::resolve(
            vec![default_profile, tx_profile],
            current_time,
            10000.0,
            10000.0,
            None,
        )
        .unwrap();
        assert_eq!(
            result,
            Some(EnforcedLimit {
                limit: 2000.0,
                unit: ChargingRateUnit::Watts,
                reason: EnforcementReason::ProfileLimit,
                should_suspend: false,
            })
        );
    }
}
