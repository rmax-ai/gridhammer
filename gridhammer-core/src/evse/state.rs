use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EvseState {
    Available,
    Preparing,
    Charging,
    SuspendedEV,
    SuspendedEVSE,
    Faulted,
    Unavailable,
    Reserved,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvseEvent {
    PlugIn,
    Unplug,
    StartCharging,
    StopCharging,
    PauseEV,
    PauseEVSE,
    ResumeCharging,
    Fault,
    ClearFault,
    SetUnavailable,
    SetAvailable,
    Reserve,
    CancelReservation,
}
