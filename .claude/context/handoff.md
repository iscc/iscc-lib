## 2026-11-22 — Review of: Every WASM snippet initializes the module; WASM agent guidance names only the `--target web` build

**Verdict:** PASS

**Summary:** All eight `@iscc/wasm` snippets on the README, the docs home page and the
getting-started tutorial now call `await init();` under a comment that sends Node.js readers to the
how-to. `crates/iscc-wasm/CLAUDE.md` and `specs/wasm-bindings.md` name only the published
`--target web` build, and the release command they quote matches `release.yml` (the `run` line and
its `RUSTFLAGS` env). `tests/test_wasm_docs.py` guards both rules. The diff touches five doc and
guidance files and one test, no code, and nothing listed under Not In Scope.

**Verification:**

- [x] `uv run pytest tests/test_wasm_docs.py` passes, including the new snippet, guidance and
    negative tests — 17 passed
- [x] `grep -nE -- '--target (nodejs|bundler)' crates/iscc-wasm/CLAUDE.md .claude/context/specs/wasm-bindings.md`
    prints nothing — exit 1, no output
- [x] `uv run zensical build` exits 0 — "No issues found"
- [x] `uv run pytest tests/test_check_spec_paths.py tests/test_check_tier1_docs.py` pass — 58 passed
- [x] `mise run check` exits 0 — every hook passed on the first run

Probes beyond the list:

- The new rule helpers, run over the pages as they were before the step, report 5 snippets without
    init and 3 unlabelled bare `init()` calls, and both unpublished targets in each guidance file.
    On the current pages they report nothing
- A repo-wide `git grep` finds `--target nodejs|bundler` only in `notes/`, agent memory, archives,
    the test's own ban list and one issue that reports an old measurement. Every `from "@iscc/wasm"`
    import in the repository sits on a page a test now covers
- `ty` and ruff `S`/`C901` pass on the edited test file

**Issues found:**

- The snippet rule saw only fences tagged `javascript`: a future tab tagged `js` or `typescript`
    would have escaped it. Fixed in this review: the page test now requires that every `@iscc/wasm`
    import on a snippet page sits inside a checked block. Proven on a `git archive` copy with one
    tutorial fence retagged `js` (the test fails with "imports @iscc/wasm outside a javascript
    block")
- Nits, not filed: an `init(` inside a comment satisfies the rule, and the rule does not check that
    `init()` comes before the first call
- The issue "WASM init and build-target guidance is wrong outside the two pages a test guards" is
    resolved and deleted

**Codex review:** No actionable findings. Codex confirmed that the init examples and the build
guidance match the WASM wrapper and the release workflow.

**Next:** Five `normal` issues remain. Suggested order:

1. The Rust and Go conformance runners disagree on malformed vectors (one wrong-type rule for
    `conformance.rs` and `conformance.go`). It needs a decisions.md entry and the CRAP baseline in
    the same step if a covered function gains a branch
2. `version:check` passes a stale second doc snippet and skips a vanished target (one script plus
    tests). It is the smallest of the five and guards the release

The other three (`alg_minhash_256([])`, Go `EncodeComponent` short digest, WASM hasher `finalize`)
each start with a behaviour decision and, for the first two, a `data-edge.json` fan-out.

**Notes:**

- The spec edit is a realignment of checkable facts (the build command list and the published
    target), covered by the decision of 2026-11-20. No new decision was needed
- `docs/howto/wasm.md` per-function snippets still omit `init()` under the page's "Setup" section.
    next.md accepted that form, and they are outside `SNIPPET_PAGES`
- learnings.md is at its 200-line budget: a new bullet needs an equal cut in the same commit
