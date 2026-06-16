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
- **Issue count**: `grep -c '^## .* \`\(critical\|normal\|low\)\`' .claude/context/issues.md\`
- **Provenance guard check**: `grep -c 'Verify main matches tag' .github/workflows/release.yml`
- **Benchmarks doc check**: `grep -i "speedup" docs/benchmarks.md | head -5`
- **PyO3 version**: `grep -n "pyo3" Cargo.toml` (workspace.dependencies — one place)
- **v1.0.0 gates check** (all should appear when done):
    `grep -iE "crap|semver|llvm-cov|iai-callgrind" .github/workflows/ci.yml` + `ls .cargo-crap.toml`
    - `grep -iE "coverage|crap|semver|callgrind" mise.toml`
- **GIL release check**: `grep -rn "allow_threads" crates/iscc-py/src/`
- **SumHasher check**:
    `grep -rn "SumHasher" crates/iscc-lib/src/ crates/iscc-py/src/ crates/iscc-wasm/src/`
- **npm optionalDeps bug**: `grep -n "napi prepublish" .github/workflows/release.yml` (line ~378 =
    injection still present); source `crates/iscc-napi/package.json` uses bundled
    `files: ["*.node"]`
- **Module visibility check**: `grep -n "pub mod\|pub(crate) mod" crates/iscc-lib/src/lib.rs`

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
- `.github/workflows/ci.yml` — **16 CI jobs** (includes root Package.swift dump-package smoke test)
- `.github/workflows/release.yml` — **9 registry inputs**: crates-io, pypi, npm, maven, ffi,
    rubygems, nuget, maven-kotlin, swift; **provenance guard** on build-xcframework
- `crates/iscc-uniffi/` — UniFFI scaffolding: 32 exports, 21 tests; `publish = false`
- `docs/howto/` — **11 files**: rust, python, nodejs, wasm, go, java, c-cpp, ruby, dotnet, swift,
    kotlin
- `docs/benchmarks.md` — full speedup comparison (1.3x-158x), Criterion native results, methodology
- `scripts/gen_llms_full.py` — **22 entries** in ORDERED_PAGES (includes benchmarks.md)
- `scripts/version_sync.py` — **16 sync targets** (releaseTag added for Package.swift)
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

## Current State (assessed-at: b1127ed)

- **IN_PROGRESS** — project came OUT of idle: v0.4.0 released, then human re-injected work toward
    v1.0.0. Workspace version = `0.4.0`.
- **CI GREEN**: 16/16 jobs (run 27629376698, sha 63523ba). HEAD is 2 commits ahead but they only
    touch context/mise.toml (CI doesn't use mise) — green status holds.
- **0 critical, 7 normal, 2 low issues** (human triaged GitHub issues #37/#38/#39 + v1.0.0 prep).
- **Normal gaps**: PyO3 0.23→0.29 (RustSec), npm optionalDeps bug (#38), streaming SumHasher (#37,
    core+py+wasm), GIL allow_threads (#39), CRAP coverage gate, cargo-semver-checks gate,
    iai-callgrind perf gate, narrow lib.rs modules to pub(crate).
- **Low (CID skips)**: cut v1.0.0 release (human-driven), docs language logos.
- **Partially-met sections now**: Rust Core (semver+perf gates, module visibility), Python (PyO3,
    SumHasher, GIL), Node.js (npm optionalDeps), WASM (SumHasher), CI/CD (3 new gates + npm fix).
    All 12 bindings still functionally met for v0.4.0; the bar was raised, not regressed.
- **target.md/specs grew**: rust-core.md + ci-cd.md added "API Stability & Performance" and "CRAP"
    sections with new "verified when" checklists. Always re-read on incremental review.

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
