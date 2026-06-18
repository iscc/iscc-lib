## 2026-06-18 — Review of: Add the `Perf` CI job that runs the iai-callgrind benches under valgrind

**Verdict:** PASS_WITH_NOTES

**Summary:** The advance agent added a clean, correctly-shaped `Perf (iai-callgrind)` CI job to
`ci.yml` (apt valgrind → cargo-binstall → `cargo binstall -y --force iai-callgrind-runner@0.16.1` →
`cargo bench -p iscc-lib --bench iai_benches` → upload `target/iai/` as `iai-baseline`) plus a
`bench:iai` mise task. Scope is exactly what next.md asked for — no baseline, no regression gate, no
harness edits, no out-of-scope touches. All local checks pass; the only deferred criterion is the
CI-only "job concludes success + uploads usable artifact", which can only be confirmed on the
post-push run.

**Verification:**

- [x] `mise run check` — all 15 pre-commit hooks Passed (YAML + TOML validation + yamlfix/taplo
    formatting clean).
- [x] `grep -q "Perf (iai-callgrind)" ci.yml` — present.
- [x] `grep -q "iai-callgrind-runner@0.16.1" ci.yml` — present (matches the `iai-callgrind = "0.16"`
    workspace dep, resolves 0.16.1).
- [x] `grep -q "binstall -y --force"` + `grep -q "valgrind"` — both present; `--force` rationale +
    "first-run measure-only, exit 0" documented as inline comments.
- [x] `grep -q -- "--bench iai_benches"` — present (scopes to the iai harness, not criterion).
- [x] `mise tasks ls | grep -q "bench:iai"` — task registered.
- [x] YAML structure validated (`yaml.safe_load`): 18 job entries (was 17), `perf` placed between
    `bench` and `semver`, `runs-on: ubuntu-latest`, no `needs:`, no `continue-on-error`, 8 steps in
    the next.md-specified order. `cargo build -p iscc-lib --bench iai_benches` → exit 0.
- [ ] **CI-only (deferred to post-push run):** `Perf` job concludes `success`, runs benches under
    valgrind, uploads a *usable* `iai-baseline`, no existing job regresses. Cannot run valgrind in
    the devcontainer. Strong evidence it will pass (see Codex review below) — but confirm on the
    actual CI run.

**Issues found:**

- (none) — the diff is correct and in-scope. Issue #3 ("Add `iai-callgrind` performance-regression
    CI gate") stays open: this slice (2a) lands the runner; the committed baseline + >10% regression
    gate are slice 2b.

**Codex review:** One P2 finding — **investigated and empirically refuted.** Codex argued the bench
binary would be stripped (the root `[profile.release] strip = true`), so iai-callgrind's
`--toggle-collect=*::__iai_callgrind_wrapper_mod::*` would match nothing and the uploaded baseline
would be all-zero/unusable. I verified the opposite on this exact configuration:

- `cargo bench -p iscc-lib --bench iai_benches --no-run` produces
    `target/release/deps/iai_benches-*` that `file` reports as **"with debug_info, not stripped"** —
    1353 symbols incl. **11 `__iai_callgrind_wrapper`** and **28 `bench_*`** symbols (via `nm`).
- A `--release` artifact (`libiscc_ffi.so`) by contrast IS `stripped`, confirming `strip = true`
    works — but `[profile.bench]` does **not** inherit the custom `strip` from `[profile.release]`
    (despite bench inheriting other release defaults). No `.cargo/config.toml` or `RUSTFLAGS`/
    `CARGO_PROFILE_*` overrides exist, so CI resolves the same profile as local.

Conclusion: iai-callgrind will see real symbols and produce non-zero counters; no `strip` override
(`CARGO_PROFILE_BENCH_STRIP=false`) is needed. Recorded in learnings so slice 2b doesn't re-litigate
it. (If a future change adds `[profile.bench] inherits = "release"` or `strip`, this protection
disappears — re-check then.)

**Next:** Slice 2b — the regression gate. After this `Perf` run lands a known-good `target/iai/` on
CI: (1) inspect the uploaded `iai-baseline` artifact to learn the exact on-disk layout / summary
format; (2) decide between iai-callgrind's `--save-baseline`/`--baseline` named-baseline flow vs.
parsing per-bench `*.summary.json`; (3) commit the baseline file; (4) add the >10% regression limit
(`LibraryBenchmarkConfig` or a `--fail-*` CLI flag) so the job fails on a >10% instruction-count
regression; (5) add a `bench:iai:baseline` refresh mise task. Only slice 2b completes the
`rust-core.md` / `ci-cd.md` perf-gate spec checkbox and lets issue #3 close. The `Perf` job has no
`continue-on-error` (correct — it should be enforcing once the gate exists).

**Notes:**

- **`mise run format` context-file churn is expected, not a regression.** The mdformat hook
    reformats non-conforming context files (`learnings.md`, `next.md`, etc.); the advance agent
    reverted `learnings*.md`, so its commit touched only `ci.yml` + `mise.toml` + handoff +
    advance-memory. This review re-touched `learnings.md` deliberately (mdformat-conforming).
- **Push range:** 10 unpushed commits (`@{upstream}..HEAD`, 0 behind). Scanned the full range for
    gate circumvention — none (this slice *adds* a gate). No `cid(meta):` commit present.
- **Watch the first CI run:** because `perf` has no `continue-on-error`, any toolchain hiccup
    (binstall flake, valgrind incompat on ubuntu-latest) turns the run red. The `--force` binstall
    guards the known rust-cache poisoning failure mode.
