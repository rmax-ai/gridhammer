#!/usr/bin/env bash
set -euo pipefail

# -------------------------------------------------------------------

# GridHammer orchestration loop

# Orchestrates iterations using LOOP_PROMPT.md.

# Engineering rules live in AGENTS.md. This script enforces process only.

# -------------------------------------------------------------------

if [ "$#" -lt 1 ]; then
  echo "Usage: $0 <goal>"
  exit 1
fi

GOAL="$*"

# Allow environment overrides for easier CI/debugging.
: "${MAX_ITERATIONS:=100}"
: "${TIMEOUT:=300}"
: "${LOG_DIR:=logs}"
: "${PROMPT_FILE:=LOOP_PROMPT.md}"

OPENCODE_BIN=$(command -v opencode || true)
: "${AGENT:=orchestrator}"

PROMPT_OVERRIDE_FILE="$LOG_DIR/LOOP_PROMPT.override.md"

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

mkdir -p "$LOG_DIR"

# -------------------------------------------------------------------

# Pre-flight checks (fail fast)

# -------------------------------------------------------------------

required_files=(
  "$PROMPT_FILE"
  "PROJECT_PROMPT.md"
  "PROJECT_SPEC.md"
  "MANIFEST.md"
  "ARCHITECTURE.md"
  "AGENTS.md"
  "RUNBOOK.md"
  "NEXT_STEPS.md"
)

for f in "${required_files[@]}"; do
  [[ -f "$f" ]] || {
    echo -e "${RED}Missing required file: $f${NC}"
    exit 1
  }
done

[[ -n "$OPENCODE_BIN" ]] || {
  echo -e "${RED}opencode not found${NC}"
  exit 1
}

command -v git >/dev/null 2>&1 || {
  echo -e "${RED}git not found${NC}"
  exit 1
}

git rev-parse --is-inside-work-tree >/dev/null 2>&1 || {
  echo -e "${RED}Not inside a git work tree${NC}"
  exit 1
}

BASELINE_GIT_STATUS=$(git status --porcelain)
if [[ -n "$BASELINE_GIT_STATUS" ]]; then
  echo -e "${YELLOW}Warning: repo has uncommitted changes before starting.${NC}"
  echo -e "${YELLOW}Loop will require returning to this baseline after each iteration.${NC}"
fi

# -------------------------------------------------------------------

# Helpers

# -------------------------------------------------------------------

git_clean_required() {
  local current
  current=$(git status --porcelain)
  if [[ "$current" != "$BASELINE_GIT_STATUS" ]]; then
    echo -e "${YELLOW}Warning: repository state changed and was not restored/committed.${NC}"
    echo -e "${YELLOW}Expected to match baseline git status after iteration.${NC}"
    echo -e "${YELLOW}Current status:${NC}"
    git status --short
    return 1
  fi
  return 0
}

run_ci() {
  local ci_log="$1"
  local ci_latest="$LOG_DIR/ci_failures_latest.txt"
  local status=0

  echo -e "${BLUE}Running CI checks...${NC}" | tee -a "$ci_log"
  echo "--- CI $(date -u +"%Y-%m-%dT%H:%M:%SZ") ---" >>"$ci_log"

  # ----------------------------------------------------------------
  # Rust Checks
  # ----------------------------------------------------------------
  if command -v cargo >/dev/null 2>&1; then
    local cargo_tomls
    cargo_tomls=$(find . -name Cargo.toml \
      -not -path "*/.git/*" \
      -not -path "*/.opencode/*" \
      -not -path "*/target/*" \
      -not -path "*/node_modules/*" \
      -print 2>/dev/null | sort)

    if [[ -n "$cargo_tomls" ]]; then
      while IFS= read -r cargo_file; do
        [[ -z "$cargo_file" ]] && continue
        local crate_dir
        crate_dir=$(dirname "$cargo_file")
        echo "[ci] Rust Crate: $crate_dir" | tee -a "$ci_log"

        set +e
        (
          cd "$crate_dir" || exit 2
          cargo fmt -- --check && cargo clippy -- -D warnings && cargo test
        ) >>"$ci_log" 2>&1
        local rust_rc=$?
        set -e
        if ((rust_rc != 0)); then
          echo "[ci] Rust checks failed for $crate_dir (rc=$rust_rc)" | tee -a "$ci_log"
          status=1
        fi
      done <<<"$cargo_tomls"
    else
      echo "[ci] No Rust crates found (skipping)" | tee -a "$ci_log"
    fi
  else
    echo "[ci] cargo not found (skipping Rust checks)" | tee -a "$ci_log"
  fi

  # ----------------------------------------------------------------
  # TypeScript/Node Checks
  # ----------------------------------------------------------------
  if command -v npm >/dev/null 2>&1; then
    local package_jsons
    package_jsons=$(find . -name package.json \
      -not -path "*/.git/*" \
      -not -path "*/.opencode/*" \
      -not -path "*/target/*" \
      -not -path "*/node_modules/*" \
      -print 2>/dev/null | sort)

    if [[ -n "$package_jsons" ]]; then
      while IFS= read -r pkg_file; do
        [[ -z "$pkg_file" ]] && continue
        local pkg_dir
        pkg_dir=$(dirname "$pkg_file")
        echo "[ci] Node Project: $pkg_dir" | tee -a "$ci_log"

        set +e
        (
          cd "$pkg_dir" || exit 2
          # Only run if scripts exist
          if grep -q '"test":' package.json; then
            npm test
          fi
          if grep -q '"lint":' package.json; then
            npm run lint
          fi
        ) >>"$ci_log" 2>&1
        local node_rc=$?
        set -e
        if ((node_rc != 0)); then
          echo "[ci] Node checks failed for $pkg_dir (rc=$node_rc)" | tee -a "$ci_log"
          status=1
        fi
      done <<<"$package_jsons"
    else
      echo "[ci] No Node projects found (skipping)" | tee -a "$ci_log"
    fi
  else
    echo "[ci] npm not found (skipping Node checks)" | tee -a "$ci_log"
  fi

  if ((status != 0)); then
    cp -f "$ci_log" "$ci_latest"
    return 1
  fi
  return 0
}

kill_child() {
  local pid="${CURRENT_PID:-}"
  [[ -z "$pid" ]] && return 0

  if [[ "$pid" =~ ^[0-9]+$ ]]; then
    # Try to kill the whole process group first (covers pipelines).
    kill -TERM -- "-$pid" 2>/dev/null || kill -TERM "$pid" 2>/dev/null || true
    sleep 1
    kill -KILL -- "-$pid" 2>/dev/null || kill -KILL "$pid" 2>/dev/null || true
  fi
}

trap kill_child EXIT INT TERM

echo -e "${BLUE}🔄 GridPulse loop started (max $MAX_ITERATIONS iterations)${NC}"
echo -e "${BLUE}📄 Prompt: $PROMPT_FILE${NC}"
echo -e "${BLUE}📂 Logs:   $LOG_DIR/${NC}"

START=$(date +%s)

# -------------------------------------------------------------------

# Main loop

# -------------------------------------------------------------------

for ((i = 1; i <= MAX_ITERATIONS; i++)); do
  ITER_START=$(date +%s)
  TS=$(date +"%Y%m%d_%H%M%S")
  LOG="$LOG_DIR/iter_${i}_${TS}.log"
  CI_LOG="$LOG_DIR/ci_${i}_${TS}.log"

  PROMPT_TO_USE="$PROMPT_FILE"
  if [[ -f "$PROMPT_OVERRIDE_FILE" ]]; then
    PROMPT_TO_USE="$PROMPT_OVERRIDE_FILE"
  fi

  # concise, deterministic instruction
  OPENCODE_GOAL="Execute one iteration using $PROMPT_TO_USE to progress toward: $GOAL"
  OPENCODE_CMD=(run --print-logs --agent "$AGENT" "$OPENCODE_GOAL" -f "$PROMPT_TO_USE")

  echo -e "\n${YELLOW}--- Iteration $i ---${NC}" | tee "$LOG"

  (
    # Avoid brittle failures with `pipefail`: `grep -v` returns 1 when it filters everything.
    # `sed` reliably returns 0 while still letting earlier pipeline failures propagate.
    "$OPENCODE_BIN" "${OPENCODE_CMD[@]}" 2>&1 | tee -a "$LOG" | sed '/INFO/d'
  ) &
  CURRENT_PID=$!

  # inactivity watchdog

  while kill -0 "$CURRENT_PID" 2>/dev/null; do
    sleep 5
    NOW=$(date +%s)

    if [[ "$OSTYPE" == "darwin"* ]]; then
      LAST=$(stat -f %m "$LOG" 2>/dev/null || echo "$NOW")
    else
      LAST=$(stat -c %Y "$LOG" 2>/dev/null || echo "$NOW")
    fi

    if ((NOW - LAST > TIMEOUT)); then
      echo -e "${RED}Timeout. Killing iteration.${NC}" | tee -a "$LOG"
      kill_child || true
      exit 1
    fi

  done

  set +e
  wait "$CURRENT_PID"
  ITER_RC=$?
  set -e
  CURRENT_PID=""

  if ((ITER_RC != 0)); then
    echo -e "${RED}Iteration process exited non-zero (rc=$ITER_RC). Continuing.${NC}" | tee -a "$LOG"
    DUR=$(($(date +%s) - ITER_START))
    echo -e "${YELLOW}Iteration $i ended early (${DUR}s)${NC}" | tee -a "$LOG"
    continue
  fi

  # -----------------------------------------------------------------

  # Post-iteration verification

  # -----------------------------------------------------------------

  if run_ci "$CI_LOG"; then
    rm -f "$PROMPT_OVERRIDE_FILE" 2>/dev/null || true
  else
    echo -e "${RED}CI failed. See $LOG_DIR/ci_failures_latest.txt${NC}" | tee -a "$LOG" | tee -a "$CI_LOG"
    cat >"$PROMPT_OVERRIDE_FILE" <<'EOF'
# LOOP PROMPT OVERRIDE — Fix CI failures

Your ONLY task this iteration is to fix the CI failures.

Read:
- logs/ci_failures_latest.txt

Then:
1) Fix the errors.
2) Re-run the CI commands from RUNBOOK.md (CI section).
3) Commit all changes.
4) Output `<promise>NEXT_TASK</promise>` when CI is green.

Do not work on NEXT_STEPS.md items until CI is green.
EOF

    # Do not enforce clean git or completion signals while CI is failing.
    DUR=$(($(date +%s) - ITER_START))
    echo -e "${YELLOW}Iteration $i ended with CI failures (${DUR}s)${NC}" | tee -a "$LOG"
    continue
  fi

  # Stop immediately when DONE is signaled, even if the repo is dirty.
  if grep -q "<promise>DONE</promise>" "$LOG"; then
    echo -e "${GREEN}✅ DONE signaled${NC}"
    break
  fi

  # Non-fatal: warn if repo state drifts from the starting baseline.
  git_clean_required || true

  if grep -q "<promise>NEXT_TASK</promise>" "$LOG"; then
    echo -e "${BLUE}➡ NEXT_TASK signaled${NC}"
  else
    echo -e "${YELLOW}No completion signal detected. Continuing.${NC}"
  fi

  DUR=$(($(date +%s) - ITER_START))
  echo -e "${GREEN}Iteration $i complete (${DUR}s)${NC}"

  # Push changes
  echo -e "${BLUE}Pushing changes to git...${NC}"
  git push
done

TOTAL=$(($(date +%s) - START))

echo -e "\n${BLUE}🏁 Finished in ${TOTAL}s${NC}"

if ((i > MAX_ITERATIONS)); then
  echo -e "${YELLOW}Reached iteration limit${NC}"
fi
