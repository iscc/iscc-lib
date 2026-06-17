# Next Work Package

## Step: Add Phase 1 Rust coverage CI job (`cargo llvm-cov` LCOV artifact) + `mise run coverage` task

## Goal

Stand up Phase 1 of the "Add Rust coverage + CRAP-metric quality gate" issue: produce a reusable
LCOV coverage report for the `iscc-lib` core crate in CI and make the same run reproducible locally.
This closes the first phased checkbox toward the target's CI/CD coverage criterion without yet
adding any gating, so it is low-risk and fully verifiable before push.

## Scope

- **Modify**: `.github/workflows/ci.yml` (add a `coverage` job that generates and uploads
    `lcov.info`)
- **Modify**: `mise.toml` (add a `[tasks.coverage]` task)
- **Modify**: `.gitignore` (ignore the generated `lcov.info`)
- **Modify (doc, excluded from file limit)**: `.claude/context/specs/ci-cd.md` (flip the Phase 1
    checkbox, line 409, to `[x]`; reword to the artifact-only state)
- **Reference**: `.claude/context/specs/ci-cd.md` → "Rust Coverage and CRAP Quality Gate" / "Phased
    rollout"; the existing `semver` job in `.github/workflows/ci.yml` (added iter 93) and
    `[tasks.semver]` in `mise.toml` as structural templates; `crates/iscc-lib` (coverage target)

## Not In Scope

- **Phase 2 (report-only `cargo crap`)** — no `cargo-crap` install, no `--format github`
    annotations, no SARIF upload to GitHub Code Scanning. Defer to a future step.
- **Phase 3 (`--fail-regression` gate + committed baseline)** — no score gating, no baseline JSON,
    no `develop`-merge baseline refresh. Defer.
- `.cargo-crap.toml` and the `mise run crap` task — both belong to Phase 2/3.
- Do NOT make coverage a failing threshold or add a coverage badge wiring in this step.
- The `iai-callgrind` perf gate and the PyO3 0.23→0.29 bump are separate issues — leave untouched.
- Do not add coverage instrumentation to the binding crates (PyO3/napi/wasm/etc.); Phase 1 targets
    `iscc-lib` only (`cargo llvm-cov -p iscc-lib`).

## Implementation Notes

- **CI job** (`.github/workflows/ci.yml`): add a top-level job named `coverage`
    (`name: Coverage (cargo llvm-cov)`, `runs-on: ubuntu-latest`). Steps:
    1. `actions/checkout@v4`
    2. `dtolnay/rust-toolchain@stable` with `components: llvm-tools-preview` (cargo-llvm-cov requires
        the `llvm-tools-preview` rustup component).
    3. `Swatinem/rust-cache@v2` (matches existing jobs).
    4. Install the tool with `taiki-e/install-action@v2` using `tool: cargo-llvm-cov` (the
        maintainer-blessed installer — fetches a prebuilt binary, mirrors how `semver` uses the
        cargo-semver-checks wrapper action).
    5. Run `cargo llvm-cov -p iscc-lib --lcov --output-path lcov.info`.
    6. Upload with `actions/upload-artifact@v4` (`name: lcov`, `path: lcov.info`). Recommendation: do
        NOT set `continue-on-error` — Phase 1 has no score gate, so its only failure mode is the test
        suite failing under instrumentation, which should be visible. (If llvm-cov proves flaky on
        the shared runner, `continue-on-error: true` is the fallback, but try plain first.) Keep the
        job standalone (no `needs:`), consistent with the other independent jobs.
- **mise task** (`mise.toml`): add `[tasks.coverage]` with
    `run = "cargo llvm-cov -p iscc-lib --lcov --output-path lcov.info"` and a one-line
    `description`, placed alongside the existing `[tasks.semver]` block. Note: CI calls `cargo`
    directly (it does NOT use `mise`), so the task is for local reproducibility only — mirror
    exactly the command the CI step runs.
- **.gitignore**: add `lcov.info` under the existing "Unit test / coverage reports" section (line
    ~40) so the generated report is never committed.
- **ci-cd.md**: flip only the Phase 1 checkbox (line 409, "CRAP job generates an LCOV report for
    `iscc-lib` via `cargo llvm-cov` (Phase 1)") to `[x]`. Leave the Phase 2, Phase 3,
    `.cargo-crap.toml`, and `mise run crap`/`mise run coverage` combined-task checkboxes unchecked —
    the `mise run crap` half of line 416 is not delivered yet.
- **Markdown formatting**: run `uv run mdformat --wrap 100 --number` (or `mise run format`) on any
    edited markdown before committing — the pre-push hook reflows changed markdown and will reject a
    push otherwise (root cause noted in the iter-93 review).

## Verification

- `cargo install cargo-llvm-cov` (network is available) succeeds, then
    `cargo llvm-cov -p iscc-lib --lcov --output-path lcov.info` exits 0 and writes a non-empty
    `lcov.info` (e.g., `test -s lcov.info`).
- `mise run coverage` exits 0 and produces `lcov.info`.
- `mise tasks | grep '^coverage'` shows the new task.
- `git status --porcelain` does NOT list `lcov.info` (confirms it is gitignored).
- `mise run check` passes on the edited files (ci.yml YAML, mise.toml TOML, ci-cd.md markdown all
    valid; no unrelated regressions).
- `grep -n "cargo-llvm-cov\|llvm-cov" .github/workflows/ci.yml` shows the new `coverage` job wiring.
- Deferred to next CI run (review agent confirms): the new `Coverage` job appears and the existing
    17 jobs stay green (18 jobs total).

## Done When

A `coverage` CI job and a `mise run coverage` task both produce a `lcov.info` LCOV report for
`iscc-lib`, `lcov.info` is gitignored, the Phase 1 checkbox in ci-cd.md is flipped, and all local
verification commands above pass.
