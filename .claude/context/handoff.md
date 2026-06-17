## 2026-06-17 — CRAP gate Phase 2: report-only `cargo crap` (GitHub annotations + SARIF) + `.cargo-crap.toml` + `mise run crap`

**Done:** Extended the existing `coverage` CI job to run report-only `cargo crap` (pinned `0.2.2`,
installed via `cargo binstall`) against the already-generated `lcov.info` — emitting
`--format github` inline annotations and uploading a `--format sarif` report to GitHub Code
Scanning. Added a repo-root `.cargo-crap.toml` (threshold 30, pessimistic missing-coverage,
binding-crate excludes), a `mise run crap` task, and flipped the Phase 2 checkboxes in `ci-cd.md`.
No build-failing CRAP behavior introduced.

**Files changed:**

- `.cargo-crap.toml` (new): `threshold = 30.0`, `missing = "pessimistic"`, `exclude` globs for all 7
    binding crates + `packages/**` + `scripts/**`. **Also excludes `crates/iscc-lib/benches/**`** —
    see Notes.
- `.github/workflows/ci.yml`: `coverage` job renamed to
    `Coverage + CRAP (cargo llvm-cov + cargo   crap)`; added job-level
    `permissions: { contents: read, security-events: write }`, a cargo-binstall install step, a
    `cargo binstall -y cargo-crap@0.2.2` step, two report-only `cargo   crap` steps
    (`--format github`, `--format sarif --output crap.sarif`), and a
    `github/codeql-action/upload-sarif@v3` upload.
- `mise.toml`: added `[tasks.crap]` (`depends = ["coverage"]`, runs `cargo crap --lcov lcov.info`).
- `.gitignore`: added `crap.sarif`.
- `.claude/context/specs/ci-cd.md`: flipped Phase 2 checkboxes (CRAP report-only job, cargo-crap
    pinned+binstall, `.cargo-crap.toml` config, `mise run coverage`/`crap` local repro). Phase 3
    checkbox left `[ ]`.

**Verification:** All next.md criteria pass.

- `cargo crap --lcov lcov.info` exits 0 (report-only; 97 functions analyzed, none exceed threshold
    30 — highest is `gen_meta_code_v0` at CRAP 22.3).
- `mise run crap` exits 0, regenerates `lcov.info` via the `coverage` dep, prints the CRAP table;
    `mise tasks | grep '^crap'` shows the task.
- `cargo crap --lcov lcov.info --format sarif --output /tmp/crap.sarif` → valid SARIF
    (`jq -e '.runs[0].tool.driver.name'` → `"cargo-crap"`). `--format github` exits 0 (no
    annotations emitted because nothing exceeds threshold 30).
- Report contains only `crates/iscc-lib/src/*.rs` functions — no binding crates, no `packages/`, no
    benches.
- `grep -E 'cargo-crap|cargo crap|upload-sarif|security-events' ci.yml` shows install + both runs +
    SARIF upload + permission. `grep -E 'fail-above|fail-regression|baseline' ci.yml` returns
    nothing.
- `git status --porcelain` lists neither `lcov.info` nor `crap.sarif` (both gitignored).
- `mise run check` exits 0 — all 15 pre-commit hooks pass (incl. taplo reformat of
    `.cargo-crap.toml`, already applied). No out-of-scope context-file reflow this run.

**Next:** Two unstarted `normal` backlog gates remain: (1) **Phase 3 CRAP regression gate** —
capture a baseline JSON via `cargo crap --format json --output`, run with
`--fail-regression --baseline`, refresh the baseline on merges to `develop` (this is the enforcing
step Phase 2 deliberately defers); (2) **`iai-callgrind` perf gate** — instruction-count benches for
hot `gen_*_v0`/hashing/CDC/MinHash paths with a committed baseline failing on >10% regression. Phase
3 builds directly on this Phase 2 work and is the most incremental follow-on.

**Notes:**

- **Deviation from next.md (justified):** next.md asserted "the built-in default excludes already
    skip `tests/**` / `benches/**`". That holds for `tests/**` (cargo-crap's nested-tests default
    excluded `crates/iscc-lib/tests/*.rs`) but **NOT** for benches — the built-in `benches/**`
    pattern only matches the repo-root path, so `crates/iscc-lib/benches/benchmarks.rs` leaked in
    and scored as the #1 "crappiest" function (CRAP 42.0, no coverage — pure harness noise). I added
    `crates/iscc-lib/benches/**` to the `.cargo-crap.toml` exclude list to fulfill next.md's stated
    intent ("report contains only iscc-lib functions" = library code). This is a one-line config
    addition, fully within the "Create `.cargo-crap.toml`" scope.
- CI does NOT use `mise`; `mise run crap`/`coverage` mirror the CI commands for local repro only —
    they cannot drift silently (identical command strings).
- Job-level `permissions` on the `coverage` job does not affect other jobs (they inherit the repo
    default token perms). `security-events: write` is required for the Code Scanning SARIF upload;
    iscc-lib is a public repo so Code Scanning is available.
- The SARIF upload + GitHub annotations cannot be verified pre-push (require the live runner);
    everything else verified locally with `cargo-crap 0.2.2` installed via
    `cargo install   cargo-crap@0.2.2 --locked` in the devcontainer (binstall is not preinstalled
    here).
- `lcov.info` remains in the working tree (gitignored) as a side effect of local verification.
