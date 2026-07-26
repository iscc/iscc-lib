"""Tests for the CID orchestrator's pure helpers in ``tools/cid.py``.

Covers log accounting (schema filtering, persistent iteration counter, summary
aggregation), verdict parsing, and headless claude command assembly. Subprocess
and network paths are intentionally out of scope — these tests exercise the
deterministic logic the loop and the meta-improve role depend on.
"""

import importlib.util
import json
import subprocess
from pathlib import Path

# Load tools/cid.py by path — it is a uv script, not an installed package.
_CID_PATH = Path(__file__).resolve().parents[1] / "tools" / "cid.py"
_spec = importlib.util.spec_from_file_location("cid_tool", _CID_PATH)
assert _spec is not None
_loader = _spec.loader
assert _loader is not None
cid = importlib.util.module_from_spec(_spec)
_loader.exec_module(cid)


def _write_log(cwd, rows):
    """Write rows (dicts or raw strings) as JSONL to the iteration log."""
    log_path = cwd / cid.LOG_FILE
    log_path.parent.mkdir(parents=True, exist_ok=True)
    lines = [r if isinstance(r, str) else json.dumps(r) for r in rows]
    log_path.write_text("\n".join(lines) + "\n", encoding="utf-8")


def _role_row(iteration, role="advance", status="OK", turns=5, cost=0.1, duration=10.0):
    """Build a canonical per-role log row."""
    return {
        "ts": "2026-06-16T00:00:00+00:00",
        "iteration": iteration,
        "role": role,
        "status": status,
        "turns": turns,
        "cost_usd": cost,
        "duration_s": duration,
    }


def test_last_iteration_empty(tmp_path):
    assert cid.last_iteration(tmp_path) == 0


def test_last_iteration_returns_max_across_schemas(tmp_path):
    _write_log(
        tmp_path,
        [
            _role_row(3),
            _role_row(7),
            {"iteration": 85, "step": "legacy", "verdict": "PASS"},  # legacy schema
        ],
    )
    # last_iteration counts any row with an iteration field, legacy included,
    # so numbering never collides with historical entries.
    assert cid.last_iteration(tmp_path) == 85


def test_is_role_entry_accepts_canonical_and_rejects_others():
    assert cid.is_role_entry(_role_row(1))
    assert cid.is_role_entry(_role_row(1, role="meta-improve"))
    assert cid.is_role_entry(_role_row(1, role="orchestrator"))  # legacy role kept
    assert not cid.is_role_entry({"iteration": 1, "step": "x", "verdict": "PASS"})
    assert not cid.is_role_entry(
        {"type": "iteration_summary", "iteration": 1, "verdict": "PASS"}
    )
    # canonical keys but an unknown role is not counted as a role entry
    assert not cid.is_role_entry(_role_row(1, role="bogus"))


def test_read_entries_skips_malformed(tmp_path):
    _write_log(tmp_path, [_role_row(1), "{not json", _role_row(2)])
    entries = cid.read_entries(tmp_path)
    assert len(entries) == 2
    assert [e["iteration"] for e in entries] == [1, 2]


def test_parse_verdict_variants(tmp_path):
    handoff = tmp_path / cid.HANDOFF_FILE
    handoff.parent.mkdir(parents=True, exist_ok=True)

    handoff.write_text("## Review\n\n**Verdict:** NEEDS_WORK\n", encoding="utf-8")
    assert cid.parse_verdict(tmp_path) == "NEEDS_WORK"

    handoff.write_text("**Verdict:** PASS_WITH_NOTES — looks good\n", encoding="utf-8")
    assert cid.parse_verdict(tmp_path) == "PASS_WITH_NOTES"

    handoff.write_text("**Verdict:** PASS\n", encoding="utf-8")
    assert cid.parse_verdict(tmp_path) == "PASS"

    # IDLE marker takes precedence over any verdict line
    handoff.write_text("**IDLE**: nothing to do\n**Verdict:** PASS\n", encoding="utf-8")
    assert cid.parse_verdict(tmp_path) == "IDLE"


def test_parse_verdict_absent(tmp_path):
    assert cid.parse_verdict(tmp_path) is None  # no handoff file
    handoff = tmp_path / cid.HANDOFF_FILE
    handoff.parent.mkdir(parents=True, exist_ok=True)
    handoff.write_text("no verdict here\n", encoding="utf-8")
    assert cid.parse_verdict(tmp_path) is None


def test_append_iteration_summary_aggregates(tmp_path):
    _write_log(
        tmp_path,
        [
            _role_row(4, role="update-state", turns=3, cost=0.1, duration=5.0),
            _role_row(4, role="advance", turns=10, cost=0.4, duration=20.0),
            _role_row(
                3, role="advance", turns=99, cost=9.9, duration=99.0
            ),  # other iter
        ],
    )
    handoff = tmp_path / cid.HANDOFF_FILE
    handoff.write_text("**Verdict:** PASS\n", encoding="utf-8")

    cid.append_iteration_summary(tmp_path, 4)

    summaries = [
        e for e in cid.read_entries(tmp_path) if e.get("type") == "iteration_summary"
    ]
    assert len(summaries) == 1
    summary = summaries[0]
    assert summary["iteration"] == 4
    assert summary["verdict"] == "PASS"
    # Only iteration-4 rows are aggregated; iteration 3 is excluded.
    assert summary["turns_total"] == 13
    assert summary["cost_total"] == 0.5
    assert summary["duration_total"] == 25.0


def test_build_agent_cmd_includes_cache_flag_and_no_max_turns():
    cmd = cid.build_agent_cmd("claude", "advance", "do it", skip_permissions=False)
    assert "--exclude-dynamic-system-prompt-sections" in cmd
    # --max-turns does not exist on the installed CLI; the guard is a wall-clock
    # timeout enforced in run_agent, so it must NOT be passed as a flag.
    assert "--max-turns" not in cmd
    assert "--dangerously-skip-permissions" not in cmd
    assert "--fallback-model" not in cmd


def test_build_agent_cmd_optional_flags():
    cmd = cid.build_agent_cmd(
        "claude", "meta-improve", "go", skip_permissions=True, fallback_model="sonnet"
    )
    assert "--dangerously-skip-permissions" in cmd
    assert cmd[cmd.index("--fallback-model") + 1] == "sonnet"
    assert "--max-turns" not in cmd


def test_every_role_has_a_timeout():
    for role in (*cid.ROLES, cid.META_ROLE, cid.AUDIT_ROLE):
        assert cid.ROLE_TIMEOUT_S.get(role), f"{role} has no wall-clock timeout"


def test_process_output_tolerates_null_cost(tmp_path):
    # A result envelope with explicit-null cost/turns must not crash the role.
    lines = (
        '{"type":"result","total_cost_usd":null,"num_turns":null,"is_error":false}\n'
    )
    fake = tmp_path / "stream.jsonl"
    fake.write_text(lines, encoding="utf-8")
    with fake.open() as fh:
        cost, turns, is_error = cid._process_output(fh, "advance")
    assert cost == 0.0
    assert turns == 0
    assert is_error is False
    # the downstream sites that previously crashed must now be safe
    assert f"${cost:.4f}" == "$0.0000"
    assert round(cost, 6) == 0


def test_is_meta_bookkeeping_classification():
    assert cid._is_meta_bookkeeping(".claude/context/meta-log.jsonl")
    assert cid._is_meta_bookkeeping(".claude/context/proposals.md")
    assert cid._is_meta_bookkeeping(".claude/agent-memory/meta-improve/MEMORY.md")
    assert not cid._is_meta_bookkeeping(".claude/agents/advance.md")
    assert not cid._is_meta_bookkeeping(".claude/agents/review.md")


# --- git-backed safety tests ---


def _git(repo, *args):
    """Run a git command in repo, asserting success."""
    subprocess.run(["git", *args], cwd=repo, capture_output=True, text=True, check=True)


def _head(repo):
    out = subprocess.run(
        ["git", "rev-parse", "HEAD"],
        cwd=repo,
        capture_output=True,
        text=True,
        check=True,
    )
    return out.stdout.strip()


def _init_repo(tmp_path):
    """Create a git repo with .claude/agents/{advance,review}.md committed at base."""
    _git(tmp_path, "init", "-q")
    _git(tmp_path, "config", "user.email", "t@t.t")
    _git(tmp_path, "config", "user.name", "t")
    agents = tmp_path / ".claude" / "agents"
    agents.mkdir(parents=True)
    (agents / "advance.md").write_text("advance base\n", encoding="utf-8")
    (agents / "review.md").write_text("review base\n", encoding="utf-8")
    (tmp_path / ".claude" / "context").mkdir(parents=True, exist_ok=True)
    (tmp_path / ".claude" / "context" / "learnings.md").write_text(
        "learn base\n", encoding="utf-8"
    )
    _git(tmp_path, "add", "-A")
    _git(tmp_path, "commit", "-qm", "base")
    return tmp_path


def _meta_commit(repo, rel, content, msg="cid(meta): change"):
    """Edit a file and commit it (simulating a meta-improve auto-apply)."""
    path = repo / rel
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(content, encoding="utf-8")
    _git(repo, "add", rel)
    _git(repo, "commit", "-qm", msg)
    return _head(repo)


def _add_apply_record(repo, commit, iteration=5, window=5):
    """Append a well-formed `apply` record for a commit to meta-log.jsonl.

    A whitelisted change must carry one of these to be measurable; without it the
    runner reverts the change as unmeasurable (see enforce_meta_safety).
    """
    path = repo / cid.META_LOG_FILE
    path.parent.mkdir(parents=True, exist_ok=True)
    rec = {
        "action": "apply",
        "commit": commit,
        "iteration": iteration,
        "window": window,
    }
    with path.open("a", encoding="utf-8") as f:
        f.write(json.dumps(rec) + "\n")


def test_enforce_meta_safety_reverts_protected_path(tmp_path):
    repo = _init_repo(tmp_path)
    base = _head(repo)
    # The reviewer is the quality gate — editing it must be reverted.
    _meta_commit(repo, ".claude/agents/review.md", "review WEAKENED\n")
    reverted = cid.enforce_meta_safety(repo, base)
    assert reverted == 1
    assert (repo / ".claude/agents/review.md").read_text() == "review base\n"


def test_enforce_meta_safety_keeps_allowed_edit(tmp_path):
    repo = _init_repo(tmp_path)
    base = _head(repo)
    # An allowed edit survives only when it is measurable (carries an apply record).
    sha = _meta_commit(repo, ".claude/agents/advance.md", "advance improved\n")
    _add_apply_record(repo, sha)
    reverted = cid.enforce_meta_safety(repo, base)
    assert reverted == 0
    assert (repo / ".claude/agents/advance.md").read_text() == "advance improved\n"


def test_enforce_meta_safety_reverts_unrecorded_change(tmp_path):
    # A whitelisted edit with NO apply record is unmeasurable — the rollback evaluator
    # would never see it — so the runner reverts it even though the path is allowed.
    repo = _init_repo(tmp_path)
    base = _head(repo)
    _meta_commit(repo, ".claude/agents/advance.md", "advance unrecorded\n")
    reverted = cid.enforce_meta_safety(repo, base)
    assert reverted == 1
    assert (repo / ".claude/agents/advance.md").read_text() == "advance base\n"


def test_enforce_meta_safety_one_change_cap(tmp_path):
    repo = _init_repo(tmp_path)
    base = _head(repo)
    # Two distinct allowed+recorded changes — only the newest may survive the cap.
    sha1 = _meta_commit(repo, ".claude/agents/advance.md", "advance v1\n")
    sha2 = _meta_commit(repo, ".claude/context/learnings.md", "learn v1\n")
    _add_apply_record(repo, sha1)
    _add_apply_record(repo, sha2)
    reverted = cid.enforce_meta_safety(repo, base)
    assert reverted == 1  # the older allowed change is reverted by the cap
    assert (repo / ".claude/context/learnings.md").read_text() == "learn v1\n"
    assert (repo / ".claude/agents/advance.md").read_text() == "advance base\n"


def test_evaluate_meta_rollbacks_reverts_on_regression(tmp_path):
    repo = _init_repo(tmp_path)
    change_sha = _meta_commit(repo, ".claude/agents/advance.md", "advance tweaked\n")
    # Summary series: clean before iter 5, all NEEDS_WORK after -> regression.
    summaries = [
        {
            "type": "iteration_summary",
            "iteration": i,
            "verdict": "PASS",
            "turns_total": 10,
        }
        for i in (3, 4, 5)
    ] + [
        {
            "type": "iteration_summary",
            "iteration": i,
            "verdict": "NEEDS_WORK",
            "turns_total": 10,
        }
        for i in (6, 7, 8)
    ]
    _write_log(repo, summaries)
    meta_log = repo / cid.META_LOG_FILE
    meta_log.write_text(
        json.dumps(
            {"action": "apply", "commit": change_sha, "iteration": 5, "window": 3}
        )
        + "\n",
        encoding="utf-8",
    )
    cid.evaluate_meta_rollbacks(repo)
    # The change must be reverted and a rollback record appended.
    assert (repo / ".claude/agents/advance.md").read_text() == "advance base\n"
    records = [json.loads(line) for line in meta_log.read_text().splitlines()]
    assert any(r.get("action") == "rollback" for r in records)


def test_evaluate_meta_rollbacks_keeps_when_no_regression(tmp_path):
    repo = _init_repo(tmp_path)
    change_sha = _meta_commit(repo, ".claude/agents/advance.md", "advance tweaked\n")
    summaries = [
        {
            "type": "iteration_summary",
            "iteration": i,
            "verdict": "PASS",
            "turns_total": 10,
        }
        for i in (3, 4, 5, 6, 7, 8)
    ]
    _write_log(repo, summaries)
    meta_log = repo / cid.META_LOG_FILE
    meta_log.write_text(
        json.dumps(
            {"action": "apply", "commit": change_sha, "iteration": 5, "window": 3}
        )
        + "\n",
        encoding="utf-8",
    )
    cid.evaluate_meta_rollbacks(repo)
    assert (repo / ".claude/agents/advance.md").read_text() == "advance tweaked\n"
    records = [json.loads(line) for line in meta_log.read_text().splitlines()]
    assert any(r.get("action") == "evaluated" for r in records)


def test_num_coerces_null_and_missing():
    assert cid._num({"cost_usd": None}, "cost_usd") == 0
    assert cid._num({}, "turns") == 0
    assert cid._num({"turns": "5"}, "turns") == 0  # non-numeric string -> 0
    assert cid._num({"turns": 5}, "turns") == 5
    assert cid._num({"cost_usd": 0.4}, "cost_usd") == 0.4


def test_append_iteration_summary_tolerates_null_metrics(tmp_path):
    # Legacy/foreign rows can carry explicit null metrics; aggregation must not crash.
    _write_log(
        tmp_path,
        [
            {
                "ts": "2026-06-16T00:00:00+00:00",
                "iteration": 9,
                "role": "review",
                "status": "OK",
                "turns": None,
                "cost_usd": None,
                "duration_s": None,
            },
            _role_row(9, role="advance", turns=4, cost=0.2, duration=8.0),
        ],
    )
    cid.append_iteration_summary(tmp_path, 9)
    summary = next(
        e for e in cid.read_entries(tmp_path) if e.get("type") == "iteration_summary"
    )
    assert summary["turns_total"] == 4
    assert summary["cost_total"] == 0.2
    assert summary["duration_total"] == 8.0


def _porcelain(repo):
    out = subprocess.run(
        ["git", "status", "--porcelain"],
        cwd=repo,
        capture_output=True,
        text=True,
        check=True,
    )
    return out.stdout


def _regression_summaries(pass_iters, nw_iters):
    """iteration_summary rows: PASS for pass_iters, NEEDS_WORK for nw_iters."""

    def row(i, verdict):
        return {
            "type": "iteration_summary",
            "iteration": i,
            "verdict": verdict,
            "turns_total": 10,
        }

    return [row(i, "PASS") for i in pass_iters] + [
        row(i, "NEEDS_WORK") for i in nw_iters
    ]


def _apply_record(repo, commit, iteration, window):
    (repo / cid.META_LOG_FILE).write_text(
        json.dumps(
            {
                "action": "apply",
                "commit": commit,
                "iteration": iteration,
                "window": window,
            }
        )
        + "\n",
        encoding="utf-8",
    )


def test_evaluate_meta_rollbacks_conflict_leaves_clean_tree(tmp_path):
    # A revert that conflicts (a later commit re-touched the file) must NOT corrupt
    # the tree or record a false rollback success — it flags the human instead.
    repo = _init_repo(tmp_path)
    change_sha = _meta_commit(repo, ".claude/agents/advance.md", "advance v1\n")
    _meta_commit(repo, ".claude/agents/advance.md", "advance later\n", msg="later edit")
    _write_log(repo, _regression_summaries((3, 4, 5), (6, 7, 8)))
    _apply_record(repo, change_sha, 5, 3)

    cid.evaluate_meta_rollbacks(repo)

    assert "UU" not in _porcelain(repo)  # no unmerged conflict markers
    records = [
        json.loads(line) for line in (repo / cid.META_LOG_FILE).read_text().splitlines()
    ]
    assert not any(r.get("action") == "rollback" for r in records)  # no false success
    assert cid.HUMAN_REVIEW_MARKER in (repo / cid.HANDOFF_FILE).read_text()


def test_evaluate_meta_rollbacks_clamps_oversized_window(tmp_path):
    # An agent-supplied window of 9999 must be clamped so the change is still judged.
    repo = _init_repo(tmp_path)
    change_sha = _meta_commit(repo, ".claude/agents/advance.md", "advance tweaked\n")
    _write_log(repo, _regression_summaries(range(1, 6), range(6, 16)))
    _apply_record(repo, change_sha, 5, 9999)

    cid.evaluate_meta_rollbacks(repo)

    # Clamped to META_WINDOW_CAP, so the regression is detected and reverted.
    assert (repo / ".claude/agents/advance.md").read_text() == "advance base\n"


def test_enforce_meta_safety_flags_human_review_on_conflict(tmp_path):
    repo = _init_repo(tmp_path)
    base = _head(repo)
    # B weakens review.md (protected) AND edits advance.md; C re-edits advance.md so
    # reverting B conflicts.
    (repo / ".claude/agents/review.md").write_text("review WEAK\n", encoding="utf-8")
    (repo / ".claude/agents/advance.md").write_text("advance v1\n", encoding="utf-8")
    _git(repo, "add", "-A")
    _git(repo, "commit", "-qm", "cid(meta): B weaken+edit")
    # C edits advance.md again with a valid record so it survives — reverting B then
    # conflicts on advance.md.
    c_sha = _meta_commit(
        repo, ".claude/agents/advance.md", "advance later\n", msg="cid(meta): C"
    )
    _add_apply_record(repo, c_sha)

    cid.enforce_meta_safety(repo, base)

    assert "UU" not in _porcelain(repo)  # conflict aborted cleanly
    assert cid.HUMAN_REVIEW_MARKER in (repo / cid.HANDOFF_FILE).read_text()


def test_evaluate_meta_rollbacks_noop_revert_is_success(tmp_path):
    # A change later restored to the exact original makes `git revert` an empty no-op
    # (rc!=0, no REVERT_HEAD). That is a success, not a conflict: record it and do not
    # wedge the loop into a permanent false human-review pause.
    repo = _init_repo(tmp_path)
    change_sha = _meta_commit(repo, ".claude/agents/advance.md", "advance v1\n")
    _meta_commit(repo, ".claude/agents/advance.md", "advance base\n", msg="restore")
    _write_log(repo, _regression_summaries((3, 4, 5), (6, 7, 8)))
    _apply_record(repo, change_sha, 5, 3)

    cid.evaluate_meta_rollbacks(repo)

    assert "UU" not in _porcelain(repo)
    records = [
        json.loads(line) for line in (repo / cid.META_LOG_FILE).read_text().splitlines()
    ]
    assert any(r.get("action") == "rollback" for r in records)  # resolved, not stuck
    # No human review flagged — handoff.md is never even created for a clean no-op.
    handoff = repo / cid.HANDOFF_FILE
    assert not handoff.exists() or cid.HUMAN_REVIEW_MARKER not in handoff.read_text()


def test_enforce_meta_safety_refuses_rewritten_history(tmp_path):
    # An amended base commit must not cause the runner to revert productive work.
    repo = _init_repo(tmp_path)
    (repo / "feature.txt").write_text("feature\n", encoding="utf-8")
    _git(repo, "add", "feature.txt")
    _git(repo, "commit", "-qm", "feat: productive work")
    pre_head = _head(repo)
    # Agent folds a review.md weakening into the productive commit via amend.
    (repo / ".claude/agents/review.md").write_text("review WEAK\n", encoding="utf-8")
    _git(repo, "add", ".claude/agents/review.md")
    _git(repo, "commit", "--amend", "--no-edit", "-q")

    reverted = cid.enforce_meta_safety(repo, pre_head)

    assert reverted == 0  # refused — history was rewritten
    assert (repo / "feature.txt").read_text() == "feature\n"  # productive work intact
    assert cid.HUMAN_REVIEW_MARKER in (repo / cid.HANDOFF_FILE).read_text()


def test_safe_revert_refuses_dirty_commit_path(tmp_path):
    # git refuses to start a revert when the commit's own file has uncommitted changes;
    # _safe_revert must report failure (commit still live), not a false success.
    repo = _init_repo(tmp_path)
    sha = _meta_commit(repo, ".claude/agents/advance.md", "advance v1\n")
    # Uncommitted local edit to the SAME file the commit touches.
    (repo / ".claude/agents/advance.md").write_text("advance dirty\n", encoding="utf-8")
    assert cid._safe_revert(repo, sha) is False
    # The commit is still applied and the dirty edit is left untouched.
    assert (repo / ".claude/agents/advance.md").read_text() == "advance dirty\n"
    assert "UU" not in _porcelain(repo)


def test_safe_revert_tolerates_unrelated_dirty_file(tmp_path):
    # A dirty file unrelated to the commit must NOT block the revert (git tolerates it).
    repo = _init_repo(tmp_path)
    (repo / "unrelated.txt").write_text("base\n", encoding="utf-8")
    _git(repo, "add", "unrelated.txt")
    _git(repo, "commit", "-qm", "add unrelated")
    sha = _meta_commit(repo, ".claude/agents/advance.md", "advance v1\n")
    (repo / "unrelated.txt").write_text(
        "modified\n", encoding="utf-8"
    )  # tracked, dirty
    assert cid._safe_revert(repo, sha) is True
    assert (repo / ".claude/agents/advance.md").read_text() == "advance base\n"
    assert (repo / "unrelated.txt").read_text() == "modified\n"  # left as-is


def test_evaluate_meta_rollbacks_returns_true_on_conflict(tmp_path):
    # An unrevertable regression returns True so cmd_run/cmd_improve pause instead of
    # building on top of a live, unreverted change.
    repo = _init_repo(tmp_path)
    change_sha = _meta_commit(repo, ".claude/agents/advance.md", "advance v1\n")
    _meta_commit(repo, ".claude/agents/advance.md", "advance later\n", msg="later edit")
    _write_log(repo, _regression_summaries((3, 4, 5), (6, 7, 8)))
    _apply_record(repo, change_sha, 5, 3)
    assert cid.evaluate_meta_rollbacks(repo) is True


def test_evaluate_meta_rollbacks_returns_false_on_clean_rollback(tmp_path):
    repo = _init_repo(tmp_path)
    change_sha = _meta_commit(repo, ".claude/agents/advance.md", "advance v1\n")
    _write_log(repo, _regression_summaries((3, 4, 5), (6, 7, 8)))
    _apply_record(repo, change_sha, 5, 3)
    assert cid.evaluate_meta_rollbacks(repo) is False  # reverted cleanly, not blocked
    assert (repo / ".claude/agents/advance.md").read_text() == "advance base\n"


def test_evaluate_meta_rollbacks_no_log_returns_false(tmp_path):
    repo = _init_repo(tmp_path)
    assert cid.evaluate_meta_rollbacks(repo) is False


def test_stash_abandoned_meta_edits_quarantines_prompt_edit(tmp_path):
    # An interrupted meta run leaves an uncommitted prompt edit AND a dirty iteration
    # log. The prompt edit must be stashed (tree restored to HEAD, recoverable) while
    # the runner-owned log is left for commit_log; human review is flagged.
    repo = _init_repo(tmp_path)
    log = repo / cid.LOG_FILE
    log.write_text("base\n", encoding="utf-8")  # track the log so it is dirty, not new
    _git(repo, "add", str(cid.LOG_FILE))
    _git(repo, "commit", "-qm", "track log")
    log.write_text("base\nmeta-row\n", encoding="utf-8")  # runner-owned dirt
    (repo / ".claude/agents/advance.md").write_text(
        "advance ABANDONED\n", encoding="utf-8"
    )

    stashed = cid._stash_abandoned_edits(repo, "meta", cid._is_meta_bookkeeping)

    assert stashed == [".claude/agents/advance.md"]
    assert (
        repo / ".claude/agents/advance.md"
    ).read_text() == "advance base\n"  # at HEAD
    assert log.read_text() == "base\nmeta-row\n"  # log left untouched for commit_log
    out = subprocess.run(
        ["git", "stash", "list"], cwd=repo, capture_output=True, text=True, check=True
    )
    assert "cid(meta): abandoned" in out.stdout  # recoverable
    assert cid.HUMAN_REVIEW_MARKER in (repo / cid.HANDOFF_FILE).read_text()


def test_stash_abandoned_meta_edits_noop_on_clean_tree(tmp_path):
    # A successful meta run leaves the tree clean — nothing to stash, nothing flagged.
    repo = _init_repo(tmp_path)
    assert cid._stash_abandoned_edits(repo, "meta", cid._is_meta_bookkeeping) == []
    handoff = repo / cid.HANDOFF_FILE
    assert not handoff.exists() or cid.HUMAN_REVIEW_MARKER not in handoff.read_text()


def test_abandoned_meta_paths_excludes_log_and_bookkeeping(tmp_path):
    # Only prompt/machinery edits count — the iteration log and meta bookkeeping are
    # runner/agent scratch and must be excluded from the abandoned set. Track them first
    # so the exclusion filter (not git's untracked filter) is what's exercised.
    repo = _init_repo(tmp_path)
    for f in (cid.LOG_FILE, cid.META_LOG_FILE, cid.PROPOSALS_FILE):
        (repo / f).write_text("base\n", encoding="utf-8")
    _git(repo, "add", "-A")
    _git(repo, "commit", "-qm", "track log+bookkeeping")
    for f in (cid.LOG_FILE, cid.META_LOG_FILE, cid.PROPOSALS_FILE):
        (repo / f).write_text("dirty\n", encoding="utf-8")
    (repo / ".claude/agents/advance.md").write_text(
        "advance edited\n", encoding="utf-8"
    )

    paths = cid._abandoned_paths(repo, cid._is_meta_bookkeeping)

    assert paths == [".claude/agents/advance.md"]


# --- audit role safety tests ---


def test_audit_due_cadence():
    assert cid.audit_due(cid.AUDIT_EVERY)
    assert cid.audit_due(12 * cid.AUDIT_EVERY)
    assert not cid.audit_due(cid.AUDIT_EVERY + 1)
    assert not cid.audit_due(0)
    assert not cid.audit_due(cid.AUDIT_EVERY, every=0)  # disabled cadence never fires


def test_is_audit_output_classification():
    assert cid._is_audit_output(".claude/context/issues.md")
    assert cid._is_audit_output(".claude/context/metrics.jsonl")
    assert cid._is_audit_output(".claude/agent-memory/audit/MEMORY.md")
    assert not cid._is_audit_output(".claude/context/handoff.md")
    assert not cid._is_audit_output(".claude/agents/review.md")
    assert not cid._is_audit_output("crates/iscc-lib/src/lib.rs")


def test_enforce_audit_safety_reverts_source_edit(tmp_path):
    # The audit charter is find-and-file only — a source commit must be reverted.
    repo = _init_repo(tmp_path)
    base = _head(repo)
    _meta_commit(repo, "src/lib.rs", "fn sneaky() {}\n", msg="cid(audit): oops")
    reverted = cid.enforce_audit_safety(repo, base)
    assert reverted == 1
    assert not (repo / "src/lib.rs").exists()  # the added file is gone after revert


def test_enforce_audit_safety_keeps_issue_filing(tmp_path):
    repo = _init_repo(tmp_path)
    base = _head(repo)
    _meta_commit(
        repo,
        ".claude/context/issues.md",
        "## Finding `normal` [audit]\n",
        msg="cid(audit): 1 finding filed",
    )
    _meta_commit(
        repo,
        ".claude/agent-memory/audit/MEMORY.md",
        "clean areas\n",
        msg="cid(audit): memory",
    )
    assert cid.enforce_audit_safety(repo, base) == 0
    assert "[audit]" in (repo / ".claude/context/issues.md").read_text()


def test_enforce_audit_safety_mixed_commit_wholly_reverted(tmp_path):
    # One commit touching issues.md AND a source file is reverted as a whole —
    # the whitelist grants no partial pardon.
    repo = _init_repo(tmp_path)
    base = _head(repo)
    (repo / "src").mkdir()
    (repo / "src" / "lib.rs").write_text("fn x() {}\n", encoding="utf-8")
    (repo / ".claude" / "context" / "issues.md").write_text(
        "## F `normal` [audit]\n", encoding="utf-8"
    )
    _git(repo, "add", "-A")
    _git(repo, "commit", "-qm", "cid(audit): mixed")
    assert cid.enforce_audit_safety(repo, base) == 1
    assert not (repo / "src" / "lib.rs").exists()


def test_enforce_audit_safety_refuses_rewritten_history(tmp_path):
    # An amended base must not cause the runner to revert productive work.
    repo = _init_repo(tmp_path)
    (repo / "feature.txt").write_text("feature\n", encoding="utf-8")
    _git(repo, "add", "feature.txt")
    _git(repo, "commit", "-qm", "feat: productive work")
    pre_head = _head(repo)
    (repo / "src").mkdir()
    (repo / "src" / "lib.rs").write_text("fn x() {}\n", encoding="utf-8")
    _git(repo, "add", "src/lib.rs")
    _git(repo, "commit", "--amend", "--no-edit", "-q")

    reverted = cid.enforce_audit_safety(repo, pre_head)

    assert reverted == 0  # refused — history was rewritten
    assert (repo / "feature.txt").read_text() == "feature\n"
    assert cid.HUMAN_REVIEW_MARKER in (repo / cid.HANDOFF_FILE).read_text()


def test_stash_abandoned_edits_audit_quarantines_out_of_charter_edit(tmp_path):
    # An interrupted audit run leaving a dirty prompt file gets quarantined the same
    # way an interrupted meta run does, under the audit stash label.
    repo = _init_repo(tmp_path)
    (repo / ".claude" / "agents" / "advance.md").write_text(
        "advance DIRTY\n", encoding="utf-8"
    )
    stashed = cid._stash_abandoned_edits(repo, "audit", cid._is_audit_output)
    assert stashed == [".claude/agents/advance.md"]
    assert (repo / ".claude" / "agents" / "advance.md").read_text() == "advance base\n"
    out = subprocess.run(
        ["git", "stash", "list"], cwd=repo, capture_output=True, text=True, check=True
    )
    assert "cid(audit): abandoned" in out.stdout
    assert cid.HUMAN_REVIEW_MARKER in (repo / cid.HANDOFF_FILE).read_text()


def test_role_commit_landed_detects_own_commit(tmp_path):
    repo = _init_repo(tmp_path)
    base = _head(repo)
    assert not cid._role_commit_landed(repo, "review", base)  # nothing committed yet
    _meta_commit(repo, "note.md", "verdict\n", msg="cid(review): PASS")
    assert cid._role_commit_landed(repo, "review", base)
    # another role's commit must not count as this role's deliverable
    assert not cid._role_commit_landed(repo, "advance", base)
    # a missing base sha is not evidence of a landed commit
    assert not cid._role_commit_landed(repo, "review", "")


def test_role_commit_landed_ignores_commits_before_the_run(tmp_path):
    repo = _init_repo(tmp_path)
    _meta_commit(repo, "note.md", "old\n", msg="cid(review): earlier iteration")
    base = _head(repo)
    assert not cid._role_commit_landed(repo, "review", base)


class _FakeProcess:
    """Minimal Popen stand-in for run_agent: empty stream, chosen return code."""

    def __init__(self, stdout, returncode):
        self.stdout = stdout
        self.returncode = returncode

    def wait(self):
        return self.returncode

    def kill(self):  # pragma: no cover - only reached if a timer fires
        pass


def _run_agent_with_fake_cli(monkeypatch, repo, returncode, on_start=None):
    """Invoke run_agent against a faked claude CLI, returning its outcome dict."""
    stream = repo / "stream.jsonl"
    stream.write_text(
        '{"type":"result","total_cost_usd":0.5,"num_turns":3,"is_error":false}\n',
        encoding="utf-8",
    )

    real_popen = cid.subprocess.Popen

    def fake_popen(args, **kwargs):
        # Only stand in for the claude CLI; git calls (ours and the runner's) are real.
        if args[0] != "claude":
            return real_popen(args, **kwargs)
        if on_start:
            on_start()
        return _FakeProcess(stream.open(encoding="utf-8"), returncode)

    monkeypatch.setattr(cid.subprocess, "Popen", fake_popen)
    base = _head(repo)
    return cid.run_agent("claude", "review", 7, repo, base_sha=base)


def test_run_agent_recovers_when_deliverable_committed(tmp_path, monkeypatch):
    # A role killed after committing its verdict must not fail the iteration.
    repo = _init_repo(tmp_path)
    outcome = _run_agent_with_fake_cli(
        monkeypatch,
        repo,
        returncode=1,
        on_start=lambda: _meta_commit(repo, "note.md", "v\n", msg="cid(review): PASS"),
    )
    assert outcome["ok"]
    assert outcome["status"] == "FAIL"  # logged status stays honest
    assert outcome["recovered"] is True
    row = json.loads((repo / cid.LOG_FILE).read_text(encoding="utf-8").strip())
    assert row["recovered"] is True
    assert cid.is_role_entry(row)  # the extra key must not break accounting


def test_run_agent_fails_when_nothing_committed(tmp_path, monkeypatch):
    repo = _init_repo(tmp_path)
    outcome = _run_agent_with_fake_cli(monkeypatch, repo, returncode=1)
    assert not outcome["ok"]
    assert outcome["status"] == "FAIL"
    assert "recovered" not in outcome


def test_run_agent_clean_run_is_not_marked_recovered(tmp_path, monkeypatch):
    repo = _init_repo(tmp_path)
    outcome = _run_agent_with_fake_cli(
        monkeypatch,
        repo,
        returncode=0,
        on_start=lambda: _meta_commit(repo, "note.md", "v\n", msg="cid(review): PASS"),
    )
    assert outcome["ok"]
    assert outcome["status"] == "OK"
    assert "recovered" not in outcome


def test_run_metrics_snapshot_appends_and_commits(tmp_path):
    # Runner-side snapshot (gates skipped for speed) writes metrics.jsonl and
    # commits it before the audit agent starts.
    repo = _init_repo(tmp_path)
    ok = cid.run_metrics_snapshot(repo, time_gates=False)
    assert ok
    lines = (repo / cid.METRICS_FILE).read_text(encoding="utf-8").strip().splitlines()
    assert len(lines) == 1
    assert "totals" in json.loads(lines[0])
    out = subprocess.run(
        ["git", "log", "--oneline", "-1"],
        cwd=repo,
        capture_output=True,
        text=True,
        check=True,
    )
    assert "cid(audit): metrics snapshot" in out.stdout
