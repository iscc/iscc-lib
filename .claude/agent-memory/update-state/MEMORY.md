# Update-State Agent Memory

Codepaths, patterns, and key findings accumulated across CID iterations.

**Size budget:** Keep under 200 lines. Archive stale entries to `MEMORY-archive.md`.

## Exploration Shortcuts

- **Per-crate READMEs**: `ls crates/*/README.md packages/*/README.md 2>&1`
- **CI jobs in a run**: `gh run view <id> --json jobs --jq '.jobs[] | {name, conclusion}'`
- **Latest CI runs**:
    `gh run list --branch "$(git branch --show-current)" --limit 3 --json status,conclusion,url,databaseId`
- **Incremental diff**: `git diff <assessed-at-hash>..HEAD --stat`
- **Tier 1 pub fns in Rust core**:
    `grep -r "pub fn gen_\|pub const META\|pub const IO\|pub const TEXT" crates/iscc-lib/src/`
- **Doc nav check**: `grep -A 15 "Reference" zensical.toml`
- **C FFI extern count**: `grep -c "#\[unsafe(no_mangle)\]" crates/iscc-ffi/src/lib.rs`
- **Benchmark functions**:
    `grep -n "^fn bench_\|criterion_group" crates/iscc-lib/benches/benchmarks.rs`
- **iai-callgrind perf GATE — COMPLETE, ENFORCING & HARDENED (issue #3 + false-green issue BOTH
    closed iter 111)**: `Perf (iai-callgrind)` CI job GREEN (job at ci.yml:281-333, NO
    `continue-on-error`). Pipeline: valgrind → binstall `iai-callgrind-runner@0.16.1 --force` → run
    benches (env `IAI_CALLGRIND_ALLOW_ASLR=true`) → `Assert non-zero instruction collection` guard
    (`grep -rEq '^summary: [1-9]' target/iai/`) → ENFORCING `Check perf regression` (ci.yml:324
    `python3 scripts/iai_regression.py --check`, no continue-on-error, fails on >10% Ir regression)
    → upload `iai-baseline` artifact `if: always()`. Committed baseline `.iai-baseline.json` (repo
    root, NOT gitignored): `{metric:"Ir", tolerance_pct:10.0, benches:{<16 entries>}}`. Tasks:
    `bench:iai`, `bench:iai:check`, `bench:iai:baseline` (mise.toml ~126-144). Refresh = reviewed
    commit. **GOTCHA: the `continue-on-error: true` near this block belongs to the SEPARATE
    `semver:` job, NOT Perf — Perf enforces.** False-green hardening DONE iter 111 (`1692e2b`) — see
    `iai_regression.py` landmark below.
- **Authoritative CI status (sandbox `gh run list` is STALE — returns old ancestor SHAs)**:
    `gh api repos/iscc/iscc-lib/commits/<tip-sha>/check-runs --jq '.check_runs[]|{name,conclusion}'`
    against the ACTUAL origin/develop tip SHA. `gh run view <id> --json conclusion,headSha` also
    works.
- **pytest-benchmark functions**: `grep -c "def test_bench_" tests/test_benchmarks.py`
- **gen_llms_full.py page count**: Python ast.literal_eval on ORDERED_PAGES list (now 22 entries)
- **UniFFI export count**: Use Grep for `#\[uniffi::export\]` in `crates/iscc-uniffi/src/lib.rs`
- **state.md Write workaround**: Write tool = permission error. Use heredoc:
    `cat > .claude/context/state.md << 'STATEEOF' ... STATEEOF`
- **CLAUDE.md files**: `ls packages/*/CLAUDE.md crates/*/CLAUDE.md 2>&1`
- **Howto guides**: `ls docs/howto/*.md | sort`
- **Version sync targets**: `uv run scripts/version_sync.py --check 2>&1 | grep "^OK:" | wc -l`
- **release.yml checks** (stable, rarely changes): inputs `grep "type: boolean" release.yml | wc -l`
    (=8); Swift `grep -i 'swift\|xcframework'`; Kotlin/Android `grep -A 20 "build-kotlin-native:"`;
    provenance `grep -c 'Verify main matches tag'`. XCFramework:
    `test -x scripts/build_xcframework.sh`.
- **Benchmarks doc check**: `grep -i "speedup" docs/benchmarks.md | head -5`
- **PyO3 — MIGRATION COMPLETE (issue #1 CLOSED iter 105)**: pinned `0.29` (`Cargo.toml`; lockfile
    single 0.29.0, no older). Core has NO PyO3 dep; scope = `crates/iscc-py/`. Load-bearing
    `#[pymodule(name = "_lowlevel", gil_used = true)]` (lib.rs:697) — explicit b/c PyO3 0.28
    silently flipped that default `true`→`false`. Hop-by-hop history (0.23→0.29) + detail archived.
- **Supply-chain audit gate AUTHORIZED but STILL NOT built (verified iter 114)**: `notes/07` +
    ci-cd.md "Supply chain — cargo-deny" (Audit row line 33, section line 139) mandate a
    `cargo deny check` CI gate + root `deny.toml` + a `mise run audit` task; NONE exist yet
    (verified: no `deny.toml`, no Audit/deny job in ci.yml, no audit/deny task in mise.toml). Titusz
    AUTHORIZED autonomous impl (commit `9770332`, iter 112). ci-cd.md "verified when" box (line 448)
    `[ ]`. **This is the LAST remaining autonomous CID work package.** CID-actionable.
- **Coverage + CRAP gate ALL 3 PHASES present & GREEN; install flake FIXED iter 101**: ONE job
    `Coverage + CRAP (cargo llvm-cov + cargo crap)` (ci.yml:348, line shifted by `--fail-above`
    addition — grep the job name), no `needs:`, NO `continue-on-error`, job-level
    `security-events: write`. Pipeline: llvm-cov → cargo-binstall → `Install cargo-crap`
    (`cargo binstall -y --force cargo-crap@0.2.2`) → `cargo llvm-cov -p iscc-lib --lcov` →
    upload-artifact `lcov` → **Phase 2 report-only** (`--format github` + `--format sarif` →
    `codeql-action/upload-sarif@v3`) → **Phase 3 enforcing** (now ci.yml:392-393, runs LAST). Flake
    fix iter 101 = `--force` on binstall (mechanism archived). `.crap-baseline.json` (repo root, NOT
    gitignored): `{$schema, version:"0.2.2", entries:[...]}`, 97 entries / 10 src files. Regen via
    `mise run crap:baseline` — reviewed commit, NOT auto. `.cargo-crap.toml`: threshold 30,
    `missing="pessimistic"`, excludes 7 binding crates + `packages/**` + `scripts/**` +
    `benches/**`. ci-cd.md Phases 1+2+3 all `[x]`. **`--fail-above` hardening DONE & GREEN (iter
    113, advance `c056779`, reviewed `cee130a`)**: enforcing CRAP step now runs
    `cargo crap --lcov lcov.info --baseline .crap-baseline.json --fail-regression --fail-above`
    (ci.yml:392-393). `--fail-above` = boolean keyed off `.cargo-crap.toml threshold = 30.0` (NO
    numeric arg); closes the regression-only blind spot (new/renamed CC-heavy fn reported `★ N new`
    & exited 0). Max CRAP ~22.3 < 30 so it passes today; ci-cd.md verified-when box (line 445)
    `[x]`. GOTCHA: yamlfix folds the long `run:` scalar onto 2 physical lines (folded newline = a
    space) — confirm via `yaml.safe_load`, not raw grep.
- **Semver gate present iter 93**: `Semver (cargo-semver-checks)` job (grep the name),
    `obi1kenobi/cargo-semver-checks-action@v2`, `package: iscc-lib`, **`continue-on-error: true`**
    (informational until v1.0.0; `mise run semver`). CAUTION: job reports `failure` (2 expected
    breaking changes from post-0.4.0 `pub(crate)` narrowing) but **run conclusion stays `success`**
    — NOT a CI failure. rust-core.md semver "verified when" stays `[ ]` (needs enforcing + ≥1.0.0);
    ci-cd.md `[x]` (informational wording).
- **Issue count (correct)**: grep `issues.md` for `^##` headers ending in a priority label
    (critical/normal/low) — anchoring to `^##` excludes the legend line, so NO -1 adjustment.
- **Unpushed check**: `git log --oneline origin/develop..HEAD` — origin lags; code after the last
    CI-run sha is UNVERIFIED, cross-check the diff. (Usual case: HEAD = +1 log-only commit.)

## Codebase Landmarks

- `crates/` — **8 crates**: iscc-lib, iscc-py, iscc-napi, iscc-wasm, iscc-ffi, iscc-jni, iscc-rb,
    iscc-uniffi (all 32/32 symbols)
- `.claude/context/specs/` — per-binding spec files
- `packages/go/` — pure Go module (no WASM, no binary artifacts)
- `packages/swift/` — SPM package with UniFFI-generated bindings (2400-line iscc_uniffi.swift)
- `Package.swift` — **root manifest** — Ferrostar toggle (`useLocalFramework`), `.binaryTarget` with
    `releaseTag`/`releaseChecksum`, two targets (iscc_uniffiFFI binary + IsccLib)
- `scripts/build_xcframework.sh` — builds XCF for 5 Apple targets, lipo fat binaries, ditto zip
- `packages/kotlin/` — Kotlin/JVM, Gradle 8.12.1, UniFFI-generated (3214-line iscc_uniffi.kt), JNA
    5.16.0; conformance tests (9 methods, 50 vectors); docs + release workflow complete
- `.github/workflows/ci.yml` — **18 YAML job entries → 19 actual jobs** (`python-test` matrix
    expands 3.10 + 3.14): functional jobs + non-blocking `Semver` + `Coverage + CRAP` (Phases 1-3 +
    `--fail-above`) + `Perf (iai-callgrind)`. `push:` under `on:` is NOT a job; read the job names
    (bare grep over-counts). NO Audit/cargo-deny job yet.
- `.github/workflows/release.yml` — **8 registry input toggles** (`type: boolean`): crates-io, pypi,
    npm, maven, ffi, rubygems, nuget, maven-kotlin. Swift XCFramework is NOT a toggle — it builds in
    `prepare-release` (line ~55). **provenance guard** on build-xcframework. After #38 fix (iter 92)
    the `publish-npm-lib` job has NO `napi prepublish` step.
- `crates/iscc-uniffi/` — UniFFI scaffolding: 32 exports, 21 tests; `publish = false`
- `docs/howto/` — **11 files**: rust, python, nodejs, wasm, go, java, c-cpp, ruby, dotnet, swift,
    kotlin
- `docs/benchmarks.md` — full speedup comparison (1.3x-158x), Criterion native results, methodology
- `scripts/gen_llms_full.py` — **22 entries** in ORDERED_PAGES (includes benchmarks.md)
- `scripts/version_sync.py` — **16 sync targets** (releaseTag added for Package.swift)
- `crates/iscc-lib/src/streaming.rs` — `DataHasher`, `InstanceHasher`, `SumHasher` (line ~157);
    `gen_sum_code_v0` drives SumHasher (lib.rs:997). Only DataHasher + InstanceHasher re-exported at
    crate root (lib.rs:24); SumHasher via `streaming::`. SumHasher wrapper in Python (iscc-py
    lib.rs:615) + WASM (iscc-wasm lib.rs:533). #37 closed iter 90.
- `crates/iscc-lib/benches/benchmarks.rs` — 12 benches in criterion_group!
- `crates/iscc-lib/benches/iai_benches.rs` — iai-callgrind 0.16 harness (iter 107), 11 `bench_*` fns
    (9 `gen_*_v0` + cdc + minhash) in `library_benchmark_group!(iscc_benches)` (16 parametrized
    cases). `[[bench]] name="iai_benches" harness=false`; dep `iai-callgrind = "0.16"`.
    **`[profile.   bench]` inherits release `strip=true`** → stripped `__iai_callgrind_wrapper`
    symbols → false green; FIX (iter 108): `[profile.bench] strip=false, debug=true` (Cargo.toml:61)
    \+ `IAI_CALLGRIND_ALLOW_ASLR=true`. `#[library_benchmark]` fns use `//` not `///` (macro
    `abort!`s on `doc`).
- `scripts/iai_regression.py` (iter 109 `949f63f`; HARDENED iter 111 `1692e2b`, now 248 lines,
    **stdlib-only**) — `--check` parses `target/iai/**/*.out` `summary:` lines vs committed
    `.iai-baseline.json`; fails on >10% Ir regression. Core fn `check_regressions` takes an
    `allow_missing` flag: shared benches over tolerance FAIL; a shared bench reporting zero Ir FAILS
    (independent of `--allow-missing`); a baselined bench missing from the run FAILS unless
    `--allow-missing`; new (run-only) benches still WARN only. `--update` regenerates baseline.
- `tests/test_iai_regression.py` (NEW iter 111) — 11 synthetic-fixture pytest tests for the gate
    (within-tolerance, over-tolerance, zero-count fails even w/ allow-missing, missing-bench fail +
    allow-missing pass, only-run warns, boundary, roundtrip, parse_ir).
- `tests/test_benchmarks.py` — 18 pytest-benchmark functions (9 gen\_\*\_v0 x 2 implementations)
- **CLAUDE.md files & per-crate READMEs**: 12 each (all crates + all packages)

## Recurring Patterns

- **Incremental review**: compare assessed-at hash vs HEAD --stat first, then re-verify only
    affected sections. Carry forward unchanged sections. (python-test matrix = 3.10 + 3.14; count
    job definitions, not run records.)
- **Verify claims independently**: always grep rather than trusting handoff; verify CI independently
    (a review PASS = LOCAL checks only, CI can still flake — see iter-100 CRAP break); re-read
    target.md diff on incremental review; verify "partially met" claims rather than parroting.
- **Issues diff**: check issues.md for NEW entries each cycle (human AND `[review]`-sourced). Watch
    `[review]` + `HUMAN REVIEW REQUESTED` flags and any critical ones that reshuffle priorities.
- **idle→active reactivation**: state.md idle but `git diff <hash>..HEAD --stat` shows large
    issues.md/target.md/specs growth → human re-scoped. Do a near-full re-review, not a diff parrot.

## Current State (assessed-at: ea284c0)

- **IN_PROGRESS — CI GREEN; one autonomous work package left (cargo-deny gate).** v0.4.0 released,
    all 12 bindings met. Workspace version `0.4.0`; v1.0.0 HELD (stay 0.4.x; land cargo-deny gate
    first).
- **Iter 114 incremental** (diff `c7e5466..HEAD`): the CRAP `--fail-above` gate LANDED (advance
    `c056779`, review PASS `cee130a`) — touched `ci.yml` (+11), `.cargo-crap.toml` (+8), ci-cd.md
    spec (box 445 → `[x]`), removed the resolved CRAP issue from issues.md. Only CI/CD section
    changed; all bindings carry forward.
- **CRAP `--fail-above` DONE & GREEN**: ci.yml:392-393
    `cargo crap --lcov lcov.info --baseline .crap-baseline.json --fail-regression --fail-above`;
    `Coverage + CRAP` job green (max CRAP ~22.3 < 30). ci-cd.md box 445 `[x]`.
- **cargo-deny gate STILL UNIMPLEMENTED (last work package)**: NO `deny.toml`, NO Audit/deny job in
    ci.yml, NO audit/deny task in mise.toml; neither tool installed. ci-cd.md box 448 `[ ]`.
    AUTHORIZED — CID-actionable.
- **✅ CI GREEN on origin/develop tip `cee130a`**, completed runs `27759986239`/`27759984667` =
    SUCCESS (check-runs API on tip SHA). All functional jobs green incl. Perf + Coverage+CRAP +
    cargo-crap; only `Semver` failure (continue-on-error, informational). HEAD `ea284c0` is +1
    log-only commit (iterations.jsonl) → CI reflects current code.
- **3 issues: 0 critical, 1 normal (cargo-deny, AUTHORIZED), 2 low** (count issues.md `^##` headers
    ending a priority label; legend excluded → no -1).
- **Low (CID skips)**: v1.0.0 release HELD (human-driven), docs language logos.
- **Partially-met sections**: Rust Core (semver informational; enforcing needs v1.0.0 cut — held;
    perf gate MET & hardened), CI/CD (**GREEN**; CRAP `--fail-above` MET, cargo-deny pending build).
    All 12 bindings + Benchmarks + Docs MET.
- **Recently closed/landed (don't re-flag)**: CRAP `--fail-above` (iter 113 `c056779`), iai
    false-green hardening (iter 111 `1692e2b`), iai perf gate (iters 107-109), PyO3 #1 (iters
    98-105), CRAP Phase 3 (iter 99), semver gate (iter 93, informational), npm #38 (iter 92), GIL
    #39 (iter 91), SumHasher #37 (iters 88-90).
- **target.md/specs**: rust-core.md perf boxes `[x]`; ci-cd.md CRAP Phases 1+2+3 + `--fail-above`
    (box 445) all `[x]`; Audit/cargo-deny box 448 `[ ]`; enforcing-semver `[ ]` (flips only at
    v1.0.0 cut). Re-read on incremental review.

## Gotchas

- **state.md Write**: Write tool = permission error. Only reliable method:
    `cat > file << 'EOF' ... EOF` via Bash tool
- **mdformat pre-commit can abort the commit** ("Could not format / renders to different HTML"):
    triggered by nested/escaped backticks inside a code span. Keep literal regex backticks OUT of
    code spans in MEMORY.md (describe in prose); test with `uv run mdformat /tmp/copy.md` first.
- Go target requires pure Go (no WASM, no wazero, no binary artifacts)
- **csbindgen**: `crates/iscc-ffi/build.rs` runs csbindgen on every `cargo build`
- **UniFFI proc macro approach**: no uniffi.toml or build.rs needed
- **Kotlin UniFFI bindings**: Uses JNA (not JNI); needs BOTH `java.library.path` AND
    `jna.library.path` at runtime
- **pytest-benchmark naming**: functions use `test_bench_*` prefix (not bare `bench_*`)
- **Release-workflow deep internals** (Kotlin GPG/Central Portal, JNA ARM32 `armv7`→`arm`, dual
    Package.swift manifests, Kotlin JAR classifier pick) archived → `MEMORY-archive.md`.
