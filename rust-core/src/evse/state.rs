#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvseState {
    Available,
    Preparing,
    Charging,
    SuspendedEV,
    SuspendedEVSE,
    Faulted,
}
