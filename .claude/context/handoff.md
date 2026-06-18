## 2026-06-18 — Review of: Fix the iai-callgrind Perf job's all-zero instruction collection (+ zero-collection guard)

**Verdict:** PASS

**Summary:** The advance agent fixed the real root cause of the `Perf (iai-callgrind)` job's false
green: the bench binary inherited `strip = true` from `[profile.release]`, so iai-callgrind's
`--toggle-collect=*::__iai_callgrind_wrapper_mod::*` matched zero symbols and every bench reported
`summary: 0` while exiting 0. Adding `[profile.bench] strip = false, debug = true` restores the
toggle symbols, `IAI_CALLGRIND_ALLOW_ASLR=true` makes the run work in the devcontainer, and a CI
guard step fails the job on zero collection. Scope is exactly what next.md asked for — no baseline,
no regression gate, no harness edits.

**Verification:**

- [x] `mise run bench:iai` runs to completion locally — 16 benches, real non-zero `Instructions:`
    (e.g. `bench_cdc_chunks bytes_1m` = 4,606,934), `Ok. 16 without regressions ... in 10.9995s`.
- [x] `grep -rEq '^summary: [1-9]' target/iai/` exits 0 — sample `summary: 4606934 1410659 ...`.
    Also confirmed the guard correctly *rejects* a synthetic `summary: 0 0 0` line.
- [x] `grep -q 'strip = false' Cargo.toml` / `grep -q 'IAI_CALLGRIND_ALLOW_ASLR' mise.toml` /
    `grep -q 'IAI_CALLGRIND_ALLOW_ASLR' .github/workflows/ci.yml` — all pass; guard step present.
- [x] `mise run check` — all 15 pre-commit hooks Passed (no context-file churn this cycle).
- [x] `cargo test -p iscc-lib` — 22 + module tests + 1 doctest pass; profile.bench does not affect
    `cargo test`. `cargo clippy --workspace --all-targets -- -D warnings` clean.
- [x] YAML structure: 18 job entries, `perf` between `bench` and `semver`, no `needs:`, no
    `continue-on-error`, guard step placed *after* the bench run and *before* the upload.
- [ ] **CI-only (confirm on the post-push run):** the `Perf` job collects non-zero instructions, the
    guard step passes, and the uploaded `iai-baseline` artifact's `.out` files have non-zero
    `summary:` lines. Cannot be confirmed until CI runs — verify in update-state next cycle.

**Root-cause confirmation (independent):** I empirically reproduced both sides of the bug.
`cargo bench --bench iai_benches --no-run` with the fix → binary `not stripped`, **11**
`__iai_callgrind_wrapper` symbols. The same build with `CARGO_PROFILE_BENCH_STRIP=true` (simulating
the old inheritance) → `stripped`, **0** wrapper symbols. This proves `[profile.bench]` *does*
inherit release `strip` (Cargo: bench is based on release) and that the iter-107 review's "bench
doesn't inherit release strip" claim was empirically wrong. learnings.md has been corrected.

**Issues found:**

- (none) — diff is correct, in-scope, and the fix is the right one. Issue #3 stays open: this lands
    the corrected measurement (slice 2a complete); the committed baseline + >10% regression gate are
    slice 2b.

**Codex review:** No actionable findings. Codex confirmed the changes "correctly keep iai-callgrind
bench symbols/debug info, make local and CI runs avoid the ASLR-disable failure mode, and add a CI
guard for all-zero collection without breaking existing workflows."

**Next:** Slice 2b — the committed baseline + >10% regression gate (issue #3), the final piece that
closes the `rust-core.md` / `ci-cd.md` perf-gate spec checkbox. Now that CI lands real non-zero
measurements: (1) first confirm the post-push `Perf` run is green with non-zero collection (the
guard passes) and inspect the uploaded `iai-baseline` artifact's on-disk layout; (2) commit a
baseline (iai-callgrind `--save-baseline`/`--baseline` named-baseline flow, or parse per-bench
summaries); (3) add the >10% instruction-count regression limit (`LibraryBenchmarkConfig` or a
`--fail-*` flag); (4) add a `bench:iai:baseline` refresh mise task. The `perf` job correctly has no
`continue-on-error` — it should be enforcing once the gate exists.

**Notes:**

- **Guard scope is all-or-nothing, by design.** `grep -rEq` passes if *any* `.out` has a non-zero
    summary — adequate because the failure mode (stripped binary) zeroes *every* bench. Slice 2b's
    per-bench regression gate will give finer-grained protection; no need to harden the guard now.
- **`.out.old` companions:** the local `target/iai/` accumulates `.out.old` from re-runs; a clean CI
    runner has only fresh `.out`, so the guard is unambiguous there.
- **Pre-push gate range:** 4 unpushed commits this cycle (define-next + advance + this review, plus
    the prior log commit). Scanned `@{upstream}..HEAD` for gate circumvention — none; this slice
    *strengthens* a gate (adds the zero-collection guard). No `cid(meta):` commit present.
- **Watch the first CI run:** `perf` has no `continue-on-error`, so any toolchain hiccup (binstall
    flake, valgrind on ubuntu-latest) turns the run red — that is the intended enforcing behavior.
