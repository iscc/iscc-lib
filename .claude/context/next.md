# Next Work Package

## Step: Fix the iai-callgrind Perf job's all-zero instruction collection (+ add a zero-collection guard)

## Goal

The `Perf (iai-callgrind)` CI job is a **false green**: it runs, exits 0, and uploads a baseline
artifact in which **every benchmark collected ZERO instructions**. Fix the root cause so the job
measures real instruction counts, and add a guard step so a zero-collection result can never
silently pass again. This is a hard prerequisite for the perf-regression gate (issue #3, slice 2b) —
you cannot gate a >10% regression against an all-zero baseline.

## Scope

- **Modify**:
    - `Cargo.toml` (workspace root) — add a `[profile.bench]` section so the benchmark binary is not
        stripped.
    - `mise.toml` — make the `bench:iai` task runnable inside the devcontainer.
    - `.github/workflows/ci.yml` — keep the Perf job measuring correctly and add a guard step that
        fails the job when collection is zero.
- **Reference**:
    - `crates/iscc-lib/benches/iai_benches.rs` (the harness — **do not edit**, it is correct)
    - `.claude/context/handoff.md` (the iter-107 review that PASSed this job — it checked the wrong
        binary and missed the zero collection)
    - `.claude/context/learnings.md` "CI/CD" → the `Perf (iai-callgrind)` entry (now partly wrong —
        see below)

## Not In Scope

- **Slice 2b — the committed baseline + >10% regression gate.** That is the NEXT step and only
    becomes possible once this fix lands real non-zero measurements. Do not commit a baseline file
    or add `--fail-*`/regression config in this step.
- **Editing the harness `iai_benches.rs`.** The harness is sound (it collects 118k–7.25M
    instructions once the binary keeps its symbols). The bug is the build profile + run environment,
    not the benches.
- The two `[review]` issues (CRAP `--fail-above 30`, supply-chain `cargo deny`/`audit`) — both have
    HUMAN REVIEW REQUESTED on a spec amendment; do not auto-scope them.
- Flipping the `Semver` gate's `continue-on-error` — that is tied to the v1.0.0 cut.

## Implementation Notes

**Root cause (diagnosed and reproduced locally this iteration — valgrind 3.19 and
`iai-callgrind-runner` 0.16.1 ARE both installed in the devcontainer):**

1. iai-callgrind launches callgrind with `--toggle-collect=*::__iai_callgrind_wrapper_mod::*` and
    collection OFF at start; it only turns collection ON when callgrind enters a function matching
    that symbol pattern.
2. The binary `cargo bench` actually runs (`target/release/deps/iai_benches-<hash>`) is **stripped**
    — the `bench` profile inherits `strip = true` from `[profile.release]` (root `Cargo.toml`).
    `nm` on it finds **0** `__iai_callgrind_wrapper_mod` symbols, so the toggle matches nothing and
    every bench reports `Collected : 0` / `summary: 0` / `totals: 0`. (Confirmed by downloading the
    CI `iai-baseline` artifact from run 27742285656: all 16 `.out` files show `summary: 0`.)
3. (The iter-107 review inspected a *different*, unstripped 3 MB binary built by a separate
    invocation and concluded the symbols were present — they are not in the binary CI runs.)

**Proven fix** — verified locally end to end (`Instructions: 118514 … 7250645` for all 16 bench
cases) via
`CARGO_PROFILE_BENCH_STRIP=false CARGO_PROFILE_BENCH_DEBUG=true IAI_CALLGRIND_ALLOW_ASLR=true cargo bench -p iscc-lib --bench iai_benches`:

1. **`Cargo.toml`** — add (there is currently no `[profile.bench]`):
    ```toml
    [profile.bench]
    strip = false
    debug = true
    ```
    `strip = false` keeps the `__iai_callgrind_wrapper_mod` symbols in `.symtab` so the toggle
    matches; `debug = true` gives callgrind line info for `--dump-line=yes`. This is the fix that
    makes CI collect real counts (GitHub runners can run iai-callgrind's `setarch -R` ASLR step
    fine).
2. **`mise.toml`** — the devcontainer kernel blocks the `personality` syscall that iai-callgrind's
    `setarch -R` (ASLR-disable) needs, so a bare `cargo bench` of the iai harness dies with
    `setarch: failed to set personality … Operation not permitted`. Setting
    `IAI_CALLGRIND_ALLOW_ASLR=true` makes iai-callgrind skip `setarch` and run with ASLR enabled.
    ASLR does **not** affect instruction counts (`Ir`), only cache-sim noise, so this is safe for
    an `Ir`-based gate. Update the `bench:iai` task (mise.toml:126-128) to run with this env, e.g.
    add `env = { IAI_CALLGRIND_ALLOW_ASLR = "true" }` to the task table. This makes
    `mise run bench:iai` runnable locally for verification.
3. **`ci.yml`** — in the `perf` job (ci.yml:280-307):
    - Set `IAI_CALLGRIND_ALLOW_ASLR: "true"` on the "Run iai-callgrind benches" step (step-level
        `env`) so CI and local behave identically and the eventual committed baseline is portable.
        (CI's `setarch` works, but pinning ASLR-on everywhere keeps `Ir` baselines reproducible.)
    - Add a **guard step** after the bench run, before the upload, that fails if collection is zero:
        ```yaml
          - name: Assert non-zero instruction collection
            run: |
              if grep -rEq '^summary: [1-9]' target/iai/; then
                echo "iai-callgrind collected non-zero instructions"
              else
                echo "::error::iai-callgrind collected ZERO instructions for all benches"
                exit 1
              fi
        ```
        On a clean CI runner `target/iai/` contains only the fresh run, so this passes only when real
        counts were collected. Keep the `Upload iai-callgrind results` step (the artifact is now
        useful for slice 2b).

**Note for the review agent:** update the `learnings.md` `Perf (iai-callgrind)` entry — the claim
"`[profile.bench]` does NOT inherit release `strip`" is **false** (the bench binary IS stripped),
and "valgrind absent in the devcontainer" is **false** (valgrind 3.19 + runner 0.16.1 are present;
only `setarch -R` is blocked).

## Verification

- `mise run bench:iai` runs locally to completion (no `setarch` crash) and prints a non-zero
    `Instructions:` line for all 16 bench cases (ends with
    `Iai-Callgrind result: Ok. 16 …   benchmarks finished`).
- After that run, `grep -rEq '^summary: [1-9]' target/iai/` exits 0 (non-zero collection present);
    the guard logic is correct.
- `grep -q 'strip = false' Cargo.toml`, `grep -q 'IAI_CALLGRIND_ALLOW_ASLR' mise.toml`, and
    `grep -q 'IAI_CALLGRIND_ALLOW_ASLR' .github/workflows/ci.yml` all succeed.
- `mise run check` passes (pre-commit hooks validate the edited YAML/TOML syntax + formatting).
- `cargo test -p iscc-lib` still passes (profile.bench changes do not affect `cargo test`).
- **CI-only (review confirms post-push):** the `Perf (iai-callgrind)` job collects non-zero
    instructions, the new guard step passes, and the uploaded `iai-baseline` artifact's `.out` files
    have non-zero `summary:` lines.

## Done When

`mise run bench:iai` collects non-zero instruction counts locally, the CI Perf job's guard step
fails on zero-collection (and passes on the now-real measurements), and the uploaded `iai-baseline`
artifact contains real instruction counts — unblocking the slice-2b regression gate.
