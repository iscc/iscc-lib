# CID Context Pack (`.claude/context/`)

This directory is the **shared working memory** for the CID (Continuous Iterative Development) loop.
It exists to keep agent prompts small, stable, and unambiguous.

## Design goals

- **Small**: keep each file concise; move deep details into `specs/` (or the codebase) and link.
- **Self-describing**: every file should make it clear *who updates it* and *how*.
- **Mechanically checkable**: prefer checklists, counts, and commands over prose.
- **Stable anchors**: avoid renaming headings that other files link to.

## Files (ownership + update policy)

| File                   | Primary owner                                                      | Update style                             | Budget    | Purpose                                                                |
| ---------------------- | ------------------------------------------------------------------ | ---------------------------------------- | --------- | ---------------------------------------------------------------------- |
| `target.md`            | human + review (when `issues.md` says `Source: [human]` + `Spec:`) | curated                                  | —         | Long-lived desired end-state + verification criteria                   |
| `state.md`             | `update-state`                                                     | overwrite                                | 200 lines | Factual snapshot of what exists at `HEAD` (records `assessed-at` hash) |
| `next.md`              | `define-next`                                                      | overwrite                                | 120 lines | Exactly one small, verifiable work package                             |
| `handoff.md`           | `advance` then `review`                                            | overwrite                                | 100 lines | Inter-role communication; latest review verdict is authoritative       |
| `learnings.md`         | `review`                                                           | append + prune to `learnings-archive.md` | 200 lines | High-signal pitfalls, patterns, and verified conventions               |
| `issues.md`            | humans + agents                                                    | append; review deletes resolved          | 300 lines | Defect tracker (40 lines per entry, no progress logs)                  |
| `decisions.md`         | `review` + humans                                                  | append; runner rotates oldest out        | 400 lines | Binding design rationale a reader cannot reconstruct from the code     |
| `decisions-archive.md` | `tools/cid.py`                                                     | append (rotation target)                 | —         | Decisions aged out of budget; titles stay in context, bodies do not    |
| `iterations.jsonl`     | `tools/cid.py` (sole writer + committer)                           | append-only                              | —         | Execution log (per-role + `iteration_summary` rows) for accounting     |
| `metrics.jsonl`        | `tools/metrics.py` via the runner                                  | append-only                              | —         | Codebase-health snapshots on the audit cadence                         |
| `proposals.md`         | `meta-improve`                                                     | overwrite                                | —         | Open self-improvement proposals awaiting human review                  |
| `meta-log.jsonl`       | `meta-improve`                                                     | append-only                              | —         | Audit trail of auto-applied loop changes + their rollback outcomes     |
| `specs/`               | human + review (when authorized via `issues.md`)                   | curated                                  | —         | Deeper specs referenced by `target.md`                                 |

## Context packs (how these files reach the agents)

Each role has a **context pack** skill at `.claude/skills/cid-ctx-<role>/SKILL.md` that inlines the
files that role reads unconditionally. `tools/cid.py` invokes it as a prompt prefix
(`/cid-ctx-review CID iteration 162. …`), Claude Code expands the pack's `` !`cat …` `` blocks
before the agent's first turn, and the task text lands in the pack's `$ARGUMENTS`.

This is why the agent definitions no longer carry `@file` blocks: **`@path` imports and
`` !`command` `` blocks are inert in `.claude/agents/*.md`** — the body is used verbatim as the
system prompt. `@path` works in CLAUDE.md, `` !`command` `` works in skills, and neither works in an
agent definition (measured on Claude Code 2.1.220). The `skills:` frontmatter field is not an
alternative: it preloads only into *spawned subagents*, and CID roles run as the main session via
`--agent`.

The packs cost nothing extra — these are files the roles read anyway — but they remove a
decide-then-read round trip per file and make double-reads impossible. Rendered payloads are 9–19k
tokens per role; `define-next` is the largest because it carries the full `target.md`.

Three rules when editing a pack:

- Inline only what the protocol reads **every** run. Anything conditional stays an on-demand read.
- Prefer a projection over a whole file when the role only needs a slice — `update-state` gets issue
    *titles* (it only needs the DONE rule), and `review`/`define-next` get decision *titles*.
- Never frame the inlined content as a cost or a context-limit risk. Telling a model it is near its
    budget makes it trim its own work, and the role most exposed to that is `review` — the quality
    gate. State the fact ("already in context; re-reading returns the same bytes") and leave token
    accounting to `tools/cid.py`, which measures it anyway.

A missing pack degrades safely: `cid.py` checks for the `SKILL.md` on disk and falls back to a plain
task prompt. The packs use POSIX `cat`/`grep`/`tail`, matching the loop's existing shell assumptions
(`git`, `jq`, `timeout`).

## Artifact budgets

Budgets are defined in `ARTIFACT_BUDGETS` in `tools/cid.py`, not in the role prompts, so they
survive a model change. After every iteration the runner:

1. rotates the oldest `decisions.md` entries into `decisions-archive.md` if it is over budget — the
    only artifact that can be trimmed mechanically, because it is strictly chronological;
2. measures the rest and records any overrun in that iteration's `iteration_summary` row, so
    artifact growth is a visible trend rather than something found by diffing files;
3. tells the owning role, at the start of its next run, which of its own artifacts are over.

They exist because unbudgeted artifacts grow without bound: across the 2026-07-24 model change every
artifact without a cap grew 24–174% while `learnings.md`, the only one with a cap, did not move.
Each iteration's output is the next one's input, so unchecked growth compounds. Full analysis in
`cauldron/cid-post-mortem.html`.

## Hygiene rules

- **No duplication**: if a fact is stable and belongs in a spec, put it in `specs/*` and link from
    `target.md`/`state.md` rather than repeating it.
- **State is not a spec**: `state.md` should be mostly *evidence + gaps*, not long descriptions of
    already-met targets.
- **Learnings pruning**: if `learnings.md` grows, merge duplicates and delete obsolete entries (git
    history keeps it). Keep the top of each section “high-signal”.
- **Issues are defects, not a queue**: “what is wrong + where + how to verify”, then deleted. No
    progress logs — git history already has them, and a step queue inside an issue removes the gap
    analysis `define-next` exists to perform.
- **Log rotation** (optional): when `iterations.jsonl` becomes large, split by month into an
    `archive/` directory and keep only the recent window here.
