# Handoff

## 2026-07-26 — Convert the Unicode freeze rule from a pre-filter to a `U+FFFF` sentinel map

**Done:** Replaced the iteration-133 delete-filter with a sentinel map in `text_clean` /
`text_collapse`: code points unassigned in Unicode 16.0.0 are now mapped to a named
`UNASSIGNED_SENTINEL` const (`'\u{FFFF}'`, permanent noncharacter) inside the same fused iterator
chain, and the unchanged category-`C` filters remove the sentinel exactly where the reference
removes unassigned code points. Corrected all docstrings/comments that gave pre-normalization
*removal* as the invariance mechanism (utils.rs ×3, generator docstring, boundary test file ×2,
crate CLAUDE.md bullet), added the six spec-mandated regression tests, refreshed
`.crap-baseline.json`, and landed both `docs/unicode.md` rewrites (sentinel mechanism + accepted
divergence class (b) + "How much does this matter?" section) in the same commit.

**Files changed:**

- `crates/iscc-lib/src/utils.rs`: `UNASSIGNED_SENTINEL` const with noncharacter rationale; both call
    sites `filter(...)` → `map(...)`; three doc comments + two step-1 inline comments rewritten
    (replace-then-reference-filter-removes, not pre-removal); two stale test comments fixed; six new
    feature-gated tests (`test_sentinel_blocks_canonical_composition`,
    `_blocks_hangul_jamo_composition`, `_preserves_final_sigma_context`,
    `_prevents_decomposition_leak`, `test_normalizer_passes_sentinel_through`,
    `test_literal_sentinel_input_removed`)
- `scripts/gen_unicode16_unassigned.py`: docstring wording only ("maps … to the noncharacter
    sentinel `U+FFFF`"); generated output untouched
    (`git diff --stat   crates/iscc-lib/src/utils/unicode16.rs` is empty)
- `crates/iscc-lib/tests/test_unicode_boundary.rs`: module doc + `BOUNDARY_CODE_POINTS` doc reworded
    to sentinel mechanism; no assertion changes
- `docs/unicode.md`: freeze-rule section rewritten around the sentinel map and its two load-bearing
    properties; CPython-3.14 sentence widened to unqualified "agrees with iscc-lib"; divergence
    class (b) + 15.1.0-rejection rationale added to the warning admonition; new "How much does this
    matter?" section near the top; final practical-guidance bullet updated
- `crates/iscc-lib/CLAUDE.md`: "Text normalization order matters" bullet rewritten (map-to-sentinel
    \+ unchanged filter, not strip-first)
- `.crap-baseline.json`: regenerated via `mise run crap:baseline` (line-number moves only —
    `text_clean` cyclomatic/crap stayed 9.0, `text_collapse` 1.0, both 100% covered; the sentinel
    branch lives in a closure so cyclomatic did not rise)

**Verification:** (all from this session's tool output)

- `grep 'filter(|&c| !is_unassigned_in_unicode16'` on utils.rs: no match (exit 1);
    `grep -c UNASSIGNED_SENTINEL` = 4 (const + doc ref + two call sites)
- `cargo test -p iscc-lib`: **335 passed, 0 failed** (≥ 332 required); all conformance vectors
    green. `cargo test -p iscc-lib --lib utils::`: 31 passed incl. all six new sentinel tests
- `cargo clippy -p iscc-lib --all-targets -- -D warnings`: clean
- CRAP gate: `cargo crap --lcov lcov.info --baseline .crap-baseline.json --fail-regression` exits 0
    (0 regressed / 0 new / 0 moved). Note: next.md's literal `--fail-above 30.0` is invalid
    cargo-crap syntax (`--fail-above` is a bare flag paired with `--threshold`); the semantic
    equivalent `--fail-regression --fail-above --threshold 30.0` also exits 0. CI uses only
    `--fail-regression`.
- iai: pre-edit `mise run bench:iai:check` exit 0 (16/16 within 10%); post-edit exit 0.
    `bench_text_code.chars_1000` Ir 1,130,335 → 1,123,152 (−0.6% vs pre-edit run — flat as
    predicted); `bench_meta_code.name_desc` 183,257 → 182,930. `.iai-baseline.json` untouched.
- docs: `grep -c 'U+FFFF' docs/unicode.md` = 4; the three banned phrases find no match (exit 1);
    "How much does this matter" at line 27; `uv run zensical build` exit 0 "No issues found";
    `uv run scripts/check_docs_nav.py` exit 0 (23 pages)
- `mise run check`: all hooks Passed (first run had one mdformat auto-reflow of docs/unicode.md;
    clean on re-run)

**Next:** Add the sequence vectors (composition-block, jamo, `Final_Sigma`, decomposition-leak) to
`crates/iscc-lib/tests/unicode_boundary.json` and propagate the boundary fixture to the bindings —
next.md explicitly deferred both to the following step. After that, the criterion-4 differential
sweep (1,112,064 scalar values + sequence classes).

**Notes:**

- Scope: 2 non-test/non-doc source files (utils.rs, generator docstring) + regenerated baseline —
    within budget. Vendored table byte-identical; superseded designs (category override, 15.1.0
    downgrade) not implemented; `packages/go` and binding crates untouched.
- `crates/iscc-lib/tests/unicode_boundary.json` itself is unchanged (its 4 single-code-point vectors
    are deletion/sentinel-agnostic by construction — ASCII context only), matching next.md.
- The CRAP prediction in next.md (cyclomatic +1 per function) did not materialize: rustc attributes
    the `if` branch to the closure, so `text_clean`/`text_collapse` scores are unchanged. The
    baseline still needed refreshing because every function below the edit point moved lines.
- `mise run crap:baseline` printed a warning that `tests/test_unicode_boundary.rs` had no matching
    LCOV entry — pre-existing behaviour (integration-test files are not instrumented targets), not
    introduced by this step.
- `.claude/context/iterations.jsonl` shows modified in git status (runner-owned); left unstaged per
    protocol.
