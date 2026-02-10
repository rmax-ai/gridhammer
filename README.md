# GridHammer

**Deterministic OCPP 2.1 stress testing for megawatt-scale charging systems.**

## Purpose

GridHammer is a **production-grade OCPP 2.1 simulation platform** designed to stress-test CPMS/CSMS implementations under Megawatt Charging System (MCS) loads.

It prioritizes **Protocol Truth** over physics. The goal is to find protocol violations, race conditions, and logic bugs in the management system by providing a strictly deterministic, replayable, and adversarial EVSE simulation.

## Core Principles

1.  **Rust decides what happens**: The core simulation (FSM, Invariants, ChargingProfiles) is authoritative.
2.  **TypeScript decides when and why**: The shell handles orchestration, scenarios, and observation.
3.  **Determinism first**: Every run is uniquely defined by its scenario and seed. "Works on my machine" is a failure state.
4.  **No UI logic**: The UI is a read-only lens. It never inputs data into the simulation.

## Architecture

The system enforces a strict boundary between control and simulation:

```mermaid
graph TD
    subgraph "TypeScript Shell (Control & Lens)"
        CLI[CLI (gridhammer)]
        Scenarios[Scenario Definitions]
        Registry[Experiment Registry]
        UI[Read-Only UI]
    end

    subgraph "Rust Core (Authoritative)"
        Sim[Simulation Kernel]
        FSM[EVSE FSMs]
        Profiles[ChargingProfile Executor]
        OCPP[OCPP 2.1 Semantics]
        Invariants[Invariant Enforcement]
    end

    CLI -->|JSON/IPC| Sim
    Scenarios -->|Data| Sim
    UI -.->|Reads Artifacts| Registry
```

*   **Rust Core**: Owns the "Physics of the Protocol" (State machines, Time, Limits).
*   **TypeScript Shell**: Owns the "Lab Environment" (Running, Replaying, Validating).

## Current Status

*   **Rust Core (Phases 1-4)**: Complete. Includes EVSE FSM, Transaction lifecycles, Site constraint projection, and comprehensive ChargingProfile enforcement.
*   **CLI (Phase 5)**: Functional. Supports running scenarios, replaying runs, and validating inputs.
*   **Golden Scenarios (Phase 6)**: Complete. Includes MCS peak load, ramp-rate stress, and fault injection scenarios.
*   **UI (Phase 7)**: In Progress (Read-only views partially implemented).

## Getting Started

### CLI Commands

GridHammer uses a stable CLI surface for all operations:

```bash
# Execute a specific scenario
gridhammer run <scenario>

# Replay a past run exactly (verifying determinism)
gridhammer replay <run-id>

# Run a parameter sweep
gridhammer sweep <param>=<range>

# Validate a scenario schema without running
gridhammer validate <scenario>

# Export artifacts from a run
gridhammer export <run-id>
```

### Artifacts

Every run produces immutable artifacts in the `runs/` directory:

*   `ocpp_trace.jsonl`: Full timeline of OCPP 2.1 messages.
*   `evse_states.jsonl`: Discrete state transitions for every EVSE.
*   `power_series.csv`: Power usage over time.
*   `assertions.json`: List of invariant checks and results.
*   `run_meta.json`: Metadata (seed, version, flags) for reproducibility.

## Documentation

*   **[ARCHITECTURE.md](ARCHITECTURE.md)**: Deep dive into the system design and boundaries.
*   **[PROJECT_SPEC.md](PROJECT_SPEC.md)**: Detailed specification of modules and data flows.
*   **[AGENTS.md](AGENTS.md)**: Protocol engineering contract and agent responsibilities.
*   **[NEXT_STEPS.md](NEXT_STEPS.md)**: Current roadmap and execution plan.

## Forbidden Domain Scope (Non-goals)

To maintain focus and correctness, the following areas are **permanently out of scope**. GridHammer is **NOT** a full ecosystem simulator:

*   **CPMS Runtime Logic**: No pricing engines, billing, invoicing, user ledgers, or roaming settlement.
*   **OCPI / Roaming**: No OCPI, OCHP, or OICP logic.
*   **Smart Gateway**: No retry smoothing, connection holding, message buffering, or "smart" pipes.
*   **Physics**: No battery chemistry, real power electronics, or grid physics.
*   **UI-Defined Behavior**: The UI is read-only. It never inputs data into the simulation.
*   **ISO-15118 PLC**: No low-level PLC timing or signal modulation.

***

*Maintained by the Steward Agent. Do not edit without architectural review.*
