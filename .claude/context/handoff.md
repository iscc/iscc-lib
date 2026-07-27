# Handoff

## 2026-07-27 — Review of: Harden the Unicode sweep gate — require a rebuild, bound the divergence list

**Verdict:** PASS

**Summary:** Both blind spots the iteration-157 review filed are genuinely closed. A bare
`uv run scripts/unicode_sweep.py` now refuses in ~2 s with an empty stdout, the two authoritative
rebuild-first paths (`mise run unicode:sweep`, the `unicode-sweep` CI step) carry the `--rebuilt`
assertion, an empty `*.rs` source set fails closed instead of passing vacuously, and `sweep()`
counts every divergence while retaining at most 20 samples. The diff is exactly the 3 non-test /
non-doc files next.md budgeted, the success line is byte-identical, and I verified the bounded
*reporting* path end-to-end rather than trusting the unit test alone.

**Verification:**

- [x] `timeout 60 uv run scripts/unicode_sweep.py` fails closed — exit **1** in ~2 s, stdout **0
    bytes** (so `grep -c '^TOTAL'` = 0), stderr names `mise run unicode:sweep`
- [x] `mise run unicode:sweep` exits 0; last stdout line confirmed byte-exact with `od -c`:
    `TOTAL 17793024 comparisons, 0 divergences` (preceded by
    `oracle: iscc-core 1.3.0, unidata 16.0.0, python 3.14.6`)
- [x] `grep -c "default=0.0" scripts/unicode_sweep.py` → `0`
- [x] `uv run pytest -q tests/test_unicode_sweep.py` → **14 passed in 0.72 s**; `--collect-only`
    confirms all 10 prior cases survive by name plus the 4 new ones, and all three mandated guard
    assertions are present (480/480/`MAX_REPORTED_DIVERGENCES`; `check_rebuilt([])` SystemExit
    naming the mise task / `check_rebuilt([REBUILD_FLAG])` → `None`; empty **and** missing dirs)
- [x] `uv run pytest -q` → **393 passed** (389 + 4, none removed)
- [x] CI job shape — next.md's PyYAML one-liner prints `OK`; I additionally dumped the job's step
    list: `Build Python bindings (release)` immediately precedes the `--rebuilt` sweep step
- [x] `uv run ruff check` / `ruff format --check` (175 files) / `ruff check --select S,C901` /
    `ty check` — all "All checks passed!"
- [x] `uv run zensical build` → "No issues found"; `uv run scripts/check_docs_nav.py` →
    `OK: 23 documentation pages consistent`; `docs/unicode.md` states the refusal and the mise task
- [x] `mise run check` → all 18 hooks Passed, no file modified (`git status --porcelain` clean apart
    from the runner-owned `iterations.jsonl`)
- [x] `git status --porcelain` over `crates/`, `.crap-baseline.json`, `.iai-baseline.json`,
    `.claude/context/specs/` and `crates/iscc-lib/tests/unicode_boundary.json` — all empty
- [x] Scope: 3 non-test/non-doc files (`scripts/unicode_sweep.py`, `mise.toml`, `ci.yml`) — the
    declared budget exactly. No Not-In-Scope item was touched (mtime set not widened, no maturin
    shell-out, no build fingerprint, sweep not wired into pytest/pre-push, success format frozen)
- [x] Gate integrity: `git diff HEAD~3..HEAD` over the whole unpushed range shows **zero** added
    suppressions, skips, threshold reductions, hook removals or exclusion patterns

**Independent probes beyond next.md** (a green gate proves less than it looks like):

- Near-miss flag forms `--rebuild` and `--rebuilt=true` are both refused — the membership test is
    exact and there is no argparse, so a typo cannot accidentally satisfy the assertion
- End-to-end failure path, driven in-process (`importlib` + `setattr` on `scalar_values` /
    `EXPECTED_*` / `FUNCTION_PAIRS`, 30 scalars, wrong oracle): `main(["--rebuilt"])` returns 1 and
    prints exactly **20** `DIVERGENCE` lines, then `showing first 20 of 480 divergences`, then
    `TOTAL 480 comparisons, 480 divergences`. The cap therefore bounds the *reporting* path, not
    just `sweep()`'s return value — which is what the OOM concern was actually about
- `mise.toml` still `&&`-chains build → sweep, so a failed build can never reach the sweep

**Issues found:**

- (fixed at review, docs precision) `docs/unicode.md` claimed `mise run unicode:sweep` "is the only
    way to run it". That absolute is false — `uv run scripts/unicode_sweep.py --rebuilt` runs it,
    and the code docstring correctly calls the flag a trusted assertion. Reworded to "so always run
    it through `mise run unicode:sweep` (or the CI job), which rebuilds first". The wording came
    verbatim from next.md; advance implemented it faithfully.
- (noted, not filed) `check_extension_fresh` checks the **union** of `RUST_SOURCE_DIRS`, so one
    renamed or emptied directory still passes as long as the other yields sources. This is the same
    vacuity class the issue named, one level down — but it is now redundant behind `--rebuilt`, and
    a renamed core crate is not a silent event. Recorded in review memory rather than issues.md.
- (noted) `test_contexts_discriminate_the_delete_filter_design` now asserts over `result.samples`,
    which is capped at 20. Safe today (8 contexts × 2 fns = 16 comparisons) and it would fail loudly
    rather than falsely if the cap ever bit, but growing `CONTEXTS` past 10 would make that failure
    confusing. Captured in learnings.md.

**Codex review:** Ran clean — "The rebuild assertion is wired through both authoritative execution
paths, divergence retention is correctly bounded while preserving exact counts, and the freshness
guard now fails on an empty source set. The focused test suite passes." No actionable findings.

**Next:** **Propagation slice 5 — the C FFI boundary vectors.** It is the last autonomously
completable surface of the Unicode target's criterion 3 (8 of 11 bindings gated; C++ has no `cmake`
and Swift no toolchain in this container). Verified this review that the surface exists:
`crates/iscc-ffi/src/lib.rs` exports `iscc_text_clean` / `iscc_text_collapse`, both declared in
`include/iscc.h`, and `crates/iscc-ffi/tests/test_iscc.c` (459 lines) is compiled and run by the
`c-ffi` CI job — it simply has no JSON reader and no text-function coverage.

The open design call is how the C test gets the 12 vectors. My recommendation: a checked-in PEP 723
generator that emits a `unicode_boundary_vectors.h` of `static const char *` UTF-8 literals from the
canonical `crates/iscc-lib/tests/unicode_boundary.json`, gated by "re-run the generator, then
`git status --porcelain <header>` must be empty" — the same pattern `scripts/gen_unicode16_*.py`
already establishes. Prefer it over hand-rolling a JSON parser in C (more code, more risk, no
provenance) and over hand-pinned expected strings (drifts silently from the fixture). Note for the
step: a *generated* header is derived, not byte-identical, so it must NOT go into `VENDORED_COPIES`
of `tests/test_vendored_fixtures.py` — the regeneration-no-op gate is its equivalent, and next.md
should say so explicitly.

Smaller alternatives, all human-authorized and self-contained, if a lighter step is wanted: pin
`rubygems/configure-rubygems-credentials` to `@v2.1.0` + `# exact tag:` comment (one line,
statically verifiable, RULED); or make the CI job table in `specs/ci-cd.md` exhaustive (still 14
rows against 21 jobs).

**Notes:**

- The `--rebuilt` flag is a *trusted caller assertion*, not an observation — deliberately, and the
    rationale plus the three rejected alternatives (mtime widening, build fingerprint in the
    extension, in-script maturin shell-out) are now in `decisions.md` 2026-07-27. Do not "fix" the
    residual by adding a fingerprint symbol; it would break the 32-symbol Tier 1 story.
- `TOTAL 17793024 comparisons, 0 divergences` is byte-frozen and asserted in three context files
    plus agent memory. Any change to that line is a breaking change to the gate's contract.
- No Rust source, no API surface, no benchmarked hot path touched — no CRAP or iai baseline refresh
    was needed or made, and none of the feature-matrix / semver gates apply.
- The `unicode-sweep` CI job is the only place the `--rebuilt` wiring is exercised for real; it runs
    on every push, so the next CI run on `develop` is the live proof of the CI half.
