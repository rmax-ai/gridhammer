use super::state::EvseState;

pub enum Input {
    PlugIn,
    Unplug,
    Authorize,
    StartTransaction,
    StopTransaction,
    SuspendEV,
    SuspendEVSE,
    ResumeEV,
    ResumeEVSE,
    Fault,
    ClearFault,
}

pub fn transition(state: EvseState, input: Input) -> Result<EvseState, String> {
    match (state, input) {
        (EvseState::Available, Input::PlugIn) => Ok(EvseState::Preparing),
        (EvseState::Preparing, Input::Unplug) => Ok(EvseState::Available),
        (EvseState::Preparing, Input::Authorize) => Ok(EvseState::Preparing),
        (EvseState::Preparing, Input::StartTransaction) => Ok(EvseState::Charging),
        (EvseState::Charging, Input::StopTransaction) => Ok(EvseState::Preparing),
        (EvseState::Charging, Input::SuspendEV) => Ok(EvseState::SuspendedEV),
        (EvseState::SuspendedEV, Input::ResumeEV) => Ok(EvseState::Charging),
        (EvseState::Charging, Input::SuspendEVSE) => Ok(EvseState::SuspendedEVSE),
        (EvseState::SuspendedEVSE, Input::ResumeEVSE) => Ok(EvseState::Charging),
        (_, Input::Fault) => Ok(EvseState::Faulted),
        (EvseState::Faulted, Input::ClearFault) => Ok(EvseState::Available),
        (EvseState::Charging, Input::Unplug) => Err("Cannot unplug while charging".to_string()),
        _ => Err("Illegal transition".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_happy_path() {
        let mut state = EvseState::Available;
        state = transition(state, Input::PlugIn).unwrap();
        assert_eq!(state, EvseState::Preparing);
        state = transition(state, Input::StartTransaction).unwrap();
        assert_eq!(state, EvseState::Charging);
        state = transition(state, Input::StopTransaction).unwrap();
        assert_eq!(state, EvseState::Preparing);
        state = transition(state, Input::Unplug).unwrap();
        assert_eq!(state, EvseState::Available);
    }

    #[test]
    fn test_fault_transitions() {
        let state = EvseState::Available;
        let new_state = transition(state, Input::Fault).unwrap();
        assert_eq!(new_state, EvseState::Faulted);

        let new_state2 = transition(new_state, Input::ClearFault).unwrap();
        assert_eq!(new_state2, EvseState::Available);

        let charging_state = EvseState::Charging;
        let faulted = transition(charging_state, Input::Fault).unwrap();
        assert_eq!(faulted, EvseState::Faulted);
    }

    #[test]
    fn test_illegal_transitions() {
        let state = EvseState::Available;
        assert!(transition(state, Input::Unplug).is_err());
        assert!(transition(state, Input::StopTransaction).is_err());

        let charging = EvseState::Charging;
        assert!(transition(charging, Input::Unplug).is_err());
    }
}
