# Next Work Package

## Step: CRAP gate Phase 2 — report-only `cargo crap` (GitHub annotations + SARIF) + `.cargo-crap.toml` + `mise run crap`

## Goal

Extend the existing Phase 1 coverage CI job so it computes per-function CRAP scores for `iscc-lib`
from `lcov.info` in **report-only** mode — emitting inline `::warning` annotations and uploading a
SARIF report to GitHub Code Scanning — backed by a pinned `cargo-crap`, a `.cargo-crap.toml` config,
and a local `mise run crap` task. This is Phase 2 of the "Add Rust coverage + CRAP-metric quality
gate" issue (Phase 1 LCOV artifact already landed; Phase 3 regression gate stays out of scope).

## Scope

- **Create**: `.cargo-crap.toml` (repo root) — `threshold`, `missing`, and binding-crate `exclude`
    globs.
- **Modify**: `.github/workflows/ci.yml` — add `cargo-crap` install + report-only `cargo crap` steps
    (GitHub + SARIF) and a SARIF upload to the existing `coverage` job; add `permissions` for SARIF.
- **Modify**: `mise.toml` — add a `[tasks.crap]` task mirroring the local check.
- **Reference**: `.claude/context/specs/ci-cd.md` (→ "Rust Coverage and CRAP Quality Gate" + the CI
    verification checkboxes at lines 411/415/416/417 — flip the ones Phase 2 satisfies); the
    existing `coverage` job (`ci.yml:293-312`) and `coverage` task (`mise.toml:110-112`) as the
    pattern to follow.

## Not In Scope

- **Phase 3** — no `--fail-above`, no `--fail-regression`, no `--baseline`, no committed baseline
    JSON, no baseline-refresh-on-`develop` workflow. Phase 2 must NOT fail the build on CRAP score.
- The `iai-callgrind` performance-regression gate (separate `normal` issue).
- The PyO3 0.23 → 0.29 bump (separate `normal` issue).
- Do NOT flip the `semver` job's `continue-on-error` to enforcing — that is tied to the v1.0.0 cut.
- Do NOT change `cargo llvm-cov` scope/flags or commit `lcov.info` / `crap.sarif` (both stay out of
    the working tree).

## Implementation Notes

- **Tool version**: pin `cargo-crap@0.2.2` (latest published release; the README on `main` shows a
    v0.3.0 badge but no v0.3.0 release exists yet). v0.2.2 supports everything needed:
    `--lcov <FILE>`, `--format {human,json,github,markdown,pr-comment,sarif}`, `--threshold`,
    `--missing {pessimistic,optimistic,skip}`, `--exclude <GLOB>`, `--output <FILE>`, and a
    `.cargo-crap.toml` config file at the project root.
- **CI install (spec mandates `cargo binstall`)**: in the `coverage` job, install cargo-binstall
    (e.g. `uses: taiki-e/install-action@v2` with `tool: cargo-binstall`), then
    `run: cargo binstall -y cargo-crap@0.2.2`. (The README also publishes prebuilt linux-x86_64
    tarballs if binstall proves flaky — a documented fallback, not the primary path.)
- **CI steps (add after the existing LCOV-generation step, reusing the `lcov.info` already in the
    workspace)**:
    1. `cargo crap --lcov lcov.info --format github` — inline `::warning` annotations. No
        `--fail-above`, so it exits 0 (report-only).
    2. `cargo crap --lcov lcov.info --format sarif --output crap.sarif` — SARIF 2.1.0 document.
    3. Upload via `github/codeql-action/upload-sarif@v3` with `sarif_file: crap.sarif`. Add job-level
        `permissions: { contents: read, security-events: write }` (the job currently has none;
        `security-events: write` is required for Code Scanning upload). iscc-lib is a public repo, so
        Code Scanning is available.
- Consider renaming the job's `name:` to reflect both tools (e.g.
    `Coverage + CRAP (cargo llvm-cov + cargo crap)`); keeping the `coverage:` job key avoids churn.
- **`.cargo-crap.toml`** (report-only — do NOT set `fail-above`): set `threshold = 30.0`,
    `missing = "pessimistic"`, and `exclude` globs for every binding crate so they don't flood the
    report with 0%-coverage noise (the LCOV only covers `iscc-lib`). Exclude `crates/iscc-py/**`,
    `crates/iscc-napi/**`, `crates/iscc-wasm/**`, `crates/iscc-ffi/**`, `crates/iscc-jni/**`,
    `crates/iscc-rb/**`, `crates/iscc-uniffi/**`, `packages/**`, `scripts/**`. The built-in default
    excludes already skip `tests/**` / `benches/**` / `examples/**`. (Leaving `--path` at its `.`
    default + exclude globs is what satisfies the spec's "excluded binding crates" checkbox — do not
    narrow with `--path crates/iscc-lib` instead.)
- **`mise.toml`**: add `[tasks.crap]` running `cargo crap --lcov lcov.info` (human format, no file
    output → clean working tree). A `depends = ["coverage"]` makes `mise run crap` regenerate
    `lcov.info` first for one-command local repro.
- **Local install for verification**: cargo-binstall is not preinstalled in the devcontainer; the
    advance agent can `cargo install cargo-crap@0.2.2 --locked` (network available, compiles from
    source) or download the prebuilt linux-x86_64 tarball. When testing the SARIF format locally,
    write to `--output /tmp/crap.sarif` so nothing lands in the working tree.
- **Doc sync**: in `ci-cd.md`, flip the Phase 2 checkbox (line 411), the `cargo-crap` pinned +
    binstall checkbox (415), the `.cargo-crap.toml` config checkbox (416), and the
    `mise run coverage` / `mise run crap` checkbox (417). Leave the Phase 3 checkboxes (413/414)
    unchecked.
- **Markdown formatting**: run `uv run mdformat --wrap 100 --number` (or `mise run format`) on any
    edited markdown before committing — the pre-push hook reflows changed markdown and rejects a
    push otherwise.

## Verification

- `cargo crap --lcov lcov.info` exits 0 locally after `mise run coverage` generates `lcov.info`
    (report-only; non-failing).
- `mise run crap` exits 0 and prints the CRAP table; `mise tasks | grep '^crap'` shows the task.
- `cargo crap --lcov lcov.info --format sarif --output /tmp/crap.sarif` produces valid JSON
    (`jq -e '.runs[0].tool.driver.name' /tmp/crap.sarif` exits 0); SARIF is written to `/tmp`, not
    the repo.
- The report contains only `iscc-lib` functions — no `crates/iscc-{py,napi,wasm,ffi,jni,rb,uniffi}`
    or `packages/` entries (confirms `.cargo-crap.toml` `exclude` globs apply).
- `grep -E 'cargo-crap|cargo crap|upload-sarif|security-events' .github/workflows/ci.yml` shows the
    install, both `cargo crap` runs (`--format github` and `--format sarif`), the SARIF upload, and
    the `security-events: write` permission.
- `grep -E 'fail-above|fail-regression|baseline' .github/workflows/ci.yml` returns nothing (Phase 2
    is strictly report-only).
- `git status --porcelain` does NOT list `lcov.info` or `crap.sarif` (both stay untracked/ignored).
- `mise run check` passes on the edited files (YAML/TOML/Markdown hygiene; no out-of-scope reflow of
    context files).
- `ci-cd.md` Phase 2 checkboxes (lines 411, 415, 416, 417) are flipped to `[x]`; Phase 3 (413/414)
    remain `[ ]`.

## Done When

The `coverage` CI job runs report-only `cargo crap` (pinned `0.2.2`) emitting GitHub annotations and
uploading a SARIF report, `.cargo-crap.toml` scopes scoring to `iscc-lib`, `mise run crap`
reproduces the check locally, all verification criteria pass, and the Phase 2 checkboxes in
`ci-cd.md` are flipped — with no build-failing CRAP behavior introduced.
