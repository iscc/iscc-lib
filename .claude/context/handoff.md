## 2026-06-17 — Add informational `cargo-semver-checks` API backward-compat CI gate

**Done:** Added a non-blocking `Semver (cargo-semver-checks)` job to `ci.yml` (via the
maintainer-blessed `obi1kenobi/cargo-semver-checks-action@v2`, `continue-on-error: true`) that
checks the `iscc-lib` public API against the last published crates.io release, plus a mirrored
`mise run semver` task for local runs. Flipped the Semver verification checkbox in `ci-cd.md`. The
gate stays informational during the 0.4.0 → 1.0.0 transition and becomes enforcing once v1.0.0
ships.

**Files changed:**

- `.github/workflows/ci.yml`: new `semver` job after `bench` — `continue-on-error: true`,
    `dtolnay/rust-toolchain@stable` + `Swatinem/rust-cache@v2`, `cargo-semver-checks-action@v2` with
    `package: iscc-lib`.
- `mise.toml`: new `[tasks.semver]` under a "--- API stability ---" section running
    `cargo semver-checks check-release -p iscc-lib`.
- `.claude/context/specs/ci-cd.md`: Semver verification checkbox flipped to `[x]`, reworded to note
    it is informational pre-1.0 via `continue-on-error`.

**Verification:**

- `grep -c "cargo-semver-checks" .github/workflows/ci.yml` → 2; the `semver` block contains
    `continue-on-error: true`.
- `mise tasks | grep '^semver'` → present; `[tasks.semver]` in `mise.toml`.
- prek hooks run on the three changed files → all Passed (mdformat, check yaml, check toml, etc.).
- Best-effort local run: installed `cargo-semver-checks v0.48.0`; `mise run semver` ran to
    completion. It reports **2 major checks failed** — exactly the expected post-0.4.0 `pub(crate)`
    narrowing of `cdc/conformance/minhash/simhash/utils` (module_missing) plus the associated
    `function_missing` for their previously-public functions. Per next.md this non-zero exit is the
    correct signal that the gate is wired; `continue-on-error` keeps CI green.

**Next:** Two larger v1.0.0 hardening gates remain from the same spec: the `iai-callgrind`
instruction-count perf gate (needs a committed baseline) and the `cargo llvm-cov` + `cargo crap`
coverage/CRAP gate (multi-phase). Also still open: the PyO3 0.23 → 0.29 security migration (scoped
to `crates/iscc-py/`, should be split per-minor). The semver gate becoming *enforcing* is a one-line
follow-up (drop `continue-on-error`) to schedule with the v1.0.0 cut.

**Notes:**

- **`mise run check` is not fully green in this working tree, but NOT due to my changes.** The
    `mdformat` hook reformats two pre-existing, out-of-scope context files —
    `.claude/context/next.md` and `.claude/agent-memory/define-next/MEMORY*.md` — that were
    committed by the define-next agent in a non-mdformat-conforming state. My three files pass all
    hooks when run in isolation (`prek run --files ...`). I restored those out-of-scope files rather
    than commit reformatting I'm not allowed to touch. The review agent may want to file this as a
    separate hygiene issue (run mdformat over the context/agent-memory markdown).
- The CI job uses the wrapper action (installs `cargo-semver-checks` via binstall internally) per
    the spec's recommendation rather than a manual `cargo install`. Confirmed on the next push the
    job appears and is non-blocking.
- No source, API, conformance, or performance surface was touched — CI config + task + doc only.
