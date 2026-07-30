---
name: ci-gates
description: Full mechanics of the enforcing CI gates — coverage+CRAP, cargo-deny audit, semver, iai perf regression — with configs, regen commands, and gotchas
metadata:
  type: project
---

# CI Gates — Full Mechanics

Detail moved out of MEMORY.md index. Covers the four hardening gates on `.github/workflows/ci.yml`.

## `coverage` job (`Coverage + CRAP`) — enforcing

- Standalone job, NO `continue-on-error`. Steps: toolchain + `llvm-tools-preview` →
    `cargo binstall -y --force {cargo-llvm-cov,cargo-crap@0.2.2}` (`--force` LOAD-BEARING,
    rust-cache gotcha) → `cargo llvm-cov -p iscc-lib --lcov` → upload `lcov` → report-only
    `cargo crap --format github` + `sarif` (`upload-sarif@v3`, job perms `security-events: write`) →
    enforcing LAST step (iter 113):
    `cargo crap --baseline .crap-baseline.json --fail-regression --fail-above`.
- `--fail-regression` blocks a baselined fn's CRAP rising; `--fail-above` blocks ANY fn over
    `threshold=30.0` incl new fns.
- `.crap-baseline.json` (repo root, COMMITTED, NOT gitignored): `{$schema, version, entries}`, 107
    fns (iter 176). Regen via `mise run crap:baseline`
    (`cargo crap ... --format json --output .crap-baseline.json`, NO `--sort` in 0.2.2).
- GOTCHA: never pipe `cargo crap` into `tail`/`head` to check exit — `$?` = pager, masks exit 1;
    redirect to a file first. (Same gotcha applies to `cargo deny`.)
- GOTCHA (iter 122): cargo-llvm-cov/cargo-crap are NOT preinstalled in the CID session env. The
    binstall'd cargo-crap 0.2.2 binary needs `GLIBC_2.39` (container glibc is older) — install it
    from source: `cargo install cargo-crap --version 0.2.2` (~1.5 min). cargo-llvm-cov 0.8.7 via
    `cargo binstall` works fine. Also `rustup component add llvm-tools-preview` first.
- GUARD GAP: the CRAP regression gate runs ONLY in CI (not in `mise run check`/pre-commit). Any
    change adding a branch/loop to a covered fn MUST refresh `.crap-baseline.json` in the SAME step
    (`mise run crap:baseline`), else the next CI push goes red (iter 121→122 incident).
- STALE-BASELINE gotcha (iter 176): if the refresh lags multiple source-changing iters, a later
    "re-baseline for fn X" step actually bakes in the WHOLE accumulated batch, not just X — a
    define-next hypothesis of "only one fn moved" will be wrong (baseline last touched iter 121,
    then the 172-175 IDv1 batch moved 6+ fns). Prove the deltas are real source changes, NOT
    nondeterministic coverage flap, by running `cargo llvm-cov`+`cargo crap` TWICE and diffing the
    JSON: identical scores = deterministic → safe to bake in. Map each moved fn to a reviewed
    commit.
- `.cargo-crap.toml`: `threshold=30.0`, `missing="pessimistic"`, `exclude` globs MUST list
    `crates/iscc-lib/benches/**` (built-in excludes are repo-root only → harness leaks at CRAP ~42).

## `audit` job (`Audit (cargo-deny)`) — enforcing (iter 114)

- NO `continue-on-error`. `taiki-e/install-action cargo-deny@0.19.9` → `cargo deny check`. Local:
    `mise run audit`. cargo-deny installable in devcontainer: `cargo install cargo-binstall`
    (~5.5min) then `cargo binstall cargo-deny@0.19.9`.
- `deny.toml` (repo root) is config v2: unlisted licenses + vulns + unmaintained DENY by default (NO
    `unlicensed=`/`vulnerability=` keys); `all-features=true`; `yanked="deny"`; `allow=[...13]` incl
    `MPL-2.0`/`BSL-1.0`/`Unicode-3.0`/`Zlib` + `private={ignore=true}` (4 binding crates carry no
    `license`); `multiple-versions="warn"` (6 dup warns, non-failing); sources unknown-registry/git
    `deny`; `ignore` = 2 dev-only `iai-callgrind` unmaintained advisories (RUSTSEC-2025-0141
    bincode, RUSTSEC-2026-0173 proc-macro-error2).
- GOTCHA: `yanked="deny"` forced a Cargo.lock bump of the wasm-bindgen family off yanked
    0.2.111/js-sys 0.3.88 (wasm-bindgen-test pins `=` exact).
- Fresh advisory with a patched release → `cargo update -p <crate>` lockfile bump, NEVER add to
    `ignore` (iter 115: RUSTSEC-2026-0204 crossbeam-epoch 0.9.18→0.9.20, dev-only via
    criterion→rayon; dry-run first to confirm single-package lock).

## `semver` job — informational pre-1.0

- `obi1kenobi/cargo-semver-checks-action@v2`, `package: iscc-lib`, baseline = last crates.io
    release. `continue-on-error: true` — INFORMATIONAL pre-1.0 (post-0.4.0 `pub(crate)` narrowing
    reports as breaking; expected). Drop `continue-on-error` at v1.0.0 (human-held). Local:
    `mise run semver`.

## `perf` job (`Perf (iai-callgrind)`) + regression gate

- iai harness COMPILES without valgrind/runner; running needs them (devcontainer HAS valgrind 3.19
    - runner 0.16.1; local `mise run bench:iai`). CI: apt valgrind → binstall runner (`--force`
        gotcha) → run benches → zero-collection guard → `Check perf regression` → upload
        `iai-baseline` (`if: always()`).
- PERF REGRESSION GATE (iter 109/110, issue #3): `scripts/iai_regression.py` (stdlib only) +
    committed `.iai-baseline.json` (repo root, 16 Ir entries). `--check` fails (exit 1) on any
    shared bench > `baseline*1.10`, any shared bench Ir==0 (false-green guard), or a baselined bench
    missing (unless `--allow-missing`); only-in-run warns only. Tasks
    `bench:iai:baseline`/`bench:iai:check`; tests `tests/test_iai_regression.py`. Committed baseline
    MUST be CI-sourced. Full mechanics → MEMORY-archive.md / learnings-archive.md.
- Bench-config facts (iter 108; full write-up in learnings.md): root `Cargo.toml`
    `[profile.bench] strip = false, debug = true` (else stripped binary → `summary: 0` false green);
    `IAI_CALLGRIND_ALLOW_ASLR=true` (mise + ci.yml) skips iai's kernel-blocked `setarch -R` (ASLR =
    cache noise, not `Ir`).
