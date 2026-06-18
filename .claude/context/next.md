# Next Work Package

## Step: Add the `Perf` CI job that runs the iai-callgrind benches under valgrind

## Goal

Stand up the Linux `Perf` CI job that installs valgrind + `iai-callgrind-runner` and actually *runs*
the instruction-count benches (today they are only compile-checked by `Bench`). This proves the
toolchain works on the runner and produces the real iai-callgrind output the regression-gate slice
needs. Advances issue "Add `iai-callgrind` performance-regression CI gate" without trying to land
the committed baseline blind (valgrind is absent locally, so a baseline can't be generated or
verified in the devcontainer).

## Scope

- **Modify**: `.github/workflows/ci.yml` — add a `perf` job.
- **Modify**: `mise.toml` — add a `bench:iai` task.
- **Reference**:
    - `.claude/context/handoff.md` — the follow-up slice description.
    - `crates/iscc-lib/benches/iai_benches.rs` — the harness this job runs (`--bench iai_benches`).
    - `.claude/context/specs/ci-cd.md` → "Performance — `iai-callgrind`" (lines 122-130) and the
        `Perf` row in the gate table (line 32).
    - `.claude/context/specs/rust-core.md` → "API Stability & Performance Invariants" (lines 359-376).
    - `.claude/context/learnings.md` → CI/CD section, "`cargo binstall` + `Swatinem/rust-cache`
        poisoning" (the `--force` rule) and the "Benchmarking (iai-callgrind)" entry.
    - Existing `coverage` job (ci.yml:293-336) for the `taiki-e/install-action` +
        `cargo binstall   -y --force` pattern, and the `bench` job (ci.yml:271-279) for the
        toolchain/cache setup.

## Not In Scope

- **No committed baseline and no regression gate yet.** Do NOT add `--baseline`,
    `--fail-regression`, a `.iai-baseline.*` file, or any in-harness regression limit. The baseline
    must be produced by *this* slice's CI run first (valgrind is unavailable locally), and the
    committed-baseline glue needs design — that is the explicit follow-up slice (2b).
- Do NOT edit `crates/iscc-lib/benches/iai_benches.rs` (no `LibraryBenchmarkConfig` regression
    config this slice).
- Do NOT add a baseline-refresh `mise` task (e.g. `bench:iai:baseline`) — that belongs with 2b.
- Do NOT flip the `Semver` job's `continue-on-error` (tied to the v1.0.0 cut).
- Do NOT touch the CRAP gate, `.cargo-crap.toml`, or `.crap-baseline.json`.
- Do NOT add `cargo deny`/`cargo audit` (separate `[review]` issue, human-review hold).
- Do NOT update docs/notes for the perf gate — defer to 2b once the gate semantics are final (this
    slice completes no spec verification checkbox).

## Implementation Notes

- **Job shape** — model on the `coverage` job for tool install and the `bench` job for setup. Key
    `perf`, name `Perf (iai-callgrind)`, `runs-on: ubuntu-latest`, default triggers (push to
    main/develop, PR to main). Steps:

    1. `actions/checkout@v4`
    2. `dtolnay/rust-toolchain@stable`
    3. `Swatinem/rust-cache@v2`
    4. Install valgrind: `sudo apt-get update && sudo apt-get install -y valgrind`
    5. Install cargo-binstall via `taiki-e/install-action@v2` (tool: `cargo-binstall`), matching the
        `coverage` job.
    6. `cargo binstall -y --force iai-callgrind-runner@0.16.1` — the **`--force` is load-bearing**:
        `Swatinem/rust-cache` restores cargo's install *metadata* without the
        `~/.cargo/bin/iai-callgrind-runner` binary, so a plain binstall would skip install and the
        next `cargo bench` would die with "no such command". (See learnings.)
    7. `cargo bench -p iscc-lib --bench iai_benches` — scopes to the iai harness only, NOT the
        wall-clock criterion `benchmarks` bench. On a first run with no baseline iai-callgrind just
        measures and reports (exit 0), so the job is green without any gate.
    8. Upload results: `actions/upload-artifact@v4` with `name: iai-baseline`, `path: target/iai/`.
        This gives the 2b slice the exact on-disk layout / summary format to build the committed
        baseline from. Optionally pass `-- --save-summary=json` to emit machine-readable summaries —
        confirm the flag name against iai-callgrind 0.16 (runtime-only; can't verify locally); if
        unsure, omit it and rely on the default `target/iai/` output.

- **Version pin** — the runner version MUST match the `iai-callgrind = "0.16"` workspace dep (it
    resolves `0.16.1`), so pin `@0.16.1`.

- **mise task** — add under a new "Performance" comment block (sibling to "Coverage"):

    ```
    [tasks."bench:iai"]
    description = "Run iai-callgrind instruction-count benches for iscc-lib (needs valgrind)"
    run = "cargo bench -p iscc-lib --bench iai_benches"
    ```

    It will fail locally without valgrind — that is expected; the task is for CI parity and use on
    valgrind-equipped machines.

- **Before committing**, run `mise run format` (or `uv run mdformat --wrap 100 --number` for context
    files) so yamlfix/taplo normalization of `ci.yml`/`mise.toml` does not fail the commit or the
    pre-push hook.

## Verification

- `mise run check` passes (YAML + TOML pre-commit hooks validate the edited `ci.yml` and
    `mise.toml`; formatting clean).
- `grep -q "Perf (iai-callgrind)" .github/workflows/ci.yml` (the job exists).
- `grep -q "iai-callgrind-runner@0.16.1" .github/workflows/ci.yml` (runner pinned to match the lib).
- `grep -q "binstall -y --force" .github/workflows/ci.yml` and
    `grep -q "valgrind"   .github/workflows/ci.yml` (toolchain install present).
- `grep -q -- "--bench iai_benches" .github/workflows/ci.yml` (runs the iai harness, not criterion).
- `mise tasks ls | grep -q "bench:iai"` (task registered).
- CI-verified by the review agent (valgrind absent locally): the `Perf` job concludes `success`,
    runs the benches under valgrind, and uploads the `target/iai` artifact; no existing job
    regresses.

## Done When

The `Perf` CI job is defined and the `bench:iai` task exists, all local syntax/grep checks pass, and
the next CI run shows the `Perf` job green with an uploaded iai-callgrind results artifact.
