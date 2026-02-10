# NEXT_STEPS.md

## GridHammer — Execution Roadmap

This document defines the **ordered, non-negotiable next steps** to bootstrap GridHammer into a usable, trustworthy system.
Steps are sequenced to protect correctness, determinism, and scope discipline.

Do not skip steps.
Do not parallelize across layers prematurely.

---

## Current task in progress

- [ ] **Gateway layer implementation**
   - [ ] WebSocket client (tokio-tungstenite)
   - [ ] Async Runtime bridging (tokio <-> sync core)
   - [ ] Message serialization/deserialization

## Next up (queued)

- [ ] **Real-time Clock Mode**
- [ ] **Interactive CLI (`live` command)**

---

## Phase 0 — Ground truth locking (Done)

**Goal:** Make it impossible to build the wrong thing.

- [x] **Freeze contracts (Done, Refined 2026-02-09: Forbidden agents explicitly listed in all contracts, AGENTS.md reconciled. Re-verified 2026-02-09.)**
   * Finalize:
     * `ARCHITECTURE.md`
     * `DECISIONS.md`
     * `AGENTS.md`
     * `MANIFEST.md`
   * Steward-agent reviews for internal consistency.
   * Any contradiction must be resolved before code.
   * **CPMS runtime logic references removed (Confirmed 0 matches).**
   * **AGENTS.md explicit prohibitions verified.**
   * **Forbidden Domain Scope explicitly synchronized across all docs.**

- [x] **Create repo skeleton (Done)**
   ```
   gridhammer/
     rust-core/
     ts-shell/
     scenarios/
     docs/
   ```
   No implementation yet.

- [x] **Add forbidden-scope checklist (Done, Verified 2026-02-08)**
   * CPMS runtime logic
   * Pricing / OCPI
   * Gateway heuristics
   * UI-defined behavior

---

## Phase 1 — Rust core foundation (authoritative)

**Goal:** Establish protocol truth and determinism.

- [x] **Rust crate bootstrapping (Done)**
   * Create `gridhammer-core`
   * Enforce:
     * no wall-clock time
     * injected clock
     * seeded RNG
     * no async in core logic

- [x] **EVSE FSM v0**
   * Define EVSE states as enums
   * Encode legal transitions exhaustively
   * Add compile-time exhaustiveness checks
   * No ChargingProfiles yet

- [x] **TransactionEvent lifecycle (Done)**
   * Implement Started / Updated / Ended
   * Encode invariant checks:
     * no orphan transactions
     * no double-ended transactions

- [x] **Artifact emission (minimal)**
   * `evse_states.jsonl`
   * Deterministic ordering

**Exit criteria**
* FSM tests pass
* Replay test yields identical output

---

## Phase 2 — ChargingProfile executor (critical path) (Done)

**Goal:** Build the heart of GridHammer.

- [x] **ChargingProfile model (Done)**
   * TxProfile
   * StationMaxProfile
   * Time-based resolution

- [x] **Conflict resolution rules (Done)**
   * Explicit priority order
   * Deterministic tie-breaking
   * Zero-power semantics

- [x] **Executor invariants (Done, Verified 2026-02-07)**
   * Every enforced limit has a reason
   * No silent clamping
   * No negative or NaN values

- [x] **Property tests (Done)**
   * Overlapping profiles
   * Profile churn
   * Zero-power suspend

**Exit criteria**
* Executor is pure and deterministic
* 100% unit-test coverage for executor logic

---

## Phase 3 — Site constraints & projection (Done)

**Goal:** Simulate grid pressure without simulating physics.

- [x] **Site topology model (Done)**
   * Transformer cap
   * Feeder groups
   * EVSE membership

- [x] **Projection logic (Done)**
   * Site constraint → EVSE-level effects
   * Expressed only as:
     * ChargingProfile updates
     * SuspendedEVSE
     * Faulted

- [x] **Failure propagation (Done)**
   * One EVSE fault affecting site cap
   * Deterministic ordering

**Exit criteria**
* Site logic never bypasses OCPP semantics
* Projection is explainable per tick

---

## Phase 4 — OCPP 2.1 surface (Done)

**Goal:** Make the protocol surface undeniable.

- [x] **Message models (Done)**
   * BootNotification
   * StatusNotification
   * TransactionEvent
   * MeterValues
   * SetChargingProfile

- [x] **Semantic validation (Done)**
   * Required fields
   * State-dependent legality
   * Message ordering invariants

- [x] **Trace output (Done)**
   * `ocpp_trace.jsonl`
   * Stable ordering
   * Stable timestamps (tick-based)

**Exit criteria**
* Traces replay identically
* Invalid sequences are rejected loudly

---

## Phase 5 — TypeScript shell (control plane)

**Goal:** Operate GridHammer without touching core logic.

- [x] **Scenario schema (Done)**
   * YAML/JSON
   * Versioned
   * Validated via Zod

- [x] **CLI v0 (Done)**
   ```
   gridhammer run <scenario>
   gridhammer replay <run-id>
   gridhammer validate <scenario>
   ```

- [x] **Experiment registry**
   * Scenario hash
   * Seed
   * Core version
   * Artifact paths

**Exit criteria**
* CLI can run and replay a scenario end-to-end
* Registry is append-only

---

## Phase 6 — Golden scenarios (trust building)

**Goal:** Prove GridHammer finds real bugs.

- [x] **Golden MCS scenarios (Done)**
   * Peak arrival wave
   * Ramp-rate stress
   * Zero-power suspend
   * Mid-session fault
   * Site curtailment

- [x] **Assertions (Done)**
   * Explicit failure expectations
   * No “manual inspection only” tests

- [x] **Replay certification (Done, Verified 2026-02-08)**
   * Run → replay → diff must be empty

**Exit criteria**
* At least one scenario fails a known-buggy CPMS
* Failure is explainable from artifacts alone

---

## Phase 7 — Visualization (optional) (Done)

**Goal:** Human insight without semantic power.

- [x] **Read-only UI (Done)**
   * EVSE timelines
   * Profile diffs
   * Power curves

- [x] **Diff view (Done)**
   * Run vs run
   * Before vs after CPMS change

**Rule**
UI consumes artifacts only.
No live simulation hooks.

---

## Phase 8 — Live CPMS Integration (Interactive)

**Goal:** Connect the deterministic core to real-world CPMS endpoints.

- [ ] **Gateway layer implementation**
   * WebSocket client (tokio-tungstenite)
   * Async Runtime bridging (tokio <-> sync core)
   * Message serialization/deserialization

- [ ] **Real-time Clock Mode**
   * Wall-clock driver for the simulation loop
   * Heartbeat generation (real-time interval)

- [ ] **Interactive CLI (`live` command)**
   * Connect to CPMS URL
   * Session control (Start/Stop transaction)
   * Fault injection triggers
   * "Human-in-the-loop" event injection

**Exit criteria**
* Successful `BootNotification` and `Heartbeat` loop with a live CPMS
* User can manually trigger a charging session via CLI
* Protocol correctness maintained in live mode

---

## Anti-goals (do not start early)

* Web UI before core invariants
* OCPI / pricing logic
* Real-time simulation
* Physics modeling
* CPMS-like retries or smoothing

---

## Success signal

GridHammer is “ready” when:

* You trust failures more than successes
* Replays never surprise you
* The CPMS team dislikes running it

---

> “The hard part is not building the system — it is building the discipline to not lie to yourself.”
