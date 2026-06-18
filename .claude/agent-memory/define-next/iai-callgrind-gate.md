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

**Slice 2a-fix (LANDED + reviewed PASS, iter 108):** fix zero-collection + guard. CI GREEN on
`1463edb` (run 27746693860) with REAL non-zero counts; slice 2a done. Root cause + fix:

- callgrind toggles collection on `--toggle-collect=*::__iai_callgrind_wrapper_mod::*`; the
    `cargo bench` binary is **stripped** (`bench` profile DOES inherit
    `[profile.release] strip = true` from root Cargo.toml — the review's "bench doesn't inherit
    strip" claim is FALSE), so the symbol is gone → 0 collected.
- **Fix:** root `Cargo.toml` `[profile.bench] strip = false` + `debug = true` (proven: all 16
    benches then collect 118k–7.25M `Ir`). Plus `IAI_CALLGRIND_ALLOW_ASLR=true` so iai-callgrind
    skips its `setarch -R` ASLR step (the devcontainer blocks the `personality` syscall; ASLR
    doesn't affect `Ir`). Set ALLOW_ASLR on the `bench:iai` mise task + the CI bench step; add a
    guard step `grep -rEq '^summary: [1-9]' target/iai/ || exit 1`.

**Slice 2b (SCOPED iter 109 — the actual gate):** committed baseline + >10% Ir regression gate.
**Design chosen (do NOT use iai's native baseline):** a custom committed JSON `.iai-baseline.json`
at repo root (mirrors `.crap-baseline.json`) + a stdlib-only `scripts/iai_regression.py` (update +
check modes). Rationale: iai baselines live in gitignored `target/iai/` as raw `callgrind.*.out`
files carrying machine-specific paths — not portable/reviewable. Key facts confirmed iter 109:

- On-disk: `target/iai/iscc-lib/iai_benches/iscc_benches/<fn>.<id>/callgrind.<fn>.<id>.out`; each
    `.out` has `summary: <Ir> <Dr> <Dw> I1mr D1mr D1mw ILmr DLmr DLmw`. **Gate on Ir (first value)
    ONLY** — deterministic. Cache/miss values are ASLR/cache noisy. 16 unique bench-case keys.
- **Avoid local-vs-CI rustc drift:** local rustc 1.96.0, CI `@stable` — Ir can differ across rustc
    versions. Build the COMMITTED baseline from the CI artifact
    (`gh run download <green-Perf-run>   -n iai-baseline`), NOT the local run, so the committed file
    matches what the CI gate measures. The CI `iai-baseline` artifact (155KB, not expired) IS
    downloadable via authed `gh`.
- **Keep the zero-collection guard** — an all-zero collection reads as a *decrease* and would PASS a
    > 10%-increase regression check, so the guard is still load-bearing.
- Script must be stdlib-only so the CI `perf` job needs no `uv`/Python setup (`python3 ...` works on
    ubuntu-latest); follow `scripts/version_sync.py` for ruff/ty/docstring style. Add
    `bench:iai:baseline` mise task (mirror `crap:baseline`).

**KEY local-verifiability correction:** valgrind 3.19 AND `iai-callgrind-runner` 0.16.1 ARE
installed in the devcontainer (state.md/learnings/handoff all wrongly say "valgrind absent
locally"). The full iai flow runs locally IF `IAI_CALLGRIND_ALLOW_ASLR=true` (else `setarch` fails:
`failed to set personality … Operation not permitted`). So slice 2b IS locally verifiable too — do
not defer it as "CI-only".

**Tooling re-discovery:** `gh run download <run-id> -n <artifact> --dir <dir>` works in-devcontainer
— always download the real CI artifact to verify "green = working" before trusting an infra job.
