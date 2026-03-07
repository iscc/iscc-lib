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

## Codebase Landmarks

- `crates/` — 7 crates: iscc-lib, iscc-py, iscc-napi, iscc-wasm, iscc-ffi, iscc-jni, **iscc-rb**
    (32/32 symbols — COMPLETE)
- `.claude/context/specs/` — per-binding spec files (ruby, go, java, nodejs, wasm, cpp, dotnet,
    swift, kotlin, rust-core, c-ffi-dx, documentation, ci-cd)
- `packages/go/` — pure Go module (no WASM bridge, no binary artifacts)
- `.github/workflows/ci.yml` — jobs: version-check, Rust, python-test (matrix 3.10+3.14), python
    (gate), Node.js, WASM, C FFI, Java, Go, Bench, Ruby, **C# / .NET** (**13 total**) ✅
- `packages/dotnet/` — `Iscc.Lib/IsccLib.cs` (**32/32 Tier 1 symbols**), `Results.cs` (**11 sealed
    records**), `IsccDataHasher.cs` + `IsccInstanceHasher.cs` (IDisposable + SafeHandle; both
    **`Finalize()` returns typed record**), `IsccException.cs`, `SmokeTests.cs` (**41 tests**),
    `ConformanceTests.cs` (**9 Theory tests, 50 vectors**), `testdata/data.json` (84KB vendored),
    `NativeMethods.g.cs` (csbindgen, 47 externs); `dotnet test` needs
    `-e LD_LIBRARY_PATH=<abs-path>/target/debug` (vstest host ignores env)
- `docs/howto/` — **9 files**: rust.md, python.md, nodejs.md, wasm.md, go.md, java.md, c-cpp.md,
    ruby.md (422 lines), **dotnet.md** (417 lines) ✅; `crates/iscc-ffi/examples/` has `iscc_sum.c`
    - `CMakeLists.txt` ✅
- `scripts/version_sync.py` — syncs workspace version across Cargo.toml, package.json, pom.xml,
    **Iscc.Lib.csproj** (added in iteration 9)
- `packages/go/codec.go` — codec enums, varnibble, header, base32/64, JsonToDataUrl,
    EncodeComponent, IsccDecompose, IsccDecode, **5 constants** (MetaTrimName, MetaTrimDescription,
    MetaTrimMeta, IoReadSize, TextNgramSize)
- `docs/c-ffi-api.md` — C FFI API reference (fully updated with iscc_gen_sum_code_v0)
- `crates/iscc-jni/java/src/main/java/io/iscc/iscc_lib/IsccLib.java` — Java class (subpath:
    `iscc_lib/`); has META_TRIM_META as `public static final int`
- `crates/iscc-ffi/src/lib.rs` line 3 — module docstring says "10 `gen_*_v0` functions"
- `crates/iscc-lib/benches/benchmarks.rs` — 277 lines; docstring says "all 10 gen\_\*\_v0"; has
    `bench_sum_code` (64KB+1MB using NamedTempFile); `criterion_group!` lists 12 benches

## Recurring Patterns

- **Incremental review**: compare `assessed-at` hash vs HEAD `--stat` first, then re-verify only
    affected sections. Always carry forward sections where no relevant files changed
- **CI has matrix jobs**: python-test runs as Python 3.10 + Python 3.14 (separate records); gate job
    `python` checks both pass. Count distinct job definitions, not run records.
- `gh run list` does NOT need `--repo` when running from within the workspace; but `--json` fields
    are needed to avoid GraphQL deprecation error
- **Verify claims independently**: review agents can make incorrect claims. Always grep for each
    missing symbol rather than trusting handoff verdict counts
- **Target may change**: always re-read target.md diff when doing incremental review; symbol counts
    and spec requirements can increase

## Current State (assessed-at: cbe38eed126e41065ab61b447ba230dbc911b75f)

- **IN_PROGRESS**: all 13 CI jobs green (run 22806295406); **C++ wrapper PARTIALLY DONE**
- **v0.2.0 released** — all 8 registries including RubyGems and NuGet pipeline in place
- **C++ wrapper (iteration 11)**: `packages/cpp/include/iscc/iscc.hpp` (681 lines, 32 symbols,
    RAII), `CMakeLists.txt`, `tests/` (52 passing tests, ASAN clean) ✅
- **C++ STILL MISSING**: CI job in `ci.yml`, `iscc.hpp` in FFI tarballs, vcpkg.json, portfile.cmake,
    conanfile.py, `packages/cpp/README.md`, `docs/howto/c-cpp.md` `iscc.hpp` section, README C++
    tab, `gen_mixed_code_v0` test
- **CI (run 22806295406)**: ALL SUCCESS — 13 jobs ✅ (no C++ job yet)

## NuGet Pipeline Details (iteration 10)

- `release.yml` has `nuget` input; `build-ffi` shared between FFI and NuGet
- `pack-nuget`: downloads cross-compiled FFI artifacts, organizes as `runtimes/<rid>/native/`, runs
    `dotnet pack -c Release`; cross-arch find uses `-path "*-${target}/*"` to avoid wrong lib
- `test-nuget`: installs from local nupkg, runs smoke test console app
- `publish-nuget`: idempotent (skips if version already on NuGet.org); uses `NUGET_API_KEY` secret
- **Manual action still needed**: NuGet.org account, NUGET_API_KEY secret, package ID reservation

## iscc-core v1.3.0 Conformance (FULLY RESOLVED — all bindings)

- 4 new test vectors vendored: test_0017–test_0020 in both `crates/iscc-lib/tests/data.json` and
    `packages/go/testdata/data.json` (50 total vectors)
- `data.json` has top-level `_metadata` object — Go uses `parseConformanceData()` to skip it; Rust
    `serde_json` silently ignores unknown fields
- Rust lib.rs assertion: 20; WASM conformance.rs line 66: 20 ✅; Go all 9 test files updated ✅

## Go Package Tier 1 Coverage (32/32 — COMPLETE)

All 32 symbols: 10 gen functions (including GenSumCodeV0), ConformanceSelftest, DataHasher,
InstanceHasher, 4 text utilities, SlidingWindow, AlgMinhash256, AlgCdcChunks, AlgSimhash,
SoftHashVideoV0, EncodeBase64, EncodeComponent, IsccDecode, IsccDecompose, JsonToDataUrl, **5
constants** (MetaTrimName, MetaTrimDescription, MetaTrimMeta, IoReadSize, TextNgramSize).

## Gotchas

- Go target requires pure Go (no WASM, no wazero, no binary artifacts)
- WASM constant name gotcha: `#[wasm_bindgen(js_name = "META_TRIM_NAME")]` exports uppercase
- `state.md` section order must include Go Bindings, README, Per-Crate READMEs sections
- Python ruff format check can fail in CI even if local `mise run check` passes
- **JCS gotcha**: Go `json.Marshal` passes current vectors. If future vectors have floats, a proper
    RFC 8785 JCS library may be needed
- **DataHasher/InstanceHasher API (Go)**: `Push([]byte)` + `Finalize(bits)` pattern
- **DataHasher/InstanceHasher API (Ruby)**: `RefCell<Option<inner>>` for interior mutability (Magnus
    `&self`); Ruby wrapper reopens native class, adds `update(data)` (chaining) +
    `finalize(bits: 64)`
- **GenIsccCodeV0 key details**: `encode_units` produces a bitfield; `wide` param always false in
    test vectors; SubType from content code's SubType (or NONE if absent); 5 conformance vectors
- **gen_sum_code_v0 WASM**: path-based I/O doesn't exist in browser WASM context — accepts
    Uint8Array/&[u8] instead; WASM and Go both solve this differently from Rust/Python/Node.js/C FFI
- **META_TRIM_META validation**: pre-decode fast check on Data-URL string length AND post-decode
    payload check; both needed in gen_meta_code_v0
- **Java META_TRIM_META**: added as compile-time `public static final int` (no JNI function needed)
- **C FFI IsccSumCodeResult**: struct-return pattern (not output-pointer); matches IsccDecodeResult
    pattern precisely; partial allocation failure handled (free iscc before returning null)
- **9 vs 10 distinction**: data.json has 9 conformance sections (no gen_sum_code_v0 vectors);
    iscc-lib has 10 gen functions. Test/conformance docstrings correctly say "9"; user-facing docs
    and benchmarks file say "10"
- **WASM count assertions**: `crates/iscc-wasm/tests/conformance.rs` has per-function
    `assert_eq!(tested, N, ...)` guards. When data.json gains new vectors, BOTH lib.rs AND the WASM
    conformance test must be updated.
- **Ruby JSON sort_keys no-op**: `JSON.generate(hash, sort_keys: true)` silently ignores `sort_keys`
    in Ruby's stdlib json gem. Use `JSON.generate(hash.sort.to_h)` instead.
- **alg_cdc_chunks API**: public fn returns `IsccResult<Vec<&[u8]>>` (validates
    `avg_chunk_size < 2`); internal callers use `alg_cdc_chunks_unchecked`
- **csbindgen**: `crates/iscc-ffi/build.rs` runs csbindgen on every `cargo build`, writing
    `packages/dotnet/Iscc.Lib/NativeMethods.g.cs` (929 lines, 47 externs, 6 structs). Class is
    `internal static unsafe partial class NativeMethods` — idiomatic wrappers in `IsccLib.cs` are
    the public surface. `dotnet test` requires `-e LD_LIBRARY_PATH=target/debug` (vstest host does
    not inherit shell env).
- **C# gen function return types**: All 10 `gen_*_v0` return typed records. BUT records are
    simplified — MetaCodeResult, TextCodeResult, InstanceCodeResult only carry `(string Iscc)`. Spec
    requires additional fields (Name/MetaHash/Description/Meta, Characters, DataHash/FileSize) which
    need C FFI struct changes first. NOT a blocking gap for current conformance criteria.
- **C# streaming Finalize() return types**: DONE — `IsccDataHasher.Finalize()` → `DataCodeResult`,
    `IsccInstanceHasher.Finalize()` → `InstanceCodeResult`. Completed in iteration 7.
- **C# ConsumeNativeStringArray**: private helper marshals `byte**` (NULL-terminated string array)
    from FFI, then calls `iscc_free_string_array`. Reuse for any future `byte**`-returning function.
- **C# ConsumeByteBuffer/ConsumeByteBufferArray**: marshal `IsccByteBuffer`/`IsccByteBufferArray`
    structs from FFI (null check + data copy + free). Used by AlgSimhash, AlgMinhash256,
    AlgCdcChunks, SoftHashVideoV0. Jagged-array inputs use `GCHandle` pinning.
- **C# empty-span null pointer**: `fixed` on empty `ReadOnlySpan<T>` produces NULL pointer in .NET.
    Use stack sentinel pattern (`byte sentinel; func(&sentinel, 0, ...)`). FIXED in all 7 locations.
- **C# ConformanceTests xUnit1026 warnings**: 9 Theory methods have unused `vectorName` parameter —
    pre-existing; serves test identification in output. Not worth suppressing.
- **C++ iscc.hpp**: `packages/cpp/include/iscc/iscc.hpp` — 681-line C++17 header-only wrapper. RAII
    guards: `UniqueString`, `UniqueStringArray`, `UniqueByteBuffer`, `UniqueByteBufferArray`.
    `IsccError : std::runtime_error`. `detail::safe_data()` for empty vector null protection.
    `detail::check_error()` and `detail::check_ptr()`. All 32 Tier 1 symbols in `namespace iscc`.
    `DataHasher` + `InstanceHasher` RAII classes (move-only). `cmake` and `g++` must be
    `apt-get install`ed in CI — they're not in the default ubuntu runner.
- **C++ nested vector gotcha**: `alg_simhash`, `soft_hash_video_v0`, `gen_video_code_v0` marshal
    nested `vector<vector<T>>` by extracting `.data()` from inner elements. Empty inner vectors
    yield `nullptr` — `safe_data()` covers top-level but not nested. Edge case for hardening.
