#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    PlugIn,
    PlugOut,
    RemoteStart,
    RemoteStop,
    FaultDetected,
    FaultCleared,
    Reserve,
    Unreserve,
    SetUnavailable,
    SetAvailable,
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
        // Available transitions
        (Available, Event::PlugIn) => Ok(Occupied),
        (Available, Event::PlugOut) => {
            Err(TransitionError::IllegalTransition { from: state, event })
        }
        (Available, Event::RemoteStart) => Ok(Occupied),
        (Available, Event::RemoteStop) => {
            Err(TransitionError::IllegalTransition { from: state, event })
        }
        (Available, Event::FaultDetected) => Ok(Faulted),
        (Available, Event::FaultCleared) => {
            Err(TransitionError::IllegalTransition { from: state, event })
        }
        (Available, Event::Reserve) => Ok(Reserved),
        (Available, Event::Unreserve) => {
            Err(TransitionError::IllegalTransition { from: state, event })
        }
        (Available, Event::SetUnavailable) => Ok(Unavailable),
        (Available, Event::SetAvailable) => {
            Err(TransitionError::IllegalTransition { from: state, event })
        }

        // Occupied transitions
        (Occupied, Event::PlugIn) => Err(TransitionError::IllegalTransition { from: state, event }),
        (Occupied, Event::PlugOut) => Ok(Available),
        (Occupied, Event::RemoteStart) => {
            Err(TransitionError::IllegalTransition { from: state, event })
        }
        (Occupied, Event::RemoteStop) => Ok(Available),
        (Occupied, Event::FaultDetected) => Ok(Faulted),
        (Occupied, Event::FaultCleared) => {
            Err(TransitionError::IllegalTransition { from: state, event })
        }
        (Occupied, Event::Reserve) => {
            Err(TransitionError::IllegalTransition { from: state, event })
        }
        (Occupied, Event::Unreserve) => {
            Err(TransitionError::IllegalTransition { from: state, event })
        }
        (Occupied, Event::SetUnavailable) => {
            Err(TransitionError::IllegalTransition { from: state, event })
        }
        (Occupied, Event::SetAvailable) => {
            Err(TransitionError::IllegalTransition { from: state, event })
        }

        // Reserved transitions
        (Reserved, Event::PlugIn) => Err(TransitionError::IllegalTransition { from: state, event }),
        (Reserved, Event::PlugOut) => {
            Err(TransitionError::IllegalTransition { from: state, event })
        }
        (Reserved, Event::RemoteStart) => {
            Err(TransitionError::IllegalTransition { from: state, event })
        }
        (Reserved, Event::RemoteStop) => {
            Err(TransitionError::IllegalTransition { from: state, event })
        }
        (Reserved, Event::FaultDetected) => Ok(Faulted),
        (Reserved, Event::FaultCleared) => {
            Err(TransitionError::IllegalTransition { from: state, event })
        }
        (Reserved, Event::Reserve) => {
            Err(TransitionError::IllegalTransition { from: state, event })
        }
        (Reserved, Event::Unreserve) => Ok(Available),
        (Reserved, Event::SetUnavailable) => {
            Err(TransitionError::IllegalTransition { from: state, event })
        }
        (Reserved, Event::SetAvailable) => {
            Err(TransitionError::IllegalTransition { from: state, event })
        }

        // Unavailable transitions
        (Unavailable, Event::PlugIn) => {
            Err(TransitionError::IllegalTransition { from: state, event })
        }
        (Unavailable, Event::PlugOut) => {
            Err(TransitionError::IllegalTransition { from: state, event })
        }
        (Unavailable, Event::RemoteStart) => {
            Err(TransitionError::IllegalTransition { from: state, event })
        }
        (Unavailable, Event::RemoteStop) => {
            Err(TransitionError::IllegalTransition { from: state, event })
        }
        (Unavailable, Event::FaultDetected) => Ok(Faulted),
        (Unavailable, Event::FaultCleared) => {
            Err(TransitionError::IllegalTransition { from: state, event })
        }
        (Unavailable, Event::Reserve) => {
            Err(TransitionError::IllegalTransition { from: state, event })
        }
        (Unavailable, Event::Unreserve) => {
            Err(TransitionError::IllegalTransition { from: state, event })
        }
        (Unavailable, Event::SetUnavailable) => {
            Err(TransitionError::IllegalTransition { from: state, event })
        }
        (Unavailable, Event::SetAvailable) => Ok(Available),

        // Faulted transitions
        (Faulted, Event::PlugIn) => Err(TransitionError::IllegalTransition { from: state, event }),
        (Faulted, Event::PlugOut) => Err(TransitionError::IllegalTransition { from: state, event }),
        (Faulted, Event::RemoteStart) => {
            Err(TransitionError::IllegalTransition { from: state, event })
        }
        (Faulted, Event::RemoteStop) => {
            Err(TransitionError::IllegalTransition { from: state, event })
        }
        (Faulted, Event::FaultDetected) => Ok(Faulted),
        (Faulted, Event::FaultCleared) => Ok(Available),
        (Faulted, Event::Reserve) => Err(TransitionError::IllegalTransition { from: state, event }),
        (Faulted, Event::Unreserve) => {
            Err(TransitionError::IllegalTransition { from: state, event })
        }
        (Faulted, Event::SetUnavailable) => {
            Err(TransitionError::IllegalTransition { from: state, event })
        }
        (Faulted, Event::SetAvailable) => {
            Err(TransitionError::IllegalTransition { from: state, event })
        }
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
        assert_eq!(transition(Reserved, FaultDetected), Ok(Faulted));
        assert_eq!(transition(Unavailable, FaultDetected), Ok(Faulted));
        assert_eq!(transition(Faulted, FaultDetected), Ok(Faulted));
        assert_eq!(transition(Available, Reserve), Ok(Reserved));
        assert_eq!(transition(Reserved, Unreserve), Ok(Available));
        assert_eq!(transition(Available, SetUnavailable), Ok(Unavailable));
        assert_eq!(transition(Unavailable, SetAvailable), Ok(Available));
        assert_eq!(transition(Faulted, FaultCleared), Ok(Available));
    }

    #[test]
    fn test_illegal_transitions() {
        // From Faulted
        assert!(transition(Faulted, PlugIn).is_err());
        assert!(transition(Faulted, PlugOut).is_err());
        assert!(transition(Faulted, RemoteStart).is_err());
        assert!(transition(Faulted, RemoteStop).is_err());
        assert!(transition(Faulted, Reserve).is_err());
        assert!(transition(Faulted, Unreserve).is_err());
        assert!(transition(Faulted, SetUnavailable).is_err());
        assert!(transition(Faulted, SetAvailable).is_err());

        // From Available
        assert!(transition(Available, PlugOut).is_err());
        assert!(transition(Available, RemoteStop).is_err());
        assert!(transition(Available, FaultCleared).is_err());
        assert!(transition(Available, Unreserve).is_err());
        assert!(transition(Available, SetAvailable).is_err());

        // From Occupied
        assert!(transition(Occupied, PlugIn).is_err());
        assert!(transition(Occupied, RemoteStart).is_err());
        assert!(transition(Occupied, FaultCleared).is_err());
        assert!(transition(Occupied, Reserve).is_err());
        assert!(transition(Occupied, Unreserve).is_err());
        assert!(transition(Occupied, SetUnavailable).is_err());
        assert!(transition(Occupied, SetAvailable).is_err());

        // From Reserved
        assert!(transition(Reserved, PlugIn).is_err());
        assert!(transition(Reserved, PlugOut).is_err());
        assert!(transition(Reserved, RemoteStart).is_err());
        assert!(transition(Reserved, RemoteStop).is_err());
        assert!(transition(Reserved, FaultCleared).is_err());
        assert!(transition(Reserved, Reserve).is_err());
        assert!(transition(Reserved, SetUnavailable).is_err());
        assert!(transition(Reserved, SetAvailable).is_err());

        // From Unavailable
        assert!(transition(Unavailable, PlugIn).is_err());
        assert!(transition(Unavailable, PlugOut).is_err());
        assert!(transition(Unavailable, RemoteStart).is_err());
        assert!(transition(Unavailable, RemoteStop).is_err());
        assert!(transition(Unavailable, FaultCleared).is_err());
        assert!(transition(Unavailable, Reserve).is_err());
        assert!(transition(Unavailable, Unreserve).is_err());
        assert!(transition(Unavailable, SetUnavailable).is_err());
    }
}
