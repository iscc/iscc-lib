# Next Work Package

## Step: C# streaming hashers (IsccDataHasher, IsccInstanceHasher) — 32/32 symbols

## Goal

Implement the final 2 of 32 Tier 1 symbols for the C# binding: `IsccDataHasher` and
`IsccInstanceHasher` streaming classes. This completes the full Tier 1 API surface for C#/.NET.

## Scope

- **Create**: `packages/dotnet/Iscc.Lib/IsccDataHasher.cs`,
    `packages/dotnet/Iscc.Lib/IsccInstanceHasher.cs`
- **Modify**: `packages/dotnet/Iscc.Lib/IsccLib.cs` (change `GetLastError` and `ConsumeNativeString`
    visibility from `private` to `internal`), `packages/dotnet/Iscc.Lib.Tests/SmokeTests.cs` (add
    streaming tests)
- **Reference**: `crates/iscc-ffi/src/lib.rs` (lines 1279-1478 — FFI streaming API),
    `packages/dotnet/Iscc.Lib/NativeMethods.g.cs` (lines 631-727, 910-925 — P/Invoke declarations
    and opaque struct defs), `packages/dotnet/Iscc.Lib/IsccLib.cs` (existing marshaling patterns)

## Not In Scope

- Structured result records (`MetaCodeResult`, `TextCodeResult`, etc.) — separate future step
- Conformance tests (`ConformanceTests.cs` + `testdata/data.json`) — separate future step
- Documentation (`docs/howto/dotnet.md`, `packages/dotnet/README.md`) — separate future step
- NuGet publish job in `release.yml` — separate future step
- Refactoring existing `IsccLib.cs` wrappers — no changes to working code
- `Stream`-based overloads (accepting `System.IO.Stream` instead of `ReadOnlySpan<byte>`) — nice to
    have but not in the 32 Tier 1 symbols
- SafeHandle subclass in a separate `Native/SafeHandles.cs` file — keep SafeHandle as private nested
    class inside each hasher for simplicity

## Implementation Notes

### Pattern: SafeHandle + IDisposable

Each streaming class wraps an opaque native pointer via a private nested `SafeHandle` subclass for
deterministic cleanup with finalization safety net:

```csharp
public sealed class IsccDataHasher : IDisposable
{
    private readonly DataHasherHandle _handle;
    private bool _finalized;

    public IsccDataHasher() { ... }
    public void Update(ReadOnlySpan<byte> data) { ... }
    public string Finalize(uint bits = 64) { ... }
    public void Dispose() { _handle.Dispose(); }

    private sealed class DataHasherHandle : SafeHandle { ... }
}
```

### SafeHandle subclass

Use a private nested class (e.g., `DataHasherHandle : SafeHandle`) that:

- Inherits from `SafeHandle` (from `System.Runtime.InteropServices`)
- Stores the native `FfiDataHasher*` as `IntPtr` (SafeHandle base class field `handle`)
- Override `IsInvalid` → `handle == IntPtr.Zero`
- Override `ReleaseHandle()` → casts `IntPtr` back to `FfiDataHasher*` via
    `(FfiDataHasher*)(void*)handle` and calls `NativeMethods.iscc_data_hasher_free()`
- Constructor accepts `FfiDataHasher*` from `iscc_data_hasher_new()`, stores as `(IntPtr)ptr`

### P/Invoke declarations already exist in NativeMethods.g.cs

- `iscc_data_hasher_new()` → `FfiDataHasher*`
- `iscc_data_hasher_update(FfiDataHasher*, byte*, nuint)` → `bool`
- `iscc_data_hasher_finalize(FfiDataHasher*, uint)` → `byte*`
- `iscc_data_hasher_free(FfiDataHasher*)` → `void`
- Same 4 for `iscc_instance_hasher_*`
- `FfiDataHasher` and `FfiInstanceHasher` are empty structs (opaque pointers)

### Key behaviors

- `Update()` after `Finalize()` → `InvalidOperationException("Hasher already finalized")`
- `Update()` after `Dispose()` → `ObjectDisposedException`
- `Finalize()` after `Finalize()` → `InvalidOperationException("Hasher already finalized")`
- `Finalize()` after `Dispose()` → `ObjectDisposedException`
- `Dispose()` is idempotent (SafeHandle handles this)
- Streaming results must match `GenDataCodeV0`/`GenInstanceCodeV0` for the same input data

### Error helper reuse

`GetLastError()` and `ConsumeNativeString()` are `private` in `IsccLib.cs`. Change both to
`internal` so the hasher classes (in the same assembly) can reuse them. This is a one-word
visibility change each — `private` → `internal`. No behavioral modification.

The hasher's `Finalize()` method calls `iscc_data_hasher_finalize()` which returns `byte*` (an ISCC
string). Use `ConsumeNativeString()` to marshal it and free the native pointer. On null return,
`ConsumeNativeString` already throws `IsccException(GetLastError())`.

### IntPtr ↔ typed pointer casting

SafeHandle stores `IntPtr`. NativeMethods uses `FfiDataHasher*`. Cast pattern:

```csharp
// Store: (IntPtr)ptr  (where ptr is FfiDataHasher*)
// Retrieve: (FfiDataHasher*)(void*)handle
```

### Disposed check pattern

Before any operation on the native handle:

```csharp
ObjectDisposedException.ThrowIf(_handle.IsInvalid || _handle.IsClosed, this);
```

This covers both never-initialized and already-disposed cases.

### InstanceHasher note

`gen_instance_code_v0` always produces 256-bit output regardless of `bits` parameter. The
`InstanceHasher.Finalize(bits)` should still accept the parameter and pass it through — the Rust
core handles the semantics.

### Tests to add (in SmokeTests.cs)

1. `DataHasher_MatchesGenDataCodeV0` — feed "Hello World" bytes via Update, verify Finalize result
    matches `GenDataCodeV0` with same data
2. `DataHasher_ChunkedUpdate_MatchesSingleUpdate` — feed data in 2 chunks, verify same result as
    single update
3. `InstanceHasher_MatchesGenInstanceCodeV0` — same equivalence check
4. `DataHasher_DisposeIsIdempotent` — call Dispose twice, no crash
5. `DataHasher_UpdateAfterFinalize_Throws` — verify `InvalidOperationException`
6. `DataHasher_FinalizeAfterFinalize_Throws` — verify `InvalidOperationException`

## Verification

- `cargo build -p iscc-ffi` succeeds (no FFI changes, just confirming lib compiles)
- `dotnet build packages/dotnet/Iscc.Lib/Iscc.Lib.csproj` — 0 errors, 0 warnings
- `dotnet test packages/dotnet/Iscc.Lib.Tests/Iscc.Lib.Tests.csproj -e LD_LIBRARY_PATH=/workspace/iscc-lib/target/debug`
    — all tests pass (35 existing + ~6 new streaming tests)
- `grep -c 'public sealed class Iscc' packages/dotnet/Iscc.Lib/IsccDataHasher.cs` returns 1
- `grep -c 'public sealed class Iscc' packages/dotnet/Iscc.Lib/IsccInstanceHasher.cs` returns 1
- `mise run check` — all hooks pass (formatting + lint)

## Done When

All verification criteria pass: both streaming hasher classes compile, pass tests including
equivalence checks against `GenDataCodeV0`/`GenInstanceCodeV0`, completing 32/32 Tier 1 symbols.
