# Next Work Package

## Step: Add informational `cargo-semver-checks` API backward-compat CI gate

## Goal

Stand up the `cargo-semver-checks` public-API backward-compatibility gate for the `iscc-lib` core
crate (issue: *Add `cargo-semver-checks` API backward-compat CI gate*). This is the smallest,
most self-contained of the four v1.0.0 hardening gates and locks the now-narrowed Tier 1 / Tier 2
surface under enforcement before the v1.0.0 stability commitment. It runs **informational** during
the 0.4.0 → 1.0.0 transition and becomes enforcing once v1.0.0 ships.

## Scope

- **Modify**: `.github/workflows/ci.yml` — add a new `semver` job (non-blocking).
- **Modify**: `mise.toml` — add a `semver` task mirroring the CI invocation for local runs.
- **Modify (doc, excluded from 3-file limit)**: `.claude/context/specs/ci-cd.md` — flip the Semver
    verification checkbox (currently line ~417) to `[x]`, noting it is informational pre-1.0.
- **Reference**: `.claude/context/specs/ci-cd.md` → "API Stability and Performance Gates" and the
    "CRAP" job style; `.claude/context/specs/rust-core.md` → "API Stability & Performance
    Invariants"; existing `c-ffi` / `wasm` jobs in `ci.yml` for the toolchain + caching action
    pattern (`dtolnay/rust-toolchain@stable`, `Swatinem/rust-cache@v2`); `crates/iscc-lib/Cargo.toml`
    (crate name `iscc-lib`, `default = ["meta-code"]`).

## Not In Scope

- **Do NOT make the gate enforcing / fail the build on breaking changes.** It MUST stay
    informational until v1.0.0 — see Implementation Notes for why CI would otherwise go red.
- Do NOT implement the `iai-callgrind` perf gate or the `cargo llvm-cov` + CRAP coverage gate —
    those are separate issues / future steps.
- Do NOT start the PyO3 0.23 → 0.29 migration.
- Do NOT bump the workspace version or cut v1.0.0.
- Do NOT add `cargo-semver-checks` to the `prek` pre-push hooks — like the coverage gate, it is a
    CI / on-demand tool, not a local push gate.
- Do NOT re-widen any `pub(crate) mod` back to `pub mod` to "satisfy" the check — the narrowing is
    intentional pre-1.0 API cleanup.

## Implementation Notes

- **Baseline = last published release.** `iscc-lib 0.4.0` is published on crates.io;
    `cargo-semver-checks` auto-detects the most recent crates.io release as the baseline. No
    committed baseline file is needed (unlike iai-callgrind/CRAP).
- **Recommended CI implementation** — use the maintainer-blessed action, which installs via
    `binstall` internally (spec-aligned) and handles baseline + rustdoc + caching:
    ```yaml
      semver:
        name: Semver (cargo-semver-checks)
        runs-on: ubuntu-latest
        # Informational during the 0.4.0 -> 1.0.0 transition; enforcing from v1.0.0.
        continue-on-error: true
        steps:
          - uses: actions/checkout@v4
          - uses: dtolnay/rust-toolchain@stable
          - uses: Swatinem/rust-cache@v2
          - name: Check semver
            uses: obi1kenobi/cargo-semver-checks-action@v2
            with:
              package: iscc-lib
    ```
    A manual fallback (`cargo install cargo-semver-checks --locked` then
    `cargo semver-checks check-release -p iscc-lib`) is equivalent if the action is undesirable.
- **`continue-on-error: true` is load-bearing.** After 0.4.0 shipped, the working tree narrowed
    `cdc / conformance / dct / minhash / simhash / utils / wtahash` from `pub mod` to
    `pub(crate) mod` (removed public module *paths*). `cargo-semver-checks` WILL report these as
    breaking changes at the same `0.4.0` version number. That is the intended pre-1.0 freedom — the
    job must report without failing the workflow so CI stays green. This matches the spec: "During
    the 0.4.0 → 1.0.0 transition the check is informational."
- **Features**: default invocation is correct. `iscc-lib` has `default = ["meta-code"]` (implies
    `text-processing`), so the default-feature scan covers the feature-gated Tier 1 symbols. No
    custom feature flags needed.
- **mise task** mirrors the CI command for local use:
    ```toml
    [tasks.semver]
    description = "Check iscc-lib public API for SemVer violations vs the last published release"
    run = "cargo semver-checks check-release -p iscc-lib"
    ```
    Place it under a new "--- API stability ---" comment section near the version tasks.
- Run `mise run format` before committing so the YAML/TOML/markdown auto-fix hooks don't reflow the
    edits during commit.

## Verification

- `mise run check` passes — the pre-commit hooks validate `ci.yml` (YAML) and `mise.toml` (TOML)
    syntax; both edited files parse cleanly.
- `grep -q "cargo-semver-checks" .github/workflows/ci.yml` and the new `semver` job block contains
    `continue-on-error: true` (informational).
- `mise tasks 2>/dev/null | grep -q '^semver'` (or `grep -q '\[tasks.semver\]' mise.toml`) — the
    local task exists.
- The Semver checkbox in `.claude/context/specs/ci-cd.md` is flipped to `[x]`.
- **Best-effort local run** (cargo registry network is available in this environment —
    `cargo search` works): `cargo install cargo-semver-checks --locked` succeeds and
    `mise run semver` runs to completion emitting a SemVer report. A **non-zero exit** caused by the
    expected post-0.4.0 `pub(crate)` module narrowing is acceptable and confirms the gate is wired
    correctly — do not try to make it pass by changing the API. If the install proves impractical,
    defer this single criterion to the CI run.
- On the next push to `develop`: a new `Semver (cargo-semver-checks)` job appears and completes, and
    the existing 16 CI jobs remain green (the new job is non-blocking via `continue-on-error`).

## Done When

`ci.yml` has an informational (`continue-on-error: true`) `Semver` job running `cargo-semver-checks`
for `iscc-lib` against the last published crates.io release, a mirrored `mise run semver` task
exists, the `ci-cd.md` Semver checkbox is checked, and CI stays green with the new non-blocking job
present.
