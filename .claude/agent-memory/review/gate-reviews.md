# Detailed Gate Review Recipes

Moved from MEMORY.md to keep the index concise. Referenced from MEMORY.md "Review Shortcuts".

## CI Job Structure

- CI: 19 YAML job entries + python-test matrix (`['3.10','3.14']`) → **20 actual jobs** (`semver`
    iter 93, `coverage` iter 94, `perf` iter 107, `audit` iter 114). Advance handoffs count YAML
    entries, not matrix expansion. Version sync: 16 targets (incl. Package.swift releaseTag).
    Release: 9 registry inputs
- **iscc-rb workspace exclusion**: `--exclude iscc-rb` in CI `rust` job is permanent — Rust job
    lacks Ruby headers/libclang-dev. Dedicated `ruby` job handles iscc-rb clippy/compile/test
- **CI-only YAML review**: `mise run check` + validate structure with
    `uv run python3 -c "import yaml; yaml.safe_load(open('.github/workflows/ci.yml'))"` (plain
    `python3` lacks `yaml` — use `uv run`) for jobs, placement, `needs:`/`continue-on-error`.
    valgrind, iai-callgrind-runner, and cargo-deny ARE present locally

## Audit (cargo-deny) — LANDED & ENFORCING (iter 114/115, ci-cd.md line 448)

- cargo-deny 0.19.9 IS in the devcontainer; verify `cargo deny check` + `mise run audit` exit 0
    (advisories/bans/licenses/sources ok; 6 `multiple-versions = "warn"` dups non-failing). Reads
    Cargo.lock + crate metadata (NOT compiled artifacts) — local green is authoritative, box flip OK
    pre-CI. Enforcing job; post-push green is CI-only confirm
- **FIX PATTERN**: a yanked crate (`yanked = "deny"`, iter 114 wasm-bindgen 0.2.111→0.2.125) OR a
    fresh advisory (iter 115 RUSTSEC-2026-0204 vs dev-only crossbeam-epoch,
    `cargo update -p crossbeam-epoch` 0.9.18→0.9.20, exactly 1 pkg) → root-fix via
    `cargo update -p`, NOT a `deny.toml` ignore; confirm dev-only reach with
    `cargo tree -i <crate> -e no-dev` = empty

## Perf (iai-callgrind) — COMPLETE, ENFORCING & HARDENED (iter 107-110, #3)

- Full recipe in `MEMORY-archive.md`. Key facts: valgrind 3.19 + `iai-callgrind-runner` 0.16.1 ARE
    in the devcontainer; `mise run bench:iai` RUNS locally (bakes `IAI_CALLGRIND_ALLOW_ASLR=true`).
    STRIP GOTCHA: `[profile.bench] strip = false, debug = true` load-bearing (else stripped → all
    `summary: 0` false green). Script-only review: `uv run pytest tests/test_iai_regression.py -q`
    (11 fixture tests) + ruff + `ty check`. Enforcing; post-push Perf-step-green is CI-only confirm

## Semver (cargo-semver-checks) — INFORMATIONAL pre-1.0 (iter 93)

- `continue-on-error: true` until the human-held v1.0.0 cut. Full recipe in `MEMORY-archive.md`

## Coverage + CRAP (iter 94/96/97/113, ci-cd.md)

- Standalone job, no `needs:`/`continue-on-error`. Phase 3 enforcing gate (iter 113, new-fn blind
    spot CLOSED) =
    `cargo crap --lcov lcov.info --baseline .crap-baseline.json --fail-regression --fail-above`
    (`--fail-above` is boolean keyed off `.cargo-crap.toml threshold = 30.0`, NO numeric arg).
    Verify with existing `lcov.info`: that command exits 0 (`0 regressed/0 new/97 unchanged`; do NOT
    pipe to tail — masks `$?`); `mise run crap:baseline` regen `.crap-baseline.json` byte-identical
    (COMMITTED). CI install MUST be `cargo binstall -y --force cargo-crap@0.2.2` (`--force`
    load-bearing, rust-cache poisoning). GOTCHA: `.cargo-crap.toml` MUST exclude
    `crates/iscc-lib/benches/**` else `bench_cdc_chunks` leaks at CRAP 42. CI YAML folds the long
    `run:` scalar across 2 lines (folded newline = space) — confirm with `yaml.safe_load`
- **CI-ONLY guard gap (iter 121→122)**: the CRAP `--fail-regression` gate runs ONLY in CI, NOT in
    `mise run check`/pre-commit — so a branch-adding source change (e.g. iter 121 `iscc_decode`
    trailing-byte guard: cyclomatic 4→5) lands green locally but turns CI red if the same step
    didn't refresh `.crap-baseline.json`. That's a follow-up-fix iteration, not a gate weakening.
- **Reviewing a `.crap-baseline.json` refresh**: regenerate + run the enforcing gate yourself, don't
    trust the handoff. `cargo llvm-cov -p iscc-lib --lcov --output-path lcov.info` then
    `cargo crap --lcov lcov.info --baseline .crap-baseline.json --fail-regression --fail-above`
    (want EXIT 0 + `↑ 0 regressed`). Tools may be missing from a fresh session: cargo-llvm-cov 0.8.7
    via `cargo binstall`, but cargo-crap 0.2.2 binstall binary needs GLIBC_2.39 (container too old)
    → `cargo install cargo-crap --version 0.2.2` from source (~1.5 min);
    `rustup component add llvm-tools-preview` first. DIFF CHECK: only the changed fn's
    `cyclomatic`/`coverage`/`crap` should move (+ it re-sorts by CRAP desc); every OTHER lib.rs
    entry must be a pure `line:` shift equal to the net lines the source change added — any
    substantive metric drift elsewhere = probe before approving (coverage-env noise)
