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

# NOTE: Phases 0-6 are complete. See git history or NEXT_STEPS.md.

run_phase "7" "Visualization (optional)" \
  "Build optional read-only web UI consuming artifacts only. Visualize EVSE timelines, ChargingProfile diffs, and power curves. Implement diff view for Run vs Run and Before vs After CPMS change. No live hooks."

run_phase "8" "Live CPMS Integration (Interactive)" \
  "Connect the deterministic core to real-world CPMS endpoints. Implement Gateway layer (WebSocket client, Async Runtime bridging), Real-time Clock Mode, and Interactive CLI ('live' command). Ensure protocol correctness in live mode."

echo
echo "======================================================"
echo "All GridHammer phases completed successfully."
echo "======================================================"
