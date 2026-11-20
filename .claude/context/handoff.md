## 2026-11-20 — Review of: Move the iscc-ffi unit tests out of `lib.rs` into `src/tests.rs`

**Verdict:** PASS

**Summary:** The `#[cfg(test)] mod tests` block of `crates/iscc-ffi/src/lib.rs` now lives in
`crates/iscc-ffi/src/tests.rs`; `lib.rs` is 1717 lines and keeps every export, type and helper. The
move is verbatim: the first 1715 lines of `lib.rs` are byte-identical to the previous version, and
the de-indented old test body differs from `tests.rs` in two places only, both a line rustfmt joined
because it now fits in 100 columns. `crates/iscc-ffi/CLAUDE.md` describes the new layout and says
why exports must stay in `lib.rs`.

**Verification:**

- [x] `grep -n 'mod tests' crates/iscc-ffi/src/lib.rs` prints one line, `mod tests;`, with
    `#[cfg(test)]` above it; `wc -l` below 2500 — lines 1716/1717, file is 1717 lines
- [x] `grep -c '#\[test\]'` is `0` in `lib.rs`; the `cargo test -p iscc-ffi --lib` passed count
    equals the count in `tests.rs`, 0 failed — 99 passed, 99 `#[test]`, and the set of test function
    names is identical to the previous file's
- [x] cbindgen plus `cargo build -p iscc-ffi` leave `iscc.h` and `NativeMethods.g.cs` unchanged —
    cbindgen written to a temp file is byte-identical to the tracked header; after
    `cargo clean -p iscc-ffi` the build rewrote `NativeMethods.g.cs` and `git status` stayed clean
- [x] `grep -rn 'no_mangle\|extern "C"' crates/iscc-ffi/src/tests.rs` prints nothing
- [x] `cargo clippy -p iscc-ffi --all-targets -- -D warnings` and `cargo fmt -p iscc-ffi --check`
    are clean — workspace clippy is clean too
- [x] `mise run check` exits 0

**Issues found:**

- (none). Resolved and deleted: "`crates/iscc-ffi/src/lib.rs` is past the crate's own split
    threshold"

**Codex review:** No findings. It confirmed all 99 tests moved without behaviour change and that the
regenerated C and C# bindings are unchanged.

**Next:** The `[human]`-sourced `normal` issue "`data-edge.json` harnesses accept a value that only
resembles the expected one": make the Swift compare match strings by Unicode scalars and reject
Boolean-backed numbers, and make the Python harness tell `0` from `False`. It needs a Swift
toolchain run and touches two test harnesses. After that, the derived-facts parity check (the other
open `normal` issue).

**Notes:**

- No quality-gate change in the unpushed range: no suppression, skipped test or loosened hook. No
    public API, core Rust or benchmarked path changed, so semver, CRAP and iai were not run
- cbindgen does not rewrite an unchanged output file, so a file timestamp says nothing about whether
    it ran. Compare against a temp-file run instead. `cargo build -p iscc-ffi` reruns `build.rs`
    only when cargo sees the package as changed; `cargo clean -p iscc-ffi` first forces it
- No learnings or decisions entry this round: `learnings.md` is at its 200-line budget and nothing
    here outranks an existing entry
