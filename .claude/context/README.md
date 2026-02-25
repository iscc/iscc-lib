# CID Context Pack (`.claude/context/`)

This directory is the **shared working memory** for the CID (Continuous Iterative Development) loop.
It exists to keep agent prompts small, stable, and unambiguous.

## Design goals

- **Small**: keep each file concise; move deep details into `specs/` (or the codebase) and link.
- **Self-describing**: every file should make it clear *who updates it* and *how*.
- **Mechanically checkable**: prefer checklists, counts, and commands over prose.
- **Stable anchors**: avoid renaming headings that other files link to.

## Files (ownership + update policy)

| File               | Primary owner                                                      | Update style                                      | Purpose                                                                |
| ------------------ | ------------------------------------------------------------------ | ------------------------------------------------- | ---------------------------------------------------------------------- |
| `target.md`        | human + review (when `issues.md` says `Source: [human]` + `Spec:`) | curated                                           | Long-lived desired end-state + verification criteria                   |
| `state.md`         | `update-state`                                                     | overwrite                                         | Factual snapshot of what exists at `HEAD` (records `assessed-at` hash) |
| `next.md`          | `define-next`                                                      | overwrite                                         | Exactly one small, verifiable work package                             |
| `handoff.md`       | `advance` then `review`                                            | overwrite                                         | Inter-role communication; latest review verdict is authoritative       |
| `learnings.md`     | `review`                                                           | append-only (occasionally prune/merge for signal) | High-signal pitfalls, patterns, and verified conventions               |
| `issues.md`        | humans + agents                                                    | append-only; review deletes resolved              | Lightweight backlog that define-next can prioritize                    |
| `iterations.jsonl` | CID tooling/agents                                                 | append-only                                       | Execution log (turns/cost/duration/status) for iteration accounting    |
| `specs/`           | human + review (when authorized via `issues.md`)                   | curated                                           | Deeper specs referenced by `target.md`                                 |

## Hygiene rules (recommended)

- **No duplication**: if a fact is stable and belongs in a spec, put it in `specs/*` and link from
    `target.md`/`state.md` rather than repeating it.
- **State is not a spec**: `state.md` should be mostly *evidence + gaps*, not long descriptions of
    already-met targets.
- **Leavings pruning**: if `learnings.md` grows, merge duplicates and delete obsolete entries (git
    history keeps it). Keep the top of each section “high-signal”.
- **Issues triage**: keep issues actionable; prefer “what is wrong + where + how to verify” over
    open-ended discussion.
- **Log rotation** (optional): when `iterations.jsonl` becomes large, split by month into an
    `archive/` directory and keep only the recent window here.
