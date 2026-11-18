## 2026-11-18 — Core `iscc_clean` trims exactly what Python `str.strip()` trims (U+001C–U+001F)

**Done:** Added the private predicate `is_py_strip_char` to `codec.rs`. It returns true for
`char::is_whitespace` or U+001C..=U+001F, and its doc comment names `str.strip()`. All four trim
sites now call `trim_matches(is_py_strip_char)`: the outer trim in `iscc_clean`, the scheme trim,
the code trim, and the `encode_seq` empty-unit guard. The `Cow` borrow paths are unchanged, and the
change adds no allocations.

**Files changed:**

- `crates/iscc-lib/src/codec.rs`: the predicate, the 4 trim sites, an `iscc_clean` doc tweak, and a
    unit test comparing the predicate with `char::is_whitespace` over every char. The test also
    checks that U+200B is not stripped.
- `crates/iscc-lib/tests/codec_clean.rs`: 7 new tests.
    - Each of the 4 separators placed around the string, before the colon, and after the colon. Run
        through both `iscc_decode` and `iscc_decompose`, each result must equal the clean
        `ISCC:GAAAAAAAAA` result.
    - A sweep over all scalar values that Python strips (Rust whitespace plus the 4 separators).
    - U+200B is rejected.
    - `encode_seq` strips the separators.
    - A unit of just a separator errors with "must not be empty". The test asserts the guard's
        message, so it fails if the guard regresses: the old guard falls through to "Empty ISCC
        string".
    - A premise test that the separators are not Rust whitespace.
- `tests/test_seq.py`: 10 new tests.
    - The strip set is derived live from the interpreter, and the test asserts it contains the 4
        separators.
    - For every char in the set, `iscc_core.encode_seq` and `iscc_lib.encode_seq` return identical
        bytes. iscc_lib exceptions are caught, so a mismatch shows up as a diff, not a crash.
    - Separators before and after the colon: both libraries agree.
    - A separator-only unit is rejected by both libraries.
    - U+200B is rejected by both libraries.
- `.crap-baseline.json`: regenerated with `mise run crap:baseline` from the same `lcov.info`.
    - The diff only moves line numbers for 6 functions: 5 in `codec.rs` (shifted by the predicate and
        its doc) and 1 in `conformance.rs`. It also adds 2 functions (`is_py_strip_char` and the test
        helper). One coverage number changed: `run_encode_seq_tests` went from 80.8% to 100%.
    - Before the refresh the gate exited 0 with 0 regressed and 6 moved. I refreshed anyway because of
        the CI-only moved-lines gap in learnings.

**Verification:**

- `cargo test -p iscc-lib`: every suite passed (297 lib tests, 17 `codec_clean`, plus the rest).
- `grep -nE '\.trim\(\)' crates/iscc-lib/src/codec.rs` prints nothing.
- `uv run maturin develop -m crates/iscc-py/Cargo.toml` then `uv run pytest tests/test_seq.py`: 77
    passed. Full `uv run pytest`: 507 passed.
- `mise run bench:iai:check` exits 0 against the unmodified `.iai-baseline.json`. All 16 benches are
    within 10%. `four_units` is 12,784 Ir against a baseline of 13,369 (−4.38%), and no bench rose
    by more than 0.05%.
- `mise run coverage`, then CI's exact
    `cargo crap --lcov lcov.info --baseline .crap-baseline.json --fail-regression --fail-above`:
    exit 0, both before the baseline refresh (0 regressed) and after it (118 unchanged). Local rustc
    is 1.100.0 (2026-11-09).
- `mise run check`: the first run failed on a `ruff-check` auto-fix. The re-run exits 0.
    `git status` shows only the 4 intended files plus the runner's `iterations.jsonl`.
- `cargo clippy -p iscc-lib --all-targets -- -D warnings`: clean.

**Next:** The Go `decodeBase32` spare-bit fix plus Rust/Go strict-base32 tests. Then the Go
`isccClean` port, which must copy this exact strip set: Go's `unicode.IsSpace` differs from Rust's
whitespace set, so derive the Go set rather than assuming it. The spec's Python criterion that both
libraries reject the same inputs is now unblocked for review to re-check.

**Notes:**

- **Text utilities diverge on Tier 1, confirmed by probe, not fixed (out of scope).** Input
    `'\x1eab\x1fcd\x1e'`:
    - `text_trim(s, 10)`: iscc_core gives `'ab\x1fcd'`, iscc_lib gives `'\x1eab\x1fcd\x1e'`.
    - `text_remove_newlines(s)`: iscc_core gives `'ab cd'`, iscc_lib gives `'\x1eab\x1fcd\x1e'`.
    - `text_collapse` agrees (`'abcd'`), because the C-category filter drops the separators.
    - Cause: `.trim()` in `utils.rs` L125/L132 and `split_whitespace` in L94/L107/L115.
    - Review should file an issue. It may also affect Meta-Code output via `text_trim`.
- The CRAP report includes test-file helpers (`is_python_whitespace` in `codec_clean.rs`, coverage
    `—`), as it already did for `test_seq.rs`.
