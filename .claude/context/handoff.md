## 2026-11-18 — `text_trim` / `text_remove_newlines` strip U+001C–U+001F in Rust and Go

**Done:** Rust `text_trim` (both paths) and `text_remove_newlines` now use the Python `str.strip()`
/ `str.split()` set. They call `codec::is_py_strip_char`, which is now `pub(crate)`, so it stays out
of Tier 2. Go `TextTrim` / `TextRemoveNewlines` use `strings.TrimFunc` / `strings.FieldsFunc` with
`isPyStripChar`, which is already in the package. Not touched: `text_clean`, `text_collapse`,
`code_meta.go` and `.iai-baseline.json`.

**Files changed:**

- `crates/iscc-lib/src/utils.rs`: added `use crate::codec::is_py_strip_char;`. `split_whitespace()`
    → `split(is_py_strip_char).filter(|s| !s.is_empty())` (rustfmt chain-wrapped it). Both `trim()`
    → `trim_matches(is_py_strip_char)`. Docstrings updated. 5 tests added: the short path (the issue
    example plus each separator), the truncation path with the separators at the cut edge (cut sized
    from the encoded length) plus U+3000 at the edge, whole and split, and `remove_newlines` with
    separators leading, interior (single and run) and trailing. U+200B is kept by both functions
- `crates/iscc-lib/src/codec.rs`: `is_py_strip_char` is now `pub(crate)`. Visibility is the only
    change
- `packages/go/utils.go`: 3 sites (lines 91, 101, 109) changed, plus their doc comments
- `packages/go/utils_test.go`: 4 tests. U+001C, U+001D, U+001E and U+001F are pinned **literally**
    for `TextTrim` (short and truncation paths) and `TextRemoveNewlines` (leading, interior and
    trailing, plus the issue example). U+200B is kept by both
- `tests/test_text_utils.py`: `PY_STRIP_CHARS` is derived live (`c.strip() == ""` over all code
    points). Tests added:
    - the issue example
    - differentials against `iscc_core.code_meta.text_remove_newlines` and `text_trim`, for each strip
        char, with the char outer, interior and as a run. `text_trim` is checked at `nbytes = size`,
        `size + 8` and a cut right after the char
    - the reverse direction: chars in `Zs/Zl/Zp/Cc/Cf` minus the strip set are kept identically by
        both, including U+200B
- `.crap-baseline.json`: refreshed with `mise run crap:baseline`. The diff has 7 line moves
    (utils.rs fns +1 or +4, from the import and the rustfmt wrap) and 1 new entry,
    `test_strict_base32.rs::assert_base32_error` (CRAP 2.0). That entry was left over from iter 199
    and does not come from this step

**Verification:**

- `cargo test -p iscc-lib`: 427 passed, 0 failed.
    `cargo test -p iscc-lib --no-default-features   --lib utils`: 14 passed, so the ungated trim
    functions build without `text-processing`
- `cargo clippy -p iscc-lib --all-targets -- -D warnings`: clean (only the known `proc-macro-error2`
    future-incompat note)
- `uv run pytest tests/test_text_utils.py`: with the **stale** extension, 3 FAILED (the issue
    example and both strip-set differentials). That shows the tests detect the old behaviour. After
    `maturin develop -m crates/iscc-py/Cargo.toml`: 40 passed
- `CGO_ENABLED=0 go -C packages/go test -count=1 ./...`: `ok`. `go vet`: clean. `gofmt -l`: empty
- `mise run bench:iai:check`: exit 0 against the unmodified `.iai-baseline.json` ("16 without
    regressions"). meta_code name_only −4.43%, name_desc −4.54%. text_code −0.06%
- CRAP: after `mise run coverage`, the gate exited 0 against the old baseline (7 moved, 1 new). I
    refreshed the baseline anyway, because learnings say moved lines red in CI. Re-run against the
    new baseline: exit 0, with 0 regressed, 0 new, 0 moved and 119 unchanged. Local rustc is 1.100.0
    (0ef1bc386 2026-11-09)
- `mise run format` changed nothing. `mise run check` exit 0

**Next:** the next ISCC-SEQ surface (Node/napi; 2 of 11 done) or the small Python `encode_seq`
docstring issue. The text-utils issue in issues.md can be closed. Bindings inherit the fix from the
core: Python is proven by the differential, and the other bindings delegate to the core.

**Notes:**

- The core-side `split` + `filter` keeps the same allocation profile as `split_whitespace`, which
    std implements as the same `Filter<Split<…>>` shape. The iai gate agrees
- The Python test module adds `import iscc_core` and `import unicodedata`. Both are already used by
    `tests/test_seq.py`. `PY_STRIP_CHARS` is derived locally and not imported across modules, as
    next.md asked
- issues.md is not edited (review owns the ledger)
