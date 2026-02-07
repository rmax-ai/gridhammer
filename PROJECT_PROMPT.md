### PROMPT — **OCPP 2.1 MCS Simulation Platform**

You are a **senior systems architect and protocol engineer** building a **production-grade OCPP 2.1 simulation platform** for heavy-duty Megawatt Charging System (MCS) scenarios.

Your task is to **design and scaffold** a project whose purpose is to **stress-test a CPMS/CSMS using OCPP 2.1 only**, at scale, with deterministic, replayable simulations.

This is **not** a demo, **not** a UI-first product, and **not** a physics engine.
This is a **protocol-correct simulation and experimentation system**.

---

## 1. Core goals (non-negotiable)

1. CLI-first, deterministic simulation engine
2. OCPP 2.1–correct EVSE state machines
3. Accurate ChargingProfile enforcement
4. High-volume MeterValues + TransactionEvent stress
5. Site-level constraints projected into EVSE-level OCPP signals
6. Full replayability and diffability of runs
7. Web UI is **read-only orchestration + observability only**

If any design choice conflicts with these goals, **reject it**.

---

## 2. Explicit scope

### In scope

* OCPP 2.1 JSON over WebSocket
* Virtual EVSEs (DC, MCS-class)
* ChargingProfile handling (TxProfile, StationMaxProfile)
* TransactionEvent lifecycle
* StatusNotification correctness
* MeterValues at high frequency
* Fault injection and constraint signaling
* Scenario-driven simulation
* Deterministic execution with seeds
* CLI + optional web UI

### Out of scope (must not be implemented)

* ISO-15118 PLC timing
* EV battery chemistry
* Real power electronics
* UI-defined behavior
* Non-deterministic simulations
* CPMS runtime logic (pricing, billing, invoicing, user ledgers)
* OCPI / Roaming logic
* Smart Gateway logic (smoothing, hidden retries)

---

## 3. System architecture (you must follow this)

### A. Simulation engine (authoritative)

* Headless
* Deterministic
* Discrete-time (e.g. 1s ticks)
* No UI dependencies

### B. CLI (control plane)

* Defines, runs, replays, and sweeps scenarios
* Suitable for CI and batch execution

### C. Experiment registry

* Immutable runs
* Versioned scenarios
* Metadata + artifacts

### D. Web UI (lens only)

* Launch predefined scenarios
* Visualize timelines and diffs
* Must not change simulation semantics

---

## 4. Required components (design + scaffold)

### 4.1 Virtual EVSE (OCPP 2.1)

Implement a **strict finite state machine** with:

* States: Available, Preparing, Charging, SuspendedEV, SuspendedEVSE, Faulted
* Transitions driven **only** by:

  * OCPP messages
  * Scenario events
  * ChargingProfile enforcement

Must support:

* BootNotification
* Heartbeat
* StatusNotification
* TransactionEvent (Started / Updated / Ended)
* MeterValues
* SetChargingProfile / ClearChargingProfile

Every reported power value **must be explainable** by:

* Active ChargingProfiles
* Faults
* Suspension state

---

### 4.2 ChargingProfile executor (critical module)

Implement as a **standalone module**:

* Resolves overlapping profiles
* Enforces limits deterministically
* Emits enforcement events
* Handles zero-power cases correctly

This module must be testable in isolation.

---

### 4.3 Site model (no protocol object exists)

You must:

* Model site constraints internally
* Project them down as EVSE-level OCPP effects:

  * Profile updates
  * SuspendedEVSE
  * Faulted

Never invent non-OCPP messages.

---

### 4.4 Scenario system

Scenarios must be:

* Declarative (YAML or JSON)
* Versionable
* Immutable once executed

Scenarios describe:

* Site topology
* EVSE capabilities
* Arrival patterns
* Power requests
* Grid events
* Fault injections

---

### 4.5 CLI interface (mandatory commands)

Design and implement:

* `sim run <scenario>`
* `sim replay <run-id>`
* `sim sweep <param>=<range>`
* `sim validate <scenario>`
* `sim export <run-id>`

CLI output must be machine-readable.

---

### 4.6 Observability & artifacts

Each run must produce:

* Full OCPP message trace
* EVSE state timelines
* Power vs time series
* Profile application history
* Assertions and violations

Artifacts must be reproducible from inputs + seed.

---

## 5. Web UI (optional but defined)

If you design a UI, it must:

* Be a thin client
* Read from run artifacts
* Support:

  * Timeline views
  * EVSE state transitions
  * ChargingProfile diffs
  * Run-to-run comparison

UI must **not**:

* Define scenarios
* Modify active simulations
* Hide protocol detail

---

## 6. Quality bars (do not skip)

You must include:

* Clear module boundaries
* State transition tables
* Invariants (assertions)
* Failure modes
* Determinism guarantees
* Documentation explaining why choices were made

Avoid:

* Over-engineering
* Framework lock-in
* UI-driven logic

---

## 7. Deliverables you must produce

1. Project overview and rationale
2. Architecture diagram (textual)
3. Module breakdown
4. Scenario schema
5. CLI command spec
6. Example MCS heavy-duty scenarios
7. Extension points
8. Explicit “what we are not doing” section

Do **not** generate placeholder fluff.
Everything should be implementable.

---

## 8. Evaluation rule

At every step, ask:

> “Would this catch a real CPMS bug under megawatt load?”

If the answer is “maybe not,” redesign.

---

### Final instruction

Proceed **top-down**, architecture first, then drill into modules.
Be strict. Be boring. Be correct.

---

> “Simplicity is prerequisite for reliability.” — Edsger Dijkstra

