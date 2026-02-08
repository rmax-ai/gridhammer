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
        (Available, Reserve) => Ok(Reserved),
        (Available, StartCharging) => Ok(Occupied),
        (Available, Fault) => Ok(Faulted),
        (Available, SetUnavailable) => Ok(Unavailable),
        (Reserved, CancelReservation) => Ok(Available),
        (Reserved, StartCharging) => Ok(Occupied),
        (Reserved, Fault) => Ok(Faulted),
        (Occupied, StopCharging) => Ok(Available),
        (Occupied, Fault) => Ok(Faulted),
        (Unavailable, SetAvailable) => Ok(Available),
        (Unavailable, Fault) => Ok(Faulted),
        (Faulted, ClearFault) => Ok(Available),
        _ => Err(TransitionError::InvalidTransition { from: state, event }),
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
