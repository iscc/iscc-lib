# Handoff

## 2026-07-25 — Review of: Land the Unicode 16.0 boundary vectors as a Rust-core fixture

**Verdict:** PASS_WITH_NOTES

**Summary:** The advance agent landed exactly what next.md asked for: an ASCII-escaped,
`data.json`-shaped fixture `crates/iscc-lib/tests/unicode_boundary.json` (4 single code points ×
`text_clean`/`text_collapse`), a loader `tests/test_unicode_boundary.rs` with the ungated
anti-silent-skip metadata test plus two `text-processing`-gated vector tests, and one bullet in
`crates/iscc-lib/CLAUDE.md`. Nothing under `crates/iscc-lib/src/`, no binding, no baseline, no
`data.json` touched — non-test/non-doc file budget used: 0. Every next.md criterion reproduced
independently this session, including the `unicodedata2==16.0.0` derivation. Review added one
hardening assertion (see Issues found) and mutation-probed it.

**Verification:**

- [x] `cargo test -p iscc-lib --test test_unicode_boundary` → `3 passed; 0 failed`
- [x] `cargo test -p iscc-lib` → exit 0, 5 targets, zero `FAILED`, no `N failed` line
- [x] `cargo test -p iscc-lib --no-default-features` → exit 0; boundary target runs exactly the
    ungated `test_boundary_fixture_metadata` (`1 passed`, plus a 0-test binary)
- [x] `cargo test -p iscc-lib --no-default-features --features text-processing` → exit 0, boundary
    target `3 passed`
- [x] `cargo clippy --workspace --exclude iscc-rb --all-targets -- -D warnings` → exit 0
- [x] `cargo clippy -p iscc-lib --no-default-features -- -D warnings` → exit 0
- [x] `cargo fmt --all --check` → exit 0
- [x] Fixture-structure heredoc → `fixture OK`, exit 0 (4 cases/section, exactly the four code
    points)
- [x] Independent derivation heredoc (`unicodedata2==16.0.0`) → `derivation OK`, exit 0 — expected
    values reproduced from Unicode 16.0 data without reading the Rust implementation
- [x] `uv run prek run check-json --files crates/iscc-lib/tests/unicode_boundary.json` → `Passed`
- [x] Core untouched:
    `git status --porcelain crates/iscc-lib/src/ tests/data.json .crap-baseline.json   .iai-baseline.json`
    → empty; `grep -c '!is_unassigned_in_unicode16(c)' src/utils.rs` → `2`
- [x] `grep -c 'unicode_boundary.json' crates/iscc-lib/CLAUDE.md` → `1`
- [x] `mise run check` → 15/15 hooks Passed, exit 0 (re-run green after the review edit)
- [x] Extra: fixture is byte-verified ASCII (max byte 125), LF-only, trailing newline
- [x] Extra: `cargo test -p iscc-lib --all-features --test test_unicode_boundary` → `3 passed`
    (covers the third CI feature-matrix leg)
- [x] Gate integrity: `git diff @{upstream}..HEAD` contains no suppression, skip, threshold or hook
    weakening; the only non-context files are 2 new tests + 1 doc line

**Issues found:**

- **The fixture was self-referential and its content unguarded** (fixed in review, additive
    test-only change). The two vector tests compare the implementation against the fixture, and the
    metadata test only asserted *4 cases per section* — so replacing all four cases with ASCII
    no-ops (or dropping the U+A7F1 regression guard) would have left the whole suite green while
    silently deleting the coverage. next.md's heredoc caught this but is not committed anywhere. The
    ungated `test_boundary_fixture_metadata` now also asserts the exact non-ASCII code-point set per
    section against a `BOUNDARY_CODE_POINTS` constant. Mutation-probed: swapping one case for `"ab"`
    fails with `left: ['⃁', '\u{113c5}', '🫩'] right: ['⃁', '꟱', ...]`; restored after. This matters
    because the file is about to be copied into 11 bindings.
- Verified the fixture has real bite rather than being hypothetical: `unicode-normalization` 0.1.25
    ships **Unicode 17.0** tables, where U+A7F1 is `Lm` with `<super> 0053` (NFKC → `S`) and U+20C1
    is `Sc`. Both "unassigned in 16.0" vectors therefore fail immediately if the freeze filter ever
    stops running first — they are live regression guards on the shipped dependency set.
- Pre-existing, not a regression, not a gate:
    `cargo clippy -p iscc-lib --no-default-features   --all-targets` fails with E0432 because
    `benches/` import `gen_meta_code_v0`/`gen_text_code_v0` unconditionally. next.md's criterion (no
    `--all-targets`) is the correct one; recorded in learnings so a future agent does not mistake it
    for fresh breakage.
- No new issue filed. `issues.md` was updated: the Unicode issue's implementation-order item (b) now
    records the Rust half as done with the remaining binding/`data.json` propagation spelled out.

**Codex review:** Clean — "The fixture accurately represents the documented Unicode 16.0 boundary
cases, and the loader exercises it correctly with appropriate feature gating. Targeted tests and
clippy checks pass across the relevant feature configurations." No actionable findings; note that it
did not flag the fixture-content gap above, consistent with its pattern of not probing what a test
*cannot* catch.

**Next:** Take the `normal` `[review]` issue **"Land the `release.yml` static checks as an
executable gate"**. It is the only remaining fully-unblocked CID-doable item, it is already scoped
in issues.md (one `scripts/check_release_workflow.py`, a prek hook restricted to
`^\.github/workflows/release\.yml$` for checks 1–2, a CI step for check 3 which needs network), and
it converts invariants that have now been hand-retyped into heredocs in three consecutive iterations
(139, 140, and again here for the guard shape) into a re-running gate. Suggested slicing if it looks
large: land checks 1–2 (pure-local, no network) as the local gate first and leave check 3 for a
follow-up CI-only step.

**Notes:**

- **Do not start Unicode step (b)** (propagating this fixture into 11 bindings + 4 sibling
    `data.json` copies). It is double-blocked: the parked HUMAN REVIEW ruling on freeze-rule
    sequence ordering, and the Go decision (Go is on Unicode 15.0 tables until go1.27, ~Aug 2026 —
    it needs either a vendored 15.0→16.0 delta or a documented skip). Wiring it now means deriving
    expected values twice.
- `specs/rust-core.md` criterion 3 correctly stays **unmet** — it requires the vectors in every
    binding's conformance test, not just the Rust suite. Criterion 4 (differential sweep) is still
    blocked on the same ruling.
- Rationale for the fixture's shape — second file instead of merging into `data.json`, keeping the
    duplicated inline `utils.rs` assertions, single code points only, and not extending
    `conformance_selftest()` — is now recorded in `decisions.md` (2026-07-25). A future reader
    finding the same four code points in two places should not "clean up" either copy.
- `learnings.md` is 206 lines against a 200-line budget. Two CI/CD entries (action-major static
    verification, `release.yml` guard shape) were archived this round; further cuts would remove
    live guidance, so the overshoot is deliberate rather than unpruned.
- Two `normal` issues remain human-gated and cannot be picked up by define-next: the
    `rubygems/configure-rubygems-credentials` tag-vs-SHA question and npm OIDC trusted publishing.
