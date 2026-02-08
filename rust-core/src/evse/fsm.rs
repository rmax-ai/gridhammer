use super::state::EvseState;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvseInput {
    PlugIn,
    Unplug,
    RemoteStart,
    RemoteStop,
    SuspendEV,
    SuspendEVSE,
    Resume,
    FaultDetected,
    ClearFault,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransitionError {
    InvalidTransition,
}

pub fn transition(current: EvseState, input: EvseInput) -> Result<EvseState, TransitionError> {
    match (current, input) {
        (EvseState::Available, EvseInput::PlugIn) => Ok(EvseState::Preparing),
        (EvseState::Preparing, EvseInput::Unplug) => Ok(EvseState::Available),
        (EvseState::Preparing, EvseInput::RemoteStart) => Ok(EvseState::Charging),
        (EvseState::Charging, EvseInput::RemoteStop) => Ok(EvseState::Preparing),
        (EvseState::Charging, EvseInput::SuspendEV) => Ok(EvseState::SuspendedEV),
        (EvseState::Charging, EvseInput::SuspendEVSE) => Ok(EvseState::SuspendedEVSE),
        (EvseState::SuspendedEV, EvseInput::Resume) => Ok(EvseState::Charging),
        (EvseState::SuspendedEV, EvseInput::Unplug) => Ok(EvseState::Available),
        (EvseState::SuspendedEVSE, EvseInput::Resume) => Ok(EvseState::Charging),
        (EvseState::SuspendedEVSE, EvseInput::Unplug) => Ok(EvseState::Available),
        (_, EvseInput::FaultDetected) => Ok(EvseState::Faulted),
        (EvseState::Faulted, EvseInput::ClearFault) => Ok(EvseState::Available),
        _ => Err(TransitionError::InvalidTransition),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_available_to_preparing() {
        assert_eq!(
            transition(EvseState::Available, EvseInput::PlugIn),
            Ok(EvseState::Preparing)
        );
    }

    #[test]
    fn test_preparing_to_charging() {
        assert_eq!(
            transition(EvseState::Preparing, EvseInput::RemoteStart),
            Ok(EvseState::Charging)
        );
    }

    #[test]
    fn test_charging_to_suspended_ev() {
        assert_eq!(
            transition(EvseState::Charging, EvseInput::SuspendEV),
            Ok(EvseState::SuspendedEV)
        );
    }

    #[test]
    fn test_any_to_faulted() {
        assert_eq!(
            transition(EvseState::Available, EvseInput::FaultDetected),
            Ok(EvseState::Faulted)
        );
        assert_eq!(
            transition(EvseState::Charging, EvseInput::FaultDetected),
            Ok(EvseState::Faulted)
        );
    }

    #[test]
    fn test_faulted_to_available() {
        assert_eq!(
            transition(EvseState::Faulted, EvseInput::ClearFault),
            Ok(EvseState::Available)
        );
    }

    #[test]
    fn test_invalid_transition() {
        assert_eq!(
            transition(EvseState::Available, EvseInput::Unplug),
            Err(TransitionError::InvalidTransition)
        );
    }
}
