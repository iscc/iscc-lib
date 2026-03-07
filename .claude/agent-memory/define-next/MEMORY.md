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
- **C FFI / C# .NET**: No data.json loader (uses Rust core `conformance_selftest`).
- **data.json copies**: `crates/iscc-lib/tests/data.json` (primary) and
    `packages/go/testdata/data.json` (identical copy). Both must be updated together.

## Project Status

- **C#/.NET bindings: 30/32 symbols wrapped, streaming classes next (→32/32)**
- v0.2.0 released, 13 CI jobs green, all 7 existing bindings "met"
- C# bindings use P/Invoke over existing C FFI (`crates/iscc-ffi/`), not a new Rust binding crate
- Multi-step sequence: scaffold ✅ → CI job ✅ → csbindgen ✅ → wrappers (30/32 ✅) → streaming →
    conformance → release → docs
- .NET CI pattern: `actions/setup-dotnet@v4`, `cargo build -p iscc-ffi`, `dotnet build`,
    `dotnet test -e LD_LIBRARY_PATH=...`

## C# / .NET Binding Architecture

- Three layers: C FFI (existing) → P/Invoke (`NativeMethods.g.cs`, csbindgen) → Idiomatic wrapper
    (`IsccLib.cs`, PascalCase)
- Package lives in `packages/dotnet/` (not `crates/`), similar to Go's `packages/go/`
- DLL name for P/Invoke: `"iscc_ffi"` — .NET auto-resolves to `libiscc_ffi.so`/`.dylib`/`.dll`
- `[return: MarshalAs(UnmanagedType.U1)]` needed for C `bool` → C# `bool` marshaling
- Tests need `LD_LIBRARY_PATH=target/debug` (absolute path in devcontainer)

## csbindgen Integration Notes — DONE

- csbindgen ✅ in build.rs, NativeMethods.g.cs committed (929 lines, 47 externs, 6 structs)
- `#[unsafe(no_mangle)]` (Rust 2024) parsed correctly — no workaround needed
- `AllowUnsafeBlocks` already in .csproj

## C# Wrapper Scoping Pattern

- **Batch by marshaling complexity**: string→string → byte[]→string → struct returns → array inputs
    → streaming types (IDisposable)
- Actual wrapper sequence: 14 symbols (step 1 ✅) → 8 (step 2 ✅) → 4 (step 3 ✅) → 4 (step 4 ✅: algo
    primitives) → 2 (step 5: streaming IDisposable)
- **Jagged array marshaling**: `GCHandle.Alloc` per inner array, build pointer array, `fixed` on
    outer. Must free handles in `finally` block
- C# disallows pointer types as generic type args — string/byte array marshaling inlined per-method
- `ConsumeByteBuffer`/`ConsumeByteBufferArray` helpers follow `ConsumeNativeString` pattern

## C# Streaming Types Pattern (for advance agent)

- FFI streaming API: `*_new()` → `*_update()` → `*_finalize()` → `*_free()`
- P/Invoke already generated for all 8 functions (4 per hasher type)
- `FfiDataHasher`/`FfiInstanceHasher` are opaque empty structs in NativeMethods.g.cs
- SafeHandle pattern: private nested class, stores `FfiDataHasher*` as IntPtr, casts back for free
- `GetLastError()` and `ConsumeNativeString()` in IsccLib.cs need `private` → `internal` to reuse
- `ObjectDisposedException.ThrowIf(_handle.IsInvalid || _handle.IsClosed, this)` for disposed check

## CI/Release Patterns

- v0.2.0 released to all registries
- Release workflow has `workflow_dispatch` with per-registry checkboxes + `ffi` boolean
- `iscc-rb` requires `libclang-dev` — cannot remove `--exclude iscc-rb` from Rust CI job
- 6 smoke test jobs gate 6 publish jobs in release.yml

## Gotchas

- JNI function names encode Java package underscores as `_1`
- WASM howto uses `@iscc/wasm` (not `@iscc/iscc-wasm`). npm lib is `@iscc/lib`
- Windows GHA runners default to `pwsh` — always add `shell: bash` for bash syntax
- `dotnet test -e LD_LIBRARY_PATH=target/debug` with relative path fails in devcontainer — use
    absolute path

## Propagation Gotchas

- When vendoring new data.json vectors, ALL binding crates with hardcoded vector count assertions
    must be updated (Rust core + WASM)

## Documentation Drift Detection

- After major architecture changes, always verify README quickstart snippets against actual source
- After fixing doc drift, remaining work is ALL low-priority (C++, Swift, Kotlin bindings). CID loop
    approaches idle state after C# completion

## Post-C#-Symbols Roadmap

After 32/32 symbols: conformance tests → docs (howto/dotnet.md, README C# section) → NuGet publish →
version sync. Then only low-priority items remain (C++, Swift, Kotlin).
