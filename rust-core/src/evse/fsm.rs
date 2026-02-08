#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    PlugIn,
    PlugOut,
    RemoteStart,
    RemoteStop,
    FaultDetected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransitionError {
    IllegalTransition {
        from: super::state::EvseState,
        event: Event,
    },
}

pub fn transition(
    state: super::state::EvseState,
    event: Event,
) -> Result<super::state::EvseState, TransitionError> {
    use super::state::EvseState::*;
    match (state, event) {
        (Available, Event::PlugIn) => Ok(Occupied),
        (Occupied, Event::PlugOut) => Ok(Available),
        (Available, Event::RemoteStart) => Ok(Occupied),
        (Occupied, Event::RemoteStop) => Ok(Available),
        (Available, Event::FaultDetected) => Ok(Faulted),
        (Occupied, Event::FaultDetected) => Ok(Faulted),
        (Reserved, Event::FaultDetected) => Ok(Faulted),
        (Unavailable, Event::FaultDetected) => Ok(Faulted),
        (Faulted, Event::FaultDetected) => Ok(Faulted),
        _ => Err(TransitionError::IllegalTransition { from: state, event }),
    }
}

#[cfg(test)]
mod tests {
    use super::transition;
    use super::Event::*;
    use crate::evse::state::EvseState::*;

    #[test]
    fn test_legal_transitions() {
        assert_eq!(transition(Available, PlugIn), Ok(Occupied));
        assert_eq!(transition(Occupied, PlugOut), Ok(Available));
        assert_eq!(transition(Available, RemoteStart), Ok(Occupied));
        assert_eq!(transition(Occupied, RemoteStop), Ok(Available));
        assert_eq!(transition(Available, FaultDetected), Ok(Faulted));
        assert_eq!(transition(Occupied, FaultDetected), Ok(Faulted));
    }

    #[test]
    fn test_illegal_transitions() {
        assert!(transition(Faulted, PlugIn).is_err());
        assert!(transition(Faulted, PlugOut).is_err());
        assert!(transition(Faulted, RemoteStart).is_err());
    }
}
