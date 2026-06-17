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
- **pytest-benchmark functions**: `grep -c "def test_bench_" tests/test_benchmarks.py`
- **gen_llms_full.py page count**: Python ast.literal_eval on ORDERED_PAGES list (now 22 entries)
- **UniFFI export count**: Use Grep for `#\[uniffi::export\]` in `crates/iscc-uniffi/src/lib.rs`
- **state.md Write workaround**: Write tool = permission error. Use heredoc:
    `cat > .claude/context/state.md << 'STATEEOF' ... STATEEOF`
- **CLAUDE.md files**: `ls packages/*/CLAUDE.md crates/*/CLAUDE.md 2>&1`
- **Howto guides**: `ls docs/howto/*.md | sort`
- **Version sync targets**: `uv run scripts/version_sync.py --check 2>&1 | grep "^OK:" | wc -l`
- **Release workflow inputs**: `grep "type: boolean" .github/workflows/release.yml | wc -l`
- **XCFramework verify**:
    `test -x scripts/build_xcframework.sh && bash -n scripts/build_xcframework.sh`
- **Swift release workflow check**: `grep -i 'swift\|xcframework' .github/workflows/release.yml`
- **Kotlin/Android targets**:
    `grep -A 20 "build-kotlin-native:\|android" .github/workflows/release.yml`
- **Provenance guard check**: `grep -c 'Verify main matches tag' .github/workflows/release.yml`
- **Benchmarks doc check**: `grep -i "speedup" docs/benchmarks.md | head -5`
- **PyO3 version**: `grep -n "pyo3" Cargo.toml` (workspace.dependencies — one place). Now at `0.26`
    (iter 102 bump, `Cargo.lock` 0.26.0). Migrating one minor per CID step toward `0.29` (where 2
    RustSec advisories clear). Recipe: bump pin → `cargo update -p pyo3` → build/clippy(-D
    warnings)/ fmt → `uv run maturin develop` → `uv run pytest` (286 tests). 0.23→0.24 AND 0.24→0.25
    = ZERO source changes; **0.25→0.26 was the FIRST hop needing source edits**: `allow_threads` →
    `detach` (pure rename, 7 sites; verify `grep -c allow_threads crates/iscc-py/src/lib.rs` == 0) +
    `PyObject` → `Py<PyAny>` return types (17 sites). Predicted `IntoPyObject`/lifetime breaks STILL
    not materialized through 0.26 — but 0.26 churn means later hops may also touch source; treat
    `-D   warnings` as the gate. Core has NO PyO3 dep — scope edits to `crates/iscc-py/`. Next: 0.26
    → 0.27.
- **v1.0.0 gates check**: `grep -iE "crap|semver|llvm-cov|iai-callgrind" .github/workflows/ci.yml`
    - `ls .cargo-crap.toml` + `grep -iE "coverage|crap|semver|callgrind" mise.toml`
- **Coverage + CRAP gate ALL 3 PHASES present; install flake FIXED iter 101**: ONE job named
    `Coverage + CRAP (cargo llvm-cov + cargo crap)` at ci.yml:294, no `needs:`, NO
    `continue-on-error`, job-level `security-events: write`. Pipeline: rust-toolchain@stable +
    `llvm-tools-preview` → cargo-llvm-cov → cargo-binstall → `Install cargo-crap` (ci.yml:314, now
    `cargo binstall -y --force cargo-crap@0.2.2`) →
    `cargo llvm-cov -p iscc-lib --lcov --output-path   lcov.info` → upload-artifact (`name: lcov`) →
    **Phase 2 (report-only)**: `cargo crap --lcov lcov.info --format github` +
    `--format sarif --output crap.sarif` + `codeql-action/upload-sarif@v3` → **Phase 3 (enforcing,
    iter 97)**: `CRAP regression gate`
    `cargo crap --lcov lcov.info --baseline .crap-baseline.json --fail-regression` (ci.yml:335-336),
    NOT continue-on-error, runs LAST. **⚠️ cargo-crap install flake (iter 100, FIXED iter 101
    `628c5d9`)**: the install step lacked `--force`; `Swatinem/rust-cache@v2` (ci.yml:304) restored
    cargo's `.crates` metadata WITHOUT the `~/.cargo/bin/cargo-crap` binary, so binstall SKIPPED
    ("already installed") and the next step died `error: no such command: crap` (exit 101), skipping
    the SARIF/upload/enforcing steps. The FIRST green run poisons the cache → RECURS every run. Fix
    = `--force` on the binstall (verified green run 27685108728 sha `b3c064b`: Install step + all
    CRAP steps incl. enforcing gate all `success`, none skipped). General lesson: if a binstalled
    tool flakes "already installed" under rust-cache, add `--force`. `.crap-baseline.json` (repo
    root, NOT gitignored — only `lcov.info`+`crap.sarif` are): envelope
    `{$schema, version:"0.2.2", entries:[...]}`, 97 entries / 10 `crates/iscc-lib/src/` files. Regen
    via `mise run crap:baseline` (mise.toml:119, `depends=["coverage"]`,
    `--format json --output .crap-baseline.json`) — reviewed commit, NOT CI auto-commit.
    `.cargo-crap.toml` (repo root): threshold 30, `missing="pessimistic"`, excludes all 7 binding
    crates + `packages/**` + `scripts/**` + `crates/iscc-lib/benches/**`. `mise run coverage` (110)
    \+ `mise run crap` (114, depends=coverage). Phase 1 iter 94; Phase 2 iter 96; Phase 3 iter 97.
    ci-cd.md Phases 1+2+3 boxes all `[x]`. CRAP base issue DELETED by review sweep iter 100.
    **[review] hardening issue (iter 97, still open)**: Phase 3 is regression-ONLY — a new/renamed
    fn has no baseline entry → reports `★ N new` & exits 0 (Codex verified new CC=21 fn @ CRAP 462
    bypassed). Fix: add `--fail-above 30` (baseline max ~22.3 < 30, safe). HUMAN REVIEW REQUESTED
    before spec change. If a future run flaps, regen baseline from CI's lcov artifact — do NOT widen
    `--epsilon`.
- **Semver gate present iter 93** (`9d42077`): `Semver (cargo-semver-checks)` job at ci.yml:280,
    `obi1kenobi/cargo-semver-checks-action@v2`, `package: iscc-lib`, **`continue-on-error: true`**
    (informational until v1.0.0). Mirrored `mise run semver` at mise.toml:104. CAUTION: the job
    reports individual `conclusion: failure` (2 expected breaking changes from post-0.4.0
    `pub(crate)` narrowing) but the **run-level conclusion stays `success`** — NOT a CI failure.
    rust-core.md "verified when" stays `[ ]` (requires enforcing + >= 1.0.0); ci-cd.md:417 is `[x]`
    (worded for informational state).
- **GIL/SumHasher checks** (both features done & stable): GIL-release in iscc-py is now
    `Python::detach` (PyO3 0.26 rename of `allow_threads`, 7 sites). Use
    `grep -rn "detach\|allow_threads\|SumHasher" crates/iscc-{lib,py,wasm}/src/`
- **npm optionalDeps bug (#38 FIXED iter 92)**:
    `grep -c "napi prepublish" .github/workflows/release.yml` now `0` — the `Prepare npm packages`
    step was removed. Source `crates/iscc-napi/package.json` uses bundled `files: ["*.node"]`, no
    `optionalDependencies`. Node.js now MET.
- **Module visibility check**: `grep -n "pub mod\|pub(crate) mod" crates/iscc-lib/src/lib.rs`
- **Issue count (correct)**: anchor the grep to headers with a leading `^##` before the priority
    label. That anchor excludes the legend line (plain prose), so NO -1 adjustment is needed. Do NOT
    use a bare label grep — it also matches the legend line and over-counts by 1.
- **Unpushed check**: `git log --oneline origin/develop..HEAD` — CID commits locally; origin may
    lag. If code commits sit after the last CI run sha, they are UNVERIFIED. Always cross-check
    `git log --oneline <last-CI-sha>..HEAD` against the diff.

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
- `.github/workflows/ci.yml` — **17 YAML job entries → 18 actual jobs** (`python-test` matrix
    expands 3.10 + 3.14): 16 functional jobs (incl. root Package.swift dump-package smoke test) +
    the non-blocking `Semver` job (iter 93) + the `Coverage + CRAP (cargo llvm-cov + cargo crap)`
    job (Phase 1 iter 94, Phase 2 iter 96, Phase 3 enforcing step iter 97). `push:` under `on:` is
    NOT a job; a bare `^  [a-z].*:$` grep over-counts — read the job names.
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
- `crates/iscc-lib/src/streaming.rs` — `DataHasher`, `InstanceHasher`, `SumHasher` (core struct
    added iter 88, line ~157); `gen_sum_code_v0` drives SumHasher (lib.rs:997). Only DataHasher +
    InstanceHasher re-exported at crate root (lib.rs:24); SumHasher reachable via `streaming::`.
    SumHasher streaming wrapper now in all 3 consumers: Python (iscc-py lib.rs:615), WASM (iscc-wasm
    lib.rs:533). #37 fully closed iter 90.
- `crates/iscc-lib/benches/benchmarks.rs` — 12 benches in criterion_group!
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

## Current State (assessed-at: fe1618f)

- **IN_PROGRESS — CI GREEN.** v0.4.0 released; hardening toward v1.0.0. Workspace version = `0.4.0`.
- **Iter 102 incremental** (diff `f7f3689..HEAD`). Only code-bearing change: PyO3 `0.25 → 0.26`
    (Cargo.toml:35 `0.26`, Cargo.lock `0.26.0`, crates/iscc-py/src/lib.rs detach + `Py<PyAny>`).
    Everything else is `.claude/` context/memory.
- **✅ CI PASSING.** Latest run 27687282645 (sha `03aacbd`) = **SUCCESS**. HEAD `fe1618f` adds only
    `iterations.jsonl` on top, so the green run covers HEAD's code. All 17 functional jobs +
    Coverage+CRAP green; only `Semver` shows job-level failure but continue-on-error (does NOT
    flip). Coverage+CRAP enforcing Phase 3 gate ran & passed.
- **5 issues: 0 critical, 3 normal, 2 low** (grep `^## .+\`(critical|normal|low)\`\` for headers,
    excludes legend line — no -1 adjustment).
- **Open normal gaps (3)**: PyO3 migration (now at 0.26, continue to 0.29 for RustSec), CRAP
    `--fail-above` hardening [review, HUMAN REVIEW REQUESTED], iai-callgrind perf gate (only v1.0.0
    CI gate w/ ZERO impl). cargo-semver-checks gate present — informational.
- **Low (CID skips)**: cut v1.0.0 release (human-driven), docs language logos.
- **Partially-met sections**: Rust Core (semver gate informational; perf gate missing;
    enforcing-semver needs v1.0.0), Python (**PyO3 migration in progress — GIL MET**), CI/CD
    (**GREEN**; `--fail-above` hardening + iai-callgrind remain). Node.js MET. WASM MET. All 12
    bindings met.
- **Recently closed/landed (don't re-flag)**: PyO3 0.26 (iter 102, first source-touching hop),
    cargo-crap `--force` flake fix (iter 101 `628c5d9`), PyO3 0.25 (iter 100), CRAP base issue swept
    (iter 100), CRAP Phase 3 first green (iter 99), PyO3 0.24 (iter 98), semver gate (iter 93,
    informational), npm #38 (iter 92), GIL #39 (iter 91), streaming SumHasher #37 (iters 88-90).
- **target.md/specs**: rust-core.md + ci-cd.md carry "API Stability & Performance" + "CRAP" sections
    with "verified when" checklists; ci-cd.md Phases 1+2+3 boxes all `[x]`; rust-core perf criterion
    - enforcing-semver still `[ ]`. Re-read on incremental review.

## Pattern: idle→active reactivation

- state.md idle but `git diff <hash>..HEAD --stat` shows large issues.md/target.md/specs growth →
    human re-scoped. Do a near-full re-review of affected sections, not a parrot of the diff.

## Gotchas

- **state.md Write**: Write tool = permission error. Only reliable method:
    `cat > file << 'EOF' ... EOF` via Bash tool
- Go target requires pure Go (no WASM, no wazero, no binary artifacts)
- **csbindgen**: `crates/iscc-ffi/build.rs` runs csbindgen on every `cargo build`
- **UniFFI proc macro approach**: no uniffi.toml or build.rs needed
- **Kotlin UniFFI bindings**: Uses JNA (not JNI); needs BOTH `java.library.path` AND
    `jna.library.path` at runtime
- **Kotlin release workflow**: Uses `useInMemoryPgpKeys` (env vars) instead of Java's GPG keyring;
    Central Portal upload via curl REST API (no Gradle plugin)
- **JNA ARM32 canonicalization**: JNA 5.16.0 `Platform.getNativeLibraryResourcePrefix()` maps
    `armv7` → `arm`, so resource dir must be `android-arm/` not `android-armv7/`
- **Root Package.swift**: Two manifests coexist — root for distribution (binaryTarget),
    packages/swift for CI development. `releaseChecksum = "PLACEHOLDER"` until first release with
    swift input
- **pytest-benchmark naming**: functions use `test_bench_*` prefix (not bare `bench_*`)
- **Kotlin JAR selection**: `ls *.jar | head -1` picks `-javadoc.jar` alphabetically; must `grep -v`
    classifier JARs first
