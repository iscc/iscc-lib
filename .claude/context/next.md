# Next Work Package

## Step: C# streaming hasher Finalize() return types

## Goal

Change `IsccDataHasher.Finalize()` and `IsccInstanceHasher.Finalize()` to return typed result
records (`DataCodeResult` / `InstanceCodeResult`) instead of raw `string`, aligning streaming
hashers with the structured return pattern already used by all 10 gen functions.

## Scope

- **Create**: (none)
- **Modify**:
    - `packages/dotnet/Iscc.Lib/IsccDataHasher.cs` — change `Finalize()` return type from `string` to
        `DataCodeResult`, wrap the consumed native string in `new DataCodeResult(...)`, update
        docstring
    - `packages/dotnet/Iscc.Lib/IsccInstanceHasher.cs` — change `Finalize()` return type from `string`
        to `InstanceCodeResult`, wrap the consumed native string in `new InstanceCodeResult(...)`,
        update docstring
    - `packages/dotnet/Iscc.Lib.Tests/SmokeTests.cs` — update test methods to use `.Iscc` property on
        results (tests are excluded from 3-file limit)
- **Reference**:
    - `packages/dotnet/Iscc.Lib/Results.cs` — `DataCodeResult` and `InstanceCodeResult` record
        definitions (already exist, read-only)
    - `packages/dotnet/Iscc.Lib/IsccLib.cs` — for the pattern used by `GenDataCodeV0` /
        `GenInstanceCodeV0` (they already return typed records)

## Not In Scope

- Adding extra fields (DataHash, FileSize) to `DataCodeResult` or `InstanceCodeResult` — these
    require C FFI struct changes first and are a separate future step
- Extracting SafeHandle subclasses to a separate `Native/SafeHandles.cs` file
- Documentation (`docs/howto/dotnet.md`, `packages/dotnet/README.md`, README C# section)
- NuGet publish pipeline or version sync
- Modifying `Results.cs` — the record types already have the correct definitions

## Implementation Notes

The change is mechanical — each hasher's `Finalize()` method already calls
`IsccLib.ConsumeNativeString(result)` which returns a `string`. Wrap that string in the record
constructor:

**IsccDataHasher.cs** (line 42-56):

```csharp
// Change return type and docstring
/// <summary>Finalize the hasher and return the ISCC Data-Code result.</summary>
public DataCodeResult Finalize(uint bits = 64)
{
    // ... existing validation unchanged ...
    _finalized = true;
    unsafe
    {
        byte* result = NativeMethods.iscc_data_hasher_finalize(
            (FfiDataHasher*)(void*)_handle.DangerousGetHandle(), bits);
        return new DataCodeResult(IsccLib.ConsumeNativeString(result));
    }
}
```

**IsccInstanceHasher.cs** — identical pattern with `InstanceCodeResult`.

**SmokeTests.cs** — test methods that need updates:

1. `DataHasher_MatchesGenDataCodeV0` (line 358-359): `string result = hasher.Finalize()` →
    `var result = hasher.Finalize()`, assert `Assert.Equal(expected.Iscc, result.Iscc)`
2. `DataHasher_ChunkedUpdate_MatchesSingleUpdate` (lines 367-374): both `Finalize()` calls return
    records now, use `var` and compare `.Iscc` properties
3. `InstanceHasher_MatchesGenInstanceCodeV0` (line 384-385): same pattern as DataHasher test
4. `DataHasher_UpdateAfterFinalize_Throws` and `DataHasher_FinalizeAfterFinalize_Throws`: these call
    `Finalize()` but don't inspect the return value — no change needed (the lambda throws before
    returning)

## Verification

- `cargo build -p iscc-ffi` succeeds (native library unchanged)
- `dotnet build packages/dotnet/Iscc.Lib/` succeeds with 0 errors
- `dotnet test packages/dotnet/Iscc.Lib.Tests/ -e LD_LIBRARY_PATH=$(pwd)/target/debug` — 91 passed,
    0 failed
- `grep -c 'DataCodeResult Finalize' packages/dotnet/Iscc.Lib/IsccDataHasher.cs` returns 1
- `grep -c 'InstanceCodeResult Finalize' packages/dotnet/Iscc.Lib/IsccInstanceHasher.cs` returns 1
- `grep -c 'string Finalize' packages/dotnet/Iscc.Lib/IsccDataHasher.cs packages/dotnet/Iscc.Lib/IsccInstanceHasher.cs`
    returns 0

## Done When

All 91 .NET tests pass with both `Finalize()` methods returning typed result records instead of
strings, and no `string Finalize` signatures remain in the hasher files.
