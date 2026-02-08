use super::state::{EvseEvent, EvseState};

pub fn transition(state: EvseState, event: EvseEvent) -> Result<EvseState, &'static str> {
    match state {
        EvseState::Available => match event {
            EvseEvent::PlugIn => Ok(EvseState::Preparing),
            EvseEvent::Fault => Ok(EvseState::Faulted),
            EvseEvent::Unplug
            | EvseEvent::StartCharging
            | EvseEvent::StopCharging
            | EvseEvent::PauseEV
            | EvseEvent::PauseEVSE
            | EvseEvent::ResumeCharging
            | EvseEvent::ClearFault => Err("Invalid transition from Available"),
        },
        EvseState::Preparing => match event {
            EvseEvent::StartCharging => Ok(EvseState::Charging),
            EvseEvent::Unplug => Ok(EvseState::Available),
            EvseEvent::Fault => Ok(EvseState::Faulted),
            EvseEvent::PlugIn
            | EvseEvent::StopCharging
            | EvseEvent::PauseEV
            | EvseEvent::PauseEVSE
            | EvseEvent::ResumeCharging
            | EvseEvent::ClearFault => Err("Invalid transition from Preparing"),
        },
        EvseState::Charging => match event {
            EvseEvent::StopCharging => Ok(EvseState::Available),
            EvseEvent::PauseEV => Ok(EvseState::SuspendedEV),
            EvseEvent::PauseEVSE => Ok(EvseState::SuspendedEVSE),
            EvseEvent::Fault => Ok(EvseState::Faulted),
            EvseEvent::PlugIn
            | EvseEvent::Unplug
            | EvseEvent::StartCharging
            | EvseEvent::ResumeCharging
            | EvseEvent::ClearFault => Err("Invalid transition from Charging"),
        },
        EvseState::SuspendedEV => match event {
            EvseEvent::ResumeCharging => Ok(EvseState::Charging),
            EvseEvent::StopCharging => Ok(EvseState::Available),
            EvseEvent::Fault => Ok(EvseState::Faulted),
            EvseEvent::PlugIn
            | EvseEvent::Unplug
            | EvseEvent::StartCharging
            | EvseEvent::PauseEV
            | EvseEvent::PauseEVSE
            | EvseEvent::ClearFault => Err("Invalid transition from SuspendedEV"),
        },
        EvseState::SuspendedEVSE => match event {
            EvseEvent::ResumeCharging => Ok(EvseState::Charging),
            EvseEvent::StopCharging => Ok(EvseState::Available),
            EvseEvent::Fault => Ok(EvseState::Faulted),
            EvseEvent::PlugIn
            | EvseEvent::Unplug
            | EvseEvent::StartCharging
            | EvseEvent::PauseEV
            | EvseEvent::PauseEVSE
            | EvseEvent::ClearFault => Err("Invalid transition from SuspendedEVSE"),
        },
        EvseState::Faulted => match event {
            EvseEvent::ClearFault => Ok(EvseState::Available),
            EvseEvent::PlugIn
            | EvseEvent::Unplug
            | EvseEvent::StartCharging
            | EvseEvent::StopCharging
            | EvseEvent::PauseEV
            | EvseEvent::PauseEVSE
            | EvseEvent::ResumeCharging
            | EvseEvent::Fault => Err("Invalid transition from Faulted"),
        },
        EvseState::Unavailable => match event {
            EvseEvent::PlugIn
            | EvseEvent::Unplug
            | EvseEvent::StartCharging
            | EvseEvent::StopCharging
            | EvseEvent::PauseEV
            | EvseEvent::PauseEVSE
            | EvseEvent::ResumeCharging
            | EvseEvent::Fault
            | EvseEvent::ClearFault => Err("Invalid transition from Unavailable"),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_available_transitions() {
        assert_eq!(
            transition(EvseState::Available, EvseEvent::PlugIn),
            Ok(EvseState::Preparing)
        );
        assert_eq!(
            transition(EvseState::Available, EvseEvent::Fault),
            Ok(EvseState::Faulted)
        );
        assert!(transition(EvseState::Available, EvseEvent::StartCharging).is_err());
    }

    #[test]
    fn test_preparing_transitions() {
        assert_eq!(
            transition(EvseState::Preparing, EvseEvent::StartCharging),
            Ok(EvseState::Charging)
        );
        assert_eq!(
            transition(EvseState::Preparing, EvseEvent::Unplug),
            Ok(EvseState::Available)
        );
        assert_eq!(
            transition(EvseState::Preparing, EvseEvent::Fault),
            Ok(EvseState::Faulted)
        );
        assert!(transition(EvseState::Preparing, EvseEvent::StopCharging).is_err());
    }

    #[test]
    fn test_charging_transitions() {
        assert_eq!(
            transition(EvseState::Charging, EvseEvent::StopCharging),
            Ok(EvseState::Available)
        );
        assert_eq!(
            transition(EvseState::Charging, EvseEvent::PauseEV),
            Ok(EvseState::SuspendedEV)
        );
        assert_eq!(
            transition(EvseState::Charging, EvseEvent::PauseEVSE),
            Ok(EvseState::SuspendedEVSE)
        );
        assert_eq!(
            transition(EvseState::Charging, EvseEvent::Fault),
            Ok(EvseState::Faulted)
        );
        assert!(transition(EvseState::Charging, EvseEvent::PlugIn).is_err());
    }

    #[test]
    fn test_suspended_ev_transitions() {
        assert_eq!(
            transition(EvseState::SuspendedEV, EvseEvent::ResumeCharging),
            Ok(EvseState::Charging)
        );
        assert_eq!(
            transition(EvseState::SuspendedEV, EvseEvent::StopCharging),
            Ok(EvseState::Available)
        );
        assert_eq!(
            transition(EvseState::SuspendedEV, EvseEvent::Fault),
            Ok(EvseState::Faulted)
        );
        assert!(transition(EvseState::SuspendedEV, EvseEvent::PauseEV).is_err());
    }

    #[test]
    fn test_suspended_evse_transitions() {
        assert_eq!(
            transition(EvseState::SuspendedEVSE, EvseEvent::ResumeCharging),
            Ok(EvseState::Charging)
        );
        assert_eq!(
            transition(EvseState::SuspendedEVSE, EvseEvent::StopCharging),
            Ok(EvseState::Available)
        );
        assert_eq!(
            transition(EvseState::SuspendedEVSE, EvseEvent::Fault),
            Ok(EvseState::Faulted)
        );
        assert!(transition(EvseState::SuspendedEVSE, EvseEvent::PauseEVSE).is_err());
    }

    #[test]
    fn test_faulted_transitions() {
        assert_eq!(
            transition(EvseState::Faulted, EvseEvent::ClearFault),
            Ok(EvseState::Available)
        );
        assert!(transition(EvseState::Faulted, EvseEvent::PlugIn).is_err());
    }

    #[test]
    fn test_unavailable_transitions() {
        // Unavailable should reject all events currently
        assert!(transition(EvseState::Unavailable, EvseEvent::PlugIn).is_err());
        assert!(transition(EvseState::Unavailable, EvseEvent::Fault).is_err());
        assert!(transition(EvseState::Unavailable, EvseEvent::ClearFault).is_err());
    }
}
