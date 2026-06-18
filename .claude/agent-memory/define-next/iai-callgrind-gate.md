---
name: iai-callgrind-gate
description: iai-callgrind perf gate (issue #3) — slice status, the zero-collection bug, and how to verify locally
metadata:
  type: project
---

The v1.0.0 iai-callgrind perf-regression gate (issue #3, `[human]`, already specced: target.md L71,
rust-core.md, ci-cd.md "Performance"). Built in slices.

**Slice 1 (DONE iter 106):** harness `crates/iscc-lib/benches/iai_benches.rs` — 11
`#[library_benchmark]` (9 `gen_*_v0` hot paths + `alg_cdc_chunks` + `alg_minhash_256`, 16 bench
cases), iai-callgrind "0.16" → 0.16.1, `[[bench]] harness=false`.

**Slice 2a (landed iter 107, but BROKEN):** `Perf (iai-callgrind)` CI job (ci.yml:280-307) +
`bench:iai` mise task. The job is a **FALSE GREEN** — it ran, exited 0, uploaded an `iai-baseline`
artifact in which **every bench collected ZERO instructions** (`summary: 0`/`Collected : 0`). The
iter-107 review PASSed it after inspecting the WRONG (unstripped, separately-built) binary.

**Slice 2a-fix (SCOPED iter 108):** fix zero-collection + add a guard. Root cause + proven fix:

- callgrind toggles collection on `--toggle-collect=*::__iai_callgrind_wrapper_mod::*`; the
    `cargo bench` binary is **stripped** (`bench` profile DOES inherit
    `[profile.release] strip = true` from root Cargo.toml — the review's "bench doesn't inherit
    strip" claim is FALSE), so the symbol is gone → 0 collected.
- **Fix:** root `Cargo.toml` `[profile.bench] strip = false` + `debug = true` (proven: all 16
    benches then collect 118k–7.25M `Ir`). Plus `IAI_CALLGRIND_ALLOW_ASLR=true` so iai-callgrind
    skips its `setarch -R` ASLR step (the devcontainer blocks the `personality` syscall; ASLR
    doesn't affect `Ir`). Set ALLOW_ASLR on the `bench:iai` mise task + the CI bench step; add a
    guard step `grep -rEq '^summary: [1-9]' target/iai/ || exit 1`.

**Slice 2b (FUTURE, the actual gate):** committed baseline + `--fail-*`/regression limit + a
`bench:iai:baseline` refresh task + spec checkbox flip. iai-callgrind baselines live in
`target/iai/` as a tree of raw `callgrind.*.out` files (NO single-file baseline like cargo-crap; NO
`*.summary.json` unless `--save-summary` is passed). The committed-baseline glue must be designed
from a real non-zero artifact (now obtainable once 2a-fix lands). Named-baseline flow =
`-- --save-baseline=NAME` / `-- --baseline=NAME`; regression = `IAI_CALLGRIND_REGRESSION='ir=10'`
env or `LibraryBenchmarkConfig`.

**KEY local-verifiability correction:** valgrind 3.19 AND `iai-callgrind-runner` 0.16.1 ARE
installed in the devcontainer (state.md/learnings/handoff all wrongly say "valgrind absent
locally"). The full iai flow runs locally IF `IAI_CALLGRIND_ALLOW_ASLR=true` (else `setarch` fails:
`failed to set personality … Operation not permitted`). So slice 2b IS locally verifiable too — do
not defer it as "CI-only".

**Tooling re-discovery:** `gh run download <run-id> -n <artifact> --dir <dir>` works in-devcontainer
— always download the real CI artifact to verify "green = working" before trusting an infra job.
