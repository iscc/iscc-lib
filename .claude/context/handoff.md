## 2026-06-18 — Fix the iai-callgrind Perf job's all-zero instruction collection (+ add a zero-collection guard)

**Done:** Fixed the root cause of the `Perf (iai-callgrind)` CI job's false green — the bench binary
was stripped (inheriting `strip = true` from `[profile.release]`), so iai-callgrind's
`--toggle-collect=*::__iai_callgrind_wrapper_mod::*` matched no symbols and every bench reported
zero collected instructions. Added a `[profile.bench]` section (`strip = false`, `debug = true`) to
keep the toggle symbols, made `bench:iai` runnable in the devcontainer via
`IAI_CALLGRIND_ALLOW_ASLR`, and added a CI guard step that fails the job if collection is zero.

**Files changed:**

- `Cargo.toml`: added `[profile.bench]` with `strip = false` + `debug = true` (with an explanatory
    comment). This keeps `__iai_callgrind_wrapper`/`bench_*` symbols in `.symtab` so callgrind's
    toggle matches and counters are real.
- `mise.toml`: added `env = { IAI_CALLGRIND_ALLOW_ASLR = "true" }` to the `bench:iai` task so the
    devcontainer kernel's blocked `personality` syscall (used by iai-callgrind's `setarch -R`) does
    not crash the run. ASLR does not affect `Ir` counts, only cache-sim noise.
- `.github/workflows/ci.yml` (`perf` job): set `IAI_CALLGRIND_ALLOW_ASLR: 'true'` on the bench step
    (pins ASLR-on everywhere for reproducible baselines) and added an
    `Assert non-zero instruction   collection` guard step
    (`grep -rEq '^summary: [1-9]' target/iai/`) before the upload. yamlfix auto-normalized `"true"`
    → `'true'`; shell block scalar preserved intact.

**Verification:**

- `mise run bench:iai` runs to completion locally with NO `setarch` crash and prints non-zero
    `Instructions:` for all 16 bench cases
    (`Iai-Callgrind result: Ok. 16 ... 16 benchmarks finished   in 11.6210s`). Sample:
    `bench_cdc_chunks bytes_1m` = 4,606,934 instructions.
- `grep -rEq '^summary: [1-9]' target/iai/` exits 0 — confirmed against real `.out` files (sample
    summary line: `summary: 4606934 1410659 ...`). Guard logic is correct.
- `grep -q 'strip = false' Cargo.toml`, `grep -q 'IAI_CALLGRIND_ALLOW_ASLR' mise.toml`,
    `grep -q 'IAI_CALLGRIND_ALLOW_ASLR' .github/workflows/ci.yml` — all succeed.
- `mise run check` — all 15 pre-commit hooks pass (yamlfix auto-fixed ci.yml on first run, clean on
    re-run).
- `cargo test -p iscc-lib` — 22 + all module tests + 1 doctest pass; profile.bench changes do not
    affect `cargo test`.
- **CI-only (confirm post-push):** the `Perf` job collects non-zero instructions, the guard step
    passes, and the uploaded `iai-baseline` artifact's `.out` files have non-zero `summary:` lines.

**Next:** Slice 2b — the committed baseline + >10% regression gate (issue #3). Now that CI lands
real non-zero measurements: (1) inspect the uploaded `iai-baseline` artifact's on-disk layout; (2)
commit a baseline (`--save-baseline`/`--baseline` named-baseline flow, or parse per-bench
summaries); (3) add the >10% instruction-count regression limit (`LibraryBenchmarkConfig` or
`--fail-*` flag); (4) add a `bench:iai:baseline` refresh mise task. The `perf` job correctly has no
`continue-on-error` — it should be enforcing once the gate exists.

**Notes:**

- **The iter-107 review's `strip` claim was empirically WRONG.** It inspected a different,
    unstripped binary built by a separate invocation; the binary CI actually runs IS stripped (bench
    inherits `strip = true` from release). The learnings.md `Perf (iai-callgrind)` entry needs
    updating: the claim "`[profile.bench]` does NOT inherit root `[profile.release] strip = true`"
    is FALSE, and "valgrind absent in the devcontainer" is FALSE (valgrind 3.19 +
    iai-callgrind-runner 0.16.1 are both present; only `setarch -R` / the `personality` syscall is
    blocked).
- The local `target/iai/` now contains `.out.old` files from the re-run; on a clean CI runner only
    fresh `.out` files exist, so the guard grep is unambiguous there. The guard intentionally
    matches `^summary: [1-9]` (any single non-zero leading digit of the `Ir` field) — robust against
    the `.old` companions too (they also have non-zero summaries here).
- No hot path touched (this is build-profile + CI plumbing only); no benchmark numbers to compare.
- Did NOT touch the harness `iai_benches.rs` (correct, out of scope), did not commit a baseline or
    add a regression gate (slice 2b), and did not touch the two `[review]` issues or the `Semver`
    gate.
