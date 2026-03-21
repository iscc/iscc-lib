# Update-State Agent Memory

Codepaths, patterns, and key findings accumulated across CID iterations.

**Size budget:** Keep under 200 lines. Archive stale entries to `MEMORY-archive.md`.

## Exploration Shortcuts

- **Java files**: `find crates/iscc-jni -type f | sort`
- **Per-crate READMEs**: `ls crates/*/README.md packages/*/README.md 2>&1`
- **CI jobs in a run**: `gh run view <id> --json jobs --jq '.jobs[] | {name, conclusion}'`
- **Latest CI runs**:
    `gh run list --branch "$(git branch --show-current)" --limit 3 --json status,conclusion,url,databaseId`
- **Incremental diff**: `git diff <assessed-at-hash>..HEAD --stat`
- **Go files**: `ls packages/go/*.go` — check pure Go source files
- **New symbol existence check**:
    `grep -r "gen_sum_code\|META_TRIM_META\|SumCodeResult" crates/ packages/ 2>/dev/null | grep -v "target\|\.lock\|\.md"`
- **Tier 1 pub fns in Rust core**:
    `grep -r "pub fn gen_\|pub const META\|pub const IO\|pub const TEXT" crates/iscc-lib/src/`
- **Go test count**: `grep -r "^func Test" packages/go/ --include="*_test.go" | wc -l`
- **Go gen functions**: `grep "^func Gen" packages/go/code_*.go`
- **Doc nav check**: `grep -A 15 "Reference" zensical.toml`
- **llms.txt page count**: `grep -c "^\-" docs/llms.txt`
- **C FFI extern count**: `grep -c "#\[unsafe(no_mangle)\]" crates/iscc-ffi/src/lib.rs`
- **Howto Sum-Code check**:
    `grep -n "### Sum-Code\|gen_sum_code_v0\|GenSumCodeV0\|genSumCodeV0" docs/howto/*.md`
- **Benchmark functions**:
    `grep -n "^fn bench_\|criterion_group" crates/iscc-lib/benches/benchmarks.rs`
- **C# public symbols**:
    `grep -n "public static\|public sealed record" packages/dotnet/Iscc.Lib/IsccLib.cs | grep -v "private\|internal\|partial class"`
- **C# test count**: `grep -c "\[Fact\]" packages/dotnet/Iscc.Lib.Tests/SmokeTests.cs`
- **C# conformance test count**:
    `grep -c "\[Theory\]" packages/dotnet/Iscc.Lib.Tests/ConformanceTests.cs`
- **C# record types**: `grep -c "sealed record" packages/dotnet/Iscc.Lib/Results.cs`
- **C# streaming Finalize check**:
    `grep -n "public.*Finalize" packages/dotnet/Iscc.Lib/IsccDataHasher.cs packages/dotnet/Iscc.Lib/IsccInstanceHasher.cs`
- **NuGet pipeline check**:
    `grep -n 'pack-nuget\|test-nuget\|publish-nuget\|NUGET_API_KEY' .github/workflows/release.yml`
- **C++ package files**: `find packages/cpp -type f | sort`
- **C++ CI job check**: `grep -n "cpp\|C++" .github/workflows/ci.yml`
- **C++ hpp symbol check**:
    `grep -n "^inline\|^struct\|^class\|// ---" packages/cpp/include/iscc/iscc.hpp`
- **C++ iscc.hpp in release.yml**: `grep -n 'iscc.hpp' .github/workflows/release.yml`
- **gen_llms_full.py page count**: `grep -c "^\s*\"" scripts/gen_llms_full.py` (ORDERED_PAGES list)
- **Conan recipe check**:
    `grep -n 'download\|package_type\|cxxflags\|_target_triple' packages/cpp/conanfile.py`
- **vcpkg SHA512 check**: `grep -n 'SKIP_SHA512\|ISCC_SHA512' packages/cpp/portfile.cmake`
- **UniFFI crate check**: `ls crates/iscc-uniffi/src/lib.rs 2>&1` (does not exist yet)
- **Swift package check**: `ls packages/swift/Package.swift 2>&1` (does not exist yet)

## Codebase Landmarks

- `crates/` — 7 crates: iscc-lib, iscc-py, iscc-napi, iscc-wasm, iscc-ffi, iscc-jni, **iscc-rb**
    (32/32 symbols — COMPLETE)
- `.claude/context/specs/` — per-binding spec files (ruby, go, java, nodejs, wasm, cpp, dotnet,
    swift, kotlin, rust-core, c-ffi-dx, documentation, ci-cd)
- `packages/go/` — pure Go module (no WASM bridge, no binary artifacts)
- `.github/workflows/ci.yml` — jobs: version-check, Rust, python-test (matrix 3.10+3.14), python
    (gate), Node.js, WASM, C FFI, Java, Go, Bench, Ruby, C# / .NET, **C++ (cmake, ASAN, test)**
    (**14 total**)
- `packages/dotnet/` — `Iscc.Lib/IsccLib.cs` (**32/32 Tier 1 symbols**), `Results.cs` (**11 sealed
    records**), `IsccDataHasher.cs` + `IsccInstanceHasher.cs` (IDisposable + SafeHandle; both
    **`Finalize()` returns typed record**), `IsccException.cs`, `SmokeTests.cs` (**41 tests**),
    `ConformanceTests.cs` (**9 Theory tests, 50 vectors**), `testdata/data.json` (84KB vendored),
    `NativeMethods.g.cs` (csbindgen, 47 externs)
- `docs/howto/` — **9 files**: rust.md, python.md, nodejs.md, wasm.md, go.md, java.md, c-cpp.md,
    ruby.md, **dotnet.md**
- `scripts/gen_llms_full.py` — **20 entries** in `ORDERED_PAGES`
- `scripts/version_sync.py` — syncs workspace version across Cargo.toml, package.json, pom.xml,
    Iscc.Lib.csproj, **vcpkg.json** and **conanfile.py**
- `crates/iscc-lib/benches/benchmarks.rs` — 277 lines; 12 benches in `criterion_group!`

## Recurring Patterns

- **Incremental review**: compare `assessed-at` hash vs HEAD `--stat` first, then re-verify only
    affected sections. Always carry forward sections where no relevant files changed
- **CI has matrix jobs**: python-test runs as Python 3.10 + Python 3.14 (separate records); gate job
    `python` checks both pass. Count distinct job definitions, not run records.
- `gh run list` does NOT need `--repo` when running from within the workspace; but `--json` fields
    are needed to avoid GraphQL deprecation error
- **Verify claims independently**: review agents can make incorrect claims. Always grep for each
    missing symbol rather than trusting handoff verdict counts. Verify issues.md directly.
- **Target may change**: always re-read target.md diff when doing incremental review

## Current State (assessed-at: 94ce7f6d3f6ca0088c213a70ac7dfc4c84570bd9)

- **IN_PROGRESS**: all **14 CI jobs** green (run 22858622397)
- **v0.3.1 released** — all 8 registries including RubyGems and NuGet
- **2 normal-priority issues** in issues.md: Swift bindings, Kotlin bindings (both not started)
- **0 critical/other normal issues** — all previous issues closed
- Swift/Kotlin issues bumped from `low` to `normal` (commit 94ce7f6) — now CID-eligible

## Gotchas

- **Bash tool backtick issue**: Use Write tool to create temp Python script for backtick content
- **state.md encoding**: Use `git checkout` for clean UTF-8; PowerShell adds BOM
- Go target requires pure Go (no WASM, no wazero, no binary artifacts)
- WASM constant name gotcha: `#[wasm_bindgen(js_name = "META_TRIM_NAME")]` exports uppercase
- **WASM count assertions**: when data.json gains new vectors, BOTH lib.rs AND conformance.rs need
    updates
- **csbindgen**: `crates/iscc-ffi/build.rs` runs csbindgen on every `cargo build`
- **C++ iscc.hpp**: 681-line C++17 header-only. cmake/g++ must be apt-get installed in CI
- **Ruby JSON sort_keys no-op**: use `JSON.generate(hash.sort.to_h)` not `sort_keys: true`
