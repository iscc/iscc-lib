---
name: loop-metrics-facts
description: Non-obvious facts about reading CID loop metrics and tuning levers — verify before relying on
metadata:
  type: project
---

Facts about reading the CID record and choosing meta changes that are not obvious from the code.
Check each against the current files before acting. Last verified at iteration 248.

- **The "Fails" column in `cid.py stats` is misleading.** It counts every row whose status is not
    `OK`, including zero-turn status-marker rows. For real failures, grep role rows for
    `"status":"FAIL"` or `"TIMEOUT"`. Since iteration 150 there are only four: define-next 155
    (TIMEOUT), review 180 (FAIL), review 195 and 212 (TIMEOUT, both recovered).

- **needs_work_rate is low but not zero.** Over iterations 209–248 it was 2/40 (215 CRAP gate, 239
    rb_sys cross-gem). Both verdicts caught real defects, and each lesson went into learnings.md.
    The rate is too sparse to tell a prompt change apart from noise, so median_turns is the more
    usable target. A drop in the rate is only real if quality held, and the reviewer stays
    path-blocked from meta edits.

- **High median_turns usually means genuine work, not prompt waste.** Large perf-gate or fan-out
    steps run 120–200 turns. The median fell from about 113 (211–220) to about 81 (238–247) without
    any meta change. Before blaming a prompt for turn cost, check what the iteration actually did.

- **`model:`/`effort:` frontmatter are live levers, passed by `cid.py`.** `AGENT_FLAG_KEYS` sends
    them as `--model`/`--effort`, because the CLI ignores frontmatter `effort:` under `--agent`.
    They are whitelisted for the three non-gate roles. Lowering effort risks the blind spot in the
    rollback rule: a drop in turns or cost can hide a silent drop in quality.

- **Zero-turn summaries pollute median_turns.** A recovered TIMEOUT logs `turns: 0`. When the
    iteration also started at review (195, 212), its summary is `turns_total: 0` with a real
    verdict. `_window_metrics` excludes only FAILED verdicts, so the 0 enters the median. This is
    proposed for Titusz (proposals.md, iteration 248). Until it is fixed, check windows for 0 rows
    by hand.

- **Do not apply a change while the loop is IDLE.** The post-change window would consist of no-op
    iterations (36 turns at 248), which makes the rollback verdict noise. Meta changes are best
    applied when product work is queued.

- **The 200-line budget for `learnings.md` is enforced as `> 200`.** Review keeps the file at the
    limit by compressing entries (200/200 at iteration 248, no overrun ever recorded). Being at the
    limit is not evidence that hygiene is needed.
