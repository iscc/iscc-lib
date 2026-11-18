# Next Work Package

## Step: `text_trim` / `text_remove_newlines` strip U+001C–U+001F in Rust and Go

## Goal

Fix the `normal` issue "`text_trim` / `text_remove_newlines` keep U+001C–U+001F that iscc-core
strips". Both the Rust core and the pure-Go port get fixed, so every surface matches the reference
`str.strip()` / `str.split()` whitespace set (Rust `is_whitespace` / Go `unicode.IsSpace`, plus the
four information separators).

## Alternatives Considered

- **Chosen:** text-utils parity in Rust and Go. It is a Tier-1 correctness defect that every binding
    shows, it unblocks the release, it is small, and the matching predicate already exists in both
    codebases. The handoff names it.
- **Rejected:** the next ISCC-SEQ surface (Node/napi). 9 surfaces remain and the fan-out continues
    next. Fixing the core first means each binding's differential inherits correct text utilities.
    The Python `encode_seq` docstring issue is a separate area (iscc-py) and would push the budget.

## Scope

- **Modify**: `crates/iscc-lib/src/utils.rs`, `crates/iscc-lib/src/codec.rs` (visibility of
    `is_py_strip_char` only), `packages/go/utils.go`. Also `.crap-baseline.json`, only if the CRAP
    gate needs it.
- **Tests**: Rust unit tests in `utils.rs`'s test module, `tests/test_text_utils.py` (differential
    vs `iscc_core`), and `packages/go/utils_test.go`
- **Reference**: `reference/iscc-core/iscc_core/code_meta.py:181-199`, the strip-set block in
    `tests/test_seq.py` (around L274, `PY_STRIP_CHARS`), `codec.rs:565-572`, and `codec.go:527-533`
    (`isPyStripChar`)

## Not In Scope

- `text_clean`'s trims: Rust `utils.rs` `line.trim()` / final `.trim()`, and Go `utils.go:70,83`.
    Every Cc character is filtered out before them, so changing them only adds hot-path cost
- `packages/go/code_meta.go:102` `TrimSpace(payload)`. The payload is already cleaned text
- `text_collapse`, and any Meta-Code or Text-Code change. No conformance vector moves
- Making `is_py_strip_char` `pub` or Tier 2. Use `pub(crate)` only, because the spec fixes the Tier
    2 list
- Regenerating `.iai-baseline.json`. **Not authorised.** If the gate fails, stop and report it in
    the handoff
- The Python `encode_seq` docstring issue and other bindings' seq work

## Implementation Notes

- **Rust:** make `codec::is_py_strip_char` `pub(crate)` and call it from `utils.rs`. Use
    `crate::codec::is_py_strip_char`. Mind the `text-processing` cfg: the trim functions may be
    ungated, so check that `cargo test -p iscc-lib --no-default-features` still builds.
    - Sites: `text_remove_newlines` (one, `split_whitespace`) and `text_trim` (two, the short path and
        the truncation path)
    - Replace `trim()` with `trim_matches(is_py_strip_char)`
    - Replace `split_whitespace()` with `split(is_py_strip_char).filter(|s| !s.is_empty())`. Keep the
        join behaviour identical
- **Go:** in `utils.go:91,101,109`, use `strings.FieldsFunc(text, isPyStripChar)` and
    `strings.TrimFunc(..., isPyStripChar)`. The predicate is already in-package.
- **Hot path:** both functions run inside `gen_meta_code_v0` (`lib.rs:347-359`), which the iai
    benches cover. Add no allocations beyond today's. `valgrind` and `iai-callgrind-runner` are
    installed here.
- **CRAP (CI-only gate):** keep the edits line-neutral above the test module where you can. If lines
    move, run `mise run coverage`, then the gate below, then `mise run crap:baseline` if it reds on
    moved lines. Commit the result in this step, and note the local rustc in the handoff.
- **Tests, one input form per site** (learnings: outer-only forms missed a trim site in iter 200):
    - `text_trim`: one short-path case (`nbytes >= len`) and one truncation case where a separator
        lands at the cut edge. Size `nbytes` from the encoded length, since the strip set has
        multi-byte members
    - `text_remove_newlines`: separators at the leading, interior and trailing positions
    - Example from the issue: `text_trim("\x1eab\x1fcd\x1e", 64)` gives `"ab\x1fcd"`, and
        `text_remove_newlines` of the same string gives `"ab cd"`
- **Python differential** (`tests/test_text_utils.py`): derive `PY_STRIP_CHARS` live, as
    `test_seq.py` does. Do not import it across test modules.
    - For each char, compare `iscc_lib` and `iscc_core` on both functions, with the char outer and
        interior
    - Add the reverse direction: space-like chars Python keeps (e.g. U+200B) are kept by both
    - Run `maturin develop -m crates/iscc-py/Cargo.toml` first. A stale extension false-reds
- **Go tests** must pin U+001C, U+001D, U+001E and U+001F **literally** for both functions. A loop
    over `isPyStripChar` cannot see the predicate shrink.

## Verification

- `cargo test -p iscc-lib` passes, including the new separator tests for both `text_trim` paths and
    `text_remove_newlines`
- `uv run pytest tests/test_text_utils.py` passes after
    `maturin develop -m crates/iscc-py/Cargo.toml`, including the live-derived strip-set
    differential against `iscc_core` and the U+200B kept-by-both case
- `CGO_ENABLED=0 go -C packages/go test -count=1 ./...` passes and `go -C packages/go vet ./...` is
    clean, with literal U+001C–U+001F cases for `TextTrim` and `TextRemoveNewlines`
- `mise run bench:iai:check` exits 0 against the committed, unmodified `.iai-baseline.json`
- `cargo crap --lcov lcov.info --baseline .crap-baseline.json --fail-regression --fail-above` exits
    0 (after `mise run coverage`)
- `mise run check` exits 0 and `cargo clippy -p iscc-lib --all-targets -- -D warnings` is clean

## Done When

All six verification criteria pass, with `text_clean`, `text_collapse` and `.iai-baseline.json`
untouched.
