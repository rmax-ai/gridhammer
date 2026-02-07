# GridHammer — Architectural Decisions

This document records **intentional, irreversible decisions**.

---

## Decision 1: Rust core + TypeScript shell

**Decision**
Use Rust for simulation correctness and TypeScript for orchestration.

**Rationale**

* Rust enforces FSM completeness and invariants
* TypeScript enables fast iteration and UX

**Consequence**
Two-language system with explicit boundary.

---

## Decision 2: CLI-first, UI-secondary

**Decision**
CLI is the primary interface; UI is read-only.

**Rationale**

* Determinism
* CI compatibility
* Reproducibility

**Consequence**
No UI-driven behavior allowed.

---

## Decision 3: Discrete-time simulation

**Decision**
Use explicit ticks, not wall-clock time.

**Rationale**

* Replayability
* Debuggability
* Control over causality

**Consequence**
Real-time realism is sacrificed.

---

## Decision 4: OCPP 2.1 only

**Decision**
Implement OCPP 2.1 strictly.

**Rationale**

* Modern CPMS behavior
* Unified TransactionEvent model
* Cleaner ChargingProfile semantics

**Consequence**
No legacy protocol support.

---

## Decision 5: Immutable artifacts

**Decision**
All run outputs are immutable.

**Rationale**

* Trust
* Auditability
* Long-term comparison

**Consequence**
Storage grows; pruning must be explicit.

---

## Decision 6: No physics simulation

**Decision**
Do not simulate EV internals or grid physics.

**Rationale**

* Protocol bugs dominate CPMS failures
* Physics adds noise, not signal

**Consequence**
Power behavior is abstract but consistent.

---

## Decision 7: Explicit non-goals

**Decision**
Document what GridHammer will not do.

**Rationale**

* Prevent scope creep
* Maintain clarity

**Consequence**
Some feature requests are rejected by design.

---

## Reconsideration policy

Decisions here change only if:

* They block correctness
* They block determinism
* They prevent real-world bug discovery

Preference is always to **tighten**, not loosen.

