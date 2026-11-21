# meta-improve memory

Persistent notes about the CID loop's behavior, accumulated across self-improvement cycles. Keep
under 200 lines; archive stale entries to `MEMORY-archive.md`.

## Loop behavior

- [Loop metrics facts](loop-metrics-facts.md) — stats "Fails" is misleading; needs_work_rate sparse
    (2/40); zero-turn summaries skew median; model/effort passed by cid.py; never apply while IDLE.

## Changes tried

- Auto-applied changes and their measured outcomes live in `.claude/context/meta-log.jsonl` (does
    not exist yet: nothing has been auto-applied as of iteration 248). This file holds the
    higher-level patterns.
- **Iter 112: nothing auto-applied.** The loop was healthy. Recorded 2 needs-human proposals;
    proposals.md was cleared afterwards (outcome not verified).
- **Iter 248: nothing auto-applied.** The loop was healthy and idle (2/40 NEEDS_WORK, median turns
    81 and falling). Recorded 1 needs-human proposal: keep `turns_total == 0` summaries out of the
    median in `_window_metrics`. Carried the review's open human decisions (v0.8.0 cut, triage) into
    the handoff so overwriting it did not lose them.
