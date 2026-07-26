# Handoff

## 2026-07-26 — Review of: Convert the Unicode freeze rule from a pre-filter to a `U+FFFF` sentinel map

**Verdict:** PASS

**Summary:** The advance agent implemented exactly the ruled design — a named `UNASSIGNED_SENTINEL`
const mapped in at both call sites inside the same fused iterator, with the category filters and the
vendored table untouched — plus six regression tests, corrected docstrings in all six locations, a
refreshed CRAP baseline and both `docs/unicode.md` rewrites, all in one commit. Review verified the
conformance claim empirically rather than by inspection: a 1,270-case sequence differential against
`iscc-core`'s algorithm running on real Unicode 16.0.0 tables found **0 mismatches** for the
sentinel design and **504** for the deleted pre-filter. All 11 verification criteria pass.

**Verification:**

- [x] `grep 'filter(|&c| !is_unassigned_in_unicode16' utils.rs` finds no match (exit 1);
    `grep -c UNASSIGNED_SENTINEL` = 4 (≥ 3 required)
- [x] `git diff --stat crates/iscc-lib/src/utils/unicode16.rs` empty — vendored table byte-identical
- [x] `cargo test -p iscc-lib`: **335 passed, 0 failed** (≥ 332 required), all conformance vectors
    green. Also green across the feature matrix: `--no-default-features` 259,
    `--features text-processing` 305, `--all-features` 335 — the `#[cfg]` gating on the new const
    and tests is correct
- [x] `cargo test -p iscc-lib --lib utils::`: 31 passed, all six named new tests present and green
- [x] `cargo clippy -p iscc-lib --all-targets -- -D warnings` clean
- [x] CRAP gate: coverage regenerated from scratch this review, then the **exact CI invocation**
    (`--fail-regression --fail-above`) exits 0 — 0 regressed / 0 new / 0 moved / 100 unchanged
- [x] `mise run bench:iai:check` exits 0, 16/16 within 10% (`bench_text_code.chars_1000` −3.89%,
    `bench_meta_code.name_desc` −1.63% — both *improvements*); `.iai-baseline.json` untouched
- [x] `grep -c 'U+FFFF' docs/unicode.md` = 4 (≥ 2); all three banned phrases absent (exit 1); "How
    much does this matter?" at line 27
- [x] `uv run zensical build` exit 0 "No issues found"; `uv run scripts/check_docs_nav.py` exit 0
    (23 pages)
- [x] `mise run check` — every hook Passed, zero reformats, working tree clean afterwards
- [x] Scope: 2 non-test/non-doc source files (`utils.rs`, generator docstring) + 1 generated
    baseline — inside the 3-file budget. Nothing in `## Not In Scope` was touched: category filters
    unchanged, `unicode_boundary.json` unchanged, no binding crate, no `packages/go`, no spec
    checkboxes, no new gate

**Independent conformance probe (this review, not from the handoff):** 127 Unicode-16.0-unassigned
code points × 10 contexts = 1,270 cases, run through the built core and compared against a
`unicodedata2==16.0.0` reimplementation of the reference `text_clean` / `text_collapse`:

| design                        | mismatches | failing contexts                                       |
| ----------------------------- | ---------- | ------------------------------------------------------ |
| sentinel map (HEAD)           | **0**/1270 | —                                                      |
| delete-filter (iteration 133) | 504/1270   | compose-block, jamo, `Final_Sigma`, diaeresis (126 ×4) |

The equivalence is analytic, not statistical: any code point unassigned in 16.0 and `U+FFFF` are
both `Cn`, `ccc = 0`, undecomposable, uncased and not `Case_Ignorable`, so they are
indistinguishable to every step of both pipelines. Rationale and the accepted scope of this evidence
→ `decisions.md` 2026-07-26, "Sentinel conformance accepted on sequence evidence".

**Issues found:**

- (none blocking) Two small inaccuracies in the advance handoff, both harmless: CI runs
    `--fail-regression --fail-above` (not "only `--fail-regression`") — I ran the exact CI
    invocation and it exits 0; and its `mise run check` hook description is fine. Its
    `--fail-above 30.0` syntax-error claim about next.md is **correct** and independently confirmed
    (`cargo crap --help`: `--fail-above` is a bare flag, threshold comes from `--threshold` /
    `.cargo-crap.toml`).

**Issues resolved (deleted from issues.md):**

- **`packages/go` `Final_Sigma`** (`critical` `[human]`) — fixed in iteration 147 but never closed
    because that iteration's review role crashed. Verified at HEAD this review: `utils.go:125` uses
    `cases.Lower(language.Und)`, 5 `Final_Sigma` regression tests present, `go test ./...` green, no
    remaining `strings.ToLower` in non-test Go source (`codec.go:398`'s `ToUpper` on a base32 ISCC
    string is ASCII-domain and fine).
- **"Convert the freeze rule from a pre-filter to a sentinel map (RULED)"** (`normal` `[human]`) —
    all six work-package items landed. Its two residuals were **not** dropped: the criterion-4 sweep
    follow-up and the sequence-vector expected outputs moved into the umbrella "Declare and gate a
    Unicode data version" issue, and the human-owned upstream-issue update moved into that issue's
    `**Upstream:**` section (the sentinel mechanism now replaces the pre-filter framing there too).

**Codex review:** No findings. It independently confirms the design ("preserves normalization and
lowercasing context while preventing newer Unicode tables from affecting unassigned code points")
and claims to have compared both functions against a Unicode 16.0 reference across all Unicode
scalar values — consistent with, but broader than, my own sequence probe. Advisory only; it did not
set the verdict.

**Next:** Add the four **sequence** vectors to `crates/iscc-lib/tests/unicode_boundary.json`
(composition-block, Hangul jamo, decomposition-leak, `Final_Sigma`). The expected outputs are
already pinned by the six `utils.rs` tests landed this iteration and are tabulated as escapes in the
issues.md umbrella entry — do not re-derive them. Extend the ungated content guard in
`tests/test_unicode_boundary.rs` so the added sequences are asserted by code-point set, not just by
shape (the existing guard would not notice a sequence vector silently degraded to ASCII). After
that, either propagate the fixture to the 11 bindings + 4 sibling `data.json` copies (Go skips the
Unicode boundary vectors per the ruling, but **must** take the `Final_Sigma` one — its blocker is
gone), or wire the criterion-4 differential sweep in as a runnable check. Both are small; the
sequence vectors are the natural next step because they are the propagation source.

**Notes:**

- The 4 existing single-code-point boundary vectors wrap each code point in ASCII, so they are
    **deletion-vs-sentinel agnostic** — they did not move this iteration and structurally cannot
    gate the distinction. Only the sequence vectors can. That is the strongest argument for doing
    them next: right now the sentinel behaviour is guarded by unit tests only, and nothing
    propagates it to the bindings.
- The criterion-4 sweep, when written, must cover **both** the 1,112,064 scalar values *and*
    sequence classes. A per-code-point sweep scores the superseded pre-filter 0 failures — that
    false assurance is the whole reason the criterion exists.
- `docs/unicode.md` now claims unqualified agreement with CPython 3.14. That is justified for the
    class that was broken (proven above) but the exhaustive scalar-value half is still owed; if the
    criterion-4 sweep ever contradicts it, the docs sentence loses, not the sweep.
- Minor docs nit, **not** actionable by CID: the page calls Todhri one of "six living scripts" added
    in Unicode 16.0, while Unicode classifies it as historical (Albanian, 18th–19th c.). The wording
    is verbatim from `specs/rust-core.md` and `decisions.md`, so changing it needs a human; the
    argument it supports (documents in that script would collapse to `""`) is unaffected either way.
- CI has **not** run on any of the six unpushed commits — iteration 147's review crashed before
    pushing, so this push carries iterations 147 and 148 together. Verified locally in this review:
    Go tests, Rust tests across the feature matrix, clippy, coverage + CRAP (CI-exact invocation),
    iai 16/16, docs build, nav parity, and all pre-commit hooks. Watch the first CI run after this
    push.
- Full-tree circumvention scan over `@{upstream}..HEAD` (all six commits, not just the advance
    diff): no lint suppressions, no skipped tests, no threshold or hook weakening; the only
    gate-adjacent file touched is `scripts/gen_unicode16_unassigned.py`, and only its docstring.
