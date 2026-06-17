# Next Work Package

## Step: CRAP gate Phase 3 — regression gate with committed baseline

## Goal

Turn the report-only CRAP gate (Phase 2) into an enforcing regression gate: commit a `cargo crap`
JSON baseline and fail CI when any `iscc-lib` function's CRAP score worsens versus it. This closes
the last open part of the "Add Rust coverage + CRAP-metric quality gate" issue and adds a real
quality gate for the v1.0.0 hardening phase.

## Scope

- **Create**: `.crap-baseline.json` (repo root) — committed CRAP baseline, generated from the
    current code + coverage.
- **Modify**: `.github/workflows/ci.yml` — add an enforcing `--fail-regression --baseline` step to
    the existing `Coverage + CRAP` job (the `coverage` job).
- **Modify**: `mise.toml` — add a `crap:baseline` task that regenerates `.crap-baseline.json`.
- **Modify (doc)**: `.claude/context/specs/ci-cd.md` — flip the Phase 3 checkbox (line 413) to `[x]`
    and update the Phase 3 prose to describe the chosen refresh mechanism.
- **Reference**: `.cargo-crap.toml` (exclude globs apply to both baseline and gate runs),
    `.claude/context/handoff.md` (Phase 3 is the recommended next step),
    `.claude/context/learnings.md` (CI/CD section — cargo-crap conventions).

## Not In Scope

- **Auto-committing/pushing the refreshed baseline from a CI job on `develop` pushes.** During an
    active CID loop a CI-side `git push` races the loop's own pushes (non-fast-forward rejection)
    and risks a push→CI→push cycle. The baseline is refreshed via `mise run crap:baseline` in a
    deliberate reviewed commit instead (matches the iai-callgrind "reviewed commit" pattern).
- The `iai-callgrind` performance-regression gate — separate v1.0.0 item.
- The PyO3 0.23 → 0.29 bump — unrelated, separate issue.
- Flipping the `semver` job to enforcing (drop `continue-on-error`) — tied to the v1.0.0 cut.
- Adding an absolute `--fail-above` threshold gate — regression mode is the deliberate design
    (tolerates existing debt, blocks worsening).
- Changing the existing report-only `--format github` / `--format sarif` steps to baseline/delta
    mode — keep them absolute (`--baseline` is incompatible with `--format sarif`).

## Implementation Notes

Verified end-to-end locally with the installed `cargo-crap 0.2.2` + `cargo-llvm-cov 0.8.7` (rustc
1.96.0 stable; CI uses `dtolnay/rust-toolchain@stable`, the same line).

**Baseline capture (exact commands):**

```bash
cargo llvm-cov -p iscc-lib --lcov --output-path lcov.info
cargo crap --lcov lcov.info --format json --output .crap-baseline.json
```

Do NOT pass `--sort` — it exists only on `cargo-crap` `main`, not in the pinned `0.2.2`. The
baseline envelope is `{ "$schema", "version": "0.2.2", "entries": [...] }`; `--baseline` only reads
this envelope shape. With `.cargo-crap.toml`'s excludes it contains exactly the 97 `iscc-lib`
functions (10 source files) — no binding crates. Generate it via the new `mise run crap:baseline`
task so it is reproducible.

**CI gate step** — append as the LAST step of the `coverage` job (after the SARIF upload, so
diagnostics still run even when the gate fails). Add a step named `CRAP regression gate` whose
`run:` is exactly: `cargo crap --lcov lcov.info --baseline .crap-baseline.json --fail-regression`

`--fail-regression` requires `--baseline`; it exits 1 if any function's score increased beyond
`--epsilon` (default `0.01`), exit 0 otherwise. The job is not `continue-on-error`, so this turns it
into a real gate. Keep the existing `--format github` and `--format sarif` report-only steps
unchanged.

**mise task** — add alongside the existing `coverage` / `crap` tasks:

```toml
[tasks."crap:baseline"]
description = "Regenerate the committed CRAP regression baseline (.crap-baseline.json)"
depends = ["coverage"]
run = "cargo crap --lcov lcov.info --format json --output .crap-baseline.json"
```

**Edge cases / risks:**

- `.crap-baseline.json` must be COMMITTED — it is not matched by `.gitignore` (only `lcov.info` and
    `crap.sarif` are). Do not add it to `.gitignore`.
- Cross-environment determinism: the committed baseline's coverage is generated in the devcontainer;
    CI regenerates coverage on `@stable`. Coverage of the deterministic test suite is stable across
    rustc patch versions and the `--epsilon 0.01` default absorbs float noise. Keep epsilon at the
    default — do NOT inflate it (that would weaken the gate). If the first CI run flaps, the fix is
    to regenerate the baseline from CI's `lcov` artifact, not to widen epsilon.
- `.cargo-crap.toml` is read only when `cargo crap` runs from the repo root (CI checkout CWD is the
    repo root — fine). Its excludes filter baseline entries before comparison, so the gate stays
    consistent with the report.

**Doc update (`ci-cd.md`):** flip checkbox 413 (`- [ ]` → `- [x]`). Update the Phase 3 bullet in the
"Phased rollout" section (around lines 89–92) to state the baseline is committed and refreshed via
`mise run crap:baseline` in a deliberate reviewed commit when merging work into `develop` — not
auto-committed by CI.

**Markdown formatting:** run `uv run mdformat --wrap 100 --number` (or `mise run format`) on any
edited markdown before committing — the pre-push hook reflows changed markdown and rejects a push
otherwise.

## Verification

- `cargo llvm-cov -p iscc-lib --lcov --output-path lcov.info` then
    `cargo crap --lcov lcov.info --baseline .crap-baseline.json --fail-regression` exits 0 against
    the committed baseline.
- A regression is caught: inflate baseline coverage into a temp file
    (`python3 -c "import json;b=json.load(open('.crap-baseline.json'));[e.update(coverage=100.0,crap=e['cyclomatic']) for e in b['entries']];json.dump(b,open('/tmp/b.json','w'))"`)
    then `cargo crap --lcov lcov.info --baseline /tmp/b.json --fail-regression` exits non-zero.
- `git ls-files --error-unmatch .crap-baseline.json` succeeds (file is tracked, not ignored); the
    JSON has top-level keys `$schema`, `version`, `entries` and every entry's `file` is under
    `crates/iscc-lib/`.
- `grep -E 'fail-regression|crap-baseline' .github/workflows/ci.yml` shows the new gate step.
- `mise tasks | grep '^crap:baseline'` lists the new task and `mise run crap:baseline` regenerates
    `.crap-baseline.json` with no spurious diff.
- `mise run check` passes (all pre-commit hooks green; no out-of-scope context reflow).
- `ci-cd.md` line 413 checkbox is `[x]`; the Phase 3 prose names `mise run crap:baseline` as the
    refresh mechanism.

## Done When

The `coverage` CI job runs an enforcing `cargo crap --fail-regression` against the committed
`.crap-baseline.json`, the `crap:baseline` mise task regenerates that baseline reproducibly, and all
verification checks above pass.
