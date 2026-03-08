# Define-Next Agent Memory

Scoping decisions, estimation patterns, and architectural knowledge accumulated across CID
iterations.

**Size budget:** Keep under 200 lines. Archive stale entries to `MEMORY-archive.md`.

## Scope Calibration Principles

- Critical issues always take priority regardless of feature trajectory
- Multiple small issues in the same crate are a natural batch (e.g., 3 fixes touching 2 files)
- Doc files are excluded from the 3-file modification limit — can batch all 6 howto guides in one
    step since they follow identical patterns
- When CI is red, formatting/lint fixes are always the first priority regardless of handoff "Next"
- Prefer concrete deliverables over research tasks when both are available
- State assessments can go stale — always verify claimed gaps by reading the actual files
- New Tier 1 symbols: always implement in Rust core first, then propagate to bindings in separate
    steps. Core + tests in one step, bindings in subsequent steps
- Header-only wrappers (C++) are mechanical: all 32 symbols can fit in one step since each wrapper
    function is 5-10 lines of boilerplate

## Signature Change Propagation

- When a Rust core function signature changes, ALL Rust-based binding crates must be updated in the
    SAME step to keep CI green
- WASM binding has its OWN inline `gen_sum_code_v0` (no filesystem in WASM)
- Go binding is pure Go — completely independent of Rust core signatures

## Architecture Decisions

- Go bindings are pure Go (no WASM, no wazero, no binary artifacts)
- All binding conformance tests follow the same structure: load data.json, iterate per-function
    groups, decode inputs per signature, compare `.iscc` output
- `gen_iscc_code_v0` test vectors have no `wide` parameter — always pass `false`
- `"stream:<hex>"` prefix denotes hex-encoded byte data for Data/Instance-Code tests

## Conformance Vector Loader Differences (critical for data.json updates)

- **Rust core** (`conformance.rs`): Uses `serde_json::Value`, auto-discovers new vectors.
- **Go** (`conformance.go`): Uses `map[string]map[string]vectorEntry` — parses ALL top-level keys.
    **BREAKS** on non-vector entries like `_metadata`. Must use `json.RawMessage` intermediate step.
- **C# .NET**: `ConformanceTests.cs` uses `System.Text.Json`, loads from vendored
    `testdata/data.json`. Must skip `_metadata` key.
- **C FFI**: No data.json loader (uses Rust core `conformance_selftest`).
- **C++ wrapper**: Same as C FFI — uses `conformance_selftest()` call, no data.json parsing.
- **data.json copies**: `crates/iscc-lib/tests/data.json` (primary),
    `packages/go/testdata/data.json`, and `packages/dotnet/Iscc.Lib.Tests/testdata/data.json` (all
    identical). Must be updated together.

## Project Status

- **C#/.NET bindings complete** (32/32 symbols, typed returns, docs, version sync, NuGet pipeline)
- v0.2.0 released, 13 CI jobs green, all 8 bindings CI green
- **Current work: C++ idiomatic header-only wrapper** (`normal` priority issue)
- After C++: only `low`-priority items remain (Swift, Kotlin, logos) — CID loop approaches idle

## C++ Wrapper Architecture

- Header-only C++17 wrapper over existing C FFI (`iscc.h`)
- Lives in `packages/cpp/` (like Go in `packages/go/`, .NET in `packages/dotnet/`)
- No separate Rust crate needed — purely C++ header + CMake
- Distribution: vcpkg port, Conan recipe, bundled with FFI release tarballs
- C FFI returns ISCC string for most gen functions (not full structs) — C++ result types mostly have
    only `iscc` field. `SumCodeResult` and `DecodeResult` are the exceptions with extra fields
- Multi-step sequence: **iscc.hpp + tests** → CI job → release integration → pkg managers → docs

## C++ Scoping: Step Breakdown

1. ✅ `iscc.hpp` + CMake + smoke test (done — 52 tests, ASAN clean)
2. ✅ CI job in `ci.yml` (compile + ASAN) — done, 14/14 jobs green
3. ✅ Release: bundle `iscc.hpp` in FFI tarballs — done
4. ✅ Documentation: README, howto/c-cpp.md update, root README C++ sections — done
5. ✅ Code quality: gen_mixed_code_v0 test + nested vector null-safety — done (53 tests)
6. ✏️ Package managers: vcpkg.json, portfile.cmake, conanfile.py — final C++ step

## CI/Release Patterns

- v0.2.0 released to all registries
- Release workflow has `workflow_dispatch` with per-registry checkboxes + `ffi` boolean
- `iscc-rb` requires `libclang-dev` — cannot remove `--exclude iscc-rb` from Rust CI job
- 6 smoke test jobs gate 6 publish jobs in release.yml
- FFI tarball staging: Unix uses `cp` + `tar czf`, Windows uses `Copy-Item` + `Compress-Archive`.
    Both need parallel changes when adding files. `publish-ffi` globs `iscc-ffi-v*.*` — auto-picks
    up new files in the staged directory

## Gotchas

- JNI function names encode Java package underscores as `_1`
- WASM howto uses `@iscc/wasm` (not `@iscc/iscc-wasm`). npm lib is `@iscc/lib`
- Windows GHA runners default to `pwsh` — always add `shell: bash` for bash syntax
- `cbindgen` export prefix `iscc_` on types but not on functions — C++ wrapper must use
    `iscc_FfiDataHasher`, `iscc_IsccSumCodeResult` etc. for type names

## Propagation Gotchas

- When vendoring new data.json vectors, ALL binding crates with hardcoded vector count assertions
    must be updated (Rust core + WASM)

## FFI Tarball Layout (for vcpkg portfile)

- Tarballs named `iscc-ffi-v{VERSION}-{TARGET}.tar.gz` (5 platforms)
- Flat layout inside: `iscc.hpp`, `iscc.h`, `libiscc_ffi.so`/`.dylib`/`.dll`, static lib
- vcpkg triplet → Rust target mapping needed: `x64-linux` → `x86_64-unknown-linux-gnu`, etc.
- Version 0.2.0 is current

## Documentation Drift Detection

- After major architecture changes, always verify README quickstart snippets against actual source
- After C++ completion, remaining work is ALL low-priority (Swift, Kotlin bindings). CID loop
    approaches idle state
