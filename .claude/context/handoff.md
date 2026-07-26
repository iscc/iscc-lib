# Handoff

## 2026-07-26 — Add the four Unicode sequence boundary vectors and pin them in the fixture guard

**Done:** Added the four multi-code-point sequence vectors (composition-block, Hangul jamo,
decomposition-leak, `Final_Sigma`) to `unicode_boundary.json` exactly as tabulated in the issues.md
umbrella entry, pinned them in the test source via a `SEQUENCE_VECTORS` const with a new ungated
guard that asserts each expected output AND its inequality with the delete-filter output, and
documented both vector families in `docs/unicode.md`. No source file changed.

**Files changed:**

- `crates/iscc-lib/tests/unicode_boundary.json`: 3 sequence cases added to `text_clean`
    (`test_0004`–`test_0006`), 1 to `text_collapse` (`test_0004`); `_metadata.description` widened
    to cover sequences. File stays pure ASCII (`raw.isascii()` verified) with the existing
    `json.dumps(indent=2, ensure_ascii=True)` shape — untouched content round-tripped
    byte-identically.
- `crates/iscc-lib/tests/test_unicode_boundary.rs`: `SEQUENCE_VECTORS` const
    `[(section, input, expected, delete_filter_output); 4]`; new ungated
    `test_boundary_fixture_sequence_vectors` (exactly-one-match, expected-equality,
    delete-filter-inequality per row); `test_boundary_fixture_metadata` now derives per-section
    counts as `4 + rows` and the exact non-ASCII code-point set as `BOUNDARY_CODE_POINTS` + that
    section's sequence-input chars (still an exact set equality, not a subset check); both gated
    vector tests derive `executed` expectations the same way (7 / 5). No new free helper functions
    (CRAP trap avoided).
- `docs/unicode.md`: second table in "Boundary behaviour" with one row per sequence vector (input,
    function, expected, delete-filter output) in `U+XXXX` notation; closing sentence now covers both
    vector families. No new page — nav stays at 23.

**Verification:**

- Fixture check from next.md's Verification section (plus a stricter variant asserting the raw
    `\uXXXX` escape strings appear literally in the ASCII bytes and the decoded code points match
    numerically via `ord()`): passes — 7 `text_clean` / 5 `text_collapse` cases, all four want-pairs
    exact.
- `cargo test -p iscc-lib`: **336 passed (281+28+22+4+1), 0 failed** (≥ 335 required).
- `cargo test -p iscc-lib --test test_unicode_boundary`: 4 passed;
    `--no-default-features --test test_unicode_boundary`: 2 passed (both ungated guards run with
    `text-processing` off).
- **Mutation check:** temporarily degrading the `Final_Sigma` fixture output to the delete-filter
    value (`U+03C3` medial sigma) makes `test_boundary_fixture_sequence_vectors` FAIL under
    `--no-default-features`; fixture restored, tests green again (2 passed).
- `cargo clippy -p iscc-lib --all-targets -- -D warnings` exits 0 (the `proc-macro-error2`
    future-incompat note is a pre-existing cargo report, not a clippy warning).
- `grep -c 'u{03C2}'` on the test file = 1 (≥ 1); `grep -c 'u{A7F1}'` = 2 (≥ 2).
- `git diff --stat crates/iscc-lib/src/` empty — no source changed.
- CRAP gate, CI-exact: `cargo llvm-cov -p iscc-lib --lcov` then
    `cargo crap --lcov lcov.info --baseline .crap-baseline.json --fail-regression --fail-above`
    exits **0** — 0 regressed / 0 new / 2 moved / 98 unchanged, so `.crap-baseline.json` is
    **untouched** per next.md's baseline-handling rule.
- `grep -c 'U+0378' docs/unicode.md` = 3 (≥ 2); `grep -c 'U+03C2'` = 1 (≥ 1).
- `uv run zensical build` exit 0, "No issues found"; `uv run scripts/check_docs_nav.py` exit 0 (23
    pages).
- `mise run check` exit 0, all hooks passed; working tree clean afterwards except the CID runner's
    own `iterations.jsonl` (not staged).

**Next:** Propagate the fixture to the 11 bindings' conformance tests and the 4 sibling `data.json`
copies (`packages/go/testdata/`, `packages/dotnet/Iscc.Lib.Tests/testdata/`,
`packages/swift/Tests/IsccLibTests/`, `packages/kotlin/src/test/resources/`). Go skips the
table-dependent vectors per the 2026-07-26 ruling but must take the `Final_Sigma` sequence vector
(its blocker was fixed in iter 147). Alternatively, wire the criterion-4 differential sweep as a
runnable check — but propagation is the natural next step since this fixture is its source.

**Notes:**

- Tool gotcha worth knowing for the propagation step: writing `\uXXXX` escape text via the Edit tool
    decodes it into literal UTF-8 characters — my first fixture edit landed non-ASCII bytes. I
    repaired it by re-dumping through Python `json.dumps(..., ensure_ascii=True)` (untouched content
    round-tripped byte-identically, confirmed by the diff) and wrote the `text_collapse` case the
    same way. The final file's escapes were verified against numeric code points with `ord()`,
    independent of any glyph rendering. Recorded in agent memory.
- The expected outputs were NOT re-derived — they are copied from the issues.md table / six
    `utils.rs` tests, and the raw-escape check confirms the file records exactly those code points.
- mdformat rewrapped the new docs paragraph during `mise run format`; content unchanged.
- Nothing in `## Not In Scope` was touched: no binding, no sibling `data.json`, no source file, no
    sweep gate, no extra vectors (no mirrored jamo-under-`text_collapse` case, no literal-`U+FFFF`
    case), no `.iai-baseline.json` refresh.
