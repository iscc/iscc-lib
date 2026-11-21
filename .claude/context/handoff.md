## 2026-11-21 — Meta-improve, iteration 258

**IDLE**: The loop is healthy. Nothing was auto-applied and no new proposal was written.

**Evidence:**

- Verdicts for 250–257 (the productive run since the last meta cycle): 8 × PASS or PASS_WITH_NOTES,
    0 × NEEDS_WORK. 258 is IDLE (37 turns).
- `turns_total` for 250–257: 73, 71, 181, 96, 63, 97, 102, 100. Median 96.5, in line with 238–247.
    The one outlier (252, 181 turns, advance 104) was a real fan-out: the data-edge meta harness
    across every surface.
- No role FAIL or TIMEOUT since iteration 212. update-state stays at 5–10 turns, define-next at
    11–24, review (Fable 5.2, high) at 14–52. No role is burning turns without doing work.
- No `meta-log.jsonl` exists yet, so the runner has nothing to evaluate or roll back.

**Why nothing was applied:** No recurring friction shows up in the record. The loop is also IDLE, so
any change would be judged on no-op iterations, which makes the rollback verdict meaningless.
learnings.md is at 200 of 200 lines, but review keeps it at the limit by compressing entries. That
is not a hygiene problem.

**Open proposals:** 1, carried over from iteration 248 (in `proposals.md`): keep `turns_total == 0`
summaries out of the median in `_window_metrics` (`tools/cid.py`). It is still a latent issue (2
rows, none since 212) and not urgent.

**Still open for Titusz (from the review handoff of iteration 258):**

- Cut v0.8.0 with `/release`. The release notes should say that the `@iscc/wasm` `alg_cdc_chunks`
    return type is now `Uint8Array[]`.
- Triage the 13 `low` issues marked `**Triage:** untriaged`: promote one to `normal` to give the
    loop work, or delete the line to park it.
- Decide on the end-of-life runtime floors (Python 3.10, .NET 8, Ruby 3.1), yank the broken Ruby
    source gems up to 0.7.0, and fix the version-sync target count in `target.md` (it says 12, the
    actual number is 21).

**Notes:** issues.md is at 298 of 300 lines and learnings.md at 200 of 200. A new entry in either
needs an equal cut in the same commit.
