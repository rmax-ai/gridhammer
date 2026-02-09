#!/usr/bin/env bash
set -euo pipefail

# GridHammer — Autonomous Development Phases
# Feeds high-level, architecture-safe goals to loop.sh
# Designed for overnight / unattended execution.
# Stops on first failure to preserve correctness.
# Assesses progress to skip completed phases.

LOOP="./loop.sh"

if [[ ! -x "$LOOP" ]]; then
  echo "Error: loop.sh not found or not executable"
  exit 1
fi



run_phase() {
  local id="$1"
  local name="$2"
  local goal="$3"

  echo
  echo "======================================================"
  echo "PHASE $id: $name"
  echo "GOAL : $goal"
  echo "======================================================"

  echo "▶ EXECUTING..."
  "$LOOP" "$goal. Verify progress and refine and improve over existing progress."
}

# ------------------------------------------------------
# PHASE ORDER
# Ground truth → Rust core → profiles → site → OCPP →
# TS shell → golden scenarios → (optional UI)
# ------------------------------------------------------

run_phase "0" "Ground Truth Locking" \
  "Freeze GridHammer contracts: reconcile ARCHITECTURE.md, MANIFEST.md, DECISIONS.md, AGENTS.md, and RUNBOOK.md. Remove any remaining CPMS logic references. Ensure AGENTS.md explicitly forbids CPMS runtime behavior. No code changes beyond repo skeleton."

run_phase "1" "Rust Core Skeleton" \
  "Bootstrap Rust core (gridhammer-core): deterministic simulation engine with injected clock and seeded RNG, no wall-clock time, no async. Add crate structure and minimal build/test harness."

run_phase "1.1" "EVSE FSM v0" \
  "Implement EVSE finite state machine in Rust using enums and exhaustive transitions. Encode legal state transitions only. Add compile-time exhaustiveness checks and unit tests. No ChargingProfiles yet."

run_phase "1.2" "TransactionEvent Lifecycle" \
  "Implement OCPP 2.1 TransactionEvent lifecycle (Started, Updated, Ended) in Rust. Enforce invariants: no orphan transactions, no double-ended transactions. Emit deterministic evse_states.jsonl."

run_phase "2" "ChargingProfile Executor (Critical)" \
  "Implement pure ChargingProfile executor in Rust: TxProfile and StationMaxProfile, deterministic conflict resolution, zero-power semantics, explicit enforcement reasons. Achieve full unit and property-test coverage."

run_phase "3" "Site Constraints & Projection" \
  "Add site topology and constraint modeling in Rust (transformer caps, feeder groups). Project site constraints strictly into EVSE-level effects via ChargingProfiles, SuspendedEVSE, or Faulted states only. No physics simulation."

run_phase "4" "OCPP 2.1 Protocol Surface" \
  "Implement strict OCPP 2.1 message models and semantic validation in Rust: BootNotification, StatusNotification, TransactionEvent, MeterValues, SetChargingProfile. Emit deterministic ocpp_trace.jsonl."

run_phase "5" "TypeScript Shell v0" \
  "Implement TypeScript control plane: scenario schema (YAML/JSON) with validation, gridhammer CLI (run, replay, validate), and experiment registry with immutable run metadata. No protocol logic in TS."

run_phase "6" "Golden MCS Scenarios" \
  "Create golden heavy-duty MCS scenarios: peak arrival wave, ramp-rate stress, zero-power suspend, mid-session fault, site curtailment. Add explicit assertions. Ensure replay diffs are empty."

run_phase "7" "Replay & Regression Hardening" \
  "Add replay verification tooling and regression checks. Ensure identical artifacts across runs with same scenario hash and seed. Fail loudly on nondeterminism."

run_phase "8" "Read-only Visualization" \
  "Build optional read-only web UI consuming artifacts only. Visualize EVSE timelines, ChargingProfile diffs, and power curves. No live hooks."

echo
echo "======================================================"
echo "All GridHammer phases completed successfully."
echo "======================================================"
