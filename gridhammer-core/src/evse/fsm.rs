use super::state::{EvseEvent, EvseState};

pub fn transition(state: EvseState, event: EvseEvent) -> Result<EvseState, &'static str> {
    match (state, event) {
        (EvseState::Available, EvseEvent::PlugIn) => Ok(EvseState::Preparing),
        (EvseState::Available, EvseEvent::Fault) => Ok(EvseState::Faulted),
        (EvseState::Preparing, EvseEvent::StartCharging) => Ok(EvseState::Charging),
        (EvseState::Preparing, EvseEvent::Unplug) => Ok(EvseState::Available),
        (EvseState::Preparing, EvseEvent::Fault) => Ok(EvseState::Faulted),
        (EvseState::Charging, EvseEvent::StopCharging) => Ok(EvseState::Available),
        (EvseState::Charging, EvseEvent::PauseEV) => Ok(EvseState::SuspendedEV),
        (EvseState::Charging, EvseEvent::PauseEVSE) => Ok(EvseState::SuspendedEVSE),
        (EvseState::Charging, EvseEvent::Fault) => Ok(EvseState::Faulted),
        (EvseState::SuspendedEV, EvseEvent::ResumeCharging) => Ok(EvseState::Charging),
        (EvseState::SuspendedEV, EvseEvent::StopCharging) => Ok(EvseState::Available),
        (EvseState::SuspendedEV, EvseEvent::Fault) => Ok(EvseState::Faulted),
        (EvseState::SuspendedEVSE, EvseEvent::ResumeCharging) => Ok(EvseState::Charging),
        (EvseState::SuspendedEVSE, EvseEvent::StopCharging) => Ok(EvseState::Available),
        (EvseState::SuspendedEVSE, EvseEvent::Fault) => Ok(EvseState::Faulted),
        (EvseState::Faulted, EvseEvent::ClearFault) => Ok(EvseState::Available),
        _ => Err("Invalid transition"),
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
}
