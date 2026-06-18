## 2026-06-18 — Add the `Perf` CI job that runs the iai-callgrind benches under valgrind

**Done:** Added a `perf` job (`Perf (iai-callgrind)`) to `.github/workflows/ci.yml` that installs
valgrind + `iai-callgrind-runner@0.16.1` and runs `cargo bench -p iscc-lib --bench iai_benches`,
uploading the `target/iai/` output as the `iai-baseline` artifact. Added a `bench:iai` mise task for
CI parity / local use on valgrind-equipped machines. No baseline, no regression gate, no harness
changes — those are the explicit follow-up slice (2b).

**Files changed:**

- `.github/workflows/ci.yml`: new `perf` job between `bench` and `semver`. Steps: checkout →
    `dtolnay/rust-toolchain@stable` → `Swatinem/rust-cache@v2` → install valgrind via apt →
    `taiki-e/install-action@v2` (cargo-binstall) →
    `cargo binstall -y --force   iai-callgrind-runner@0.16.1` →
    `cargo bench -p iscc-lib --bench iai_benches` → `actions/upload-artifact@v4` (name
    `iai-baseline`, path `target/iai/`). Default triggers (inherits the workflow-level push
    main/develop + PR to main). The `--force` rationale and the "first-run = measure-only, exit 0"
    behavior are documented as inline comments.
- `mise.toml`: new "Performance" block with `[tasks."bench:iai"]` →
    `cargo bench -p iscc-lib --bench iai_benches`.

**Verification:**

- `mise run check` → all 15 pre-commit hooks Passed (YAML + TOML validation + yamlfix/taplo
    formatting clean on the edited files).
- All six next.md grep/task checks pass: `Perf (iai-callgrind)`, `iai-callgrind-runner@0.16.1`,
    `binstall -y --force`, `valgrind`, `--bench iai_benches` present in ci.yml; `bench:iai` shows in
    `mise tasks ls`.
- `cargo build -p iscc-lib --bench iai_benches` → exit 0 (harness still compiles; the job will run
    it under valgrind on CI).
- NOT run locally (valgrind absent in devcontainer): `cargo bench -p iscc-lib --bench iai_benches`.
    This is the step the review agent must confirm on the actual CI run.

**Next:** Slice 2b — the regression gate. Once this `Perf` run lands a known-good `target/iai/`
output on CI, design the committed baseline: decide between iai-callgrind's `--save-baseline` /
`--baseline` named-baseline flow vs. parsing the per-bench `*.summary.json`, commit the baseline
file, add a `LibraryBenchmarkConfig` regression limit (or `--fail-...` CLI flag) to fail the job on
a >X% instruction-count regression, and add a `bench:iai:baseline` refresh mise task. Inspect the
uploaded `iai-baseline` artifact from this slice's first CI run to learn the exact on-disk layout
and summary format before committing to an approach.

**Notes:**

- **Omitted `-- --save-summary=json`.** next.md flagged it as optional and unverifiable locally (no
    valgrind/runner). I left it off so the job relies on iai-callgrind 0.16's default `target/iai/`
    output; if 2b needs machine-readable summaries, the flag name can be confirmed against the
    artifact this run produces. No risk to this slice landing green.
- **No tests added** — this is a CI-workflow + task-runner change with no Rust/source surface. No
    Tier 1/Tier 2 API touched, no hot path touched, no perf-sensitive code touched.
- **Context-file churn during `mise run format`:** the quiet `mise run format` mdformat hook
    reformatted pre-existing non-conforming `learnings.md` / `learnings-archive.md` (files I did not
    edit). I reverted both via `git checkout` so they are untouched; only my two files plus the
    loop-managed `iterations.jsonl` remain modified. The subsequent `mise run check` mdformat pass
    was clean and did not re-touch them.
- Staged for commit: `.github/workflows/ci.yml`, `mise.toml`, `.claude/context/handoff.md`,
    `.claude/agent-memory/advance/MEMORY.md`. NOT staged: `iterations.jsonl` (loop-runner managed).
