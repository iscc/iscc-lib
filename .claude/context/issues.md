# Issues

Tracked issues and feature requests for the CID workflow.

## Purpose

This file is a lightweight, append-only channel for humans and agents to flag problems that
define-next can prioritize. It bridges the gap between ad-hoc observations and the formal
state→target planning loop.

## Format

Each issue is an H2 heading with a priority tag, description, and source attribution:

```markdown
## [priority] Issue title

Description of the problem, including context and any relevant file paths or error messages.

**Source:** [human|review|advance]
**Spec:** .claude/context/target.md#section-name (optional — only if rooted in a spec gap)
**Upstream:** iscc/iscc-core (optional — only if the fix belongs in the upstream reference)
```

### Spec field

The optional `**Spec:**` field links an issue to a specific section in target.md or a sub-spec file
that needs updating. Source determines what happens:

- **`[human]` + `Spec:`** — the review agent updates the referenced spec directly when resolving the
    issue. The human created the issue, so the spec change is implicitly authorized.
- **`[review]`/`[advance]` + `Spec:`** — the review agent flags with `HUMAN REVIEW REQUESTED` and
    describes the proposed spec change in the issue description. It does NOT modify target.md. The
    human must approve and make the change.

### Upstream field

The optional `**Upstream:**` field marks an issue as belonging to an external repository (e.g.,
`iscc/iscc-core`). Upstream issues are always human-gated — filing on GitHub is a visible public
action that requires review regardless of source:

- Any issue with `**Upstream:**` triggers `HUMAN REVIEW REQUESTED` in the CID loop
- The issue description should include concrete evidence: failing conformance vectors, expected vs
    actual output, specific code references in the upstream repo
- The issue stays in issues.md as a draft until the human reviews it and either asks an interactive
    session to file it via `gh issue create -R <repo>` or files it manually
- After filing, update the issue description with the GitHub issue URL and delete it from issues.md

## Priority Levels

- **critical** — must be addressed in the next iteration; overrides normal gap analysis
- **normal** — considered alongside state→target gaps; define-next weighs it against other work
- **low** — pick up when no higher-priority work remains

## Management Rules

- **Append only** — add new issues at the end of this file
- **Resolution** — the review agent deletes resolved issues after verifying the fix (history lives
    in git)
- **Source tags** — agents that add issues must include a source tag: `[human]`, `[review]`, or
    `[advance]`
- **Scope** — track any problems that affect correctness, architecture, maintainability, or
    performance; not style preferences or minor nits

<!-- Add issues below this line -->
