## 2026-03-05 — Set up csbindgen to generate full .NET P/Invoke surface

**Done:** Added csbindgen (v1.9.7) integration to `crates/iscc-ffi/` that auto-generates
`NativeMethods.g.cs` with complete P/Invoke declarations for all 43 FFI functions. The generated
file includes struct definitions (`IsccByteBuffer`, `IsccByteBufferArray`, `IsccSumCodeResult`,
`IsccDecodeResult`, `FfiDataHasher`, `FfiInstanceHasher`) with proper `[StructLayout]` attributes
and type mappings.

**Files changed:**

- `crates/iscc-ffi/Cargo.toml`: Added `[build-dependencies] csbindgen = "1.9.7"`
- `crates/iscc-ffi/build.rs`: New file — csbindgen builder configuration targeting
    `NativeMethods.g.cs` in the dotnet project
- `packages/dotnet/Iscc.Lib/Iscc.Lib.csproj`: Added `<AllowUnsafeBlocks>true</AllowUnsafeBlocks>` to
    enable compilation of csbindgen's unsafe pointer code
- `packages/dotnet/Iscc.Lib/NativeMethods.g.cs`: Generated file — 929 lines of P/Invoke declarations
    covering all FFI functions, structs, and opaque types

**Verification:**

- `cargo build -p iscc-ffi` — succeeds, generates `NativeMethods.g.cs` (929 lines)
- `NativeMethods.g.cs` contains all required functions: `iscc_gen_meta_code_v0`,
    `iscc_gen_data_code_v0`, `iscc_gen_instance_code_v0`, `iscc_decode`,
    `iscc_conformance_selftest`, `iscc_free_string`, plus 37 more
- `dotnet build packages/dotnet/Iscc.Lib.Tests/Iscc.Lib.Tests.csproj` — succeeds (0 warnings, 0
    errors)
- `dotnet test ... -e LD_LIBRARY_PATH=/workspace/iscc-lib/target/debug` — 1 passed, 0 failed
    (existing `ConformanceSelftest` smoke test still works)
- `cargo clippy -p iscc-ffi -- -D warnings` — clean
- `mise run check` — all 15 hooks pass

**Next:** Build idiomatic C# wrappers in `IsccLib.cs` that delegate to `NativeMethods` — PascalCase
methods, string marshaling, record result types, SafeHandle for streaming hashers, and proper memory
management (auto-free returned strings). Start with the gen functions that return simple strings
(`GenMetaCodeV0`, `GenTextCodeV0`, etc.) since those have the simplest marshaling.

**Notes:**

- csbindgen parsed `#[unsafe(no_mangle)]` (Rust 2024 edition syntax) without issues — no need for
    the `input_bindgen_file` fallback mentioned in the implementation notes
- The `NativeMethods` class is `internal` (not public) — this is correct, as idiomatic C# wrappers
    in `IsccLib.cs` will be the public API surface
- The generated file uses `byte*` for C strings (not `IntPtr`) and includes `[MarshalAs]` for bool
    fields — good type safety
- `IsccLib.cs` still has its own manual `DllImport` for `iscc_conformance_selftest` — this creates a
    harmless duplicate with `NativeMethods.iscc_conformance_selftest`. The next step should refactor
    `IsccLib.cs` to delegate to `NativeMethods` instead
- No tests were added for the generated file itself — the existing smoke test validates that the
    generated code compiles and the native library loads correctly. Conformance tests against
    `data.json` are planned for a future step
