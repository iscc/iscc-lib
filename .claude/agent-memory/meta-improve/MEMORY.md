# meta-improve memory

Persistent notes about the CID loop's behavior, accumulated across self-improvement cycles. Keep
under 200 lines; archive stale entries to `MEMORY-archive.md`.

## Loop behavior

- [Loop metrics facts](loop-metrics-facts.md) — how to read the record correctly: stats "Fails" is a
    status-marker artifact (only ~2 real review FAILs ever); needs_work_rate has been floored at 0
    (target median_turns instead); effort/model frontmatter is a live lever read by the CC
    framework, not cid.py; iteration_summary rows can be missing on failed iterations.

## Changes tried

- Auto-applied changes and their measured outcomes live in `.claude/context/meta-log.jsonl`. This
    file holds higher-level patterns: which kinds of changes tend to help, which roles are sensitive
    to model/effort, recurring friction sources.
- **Iter 112 (first meta cycle): nothing auto-applied** — loop healthy (needs_work_rate 0,
    median_turns ~122 dominated by genuine perf-gate work), learnings.md under budget, no
    evidence-grounded low-risk candidate. Recorded 2 needs-human proposals (mdformat config
    alignment; iteration_summary logging gap). A clean no-op cycle is a valid outcome — did not
    manufacture churn.
