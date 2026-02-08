use crate::evse::state::EvseState;

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum Event {
    PlugIn,
    Unplug,
    StartCharging,
    StopCharging,
    SuspendByEV,
    SuspendByEVSE,
    ClearSuspension,
    FaultDetected,
    FaultCleared,
}

pub fn transition(state: EvseState, event: Event) -> Result<EvseState, String> {
    match (state, event) {
        (EvseState::Available, Event::PlugIn) => Ok(EvseState::Preparing),
        (EvseState::Preparing, Event::Unplug) => Ok(EvseState::Available),
        (EvseState::Preparing, Event::StartCharging) => Ok(EvseState::Charging),
        (EvseState::Charging, Event::Unplug) => Ok(EvseState::Available),
        (EvseState::Charging, Event::StopCharging) => Ok(EvseState::Available),
        (EvseState::Charging, Event::SuspendByEV) => Ok(EvseState::SuspendedEV),
        (EvseState::Charging, Event::SuspendByEVSE) => Ok(EvseState::SuspendedEVSE),
        (EvseState::SuspendedEV, Event::ClearSuspension) => Ok(EvseState::Charging),
        (EvseState::SuspendedEVSE, Event::ClearSuspension) => Ok(EvseState::Charging),
        (EvseState::SuspendedEV, Event::Unplug) => Ok(EvseState::Available),
        (EvseState::SuspendedEVSE, Event::Unplug) => Ok(EvseState::Available),
        (_, Event::FaultDetected) => Ok(EvseState::Faulted),
        (EvseState::Faulted, Event::FaultCleared) => Ok(EvseState::Available),
        _ => Err(format!(
            "Invalid transition from {:?} on {:?}",
            state, event
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_legal_transitions() {
        assert_eq!(
            transition(EvseState::Available, Event::PlugIn),
            Ok(EvseState::Preparing)
        );
        assert_eq!(
            transition(EvseState::Preparing, Event::StartCharging),
            Ok(EvseState::Charging)
        );
        assert_eq!(
            transition(EvseState::Charging, Event::SuspendByEV),
            Ok(EvseState::SuspendedEV)
        );
        assert_eq!(
            transition(EvseState::SuspendedEV, Event::ClearSuspension),
            Ok(EvseState::Charging)
        );
        assert_eq!(
            transition(EvseState::Charging, Event::Unplug),
            Ok(EvseState::Available)
        );
        assert_eq!(
            transition(EvseState::Faulted, Event::FaultCleared),
            Ok(EvseState::Available)
        );
        assert_eq!(
            transition(EvseState::Available, Event::FaultDetected),
            Ok(EvseState::Faulted)
        );
    }

    #[test]
    fn test_illegal_transitions() {
        assert!(transition(EvseState::Available, Event::StartCharging).is_err());
        assert!(transition(EvseState::Preparing, Event::SuspendByEV).is_err());
        assert!(transition(EvseState::Charging, Event::PlugIn).is_err());
        assert!(transition(EvseState::SuspendedEV, Event::StartCharging).is_err());
        assert!(transition(EvseState::Faulted, Event::PlugIn).is_err());
    }
}
