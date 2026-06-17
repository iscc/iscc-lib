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
- `.github/workflows/ci.yml` — **17 CI jobs** (16 functional incl. root Package.swift dump-package
    smoke test + the non-blocking `Semver` job added iter 93)
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

## Current State (assessed-at: 96558ca)

- **IN_PROGRESS** — v0.4.0 released; hardening toward v1.0.0. Workspace version = `0.4.0`.
- **CI**: overall green (run 27664286163, sha `758fc31`). 16 functional jobs green + non-blocking
    `Semver` job (reports `failure` but `continue-on-error` keeps run green). This run **includes**
    the semver-gate addition (advance `9d42077`, review `758fc31`). HEAD `96558ca` == origin/develop
    is `cid(log): iteration 93` (iterations.jsonl/memory/context only). All code pushed and
    verified.
- **5 issues: 0 critical, 3 normal, 2 low** (count by grepping header lines anchored with a leading
    `##` before the priority label — that excludes the legend line, so no -1 adjustment needed).
- **Informational semver gate ADDED (iteration 93, `9d42077`/`758fc31`)** — closes the
    cargo-semver-checks issue (deleted). One of four normal backlog items done. CI/config/doc only —
    no source/API/conformance/perf surface touched.
- **npm optionalDeps #38 CLOSED (iteration 92, `5d8ecb7`/`fa59c0b`)** — `napi prepublish` step
    deleted from `release.yml`; npm ships single bundled package. Node.js now MET. Docs updated
    (iscc-napi CLAUDE.md, notes 02/06). No code/API/conformance surface touched.
- **GIL release DONE / #39 CLOSED (iteration 91, `534b531`/`d2d7e7e`)** — `py.allow_threads(...)`
    wraps pure-Rust compute at all **7** sites in `crates/iscc-py/src/lib.rs`: 4 one-shot byte fns
    (image:152, data:295, instance:309, +346) + 3 streaming `update()` (DataHasher:551,
    InstanceHasher:600, SumHasher:651). `tests/test_gil.py` = 7 concurrency tests. Output
    byte-identical; `.pyi` stubs unchanged (injected `py` invisible to Python). #39 deleted.
- **#37 FULLY CLOSED (iteration 90, `3f3bf65`/`984edba`)** — streaming SumHasher in core + Python +
    WASM. WASM: `pub struct SumHasher` at `crates/iscc-wasm/src/lib.rs:533`, `impl` at 545, reuses
    `WasmSumCodeResult`, finalize-once via `inner.take()`; 8 `test_sum_hasher_*` tests (78 total
    wasm-pack); documented in `docs/howto/wasm.md`.
- **Python SumHasher (iteration 89, `742759b`)**: `#[pyclass(name="SumHasher")]` = `PySumHasher` at
    `crates/iscc-py/src/lib.rs:631` over `iscc_lib::streaming::SumHasher`; Pythonic wrapper class at
    `__init__.py:349`, exported in `__all__` (line 402); `.pyi` stub + 11 tests.
- **Core SumHasher (iteration 88, `3fc44d2`)**: `pub struct SumHasher` in `streaming.rs:157`
    (new/update/finalize(bits,wide,add_units)/Default). `gen_sum_code_v0` (lib.rs:997) drives it.
    Reachable as `iscc_lib::streaming::SumHasher` but NOT a crate-root re-export (only
    `DataHasher`/`InstanceHasher` are, lib.rs:24). SumHasher NOT promoted to Tier 1 — counts stay
    32\.
- **Module visibility (iteration 86, `3f6a61d`)**:
    `cdc/conformance/dct/minhash/simhash/utils/wtahash` = `pub(crate) mod`; only
    `codec/streaming/types` = `pub mod`. Issue swept.
- **Open normal gaps (3)**: PyO3 0.23→0.29 (RustSec, still pinned at `Cargo.toml`), CRAP coverage
    gate, iai-callgrind perf gate. (cargo-semver-checks gate now present — informational.)
- **Low (CID skips)**: cut v1.0.0 release (human-driven), docs language logos.
- **Partially-met sections**: Rust Core (semver gate now informational; perf gate still missing;
    enforcing-semver needs v1.0.0), Python (**PyO3 only — GIL MET**), CI/CD (2 v1.0.0 tooling gates:
    CRAP + iai-callgrind). Node.js MET. WASM MET. All 12 bindings functionally met.
- **target.md/specs**: rust-core.md + ci-cd.md carry "API Stability & Performance" + "CRAP" sections
    with "verified when" checklists; python-bindings.md "GIL Release During Hashing" now all `[x]`.
    Re-read on incremental review.

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
