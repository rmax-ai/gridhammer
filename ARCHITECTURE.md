# GridHammer — Architecture

## Purpose

GridHammer is a **deterministic OCPP 2.1 stress-testing platform** designed to expose failures in CPMS/CSMS implementations under heavy-duty Megawatt Charging System (MCS) load.

GridHammer simulates **protocol truth**, not physics.

---

## Architectural principles

1. **Rust is the source of truth**

   * All protocol semantics, state machines, and invariants live in Rust.
2. **TypeScript is control and observation**

   * CLI, scenarios, orchestration, and visualization only.
3. **Determinism over realism**

   * Replayability is mandatory.
4. **No UI-driven behavior**

   * UI never alters simulation semantics.
5. **Illegal states must be unrepresentable**

   * Prefer compile-time guarantees.

---

## High-level system layout

```
TypeScript Shell (control + lens)
│
│  CLI  ── scenarios ── experiment registry ── UI
│
└──── JSON / IPC ────▶
                    Rust Core (authoritative)
                    │
                    ├─ Simulation kernel
                    ├─ EVSE FSMs
                    ├─ ChargingProfile executor
                    ├─ OCPP 2.1 semantics
                    └─ Invariant enforcement
```

---

## Rust core responsibilities

The Rust core owns:

* EVSE finite state machines
* TransactionEvent lifecycle correctness
* ChargingProfile resolution and enforcement
* Site-level constraint projection
* Deterministic simulation clock
* Protocol invariants and assertions

The Rust core **does not**:

* Parse CLI flags
* Render UI
* Mutate scenarios
* Persist experiment metadata

---

## TypeScript shell responsibilities

The TypeScript shell owns:

* CLI UX (`gridhammer`)
  * `run`, `replay`, `sweep`, `validate`, `export`
* Scenario authoring and validation
* Experiment registry
* Artifact management
* Visualization and diffing

The TypeScript shell **does not**:

* Implement protocol logic
* Override Rust decisions
* Apply hidden defaults

---

## Determinism model

A run is uniquely defined by:

* Scenario hash
* Rust core version
* Seed
* CLI flags

Any deviation invalidates comparison.

---

## Extension boundaries

Allowed:

* New scenario types
* Additional invariants
* New visualizations

Forbidden:

* UI-defined simulation behavior
* Non-deterministic time
* Implicit async logic in core

---

## Forbidden Domain Scope (Non-goals)

The following areas are **permanently out of scope**:

*   **CPMS runtime logic**: Pricing engines, billing, invoicing, user ledgers, roaming settlement.
*   **OCPI / Roaming**: No OCPI, OCHP, or OICP logic.
*   **Smart Gateway**: No retry smoothing, connection holding, message buffering, or "smart" pipes.
*   **Physics**: No battery chemistry, real power electronics, or grid physics.
*   **UI-defined behavior**: The UI is read-only. It never inputs data into the simulation. All inputs come from scenarios.
*   **ISO-15118 PLC**: No low-level PLC timing or signal modulation.

Explicitly forbidden agents: `pricing-agent`, `billing-agent`, `invoicing-agent`, `ledger-agent`, `user-agent`, `ocpi-agent`, `roaming-agent`, `settlement-agent`, `smart-gateway-agent`, `physics-agent`, `plc-agent`.

