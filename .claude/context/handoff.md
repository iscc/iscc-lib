# Handoff

## 2026-07-26 — Review of: Add the four Unicode sequence boundary vectors and pin them in the fixture guard

**Verdict:** PASS_WITH_NOTES

**Summary:** The four sequence vectors landed exactly as scoped — three `text_clean` cases and one
`text_collapse` case in `unicode_boundary.json`, pinned in the test source by a `SEQUENCE_VECTORS`
const and a second **ungated** guard, with per-section counts derived rather than hard-coded and the
exact-code-point-set assertion kept exact. Zero source files touched (budget was zero), all 11
verification criteria pass, and review mutation-probed four independent degradation modes — every
one reds the suite with `text-processing` off. One defect: the `delete_filter_output` oracle for the
`U+A7F1` row named the wrong superseded design, and the wrong value reached published docs; review
corrected it in this commit.

**Verification:**

- [x] Fixture shape and content (next.md's Python check) — `raw.isascii()` true, 7 `text_clean` / 5
    `text_collapse` cases, all four `(input → expected)` pairs exact; dumped every case as numeric
    code points to confirm the escapes decode as intended
- [x] `cargo test -p iscc-lib` — **336 passed, 0 failed** (≥ 335 required), re-run green after the
    review fix
- [x] `cargo test -p iscc-lib --test test_unicode_boundary` — 4 passed
- [x] `cargo test -p iscc-lib --no-default-features --test test_unicode_boundary` — 2 passed; also
    checked `--features text-processing` (4) and `--all-features` (4), so the `#[cfg]` gating is
    right across the whole matrix
- [x] `cargo clippy -p iscc-lib --all-targets -- -D warnings` — exit 0 (workspace clippy also exit
    0; the `proc-macro-error2` future-incompat note is the known dev-only cargo report)
- [x] `grep -c 'u{03C2}'` = 1 (≥ 1) and `grep -c 'u{A7F1}'` = 2 (≥ 2) in the test source — both
    still hold after the review fix
- [x] `git diff HEAD~1..HEAD --stat crates/iscc-lib/src/` empty — no source file changed
- [x] CRAP gate, CI-exact (`cargo llvm-cov -p iscc-lib --lcov` →
    `cargo crap --lcov lcov.info   --baseline .crap-baseline.json --fail-regression --fail-above`) —
    exit **0**, 0 regressed / 0 new / 2 moved / 98 unchanged; re-run after the review fix, still 0.
    `.crap-baseline.json` correctly left untouched (`git status --porcelain` on it is empty)
- [x] `grep -c 'U+0378' docs/unicode.md` = 3 (≥ 2), `grep -c 'U+03C2'` = 1 (≥ 1)
- [x] `uv run zensical build` exit 0 "No issues found"; `uv run scripts/check_docs_nav.py` exit 0 —
    23 pages, no new page added
- [x] `mise run check` — every hook Passed, no reformats, working tree clean apart from the runner's
    own `iterations.jsonl`
- [x] Scope: **0** non-test, non-doc source files (budget: zero). Nothing from `## Not In Scope` was
    touched — no binding, no sibling `data.json`, no source file, no sweep gate, no extra vectors,
    no `.iai-baseline.json` refresh
- [x] Gate-integrity scan over all unpushed commits (`@{upstream}..HEAD`, 3 commits): no lint
    suppressions, no skipped tests, no threshold or hook weakening, no ignore-list additions

**Independent mutation probe (this review, not from the handoff):** four separate degradations of
`unicode_boundary.json`, each run under `--no-default-features` so only the ungated guards fire:

| mutation                                                           | caught by                             |
| ------------------------------------------------------------------ | ------------------------------------- |
| row 3's output degraded to the real delete-filter value (`U+00E9`) | sequence guard (`assert_eq`)          |
| the Hangul-jamo case deleted                                       | both guards (count + exactly-one)     |
| `U+0378` swapped for `U+0379` in the sigma input                   | both guards (code-point set + lookup) |
| a sequence case duplicated under a new name                        | both guards (count + exactly-one)     |

The count assertion and the code-point-set assertion are both derived from the const table, not from
the fixture, so neither is self-referential; the set-equality check is non-vacuous (both sides
non-empty, 8 code points per section).

**Issues found:**

- **Wrong superseded-design attribution, fixed in this commit.** The `SEQUENCE_VECTORS` row for
    `e U+A7F1 U+0301` carried `e\u{015A}` as its `delete_filter_output`, and `docs/unicode.md`
    published that value under a **"Delete filter would produce"** column heading. A delete filter
    removes the code point *before* normalization, so that input collapses to `e` + `U+0301` and
    NFKC-composes to **`U+00E9`** — the same value as row 1. `e U+015A` is what the superseded
    *category-override* design yields, which `crates/iscc-lib/src/utils.rs:385` states correctly.
    Verified three ways, not by inspection: `unicodedata2==17.0.0` reports `U+A7F1` as MODIFIER
    LETTER CAPITAL S with `<super> 0053`, `NFKC("e"+U+A7F1+U+0301)` = `e U+015A` while
    `NFKC("e"+U+0301)` = `U+00E9`; and `git show 068d77a:…/utils.rs` confirms the iteration-133
    design really was a pre-normalization `.filter(|&c| !is_unassigned_in_unicode16(c))`. **Origin:
    next.md**, whose Implementation Notes tabulated the value under that label and explicitly
    forbade re-deriving it — advance followed the instruction as written and is not at fault. Review
    fixed the docs cell, the const value, and added a comment/sentence in both places recording that
    the `U+A7F1` row additionally guards the category-override hazard. All tests, clippy, CRAP, docs
    build and hooks re-run green after the fix.
- (non-blocking) `issues.md`'s source table headed that column **"must NOT be"**, which is agnostic
    and was not itself wrong. Updated anyway to say "delete filter gives", with row 3 corrected and
    an explicit "do not relabel this back" note, so the propagation step cannot reintroduce the
    mislabel into 11 bindings.

**Issues resolved (updated, not deleted):** the umbrella *Declare and gate a Unicode data version*
issue stays open — its remainder **(b)** is now half done. The sequence-vector half is marked ✅ with
the guard and mutation evidence; the propagation half is what remains, alongside **(a2)**, the
criterion-4 sweep.

**Codex review:** One P2 finding, and it is **the same defect I derived independently** — "when this
tuple models the superseded pre-normalization delete filter, deleting `U+A7F1` leaves `e U+0301`,
whose NFKC output is `U+00E9`, not `e U+015A`… correct this value and the matching docs row, or
rename the oracle to describe that different design." I took the first branch (correct the value)
and kept the category-override fact as prose, so both hazards stay documented. No other findings.

**Next:** Propagate the fixture to the 11 bindings' conformance tests and the four sibling
`data.json` copies (`packages/go/testdata/`, `packages/dotnet/Iscc.Lib.Tests/testdata/`,
`packages/swift/Tests/IsccLibTests/`, `packages/kotlin/src/test/resources/`). Go skips the
table-dependent vectors per the 2026-07-26 ruling but **must** take the `Final_Sigma` sequence
vector — its blocker was fixed in iteration 147. This is the natural next step because the Rust
fixture is now complete and is the propagation source. The alternative — wiring the criterion-4
differential sweep (1,112,064 scalar values **plus** sequence classes) as a runnable check — is
still owed and is the last Unicode item after propagation.

**Notes:**

- **Propagation will be a wide diff by nature** (11 bindings × conformance loaders + 4 `data.json`
    copies). It is test-only, so the 3-file budget does not bind, but define-next should still slice
    it — e.g. the four `data.json` copies first, then bindings in language groups — rather than one
    step touching everything. Each binding's loader has to be checked individually: the fixture is a
    *second* vector file, so a binding that hard-codes `data.json` needs new plumbing, not a copy.
- **Escape-decoding tool gotcha, confirmed twice.** Writing `\uXXXX` text through the Edit tool
    decodes it into literal UTF-8 characters — it bit advance on the fixture and bit me on
    `issues.md` in this review. For any ASCII-escaped file, edit through Python
    (`json.dumps(..., ensure_ascii=True)` or explicit `"\\u"` in a non-raw string) and verify with
    `raw.isascii()` plus numeric `ord()` checks, never by looking at glyphs. Recorded in agent
    memory; it will matter again during propagation, since every sibling `data.json` is
    ASCII-escaped.
- **The lesson worth carrying past this iteration** is not the one wrong code point: a "must NOT be"
    oracle is only meaningful if the design it came from is named. next.md relabelled an agnostic
    column as a specific design and instructed advance not to re-derive; that combination is how a
    wrong value reaches published docs with every gate green. Green gates say nothing about truth.
- CI is green on the pushed head (`c597496`), so this push carries only iterations 149's three
    commits plus this review — no backlog.
- `learnings.md` was at 205 lines; two Python-binding pins (PyO3 0.29, `_lowlevel.pyi` typing) moved
    to `learnings-archive.md` and two docs bullets were merged, holding it at 200.
