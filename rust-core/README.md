# GridHammer Core

This crate implements the authoritative simulation kernel and OCPP 2.1 protocol logic.

## Constraints

* **No Async**: Core logic must be synchronous and deterministic.
* **No Wall-Clock**: Use `sim::clock::SimClock` only.
* **Seeded RNG**: Use `sim::rng::SimRng` only.
* **No IO**: Do not access files or network directly.
