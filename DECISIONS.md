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
Power behavior is abstract but consistent. No `physics-agent`.

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

## Decision 8: No CPMS Runtime Logic

**Decision**
GridHammer expressly forbids implementation of CPMS business logic (pricing, billing, invoicing, user ledgers, roaming settlement).

**Rationale**
* GridHammer is a test tool, not a CPMS.
* Implementing CPMS logic creates circular dependencies (testing our logic vs target logic).
* Focus must remain on protocol stress.

**Consequence**
Agents like `pricing-agent`, `billing-agent`, `invoicing-agent`, `ledger-agent`, `user-agent`, and `settlement-agent` are permanently out of scope.

---

## Decision 9: No OCPI or Roaming

**Decision**
GridHammer will not implement OCPI, OCHP, OICP, or any roaming settlement protocols.

**Rationale**
* GridHammer simulates a single Charging Station (or set of EVSEs) connecting to a single CSMS.
* Roaming complexity belongs in the CSMS or a dedicated roaming simulator.

**Consequence**
No `ocpi-agent` or `roaming-agent`.

---

## Decision 10: No Smart Gateway Logic

**Decision**
The Gateway layer must be a dumb pipe. No message buffering (beyond TCP/WebSocket framing), no retry smoothing, no connection holding.

**Rationale**
* "Smart" gateways hide network/CSMS failures.
* We want to expose failures, not mask them.

**Consequence**
Network errors propagate immediately to the core FSM. No `smart-gateway-agent`.

---

## Reconsideration policy

Decisions here change only if:

* They block correctness
* They block determinism
* They prevent real-world bug discovery

Preference is always to **tighten**, not loosen.

