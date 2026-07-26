# Next Work Package

## Step: Convert the Unicode freeze rule from a pre-filter to a `U+FFFF` sentinel map

## Goal

Make `text_clean` / `text_collapse` conformant with the reference on sequences containing
Unicode-16.0.0-unassigned code points by replacing such code points with the noncharacter sentinel
`U+FFFF` instead of deleting them (issues.md: "Convert the freeze rule from a pre-filter to a
sentinel map (RULED)", `specs/rust-core.md` requirement 1). This closes the largest open correctness
item — the iteration-133 pre-filter fails 42 of 140 sequence cases — and lands the two
`docs/unicode.md` rewrites in the same commit so the public page never documents a rejected
mechanism.

## Scope

- **Modify**:
    - `crates/iscc-lib/src/utils.rs` — sentinel `const`, both call sites (lines ~103 and ~181), three
        doc comments (`is_unassigned_in_unicode16` ~line 26, `text_clean` ~92, `text_collapse` ~171),
        the two inline step-1 comments, plus the new regression tests in the existing `mod tests`
    - `scripts/gen_unicode16_unassigned.py` — docstring wording only (lines ~6–8 say the core "strips"
        the code points "before any normalization"); **the generated output must not change**
    - `crates/iscc-lib/tests/test_unicode_boundary.rs` — stale "stripped before normalization" wording
        in the module/const docs (test file; no assertion changes needed)
    - `docs/unicode.md` — work-package items 5 and 6 (see Implementation Notes)
    - `crates/iscc-lib/CLAUDE.md` — the "Text normalization order matters" bullet (~lines 158–163)
    - `.crap-baseline.json` — regenerate with `mise run crap:baseline` (generated artifact)
- **Reference**: `.claude/context/specs/rust-core.md` (requirement 1 + the **Verified when** list,
    lines ~55–195 — the authority); `.claude/context/issues.md` ("Convert the freeze rule …" work
    package, 6 points); `.claude/context/decisions.md` 2026-07-26 entries;
    `crates/iscc-lib/src/utils/unicode16.rs` (data only — do not edit);
    `crates/iscc-lib/tests/unicode_boundary.json`

**File budget:** 2 non-test, non-doc source files (`utils.rs`, the generator docstring) plus one
regenerated baseline artifact — inside the 3-file limit.

## Not In Scope

- **The superseded designs.** Do not implement the category override (`U+A7F1` decomposes to `S`
    under Unicode 17 and injects a spurious letter — 1 single-code-point and 10 sequence failures)
    and do not lower the declared version to 15.1.0 (rejected: collapses documents in the 6 living
    scripts added in 16.0 to `""`).
- **No change to the category filters or the vendored table.** `is_c_category` / `is_cmp_category`
    stay exactly as the reference defines them (`U+FFFF` is `Cn`, so they already remove it), and
    `crates/iscc-lib/src/utils/unicode16.rs` plus the generator's *output* stay byte-identical.
- **No sequence vectors in `crates/iscc-lib/tests/unicode_boundary.json`** and no propagation of the
    boundary fixture to the 11 bindings or the 4 sibling `data.json` copies — that is the next step.
- **The criterion-4 differential sweep** (1,112,064 scalar values + sequence classes as a runnable
    check) is a separate step. A throwaway probe to satisfy yourself is fine; do not wire a new
    gate.
- **Do not touch `packages/go`** (the `Final_Sigma` fix landed in iteration 147; the per-call
    `cases.Caser` must not be hoisted) and do not touch any binding crate — they inherit the core.
- **Do not refresh `.iai-baseline.json`.** Ir is expected flat; a real move means the implementation
    is not the specified one.
- Do not tick checkboxes in `.claude/context/specs/`.

## Implementation Notes

**The code change (issues.md item 1).** Name the sentinel once, e.g.

```rust
/// Replacement for code points unassigned in Unicode 16.0.0.
///
/// `U+FFFF` is a *noncharacter*: under Unicode's Noncharacter stability policy it is
/// permanently category `Cn`, `ccc = 0` and has no decomposition, so it can never gain
/// an assignment or a decomposition in a future table version.
const UNASSIGNED_SENTINEL: char = '\u{FFFF}';
```

then, at both call sites, inside the same fused iterator chain:

```text
// from:
.filter(|&c| !is_unassigned_in_unicode16(c))
// to:
.map(|c| if is_unassigned_in_unicode16(c) { UNASSIGNED_SENTINEL } else { c })
```

**Docstrings (item 2).** All three current doc comments give *pre-normalization removal* as the
reason for invariance — wrong on both counts. New wording: unassigned code points are **replaced
by** the noncharacter `U+FFFF` before normalization; the function's own category-`C` filter
(unchanged from the reference) then removes the sentinel, so *removal happens exactly where the
reference does it* — which is what preserves composition-blocking and `Final_Sigma` context — while
*mapping into a permanent noncharacter* is what severs dependence on the tables a dependency ships.
Apply the same correction to `crates/iscc-lib/CLAUDE.md` and the two test-file doc lines.

**Verified facts — do not re-derive.** Expected values come from `specs/rust-core.md` **Verified
when** and are mechanically consistent with the implementation: `U+FFFF` has `ccc = 0`, so it blocks
canonical composition and Hangul jamo composition of its neighbours; it is neither `Cased` nor
`Case_Ignorable`, so a preceding `Σ` still lowercases to final `ς`. The existing 25 `utils.rs` tests
and the 4 single-code-point vectors in `tests/unicode_boundary.json` are **unaffected** (they wrap
the code point in plain ASCII `a`/`b`, where deletion and sentinel-then-strip agree).

**Tests (item 3)** — add to `utils.rs`'s `mod tests`, feature-gated like the neighbouring
freeze-rule tests:

- `text_clean("e\u{0378}\u{0301}") == "e\u{0301}"` (no `U+00E9`)
- `text_clean("\u{1100}\u{0378}\u{1161}") == "\u{1100}\u{1161}"` (no `U+AC00`)
- `text_collapse("\u{0391}\u{03A3}\u{0378}\u{0392}") == "\u{03B1}\u{03C2}\u{03B2}"` (final sigma)
- `text_clean("e\u{A7F1}\u{0301}") == "e\u{0301}"` (**not** `"e\u{015A}"` — the case the superseded
    category override gets wrong)
- normalizer pass-through: `"\u{FFFF}".nfkc().collect::<String>() == "\u{FFFF}"` and the same for
    `.nfd()` — the one assumption the design rests on
- no-ambiguity: a literal `U+FFFF` in the input is itself unassigned in 16.0, so
    `text_clean("a\u{FFFF}b") == "ab"` and `text_collapse("a\u{FFFF}b") == "ab"`

**Gates (item 4).** The `map` adds a branch to two covered functions, so the **CI-only** CRAP
`--fail-regression` check reds unless `.crap-baseline.json` is refreshed in this same commit
(baseline today: `text_clean` cyclomatic/crap 9.0, `text_collapse` 1.0, both 100% covered — expect
each to rise by ~1 with coverage still 100%). Refresh with `mise run crap:baseline` (depends on
`mise run coverage`, several minutes). For perf, run `mise run bench:iai:check` **before** editing
as a baseline sanity probe (valgrind and `iai-callgrind-runner` are both installed in this
container), then again after; Ir must be flat.

**Docs (items 5 and 6) — `docs/unicode.md`:**

1. Rewrite the "Declared version and freeze rule" section (lines ~29–40): the mechanism is a
    *sentinel map*, not pre-normalization removal. Keep the 731-range / 819,533-code-point table
    and generator sentence. Replace "Because removal happens first…" with the two load-bearing
    properties: removal stays where the reference performs it (conformance, including sequences),
    and mapping into a permanent noncharacter makes output invariant under table upgrades.
2. Widen the CPython-3.14 sentence (line ~74) back to unqualified agreement — drop "on the
    single-code-point behaviour described on this page".
3. State accepted divergence class (b): characters assigned between 15.1 and 16.0 hash differently
    than `iscc-core` on CPython ≤ 3.13 produced historically — accepted by decision — and why
    lowering to 15.1.0 was rejected (the 6 living scripts added in 16.0 would collapse to `""`).
4. Add a short **"How much does this matter?"** section near the top: Data-Code and Instance-Code
    unaffected (raw bytes); Meta-Code and Text-Code are similarity-preserving so affected codes
    stay Hamming-close and similarity matching keeps working; newly assigned code points are rare
    in real text; residual exposure is exact-match lookups on short inputs and the exact `name` /
    `description` fields.

Do **not** add a new docs page (the 23-page nav/`ORDERED_PAGES`/`llms.txt` triple stays as is) and
do not restate the parked-vs-settled history — the page describes current behaviour only.

## Verification

- `grep -n 'filter(|&c| !is_unassigned_in_unicode16' crates/iscc-lib/src/utils.rs` finds **no
    match** (exit 1), and `grep -c 'UNASSIGNED_SENTINEL' crates/iscc-lib/src/utils.rs` is ≥ 3 (const
    \+ two call sites)
- `git diff --stat crates/iscc-lib/src/utils/unicode16.rs` is empty (vendored table untouched)
- `cargo test -p iscc-lib` passes — ≥ 332 tests (328 today + the new ones), zero failures, all
    `gen_*_v0` conformance vectors green
- `cargo test -p iscc-lib --lib utils::` passes and includes the six new assertions listed above
- `cargo clippy -p iscc-lib --all-targets -- -D warnings` is clean
- `mise run crap:baseline` is run and then
    `cargo crap --lcov lcov.info --baseline .crap-baseline.json --fail-regression --fail-above 30.0`
    exits 0 on the working tree
- `mise run bench:iai:check` exits 0 and `git diff --stat .iai-baseline.json` is empty. (If the
    check already fails on the unedited tree for a local-vs-CI toolchain offset, the handoff must
    instead record the `text_clean` / `text_collapse` Ir from the pre-edit and post-edit runs
    showing a change under 10%, with the baseline still untouched.)
- `grep -c 'U+FFFF' docs/unicode.md` ≥ 2, while
    `grep -n 'Because removal happens first\|removed from the input\|single-code-point behaviour described on this page' docs/unicode.md`
    finds **no match**, and `grep -n 'How much does this matter' docs/unicode.md` matches
- `uv run zensical build` exits 0 and reports "No issues found"; `uv run scripts/check_docs_nav.py`
    exits 0 (still 23 pages)
- `mise run check` — all hooks Passed

## Done When

The sentinel map, the corrected docstrings, the six regression assertions, the refreshed CRAP
baseline and both `docs/unicode.md` rewrites are in one commit and every verification check above
passes.
