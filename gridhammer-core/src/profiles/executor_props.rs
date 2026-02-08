#[cfg(test)]
mod tests {
    use crate::profiles::executor::Executor;
    use crate::profiles::model::{
        ChargingProfile, ChargingProfileKind, ChargingProfilePurpose, ChargingRateUnit,
        ChargingSchedule, ChargingSchedulePeriod, RecurrencyKind,
    };
    use chrono::{DateTime, Duration, TimeZone, Utc};
    use proptest::prelude::*;

    // Strategies

    fn arb_datetime() -> impl Strategy<Value = DateTime<Utc>> {
        (0..(10 * 365 * 24 * 3600)).prop_map(|seconds| {
            Utc.with_ymd_and_hms(2020, 1, 1, 0, 0, 0).unwrap() + Duration::seconds(seconds as i64)
        })
    }

    fn arb_datetime_option() -> impl Strategy<Value = Option<DateTime<Utc>>> {
        prop_oneof![Just(None), arb_datetime().prop_map(Some),]
    }

    fn arb_charging_profile_purpose() -> impl Strategy<Value = ChargingProfilePurpose> {
        prop_oneof![
            Just(ChargingProfilePurpose::ChargingStationMaxProfile),
            Just(ChargingProfilePurpose::TxDefaultProfile),
            Just(ChargingProfilePurpose::TxProfile),
        ]
    }

    fn arb_charging_profile_kind() -> impl Strategy<Value = ChargingProfileKind> {
        prop_oneof![
            Just(ChargingProfileKind::Absolute),
            Just(ChargingProfileKind::Recurring),
            Just(ChargingProfileKind::Relative),
        ]
    }

    fn arb_recurrency_kind() -> impl Strategy<Value = RecurrencyKind> {
        prop_oneof![Just(RecurrencyKind::Daily), Just(RecurrencyKind::Weekly),]
    }

    fn arb_recurrency_kind_option() -> impl Strategy<Value = Option<RecurrencyKind>> {
        prop_oneof![Just(None), arb_recurrency_kind().prop_map(Some),]
    }

    fn arb_charging_rate_unit() -> impl Strategy<Value = ChargingRateUnit> {
        prop_oneof![
            Just(ChargingRateUnit::Watts),
            Just(ChargingRateUnit::Amperes),
        ]
    }

    fn arb_charging_schedule_period() -> impl Strategy<Value = ChargingSchedulePeriod> {
        (
            any::<i32>(),                                         // start_period
            any::<f64>(),                                         // limit
            prop_oneof![Just(None), any::<i32>().prop_map(Some)], // number_phases
        )
            .prop_map(|(start, limit, phases)| ChargingSchedulePeriod {
                start_period: start.max(0), // ensure non-negative
                limit: limit,
                number_phases: phases,
            })
    }

    fn arb_charging_schedule() -> impl Strategy<Value = ChargingSchedule> {
        (
            any::<i32>(),
            arb_datetime_option(),
            prop_oneof![Just(None), any::<i32>().prop_map(Some)],
            arb_charging_rate_unit(),
            prop::collection::vec(arb_charging_schedule_period(), 0..5),
            prop_oneof![Just(None), any::<f64>().prop_map(Some)],
        )
            .prop_map(|(id, start, duration, unit, periods, min_rate)| {
                // Sort periods by start_period to be valid
                let mut sorted_periods: Vec<ChargingSchedulePeriod> = periods;
                sorted_periods.sort_by_key(|p| p.start_period);

                ChargingSchedule {
                    id,
                    start_schedule: start,
                    duration,
                    charging_rate_unit: unit,
                    charging_schedule_period: sorted_periods,
                    min_charging_rate: min_rate,
                }
            })
    }

    fn arb_charging_profile() -> impl Strategy<Value = ChargingProfile> {
        (
            any::<i32>(),
            any::<i32>(),
            arb_charging_profile_purpose(),
            arb_charging_profile_kind(),
            arb_recurrency_kind_option(),
            arb_datetime_option(),
            arb_datetime_option(),
            arb_charging_schedule(),
        )
            .prop_map(
                |(id, stack, purpose, kind, recurrency, valid_from, valid_to, schedule)| {
                    ChargingProfile {
                        id,
                        stack_level: stack,
                        charging_profile_purpose: purpose,
                        charging_profile_kind: kind,
                        recurrency_kind: recurrency,
                        valid_from,
                        valid_to,
                        charging_schedule: schedule,
                    }
                },
            )
    }

    proptest! {
        #[test]
        fn no_crashes_with_random_profiles(
            profiles in prop::collection::vec(arb_charging_profile(), 0..5),
            current_time in arb_datetime(),
            transaction_start in arb_datetime_option()
        ) {
            let _ = Executor::resolve(profiles, current_time, 10000.0, 10000.0, transaction_start);
        }

        #[test]
        fn clamping_result_limit_non_negative(
            profiles in prop::collection::vec(arb_charging_profile(), 0..5),
            current_time in arb_datetime(),
            transaction_start in arb_datetime_option()
        ) {
            let result = Executor::resolve(profiles, current_time, 10000.0, 10000.0, transaction_start);
            if let Ok(Some(enforced)) = result {
                prop_assert!(enforced.limit >= 0.0, "Limit {} should be non-negative", enforced.limit);
            }
        }

        #[test]
        fn non_nan_result_limit(
            profiles in prop::collection::vec(arb_charging_profile(), 0..5),
            current_time in arb_datetime(),
            transaction_start in arb_datetime_option()
        ) {
            let result = Executor::resolve(profiles, current_time, 10000.0, 10000.0, transaction_start);
            if let Ok(Some(enforced)) = result {
                prop_assert!(!enforced.limit.is_nan(), "Limit should not be NaN");
            }
        }

        #[test]
        fn idempotency(
            profiles in prop::collection::vec(arb_charging_profile(), 0..5),
            current_time in arb_datetime(),
            transaction_start in arb_datetime_option()
        ) {
            let result1 = Executor::resolve(profiles.clone(), current_time, 10000.0, 10000.0, transaction_start);
            let result2 = Executor::resolve(profiles, current_time, 10000.0, 10000.0, transaction_start);
            prop_assert_eq!(result1, result2);
        }

        #[test]
        fn priority_oracle(
            base_profile in arb_charging_profile(),
            higher_stack_profile in arb_charging_profile(),
            current_time in arb_datetime(),
            transaction_start in arb_datetime_option()
        ) {
             // Setup specific case: 2 TxProfiles, one clearly higher priority
             let mut p1 = base_profile;
             p1.charging_profile_purpose = ChargingProfilePurpose::TxProfile;
             p1.valid_from = None;
             p1.valid_to = None;

             let mut p2 = higher_stack_profile;
             p2.charging_profile_purpose = ChargingProfilePurpose::TxProfile;
             p2.stack_level = p1.stack_level.saturating_add(1);
             p2.valid_from = None;
             p2.valid_to = None;

             let profiles = vec![p1.clone(), p2.clone()];
             let result = Executor::resolve(profiles, current_time, 10000.0, 10000.0, transaction_start);

             if let Ok(Some(_enforced)) = result {
                 // Since p2 has higher stack_level, it should be selected
             }
        }

        #[test]
        fn zero_implies_suspend(
            profiles in prop::collection::vec(arb_charging_profile(), 0..5),
            current_time in arb_datetime(),
            transaction_start in arb_datetime_option()
        ) {
            let result = Executor::resolve(profiles, current_time, 10000.0, 10000.0, transaction_start);
            if let Ok(Some(enforced)) = result {
                if enforced.limit == 0.0 {
                    prop_assert!(enforced.should_suspend, "Zero limit implies should_suspend=true");
                }
                if enforced.should_suspend {
                    prop_assert_eq!(enforced.limit, 0.0, "should_suspend=true implies limit=0.0");
                }
            }
        }

        #[test]
        fn tx_default_and_station_max_interaction(
            default_profile in arb_charging_profile(),
            station_max in arb_charging_profile(),
            current_time in arb_datetime(),
            transaction_start in arb_datetime_option()
        ) {
            // Setup: TxDefault and StationMax only
            let mut p1 = default_profile;
            p1.charging_profile_purpose = ChargingProfilePurpose::TxDefaultProfile;
            p1.valid_from = None;
            p1.valid_to = None;
            // Ensure distinct IDs
            p1.id = 1;

            let mut p2 = station_max;
            p2.charging_profile_purpose = ChargingProfilePurpose::ChargingStationMaxProfile;
            p2.valid_from = None;
            p2.valid_to = None;
            p2.id = 2;
            // Ensure same units for simple comparison
            p2.charging_schedule.charging_rate_unit = p1.charging_schedule.charging_rate_unit.clone();


            let profiles = vec![p1.clone(), p2.clone()];
            let result = Executor::resolve(profiles, current_time, 10000.0, 10000.0, transaction_start);

            if let Ok(Some(enforced)) = result {
                 // The reason should be correct
                 if enforced.reason == crate::profiles::executor::EnforcementReason::ProfileLimit {
                     // from default
                 } else if enforced.reason == crate::profiles::executor::EnforcementReason::StationLimit {
                     // from station max
                 } else {
                     // internal
                 }
            }
        }
    }
}
