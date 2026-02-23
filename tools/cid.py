#!/usr/bin/env python3
# /// script
# requires-python = ">=3.10"
# dependencies = []
# ///
"""CID — Continuous Iterative Development orchestrator.

Runs Claude Code agents in a loop to iteratively advance the project toward its target state.
Each iteration executes four roles: update-state, define-next, advance, review.

Usage:
    uv run tools/cid.py status
    uv run tools/cid.py step --skip-permissions
    uv run tools/cid.py run --skip-permissions --max-iterations 5
    uv run tools/cid.py role update-state --skip-permissions
"""

import argparse
import json
import os
import shutil
import subprocess
import sys
import time
from datetime import datetime, timezone
from pathlib import Path

# CID agent roles executed in order per iteration
ROLES = ("update-state", "define-next", "advance", "review")

CONTEXT_DIR = Path(".claude/context")
STATE_FILE = CONTEXT_DIR / "state.md"
LOG_FILE = CONTEXT_DIR / "iterations.jsonl"
DONE_MARKER = "## Status: DONE"


# --- CLI discovery ---


def find_claude():
    """Find the claude executable, handling Windows .cmd wrapper.

    Returns the path to the claude executable, or None if not found.
    """
    if sys.platform == "win32":
        cmd_path = shutil.which("claude.cmd")
        if cmd_path:
            return cmd_path
    return shutil.which("claude")


def sanitize_env():
    """Create environment with API keys removed.

    Ensures subscription login is used instead of API billing.
    """
    env = os.environ.copy()
    env.pop("ANTHROPIC_API_KEY", None)
    env.pop("ANTHROPIC_AUTH_TOKEN", None)
    # Prevent nested session detection when invoking claude from within claude
    env.pop("CLAUDECODE", None)
    return env


# --- Agent invocation ---


def _process_output(stdout, role):
    """Process streaming JSON output from claude CLI.

    Returns (cost, turns, is_error) tuple.
    """
    cost = 0.0
    turns = 0
    is_error = False

    for line in iter(stdout.readline, ""):
        line = line.rstrip("\n")
        if not line:
            continue
        try:
            data = json.loads(line)
        except json.JSONDecodeError:
            continue

        msg_type = data.get("type")

        if msg_type == "assistant":
            for item in data.get("message", {}).get("content", []):
                if item.get("type") == "text":
                    text = item.get("text", "").strip()
                    if text:
                        preview = text[:200] + ("..." if len(text) > 200 else "")
                        print(f"  [{role}] {preview}")
        elif msg_type == "result":
            cost = data.get("total_cost_usd", 0)
            turns = data.get("num_turns", 0)
            is_error = data.get("is_error", False)

    return cost, turns, is_error


def log_entry(cwd, entry):
    """Append a single JSON line to the iteration log."""
    log_path = cwd / LOG_FILE
    log_path.parent.mkdir(parents=True, exist_ok=True)
    with open(log_path, "a", encoding="utf-8") as f:
        f.write(json.dumps(entry, separators=(",", ":")) + "\n")


def run_agent(claude_cmd, role, iteration, cwd, skip_permissions=False):
    """Invoke a CID agent role via claude CLI.

    Returns a dict with keys: ok, role, iteration, turns, cost_usd, duration_s, status.
    """
    prompt = f"CID iteration {iteration}. Execute your protocol."

    cmd = [
        claude_cmd,
        "-p",
        prompt,
        "--agent",
        role,
        "--output-format",
        "stream-json",
        "--verbose",
    ]
    if skip_permissions:
        cmd.append("--dangerously-skip-permissions")

    start = time.time()
    print(f"  Starting {role}...")

    try:
        flags = (
            getattr(subprocess, "CREATE_NO_WINDOW", 0) if sys.platform == "win32" else 0
        )
        process = subprocess.Popen(  # noqa: S603
            cmd,
            stdout=subprocess.PIPE,
            stderr=subprocess.STDOUT,
            text=True,
            cwd=cwd,
            env=sanitize_env(),
            bufsize=1,
            creationflags=flags,
        )
    except OSError as e:
        print(f"  ERROR: Failed to start claude: {e}")
        entry = {
            "ts": datetime.now(timezone.utc).isoformat(),
            "iteration": iteration,
            "role": role,
            "status": "ERROR",
            "turns": 0,
            "cost_usd": 0.0,
            "duration_s": 0.0,
            "error": str(e),
        }
        log_entry(cwd, entry)
        return {**entry, "ok": False}

    stdout = process.stdout  # guaranteed non-None by stdout=PIPE
    if stdout is None:  # pragma: no cover
        return {"ok": False, "role": role, "iteration": iteration}
    cost, turns, is_error = _process_output(stdout, role)
    process.wait()
    elapsed = time.time() - start

    status = "FAIL" if (process.returncode != 0 or is_error) else "OK"
    print(f"  {role} {status} ({turns} turns, ${cost:.4f}, {elapsed:.0f}s)")

    entry = {
        "ts": datetime.now(timezone.utc).isoformat(),
        "iteration": iteration,
        "role": role,
        "status": status,
        "turns": turns,
        "cost_usd": round(cost, 6),
        "duration_s": round(elapsed, 1),
    }
    log_entry(cwd, entry)

    return {**entry, "ok": status == "OK"}


# --- State inspection ---


def is_done(cwd):
    """Check if state.md indicates the project has reached its target."""
    state_path = cwd / STATE_FILE
    if not state_path.exists():
        return False
    return DONE_MARKER in state_path.read_text(encoding="utf-8")


def read_state_summary(cwd):
    """Read and return the contents of state.md."""
    state_path = cwd / STATE_FILE
    if not state_path.exists():
        return "(no state.md found — run 'cid step' to initialize)"
    return state_path.read_text(encoding="utf-8")


# --- Iteration logic ---


def run_iteration(claude_cmd, iteration, cwd, skip_permissions=False):
    """Run one CID iteration (all 4 roles in sequence).

    Returns 'done' if the project reached target, 'ok' if iteration succeeded,
    or 'fail' if a role failed.
    """
    for role in ROLES:
        print(f"\n{'─' * 60}")
        print(f"  CID Iteration {iteration} — {role}")
        print(f"{'─' * 60}")

        result = run_agent(claude_cmd, role, iteration, cwd, skip_permissions)

        if not result["ok"]:
            print(f"\n  *** {role} failed. Stopping iteration. ***")
            return "fail"

        # Check for DONE after update-state
        if role == "update-state" and is_done(cwd):
            print("\n  *** Target state reached! Project is DONE. ***")
            return "done"

    return "ok"


# --- CLI commands ---


def cmd_run(args):
    """Full CID loop: iterate until done or max iterations reached."""
    claude_cmd = find_claude()
    if not claude_cmd:
        print("ERROR: claude not found in PATH")
        sys.exit(1)

    cwd = Path(args.workdir).resolve()
    max_iter = args.max_iterations
    pause = args.pause

    print(f"CID run — max {max_iter} iterations")
    print(f"Working directory: {cwd}")
    if pause:
        print(f"Pause between iterations: {pause}s")
    print()

    for i in range(1, max_iter + 1):
        result = run_iteration(claude_cmd, i, cwd, args.skip_permissions)

        if result == "done":
            print(f"\nProject complete after {i} iteration(s).")
            break
        elif result == "fail":
            print(f"\nIteration {i} failed. Stopping.")
            sys.exit(1)

        if i < max_iter and pause:
            print(f"\nPausing {pause}s before next iteration...")
            time.sleep(pause)
    else:
        print(f"\nReached max iterations ({max_iter}).")


def cmd_step(args):
    """Single CID iteration."""
    claude_cmd = find_claude()
    if not claude_cmd:
        print("ERROR: claude not found in PATH")
        sys.exit(1)

    cwd = Path(args.workdir).resolve()
    result = run_iteration(claude_cmd, args.iteration, cwd, args.skip_permissions)

    if result == "fail":
        sys.exit(1)


def cmd_status(args):
    """Show current project state."""
    cwd = Path(args.workdir).resolve()
    print(read_state_summary(cwd))


def cmd_stats(args):
    """Show summary statistics from the iteration log."""
    cwd = Path(args.workdir).resolve()
    log_path = cwd / LOG_FILE

    if not log_path.exists():
        print("(no iteration log found — run a CID iteration first)")
        return

    entries = []
    for line in log_path.read_text(encoding="utf-8").splitlines():
        line = line.strip()
        if line:
            try:
                entries.append(json.loads(line))
            except json.JSONDecodeError:
                continue

    if not entries:
        print("(iteration log is empty)")
        return

    total_cost = sum(e.get("cost_usd", 0) for e in entries)
    total_duration = sum(e.get("duration_s", 0) for e in entries)
    total_turns = sum(e.get("turns", 0) for e in entries)
    ok_count = sum(1 for e in entries if e.get("status") == "OK")
    fail_count = len(entries) - ok_count
    iterations = {e.get("iteration") for e in entries}

    print(
        f"CID Iteration Log — {len(entries)} agent runs across {len(iterations)} iterations"
    )
    print(f"{'─' * 60}")
    print(f"  Total cost:     ${total_cost:.4f}")
    print(f"  Total duration: {total_duration:.0f}s ({total_duration / 3600:.1f}h)")
    print(f"  Total turns:    {total_turns}")
    print(f"  Success/Fail:   {ok_count}/{fail_count}")
    print()

    # Per-role breakdown
    role_stats = {}
    for e in entries:
        role = e.get("role", "unknown")
        if role not in role_stats:
            role_stats[role] = {
                "cost": 0.0,
                "duration": 0.0,
                "turns": 0,
                "runs": 0,
                "fails": 0,
            }
        rs = role_stats[role]
        rs["cost"] += e.get("cost_usd", 0)
        rs["duration"] += e.get("duration_s", 0)
        rs["turns"] += e.get("turns", 0)
        rs["runs"] += 1
        if e.get("status") != "OK":
            rs["fails"] += 1

    print(
        f"  {'Role':<14} {'Runs':>5} {'Fails':>6} {'Cost':>9} {'Duration':>10} {'Turns':>6}"
    )
    print(f"  {'─' * 14} {'─' * 5} {'─' * 6} {'─' * 9} {'─' * 10} {'─' * 6}")
    for role in ROLES:
        rs = role_stats.get(role)
        if not rs:
            continue
        print(
            f"  {role:<14} {rs['runs']:>5} {rs['fails']:>6} "
            f"${rs['cost']:>8.4f} {rs['duration']:>9.0f}s {rs['turns']:>6}"
        )
    # Show any roles not in ROLES (e.g., maintenance agents)
    for role, rs in role_stats.items():
        if role not in ROLES:
            print(
                f"  {role:<14} {rs['runs']:>5} {rs['fails']:>6} "
                f"${rs['cost']:>8.4f} {rs['duration']:>9.0f}s {rs['turns']:>6}"
            )

    # Last 5 entries
    print()
    print("  Recent entries:")
    for e in entries[-5:]:
        ts = e.get("ts", "?")[:19]
        role = e.get("role", "?")
        status = e.get("status", "?")
        cost = e.get("cost_usd", 0)
        duration = e.get("duration_s", 0)
        print(
            f"    {ts}  iter={e.get('iteration', '?')}  {role:<14} {status}  ${cost:.4f}  {duration:.0f}s"
        )


def cmd_role(args):
    """Run a single CID role (for testing/debugging)."""
    claude_cmd = find_claude()
    if not claude_cmd:
        print("ERROR: claude not found in PATH")
        sys.exit(1)

    cwd = Path(args.workdir).resolve()
    result = run_agent(
        claude_cmd, args.role, args.iteration, cwd, args.skip_permissions
    )

    if not result["ok"]:
        sys.exit(1)


# --- Entry point ---


def main():
    """Parse arguments and dispatch to the appropriate command."""
    parser = argparse.ArgumentParser(
        prog="cid",
        description="CID — Continuous Iterative Development orchestrator",
    )
    parser.add_argument(
        "--workdir",
        default=".",
        help="Project working directory (default: current directory)",
    )
    parser.add_argument(
        "--skip-permissions",
        action="store_true",
        default=False,
        help="Pass --dangerously-skip-permissions to claude",
    )

    sub = parser.add_subparsers(dest="command", required=True)

    # run
    run_p = sub.add_parser("run", help="Full CID loop until done or max iterations")
    run_p.add_argument(
        "--max-iterations",
        type=int,
        default=20,
        help="Maximum iterations (default: 20)",
    )
    run_p.add_argument(
        "--pause",
        type=int,
        default=600,
        help="Seconds to pause between iterations (default: 600)",
    )
    run_p.set_defaults(func=cmd_run)

    # step
    step_p = sub.add_parser("step", help="Run a single CID iteration (all 4 roles)")
    step_p.add_argument(
        "--iteration", type=int, default=1, help="Iteration number (default: 1)"
    )
    step_p.set_defaults(func=cmd_step)

    # status
    status_p = sub.add_parser("status", help="Show current project state from state.md")
    status_p.set_defaults(func=cmd_status)

    # stats
    stats_p = sub.add_parser("stats", help="Show iteration log summary statistics")
    stats_p.set_defaults(func=cmd_stats)

    # role (for testing individual agents)
    role_p = sub.add_parser(
        "role", help="Run a single CID role (for testing/debugging)"
    )
    role_p.add_argument("role", choices=ROLES, help="Which role to run")
    role_p.add_argument(
        "--iteration", type=int, default=1, help="Iteration number (default: 1)"
    )
    role_p.set_defaults(func=cmd_role)

    args = parser.parse_args()
    args.func(args)


if __name__ == "__main__":
    main()
