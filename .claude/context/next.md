# Next Work Package

## Step: Realign agent-facing specs and CLAUDE.md files with the code

## Goal

Close two `normal` issues that both concern instruction files agents read as truth: "Specs and the
core `CLAUDE.md` contradict the code in four places" [audit] and "`ci-cd.md` describes a tag trigger
the release workflow does not have". Wrong statements there get implemented or "fixed back" into the
code; both block v0.8.0.

## Alternatives Considered

- **Chosen:** spec/CLAUDE.md drift bundle — docs-only, so it adds nothing CI-risky on top of the
    three unpushed `cid(human)` commits (Python 3.15 leg, .NET 8/10 matrix). If the next push goes
    red, the cause is attributable to the human CI changes. Both issues share
    `specs/swift-bindings.md` and the same fix shape.
- **Rejected:** Ruby floor 3.1 → 3.3 — edits `ci.yml` and `release.yml` and needs cross-gem image
    research. Stacking a second CI change on unverified CI changes muddies a red run. Take it after
    the human CI changes have gone green.
- **Rejected:** Go canonical `meta` (JCS key order, `-0`, lone surrogates) — the lone-surrogate
    vector needs every harness's JSON parser to accept `\ud800` in `data-edge.json` (serde_json
    rejects it). That is a multi-surface risk and gets its own step.

## Scope

- **Modify** (all docs/specs, none count toward the file cap):
    - `.claude/context/specs/ci-cd.md` — Workflow Files table Release row (line ~10), "Tag trigger as
        convenience" bullet (~179), `push: tags: [v*.*.*]` in the Trigger Configuration YAML
        (~191-193)
    - `.claude/context/specs/swift-bindings.md` — Release Flow steps 1-2 (~239-240) and "Named
        parameters with defaults" (~147)
    - `.claude/context/specs/c-ffi-dx.md` — "Tag push triggers FFI builds" (~128)
    - `.claude/context/specs/python-bindings.md` — `py.allow_threads(...)` (~274) → `py.detach(...)`
    - `.claude/context/specs/rust-core.md` — JS recipe `const [mt, st, …] = iscc_decode(…)` (~485)
    - `crates/iscc-py/CLAUDE.md` — "Tag-triggered" bullet (~134)
    - `crates/iscc-wasm/CLAUDE.md` — "tag-triggered CI … tag version" (~124)
    - `crates/iscc-lib/CLAUDE.md` — binding crate list (lines 7-8) lacks `iscc-uniffi`; Tier 2 list
        (~40-44) lacks `read_unit` and `SEQ_MAINTYPES`
- **Reference**: `.github/workflows/release.yml` (trigger, `prepare-release` tag creation, Swift
    checksum/force-tag job, WASM version patch), `crates/iscc-uniffi/src/lib.rs`,
    `crates/iscc-napi/src/lib.rs` + `crates/iscc-wasm/src/lib.rs` (`iscc_decode` return shape),
    `crates/iscc-lib/src/codec.rs` (`read_unit`, `SEQ_MAINTYPES`), the correct paragraph at
    `specs/ci-cd.md` ~224-229

## Not In Scope

- Any edit to `.github/workflows/*.yml` or code. This step describes the code; it changes none.
- Other drift in these files (e.g. the iscc-py CLAUDE.md wheel matrix omitting aarch64): note it in
    the handoff, do not fix it here
- `notes/` — never updated in sweeps (root CLAUDE.md)
- Adding file lists to the per-crate `CLAUDE.md` files (decision 2026-11-21: they carry none — the
    binding-crate list in `iscc-lib/CLAUDE.md` is a role statement; keep it a short prose list)
- Other open `normal` issues (architecture.md, llms.txt, KAA comments, pkg-config version)

## Implementation Notes

- Read `release.yml` before writing each replacement. Every replacement states what the workflow
    does: `workflow_dispatch` is the only trigger; with a `version` input, `prepare-release` creates
    the `vX.Y.Z` / `packages/go/vX.Y.Z` tags and the GitHub Release. Do not invent a mechanism.
- ci-cd.md Trigger Configuration: drop the `push:` block and keep the `workflow_dispatch` inputs as
    excerpted. Replace the "Tag trigger as convenience" bullet with the `version`-input behaviour or
    remove it, whichever reads cleaner next to the ~224 paragraph. Do not duplicate that paragraph.
- swift-bindings.md Release Flow: describe the dispatch → tag → XCFramework → `Package.swift` update
    → force-update tag sequence as the Swift job implements it. Line ~147: say the UniFFI exports
    declare no defaults, so Swift/Kotlin callers pass every argument (check the `#[uniffi::export]`
    signatures for any `default(...)` first).
- rust-core.md ~485: destructure the object the napi/WASM `iscc_decode` returns. Take the field
    names from the `.d.ts` or the `#[napi(object)]` struct; do not guess.
- python-bindings.md ~274: also sweep that spec for any other `allow_threads` mention.
- After editing, sweep once for the same false claims elsewhere in `.claude/context/specs/` and the
    per-crate `CLAUDE.md` files: `git grep -n -i -E 'tag[- ]trigger|tags: \[v|allow_threads'`. Fix
    hits that state the false behaviour, and list them in the handoff.
- `specs/*.md` edits trigger the `check_spec_paths` prek hook; keep every backticked path real.

## Verification

- `git grep -n -i -E 'tag[- ]triggered|tag trigger as|tags: \[v\*|Tag push triggers' -- .claude/context/specs crates/*/CLAUDE.md`
    prints nothing
- `git grep -n 'allow_threads' -- .claude/context/specs` prints nothing, and
    `git grep -n 'const \[mt' -- .claude/context/specs` prints nothing
- `grep -q iscc-uniffi crates/iscc-lib/CLAUDE.md && grep -q read_unit crates/iscc-lib/CLAUDE.md && grep -q SEQ_MAINTYPES crates/iscc-lib/CLAUDE.md`
    exits 0
- `git grep -n 'parameters with defaults' -- .claude/context/specs/swift-bindings.md` prints nothing
- `uv run scripts/check_spec_paths.py` and `uv run scripts/check_tier1_docs.py` exit 0
- `mise run check` passes; the step's commits touch only `.claude/context/specs/`,
    `crates/*/CLAUDE.md` and CID context files

## Done When

All seven trigger claims and the four audit contradictions describe the code as it is, and every
verification check passes.
