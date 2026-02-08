use super::state::{EvseEvent, EvseState};

/// Pure transition function for EVSE state machine.
///
/// Follows strict precedence and logical transitions.
/// Note: Availability changes during active transactions are handled by the application layer queuing
/// the state change. This FSM represents the *current* effective state.
pub fn transition(state: EvseState, event: EvseEvent) -> Result<EvseState, &'static str> {
    match state {
        EvseState::Available => match event {
            EvseEvent::PlugIn => Ok(EvseState::Preparing),
            EvseEvent::Reserve => Ok(EvseState::Reserved),
            EvseEvent::SetUnavailable => Ok(EvseState::Unavailable),
            EvseEvent::Fault => Ok(EvseState::Faulted),
            // Illegal events
            EvseEvent::Unplug
            | EvseEvent::StartCharging
            | EvseEvent::StopCharging
            | EvseEvent::PauseEV
            | EvseEvent::PauseEVSE
            | EvseEvent::ResumeCharging
            | EvseEvent::ClearFault
            | EvseEvent::SetAvailable
            | EvseEvent::CancelReservation => Err("Invalid transition from Available"),
        },
        EvseState::Reserved => match event {
            EvseEvent::PlugIn => Ok(EvseState::Preparing),
            EvseEvent::CancelReservation => Ok(EvseState::Available),
            EvseEvent::SetUnavailable => Ok(EvseState::Unavailable),
            EvseEvent::Fault => Ok(EvseState::Faulted),
            // Illegal events
            EvseEvent::Unplug
            | EvseEvent::StartCharging
            | EvseEvent::StopCharging
            | EvseEvent::PauseEV
            | EvseEvent::PauseEVSE
            | EvseEvent::ResumeCharging
            | EvseEvent::ClearFault
            | EvseEvent::SetAvailable
            | EvseEvent::Reserve => Err("Invalid transition from Reserved"),
        },
        EvseState::Preparing => match event {
            EvseEvent::StartCharging => Ok(EvseState::Charging),
            EvseEvent::Unplug => Ok(EvseState::Available),
            EvseEvent::Fault => Ok(EvseState::Faulted),
            // Illegal events
            EvseEvent::PlugIn
            | EvseEvent::StopCharging
            | EvseEvent::PauseEV
            | EvseEvent::PauseEVSE
            | EvseEvent::ResumeCharging
            | EvseEvent::ClearFault
            | EvseEvent::SetUnavailable
            | EvseEvent::SetAvailable
            | EvseEvent::Reserve
            | EvseEvent::CancelReservation => Err("Invalid transition from Preparing"),
        },
        EvseState::Charging => match event {
            EvseEvent::StopCharging => Ok(EvseState::Available),
            EvseEvent::PauseEV => Ok(EvseState::SuspendedEV),
            EvseEvent::PauseEVSE => Ok(EvseState::SuspendedEVSE),
            EvseEvent::Fault => Ok(EvseState::Faulted),
            // Illegal events
            EvseEvent::PlugIn
            | EvseEvent::Unplug
            | EvseEvent::StartCharging
            | EvseEvent::ResumeCharging
            | EvseEvent::ClearFault
            | EvseEvent::SetUnavailable
            | EvseEvent::SetAvailable
            | EvseEvent::Reserve
            | EvseEvent::CancelReservation => Err("Invalid transition from Charging"),
        },
        EvseState::SuspendedEV => match event {
            EvseEvent::ResumeCharging => Ok(EvseState::Charging),
            EvseEvent::StopCharging => Ok(EvseState::Available),
            EvseEvent::Fault => Ok(EvseState::Faulted),
            // Illegal events
            EvseEvent::PlugIn
            | EvseEvent::Unplug
            | EvseEvent::StartCharging
            | EvseEvent::PauseEV
            | EvseEvent::PauseEVSE
            | EvseEvent::ClearFault
            | EvseEvent::SetUnavailable
            | EvseEvent::SetAvailable
            | EvseEvent::Reserve
            | EvseEvent::CancelReservation => Err("Invalid transition from SuspendedEV"),
        },
        EvseState::SuspendedEVSE => match event {
            EvseEvent::ResumeCharging => Ok(EvseState::Charging),
            EvseEvent::StopCharging => Ok(EvseState::Available),
            EvseEvent::Fault => Ok(EvseState::Faulted),
            // Illegal events
            EvseEvent::PlugIn
            | EvseEvent::Unplug
            | EvseEvent::StartCharging
            | EvseEvent::PauseEV
            | EvseEvent::PauseEVSE
            | EvseEvent::ClearFault
            | EvseEvent::SetUnavailable
            | EvseEvent::SetAvailable
            | EvseEvent::Reserve
            | EvseEvent::CancelReservation => Err("Invalid transition from SuspendedEVSE"),
        },
        EvseState::Faulted => match event {
            EvseEvent::ClearFault => Ok(EvseState::Available),
            // Illegal events - must clear fault first
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
            | EvseEvent::CancelReservation => Err("Invalid transition from Faulted"),
        },
        EvseState::Unavailable => match event {
            EvseEvent::SetAvailable => Ok(EvseState::Available),
            EvseEvent::Fault => Ok(EvseState::Faulted),
            // Illegal events
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
            | EvseEvent::CancelReservation => Err("Invalid transition from Unavailable"),
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
