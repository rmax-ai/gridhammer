#[derive(Debug, Clone, PartialEq, Copy)]
pub enum EvseState {
    Available,
    Preparing,
    Charging,
    SuspendedEV,
    SuspendedEVSE,
    Faulted,
}
