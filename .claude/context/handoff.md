# Handoff

## 2026-07-27 — Review of: Freeze `Final_Sigma` case classification at Unicode 16.0.0

**Verdict:** PASS

**Summary:** The advance agent vendored the Unicode 16.0.0 `Cased` / `Case_Ignorable` classification
and routed `text_collapse`'s lowercasing through `to_lowercase_unicode16`, which pre-substitutes
every `Σ` from those tables so `str::to_lowercase()`'s compiler-table (17.0) `Final_Sigma` branch
can never fire. Every claim in the handoff was independently re-verified at review, including a
fresh 17,793,024-comparison differential sweep (0 divergences), three semantic mutation probes and a
from-scratch audit of both generated tables against CPython 3.14 category data. The work is
correctly scoped, well documented and the tests are load-bearing; review fixed two prose
imprecisions in `docs/unicode.md`.

**Verification:** (every criterion from next.md, all re-run this session)

- [x] Generator exits 0 and is byte-stable — re-ran `uv run --script scripts/gen_unicode16_case.py`
    against the **committed** file: md5 unchanged (`75e855cd…`) and
    `git status --porcelain crates/iscc-lib/src/utils/unicode16_case.rs` empty. (Stronger than the
    advance agent's check, which could only observe the untracked-file state.)
- [x] `grep -c 'CASED_RANGES: \[(u32, u32); 152\]'` = 1 and
    `grep -c 'CASE_IGNORABLE_RANGES: \[(u32, u32); 452\]'` = 1
- [x] Generator fails closed — probed **three** ways, not one: range count 152→153 → exit 1; code
    point count 2_749→2_748 → exit 1; `unidata_version` guard → exit 1. Output md5 unchanged in all
    three; script restored, porcelain clean.
- [x] `cargo test -p iscc-lib`: 0 failed; lib unit binary **288 passed** (≥288 met), integration
    binaries 28/22/4/1 green. Feature matrix also re-run: `--no-default-features` 220 passed,
    `+text-processing` 259 passed, `--all-features` 288 passed — the `#[cfg]` gating is correct.
- [x] `cargo clippy --workspace --all-targets -- -D warnings` clean (only the known dev-only
    `proc-macro-error2` future-incompat note)
- [x] `maturin develop --release` then the sweep: **`TOTAL 17793024 comparisons, 0 divergences`,
    exit 0** — re-run by review with a freshly rebuilt extension (the committed `.so` was older than
    `src/utils.rs`, so the rebuild was not optional)
- [x] Mutation probe — ran next.md's truncation (4 tests red) **plus two semantic mutations that
    keep the range count constant**, so the *behavioural* tests are proven load-bearing rather than
    just the shape test: dropping U+0295 from `CASED_RANGES` reds
    `test_final_sigma_{following,preceding}_context_cased_in_16`; dropping U+0027 from
    `CASE_IGNORABLE_RANGES` reds `test_final_sigma_apostrophe_is_case_ignorable`. File restored
    md5-identical.
- [x] CRAP — `mise run coverage` then the **CI-exact**
    `cargo crap --lcov lcov.info --baseline .crap-baseline.json --fail-regression --fail-above`
    exits 0 (0 regressed, 0 new, 105 unchanged, nothing above 30). `mise run crap:baseline`
    reproduces the committed baseline byte-for-byte.
- [x] `mise run bench:iai:check` exit 0 — 16/16 within 10%; `bench_text_code.chars_1000` **−3.83%
    Ir** (an improvement; no baseline refresh warranted)
- [x] `uv run pytest -q`: **379 passed** with the rebuilt extension. Pre-push Python gates also
    green: `ty check` (the new PEP 723 script is stdlib-only and needed **no** `[tool.ty.src]`
    exclusion), ruff `--select S`, ruff `--select C901`.
- [x] Docs — `grep -c 'U+0295' docs/unicode.md` = 1; the rustc-version and vendored-table claims are
    stated; `uv run scripts/check_docs_nav.py` OK, **23** pages; `uv run zensical build` "No issues
    found"
- [x] `mise run check` exit 0 (all 17 prek hooks) and `git status --porcelain` clean afterwards
- [x] Scope guards — porcelain empty for `scripts/unicode_sweep.py`,
    `crates/iscc-lib/tests/unicode_boundary.json`, `.github`, `.claude/context/specs`;
    `git ls-files -- '*unicode_boundary.json'` = **2**. No gate-circumvention pattern anywhere in
    `git diff @{upstream}..HEAD` (no `allow(`, `noqa`, `#[ignore]`, threshold or hook edits).

**Independent checks beyond next.md** (the generator derives its tables *behaviourally*, so it
cannot serve as its own oracle):

- **Both tables audited from a different source.** Against CPython 3.14 category data the vendored
    tables are definitionally exact: `Lu∪Ll∪Lt ⊆ CASED` (0 missing),
    `Mn∪Me∪Cf∪Lm∪Sk ⊆ CASE_IGNORABLE` (0 missing), the two tables are disjoint, `CASED ∖ (Lu∪Ll∪Lt)`
    is exactly 130 `So` + 32 `Nl` + 2 `Lo` (= `Other_Uppercase` ∪ `Other_Lowercase` minus the
    case-ignorable ones), and `CASE_IGNORABLE ∖ (Mn∪Me∪Cf∪Lm∪Sk)` is exactly the 17 UAX #29
    MidLetter/MidNumLet/Single_Quote code points. U+0345 (both properties) is in the ignorable table
    only; U+0295 is cased; U+FFFF is in neither.
- **Pre-fix defect corroborated without a revert.** A throwaway crate under the current toolchain
    shows bare std giving `"\u{0295}\u{03A3}".to_lowercase()` → `"ʕσ"` and `"ΑΣʕΒ"` → `"αςʕβ"`, both
    the opposite of the oracle — so the 3 divergent rows at U+0295 the handoff reports are real, and
    the fix is load-bearing rather than defensive.
- **All 7 expected values re-confirmed three-way** (live `iscc_core` 1.3.0 vs `iscc_lib` vs the
    literal in the test source) — no test merely mirrors the implementation.
- **Algorithmic equivalence with CPython's `handle_capital_sigma`** read line by line: the port's
    `last_non_ignorable` / `cased_lookahead[i+1]` pair matches the reference scan, and the
    restricted `CASED_RANGES` (`Cased ∖ Case_Ignorable`) is genuinely unobservable because casedness
    is only ever tested on non-ignorable characters. `String::with_capacity(text.len())` is exact
    (Σ, σ and ς are all 2 bytes). Placement matches the reference's `NFD → .lower()` order.

**Issues found:**

- (fixed by review, prose only) `docs/unicode.md` stated the `Final_Sigma` condition as "no `Cased`
    character follows". That is wrong: `ΑΣ,Β` → `αςβ` (a comma is not `Case_Ignorable`, so the scan
    stops there) while `ΑΣ.Β` → `ασβ` (a period is `MidNumLet`, hence ignorable). Reworded to "the
    nearest *following* non-`Case_Ignorable` character is not `Cased` (or there is none)", verified
    against the oracle both ways.
- (fixed by review, prose only) The docs quoted "152 `Cased` ranges (4,311 code points)" without
    saying the table is `Cased` **minus** `Case_Ignorable` — a reader recomputing the UCD `Cased`
    set would get a different count. Added one sentence naming U+0345 as the illustrative case.
- (noted, not a defect) Only the *conditional* `Final_Sigma` mapping is frozen; every unconditional
    lowercase mapping is still supplied by rustc's tables. The sweep measures that residual at
    exactly zero today across all 1,112,064 scalars, but nothing *guards* it until criterion 4
    lands. Recorded in `decisions.md` 2026-07-27 and in the `issues.md` entry so the sweep gate is
    scoped to cover it.
- (scope note, accepted) Literally counted, the commit touches 4 non-test non-doc files. Two of them
    — `crates/iscc-lib/src/utils/unicode16_case.rs` and `.crap-baseline.json` — are tool output
    regenerated in-commit and were explicitly excluded by next.md's budget accounting, matching the
    iteration-133/148 precedent. Hand-written surface is 2 files, as specified.

**Codex review:** Available and clean — "The Unicode 16.0 Final_Sigma implementation, generated
property tables, feature gating, and tests are consistent and pass on both current Rust and the
declared MSRV." No actionable findings; the docs-prose imprecision above was found by review, not by
Codex.

**Next:** Land criterion 4 — turn the throwaway probe into a permanent, runnable gate
(`scripts/unicode_sweep.py` + a mise task + a CI job). It can now land **green**: the divergence set
is empty, re-measured this review. Scope its blind spots explicitly, because a sweep gate is exactly
the kind of check that can pass vacuously:

- fail closed on a missing/unimportable table or extension, and assert
    `unicodedata.unidata_version == "16.0.0"` (a 3.13 runner would silently measure the wrong
    oracle)
- assert `total == 17_793_024` so a zero-case run cannot read as green
- rebuild the Python extension **before** sweeping — a stale gitignored `.so` measures the previous
    commit (it was stale at HEAD this iteration)
- decide where it runs: ~4 min wall clock is too slow for pre-push, so CI-only (its own job) with a
    `mise run sweep` escape hatch is the likely shape — say so in next.md rather than leaving it to
    advance
- the gate's real job is the *unguarded* residual named above (unconditional case mappings,
    normalization tables), not the sigma condition it will re-prove

After that, the remaining Unicode work is propagation slice 5 (C FFI / C++ / Swift — all three
blocked on missing toolchains in this container, so they may need a human or a different runner) and
the four sibling `data.json` copies.

**Notes:**

- The pre-fix count discrepancy the advance handoff flags (3 comparison rows vs next.md's "1
    divergence") is a units mismatch, not an oracle conflict — 1 code point (U+0295) × 3 sigma
    contexts. Independently reproduced.
- `bench_text_code.chars_1000` got **faster** (−3.83% Ir) despite the added work. Plausibly code
    layout; worth remembering that the iai baseline for that bench is now conservative by ~4%, so a
    future genuine regression of up to that size would hide inside the band. Not worth a refresh on
    its own — fold it into the next deliberate baseline update.
- `crates/iscc-lib/src/codec.rs` still uses bare `to_uppercase()` / `to_lowercase()` on ISCC code
    strings. Reviewed and deliberately left alone: it mirrors `iscc-core`'s `code.upper()`, only
    ever feeds a base32 decoder, and cannot affect hash output. Not an issue.
- The two vendored Unicode tables are now independent artifacts with independent generators
    (`gen_unicode16_unassigned.py`, `gen_unicode16_case.py`). If a third ever appears, consider a
    single `mise run unicode:regen` task plus a pytest anchor asserting all generators are
    byte-stable — cheaper than remembering which script owns which table.
