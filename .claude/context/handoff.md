## 2026-11-22 — meta-improve, iteration 308

**IDLE**: Loop is healthy and idle; nothing auto-applied, 1 carried-over needs-human proposal.

**Evidence:**

- Verdicts 259–308: 2/50 NEEDS_WORK (268 WASM `.d.ts`/JS-exception defect, 278 JNI release layout).
    Both caught real defects, and both lessons are already in learnings.md. No `FAIL` or `TIMEOUT`
    role run since 212.
- Median `turns_total` 284–307: 85.5, in line with the 81–97 of earlier windows. The peaks (297 163,
    298 168, 296 134) were multi-surface fan-outs, not wasted turns.
- Median turns per role, 259–307: update-state 8, define-next 15, advance 27, review 30. No role
    stands out.
- learnings.md is at 200/200 lines. Review keeps it at the limit; no overrun recorded.

**Auto-applied:** nothing. Applying during IDLE would make the runner's rollback window consist of
no-op iterations (39 turns at 308), so the verdict would be noise. No meta-log rows exist, so the
runner has no rollback to report.

**Open proposals:** 1 (carried from 248/258): leave `turns_total == 0` summaries out of the median
in `_window_metrics`. Latent, not urgent.

**Carried from the review handoff (human decisions, unchanged):**

1. Cut v0.8.0 with `/release 0.8.0`. Merging `develop` into `main` also closes the 6 Dependabot
    alerts.
2. Triage the 17 untriaged `low` issues (promote to `normal` or move to `postponed.md`). The
    smallest real defect is `gen_mixed_code_v0` with `bits` below 8 (panics in Go and Rust debug
    builds).
3. Refresh the iai baseline for `bench_instance_code` (a `cid(human)` commit by convention).

Until one of these happens, a restarted loop writes `Step: NONE` and exits after one cycle. .NET 8
floor stays (decision of 2026-11-21).
