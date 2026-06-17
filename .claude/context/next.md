# Next Work Package

## Step: Fix the CI `Coverage + CRAP` cargo-crap install flake (make CI green)

## Goal

The `Coverage + CRAP` CI job is failing on every run with `error: no such command: crap`, flipping
the whole CI run to RED. Make the `cargo-crap` install deterministic so the binary is always present
and the CRAP report/gate steps run. A green CI is the prerequisite for all further v1.0.0 work.

## Scope

- **Modify**: `.github/workflows/ci.yml` — the `Install cargo-crap` step (line 314) in the
    `coverage` (`Coverage + CRAP`) job.
- **Reference**:
    - `.claude/context/state.md` → "Next Milestone" item 1 (root-cause analysis of the flake)
    - `.claude/context/learnings.md` → "CI/CD" → CRAP gate entries
    - `.claude/context/specs/ci-cd.md` → "Rust Coverage and CRAP Quality Gate" (install described
        generically as "via `cargo binstall`" — stays accurate, do NOT edit)

## Not In Scope

- **Do NOT advance the PyO3 migration (0.25 → 0.26)** — that is the prior handoff "Next", but a red
    CI blocks all feature work. Resume it only after CI is green again.
- Do NOT add `--fail-above 30` to the CRAP gate (separate [review] issue, needs HUMAN REVIEW).
- Do NOT touch the `iai-callgrind` perf gate, the semver job, or flip any `continue-on-error`.
- Do NOT rewrite the install to a different action (e.g. `taiki-e/install-action` for cargo-crap) or
    restructure the `coverage` job — keep the change to a single flag on the existing step.
- Do NOT edit `ci-cd.md` prose: it documents the install generically ("installed via
    `cargo binstall`"), which remains true after the fix.

## Implementation Notes

Root cause (from state.md): `Swatinem/rust-cache@v2` restores cargo's installed-crate metadata
(`.crates.toml` / `.crates2.json`) WITHOUT the actual `~/.cargo/bin/cargo-crap` binary. So
`cargo binstall -y cargo-crap@0.2.2` sees the metadata, logs
`cargo-crap v0.2.2 is already installed, use --force to override`, and **skips** the install — but
the binary is not on PATH, so the next `cargo crap` step dies with `error: no such command: crap`.

Minimal robust fix: add `--force` to the binstall invocation so it installs the binary every run
regardless of the cached metadata record. Change line 314 from:

```yaml
run: cargo binstall -y cargo-crap@0.2.2
```

to:

```yaml
run: cargo binstall -y --force cargo-crap@0.2.2
```

`--force` is preferred over disabling `cache-bin` on the rust-cache step because it is explicit,
self-documenting, and independent of rust-cache's internal caching behavior. cargo-crap is a small
binary, so the re-download cost each run is negligible. Leave every other step in the `coverage` job
(toolchain, rust-cache, cargo-llvm-cov install, lcov generation, the two report-only CRAP steps,
SARIF upload, and the `--fail-regression` enforcing gate) exactly as-is.

## Verification

- `grep -n 'cargo binstall -y --force cargo-crap@0.2.2' .github/workflows/ci.yml` matches exactly
    one line.
- `python3 -c "import yaml; yaml.safe_load(open('.github/workflows/ci.yml'))"` exits 0 (workflow
    YAML still parses).
- `git diff --stat` shows `.github/workflows/ci.yml` as the only changed file, with a 1-line change
    (the `Install cargo-crap` step), and no other `coverage`-job step altered.
- Next CI run's `Coverage + CRAP` job reaches the CRAP report/gate steps and the overall run returns
    to GREEN (review agent confirms on the next push — cannot be run locally).

## Done When

The `Install cargo-crap` step in `ci.yml` uses `cargo binstall -y --force cargo-crap@0.2.2`, the
workflow YAML still parses, and the change is isolated to that single line so the next CI run's
`Coverage + CRAP` job installs the binary and goes green.
