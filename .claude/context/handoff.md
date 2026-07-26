# Handoff

## 2026-07-26 — Propagate the Unicode boundary fixture to the WASM and Ruby test suites (propagation slice 2)

**Done:** Gated the Unicode 16.0.0 sentinel freeze rule on two more binding surfaces. The WASM suite
gains `tests/unicode_boundary.rs` (canonical fixture via `include_str!`, no vendored copy; metadata
guard + one looping test per section, ungated by the `conformance` feature) and the Ruby suite gains
`test/test_unicode_boundary.rb` (`define_method` per fixture case with fresh
`BOUNDARY_JSON`/`BOUNDARY_DATA` constants, plus a metadata guard). All 12 vectors pass on both
surfaces with zero skips. Criterion 3 moves from 2 of 11 surfaces to 4 of 11. Zero non-test, non-doc
source files changed.

**Files changed:**

- `crates/iscc-wasm/tests/unicode_boundary.rs`: new — 3 `#[wasm_bindgen_test]` fns (metadata guard
    asserting version `16.0.0` + counts 7/5; `text_clean` and `text_collapse` loops passing the case
    name into each `assert_eq!` message)
- `crates/iscc-rb/test/test_unicode_boundary.rb`: new — `TestUnicodeBoundary < Minitest::Test`, one
    named test per vector + metadata guard; only fixture-derived strings, no non-ASCII literals in
    the file
- `docs/unicode.md`: boundary-vector paragraph now names the Python, WASM, and Ruby binding suites
    as direct readers of the canonical fixture
- `crates/iscc-wasm/CLAUDE.md`: `tests/` layout block lists `unicode_boundary.rs`
- `crates/iscc-rb/CLAUDE.md`: file table gains a `test/test_unicode_boundary.rb` row (mdformat
    realigned the whole table — that is the only reason other rows show whitespace diffs)

**Verification:** (all run this session)

- `wasm-pack test --node crates/iscc-wasm --features conformance` — exit 0; per-target results:
    conformance 9 passed, **unicode_boundary 3 passed / 0 failed**, unit 78 passed (baseline
    maintained), doctests 0
- In `crates/iscc-rb`: `bundle exec rake compile` — exit 0; `bundle exec rake test` — **124 runs**
    (111 baseline + 12 vectors + 1 metadata guard), 314 assertions, **0 failures, 0 errors, 0
    skips**; `bundle exec standardrb` — exit 0, no output
- `cargo clippy --workspace --exclude iscc-rb --all-targets -- -D warnings` — exit 0 (new test
    target compiles on host); `cargo clippy -p iscc-rb -- -D warnings` — exit 0
- `cargo fmt --all --check` — exit 0
- Skip greps: `#\[ignore\]` in the wasm file and `skip[ (]` in the Ruby file both find nothing (exit
    1\)
- `find . -name unicode_boundary.json …` — exactly 2 paths (canonical + Go vendored); no new copy
- `git status --porcelain crates/iscc-lib/ packages/go/ .crap-baseline.json .iai-baseline.json` —
    empty
- `cargo test -p iscc-lib` — 336 passed (281+28+22+4+1), 0 failed, unchanged
- `mise run check` — exit 0, every prek hook Passed, no reformats of the new files on re-run
- `uv run zensical build` — "No issues found"; `uv run scripts/check_docs_nav.py` — OK, 23 pages
- `grep -F -c 'unicode_boundary.rs' crates/iscc-wasm/CLAUDE.md` = 1;
    `grep -F -c 'test_unicode_boundary.rb' crates/iscc-rb/CLAUDE.md` = 1

**Next:** Land the small `[review]` vendored-copy byte-identity drift gate next — next.md's
Not-In-Scope explicitly sequences it *before* the `packages/{dotnet,kotlin,swift}` slice, which adds
three more `data.json`-family copies. After that, continue propagation: C FFI + JNI are the next
cheapest in-container pair; napi remains the most expensive slice (its checked-in
`iscc-lib.linux-x64-gnu.node` is stale and must be rebuilt first).

**Notes:**

- The Ruby extension `.so` was rebuilt via `rake compile` before `rake test` per next.md; the
    artifact is gitignored so this leaves no tree diff. The 124-run total exceeds next.md's "at
    least 123" because the metadata guard is itself a run (next.md counted only the 12 vectors).
- First `mise run format` invocation died with a transient `OSError: [Errno 22]` from mdformat on
    `crates/iscc-rb/CLAUDE.md`; the immediate re-run succeeded and reformatted the table, and two
    subsequent full runs (`format`, then `check`) were stable/clean. Looks like a one-off FS race,
    not a repo problem — flagging only in case the reviewer sees it again.
- No skip constructs, no feature gating on the WASM test (mirrors ungated `conformance.rs`), no
    oracle duplication — the `delete_filter_output` values stay in the Rust `SEQUENCE_VECTORS` const
    and issues.md table only, per next.md.
- `.claude/context/iterations.jsonl` shows modified in `git status`; that is the runner's file and
    is left unstaged per protocol.
