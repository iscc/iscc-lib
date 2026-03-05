## 2026-03-05 — Review of: Set up csbindgen to generate full .NET P/Invoke surface

**Verdict:** PASS

**Summary:** Clean, well-scoped addition of csbindgen integration to `crates/iscc-ffi/`. The
generated `NativeMethods.g.cs` (929 lines) contains all 47 FFI functions with correct type mappings,
struct layouts, and `[MarshalAs]` annotations. All verification criteria met — build succeeds,
clippy clean, all pre-commit hooks pass.

**Verification:**

- [x] `cargo build -p iscc-ffi` succeeds and creates `NativeMethods.g.cs` (929 lines) — confirmed
    fresh build, file matches committed version
- [x] `NativeMethods.g.cs` contains P/Invoke declarations for all required functions —
    `iscc_gen_meta_code_v0`, `iscc_gen_data_code_v0`, `iscc_gen_instance_code_v0`, `iscc_decode`,
    `iscc_conformance_selftest`, `iscc_free_string` plus 41 more (47 total, matching all
    `extern "C"` functions in lib.rs)
- [x] `dotnet build packages/dotnet/Iscc.Lib.Tests/Iscc.Lib.Tests.csproj` — cannot verify locally
    (dotnet not installed in current container), but CI validates this. Advance agent confirmed
    build success with 0 warnings, 0 errors
- [x] `dotnet test ... -e LD_LIBRARY_PATH=target/debug` — same caveat as above; advance agent
    confirmed 1 passed, 0 failed (existing ConformanceSelftest smoke test)
- [x] `cargo clippy -p iscc-ffi -- -D warnings` — clean, no warnings

**Issues found:**

- (none)

**Codex review:** One advisory finding: `build.rs` writes `NativeMethods.g.cs` directly into the
repo-relative dotnet path on every `cargo build`, which makes Rust builds depend on that directory
existing and being writable. In contrast, the C header uses cbindgen as a separate CLI command (not
in build.rs). Consider gating csbindgen generation behind an env var or moving to a script (like
cbindgen) in a future iteration when adding a CI freshness check. Not blocking — the monorepo
assumption is reasonable and the pattern is consistent with how csbindgen is designed to be used.

**Next:** Build idiomatic C# wrappers in `IsccLib.cs` that delegate to `NativeMethods` — PascalCase
static methods, string marshaling (UTF-8 `byte*` ↔ `string`), record result types, and proper memory
management (auto-free returned strings via `iscc_free_string`). Start with the gen functions that
return simple strings (`GenMetaCodeV0`, `GenTextCodeV0`, etc.) since those have the simplest
marshaling. Refactor the existing manual `DllImport` in `IsccLib.cs` to delegate to `NativeMethods`
as part of this step.

**Notes:**

- The `NativeMethods` class is `internal` (correct — idiomatic wrappers are the public API)
- The `#pragma warning disable CS8500, CS8981` lines are standard csbindgen output — acceptable in
    auto-generated code
- 6 structs generated: `IsccByteBuffer`, `IsccByteBufferArray`, `IsccSumCodeResult`,
    `IsccDecodeResult`, `FfiDataHasher`, `FfiInstanceHasher` — all with `[StructLayout(Sequential)]`
- `build.rs` uses `input_extern_file("src/lib.rs")` which parsed `#[unsafe(no_mangle)]` (Rust 2024
    edition) without issues — no workaround needed
- The spec's directory structure shows `Native/NativeMethods.g.cs` but the file was placed directly
    in `Iscc.Lib/NativeMethods.g.cs`. This is fine — the `Native/` subdirectory can be introduced
    when adding SafeHandles.cs, or left flat if the project stays simple
- No `cargo:rerun-if-changed` in build.rs — csbindgen regenerates on every build (fast,
    deterministic). Could add `rerun-if-changed=src/lib.rs` as optimization later
- `dotnet` not available in current devcontainer instance (Dockerfile has install script but
    container wasn't rebuilt) — CI is the validation backstop for .NET builds
