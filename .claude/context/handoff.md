## 2026-06-17 — Add Phase 1 Rust coverage CI job (`cargo llvm-cov` LCOV artifact) + `mise run coverage` task

**Done:** Added a standalone `coverage` CI job to `ci.yml` that installs `cargo-llvm-cov` (via
`taiki-e/install-action@v2`) and the `llvm-tools-preview` rustup component, generates an LCOV report
for `iscc-lib`, and uploads it as the `lcov` artifact. Mirrored the exact command as a local
`mise run coverage` task, gitignored `lcov.info`, and flipped the Phase 1 checkbox in `ci-cd.md`.

**Files changed:**

- `.github/workflows/ci.yml`: new `coverage` job (`Coverage (cargo llvm-cov)`, no `needs:`, no
    `continue-on-error`) — checkout → toolchain w/ `llvm-tools-preview` → rust-cache →
    `taiki-e/install-action@v2` (`tool: cargo-llvm-cov`) →
    `cargo llvm-cov -p iscc-lib --lcov   --output-path lcov.info` → `actions/upload-artifact@v4`
    (`name: lcov`). 17 CI jobs total now.
- `mise.toml`: added `[tasks.coverage]` under a new `# --- Coverage ---` section, identical command
    to the CI step (local reproducibility only; CI calls cargo directly).
- `.gitignore`: added `lcov.info` under "Unit test / coverage reports".
- `.claude/context/specs/ci-cd.md` (doc): Phase 1 checkbox (was line 409) flipped to `[x]`, reworded
    to "Coverage job generates and uploads an LCOV report …". mdformat reflowed the line on save.

**Verification:**

- `cargo install cargo-llvm-cov` (v0.8.7) + `rustup component add llvm-tools-preview` succeeded.
- `cargo llvm-cov -p iscc-lib --lcov --output-path lcov.info` → exit 0, all 22 lib tests pass under
    instrumentation, `lcov.info` written (5156 lines, non-empty).
- `mise run coverage` → exit 0, reproduces the same 5156-line `lcov.info`.
- `mise tasks | grep '^coverage'` → present with description.
- `git status --porcelain` → `lcov.info` NOT listed (gitignore confirmed).
- `grep -n "cargo-llvm-cov\|llvm-cov\|coverage:" .github/workflows/ci.yml` → job + install + run
    lines all present (lines 293–307).
- `mise run check` → all hooks Passed (mdformat, YAML, TOML clean on all edited files; no
    out-of-scope context files reflowed this run).
- Deferred to next CI run (review to confirm): the new `Coverage` job appears and the existing 16
    jobs stay green (17 total).

**Next:** Two backlog gates remain unstarted and pair naturally: (1) **Phase 2 report-only
`cargo crap`** — `cargo binstall cargo-crap` (pinned), run vs `lcov.info` with `--format github`
annotations + SARIF upload to Code Scanning, plus `.cargo-crap.toml` and the `mise run crap` task
(non-failing). (2) **`iai-callgrind` perf gate** — instruction-count benches + committed baseline,
fail on >10% regression. Either is a self-contained CI-config step.

**Notes:**

- No source/API/conformance/perf surface touched — CI config + mise task + gitignore + spec doc
    only. No API-break, no hot-path change.
- Followed the `semver` job (iter 93) as the structural template: standalone job, no `needs:`,
    maintainer-blessed installer action. Deliberately did NOT set `continue-on-error` on `coverage`
    per next.md — Phase 1 has no score gate, so its only failure mode (tests failing under
    instrumentation) should be visible. If llvm-cov proves flaky on the shared runner, the
    documented fallback is `continue-on-error: true`.
- `taiki-e/install-action@v2` with `tool: cargo-llvm-cov` is the maintainer-recommended installer
    (fetches a prebuilt binary); the local install used `cargo install` only because that is the
    next.md verification step — CI uses the faster action.
- `iterations.jsonl` is runner-owned and was left unstaged.
