## 2026-06-17 — CRAP gate Phase 3 — regression gate with committed baseline

**Done:** Turned the report-only Phase 2 CRAP gate into an enforcing regression gate. Captured a
committed `.crap-baseline.json` (97 `iscc-lib` functions, 10 source files), added an enforcing
`cargo crap --fail-regression --baseline` step as the last step of the `Coverage + CRAP` CI job, and
added a reproducible `mise run crap:baseline` task to regenerate the baseline.

**Files changed:**

- `.crap-baseline.json` (new): committed CRAP baseline. Envelope
    `{$schema, version: "0.2.2",   entries:[...]}` — 97 entries, every `file` under
    `./crates/iscc-lib/src/`. Generated via the exact commands in next.md; `.cargo-crap.toml`
    excludes filter out all binding crates + benches.
- `.github/workflows/ci.yml`: appended a `CRAP regression gate` step (after the SARIF upload)
    running `cargo crap --lcov lcov.info --baseline .crap-baseline.json --fail-regression`. Not
    `continue-on-error`, so it is a real gate. The two report-only steps are unchanged.
- `mise.toml`: added `[tasks."crap:baseline"]` (`depends=["coverage"]`) running
    `cargo crap --lcov lcov.info --format json --output .crap-baseline.json`.
- `.claude/context/specs/ci-cd.md`: flipped the Phase 3 checkbox (now `[x]`) and rewrote the Phase 3
    "Phased rollout" bullet to describe the committed baseline + `mise run crap:baseline` refresh in
    a reviewed commit (not CI auto-commit).

**Verification:** All next.md criteria pass.

- Pass case: `cargo crap --lcov lcov.info --baseline .crap-baseline.json --fail-regression` exits 0
    (`0 regressed`, `97 unchanged`) against fresh-regenerated coverage.
- Catch case: inflated-coverage baseline (`/tmp/b.json`) → exit 1 (`17 regressed`). NOTE: when
    testing exit codes, do NOT pipe `cargo crap` into `tail` — `$?` then reflects `tail`, masking
    the gate's real exit. Redirect to a file and check `$?` directly.
- `mise run crap:baseline` regenerates `.crap-baseline.json` byte-for-byte identical (idempotent, no
    spurious diff). `mise tasks | grep '^crap'` lists both `crap` and `crap:baseline`.
- `git check-ignore .crap-baseline.json` → not ignored (only `lcov.info` + `crap.sarif` are); JSON
    top-level keys are `$schema`, `version`, `entries`; all entry files under `crates/iscc-lib/`.
- `grep -E 'fail-regression|crap-baseline' .github/workflows/ci.yml` shows the gate step.
- `mise run check` — all 15 pre-commit hooks green (mdformat included; no out-of-scope context
    reflow). Baseline (18.8 KB / 782 lines) passes the large-file hook.

**Next:** Two unstarted `normal` v1.0.0 CI gates remain:

1. **`iai-callgrind` perf-regression gate** — instruction-count benches for hot
    `gen_*_v0`/hashing/CDC/MinHash with a committed baseline failing on >10% regression (ci-cd.md
    line 420 checkbox, still `[ ]`). Mirrors the reviewed-baseline pattern just established.
2. **PyO3 0.23 → 0.29 bump** — clears two RustSec advisories in the wheel; scope to
    `crates/iscc-py/`, migrate incrementally.

Also: flipping `semver` to enforcing and this `coverage` job's regression gate become the v1.0.0 cut
checklist.

**Notes:**

- `mise run check` runs only the pre-commit stage (formatting/lint), not pre-push (clippy/test).
    This change touches zero Rust source, so cargo test/clippy are unaffected — verified the gate
    against a fresh `cargo llvm-cov` run, not a stale `lcov.info`.
- Cross-environment determinism risk (next.md): the committed baseline's coverage comes from the
    devcontainer; CI regenerates on `@stable`. The `--epsilon 0.01` default absorbs float noise. If
    CI's first run flaps, the fix is to regenerate the baseline from CI's `lcov` artifact, NOT to
    widen epsilon. Did not touch epsilon.
- `.claude/context/iterations.jsonl` shows as modified in git status — that is loop bookkeeping, NOT
    staged by this commit (only handoff.md among context files is staged).
- Did not pass `--sort` (0.2.2 lacks it) and used `--format json` exactly per next.md.
