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
- **Kotlin native targets**: `grep -A 20 "build-kotlin-native:" .github/workflows/release.yml`
- **Android target check**: `grep "android" .github/workflows/release.yml`
- **Provenance guard check**: `grep -c 'Verify main matches tag' .github/workflows/release.yml`
- **Benchmarks doc check**: `grep -i "speedup" docs/benchmarks.md | head -5`
- **PyO3 version**: `grep -n "pyo3" Cargo.toml` (workspace.dependencies — one place)
- **v1.0.0 gates check** (all should appear when done):
    `grep -iE "crap|semver|llvm-cov|iai-callgrind" .github/workflows/ci.yml` + `ls .cargo-crap.toml`
    - `grep -iE "coverage|crap|semver|callgrind" mise.toml`
- **Coverage + CRAP gate Phases 1+2 present iter 94/96**: ONE job named
    `Coverage + CRAP (cargo llvm-cov + cargo crap)` at ci.yml:293 (renamed from
    `Coverage (cargo llvm-cov)` in iter 96), no `needs:`, NO `continue-on-error`, job-level
    `security-events: write`. Pipeline: rust-toolchain@stable + `llvm-tools-preview` → install
    cargo-llvm-cov → install cargo-binstall → `cargo binstall -y   cargo-crap@0.2.2` →
    `cargo llvm-cov -p iscc-lib --lcov --output-path lcov.info` → upload-artifact (`name: lcov`) →
    `cargo crap --lcov lcov.info --format github` → `--format sarif --output   crap.sarif` →
    `codeql-action/upload-sarif@v3`. REPORT-ONLY: no `fail-above`/`fail-regression`, so steps exit 0
    (highest CRAP `gen_meta_code_v0`=22.3 < threshold 30 → zero annotations in practice).
    `.cargo-crap.toml` (repo root): threshold 30, `missing="pessimistic"`, excludes all 7 binding
    crates + `packages/**` + `scripts/**` + `crates/iscc-lib/benches/**`. Mirrored
    `mise run coverage` (mise.toml:110) + `mise run crap` (depends=["coverage"]). `lcov.info` +
    `crap.sarif` gitignored. Phase 1 (LCOV) iter 94 (`0697195`); Phase 2 (report-only crap+SARIF)
    iter 96 (`6ed51c5`/`cbc0d14`). Phase 3 (`--fail-regression` baseline refreshed on develop
    merges) REMAINS → CRAP issue stays OPEN. ci-cd.md Phase 2 checkboxes now `[x]`; Phase 3 `[ ]`.
- **Semver gate present iter 93** (`9d42077`): `Semver (cargo-semver-checks)` job at ci.yml:280,
    `obi1kenobi/cargo-semver-checks-action@v2`, `package: iscc-lib`, **`continue-on-error: true`**
    (informational until v1.0.0). Mirrored `mise run semver` at mise.toml:104. CAUTION: the job
    reports individual `conclusion: failure` (2 expected breaking changes from post-0.4.0
    `pub(crate)` narrowing) but the **run-level conclusion stays `success`** — NOT a CI failure.
    rust-core.md "verified when" stays `[ ]` (requires enforcing + >= 1.0.0); ci-cd.md:417 is `[x]`
    (worded for informational state).
- **GIL release check**: `grep -rn "allow_threads" crates/iscc-py/src/`
- **SumHasher check**:
    `grep -rn "SumHasher" crates/iscc-lib/src/ crates/iscc-py/src/ crates/iscc-wasm/src/`
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
    job (iter 94 Phase 1, iter 96 Phase 2). `push:` under `on:` is NOT a job; a bare `^  [a-z].*:$`
    grep over-counts — read the job names.
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
- **CLAUDE.md files**: 12 total (all crates + all packages)
- **Per-crate READMEs**: 12 total (all crates + all packages)

## Recurring Patterns

- **Incremental review**: compare assessed-at hash vs HEAD --stat first, then re-verify only
    affected sections. Carry forward unchanged sections.
- **CI has matrix jobs**: python-test = 3.10 + 3.14; gate job checks both. Count definitions not run
    records.
- **Verify claims independently**: always grep rather than trusting handoff.
- **Target may change**: always re-read target.md diff when doing incremental review.
- **Handoff predictions may be wrong**: always verify CI independently.
- **Issues filed by human**: Human filed 5 new issues from Codex PR review — always check issues.md
    diff for new entries, especially critical ones that change priorities.
- **Review-filed issues**: Review agent can file issues (e.g., JNA ARM32 path mismatch). Check for
    `[review]` source tag and `HUMAN REVIEW REQUESTED` flag.
- **Prior state may have errors**: Always verify "partially met" claims — e.g., benchmarks doc
    existed but was marked missing in iteration 6 state.

## Current State (assessed-at: 150c778)

- **IN_PROGRESS** — v0.4.0 released; hardening toward v1.0.0. Workspace version = `0.4.0`.
- **Iter 96 = CRAP Phase 2 LANDED** (`6ed51c5` advance, `cbc0d14` review PASS). Diff `e95f57b..HEAD`
    code-bearing files: `.cargo-crap.toml` (new), `ci.yml` (+20: rename job, install binstall+crap,
    2 crap runs, SARIF upload), `mise.toml` (+5: crap task), `.gitignore` (+crap.sarif). Rest is
    `.claude/` context/memory. See Coverage+CRAP landmark for the live facts.
- **CI**: overall green (run 27675312942, sha `cbc0d14`). 18 actual jobs: 16 functional, the
    `Coverage + CRAP` job (green), and non-blocking `Semver` (reports `failure` but
    `continue-on-error` keeps run green — NOT a CI failure). HEAD `150c778` ("cid(log): iteration
    96") is **1 commit ahead** of origin/develop (`cbc0d14`), context/log-only & unpushed. Latest
    code pushed+verified at `cbc0d14`.
- **5 issues: 0 critical, 3 normal, 2 low** (count by grepping header lines anchored with a leading
    `##` before the priority label — excludes the legend line, no -1 adjustment).
- **Open normal gaps (3)**: PyO3 0.23→0.29 (RustSec, still pinned at `Cargo.toml`), CRAP gate
    **Phase 3** (`--fail-regression` baseline — Phases 1+2 DONE), iai-callgrind perf gate.
    (cargo-semver-checks gate present — informational.) Next incremental step = CRAP Phase 3
    (capture `cargo crap --format json` baseline, `--fail-regression --baseline`, refresh on develop
    merges); closes the CRAP issue.
- **Low (CID skips)**: cut v1.0.0 release (human-driven), docs language logos.
- **Partially-met sections**: Rust Core (semver gate informational; perf gate missing;
    enforcing-semver needs v1.0.0), Python (**PyO3 only — GIL MET**), CI/CD (Coverage+CRAP Phases
    1+2 done; CRAP Phase 3 + iai-callgrind remain). Node.js MET. WASM MET. All 12 bindings
    functionally met.
- **Recently closed (don't re-flag)**: semver gate (iter 93, informational), npm #38 (iter 92), GIL
    #39 (iter 91), streaming SumHasher #37 (iters 88-90, core+Python+WASM). See Codebase Landmarks
    for live SumHasher / module-visibility facts.
- **target.md/specs**: rust-core.md + ci-cd.md carry "API Stability & Performance" + "CRAP" sections
    with "verified when" checklists; ci-cd.md Phase 2 boxes flipped `[x]`, Phase 3 `[ ]`. Re-read on
    incremental review.

## Pattern: idle→active reactivation

- When state.md shows near-complete/idle but `git diff <hash>..HEAD --stat` shows large
    issues.md/target.md/specs growth, the human likely re-scoped. Treat as a near-full re-review of
    affected sections, not a parrot of the diff. Commit that triggered this: 7463ef8 "docs(cid):
    triage GitHub issues into context".

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
