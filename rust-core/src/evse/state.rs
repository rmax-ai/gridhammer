#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvseState {
    Available,
    Occupied,
    Reserved,
    Unavailable,
    Faulted,
}
