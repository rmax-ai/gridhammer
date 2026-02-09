# GridHammer — Project Manifest

## Project name

**GridHammer**

## One-line description

Deterministic OCPP 2.1 stress testing for megawatt-scale charging systems.

---

## Core components

### Rust Core

* Language: Rust (stable)
* Role: Authoritative simulation engine
* Guarantees: Protocol correctness, determinism, invariant enforcement

### TypeScript Shell

* Language: TypeScript (Node.js LTS)
* Role: Control plane and observability
* Guarantees: Ergonomic workflows, reproducibility

---

## Required capabilities

* OCPP 2.1 message correctness
* EVSE FSM with explicit transitions
* ChargingProfile conflict resolution
* High-frequency MeterValues
* Fault and constraint injection
* Full replayability

---

## CLI surface (stable API)

```
gridhammer run <scenario>
gridhammer replay <run-id>
gridhammer sweep <param>=<range>
gridhammer validate <scenario>
gridhammer export <run-id>
```

CLI output is machine-readable by default.

---

## Artifacts

Each run produces:

* `ocpp_trace.jsonl`
* `evse_states.jsonl`
* `power_series.csv`
* `assertions.json`
* `run_meta.json`

Artifacts are immutable.

---

## Versioning policy

* Rust core: semantic versioning
* Scenario schema: explicit version field
* Breaking changes require migration notes

---

## Quality bars

* No silent protocol deviations
* No implicit defaults
* No hidden state

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

---

## Success criteria

GridHammer is successful if it:

* Finds real CPMS bugs
* Produces explainable failures
* Allows exact replay months later

