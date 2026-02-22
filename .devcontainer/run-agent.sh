#!/usr/bin/env bash
# Autonomous Claude Code agent loop for iscc-lib development.
# Runs consecutive claude -p calls, each advancing the project
# one small step towards the ideal state.
#
# Usage:
#   bash .devcontainer/run-agent.sh [iterations]
#
# Default: 20 iterations. Set to 0 for infinite loop.
# Logs are written to .devcontainer/agent-logs/
set -euo pipefail

MAX_ITERATIONS="${1:-20}"
LOG_DIR=".devcontainer/agent-logs"
mkdir -p "$LOG_DIR"

if [ ! -f "$HOME/.claude/.credentials.json" ]; then
  echo "ERROR: Claude credentials not found at $HOME/.claude/.credentials.json"
  echo "Make sure your host ~/.claude/.credentials.json is bind-mounted into the container."
  exit 1
fi

PROMPT='You are working on the iscc-lib project autonomously. Your goal is to advance it towards the ideal state described in .claude/context/ideal-state.md.

Instructions:
1. Read .claude/context/ideal-state.md and CLAUDE.md to understand the project goals and architecture.
2. Check the current state: run git log --oneline -20 to see recent progress, inspect existing files and tests.
3. Identify the single next small step that advances the project towards Milestone 1.
4. Implement that step with tests.
5. Run tests to verify correctness (cargo test).
6. If tests pass, commit with a descriptive message.
7. If tests fail, fix the issue before committing.
8. Stop after completing one meaningful step. Do not try to do everything at once.

Focus on Phase 0 (conformance baseline) and Phase 1 (core + Python bindings) as defined in notes/00-overview.md.
Work in small, verifiable increments. Each step should be independently correct and testable.
Consult the architecture notes in notes/ when making design decisions.'

echo "=== Starting autonomous agent loop ==="
echo "Max iterations: $MAX_ITERATIONS (0 = infinite)"
echo "Logs: $LOG_DIR/"
echo ""

iteration=0
while true; do
  iteration=$((iteration + 1))

  if [ "$MAX_ITERATIONS" -gt 0 ] && [ "$iteration" -gt "$MAX_ITERATIONS" ]; then
    echo "=== Reached max iterations ($MAX_ITERATIONS). Stopping. ==="
    break
  fi

  timestamp=$(date +%Y%m%d-%H%M%S)
  log_file="$LOG_DIR/run-${timestamp}-${iteration}.log"

  echo "--- Iteration $iteration ($(date)) ---"
  echo "Log: $log_file"

  if claude -p "$PROMPT" --dangerously-skip-permissions 2>&1 | tee "$log_file"; then
    echo ""
    echo "--- Iteration $iteration completed successfully ---"
  else
    echo ""
    echo "--- Iteration $iteration exited with error (continuing) ---"
  fi

  echo ""

  # Pause between iterations to avoid rate limiting
  sleep 10
done

echo "=== Agent loop complete. $iteration iterations run. ==="
