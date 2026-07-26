#!/usr/bin/env python3
# /// script
# requires-python = ">=3.10"
# dependencies = []
# ///
"""CID — Continuous Iterative Development orchestrator.

Runs Claude Code agents in a loop to iteratively advance the project toward its target state.
Each iteration executes four roles: update-state, define-next, advance, review. Two roles run
outside the iteration sequence: meta-improve (on IDLE) and audit (every AUDIT_EVERY iterations
and on demand — whole-codebase maintainability audit that files evidence-backed issues).

Usage:
    uv run tools/cid.py status
    uv run tools/cid.py step --skip-permissions
    uv run tools/cid.py run --skip-permissions --max-iterations 5
    uv run tools/cid.py role update-state --skip-permissions
    uv run tools/cid.py audit --skip-permissions
"""

import argparse
import json
import os
import select
import shutil
import subprocess
import sys
import threading
import time
from datetime import datetime, timezone
from pathlib import Path

# CID agent roles executed in order per iteration
ROLES = ("update-state", "define-next", "advance", "review")

# Self-improvement role, run only on IDLE or via the `improve` command (not part of ROLES)
META_ROLE = "meta-improve"

# Codebase audit role, run on a fixed iteration cadence and via the `audit` command
# (not part of ROLES). Cadence rather than IDLE-coupling: debt accumulates fastest
# during busy phases, exactly when IDLE never happens.
AUDIT_ROLE = "audit"
AUDIT_EVERY = 10

CONTEXT_DIR = Path(".claude/context")
STATE_FILE = CONTEXT_DIR / "state.md"
HANDOFF_FILE = CONTEXT_DIR / "handoff.md"
LOG_FILE = CONTEXT_DIR / "iterations.jsonl"
META_LOG_FILE = CONTEXT_DIR / "meta-log.jsonl"
PROPOSALS_FILE = CONTEXT_DIR / "proposals.md"
AGENTS_DIR = Path(".claude/agents")
META_MEMORY_DIR = Path(".claude/agent-memory/meta-improve")
AUDIT_MEMORY_DIR = Path(".claude/agent-memory/audit")
ISSUES_FILE = CONTEXT_DIR / "issues.md"
METRICS_FILE = CONTEXT_DIR / "metrics.jsonl"
DONE_MARKER = "## Status: DONE"

# Hard enforcement of the meta-improve guardrails (the agent's prose rules are not
# enforcement — it runs with Edit/Write/Bash). Paths use posix separators to match
# git's output on every platform. Files the meta role MAY auto-edit:
META_AUTO_OK = frozenset(
    p.as_posix()
    for p in (
        AGENTS_DIR / "update-state.md",
        AGENTS_DIR / "define-next.md",
        AGENTS_DIR / "advance.md",  # NOT review.md — the reviewer is the quality gate
        CONTEXT_DIR / "learnings.md",
    )
)
# Bookkeeping the meta role writes freely (never counts as "a change"):
META_BOOKKEEPING = frozenset(
    p.as_posix() for p in (META_LOG_FILE, PROPOSALS_FILE, HANDOFF_FILE)
)
# The audit role's only output channels — findings go to issues.md, the runner
# snapshots metrics. Enforced by enforce_audit_safety in the trusted runner: the
# audit agent must never change source code, prompts, or other context files.
AUDIT_OK = frozenset(p.as_posix() for p in (ISSUES_FILE, METRICS_FILE))
# Upper bound on the agent-supplied rollback window — the runner owns this schedule,
# so a meta change can never defer its own evaluation indefinitely.
META_WINDOW_CAP = 10
HUMAN_REVIEW_MARKER = "**HUMAN REVIEW REQUESTED**"
IDLE_MARKER = "**IDLE**"
VERDICT_MARKER = "**Verdict:**"

# Canonical schema for a per-role log row. cid.py is the sole writer of the log;
# rows missing these keys are legacy/foreign and ignored by accounting helpers.
ROLE_ENTRY_KEYS = frozenset(
    {"ts", "iteration", "role", "status", "turns", "cost_usd", "duration_s"}
)

# Per-role wall-clock timeout in seconds — the loop's runaway guard. Enforced in
# run_agent by killing the claude subprocess if it overruns (cross-platform). A
# turn-count flag is intentionally NOT used: the installed claude CLI has no
# --max-turns, so it would be silently ignored. Values are generous (these catch
# stuck runs, not normal long iterations like advance + a 30-min codex review).
# A role that already committed its deliverable when the guard fires does not fail
# the iteration — see _role_commit_landed.
# advance runs on Fable 5, whose single requests on hard tasks can run for many
# minutes, so it gets extra headroom over the other roles.
ROLE_TIMEOUT_S = {
    "update-state": 1200,
    "define-next": 1200,
    "advance": 3600,
    "review": 3000,
    META_ROLE: 1800,
    # Whole-codebase sweep with a parallel finder/verifier workflow — long but
    # rare (every AUDIT_EVERY iterations).
    AUDIT_ROLE: 3600,
}


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
            # Coerce: an explicit null (key present, value None) survives .get's
            # default, so a null cost/turns would crash round()/format downstream.
            raw_cost = data.get("total_cost_usd")
            raw_turns = data.get("num_turns")
            cost = raw_cost if isinstance(raw_cost, (int, float)) else 0.0
            turns = raw_turns if isinstance(raw_turns, int) else 0
            is_error = bool(data.get("is_error", False))

    return cost, turns, is_error


def log_entry(cwd, entry):
    """Append a single JSON line to the iteration log."""
    log_path = cwd / LOG_FILE
    log_path.parent.mkdir(parents=True, exist_ok=True)
    with open(log_path, "a", encoding="utf-8") as f:
        f.write(json.dumps(entry, separators=(",", ":")) + "\n")


def read_entries(cwd):
    """Read all parseable JSON rows from the iteration log.

    Returns a list of dicts. Malformed lines are skipped silently.
    """
    log_path = cwd / LOG_FILE
    if not log_path.exists():
        return []
    entries = []
    for line in log_path.read_text(encoding="utf-8").splitlines():
        line = line.strip()
        if not line:
            continue
        try:
            entries.append(json.loads(line))
        except json.JSONDecodeError:
            continue
    return entries


def is_role_entry(entry):
    """Return True if an entry matches the canonical per-role log schema.

    Legacy and foreign rows (different schema) are excluded from accounting.
    """
    return ROLE_ENTRY_KEYS.issubset(entry.keys()) and entry.get("role") in (
        *ROLES,
        META_ROLE,
        "orchestrator",
    )


def _num(entry, key):
    """Return entry[key] as a number, treating null/missing/non-numeric as 0.

    Legacy rows sometimes carry an explicit ``null`` metric, which ``dict.get``
    returns verbatim — so accounting must coerce rather than rely on a default.
    """
    value = entry.get(key)
    return value if isinstance(value, (int, float)) else 0


def last_iteration(cwd):
    """Return the highest iteration number recorded in the log (0 if none).

    Used to continue numbering across separate runs instead of resetting to 1.
    """
    last = 0
    for entry in read_entries(cwd):
        try:
            last = max(last, int(entry.get("iteration", 0)))
        except (TypeError, ValueError):
            continue
    return last


def parse_verdict(cwd):
    """Extract the latest review verdict from handoff.md.

    Returns one of PASS, PASS_WITH_NOTES, NEEDS_WORK, IDLE, or None if absent.
    """
    handoff_path = cwd / HANDOFF_FILE
    if not handoff_path.exists():
        return None
    text = handoff_path.read_text(encoding="utf-8")
    if IDLE_MARKER in text:
        return "IDLE"
    for line in text.splitlines():
        if VERDICT_MARKER in line:
            tail = line.split(VERDICT_MARKER, 1)[1].strip()
            for verdict in ("PASS_WITH_NOTES", "NEEDS_WORK", "PASS"):
                if verdict in tail:
                    return verdict
    return None


def append_iteration_summary(cwd, iteration):
    """Append a clean per-iteration summary row aggregating this iteration's roles.

    Provides a stable metric series (verdict + totals) for meta-improve to evaluate.
    """
    rows = [
        e
        for e in read_entries(cwd)
        if is_role_entry(e) and e.get("iteration") == iteration
    ]
    summary = {
        "ts": datetime.now(timezone.utc).isoformat(),
        "type": "iteration_summary",
        "iteration": iteration,
        "verdict": parse_verdict(cwd) or "UNKNOWN",
        "turns_total": sum(_num(e, "turns") for e in rows),
        "cost_total": round(sum(_num(e, "cost_usd") for e in rows), 6),
        "duration_total": round(sum(_num(e, "duration_s") for e in rows), 1),
    }
    log_entry(cwd, summary)


def commit_path(cwd, rel_path, message):
    """Commit a single tracked file as a standalone, best-effort change.

    The pathspec form commits only that file, leaving any other staged changes
    untouched. Failures (nothing to commit, not a git repo) are non-fatal so they
    never abort the loop. cid.py owns iterations.jsonl/meta-log.jsonl this way.
    """
    rel = str(rel_path)
    try:
        add = subprocess.run(  # noqa: S603
            ["git", "add", rel],  # noqa: S607
            cwd=cwd,
            capture_output=True,
            text=True,
            check=False,
        )
        if add.returncode != 0:
            return
        status = subprocess.run(  # noqa: S603
            ["git", "diff", "--cached", "--quiet", "--", rel],  # noqa: S607
            cwd=cwd,
            check=False,
        )
        if status.returncode == 0:
            return  # nothing staged for this file
        subprocess.run(  # noqa: S603
            ["git", "commit", "-m", message, rel],  # noqa: S607
            cwd=cwd,
            capture_output=True,
            text=True,
            check=False,
        )
    except OSError as e:
        print(f"  WARN: could not commit {rel}: {e}")


def commit_log(cwd, iteration):
    """Commit the iteration log (cid.py is its sole owner; agents never stage it)."""
    commit_path(cwd, LOG_FILE, f"cid(log): iteration {iteration}")


# --- Meta-improve safety (enforced in the trusted runner, not the agent prompt) ---


def _git(cwd, *args):
    """Run a git command, returning (returncode, stdout). Never raises on git error."""
    try:
        proc = subprocess.run(  # noqa: S603
            ["git", *args],  # noqa: S607
            cwd=cwd,
            capture_output=True,
            text=True,
            check=False,
        )
    except OSError as e:
        return 1, str(e)
    return proc.returncode, proc.stdout


def _commits_since(cwd, base_sha):
    """Return commit SHAs in base_sha..HEAD, oldest first (empty if base is HEAD)."""
    rc, out = _git(cwd, "rev-list", "--reverse", f"{base_sha}..HEAD")
    if rc != 0:
        return []
    return [line.strip() for line in out.splitlines() if line.strip()]


def _role_commit_landed(cwd, role, base_sha):
    """True if the role's own `cid(<role>):` commit landed during its run.

    Every role protocol ends by committing its deliverable, so such a commit in
    base_sha..HEAD means the work is complete even if the subprocess was cut short
    afterwards (wall-clock timeout, API error) during optional follow-up such as
    memory housekeeping. Lets the loop tell a genuinely stuck role apart from one
    that finished and then overran.
    """
    if not base_sha:
        return False
    rc, out = _git(cwd, "log", "--format=%s", f"{base_sha}..HEAD")
    if rc != 0:
        return False
    return any(line.startswith(f"cid({role}):") for line in out.splitlines())


def _commit_files(cwd, sha):
    """Return the list of files changed by a commit (posix paths)."""
    rc, out = _git(
        cwd, "show", "--no-commit-id", "--name-only", "--pretty=format:", sha
    )
    if rc != 0:
        return []
    return [line.strip() for line in out.splitlines() if line.strip()]


def _is_meta_bookkeeping(path):
    """True if a path is meta-improve bookkeeping (log/proposals/handoff/own memory)."""
    return path in META_BOOKKEEPING or path.startswith(META_MEMORY_DIR.as_posix() + "/")


def _is_audit_output(path):
    """True if a path is a permitted audit-role output (issues/metrics/own memory)."""
    return path in AUDIT_OK or path.startswith(AUDIT_MEMORY_DIR.as_posix() + "/")


def _paths_dirty(cwd, paths):
    """True if any of the given tracked paths has uncommitted (staged or unstaged) changes.

    A revert refuses to start when the target commit's own files are dirty, so this
    pre-check lets _safe_revert tell that refusal apart from a genuine empty no-op.
    """
    if not paths:
        return False
    rc, out = _git(cwd, "status", "--porcelain", "--", *paths)
    return rc == 0 and bool(out.strip())


def _safe_revert(cwd, commit):
    """Revert a commit. Returns True iff the commit's change is no longer in effect.

    Three non-clean outcomes are distinguished (the tests pin git's behavior for each):
    a real merge conflict leaves REVERT_HEAD set — abort and report failure rather than
    commit over conflict markers. A non-zero exit with no REVERT_HEAD and a clean
    precondition is an empty no-op revert (the diff was already undone) — a success, the
    tree is already correct. But git also *refuses to start* a revert when the commit's
    own files carry uncommitted local changes; that refusal leaves the bad commit live,
    so it must report failure, not success. Pre-checking only the commit's own paths
    lets an unrelated dirty file (which git tolerates) still revert.
    """
    if _paths_dirty(cwd, _commit_files(cwd, commit)):
        return False  # revert would refuse to start; the commit would stay live
    rc, _ = _git(cwd, "revert", "--no-edit", commit)
    if rc == 0:
        return True
    if _git(cwd, "rev-parse", "-q", "--verify", "REVERT_HEAD")[0] == 0:
        _git(cwd, "revert", "--abort")
        return False  # genuine conflict — REVERT_HEAD is set
    return True  # no revert in progress and tree clean for these paths — empty no-op


def flag_human_review(cwd, reason):
    """Prepend a HUMAN REVIEW REQUESTED marker to handoff.md and commit it.

    Gives the runner's safety paths a durable, loop-visible signal (check_human_review
    reads handoff.md) when they cannot safely auto-resolve a situation.
    """
    handoff = cwd / HANDOFF_FILE
    try:
        existing = handoff.read_text(encoding="utf-8") if handoff.exists() else ""
        handoff.parent.mkdir(parents=True, exist_ok=True)
        handoff.write_text(
            f"> {HUMAN_REVIEW_MARKER}: {reason}\n\n{existing}", encoding="utf-8"
        )
    except OSError as e:
        print(f"  WARN: could not flag human review: {e}")
        return
    commit_path(cwd, HANDOFF_FILE, "cid(meta): flag human review (safety)")


def _history_intact(cwd, base_sha):
    """True if base_sha is still an ancestor of HEAD (no amend/reset rewrote it).

    Guards against operating on a rewritten range, which would otherwise revert
    productive work folded into an amended base commit.
    """
    rc, _ = _git(cwd, "merge-base", "--is-ancestor", base_sha, "HEAD")
    return rc == 0


def _has_apply_record(cwd, sha):
    """True if meta-log.jsonl carries a well-formed `apply` record for this commit.

    A surviving auto-applied change must be measurable: the rollback evaluator only
    ever inspects commits named by an `apply` record, so a whitelisted change with no
    (or a mangled) record would never be measured or reverted on regression. Requiring
    the record here closes that gap — an unrecorded change is unsafe to keep.
    """
    log_path = cwd / META_LOG_FILE
    if not log_path.exists():
        return False
    for line in log_path.read_text(encoding="utf-8").splitlines():
        line = line.strip()
        if not line:
            continue
        try:
            rec = json.loads(line)
        except json.JSONDecodeError:
            continue
        if (
            rec.get("action") == "apply"
            and rec.get("commit") == sha
            and "iteration" in rec
        ):
            return True
    return False


def enforce_meta_safety(cwd, base_sha):
    """Revert any meta commit that breaks the guardrails. Returns reverted SHA count.

    Inspects commits added since base_sha and reverts (in the trusted runner, which
    the agent cannot override) any that touch a path outside the auto-edit whitelist
    or that exceed the one-change-per-cycle cap. Pure-bookkeeping commits are exempt.
    The runner never pushes meta commits, so reverts stay local for human review.
    """
    if not base_sha:
        return 0  # no prior HEAD to diff against
    if not _history_intact(cwd, base_sha):
        # The agent rewrote history (amend/reset) — reverting the rewritten range
        # would clobber productive work. Refuse and escalate instead.
        flag_human_review(
            cwd,
            "meta-safety: git history was rewritten since the agent started "
            "(amend/reset); refusing to auto-revert — resolve manually",
        )
        print("  *** meta-safety: history rewritten — refusing to auto-revert ***")
        return 0
    reverted = 0
    allowed_changes = 0
    # Newest first so each `git revert` applies cleanly on top of HEAD.
    for sha in reversed(_commits_since(cwd, base_sha)):
        files = _commit_files(cwd, sha)
        if not files or all(_is_meta_bookkeeping(f) for f in files):
            continue
        offending = [
            f for f in files if not _is_meta_bookkeeping(f) and f not in META_AUTO_OK
        ]
        reason = None
        if offending:
            reason = f"touched protected path(s): {', '.join(offending)}"
        elif not _has_apply_record(cwd, sha):
            # A whitelisted change with no apply record is unmeasurable — the rollback
            # evaluator would never see it, so it can never be validated or reverted.
            reason = "no apply record in meta-log.jsonl — change would be unmeasurable"
        elif allowed_changes >= 1:
            reason = "exceeds the one-change-per-cycle cap"
        else:
            allowed_changes += 1  # the single kept change this cycle
        if reason:
            print(f"  *** meta-safety: reverting {sha[:8]} — {reason} ***")
            if _safe_revert(cwd, sha):
                reverted += 1
            else:
                flag_human_review(
                    cwd,
                    f"meta-safety could not auto-revert {sha[:8]} ({reason}) — "
                    "git revert conflict; the unreverted change is live, resolve manually",
                )
                print(
                    f"  *** meta-safety: could not auto-revert {sha[:8]} — "
                    "HUMAN REVIEW REQUIRED (git revert conflict) ***"
                )
    return reverted


def enforce_audit_safety(cwd, base_sha):
    """Revert any audit commit that touches files outside AUDIT_OK. Returns count.

    The audit role's charter is find-and-file only: issues.md, the metrics log, and
    its own memory. Any commit since base_sha touching other paths (source code,
    prompts, context files) is reverted in the trusted runner, which the agent cannot
    override. Simpler than the meta guardrails on purpose — no change budget or
    rollback window, because a compliant audit changes no behavior at all.
    """
    if not base_sha:
        return 0  # no prior HEAD to diff against
    if not _history_intact(cwd, base_sha):
        flag_human_review(
            cwd,
            "audit-safety: git history was rewritten since the agent started "
            "(amend/reset); refusing to auto-revert — resolve manually",
        )
        print("  *** audit-safety: history rewritten — refusing to auto-revert ***")
        return 0
    reverted = 0
    # Newest first so each `git revert` applies cleanly on top of HEAD.
    for sha in reversed(_commits_since(cwd, base_sha)):
        files = _commit_files(cwd, sha)
        offending = [f for f in files if not _is_audit_output(f)]
        if not files or not offending:
            continue
        print(
            f"  *** audit-safety: reverting {sha[:8]} — touched "
            f"non-audit path(s): {', '.join(offending)} ***"
        )
        if _safe_revert(cwd, sha):
            reverted += 1
        else:
            flag_human_review(
                cwd,
                f"audit-safety could not auto-revert {sha[:8]} (touched "
                f"{', '.join(offending)}) — git revert conflict; the unreverted "
                "change is live, resolve manually",
            )
            print(
                f"  *** audit-safety: could not auto-revert {sha[:8]} — "
                "HUMAN REVIEW REQUIRED (git revert conflict) ***"
            )
    return reverted


def _abandoned_paths(cwd, is_role_output):
    """Tracked files with uncommitted changes that are neither the iteration log nor
    permitted role output — edits an interrupted role run left behind.

    The iteration log is runner-owned (committed by commit_log) and role output is the
    agent's own scratch, so both are excluded; what remains is abandoned edits.
    """
    rc, out = _git(cwd, "status", "--porcelain", "--untracked-files=no")
    if rc != 0:
        return []
    paths = []
    for line in out.splitlines():
        # porcelain v1: "XY <path>"; a rename shows "XY old -> new".
        entry = line[3:].strip() if len(line) > 3 else ""
        if not entry:
            continue
        path = entry.split(" -> ")[-1]  # defensively take a rename's target
        if path == LOG_FILE.as_posix() or is_role_output(path):
            continue
        paths.append(path)
    return paths


def _stash_abandoned_edits(cwd, role, is_role_output):
    """Stash out-of-charter edits a role run left uncommitted, then flag human review.

    Guarded roles (meta-improve, audit) commit their own work; uncommitted edits to
    other files after they exit are abandoned (e.g. an interrupted or timed-out run)
    and would otherwise leak into the next run unreviewed — as a silently modified
    prompt (meta) or a dirty source tree the advance agent must not reset (audit). A
    successful run leaves the tree clean, so this is a no-op then. Stashing returns
    the tree to the validated committed state while keeping the edits recoverable via
    `git stash pop`. Returns the stashed paths.
    """
    paths = _abandoned_paths(cwd, is_role_output)
    if not paths:
        return []
    listing = ", ".join(paths)
    rc, _ = _git(
        cwd,
        "stash",
        "push",
        "-m",
        f"cid({role}): abandoned uncommitted edits",
        "--",
        *paths,
    )
    if rc != 0:
        flag_human_review(
            cwd,
            f"{role} left uncommitted edits to {listing} that could not be "
            "stashed — an unreviewed change is live, resolve manually",
        )
        print(f"  *** {role}-safety: could not stash abandoned edits: {listing} ***")
        return []
    flag_human_review(
        cwd,
        f"{role} left uncommitted edits to {listing} (likely interrupted); "
        "stashed them (see `git stash list`) and restored the tree to HEAD — review "
        "before the next run",
    )
    print(f"  *** {role}-safety: stashed abandoned uncommitted edits: {listing} ***")
    return paths


def _verdict_series(cwd):
    """Return (iteration, verdict, turns_total) tuples from iteration_summary rows."""
    series = []
    for e in read_entries(cwd):
        if e.get("type") == "iteration_summary":
            series.append(
                (int(e.get("iteration", 0)), e.get("verdict"), _num(e, "turns_total"))
            )
    series.sort(key=lambda t: t[0])
    return series


def _window_metrics(series, lo, hi):
    """Compute (needs_work_rate, median_turns) over iterations in (lo, hi]."""
    rows = [t for t in series if lo < t[0] <= hi]
    if not rows:
        return None
    nw = sum(1 for _, v, _ in rows if v == "NEEDS_WORK") / len(rows)
    turns = sorted(t[2] for t in rows)
    mid = len(turns) // 2
    median = turns[mid] if len(turns) % 2 else (turns[mid - 1] + turns[mid]) / 2
    return nw, median


def evaluate_meta_rollbacks(cwd):
    """Auto-revert meta changes whose post-change window shows a regression.

    Trusted-runner side of the rollback contract: for each pending `apply` record in
    meta-log.jsonl whose evaluation window has elapsed, compute the metric before and
    after the change from the durable iteration_summary series and revert if it
    regressed. The runner computes the metric itself — it never trusts an
    agent-supplied baseline. Pending changes with too little data are left untouched.

    Returns True if a regressed change could NOT be auto-reverted (git revert conflict)
    and human review was flagged — the caller must pause rather than build on top of a
    live, unreverted regression.
    """
    log_path = cwd / META_LOG_FILE
    if not log_path.exists():
        return False
    series = _verdict_series(cwd)
    latest = series[-1][0] if series else 0
    evaluated = set()  # commits already resolved (rolled back or confirmed)
    pending = []
    for line in log_path.read_text(encoding="utf-8").splitlines():
        line = line.strip()
        if not line:
            continue
        try:
            rec = json.loads(line)
        except json.JSONDecodeError:
            continue
        action, commit = rec.get("action"), rec.get("commit")
        if action in ("rollback", "evaluated") and commit:
            evaluated.add(commit)
        elif action == "apply" and commit:
            pending.append(rec)

    blocked = False
    for rec in pending:
        commit = rec["commit"]
        if commit in evaluated:
            continue
        applied_at = int(rec.get("iteration", 0))
        # The window is a runner-owned schedule, not agent input — clamp it so a
        # change can never defer its own evaluation indefinitely (window=9999).
        window = max(1, min(int(rec.get("window", 5)), META_WINDOW_CAP))
        if latest - applied_at < window:
            continue  # not enough post-change iterations to judge yet
        before = _window_metrics(series, applied_at - window, applied_at)
        after = _window_metrics(series, applied_at, applied_at + window)
        if not before or not after:
            continue
        regressed = after[0] > before[0] or after[1] > before[1] * 1.25
        if regressed:
            print(f"  *** meta-rollback: reverting {commit[:8]} (metric regressed) ***")
            if not _safe_revert(cwd, commit):
                # Conflict — do NOT record success or dedup it; flag, signal the caller
                # to pause, and retry on a later run.
                flag_human_review(
                    cwd,
                    f"meta-rollback: regressed change {commit[:8]} could not be "
                    "reverted (git revert conflict) — resolve manually",
                )
                blocked = True
                continue
            result = "rollback"
        else:
            print(f"  meta-rollback: {commit[:8]} held (no regression)")
            result = "evaluated"
        # Record the resolution in meta-log.jsonl (the runner is a trusted writer).
        with open(log_path, "a", encoding="utf-8") as f:
            f.write(
                json.dumps(
                    {
                        "ts": datetime.now(timezone.utc).isoformat(),
                        "action": result,
                        "commit": commit,
                        "before": before,
                        "after": after,
                    },
                    separators=(",", ":"),
                )
                + "\n"
            )
        commit_path(cwd, META_LOG_FILE, f"cid(meta): rollback eval for {commit[:8]}")
    return blocked


def build_agent_cmd(claude_cmd, role, prompt, skip_permissions, fallback_model=None):
    """Assemble the claude CLI argument list for a headless agent run.

    Adds prompt-cache friendliness (--exclude-dynamic-system-prompt-sections) and an
    optional fallback model. The runaway guard is a wall-clock timeout enforced in
    run_agent, not a CLI flag — the installed claude CLI has no --max-turns.
    """
    cmd = [
        claude_cmd,
        "-p",
        prompt,
        "--agent",
        role,
        "--output-format",
        "stream-json",
        "--verbose",
        # Stable system-prompt prefix across the 4+ invocations per iteration
        "--exclude-dynamic-system-prompt-sections",
    ]
    if fallback_model:
        cmd += ["--fallback-model", fallback_model]
    if skip_permissions:
        cmd.append("--dangerously-skip-permissions")
    return cmd


def run_agent(
    claude_cmd,
    role,
    iteration,
    cwd,
    skip_permissions=False,
    fallback_model=None,
    base_sha=None,
):
    """Invoke a CID agent role via claude CLI.

    Returns a dict with keys: ok, role, iteration, turns, cost_usd, duration_s, status.
    Pass base_sha (HEAD before the run) to recognise a run that committed its
    deliverable and only then overran or errored: the logged status stays honest but
    `ok` is True, so the loop continues instead of discarding completed work.
    """
    prompt = f"CID iteration {iteration}. Execute your protocol."

    cmd = build_agent_cmd(
        claude_cmd, role, prompt, skip_permissions, fallback_model=fallback_model
    )

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

    # Wall-clock runaway guard: kill the subprocess if the role overruns its budget.
    # Cross-platform (Timer + process.kill work on POSIX and Windows).
    timed_out = {"v": False}
    timeout_s = ROLE_TIMEOUT_S.get(role)
    timer = None
    if timeout_s:

        def _kill_on_timeout():
            timed_out["v"] = True
            try:
                process.kill()
            except OSError:
                pass  # already exited (race with normal completion)

        timer = threading.Timer(timeout_s, _kill_on_timeout)
        timer.start()
    try:
        cost, turns, is_error = _process_output(stdout, role)
        process.wait()
    finally:
        if timer:
            timer.cancel()
    elapsed = time.time() - start

    if timed_out["v"]:
        status = "TIMEOUT"
    elif process.returncode != 0 or is_error:
        status = "FAIL"
    else:
        status = "OK"
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
    recovered = status != "OK" and _role_commit_landed(cwd, role, base_sha)
    if recovered:
        entry["recovered"] = True
        print(f"  {role} committed its deliverable before the {status} — continuing")
        leftovers = _abandoned_paths(cwd, lambda _p: False)
        if leftovers:
            print(f"  NOTE: {role} left uncommitted edits: {', '.join(leftovers)}")
    log_entry(cwd, entry)

    return {**entry, "ok": status == "OK" or recovered}


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


def check_human_review(cwd):
    """Check if handoff.md contains a human review request.

    Returns the reason string if found, None otherwise.
    """
    handoff_path = cwd / HANDOFF_FILE
    if not handoff_path.exists():
        return None
    for line in handoff_path.read_text(encoding="utf-8").splitlines():
        if HUMAN_REVIEW_MARKER in line:
            # Extract reason after the marker (strip blockquote prefix and colon)
            reason = line.split(HUMAN_REVIEW_MARKER, 1)[1].lstrip(":").strip()
            return reason or "(no reason given)"
    return None


def check_idle(cwd):
    """Check if handoff.md indicates no actionable work remains.

    Returns the reason string if found, None otherwise.
    """
    handoff_path = cwd / HANDOFF_FILE
    if not handoff_path.exists():
        return None
    for line in handoff_path.read_text(encoding="utf-8").splitlines():
        if IDLE_MARKER in line:
            reason = line.split(IDLE_MARKER, 1)[1].lstrip(":").strip()
            return reason or "no actionable work remains"
    return None


# --- Iteration logic ---


def _classify_outcome(cwd):
    """Determine the iteration outcome after the review role completes.

    Returns 'pause' (human review requested), 'idle' (no actionable work), or 'ok'.
    """
    reason = check_human_review(cwd)
    if reason:
        print(f"\n  *** HUMAN REVIEW REQUESTED: {reason} ***")
        print("  Pausing CID loop. Review handoff.md, then resume with cid:run.")
        return "pause"
    idle_reason = check_idle(cwd)
    if idle_reason:
        print(f"\n  *** IDLE: {idle_reason} ***")
        return "idle"
    return "ok"


def run_iteration(
    claude_cmd, iteration, cwd, skip_permissions=False, fallback_model=None
):
    """Run one CID iteration (all 4 roles in sequence).

    Returns 'done' if the project reached target, 'idle' if no actionable work remains,
    'ok' if iteration succeeded, 'pause' if human review was requested, or 'fail' if a
    role failed. Always records an iteration summary and commits the log on a
    completed iteration.
    """
    result = "ok"
    for role in ROLES:
        print(f"\n{'─' * 60}")
        print(f"  CID Iteration {iteration} — {role}")
        print(f"{'─' * 60}")

        _, pre_head = _git(cwd, "rev-parse", "HEAD")
        outcome = run_agent(
            claude_cmd,
            role,
            iteration,
            cwd,
            skip_permissions,
            fallback_model,
            base_sha=pre_head.strip(),
        )

        if not outcome["ok"]:
            print(f"\n  *** {role} failed. Stopping iteration. ***")
            result = "fail"
            break

        # Check for DONE after update-state — no further roles run this iteration
        if role == "update-state" and is_done(cwd):
            print("\n  *** Target state reached! Project is DONE. ***")
            result = "done"
            break

    if result not in ("fail", "done"):
        result = _classify_outcome(cwd)
        append_iteration_summary(cwd, iteration)

    commit_log(cwd, iteration)
    return result


def wait_with_skip(seconds):
    """Sleep for the given duration, but return early if Enter is pressed.

    Uses select() on stdin for cross-platform support (Linux/macOS).
    Falls back to plain sleep on Windows where select() on stdin is unavailable.
    """
    if sys.platform == "win32":
        import msvcrt

        deadline = time.monotonic() + seconds
        while time.monotonic() < deadline:
            if msvcrt.kbhit():
                msvcrt.getwch()
                return
            time.sleep(0.25)
    else:
        ready, _, _ = select.select([sys.stdin], [], [], seconds)
        if ready:
            sys.stdin.readline()


# --- Audit role ---


def audit_due(iteration, every=AUDIT_EVERY):
    """True if the audit cadence fires after this completed iteration."""
    return every > 0 and iteration > 0 and iteration % every == 0


def run_metrics_snapshot(cwd, time_gates=True):
    """Append a gate-timed codebase-health snapshot and commit the metrics log.

    Runner-side so the snapshot exists deterministically before the audit agent
    starts — the agent reads trends, it never produces the data. Failures are
    reported but non-fatal: a missed snapshot must not block the audit.
    """
    script = Path(__file__).resolve().parent / "metrics.py"
    cmd = [sys.executable, str(script)]
    if time_gates:
        cmd.append("--time-gates")
    try:
        # Fixed argv (this repo's own script), no shell — safe subprocess use.
        proc = subprocess.run(  # noqa: S603
            cmd,
            cwd=cwd,
            env=sanitize_env(),
            timeout=3600,
            check=False,
        )
        ok = proc.returncode == 0
    except (OSError, subprocess.TimeoutExpired) as e:
        print(f"  WARN: metrics snapshot failed: {e}")
        ok = False
    if ok:
        commit_path(cwd, METRICS_FILE, "cid(audit): metrics snapshot")
    return ok


def run_audit(claude_cmd, iteration, cwd, skip_permissions=False, fallback_model=None):
    """Run the codebase audit role once: metrics snapshot, agent, safety enforcement.

    The audit agent reads the whole codebase plus metric trends and files at most a
    handful of evidence-backed maintainability issues. The trusted runner reverts any
    commit outside its output whitelist and quarantines abandoned edits, so a broken
    or interrupted audit can never change project behavior.
    """
    print(f"\n{'─' * 60}")
    print(f"  CID audit (cadence: every {AUDIT_EVERY} iterations)")
    print(f"{'─' * 60}")
    run_metrics_snapshot(cwd)
    _, pre_head = _git(cwd, "rev-parse", "HEAD")
    result = run_agent(
        claude_cmd, AUDIT_ROLE, iteration, cwd, skip_permissions, fallback_model
    )
    enforce_audit_safety(cwd, pre_head.strip())
    _stash_abandoned_edits(cwd, "audit", _is_audit_output)
    commit_log(cwd, iteration)
    return result


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
    fallback = args.fallback_model

    # Evaluate any meta change whose post-change window has now elapsed (trusted-runner
    # rollback), before adding more iterations on top. If a regressed change could not
    # be auto-reverted, pause instead of building on top of a live regression.
    if evaluate_meta_rollbacks(cwd):
        print(
            "\n  *** HUMAN REVIEW REQUESTED: a regressed meta change could not be "
            "auto-reverted (git revert conflict).\n"
            "  Resolve handoff.md, then rerun cid:run. ***"
        )
        return

    start = last_iteration(cwd)
    print(f"CID run — max {max_iter} iterations (continuing from {start})")
    print(f"Working directory: {cwd}")
    if pause:
        print(f"Pause between iterations: {pause}s")
    print()

    for i in range(start + 1, start + max_iter + 1):
        result = run_iteration(claude_cmd, i, cwd, args.skip_permissions, fallback)

        if result == "done":
            print(f"\nProject complete at iteration {i}.")
            break
        elif result == "idle":
            print(f"\nNo actionable work at iteration {i}.")
            maybe_run_meta_improve(claude_cmd, i, cwd, args)
            print("Exiting CID loop.")
            break
        elif result == "pause":
            print(f"\nPaused at iteration {i} — human review needed.")
            break
        elif result == "fail":
            print(f"\nIteration {i} failed. Stopping.")
            sys.exit(1)

        if audit_due(i) and not getattr(args, "no_audit", False):
            run_audit(claude_cmd, i, cwd, args.skip_permissions, fallback)

        if i < start + max_iter and pause:
            print(
                f"\nPausing {pause}s before next iteration (press Enter to continue)..."
            )
            wait_with_skip(pause)
    else:
        print(f"\nReached max iterations ({max_iter}).")


def maybe_run_meta_improve(claude_cmd, iteration, cwd, args):
    """Run the self-improvement role once when the loop reaches IDLE.

    Turns an otherwise-wasted idle exit into one bounded meta-improvement pass.
    Skipped when --no-improve is set. The meta row is committed to the log.
    """
    if getattr(args, "no_improve", False):
        return
    print("\n  Running meta-improve (self-improvement pass) before exit...")
    _, pre_head = _git(cwd, "rev-parse", "HEAD")
    run_agent(
        claude_cmd,
        META_ROLE,
        iteration,
        cwd,
        args.skip_permissions,
        args.fallback_model,
    )
    # Hard-enforce the guardrails on whatever the agent committed, then quarantine any
    # uncommitted prompt edits it abandoned (e.g. on timeout) so they cannot leak into
    # the next run unreviewed.
    enforce_meta_safety(cwd, pre_head.strip())
    _stash_abandoned_edits(cwd, "meta", _is_meta_bookkeeping)
    commit_log(cwd, iteration)
    reason = check_human_review(cwd)
    if reason:
        print(f"\n  *** meta-improve flagged HUMAN REVIEW: {reason} ***")


def cmd_step(args):
    """Single CID iteration."""
    claude_cmd = find_claude()
    if not claude_cmd:
        print("ERROR: claude not found in PATH")
        sys.exit(1)

    cwd = Path(args.workdir).resolve()
    iteration = (
        args.iteration if args.iteration is not None else last_iteration(cwd) + 1
    )
    result = run_iteration(
        claude_cmd, iteration, cwd, args.skip_permissions, args.fallback_model
    )

    if result == "fail":
        sys.exit(1)


def cmd_improve(args):
    """Run the meta-improve self-improvement role once, on demand."""
    claude_cmd = find_claude()
    if not claude_cmd:
        print("ERROR: claude not found in PATH")
        sys.exit(1)

    cwd = Path(args.workdir).resolve()
    if evaluate_meta_rollbacks(cwd):
        print(
            "\n  *** HUMAN REVIEW REQUESTED: a prior regressed meta change could not be "
            "auto-reverted. Resolve handoff.md before running improve. ***"
        )
        sys.exit(1)
    iteration = last_iteration(cwd) + 1
    _, pre_head = _git(cwd, "rev-parse", "HEAD")
    result = run_agent(
        claude_cmd,
        META_ROLE,
        iteration,
        cwd,
        args.skip_permissions,
        args.fallback_model,
    )
    enforce_meta_safety(cwd, pre_head.strip())
    _stash_abandoned_edits(cwd, "meta", _is_meta_bookkeeping)
    commit_log(cwd, iteration)
    if not result["ok"]:
        sys.exit(1)


def cmd_audit(args):
    """Run the codebase audit role once, on demand."""
    claude_cmd = find_claude()
    if not claude_cmd:
        print("ERROR: claude not found in PATH")
        sys.exit(1)

    cwd = Path(args.workdir).resolve()
    iteration = last_iteration(cwd) + 1
    result = run_audit(
        claude_cmd, iteration, cwd, args.skip_permissions, args.fallback_model
    )
    if not result["ok"]:
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

    entries = read_entries(cwd)
    role_entries = [e for e in entries if is_role_entry(e)]
    summaries = [e for e in entries if e.get("type") == "iteration_summary"]

    if not role_entries:
        print("(iteration log has no canonical role entries yet)")
        return

    total_cost = sum(_num(e, "cost_usd") for e in role_entries)
    total_duration = sum(_num(e, "duration_s") for e in role_entries)
    total_turns = sum(_num(e, "turns") for e in role_entries)
    ok_count = sum(1 for e in role_entries if e.get("status") == "OK")
    fail_count = len(role_entries) - ok_count
    iterations = {e.get("iteration") for e in role_entries}
    skipped = len(entries) - len(role_entries) - len(summaries)

    print(
        f"CID Iteration Log — {len(role_entries)} role runs across {len(iterations)} iterations"
    )
    print(f"{'─' * 60}")
    print(f"  Total cost:     ${total_cost:.4f}")
    print(f"  Total duration: {total_duration:.0f}s ({total_duration / 3600:.1f}h)")
    print(f"  Total turns:    {total_turns}")
    print(f"  Success/Fail:   {ok_count}/{fail_count}")
    if skipped:
        print(f"  Legacy rows ignored: {skipped}")
    print()

    # Per-role breakdown
    role_stats = {}
    for e in role_entries:
        role = e.get("role", "unknown")
        rs = role_stats.setdefault(
            role, {"cost": 0.0, "duration": 0.0, "turns": 0, "runs": 0, "fails": 0}
        )
        rs["cost"] += _num(e, "cost_usd")
        rs["duration"] += _num(e, "duration_s")
        rs["turns"] += _num(e, "turns")
        rs["runs"] += 1
        if e.get("status") != "OK":
            rs["fails"] += 1

    print(
        f"  {'Role':<14} {'Runs':>5} {'Fails':>6} {'Cost':>9} {'Duration':>10} {'Turns':>6}"
    )
    print(f"  {'─' * 14} {'─' * 5} {'─' * 6} {'─' * 9} {'─' * 10} {'─' * 6}")
    ordered = [
        *ROLES,
        META_ROLE,
        *(r for r in role_stats if r not in (*ROLES, META_ROLE)),
    ]
    for role in ordered:
        rs = role_stats.get(role)
        if not rs:
            continue
        print(
            f"  {role:<14} {rs['runs']:>5} {rs['fails']:>6} "
            f"${rs['cost']:>8.4f} {rs['duration']:>9.0f}s {rs['turns']:>6}"
        )

    # Verdict distribution from iteration summaries
    if summaries:
        verdicts = {}
        for s in summaries:
            verdicts[s.get("verdict", "UNKNOWN")] = (
                verdicts.get(s.get("verdict", "UNKNOWN"), 0) + 1
            )
        print()
        print(
            f"  Verdicts ({len(summaries)} iterations): "
            + ", ".join(f"{k}={v}" for k, v in sorted(verdicts.items()))
        )

    # Last 5 role entries
    print()
    print("  Recent entries:")
    for e in role_entries[-5:]:
        ts = e.get("ts", "?")[:19]
        role = e.get("role", "?")
        status = e.get("status", "?")
        cost = _num(e, "cost_usd")
        duration = _num(e, "duration_s")
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
    iteration = (
        args.iteration if args.iteration is not None else last_iteration(cwd) + 1
    )
    result = run_agent(
        claude_cmd,
        args.role,
        iteration,
        cwd,
        args.skip_permissions,
        args.fallback_model,
    )
    commit_log(cwd, iteration)

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
    parser.add_argument(
        "--fallback-model",
        default=None,
        help="Fallback model for claude when the primary is overloaded (e.g. 'sonnet')",
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
        default=1200,
        help="Seconds to pause between iterations (default: 1200)",
    )
    run_p.add_argument(
        "--no-improve",
        action="store_true",
        default=False,
        help="Do not run the meta-improve role when the loop reaches IDLE",
    )
    run_p.add_argument(
        "--no-audit",
        action="store_true",
        default=False,
        help=f"Do not run the audit role every {AUDIT_EVERY} iterations",
    )
    run_p.set_defaults(func=cmd_run)

    # step
    step_p = sub.add_parser("step", help="Run a single CID iteration (all 4 roles)")
    step_p.add_argument(
        "--iteration",
        type=int,
        default=None,
        help="Iteration number (default: continue from the log)",
    )
    step_p.set_defaults(func=cmd_step)

    # status
    status_p = sub.add_parser("status", help="Show current project state from state.md")
    status_p.set_defaults(func=cmd_status)

    # stats
    stats_p = sub.add_parser("stats", help="Show iteration log summary statistics")
    stats_p.set_defaults(func=cmd_stats)

    # improve (self-improvement pass)
    improve_p = sub.add_parser(
        "improve", help="Run the meta-improve self-improvement role once"
    )
    improve_p.set_defaults(func=cmd_improve)

    # audit (codebase maintainability pass)
    audit_p = sub.add_parser("audit", help="Run the codebase audit role once")
    audit_p.set_defaults(func=cmd_audit)

    # role (for testing individual agents)
    role_p = sub.add_parser(
        "role", help="Run a single CID role (for testing/debugging)"
    )
    role_p.add_argument("role", choices=(*ROLES, META_ROLE), help="Which role to run")
    role_p.add_argument(
        "--iteration",
        type=int,
        default=None,
        help="Iteration number (default: continue from the log)",
    )
    role_p.set_defaults(func=cmd_role)

    args = parser.parse_args()
    args.func(args)


if __name__ == "__main__":
    main()
