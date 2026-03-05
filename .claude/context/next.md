# Next Work Package

## Step: Set up csbindgen to generate full .NET P/Invoke surface

## Goal

Add csbindgen integration to `crates/iscc-ffi/` that auto-generates `NativeMethods.g.cs` with
P/Invoke declarations for all FFI functions, giving the .NET binding the complete raw interop layer
without manual declarations. Part of the "Implement C# / .NET bindings via csbindgen" `normal`
issue.

## Scope

- **Create**: `crates/iscc-ffi/build.rs` — csbindgen builder configuration
- **Modify**: `crates/iscc-ffi/Cargo.toml` — add `csbindgen` as build-dependency
- **Modify**: `packages/dotnet/Iscc.Lib/Iscc.Lib.csproj` — add
    `<AllowUnsafeBlocks>true</AllowUnsafeBlocks>`
- **Generated output** (committed): `packages/dotnet/Iscc.Lib/NativeMethods.g.cs`
- **Reference**: `crates/iscc-ffi/src/lib.rs`, `crates/iscc-ffi/include/iscc.h`,
    `.claude/context/specs/dotnet-bindings.md`

## Not In Scope

- Refactoring `IsccLib.cs` to delegate to `NativeMethods` — keep the existing inline P/Invoke for
    `ConformanceSelftest`; duplicate declarations across classes are harmless in C#
- Idiomatic C# wrappers (PascalCase methods, record types, SafeHandle, IDisposable) — those build on
    top of NativeMethods in a future step
- Conformance tests against `data.json`
- CI freshness check for the generated file (add later, similar to cbindgen header check)
- Release pipeline / NuGet packaging
- `generate-bindings.sh` convenience script — can be added later

## Implementation Notes

### csbindgen Configuration

Use `csbindgen` (v1.9.7) in `build.rs` with the builder API:

```rust
fn main() {
    csbindgen::Builder::default()
        .input_extern_file("src/lib.rs")
        .csharp_dll_name("iscc_ffi")
        .csharp_class_name("NativeMethods")
        .csharp_namespace("Iscc.Lib")
        .generate_csharp_file("../../packages/dotnet/Iscc.Lib/NativeMethods.g.cs")
        .unwrap();
}
```

Key configuration points:

- **Namespace**: `Iscc.Lib` (same namespace as `IsccLib.cs` for easy access)
- **DLL name**: `"iscc_ffi"` — .NET auto-resolves platform-specific names
- **Output path**: `../../packages/dotnet/Iscc.Lib/NativeMethods.g.cs` relative to iscc-ffi crate
    root

### Rust 2024 Edition Compatibility

The FFI crate uses `#[unsafe(no_mangle)]` (Rust 2024 edition syntax, ~48 occurrences). csbindgen
parses `extern "C" fn` signatures — the `no_mangle` attribute form should not affect parsing. If
csbindgen fails to parse `#[unsafe(no_mangle)]`, a workaround is to use `input_bindgen_file` mode
with the existing `iscc.h` header instead, or to try `input_extern_file` with multiple source files.

### Type Mapping Expected

csbindgen should auto-map:

- `*const c_char` / `*mut c_char` → `byte*` (string pointers)
- `bool` → `[MarshalAs(UnmanagedType.U1)] bool` or `byte`
- `u32` → `uint`
- `u8` → `byte`
- `usize` / `uintptr_t` → `nuint` or `UIntPtr`
- `*const *const c_char` → `byte**` (double pointers for arrays)
- Struct returns (e.g., `IsccSumCodeResult`) → C# struct with `[StructLayout]`
- Opaque pointer types (`*mut FfiDataHasher`) → typed pointer or `IntPtr`

### AllowUnsafeBlocks

csbindgen generates `unsafe` static methods (raw pointers are inherently unsafe in C#). Add
`<AllowUnsafeBlocks>true</AllowUnsafeBlocks>` to `Iscc.Lib.csproj` `<PropertyGroup>` to enable
compilation.

### Cargo.toml Change

Add only:

```toml
[build-dependencies]
csbindgen = "1.9.7"
```

No other dependency changes needed. The workspace `[workspace.dependencies]` does NOT need a
csbindgen entry since it's only used as a build-dep for one crate.

### Commit the Generated File

After `cargo build -p iscc-ffi` generates `NativeMethods.g.cs`, commit it so that .NET builds work
without requiring the Rust toolchain. This follows the same pattern as the committed `iscc.h`
header.

### FFI Source Structure

All FFI code is in a single file: `crates/iscc-ffi/src/lib.rs` (no submodules). The file contains
~48 `extern "C"` functions covering constants, gen functions, text utilities, encoding, codec,
algorithm primitives, streaming hashers, and free/error functions.

## Verification

- `cargo build -p iscc-ffi` succeeds and creates `packages/dotnet/Iscc.Lib/NativeMethods.g.cs`
- `NativeMethods.g.cs` contains P/Invoke declarations for at least: `iscc_gen_meta_code_v0`,
    `iscc_gen_data_code_v0`, `iscc_gen_instance_code_v0`, `iscc_decode`,
    `iscc_conformance_selftest`, `iscc_free_string`
- `dotnet build packages/dotnet/Iscc.Lib.Tests/Iscc.Lib.Tests.csproj` succeeds (generated code
    compiles alongside existing IsccLib.cs)
- `dotnet test packages/dotnet/Iscc.Lib.Tests/Iscc.Lib.Tests.csproj -e LD_LIBRARY_PATH=target/debug`
    passes (existing ConformanceSelftest smoke test still works)
- `cargo clippy -p iscc-ffi -- -D warnings` is clean

## Done When

All five verification criteria pass — csbindgen generates the complete P/Invoke surface, the .NET
project compiles with the generated code, and the existing smoke test still passes.
