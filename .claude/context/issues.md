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

## [low] Evaluate unofficial TypeScript port branciard/iscc-core-ts

An unofficial TypeScript implementation of ISCC exists at `branciard/iscc-core-ts`. Two actions:

1. **Conformance check**: verify whether it passes the official `data.json` test vectors. If it
    does, it could be referenced as a community implementation. If not, note the gaps.
2. **Documentation mention**: if conformant (or partially conformant), mention it in the iscc-lib
    documentation site (e.g., in an "Ecosystem" or "Related Projects" section) as an independent
    community port alongside iscc-lib's own bindings.

This is not urgent — iscc-lib's own Node.js/WASM bindings will serve the JS/TS ecosystem. But
acknowledging community implementations builds goodwill and helps adopters find options.

**Source:** [human]

## [low] iscc-jni: All exceptions mapped to `IllegalArgumentException`

In `crates/iscc-jni/src/lib.rs:34`, the `throw_and_default` function always throws
`java/lang/IllegalArgumentException` for all error types. State violations (e.g., hasher already
finalized) should throw `IllegalStateException` instead.

Fix: add a `throw_state_error` variant that throws `IllegalStateException` and use it for
state-related errors (finalized hashers, etc.).

**Source:** [human]

## [low] iscc-wasm: Stale CLAUDE.md says DataHasher/InstanceHasher not yet bound

In `crates/iscc-wasm/CLAUDE.md:130-131`, the documentation states "DataHasher and InstanceHasher
(streaming types) are not yet bound." Both are now fully exported in `lib.rs` with constructor,
`update()`, and `finalize()` methods.

Fix: update CLAUDE.md to reflect the current state of the bindings.

**Source:** [human]

## [normal] Create version sync tooling

The project needs `mise run version:sync` and `mise run version:check` tasks to keep non-Cargo
manifests in sync with the workspace version. Currently `package.json` and `pom.xml` must be updated
by hand.

Required deliverables:

1. **`scripts/version_sync.py`** — reads `workspace.package.version` from root `Cargo.toml`,
    updates:
    - `crates/iscc-napi/package.json` (`"version"` field)
    - `crates/iscc-jni/java/pom.xml` (`<version>` element — drop `-SNAPSHOT` suffix if present, or
        keep it based on a flag)
2. **`mise run version:sync`** task in `mise.toml` — runs the sync script
3. **`mise run version:check`** task in `mise.toml` — runs the sync script in validation mode (exits
    non-zero if any manifest is out of sync)
4. The sync script must be cross-platform (Python, no platform-specific dependencies)
5. Use simple string/regex parsing — no TOML/XML library dependencies beyond the standard library

Not blocking the first 0.0.1 release (all manifests already at 0.0.1), but required before any
subsequent version bump.

**Source:** [human] **Spec:** .claude/context/specs/ci-cd.md#sync-tooling
