# NEXT_STEPS.md

## GridHammer — Execution Roadmap

This document defines the **ordered, non-negotiable next steps** to bootstrap GridHammer into a usable, trustworthy system.
Steps are sequenced to protect correctness, determinism, and scope discipline.

Do not skip steps.
Do not parallelize across layers prematurely.

---

## Phase 0 — Ground truth locking (Done)

**Goal:** Make it impossible to build the wrong thing.

1. **Freeze contracts (Done, Verified 2026-02-07)**

   * Finalize:

     * `ARCHITECTURE.md`
     * `DECISIONS.md`
     * `AGENTS.md`
     * `MANIFEST.md`
   * Steward-agent reviews for internal consistency.
   * Any contradiction must be resolved before code.

2. **Create repo skeleton (Done)**

   ```
   gridhammer/
     rust-core/
     ts-shell/
     scenarios/
     docs/
   ```

   No implementation yet.

3. **Add forbidden-scope checklist (Done, Verified 2026-02-07)**

   * CPMS runtime logic
   * Pricing / OCPI
   * Gateway heuristics
   * UI-defined behavior

---

## Phase 1 — Rust core foundation (authoritative) (Done)

**Goal:** Establish protocol truth and determinism.

1. **Rust crate bootstrapping (Done)**

   * Create `gridhammer-core`
   * Enforce:

     * no wall-clock time
     * injected clock
     * seeded RNG
     * no async in core logic

2. **EVSE FSM v0 (Done)**

   * Define EVSE states as enums
   * Encode legal transitions exhaustively
   * Add compile-time exhaustiveness checks
   * No ChargingProfiles yet

3. **TransactionEvent lifecycle (Done)**

   * Implement Started / Updated / Ended
   * Encode invariant checks:

     * no orphan transactions
     * no double-ended transactions

4. **Artifact emission (minimal) (Done)**

   * `evse_states.jsonl`
   * Deterministic ordering

**Exit criteria**

* FSM tests pass
* Replay test yields identical output

---

## Phase 2 — ChargingProfile executor (critical path) (Done)

**Goal:** Build the heart of GridHammer.

1. **ChargingProfile model (Done)**

   * TxProfile
   * StationMaxProfile
   * Time-based resolution

2. **Conflict resolution rules (Done)**

   * Explicit priority order
   * Deterministic tie-breaking
   * Zero-power semantics

3. **Executor invariants (Done, Verified 2026-02-07)**

   * Every enforced limit has a reason
   * No silent clamping
   * No negative or NaN values

4. **Property tests (Done)**

   * Overlapping profiles
   * Profile churn
   * Zero-power suspend

**Exit criteria**

* Executor is pure and deterministic
* 100% unit-test coverage for executor logic

---

## Phase 3 — Site constraints & projection (now)

**Goal:** Simulate grid pressure without simulating physics.

1. **Site topology model**

   * Transformer cap
   * Feeder groups
   * EVSE membership

2. **Projection logic**

   * Site constraint → EVSE-level effects
   * Expressed only as:

     * ChargingProfile updates
     * SuspendedEVSE
     * Faulted

3. **Failure propagation**

   * One EVSE fault affecting site cap
   * Deterministic ordering

**Exit criteria**

* Site logic never bypasses OCPP semantics
* Projection is explainable per tick

---

## Phase 4 — OCPP 2.1 surface

**Goal:** Make the protocol surface undeniable.

1. **Message models**

   * BootNotification
   * StatusNotification
   * TransactionEvent
   * MeterValues
   * SetChargingProfile

2. **Semantic validation**

   * Required fields
   * State-dependent legality
   * Message ordering invariants

3. **Trace output**

   * `ocpp_trace.jsonl`
   * Stable ordering
   * Stable timestamps (tick-based)

**Exit criteria**

* Traces replay identically
* Invalid sequences are rejected loudly

---

## Phase 5 — TypeScript shell (control plane)

**Goal:** Operate GridHammer without touching core logic.

1. **Scenario schema**

   * YAML/JSON
   * Versioned
   * Validated via Zod

2. **CLI v0**

   ```
   gridhammer run <scenario>
   gridhammer replay <run-id>
   gridhammer validate <scenario>
   ```

3. **Experiment registry**

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

1. **Golden MCS scenarios**

   * Peak arrival wave
   * Ramp-rate stress
   * Zero-power suspend
   * Mid-session fault
   * Site curtailment

2. **Assertions**

   * Explicit failure expectations
   * No “manual inspection only” tests

3. **Replay certification**

   * Run → replay → diff must be empty

**Exit criteria**

* At least one scenario fails a known-buggy CPMS
* Failure is explainable from artifacts alone

---

## Phase 7 — Visualization (optional)

**Goal:** Human insight without semantic power.

1. **Read-only UI**

   * EVSE timelines
   * Profile diffs
   * Power curves

2. **Diff view**

   * Run vs run
   * Before vs after CPMS change

**Rule**
UI consumes artifacts only.
No live simulation hooks.

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

