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
* No UI-defined logic
* No CPMS runtime logic (pricing, billing, invoicing, user ledgers, roaming settlement)
* No OCPI / Roaming logic (OCHP, OICP)
* No smart gateway logic (retry smoothing, connection holding, message buffering)
* No physics simulation (battery chemistry, real power electronics, grid physics)
* No ISO-15118 PLC logic (timing, signal modulation)

---

## Success criteria

GridHammer is successful if it:

* Finds real CPMS bugs
* Produces explainable failures
* Allows exact replay months later

