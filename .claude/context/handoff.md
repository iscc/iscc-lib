## 2026-11-18 — Review of: `text_trim` / `text_remove_newlines` strip U+001C–U+001F in Rust and Go

**Verdict:** PASS_WITH_NOTES

**Summary:** Rust `text_trim` (both paths) and `text_remove_newlines` now trim and split on the
Python `str.strip()` set via `codec::is_py_strip_char` (made `pub(crate)`, still outside Tier 2). Go
`TextTrim` / `TextRemoveNewlines` use `TrimFunc` / `FieldsFunc` with `isPyStripChar`. The tests
cover each trim site: the short path, a separator at the truncation cut, and a split multi-byte
strip char. A live-derived Python differential checks both directions. I re-ran every gate myself.
The only note is a small Go speed cost (below), which is accepted and recorded.

**Verification:**

- [x] `cargo test -p iscc-lib`: 427 passed (302+17+28+44+13+22+1), with the new separator tests for
    both `text_trim` paths and `text_remove_newlines`. CI feature matrix also green:
    `--no-default-features` 363, `--features text-processing` 396, `--all-features` 427
- [x] `uv run pytest tests/test_text_utils.py` after `maturin develop`: 40 passed, including the
    live-derived strip-set differential and the U+200B kept-by-both case
- [x] `CGO_ENABLED=0 go -C packages/go test -count=1 ./...` `ok`; `go vet` clean; `gofmt -l` empty.
    U+001C–U+001F are written out literally for `TextTrim` (short and truncation paths) and
    `TextRemoveNewlines`
- [x] `mise run bench:iai:check` exit 0, `.iai-baseline.json` unmodified (16 benches, 0 regressed;
    meta_code −4.4% / −4.5%, inside the drift other benches show too)
- [x] `cargo crap … --fail-regression --fail-above` exit 0 after `mise run coverage` (0 regressed, 0
    new, 0 moved, 119 unchanged; rustc 1.100.0). `mise run crap:baseline` regenerates the committed
    `.crap-baseline.json` byte for byte
- [x] `mise run check` exit 0. `cargo clippy -p iscc-lib --all-targets -- -D warnings` is clean, and
    so are clippy `--workspace --all-targets` and `--no-default-features`
- [x] Pre-push Python gates on the new test code (release build): `ty check`, ruff `S`, ruff `C901`
    all pass, and the full `pytest` gives 517 passed with coverage enforced

**Issues found:**

- Go `FieldsFunc` loses the ASCII fast path of `strings.Fields`. A 6-word title takes about 2× as
    long (502 → 1000 ns/op). Allocations are equal up to 32 fields and higher beyond that (512
    fields: 2 → 6). Accepted as a trade-off in decisions.md (2026-11-18, "Go text utilities drop the
    … ASCII fast path"). Not filed as an issue
- `.crap-baseline.json` was refreshed even though the local gate was green against the old file.
    That follows learnings: moved lines turn CI red. Only `line:` values shifted, plus one test
    helper entry (`assert_base32_error`, CRAP 2.0) that was missing before. Scope: 3 source files
    plus this generated baseline, the same pattern accepted for the ISCC-SEQ core step (195)
- Resolved issue deleted: "`text_trim` / `text_remove_newlines` keep U+001C–U+001F". The spec
    wording "trim whitespace" is still accurate, so no spec edit was needed

**Codex review:** Two runs. On `47ad4bd` (the main commit), one P2 finding: Go `TextRemoveNewlines`
should keep `strings.Fields` when the input has no U+001C–U+001F. I reproduced the numbers and
rejected the change with reasons in decisions.md: two code paths for a sub-microsecond gain on a
path with no perf gate, and typical names allocate no more than before. On `HEAD` (the handoff fixup
commit): no findings.

**Next:** The text-utils issue is closed. The other 7 `normal` issues are still open. Candidates:
(a) the next ISCC-SEQ surface (Node/napi; 2 of 11 done, Rust and Go), (b) the small Python
`encode_seq` docstring issue (`[human]`), (c) the `uv.lock` Dependabot alerts (`soupsieve` has two
**high** alerts; dev-only, but quick to fix), (d) the Ruby `json` Dependabot alert, the Ruby locale
fixture, the `@iscc/wasm` license and the `c-ffi-api.md` type names. The bindings get the text-utils
fix through the core, so the SEQ differentials no longer inherit the U+001C gap.

**Notes:**

- Every surface now uses the Python whitespace set for codec input and for both text utilities.
    `text_clean` and `text_collapse` were deliberately left alone, because every Cc character is
    filtered out before their trims
- Meta-Code / Text-Code output does not change: `text_clean` drops U+001C–U+001F before
    `text_remove_newlines` / `text_trim` run inside `gen_meta_code_v0`
- Binding docstrings and docs still say "collapse whitespace", which remains accurate. No docs work
    is owed
