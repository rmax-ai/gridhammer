use super::state::{EvseEvent, EvseState};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransitionError {
    InvalidTransition { from: EvseState, event: EvseEvent },
}

impl fmt::Display for TransitionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidTransition { from, event } => {
                write!(f, "Invalid transition from {:?} via {:?}", from, event)
            }
        }
    }
}

impl std::error::Error for TransitionError {}

/// Pure transition function for EVSE state machine.
///
/// Follows strict precedence and logical transitions.
/// Note: Availability changes during active transactions are handled by the application layer queuing
/// the state change. This FSM represents the *current* effective state.
pub fn transition(state: EvseState, event: EvseEvent) -> Result<EvseState, TransitionError> {
    match state {
        EvseState::Available => match event {
            EvseEvent::PlugIn => Ok(EvseState::Preparing),
            EvseEvent::Reserve => Ok(EvseState::Reserved),
            EvseEvent::SetUnavailable => Ok(EvseState::Unavailable),
            EvseEvent::Fault => Ok(EvseState::Faulted),
            // Explicitly list illegal events for exhaustiveness
            EvseEvent::Unplug
            | EvseEvent::StartCharging
            | EvseEvent::StopCharging
            | EvseEvent::PauseEV
            | EvseEvent::PauseEVSE
            | EvseEvent::ResumeCharging
            | EvseEvent::ClearFault
            | EvseEvent::SetAvailable
            | EvseEvent::CancelReservation => {
                Err(TransitionError::InvalidTransition { from: state, event })
            }
        },
        EvseState::Reserved => match event {
            EvseEvent::PlugIn => Ok(EvseState::Preparing),
            EvseEvent::CancelReservation => Ok(EvseState::Available),
            EvseEvent::SetUnavailable => Ok(EvseState::Unavailable),
            EvseEvent::Fault => Ok(EvseState::Faulted),
            // Explicitly list illegal events
            EvseEvent::Unplug
            | EvseEvent::StartCharging
            | EvseEvent::StopCharging
            | EvseEvent::PauseEV
            | EvseEvent::PauseEVSE
            | EvseEvent::ResumeCharging
            | EvseEvent::ClearFault
            | EvseEvent::SetAvailable
            | EvseEvent::Reserve => Err(TransitionError::InvalidTransition { from: state, event }),
        },
        EvseState::Preparing => match event {
            EvseEvent::StartCharging => Ok(EvseState::Charging),
            EvseEvent::Unplug => Ok(EvseState::Available),
            EvseEvent::Fault => Ok(EvseState::Faulted),
            // Explicitly list illegal events
            EvseEvent::PlugIn
            | EvseEvent::StopCharging
            | EvseEvent::PauseEV
            | EvseEvent::PauseEVSE
            | EvseEvent::ResumeCharging
            | EvseEvent::ClearFault
            | EvseEvent::SetUnavailable
            | EvseEvent::SetAvailable
            | EvseEvent::Reserve
            | EvseEvent::CancelReservation => {
                Err(TransitionError::InvalidTransition { from: state, event })
            }
        },
        EvseState::Charging => match event {
            EvseEvent::StopCharging => Ok(EvseState::Available),
            EvseEvent::Unplug => Ok(EvseState::Available),
            EvseEvent::PauseEV => Ok(EvseState::SuspendedEV),
            EvseEvent::PauseEVSE => Ok(EvseState::SuspendedEVSE),
            EvseEvent::Fault => Ok(EvseState::Faulted),
            // Explicitly list illegal events
            EvseEvent::PlugIn
            | EvseEvent::StartCharging
            | EvseEvent::ResumeCharging
            | EvseEvent::ClearFault
            | EvseEvent::SetUnavailable
            | EvseEvent::SetAvailable
            | EvseEvent::Reserve
            | EvseEvent::CancelReservation => {
                Err(TransitionError::InvalidTransition { from: state, event })
            }
        },
        EvseState::SuspendedEV => match event {
            EvseEvent::ResumeCharging => Ok(EvseState::Charging),
            EvseEvent::StopCharging => Ok(EvseState::Available),
            EvseEvent::Unplug => Ok(EvseState::Available),
            EvseEvent::PauseEVSE => Ok(EvseState::SuspendedEVSE),
            EvseEvent::Fault => Ok(EvseState::Faulted),
            // Explicitly list illegal events
            EvseEvent::PlugIn
            | EvseEvent::StartCharging
            | EvseEvent::PauseEV
            | EvseEvent::ClearFault
            | EvseEvent::SetUnavailable
            | EvseEvent::SetAvailable
            | EvseEvent::Reserve
            | EvseEvent::CancelReservation => {
                Err(TransitionError::InvalidTransition { from: state, event })
            }
        },
        EvseState::SuspendedEVSE => match event {
            EvseEvent::ResumeCharging => Ok(EvseState::Charging),
            EvseEvent::StopCharging => Ok(EvseState::Available),
            EvseEvent::Unplug => Ok(EvseState::Available),
            EvseEvent::Fault => Ok(EvseState::Faulted),
            // Explicitly list illegal events
            EvseEvent::PlugIn
            | EvseEvent::StartCharging
            | EvseEvent::PauseEV
            | EvseEvent::PauseEVSE
            | EvseEvent::ClearFault
            | EvseEvent::SetUnavailable
            | EvseEvent::SetAvailable
            | EvseEvent::Reserve
            | EvseEvent::CancelReservation => {
                Err(TransitionError::InvalidTransition { from: state, event })
            }
        },
        EvseState::Faulted => match event {
            EvseEvent::ClearFault => Ok(EvseState::Available),
            // Explicitly list illegal events - must clear fault first
            EvseEvent::PlugIn
            | EvseEvent::Unplug
            | EvseEvent::StartCharging
            | EvseEvent::StopCharging
            | EvseEvent::PauseEV
            | EvseEvent::PauseEVSE
            | EvseEvent::ResumeCharging
            | EvseEvent::Fault
            | EvseEvent::SetUnavailable
            | EvseEvent::SetAvailable
            | EvseEvent::Reserve
            | EvseEvent::CancelReservation => {
                Err(TransitionError::InvalidTransition { from: state, event })
            }
        },
        EvseState::Unavailable => match event {
            EvseEvent::SetAvailable => Ok(EvseState::Available),
            EvseEvent::Fault => Ok(EvseState::Faulted),
            // Explicitly list illegal events
            EvseEvent::PlugIn
            | EvseEvent::Unplug
            | EvseEvent::StartCharging
            | EvseEvent::StopCharging
            | EvseEvent::PauseEV
            | EvseEvent::PauseEVSE
            | EvseEvent::ResumeCharging
            | EvseEvent::ClearFault
            | EvseEvent::SetUnavailable
            | EvseEvent::Reserve
            | EvseEvent::CancelReservation => {
                Err(TransitionError::InvalidTransition { from: state, event })
            }
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
        assert_eq!(
            transition(EvseState::Available, EvseEvent::Reserve),
            Ok(EvseState::Reserved)
        );
        assert_eq!(
            transition(EvseState::Available, EvseEvent::SetUnavailable),
            Ok(EvseState::Unavailable)
        );
        assert!(transition(EvseState::Available, EvseEvent::StartCharging).is_err());
    }

    #[test]
    fn test_reserved_transitions() {
        assert_eq!(
            transition(EvseState::Reserved, EvseEvent::PlugIn),
            Ok(EvseState::Preparing)
        );
        assert_eq!(
            transition(EvseState::Reserved, EvseEvent::CancelReservation),
            Ok(EvseState::Available)
        );
        assert_eq!(
            transition(EvseState::Reserved, EvseEvent::SetUnavailable),
            Ok(EvseState::Unavailable)
        );
        assert!(transition(EvseState::Reserved, EvseEvent::StartCharging).is_err());
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
    fn test_suspended_ev_to_suspended_evse() {
        assert_eq!(
            transition(EvseState::SuspendedEV, EvseEvent::PauseEVSE),
            Ok(EvseState::SuspendedEVSE)
        );
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
        // Faulted states are sticky until cleared
        assert!(transition(EvseState::Faulted, EvseEvent::SetUnavailable).is_err());
    }

    #[test]
    fn test_unavailable_transitions() {
        assert_eq!(
            transition(EvseState::Unavailable, EvseEvent::SetAvailable),
            Ok(EvseState::Available)
        );
        assert_eq!(
            transition(EvseState::Unavailable, EvseEvent::Fault),
            Ok(EvseState::Faulted)
        );
        assert!(transition(EvseState::Unavailable, EvseEvent::PlugIn).is_err());
    }
}
