---
name: loop-metrics-facts
description: Non-obvious facts about reading CID loop metrics and tuning levers — verify before relying on
metadata:
  type: project
---

Hard-won, non-obvious facts for reading the CID record and choosing meta changes. Verify each
against current files before acting — they were true as of iteration 112.

- **`cid.py stats` "Fails" column is misleading.** It counts every non-`OK` status row, including
    the zero-turn `status:"PASS"`/verdict status-marker rows that some review runs emit alongside
    their real `OK` metrics row. Real failures are `status:"FAIL"` rows in `iterations.jsonl` — at
    iter 112 there were only ~2 review FAILs *ever* (e.g. iter 106), not 51. Grep
    `'"status":"FAIL"'` for the true count; don't act on the stats fail column.

- **needs_work_rate has been 0 over a long window.** All 21 `iteration_summary` rows (iters 88→111)
    were PASS / PASS_WITH_NOTES / IDLE — no NEEDS_WORK. So that metric is already floored and cannot
    be a target; the only live efficiency metric is **median_turns**. A future *drop* in
    needs_work_rate is only real if quality held (the reviewer is path-blocked from your edits —
    keep it that way).

- **High median_turns is usually genuine work, not prompt waste.** Iters 107/108/109 (190/192/208
    turns) were the multi-slice iai-callgrind perf-gate; turns fell back to ~122 afterward. Before
    blaming a prompt for turn cost, check whether those iterations did inherently large work.

- **`effort:` and `model:` frontmatter are consumed by the Claude Code `--agent` framework, not by
    `cid.py`.** `cid.py build_agent_cmd` only adds `-p/--agent/--output-format/--fallback-model`. So
    those frontmatter values *are* live tuning levers (whitelisted for the 3 non-gate roles), but
    tuning effort *down* risks the runner's documented asymmetry blind spot — a turns/cost drop that
    hides a silent quality drop. Need real evidence a role is over-powered before touching them.

- **`iteration_summary` rows can be missing for failed/interrupted iterations** (absent for 95, 103,
    106 at iter 112; 106 had a review FAIL). Meta-improve's before/after `window` comparison may
    therefore span non-adjacent iterations. See [[loop-metrics-facts]] proposal in proposals.md to
    have cid.py emit a row even on abort.

- **mdformat has two non-identical configs**: pre-commit auto-fix versus a pre-push check using
    `--wrap 100 --number` plus the mkdocs plugin. A context-file commit can pass commit but fail the
    batch push, forcing review to reformat+amend. Low frequency (visible once in last 200 commits:
    `758fc31`, iter 93).
