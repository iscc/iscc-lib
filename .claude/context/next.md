# Next Work Package

## Step: Add the four Unicode sequence boundary vectors and pin them in the fixture guard

## Goal

Add the four multi-code-point **sequence** vectors to `crates/iscc-lib/tests/unicode_boundary.json`
and extend the ungated guard so their exact inputs and outputs are pinned in the test source
(issues.md: "Declare and gate a Unicode data version (DECIDED)", remainder **(b)**;
`specs/rust-core.md` requirement 3). This matters because the four existing vectors wrap a single
code point in plain ASCII and are therefore **deletion-vs-sentinel agnostic** — the fixture that is
the propagation source for all 11 bindings currently cannot gate the distinction iteration 148 just
fixed, so the sentinel behaviour is guarded by unit tests in one crate only.

## Scope

- **Modify**:
    - `crates/iscc-lib/tests/unicode_boundary.json` — add 3 cases to `text_clean`, 1 to
        `text_collapse`, and widen `_metadata.description` (test fixture)
    - `crates/iscc-lib/tests/test_unicode_boundary.rs` — a `SEQUENCE_VECTORS` const, an added ungated
        content guard, updated per-section case counts (test file)
    - `docs/unicode.md` — extend the "Boundary behaviour" section with the sequence vectors (doc)
    - `.crap-baseline.json` — **only if** the CI-exact CRAP gate fails on the edited tree (generated
        artifact; see Implementation Notes)
- **Reference**:
    - `.claude/context/issues.md` — the umbrella Unicode issue's sequence-vector table (~line 316)
        holds the authoritative escapes
    - `.claude/context/specs/rust-core.md` — requirement 3 and the freeze-rule **Verified when** list
        (~lines 96–195)
    - `crates/iscc-lib/src/utils.rs` lines ~350–407 — the six sentinel regression tests that pin the
        expected outputs (read only; do not edit)
    - `.claude/context/decisions.md` 2026-07-26 entries (sentinel design; Go skips the table-dependent
        vectors)

**File budget:** **zero** non-test, non-doc source files. Two test files, one doc page, and at most
one regenerated baseline artifact.

## Not In Scope

- **Propagation.** Do not copy the fixture into any of the 11 bindings, and do not touch the 4
    sibling `data.json` copies (`packages/go/testdata/`, `packages/dotnet/Iscc.Lib.Tests/testdata/`,
    `packages/swift/Tests/IsccLibTests/`, `packages/kotlin/src/test/resources/`). That is the next
    step, and it carries the Go skip ruling with it.
- **The criterion-4 differential sweep** (1,112,064 scalar values + sequence classes as a runnable
    check) — a separate step. Do not wire a new gate or script.
- **No source changes.** `crates/iscc-lib/src/utils.rs`, `src/utils/unicode16.rs` and every binding
    crate stay byte-identical; this step only observes behaviour that already exists and is already
    green.
- **Do not invent extra vectors.** Exactly the four tabulated sequences. In particular do not add
    mirrored cross-section cases (the jamo sequence under `text_collapse` legitimately recomposes to
    `U+AC00` and would read as a contradiction), and do not add a literal-`U+FFFF` case.
- **Do not re-derive the expected outputs** with a Python/`unicodedata2` probe — they are pinned by
    the six `utils.rs` tests that passed CI at HEAD and are reproduced verbatim below.
- Do not tick checkboxes in `.claude/context/specs/`, and do not edit `issues.md` (review closes
    items).
- **Do not refresh `.iai-baseline.json`** — no hot-path code changes, so Ir cannot move.

## Implementation Notes

**1. Fixture (`tests/unicode_boundary.json`).** The file is pure ASCII by design (composed and
decomposed forms render identically, and `U+0378` renders as tofu), so every string stays
`\uXXXX`-escaped. Add these four cases verbatim — the escapes were generated with
`json.dumps(..., ensure_ascii=True)` and match the issues.md table exactly:

```text
"text_clean": {
  "test_0004_seq_u0378_blocks_canonical_composition": {
    "inputs": ["e\u0378\u0301"],
    "outputs": { "result": "e\u0301" }
  },
  "test_0005_seq_u0378_blocks_hangul_composition": {
    "inputs": ["\u1100\u0378\u1161"],
    "outputs": { "result": "\u1100\u1161" }
  },
  "test_0006_seq_ua7f1_no_decomposition_leak": {
    "inputs": ["e\ua7f1\u0301"],
    "outputs": { "result": "e\u0301" }
  }
},
"text_collapse": {
  "test_0004_seq_u0378_preserves_final_sigma": {
    "inputs": ["\u0391\u03a3\u0378\u0392"],
    "outputs": { "result": "\u03b1\u03c2\u03b2" }
  }
}
```

Keep the existing `test_0000`–`test_0003` cases untouched and keep the existing nesting shape
(`inputs` array of one string, `outputs.result`). The outputs the superseded delete-filter design
would wrongly produce are `\u00e9`, `\uac00`, `e\u015a` and `\u03b1\u03c3\u03b2` respectively; they
belong in the Rust guard (below), not in the JSON. Widen `_metadata.description` so it no longer
describes the fixture as single code points only.

**2. Guard (`tests/test_unicode_boundary.rs`).** Add a const table so the sequence expectations live
in the test source, not only in the fixture:

```rust
/// Sequence boundary vectors: `(section, input, expected, delete_filter_output)`.
/// Unlike the single-code-point vectors these distinguish the `U+FFFF` sentinel map
/// from the superseded delete-filter design, so their exact strings are pinned
/// here - a fixture case silently degraded to ASCII must fail this guard.
const SEQUENCE_VECTORS: [(&str, &str, &str, &str); 4] = [ /* ... */ ];
```

Then, still **ungated** (no `#[cfg(feature = "text-processing")]`, so a feature-off build still
guards the fixture):

- For every row: find the case in that section whose `inputs[0]` equals `input`, assert exactly one
    match, assert its `outputs.result` equals `expected`, and assert it is **not** equal to
    `delete_filter_output`.
- Per-section case count: derive it as `4 + rows for that section` (7 for `text_clean`, 5 for
    `text_collapse`) rather than hard-coding two magic numbers in two places.
- Keep the existing exact non-ASCII code-point-set assertion working by deriving the expected set as
    `BOUNDARY_CODE_POINTS` plus the non-ASCII chars of that section's sequence inputs — do not
    weaken it to a subset check.
- Update the two `#[cfg(feature = "text-processing")]` vector tests' `executed` assertions the same
    way (7 and 5), so a silently dropped case still fails.

**CRAP trap — keep new logic inside `#[test]` functions.** `cargo crap` scores only non-`#[test]`
functions in integration-test files (`.crap-baseline.json` contains exactly two entries for this
file: `boundary_data` cyclomatic 1 / crap 2, `run_boundary_section` cyclomatic 2 / crap 6), and
coverage for those is `null` → `missing = "pessimistic"` → `crap = c² + c`. A new free helper with
cyclomatic ≥ 6 therefore scores 42 and trips `--fail-above` (threshold 30) even though it is test
code. Put the new assertions in a second `#[test] fn` (e.g.
`test_boundary_fixture_sequence_vectors`) instead of a new helper, and do not add branches to
`boundary_data` / `run_boundary_section`.

**Baseline handling.** Adding lines shifts those two functions' line numbers, which `cargo crap`
tracks as *moved*, not *regressed*. Run the CI-exact gate on the edited tree:

```bash
cargo llvm-cov -p iscc-lib --lcov --output-path lcov.info
cargo crap --lcov lcov.info --baseline .crap-baseline.json --fail-regression --fail-above
```

If it exits 0, **leave `.crap-baseline.json` untouched** (baselines are refreshed only when a gate
demands it). If it exits non-zero, refresh with `mise run crap:baseline` in this same commit and say
so in the handoff. Note `--fail-above` is a bare flag — the 30.0 threshold comes from
`.cargo-crap.toml`.

**3. Docs (`docs/unicode.md`).** The "Boundary behaviour" section (~lines 64–83) documents only the
four single-code-point vectors. Add a short second table for the sequence vectors immediately after
it, and adjust the closing sentence so it covers both families. Write the code points as `U+XXXX`
notation rather than literal glyphs (`U+0378` is unassigned and renders as tofu; `e` + `U+0301` and
`U+00E9` are visually identical). One row per vector: input sequence, function, expected output, and
the output a delete filter would wrongly produce. Two sentences of context are enough: the sentinel
keeps its neighbours apart during normalization and lowercasing, which is why these sequences — and
not the ASCII-wrapped single code points — are what pins the design. Do **not** add a new page (the
nav / `ORDERED_PAGES` / `llms.txt` triple stays at 23 pages).

**Verified facts — do not re-derive.** `crates/iscc-lib/src/utils.rs` asserts, at HEAD, in tests
that passed CI: `text_clean("e\u{0378}\u{0301}") == "e\u{0301}"`,
`text_clean("\u{1100}\u{0378}\u{1161}") == "\u{1100}\u{1161}"`,
`text_clean("e\u{A7F1}\u{0301}") == "e\u{0301}"`, and
`text_collapse("\u{0391}\u{03A3}\u{0378}\u{0392}") == "\u{03B1}\u{03C2}\u{03B2}"`. The fixture must
record exactly these.

## Verification

- Fixture shape and content, mechanically:

```bash
python3 - <<'PY'
import json, pathlib
p = pathlib.Path("crates/iscc-lib/tests/unicode_boundary.json")
raw = p.read_bytes()
assert raw.isascii(), "fixture must stay ASCII-escaped"
d = json.loads(raw)
assert len(d["text_clean"]) == 7 and len(d["text_collapse"]) == 5
want = {
    ("text_clean", "e\u0378\u0301"): "e\u0301",
    ("text_clean", "\u1100\u0378\u1161"): "\u1100\u1161",
    ("text_clean", "e\ua7f1\u0301"): "e\u0301",
    ("text_collapse", "\u0391\u03a3\u0378\u0392"): "\u03b1\u03c2\u03b2",
}
got = {(s, c["inputs"][0]): c["outputs"]["result"]
       for s in ("text_clean", "text_collapse") for c in d[s].values()}
for k, v in want.items():
    assert got.get(k) == v, (k, got.get(k), v)
print("fixture OK")
PY
```

- `cargo test -p iscc-lib` passes with **0 failures** and at least 335 tests
- `cargo test -p iscc-lib --test test_unicode_boundary` passes (guards + both vector tests)
- `cargo test -p iscc-lib --no-default-features --test test_unicode_boundary` passes — the fixture
    guards run with `text-processing` off
- `cargo clippy -p iscc-lib --all-targets -- -D warnings` is clean
- `grep -c 'u{03C2}' crates/iscc-lib/tests/test_unicode_boundary.rs` is ≥ 1 and
    `grep -c 'u{A7F1}' crates/iscc-lib/tests/test_unicode_boundary.rs` is ≥ 2 — the sequence
    expectations are pinned in the test source, not only in the fixture
- `git diff --stat crates/iscc-lib/src/` is **empty** — no source file changed
- `cargo llvm-cov -p iscc-lib --lcov --output-path lcov.info` then
    `cargo crap --lcov lcov.info --baseline .crap-baseline.json --fail-regression --fail-above`
    exits 0 on the working tree
- `grep -c 'U+0378' docs/unicode.md` is ≥ 2 and `grep -c 'U+03C2' docs/unicode.md` is ≥ 1
- `uv run zensical build` exits 0 and reports "No issues found"; `uv run scripts/check_docs_nav.py`
    exits 0 (still 23 pages)
- `mise run check` — every hook Passed and the working tree is clean afterwards

## Done When

The four sequence vectors are in the fixture, pinned by an ungated guard that fails on a degraded
case, documented in `docs/unicode.md`, and every verification check above passes.
