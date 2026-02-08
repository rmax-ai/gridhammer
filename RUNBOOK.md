# GridHammer — Runbook

## Prerequisites

* Rust toolchain (stable)
* Node.js LTS
* GridHammer CLI installed

---

## CI

Run these commands to verify the build:

### Rust Core
```bash
# Run from repository root
cargo fmt --all -- --check
cargo clippy --workspace -- -D warnings
cargo test --workspace
```

### TypeScript Shell
```bash
cd ts-shell
npm test
npm run lint
```

---

## Running a scenario

```
gridhammer run scenarios/mcs-12mw.yaml --seed 42
```

Expected behavior:

* Scenario is validated
* Hash is computed
* Rust core is invoked
* Artifacts are written to a new run directory

---

## Replaying a run

```
gridhammer replay run-2026-02-07T18-30-00
```

Replay must:

* Produce identical artifacts
* Re-emit identical OCPP traces
* Re-trigger identical assertions

If not, determinism is broken.

---

## Sweeping parameters

```
gridhammer sweep power=1.5..3.75 --step 0.25
```

Each sweep:

* Produces independent runs
* Shares scenario hash
* Varies only declared parameters

---

## Validating scenarios

```
gridhammer validate scenarios/mcs-12mw.yaml
```

Validation checks:

* Schema correctness
* Logical consistency
* Explicit limits

---

## Exporting a run

```
gridhammer export run-2026-02-07T18-30-00
```

Exports:

* Compacted artifacts for sharing
* Portable archive format

---

## Interpreting failures

Check in order:

1. `assertions.json`
2. EVSE state timeline
3. ChargingProfile history
4. OCPP trace around failure timestamp

Never trust visualizations without trace confirmation.

---

## Incident workflow

1. Capture failing run ID
2. Replay locally
3. Reduce scenario if needed
4. Attach run artifacts to issue

---

## Operational rules

* Never delete run artifacts
* Never mutate scenarios after execution
* Never compare runs with different hashes

---

## Common mistakes

* Running without seed
* Editing scenario after execution
* Trusting UI over traces

---

## Forbidden Domain Scope

The following areas are **permanently out of scope**:

*   **CPMS runtime logic**: Pricing engines, billing, invoicing, user ledgers, roaming settlement.
*   **OCPI / Roaming**: No OCPI, OCHP, or OICP logic.
*   **Smart Gateway**: No retry smoothing, connection holding, message buffering, or "smart" pipes.
*   **Physics**: No battery chemistry, real power electronics, or grid physics.
*   **UI-defined behavior**: The UI is read-only. It never inputs data into the simulation. All inputs come from scenarios.
*   **ISO-15118 PLC**: No low-level PLC timing or signal modulation.

If you need these features, you are looking for a CPMS, not GridHammer.


