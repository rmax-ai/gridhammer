#[cfg(test)]
mod tests {
    use crate::profiles::executor::{EnforcementReason, Executor};
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
            mut p1 in arb_charging_profile(),
            mut p2 in arb_charging_profile(),
            current_time in arb_datetime(),
            transaction_start in arb_datetime_option()
        ) {
             // p1: TxProfile, stack 1, limit 10
             p1.charging_profile_purpose = ChargingProfilePurpose::TxProfile;
             p1.charging_profile_kind = ChargingProfileKind::Absolute;
             p1.stack_level = 1;
             p1.valid_from = None;
             p1.valid_to = None;
             p1.charging_schedule.start_schedule = None;

             // p2: TxProfile, stack 2, limit 20
             p2.charging_profile_purpose = ChargingProfilePurpose::TxProfile;
             p2.charging_profile_kind = ChargingProfileKind::Absolute;
             p2.stack_level = 2;
             p2.valid_from = None;
             p2.valid_to = None;
             p2.charging_schedule.start_schedule = None;

             p2.charging_schedule.charging_rate_unit = ChargingRateUnit::Watts;
             p2.charging_schedule.charging_schedule_period = vec![ChargingSchedulePeriod {
                 start_period: 0,
                 limit: 20.0,
                 number_phases: None,
             }];

             let profiles = vec![p1, p2];
             let result = Executor::resolve(profiles, current_time, 10000.0, 10000.0, transaction_start);

             if let Ok(Some(enforced)) = result {
                 // p2 wins due to stack_level, so limit should be 20.0
                 prop_assert_eq!(enforced.limit, 20.0);
                 prop_assert_eq!(enforced.reason, EnforcementReason::ProfileLimit);
             } else {
                 prop_assert!(false, "Should have resolved a profile");
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
            mut default_profile in arb_charging_profile(),
            mut station_max in arb_charging_profile(),
            current_time in arb_datetime(),
            transaction_start in arb_datetime_option()
        ) {
            // Setup constant schedules
            default_profile.charging_profile_purpose = ChargingProfilePurpose::TxDefaultProfile;
            default_profile.charging_profile_kind = ChargingProfileKind::Absolute;
            default_profile.valid_from = None;
            default_profile.valid_to = None;
            default_profile.charging_schedule.start_schedule = None;
            default_profile.charging_schedule.charging_rate_unit = ChargingRateUnit::Watts;
             default_profile.charging_schedule.charging_schedule_period = vec![ChargingSchedulePeriod {
                 start_period: 0,
                 limit: 100.0,
                 number_phases: None,
             }];

            station_max.charging_profile_purpose = ChargingProfilePurpose::ChargingStationMaxProfile;
            station_max.charging_profile_kind = ChargingProfileKind::Absolute;
            station_max.valid_from = None;
            station_max.valid_to = None;
            station_max.charging_schedule.start_schedule = None;
            station_max.charging_schedule.charging_rate_unit = ChargingRateUnit::Watts;
             station_max.charging_schedule.charging_schedule_period = vec![ChargingSchedulePeriod {
                 start_period: 0,
                 limit: 50.0,
                 number_phases: None,
             }];

            // Case 1: Station Max (50) < Default (100)
            let profiles = vec![default_profile.clone(), station_max.clone()];
            let result = Executor::resolve(profiles, current_time, 10000.0, 10000.0, transaction_start);

            if let Ok(Some(enforced)) = result {
                prop_assert_eq!(enforced.limit, 50.0);
                prop_assert_eq!(enforced.reason, EnforcementReason::StationLimit);
            } else {
                 prop_assert!(false, "Should have resolved");
            }

            // Case 2: Station Max (50) > Default (20)
             let mut default_low = default_profile.clone();
             default_low.charging_schedule.charging_schedule_period = vec![ChargingSchedulePeriod {
                 start_period: 0,
                 limit: 20.0,
                 number_phases: None,
             }];

            let profiles_low = vec![default_low, station_max.clone()];
            let result_low = Executor::resolve(profiles_low, current_time, 10000.0, 10000.0, transaction_start);

            if let Ok(Some(enforced)) = result_low {
                prop_assert_eq!(enforced.limit, 20.0);
                prop_assert_eq!(enforced.reason, EnforcementReason::ProfileLimit);
            } else {
                 prop_assert!(false, "Should have resolved");
            }
        }
    }
}
