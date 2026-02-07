# GridHammer Core

This crate implements the deterministic simulation engine for GridHammer.

## Responsibilities

* **Simulation Kernel**: Discrete-time event loop with injected clock.
* **State Machines**: Authoritative EVSE FSMs (Finite State Machines).
* **Profiles**: ChargingProfile resolution and enforcement.
* **Invariants**: Protocol correctness assertions.

## Architecture

This library is designed to be:
* **Deterministic**: All randomness is seeded; time is virtual.
* **Async-free**: Core logic is synchronous and pure.
* **Headless**: Can run without any UI or network transport.

## Usage

```rust
use gridhammer_core::sim::engine::Engine;
use chrono::Utc;

let start = Utc::now();
let seed = 42;
let evse_count = 10;

let mut engine = Engine::new(start, seed, evse_count);
engine.tick();
```
