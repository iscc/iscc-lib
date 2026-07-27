# Handoff

## 2026-07-27 — Freeze `Final_Sigma` case classification at Unicode 16.0.0

**Done:** Vendored the Unicode 16.0.0 `Cased` / `Case_Ignorable` classification (new generator
`scripts/gen_unicode16_case.py` → data module `crates/iscc-lib/src/utils/unicode16_case.rs`) and
routed `text_collapse`'s lowercasing through a new `to_lowercase_unicode16` that decides the
`Final_Sigma` condition from those tables before delegating all remaining case mappings to
`str::to_lowercase()` — so std's compiler-table (Unicode 17.0) sigma branch can never fire. The
full-code-space + sequence-class differential sweep against `iscc-core` on CPython 3.14 went from
**3 divergent comparisons (single code point U+0295) to 0** — measured before and after, not
assumed.

**Files changed:**

- `scripts/gen_unicode16_case.py` (new, counted file 1 of 2): PEP 723 generator
    (`requires-python = "==3.14.*"`, no deps; refuses to run unless
    `unicodedata.unidata_version == "16.0.0"`). Derives both properties behaviourally from the two
    probes CPython's `handle_capital_sigma` consults (`(ch+Σ).lower()` / `("A"+ch+Σ).lower()` ending
    in ς), merges into maximal inclusive ranges, fails closed on any shape mismatch (`SystemExit`
    before writing).
- `crates/iscc-lib/src/utils/unicode16_case.rs` (new, generated tool output): `CASED_RANGES`
    `[(u32,u32); 152]` (4,311 cps; = `Cased` restricted to non-`Case_Ignorable`, documented as
    unobservable) + `CASE_IGNORABLE_RANGES` `[(u32,u32); 452]` (2,749 cps). U+0295 in cased, U+FFFF
    in neither — both enforced by generator invariants and a Rust test.
- `crates/iscc-lib/src/utils.rs` (counted file 2 of 2): `mod unicode16_case`; range lookup factored
    into shared `in_ranges` (existing comparator unchanged, `is_unassigned_in_unicode16` fast path
    kept in place); `is_cased_in_unicode16` / `is_case_ignorable_in_unicode16`; `cased_lookahead`
    (single reverse pass — sigma-dense strings stay linear) + `to_lowercase_unicode16` (hot path:
    `!text.contains('Σ')` → plain `to_lowercase()`); `text_collapse` step 1 now feeds the
    sentinel-mapped, NFD-normalized string to `to_lowercase_unicode16` — nothing else in the
    pipeline moved. 7 new tests (6 sigma-context cases + table invariants).
- `.crap-baseline.json`: regenerated in this commit (tool output).
- `docs/unicode.md` (docs, excluded): scoped the "severs all dependence" overclaim to the
    *unassigned* classification; new "Case-property freeze (`Final_Sigma`)" section naming U+0295
    and the rustc-version dependence of bare `str::to_lowercase()`.
- `crates/iscc-lib/CLAUDE.md` (docs, excluded): new pitfall entry — do not replace
    `to_lowercase_unicode16` with bare `.to_lowercase()`.

**Verification:** (every criterion from next.md, all run this session)

- Generator: exit 0; second run **byte-identical** (md5 verified; the porcelain check reads `??`
    only because the file is new/untracked — determinism itself is proven). Both shape greps = 1.
- Fail-closed probe: `EXPECTED_CASED_RANGE_COUNT` 152→153 → exit **1**, output file untouched (md5),
    constant restored (verified by grep).
- `cargo test -p iscc-lib`: **0 failed**; lib unit binary **288 passed** (meets ≥288 exactly: 281
    prior + 7 new; re-run after the format pass). Integration binaries 28/22/4/1 all green.
- `cargo clippy --workspace --all-targets -- -D warnings`: clean. Feature matrix:
    `--no-default-features` and `--no-default-features --features text-processing` both compile.
- Oracle check: all 7 expected values re-confirmed against live `iscc_core.text_collapse` on CPython
    3.14.6 / unidata 16.0.0 before writing the tests — no table corrections needed.
- Sweep probe (from `/tmp`, no tree diff): **before** the fix —
    `TOTAL 17793024 comparisons, 3 divergences`, all at cp=U+0295 (3 contexts; next.md's "1
    divergence" = 1 code point — the probe counts per context row). **After** —
    `TOTAL 17793024 comparisons, 0 divergences`, exit **0**. Extension rebuilt via
    `uv run maturin develop --release` before each run.
- Mutation probe: `CASED_RANGES` truncated to 10 entries → `cargo test -p iscc-lib --lib` **4
    failed**; restored (md5-identical) → 288 passed.
- CRAP: `mise run coverage` + `mise run crap:baseline` run, baseline staged in this commit;
    `mise run crap` exit 0 — 105 functions, none above 30. New fns all 100% covered:
    `to_lowercase_unicode16` 7.0, `cased_lookahead` 3.0, `in_ranges` / `is_cased_in_unicode16` 1.0.
- `mise run bench:iai:check` exit 0 — 16/16 within 10% of baseline; `bench_text_code.chars_1000`
    **-3.83% Ir** (fast path holds; no baseline refresh needed).
- `uv run pytest -q`: **379 passed** with the rebuilt extension.
- Docs: `grep -c 'U+0295' docs/unicode.md` = 1; pinning + rustc-version claims stated;
    `check_docs_nav.py` OK, **23** pages.
- `mise run check` exit 0 (all prek hooks).
- Scope guards: porcelain empty for `scripts/unicode_sweep.py`, `unicode_boundary.json`, `.github`,
    `.claude/context/specs`; `git ls-files -- '*unicode_boundary.json'` = **2**.

**Next:** The permanent sweep gate (criterion 4) can now land green: turn
`/tmp/unicode_sweep_probe.py` into `scripts/unicode_sweep.py` + a mise task + CI placement. Scope
its blind spots per the earlier review note: fail-closed on missing table, assert
`unidata_version == "16.0.0"`, assert `total == 17_793_024` so a zero-case run cannot read as green,
and rebuild the extension before sweeping (a stale `.so` silently measures the previous commit).

**Notes:**

- Pre-fix divergence count was **3 comparison rows**, not 1 — all the same code point U+0295 across
    3 sigma contexts. next.md's "exactly 1 divergence" counts code points; the probe (its own spec)
    counts rows. The divergence *set* is `{U+0295}` exactly as stated, so I treated this as
    consistent, not an oracle conflict.
- next.md's test table has 7 rows; row 7 (sentinel keeps final sigma) was already pinned by the
    pre-existing `test_sentinel_preserves_final_sigma_context`, so I added 6 sigma tests + 1 table
    test = 288 total, meeting the ≥288 bar exactly. All expected values verified against the live
    oracle first.
- The generated table intentionally stores `Cased − Case_Ignorable` in `CASED_RANGES` (that is what
    the behavioural probes measure); both the module doc and the generator docstring say so and why
    it is unobservable. Sizes 152/4,311 and 452/2,749 matched next.md's re-derived facts on first
    run.
- `to_lowercase_unicode16` tracks preceding context on the **original** (pre-substitution) chars and
    checks casedness only of non-ignorable chars, so the restricted `CASED_RANGES` is always
    consulted safely; Σ itself is cased-and-not-ignorable, making consecutive sigmas (`ΑΣΣ` → `ασς`)
    match CPython.
- Nothing out of scope touched: no Go changes, no fixture changes, no sweep-gate files, no spec
    edits. issues.md untouched (review owns issue resolution).
