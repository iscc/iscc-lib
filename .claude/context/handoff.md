# Handoff

## 2026-07-26 — Review of: Propagate the Unicode boundary fixture to the WASM and Ruby test suites (propagation slice 2)

**Verdict:** PASS

**Summary:** Two new test files gate the Unicode 16.0.0 sentinel freeze rule on the WASM and Ruby
binding surfaces, both reading the canonical fixture in place (no new vendored copy), with zero
skips and zero non-test/non-doc source changes. Every next.md criterion passes as specified. I
mutation-probed both suites independently: a delete-filter-shaped expected value reds the named
vector on each surface, and a deleted fixture case reds each metadata guard — so these are real
gates, not green-by-construction loops.

**Verification:**

- [x] `wasm-pack test --node crates/iscc-wasm --features conformance` — exit 0; per target: lib 0,
    conformance 9, **unicode_boundary 3 passed / 0 failed**, unit 78 (baseline held), doctests 0
- [x] `unicode_boundary` WASM target shows `ok` with ≥ 3 passed, 0 failed — exactly 3
- [x] `cargo clippy --workspace --exclude iscc-rb --all-targets -- -D warnings` — exit 0;
    `cargo clippy -p iscc-rb -- -D warnings` — exit 0
- [x] `cargo fmt --all --check` — exit 0
- [x] `bundle exec rake compile` — exit 0; `bundle exec rake test` — **124 runs, 314 assertions, 0
    failures, 0 errors, 0 skips** (111 baseline + 12 vectors + 1 guard; next.md's floor was 123)
- [x] `bundle exec standardrb` — exit 0, no output
- [x] No skip construct in either new file — both greps exit 1
- [x] `find . -name unicode_boundary.json …` — exactly 2 paths; this slice vendors no new copy
- [x] `git status --porcelain crates/iscc-lib/ packages/go/ .crap-baseline.json .iai-baseline.json`
    — empty
- [x] `cargo test -p iscc-lib` — 281+28+22+4+1 = **336 passed, 0 failed**, unchanged
- [x] `mise run check` — exit 0, no reformats (`git status` clean apart from the runner's
    `iterations.jsonl`)
- [x] `uv run zensical build` — "No issues found"; `uv run scripts/check_docs_nav.py` — OK, 23 pages
- [x] `grep -F -c 'unicode_boundary.rs' crates/iscc-wasm/CLAUDE.md` = 1;
    `grep -F -c 'test_unicode_boundary.rb' crates/iscc-rb/CLAUDE.md` = 1; `docs/unicode.md` names
    both suites
- [x] **Extra (review-added) — mutation probes.** (1) Canonical fixture's
    `text_clean/test_0003_ua7f1…` expected value set to the pre-freeze `aSb`: WASM reds
    `test_text_clean_boundary` naming the case, Ruby reds
    `test_text_clean_test_0003_ua7f1_unassigned_16_stripped`. (2) One `text_clean` case deleted:
    both metadata guards red with `expected 7 … Actual: 6`. Fixture restored and `cmp`-verified
    identical.
- [x] **Extra — gate reality.** `ci.yml` runs `wasm-pack test --node … --features conformance` (job
    `WASM`) and, in the `ruby` job, `bundle exec rake compile` **before** `bundle exec rake test`,
    so the stale-`.so` hazard cannot reach CI.
- [x] **Extra — mechanism check.** Both surfaces compile the same Rust core (`UNASSIGNED_SENTINEL`
    in `crates/iscc-lib/src/utils.rs`), and both run all four sequence vectors, so neither can pass
    "for the right output by the wrong mechanism" the way `packages/go` does.
- [x] **Extra — gate integrity.** Full unpushed range (`@{upstream}..HEAD`, 6 commits incl. the
    audit metrics commit) contains no lint suppression, no skip/ignore, no threshold or hook change;
    no file under `.pre-commit-config.yaml`, `.github/`, or any lint config is touched.

**Issues found:**

- (none blocking) Minor fix applied by review: the `tests/` tree in `crates/iscc-wasm/CLAUDE.md` had
    misaligned comment columns after the new entry, and its `conformance.rs` line described the
    fixture as "vendored" — `conformance.rs` actually `include_str!`s the canonical
    `crates/iscc-lib/tests/data.json`. Realigned and corrected to "canonical".
- Bookkeeping note (not a defect): the "N of 11 surfaces" tally counts the 11 native bindings named
    in `docs/unicode.md`; `packages/go` is the separate pure-Go port and UniFFI is the mechanism
    behind Kotlin/Swift, not an independent surface. That reconciles state.md's "9 surfaces" with
    its 10-name list. Recorded in issues.md so update-state can carry it forward: the count is now
    **4 of 11 gated, 7 left**.

**Codex review:** No findings — "The new WASM and Ruby tests correctly consume the canonical
fixture, exercise all boundary vectors, and are integrated into existing test runners. No functional
regressions or actionable defects were found." Consistent with my own probes; taken as corroboration
only, since a one-paragraph clean verdict is not evidence on its own.

**Next:** Land the `[review]` **vendored-copy byte-identity drift gate** ("Gate byte-identity of the
vendored test-vector copies", `normal`). It is deliberately sequenced *before* the
`packages/{dotnet,kotlin,swift}` slice, which adds three more `data.json` copies. Shape it as a
pytest anchor test over an explicit `(canonical, copy)` table (a `files:`-scoped prek hook never
sees deletions, so a hook alone is not sufficient); give the table a count floor so the assertion
cannot pass vacuously on an empty list. Green at HEAD today — all six copies match. After that,
resume propagation: **C FFI + JNI/Java** is the next cheapest in-container pair; napi stays the most
expensive slice because its checked-in `iscc-lib.linux-x64-gnu.node` is stale and must be rebuilt
first.

**Notes:**

- **`cargo test` does not run the WASM tests.** `cargo test -p iscc-wasm` compiles all 5 targets and
    reports `0 passed` for each — `#[wasm_bindgen_test]` only registers under
    `wasm-pack test   --node`. So `cargo test --workspace` / `clippy --all-targets` prove the new
    file *compiles*; coverage comes solely from the dedicated CI `WASM` job. Do not "fix" the 0-test
    host result.
- **Read wasm-pack output from a file, not `| tail`.** The per-target `test result:` lines are
    mid-log; a tail shows only `unit.rs` and the doctests, which is how a 0-test target could slip
    past unnoticed.
- The Ruby `.so` is gitignored, so `rake compile` leaves no tree diff but is mandatory locally — a
    pre-sentinel build returns `aSb` for `text_clean("a" + U+A7F1 + "b")`. Only the U+A7F1 rows
    discriminate staleness (U+0378 is `Cn` in every Unicode version).
- The advance handoff reported a one-off `OSError: [Errno 22]` from mdformat during
    `mise run format`. I did not reproduce it across `mise run check` plus two `zensical`/docs runs;
    treating it as a bind-mount FS blip. Worth re-flagging only if it recurs.
- No `decisions.md` entry this iteration — the step made no design trade-off a future reader could
    not reconstruct from the code.
