# Handoff

## 2026-07-26 — Review of: Document the declared Unicode 16.0.0 data version for library consumers

**Verdict:** PASS_WITH_NOTES

**Summary:** `docs/unicode.md` is a genuinely good Explanation page — the freeze rule, the vendored
table figures (731 ranges / 819,533 code points, both re-derived from
`crates/iscc-lib/src/utils/unicode16.rs` in this review) and all four boundary rows match
`unicode_boundary.json` and real Unicode data exactly, and the wiring (nav, `ORDERED_PAGES`,
`llms.txt`, go.md note) is correct and complete. **But the page shipped two false consumer-facing
claims** — an unqualified "CPython 3.14 agrees with iscc-lib" and "ASCII and Latin text is never
affected" (repeated in `docs/howto/go.md`) — both prescribed verbatim by next.md's Implementation
Notes and both flagged independently by the Codex review. I corrected them in place with measured
figures rather than holding the batch; every next.md criterion passes.

**Verification:**

- [x] `uv run zensical build` → exit 0, "No issues found" (re-run after my fixup)
- [x] `uv run python scripts/gen_llms_full.py` → exit 0, 23 pages, no `Auto-discovered`, no
    `Warning:` line
- [x] `grep -q "16.0.0" site/llms-full.txt` → 8 matches after the run
- [x] `grep -E -c 'U\+(1FAE9|113C5|20C1|A7F1)' docs/unicode.md` → 8 (≥ 4 required)
- [x] `grep -q "iscc-core/issues/137" docs/unicode.md` → OK
- [x] `grep -q "unicode.md" docs/llms.txt` and `grep -q "Unicode 15.0" docs/howto/go.md` → both OK
- [x] Boundary table vs `unicode_boundary.json` — all 4 rows agree; additionally re-verified against
    real Unicode data (`unicodedata2==16.0.0`: U+1FAE9 `So` FACE WITH BAGS UNDER EYES, U+113C5 `Mc`
    TULU-TIGALARI VOWEL SIGN AI, both `Cn`; `==17.0.0`: U+20C1 `Sc` SAUDI RIYAL SIGN, U+A7F1 `Lm`
    MODIFIER LETTER CAPITAL S — so "assigned in Unicode 17" is exact)
- [x] `git status --porcelain crates/ packages/go/*.go` → empty (no source or fixture change)
- [x] `uv run prek run --files` over the touched files → all applicable hooks `Passed`
- [x] `uv run ruff check` → "All checks passed!"
- [x] `mise run check` → all 16 hooks `Passed`; only runner-owned `iterations.jsonl` dirty
- [x] Scope: 2 non-test/non-doc files (`zensical.toml`, `scripts/gen_llms_full.py`) — within the
    3-file budget; nothing under `crates/`, no spec checkbox ticked, no `issues.md` edit by advance
- [x] Gate integrity: `git diff @{upstream}..HEAD` over `.pre-commit-config.yaml`, `.github/`,
    `pyproject.toml`, `Cargo.toml`, `deny.toml`, `mise.toml` → no changes; zero suppression / skip /
    threshold patterns added
- [x] Constraint compliance: no equivalence claim to uniform 16.0.0 tables, no sequence-behaviour
    assertion, Go divergence stated as a tracked limitation
- [x] Go claims re-verified independently: `unicode.Version == "15.0.0"` on go1.26.1,
    `packages/go/utils.go` filters on `unicode.C`

**Issues found:**

- **(fixed in review) "ASCII and Latin text is never affected" was false — and dangerously so.**
    Unicode 16.0 assigned **5,185** code points (measured by diffing `unicodedata2` 15.1.0 vs 16.0.0
    assigned-set dumps): 3,995 Egyptian Hieroglyph additions, seven new scripts, the 7 new emoji and
    **32 LATIN-named** additions — including `U+A7CB` LATIN CAPITAL LETTER RAMS HORN, which is the
    exact character in this project's own recorded Go-vs-Rust divergence reproduction in issues.md.
    The page and the `docs/howto/go.md` note therefore told Go users their Latin corpus was safe
    against the one interop failure the project has actually demonstrated. Rewrote both to quantify
    the affected set. Also corrected "realistically the seven emoji" (understates by ~740×).
- **(fixed in review) "CPython 3.14 ships Unicode 16.0.0 tables and agrees with iscc-lib" was
    unqualified**, contradicting the open `HUMAN REVIEW REQUESTED` issue "Freeze-rule ordering
    diverges from iscc-core on sequences" (divergence persists on identical 16.0 data). Scoped to
    "agrees … on the single-code-point behaviour described on this page" — removing the false claim
    without asserting sequence behaviour, which next.md excluded. Recorded as a placeholder in
    `issues.md` (that issue, new point 4) so the ordering ruling revisits it.
- **Root cause is next.md, not carelessness alone:** both false claims were dictated in the
    Implementation Notes under "Other settled facts worth stating on the page". advance treated
    prose facts as settled and did not sanity-check them against `issues.md`, which was in its own
    context and contains the counter-example. Learnings updated for both roles.

**Codex review:** Available and accurate — it found **both** defects independently and at the right
severity (P2/P2), including the U+A7CB counter-example. No other findings; it raised nothing about
the wiring, nav, or generator change. This is the third consecutive iteration where Codex caught a
"documented-rule vs reality" gap (142: matcher vs its own docstring; 143: doc claims vs Unicode
data) — its value on prose-vs-fact checking is now well established.

**Next:** Two reachable options, in preference order:

1. **`--check-action-inputs` for `scripts/check_release_workflow.py`** (issues.md `[review]`, still
    open and unblocked): opt-in, network-fetching, offline-skipping mode + a `ci.yml` step, kept
    out of the network-free pytest suite. This is the last hand-performed release-workflow check.
2. If workflow tooling feels over-weighted again, the docs surface has an adjacent gap worth one
    step: nothing gates `zensical.toml` nav ↔ `scripts/gen_llms_full.py` `ORDERED_PAGES` ↔
    `docs/llms.txt` parity — a page can land in one and be silently missing from the others (this
    iteration wired all three by hand). Inventing that gate needs human sign-off per the prior
    step's constraint, so define-next should either scope it explicitly or file it.

Everything Unicode-related beyond documentation stays parked on Titusz: the freeze-rule ordering
ruling and the Go 15.0-tables decision both block binding propagation.

**Notes:**

- The published page now carries a *deliberate placeholder*. When the ordering ruling lands, the
    CPython-3.14 sentence in `docs/unicode.md` must move in the same step — tracked as point 4 of
    the freeze-rule issue and in `decisions.md` (two entries dated 2026-07-26).
- `learnings.md` was at 199 lines; two entries (git file-mode, `cargo tree --target all`) moved to
    `learnings-archive.md` to make room. It sits at 201 after mdformat reflow — the next reviewer
    should archive one more entry rather than let it drift.
- Useful measurement recipe now in learnings: dump the assigned code-point set from two
    `uvx --with unicodedata2==<ver> python` runs and diff — ~40s, and it settles any "which
    characters changed in Unicode X" question empirically instead of by recollection.
- Docs-only iteration: CRAP and iai baselines untouched by design; `site/` is gitignored.
