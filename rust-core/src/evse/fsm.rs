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
    match input {
        EvseInput::PlugIn => match current {
            EvseState::Available => Ok(EvseState::Preparing),
            EvseState::Preparing => Err(TransitionError::InvalidTransition),
            EvseState::Charging => Err(TransitionError::InvalidTransition),
            EvseState::SuspendedEV => Err(TransitionError::InvalidTransition),
            EvseState::SuspendedEVSE => Err(TransitionError::InvalidTransition),
            EvseState::Faulted => Err(TransitionError::InvalidTransition),
        },
        EvseInput::Unplug => match current {
            EvseState::Preparing => Ok(EvseState::Available),
            EvseState::SuspendedEV => Ok(EvseState::Available),
            EvseState::SuspendedEVSE => Ok(EvseState::Available),
            EvseState::Charging => Ok(EvseState::Available),
            EvseState::Available => Err(TransitionError::InvalidTransition),
            EvseState::Faulted => Err(TransitionError::InvalidTransition),
        },
        EvseInput::RemoteStart => match current {
            EvseState::Preparing => Ok(EvseState::Charging),
            EvseState::Available => Err(TransitionError::InvalidTransition),
            EvseState::Charging => Err(TransitionError::InvalidTransition),
            EvseState::SuspendedEV => Err(TransitionError::InvalidTransition),
            EvseState::SuspendedEVSE => Err(TransitionError::InvalidTransition),
            EvseState::Faulted => Err(TransitionError::InvalidTransition),
        },
        EvseInput::RemoteStop => match current {
            EvseState::Charging => Ok(EvseState::Preparing),
            EvseState::SuspendedEV => Ok(EvseState::Preparing),
            EvseState::SuspendedEVSE => Ok(EvseState::Preparing),
            EvseState::Available => Err(TransitionError::InvalidTransition),
            EvseState::Preparing => Err(TransitionError::InvalidTransition),
            EvseState::Faulted => Err(TransitionError::InvalidTransition),
        },
        EvseInput::SuspendEV => match current {
            EvseState::Charging => Ok(EvseState::SuspendedEV),
            EvseState::Available => Err(TransitionError::InvalidTransition),
            EvseState::Preparing => Err(TransitionError::InvalidTransition),
            EvseState::SuspendedEV => Err(TransitionError::InvalidTransition),
            EvseState::SuspendedEVSE => Err(TransitionError::InvalidTransition),
            EvseState::Faulted => Err(TransitionError::InvalidTransition),
        },
        EvseInput::SuspendEVSE => match current {
            EvseState::Charging => Ok(EvseState::SuspendedEVSE),
            EvseState::Available => Err(TransitionError::InvalidTransition),
            EvseState::Preparing => Err(TransitionError::InvalidTransition),
            EvseState::SuspendedEV => Err(TransitionError::InvalidTransition),
            EvseState::SuspendedEVSE => Err(TransitionError::InvalidTransition),
            EvseState::Faulted => Err(TransitionError::InvalidTransition),
        },
        EvseInput::Resume => match current {
            EvseState::SuspendedEV => Ok(EvseState::Charging),
            EvseState::SuspendedEVSE => Ok(EvseState::Charging),
            EvseState::Available => Err(TransitionError::InvalidTransition),
            EvseState::Preparing => Err(TransitionError::InvalidTransition),
            EvseState::Charging => Err(TransitionError::InvalidTransition),
            EvseState::Faulted => Err(TransitionError::InvalidTransition),
        },
        EvseInput::FaultDetected => match current {
            EvseState::Available => Ok(EvseState::Faulted),
            EvseState::Preparing => Ok(EvseState::Faulted),
            EvseState::Charging => Ok(EvseState::Faulted),
            EvseState::SuspendedEV => Ok(EvseState::Faulted),
            EvseState::SuspendedEVSE => Ok(EvseState::Faulted),
            EvseState::Faulted => Ok(EvseState::Faulted),
        },
        EvseInput::ClearFault => match current {
            EvseState::Faulted => Ok(EvseState::Available),
            EvseState::Available => Err(TransitionError::InvalidTransition),
            EvseState::Preparing => Err(TransitionError::InvalidTransition),
            EvseState::Charging => Err(TransitionError::InvalidTransition),
            EvseState::SuspendedEV => Err(TransitionError::InvalidTransition),
            EvseState::SuspendedEVSE => Err(TransitionError::InvalidTransition),
        },
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

    #[test]
    fn test_charging_to_unplug() {
        assert_eq!(
            transition(EvseState::Charging, EvseInput::Unplug),
            Ok(EvseState::Available)
        );
    }

    #[test]
    fn test_suspended_to_remotestop() {
        assert_eq!(
            transition(EvseState::SuspendedEV, EvseInput::RemoteStop),
            Ok(EvseState::Preparing)
        );
        assert_eq!(
            transition(EvseState::SuspendedEVSE, EvseInput::RemoteStop),
            Ok(EvseState::Preparing)
        );
    }
}
