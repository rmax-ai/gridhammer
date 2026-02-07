## GridHammer — Specification (Rust core + TypeScript shell)

### Design principle (non-negotiable)

* **Rust decides what happens**
* **TypeScript decides when and why**
* **Nothing important happens implicitly**

---

## 1. High-level architecture

```
┌──────────────────────────┐
│      TypeScript Shell     │
│                          │
│  - CLI (gridhammer)      │
│  - Scenario authoring    │
│  - Experiment registry   │
│  - Web UI (read-only)    │
│                          │
└───────────┬──────────────┘
            │  (JSON / IPC / gRPC)
            ▼
┌──────────────────────────┐
│        Rust Core          │
│                          │
│  - Simulation kernel     │
│  - EVSE FSM              │
│  - ChargingProfile exec  │
│  - OCPP 2.1 semantics    │
│  - Deterministic clock   │
│                          │
└──────────────────────────┘
```

---

## 2. Rust core (authoritative layer)

### 2.1 Responsibilities

The Rust core **must** be able to run headless forever and still be correct.

It owns:

* EVSE finite state machines
* Transaction lifecycle semantics
* ChargingProfile resolution and enforcement
* Deterministic simulation clock
* OCPP 2.1 message correctness
* Assertion of invariants

It does **not**:

* Parse CLI flags
* Render UIs
* Decide scenarios
* Persist results beyond emitting artifacts

---

### 2.2 Rust crates (internal structure)

```
gridhammer-core/
├─ sim/
│  ├─ engine.rs        # discrete-time simulation loop
│  ├─ clock.rs         # injected deterministic clock
│  └─ rng.rs           # seeded RNG
│
├─ ocpp/
│  ├─ messages.rs      # OCPP 2.1 message enums
│  ├─ validation.rs    # schema + semantic checks
│  └─ transport.rs    # abstract (no WebSocket here)
│
├─ evse/
│  ├─ state.rs         # EVSE states (enum)
│  ├─ fsm.rs           # transition table
│  └─ transaction.rs  # TransactionEvent logic
│
├─ profiles/
│  ├─ model.rs         # ChargingProfile structures
│  ├─ resolver.rs     # conflict resolution
│  └─ executor.rs     # enforced power output
│
├─ site/
│  ├─ topology.rs     # site + transformer limits
│  └─ projection.rs   # site → EVSE OCPP effects
│
├─ invariants/
│  └─ assertions.rs   # protocol guarantees
│
└─ lib.rs
```

---

### 2.3 EVSE FSM (Rust, compile-time enforced)

* States as **enum**
* Transitions as exhaustive `match`
* Illegal transitions = compile error or panic (depending on mode)

Example invariant:

* `Charging → Faulted` must always emit:

  * `StatusNotification(Faulted)`
  * `TransactionEvent(Ended, reason=Error)`

No silent transitions. Ever.

---

### 2.4 ChargingProfile executor (Rust, pure logic)

**Inputs**

* Active profiles (Tx, StationMax)
* Time (tick)
* Requested power
* Site-imposed caps

**Outputs**

* Enforced power limit
* Enforcement reason
* Optional suspension signal

Properties:

* Pure function
* Deterministic
* Fully unit-testable
* No knowledge of transport or UI

This module is the *heart* of GridHammer.

---

### 2.5 Simulation kernel

* Discrete ticks (e.g. 1s)

* Tick order:

  1. Apply scenario events
  2. Resolve site constraints
  3. Resolve ChargingProfiles
  4. Advance EVSE FSMs
  5. Emit OCPP events

* No wall-clock time

* No sleeps

* Replayable with seed + scenario hash

---

### 2.6 Rust outputs (artifacts)

The core emits **structured streams**, never prose:

* `ocpp_trace.jsonl`
* `evse_states.jsonl`
* `power_series.csv`
* `assertions.json`

Everything needed to replay lives here.

---

## 3. TypeScript shell (control + lens)

### 3.1 Responsibilities

TypeScript owns:

* CLI UX
* Scenario schema & validation
* Experiment orchestration
* Persistence of runs
* Visualization & diffing

It does **not**:

* Implement protocol logic
* Override Rust decisions
* Patch incorrect behavior

---

### 3.2 CLI (TypeScript)

Binary:

```
gridhammer
```

Commands:

```
gridhammer run <scenario>
gridhammer replay <run-id>
gridhammer sweep <param>=<range>
gridhammer validate <scenario>
gridhammer export <run-id>
```

The CLI:

* Validates input
* Launches Rust core
* Stores artifacts
* Never interprets protocol semantics

---

### 3.3 Scenario system (TypeScript-defined, Rust-consumed)

Scenarios are **data**, not code.

Defined in YAML/JSON:

* Site topology
* EVSE capabilities
* Arrival schedules
* Requested power
* Grid events
* Fault injections

TS validates schema → passes to Rust unchanged.

Once executed:

* Scenario is immutable
* Hash is stored with run

---

### 3.4 Experiment registry

A thin TS-managed layer:

* Run metadata
* Scenario hash
* Core version
* CLI flags
* Artifact paths

Think “test results database,” not stateful system.

---

### 3.5 Web UI (optional, read-only)

Stack:

* Vite + React + TypeScript

Capabilities:

* Timeline visualization
* EVSE state transitions
* ChargingProfile diffs
* Run-to-run comparison

UI reads artifacts.
UI never calls the core directly.

---

## 4. Rust ↔ TypeScript boundary

Keep it **boring and explicit**.

Options (in order of preference):

1. JSON over stdin/stdout (simplest, most debuggable)
2. gRPC (if you need long-running sessions)
3. WASM (future, not v1)

Contract:

* TS sends `RunConfig`
* Rust responds with `RunResult`
* Everything else is file-based

No shared mutable state.

---

## 5. Determinism guarantees

A GridHammer run is uniquely defined by:

* Scenario hash
* Core version
* Seed
* CLI flags

If any of these differ, the run is *not comparable*.

This must be enforced in tooling.


---

## 7. Forbidden Scope (Checklist)

This section enumerates behavior that is explicitly **out of scope**. Any attempt to implement these features must be rejected.

*   **CPMS runtime logic**: Pricing engines, billing, invoicing, user ledgers, roaming settlement.
*   **OCPI / Roaming**: No OCPI, OCHP, or OICP logic.
*   **Smart Gateway**: No retry smoothing, connection holding, message buffering, or "smart" pipes.
*   **Physics**: No battery chemistry, real power electronics, or grid physics.
*   **UI-defined behavior**: The UI is read-only. It never inputs data into the simulation. All inputs come from scenarios.

Reject any task that implies:

* Protocol shortcuts
* Hidden timing bugs
* “Works on my machine” simulations
* Unreplayable failures

---

