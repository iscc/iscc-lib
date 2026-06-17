## 2026-06-17 — Review of: Fix the CI `Coverage + CRAP` cargo-crap install flake

**Verdict:** PASS

**Summary:** The advance agent added `--force` to the `Install cargo-crap` step
(`cargo binstall -y --force cargo-crap@0.2.2`, ci.yml:314) so the binary is reinstalled on every run
regardless of the `.crates.toml` metadata that `Swatinem/rust-cache@v2` restores without the actual
binary. Clean, minimal, single-line change that exactly matches next.md — and it *strengthens* the
CRAP quality gate's reliability rather than weakening it. All local verification is green.

**Verification:**

- [x] `grep -n 'cargo binstall -y --force cargo-crap@0.2.2' .github/workflows/ci.yml` — matches
    exactly one line (314)
- [x] Workflow YAML still parses — `yaml.safe_load(...)` → `YAML OK`
- [x] `git diff --stat` isolates the change — `ci.yml` is the only non-context file, 1-line change
    (`+`/`-` on the install step only); no other `coverage`-job step altered
- [x] `mise run check` — all 15 pre-commit hooks pass (incl. `check yaml`, `YAML formatting`)
- [ ] Next CI run's `Coverage + CRAP` reaches the report/gate steps and the run goes GREEN — cannot
    run locally; **must be confirmed on the next push's CI run** (see Notes)

**Issues found:**

- (none) — no source code touched, no API break, no perf-path change. Scanned all 4 unpushed commits
    (`git diff @{upstream}..HEAD`) for gate circumvention: none — the only CI/config change is
    adding `--force`, which makes the enforcing CRAP gate actually run (the opposite of
    circumvention). Concurrency clear: `ps aux` shows a single `mise run cid:run` tree (pid 78441),
    no duplicate loop.

**Codex review:** No blocking findings. Codex confirmed the change is "a narrow update to force
cargo-crap installation and should address the cached-metadata/missing-binary failure without
breaking subsequent steps. The other changes are internal documentation/context updates."

**Next:** With the CI fix in place, the prerequisite (green CI) is met *pending the push's CI run*.
Resume the deferred PyO3 incremental migration **0.25 → 0.26** (issue #1). Same self-contained
recipe: bump the pin in root `Cargo.toml` → `cargo update -p pyo3` → build/clippy(`-D warnings`)/fmt
→ `uv run maturin develop` → `uv run pytest` (286 tests), scoped to `crates/iscc-py/` (core has no
PyO3 dep). The predicted `IntoPyObject`/lifetime breaks did NOT materialize at 0.24 or 0.25 — treat
skeptically for 0.26 but watch `-D warnings`. RustSec advisories clear only at 0.29, so keep going
one minor per step. Also still open: CRAP `--fail-above 30` hardening ([review] issue, HUMAN REVIEW
REQUESTED before amending the spec) and the `iai-callgrind` perf-regression CI gate.

**Notes:**

- **CI confirmation pending**: this fix's effectiveness can only be proven by the next CI run. After
    this push, the next update-state agent MUST check the `Coverage + CRAP` job conclusion on the
    new run before declaring CI green. If it still fails, the alternative is excluding the binstall
    record from the restored rust-cache (`cache-bin: false` or scoping the cache key) rather than
    `--force`.
- This push sends a clean 4-commit fast-forward (`@{upstream}...HEAD` = `0 4`): the iter-99 log,
    update-state, define-next, and advance. No backlog.
- `--force` re-downloads cargo-crap each run; the binary is small so the cost is negligible and was
    a deliberate, documented trade-off over disabling rust-cache's `cache-bin`.
- No Rust/Python source changed, so pre-push gates (clippy/cargo test/pytest) are not implicated;
    `mise run check` (pre-commit stage) is the relevant gate for a workflow-YAML edit and is clean.
