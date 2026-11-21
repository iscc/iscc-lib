# CLAUDE.md -- Iscc.Lib (.NET)

C# / .NET wrapper around `iscc-ffi` via P/Invoke, providing an idiomatic .NET API for ISO 24138 ISCC
code generation.

## Package Role

- Idiomatic C# facade over the csbindgen-generated `NativeMethods` P/Invoke layer
- Loads the `iscc_ffi` shared library (`libiscc_ffi.so`, `libiscc_ffi.dylib`, `iscc_ffi.dll`) at
    runtime via .NET's native library resolver and NuGet RID conventions
- Does NOT implement any ISCC logic; all computation delegates through `iscc-ffi` to `iscc-lib`
- Published to NuGet as `Iscc.Lib`

## C FFI to C# Type Mapping

| C / Rust FFI type             | C# P/Invoke type       | C# public API type     | Notes                                     |
| ----------------------------- | ---------------------- | ---------------------- | ----------------------------------------- |
| `const char *` (input)        | `byte*`                | `string`               | Converted via `ToNativeUtf8` (NUL-termed) |
| `char *` (output)             | `byte*`                | `string`               | Consumed via `ConsumeNativeString`        |
| `const uint8_t *` + `size_t`  | `byte*` + `nuint`      | `ReadOnlySpan<byte>`   | Pinned with `fixed`                       |
| `const int32_t *` + `size_t`  | `int*` + `nuint`       | `ReadOnlySpan<int>`    | Pinned with `fixed`                       |
| `uint32_t`                    | `uint`                 | `uint`                 | `bits` parameters, feature values         |
| `bool`                        | `[MarshalAs(U1)] bool` | `bool`                 | Single-byte C bool                        |
| `size_t`                      | `nuint`                | implicit (from Length) | Buffer/array lengths                      |
| `uint64_t`                    | `ulong`                | `ulong`                | `filesize` in SumCodeResult               |
| `IsccByteBuffer`              | `IsccByteBuffer`       | `byte[]`               | Consumed via `ConsumeByteBuffer`          |
| `IsccByteBufferArray`         | `IsccByteBufferArray`  | `byte[][]`             | Consumed via `ConsumeByteBufferArray`     |
| `IsccSumCodeResult`           | `IsccSumCodeResult`    | `SumCodeResult`        | Struct with `ok` flag, freed after copy   |
| `IsccDecodeResult`            | `IsccDecodeResult`     | `DecodeResult`         | Struct with `ok` flag, freed after copy   |
| `FfiDataHasher*` (opaque)     | `FfiDataHasher*`       | `IsccDataHasher`       | Wrapped in `SafeHandle`                   |
| `FfiInstanceHasher*` (opaque) | `FfiInstanceHasher*`   | `IsccInstanceHasher`   | Wrapped in `SafeHandle`                   |

## Build Commands

```bash
# Prerequisites: cargo (for iscc-ffi), dotnet SDK 8.0

# Build the native library (required before .NET build/test)
cargo build -p iscc-ffi

# Build the .NET library
dotnet build packages/dotnet/Iscc.Lib/Iscc.Lib.csproj

# Build and run tests (requires native library in LD_LIBRARY_PATH)
dotnet build packages/dotnet/Iscc.Lib.Tests/Iscc.Lib.Tests.csproj
dotnet test packages/dotnet/Iscc.Lib.Tests/ -e LD_LIBRARY_PATH=$(pwd)/target/debug

# Pack NuGet package (release build, CI populates runtimes/ first)
dotnet pack packages/dotnet/Iscc.Lib/ -c Release -o nupkg
```

## Test Patterns

`Iscc.Lib.Tests/testdata/data.json` and `data-edge.json` are vendored copies of the fixtures in
`crates/iscc-lib/tests/`.

### Smoke tests (`SmokeTests.cs`)

- One `[Fact]` per public method verifying basic functionality
- Constants: assert exact expected values (`MetaTrimName` = 128, etc.)
- Text utilities: verify string transformations
- Gen functions: verify returned ISCC strings start with `"ISCC:"`
- Streaming hashers: verify `IsccDataHasher` / `IsccInstanceHasher` match their non-streaming
    counterparts, test chunked update, test dispose idempotency, test double-finalize throws
- Error handling: verify `IsccException` construction

### Conformance tests (`ConformanceTests.cs`)

- `[Theory]` + `[MemberData]` pattern for parameterized tests from `testdata/data.json`
- One test method per gen function with `data.json` vectors, plus `EncodeSeq` / `DecodeSeq` and
    `SeqRoundTripBytes` (`EncodeSeq(DecodeSeq(x)) == x`)
- Static `Lazy<JsonElement>` caches the parsed JSON across test methods
- `DecodeStream` helper converts `"stream:<hex>"` format to `byte[]`
- `PrepareMeta` helper handles null / string / JSON-object meta parameter variants
- Asserts exact ISCC string equality against expected outputs

### Edge-case tests (`DataEdgeTests.cs`)

- Runs every vector of `testdata/data-edge.json` through a section → caller table; `except.dotnet`
    is the only deviation and an unknown section fails. A rejection must be an `IsccException` from
    the raw call; no vector count is hardcoded

## Publishing (NuGet)

- **Package ID:** `Iscc.Lib`
- **Readme:** `packages/dotnet/README.md` is packed into the `.nupkg` as the package readme
- **Target framework:** `net8.0`
- **Version source:** `<Version>` in `Iscc.Lib.csproj`, synced with workspace version via
    `mise run version:sync`
- **Native library bundling:** CI downloads the FFI artifact for every supported RID, maps them to
    NuGet RID directories under `runtimes/<rid>/native/`, then runs `dotnet pack`
- **Supported RIDs:** `linux-x64`, `linux-arm64`, `osx-arm64`, `osx-x64`, `win-x64`
- **Smoke testing:** CI creates a fresh console project on ubuntu, macOS, and Windows, installs the
    packed `.nupkg` from a local source, runs `ConformanceSelftest()` as a sanity check
- **Publishing:** `dotnet nuget push` to `nuget.org` with `NUGET_API_KEY` secret (not OIDC)
- **Duplicate guard:** CI checks the NuGet API for existing version before pushing

## P/Invoke Patterns and Conventions

### String marshalling

- **Input strings:** converted to NUL-terminated UTF-8 byte arrays via `ToNativeUtf8()` (which
    throws `IsccException` on an embedded NUL, since the native side would truncate there), then
    pinned with `fixed` and passed as `byte*`. Optional strings pass `null` when the `byte[]?` is
    null (the `fixed` statement produces a null pointer for null arrays).
- **Output strings:** native returns `byte*` (heap-allocated by Rust). `ConsumeNativeString`
    marshals to `string` via `Marshal.PtrToStringUTF8`, then frees via `iscc_free_string`. Throws
    `IsccException` if the pointer is null.

### Memory ownership

- **Rust allocates, Rust frees.** Every native return value must be freed through the corresponding
    `iscc_free_*` function. Managed copies are made before freeing.
- Strings: `iscc_free_string`
- String arrays (NULL-terminated): `iscc_free_string_array`
- Byte buffers (`IsccByteBuffer`): `iscc_free_byte_buffer`
- Byte buffer arrays (`IsccByteBufferArray`): `iscc_free_byte_buffer_array`
- Struct results: `iscc_free_sum_code_result`, `iscc_free_decode_result`

### Error handling

- Native functions return `NULL` (pointers) or set `ok = false` (structs) on error
- `ConsumeNativeString` checks for null and throws `IsccException(GetLastError())`
- Struct results check `result.ok` before reading fields
- `GetLastError()` reads from `iscc_last_error()` (borrowed pointer, not freed by caller)

### Opaque handle pattern (streaming hashers)

- `FfiDataHasher*` / `FfiInstanceHasher*` are opaque pointers from the native library
- Wrapped in private `SafeHandle` subclasses (`DataHasherHandle`, `InstanceHasherHandle`)
- `SafeHandle.ReleaseHandle()` calls `iscc_data_hasher_free` / `iscc_instance_hasher_free`
- The public `IsccDataHasher` / `IsccInstanceHasher` classes implement `IDisposable`
- A `_finalized` flag prevents double-finalize (throws `InvalidOperationException`)
- `ObjectDisposedException.ThrowIf` guards all methods after disposal

### Array parameter marshalling

- **Jagged arrays** (`int[][] frameSigs`, `byte[][] digests`, `string[] codes`/`units`): all
    jagged-array methods go through the private `PinnedRows` helper in `IsccLib.cs` (`Pin` for
    jagged arrays, `PinStrings` converts via `ToNativeUtf8` first) inside a `using`. It throws
    `ArgumentNullException` (parameter name + `name[i]` in the message) for a null array or element
    before pinning anything, pins each row with `GCHandle.Alloc(..., Pinned)`, frees already pinned
    handles if pinning fails, and frees all handles on `Dispose`. Call sites `fixed` its `Pointers`
    (`nint[]`, cast to `int**`/`byte**`) and pass `Count` as the length. Never add a `GCHandle` loop
    at a call site
- **Parallel length arrays** (`frame_lens`, `digest_lens`): `PinnedRows.Lengths` (`nuint[]`), passed
    with a second `fixed`

### Empty spans

- `fixed` over an empty span yields a null pointer. The C FFI treats a NULL pointer with length 0 as
    an empty input on every pointer+length parameter, so call sites pass `fixed` pointers with no
    special case for empty spans, and an empty input surfaces the core's own error (if any).
- `PinnedRows` sizes `Pointers` and `Lengths` to `Count`, so a jagged `[]` passes a NULL pointer
    with count 0, and an empty inner row passes its own pointer with length 0

## Common Pitfalls

- **Forgetting to build `iscc-ffi` first:** `dotnet build` succeeds, but tests fail at runtime with
    `DllNotFoundException`. Always `cargo build -p iscc-ffi` before running .NET tests.
- **LD_LIBRARY_PATH on Linux:** the test runner does not find `libiscc_ffi.so` unless
    `LD_LIBRARY_PATH` includes `target/debug` (or `target/release`). CI passes it via `-e`.
- **NativeMethods.g.cs is auto-generated:** never edit this file. Regenerate it from `iscc-ffi`
    using csbindgen when the FFI surface changes.
- **Empty spans produce null pointers:** `fixed (byte* p = ReadOnlySpan<byte>.Empty)` yields `null`.
    Pass it on with length 0; the FFI accepts `(NULL, 0)`. Only a NULL pointer with a non-zero
    length is an error.
- **GCHandle leaks:** a missing `Free()` pins objects permanently. Pin jagged arrays only through
    `PinnedRows` in a `using`, which frees every handle, also when pinning fails halfway.
- **Bool marshalling:** C `bool` is 1 byte, but .NET `bool` defaults to 4 bytes.
    `[MarshalAs(UnmanagedType.U1)]` is required on both parameters and return values.
- **Version drift:** the `<Version>` in `Iscc.Lib.csproj` must stay in sync with the workspace
    `Cargo.toml` version. Use `mise run version:sync` to update all manifests, and
    `mise run version:check` to verify.
- **Test dependencies are pinned via a committed lock file:** `Iscc.Lib.Tests.csproj` pins exact
    `PackageReference` versions and sets `RestorePackagesWithLockFile`, and CI restores with
    `--locked-mode`, so a `PackageReference` version change must regenerate `packages.lock.json` in
    the same commit:
    `dotnet restore packages/dotnet/Iscc.Lib.Tests/Iscc.Lib.Tests.csproj --force-evaluate`.
- **runtimes/ directory is gitignored:** native libraries under `runtimes/` are populated by CI
    during the pack step. Local development uses `LD_LIBRARY_PATH` instead of bundled runtimes.
- **Struct result lifetime:** `IsccSumCodeResult` and `IsccDecodeResult` contain native pointers.
    Copy all data to managed types in a `try` block, then free the struct in `finally`. Do not hold
    references to native pointers after freeing.
