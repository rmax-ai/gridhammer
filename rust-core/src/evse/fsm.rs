use crate::evse::state::EvseState;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvseEvent {
    Reserve,
    CancelReservation,
    StartCharging,
    StopCharging,
    SetUnavailable,
    SetAvailable,
    Fault,
    ClearFault,
}

#[derive(Debug, Clone)]
pub enum TransitionError {
    InvalidTransition { from: EvseState, event: EvseEvent },
}

pub fn try_transition(state: EvseState, event: EvseEvent) -> Result<EvseState, TransitionError> {
    use EvseEvent::*;
    use EvseState::*;

    match (state, event) {
        // Available state transitions
        (Available, Reserve) => Ok(Reserved),
        (Available, StartCharging) => Ok(Occupied),
        (Available, Fault) => Ok(Faulted),
        (Available, SetUnavailable) => Ok(Unavailable),
        (Available, CancelReservation) => {
            Err(TransitionError::InvalidTransition { from: state, event })
        }
        (Available, StopCharging) => Err(TransitionError::InvalidTransition { from: state, event }),
        (Available, SetAvailable) => Err(TransitionError::InvalidTransition { from: state, event }),
        (Available, ClearFault) => Err(TransitionError::InvalidTransition { from: state, event }),

        // Reserved state transitions
        (Reserved, CancelReservation) => Ok(Available),
        (Reserved, StartCharging) => Ok(Occupied),
        (Reserved, Fault) => Ok(Faulted),
        (Reserved, Reserve) => Err(TransitionError::InvalidTransition { from: state, event }),
        (Reserved, StopCharging) => Err(TransitionError::InvalidTransition { from: state, event }),
        (Reserved, SetUnavailable) => {
            Err(TransitionError::InvalidTransition { from: state, event })
        }
        (Reserved, SetAvailable) => Err(TransitionError::InvalidTransition { from: state, event }),
        (Reserved, ClearFault) => Err(TransitionError::InvalidTransition { from: state, event }),

        // Occupied state transitions
        (Occupied, StopCharging) => Ok(Available),
        (Occupied, Fault) => Ok(Faulted),
        (Occupied, Reserve) => Err(TransitionError::InvalidTransition { from: state, event }),
        (Occupied, CancelReservation) => {
            Err(TransitionError::InvalidTransition { from: state, event })
        }
        (Occupied, StartCharging) => Err(TransitionError::InvalidTransition { from: state, event }),
        (Occupied, SetUnavailable) => {
            Err(TransitionError::InvalidTransition { from: state, event })
        }
        (Occupied, SetAvailable) => Err(TransitionError::InvalidTransition { from: state, event }),
        (Occupied, ClearFault) => Err(TransitionError::InvalidTransition { from: state, event }),

        // Unavailable state transitions
        (Unavailable, SetAvailable) => Ok(Available),
        (Unavailable, Fault) => Ok(Faulted),
        (Unavailable, Reserve) => Err(TransitionError::InvalidTransition { from: state, event }),
        (Unavailable, CancelReservation) => {
            Err(TransitionError::InvalidTransition { from: state, event })
        }
        (Unavailable, StartCharging) => {
            Err(TransitionError::InvalidTransition { from: state, event })
        }
        (Unavailable, StopCharging) => {
            Err(TransitionError::InvalidTransition { from: state, event })
        }
        (Unavailable, SetUnavailable) => {
            Err(TransitionError::InvalidTransition { from: state, event })
        }
        (Unavailable, ClearFault) => Err(TransitionError::InvalidTransition { from: state, event }),

        // Faulted state transitions
        (Faulted, ClearFault) => Ok(Available),
        (Faulted, Reserve) => Err(TransitionError::InvalidTransition { from: state, event }),
        (Faulted, CancelReservation) => {
            Err(TransitionError::InvalidTransition { from: state, event })
        }
        (Faulted, StartCharging) => Err(TransitionError::InvalidTransition { from: state, event }),
        (Faulted, StopCharging) => Err(TransitionError::InvalidTransition { from: state, event }),
        (Faulted, SetUnavailable) => Err(TransitionError::InvalidTransition { from: state, event }),
        (Faulted, SetAvailable) => Err(TransitionError::InvalidTransition { from: state, event }),
        (Faulted, Fault) => Err(TransitionError::InvalidTransition { from: state, event }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_transitions() {
        assert_eq!(
            try_transition(EvseState::Available, EvseEvent::Reserve).unwrap(),
            EvseState::Reserved
        );
        assert_eq!(
            try_transition(EvseState::Available, EvseEvent::StartCharging).unwrap(),
            EvseState::Occupied
        );
        assert_eq!(
            try_transition(EvseState::Available, EvseEvent::Fault).unwrap(),
            EvseState::Faulted
        );
        assert_eq!(
            try_transition(EvseState::Available, EvseEvent::SetUnavailable).unwrap(),
            EvseState::Unavailable
        );
        assert_eq!(
            try_transition(EvseState::Reserved, EvseEvent::CancelReservation).unwrap(),
            EvseState::Available
        );
        assert_eq!(
            try_transition(EvseState::Reserved, EvseEvent::StartCharging).unwrap(),
            EvseState::Occupied
        );
        assert_eq!(
            try_transition(EvseState::Reserved, EvseEvent::Fault).unwrap(),
            EvseState::Faulted
        );
        assert_eq!(
            try_transition(EvseState::Occupied, EvseEvent::StopCharging).unwrap(),
            EvseState::Available
        );
        assert_eq!(
            try_transition(EvseState::Occupied, EvseEvent::Fault).unwrap(),
            EvseState::Faulted
        );
        assert_eq!(
            try_transition(EvseState::Unavailable, EvseEvent::SetAvailable).unwrap(),
            EvseState::Available
        );
        assert_eq!(
            try_transition(EvseState::Unavailable, EvseEvent::Fault).unwrap(),
            EvseState::Faulted
        );
        assert_eq!(
            try_transition(EvseState::Faulted, EvseEvent::ClearFault).unwrap(),
            EvseState::Available
        );
    }

    #[test]
    fn test_invalid_transitions() {
        assert!(try_transition(EvseState::Available, EvseEvent::StopCharging).is_err());
        assert!(try_transition(EvseState::Available, EvseEvent::CancelReservation).is_err());
        assert!(try_transition(EvseState::Available, EvseEvent::SetAvailable).is_err());
        assert!(try_transition(EvseState::Available, EvseEvent::ClearFault).is_err());
        assert!(try_transition(EvseState::Reserved, EvseEvent::Reserve).is_err());
        assert!(try_transition(EvseState::Reserved, EvseEvent::StopCharging).is_err());
        assert!(try_transition(EvseState::Reserved, EvseEvent::SetUnavailable).is_err());
        assert!(try_transition(EvseState::Reserved, EvseEvent::SetAvailable).is_err());
        assert!(try_transition(EvseState::Reserved, EvseEvent::ClearFault).is_err());
        assert!(try_transition(EvseState::Occupied, EvseEvent::Reserve).is_err());
        assert!(try_transition(EvseState::Occupied, EvseEvent::CancelReservation).is_err());
        assert!(try_transition(EvseState::Occupied, EvseEvent::StartCharging).is_err());
        assert!(try_transition(EvseState::Occupied, EvseEvent::SetUnavailable).is_err());
        assert!(try_transition(EvseState::Occupied, EvseEvent::SetAvailable).is_err());
        assert!(try_transition(EvseState::Occupied, EvseEvent::ClearFault).is_err());
        assert!(try_transition(EvseState::Unavailable, EvseEvent::Reserve).is_err());
        assert!(try_transition(EvseState::Unavailable, EvseEvent::CancelReservation).is_err());
        assert!(try_transition(EvseState::Unavailable, EvseEvent::StartCharging).is_err());
        assert!(try_transition(EvseState::Unavailable, EvseEvent::StopCharging).is_err());
        assert!(try_transition(EvseState::Unavailable, EvseEvent::SetUnavailable).is_err());
        assert!(try_transition(EvseState::Unavailable, EvseEvent::ClearFault).is_err());
        assert!(try_transition(EvseState::Faulted, EvseEvent::Reserve).is_err());
        assert!(try_transition(EvseState::Faulted, EvseEvent::CancelReservation).is_err());
        assert!(try_transition(EvseState::Faulted, EvseEvent::StartCharging).is_err());
        assert!(try_transition(EvseState::Faulted, EvseEvent::StopCharging).is_err());
        assert!(try_transition(EvseState::Faulted, EvseEvent::SetUnavailable).is_err());
        assert!(try_transition(EvseState::Faulted, EvseEvent::SetAvailable).is_err());
        assert!(try_transition(EvseState::Faulted, EvseEvent::Fault).is_err());
    }
}
