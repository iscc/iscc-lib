# Next Work Package

## Step: Create UniFFI scaffolding crate with all 32 Tier 1 symbols

## Goal

Create `crates/iscc-uniffi/` — the shared UniFFI scaffolding crate that exposes all 32 Tier 1
symbols from `iscc-lib` via UniFFI proc macros. This is the foundation for both Swift and Kotlin
bindings; neither can proceed without it. Addresses the "Implement Swift bindings via UniFFI"
normal-priority issue.

## Scope

- **Create**: `crates/iscc-uniffi/Cargo.toml`, `crates/iscc-uniffi/src/lib.rs`
- **Modify**: `Cargo.toml` (root workspace — add `crates/iscc-uniffi` to `members` and add `uniffi`
    to `workspace.dependencies`)
- **Reference**:
    - `crates/iscc-lib/src/lib.rs` (Tier 1 function signatures)
    - `crates/iscc-lib/src/types.rs` (result struct definitions)
    - `crates/iscc-lib/src/streaming.rs` (DataHasher/InstanceHasher)
    - `crates/iscc-ffi/src/lib.rs` (binding wrapping pattern — how FFI maps iscc-lib types)
    - `.claude/context/specs/swift-bindings.md` (UniFFI interface design)

## Not In Scope

- Swift package (`packages/swift/`) — separate step after UniFFI crate compiles
- Kotlin package (`packages/kotlin/`) — depends on Swift being done first
- `uniffi.toml` config — only needed for binding generation customization, not for compilation
- `build.rs` — not needed with proc macro approach (`#[uniffi::export]`)
- CI job for UniFFI crate — will be added with Swift CI job
- Binding generation (`uniffi-bindgen generate`) — separate step
- Version sync targets — will be added with Swift package
- Documentation (`docs/howto/swift.md`, README updates) — separate step
- Tests beyond basic compilation — conformance testing will happen in Swift/Kotlin test suites

## Implementation Notes

### Crate setup (`Cargo.toml`)

- `crate-type = ["cdylib", "staticlib", "lib"]` — `cdylib` for dynamic loading, `staticlib` for
    XCFramework, `lib` for tests
- Dependencies: `iscc-lib` (path, all features), `uniffi` (workspace)
- Add `uniffi = "0.31"` to `[workspace.dependencies]` in root Cargo.toml
- `publish = false` — this crate is not published to crates.io (only used to generate bindings)

### UniFFI interface (`src/lib.rs`)

Use **proc macros only** (no UDL files, no `build.rs`). The pattern:

1. **Scaffolding setup**: `uniffi::setup_scaffolding!();` at top of lib.rs.

2. **Error type**: `#[derive(Debug, thiserror::Error, uniffi::Error)]` enum `IsccUniError` with a
    single `IsccError { msg: String }` variant. Map from `iscc_lib::IsccError` via `From` impl.

3. **Result records**: `#[derive(uniffi::Record)]` for each result type. UniFFI records map to Swift
    structs and Kotlin data classes. Mirror all fields from `iscc_lib::types`:

    - `MetaCodeResult` — `iscc`, `name`, `description` (Option), `meta` (Option), `metahash`
    - `TextCodeResult` — `iscc`, `characters` (as `u64`, UniFFI doesn't support `usize`)
    - `ImageCodeResult` — `iscc`
    - `AudioCodeResult` — `iscc`
    - `VideoCodeResult` — `iscc`
    - `MixedCodeResult` — `iscc`, `parts` (Vec<String>)
    - `DataCodeResult` — `iscc`
    - `InstanceCodeResult` — `iscc`, `datahash`, `filesize` (u64)
    - `IsccCodeResult` — `iscc`
    - `SumCodeResult` — `iscc`, `datahash`, `filesize` (u64), `units` (Option\<Vec<String>>)
    - `DecodeResult` — `maintype` (u8), `subtype` (u8), `version` (u8), `length` (u8), `digest`
        (Vec<u8>)

4. **Gen functions**: `#[uniffi::export]` on free functions. Each wraps the `iscc_lib` call,
    converts the result type, and maps errors:

    - `gen_meta_code_v0(name: String, description: Option<String>, meta: Option<String>, bits: u32)`
    - `gen_text_code_v0(text: String, bits: u32)`
    - `gen_image_code_v0(pixels: Vec<u8>, bits: u32)`
    - `gen_audio_code_v0(cv: Vec<i32>, bits: u32)`
    - `gen_video_code_v0(frame_sigs: Vec<Vec<i32>>, bits: u32)` — flatten generics for UniFFI
    - `gen_mixed_code_v0(codes: Vec<String>, bits: u32)`
    - `gen_data_code_v0(data: Vec<u8>, bits: u32)`
    - `gen_instance_code_v0(data: Vec<u8>, bits: u32)`
    - `gen_iscc_code_v0(codes: Vec<String>, wide: bool)`
    - `gen_sum_code_v0(path: String, bits: u32, wide: bool, add_units: bool)` — takes String path

5. **Utility functions**: `#[uniffi::export]` wrappers for:

    - `text_clean`, `text_remove_newlines`, `text_trim`, `text_collapse`
    - `sliding_window` (returns `Vec<String>`)
    - `alg_minhash_256` (features: `Vec<u32>`, returns `Vec<u8>`)
    - `alg_cdc_chunks` — returns `Vec<Vec<u8>>` (UniFFI can't do borrowed slices)
    - `alg_simhash` — takes `Vec<Vec<u8>>`, returns `Vec<u8>`
    - `soft_hash_video_v0` — takes `Vec<Vec<i32>>`, returns `Vec<u8>`
    - `encode_base64`, `json_to_data_url`, `iscc_decompose`, `encode_component`, `iscc_decode`
    - `conformance_selftest`

6. **Constants as getter functions**: UniFFI doesn't support `const` exports. Use:

    ```rust
    #[uniffi::export]
    fn meta_trim_name() -> u32 { iscc_lib::META_TRIM_NAME as u32 }
    ```

    Same for `META_TRIM_DESCRIPTION`, `META_TRIM_META`, `IO_READ_SIZE`, `TEXT_NGRAM_SIZE`.

7. **Streaming types**: `#[derive(uniffi::Object)]` on wrapper structs with interior
    `Mutex<Option<Inner>>`. UniFFI Objects map to classes in Swift/Kotlin with reference counting:

    ```rust
    #[derive(uniffi::Object)]
    pub struct DataHasher {
        inner: std::sync::Mutex<Option<iscc_lib::DataHasher>>,
    }
    #[uniffi::export]
    impl DataHasher {
        #[uniffi::constructor]
        pub fn new() -> Self { ... }
        pub fn update(&self, data: Vec<u8>) { ... }
        pub fn finalize(&self, bits: u32) -> Result<DataCodeResult, IsccUniError> { ... }
    }
    ```

    Same pattern for `InstanceHasher`.

### Key type mapping notes

- `usize` -> `u64` (UniFFI supports u64 but not usize)
- `&str` -> `String` (UniFFI uses owned types)
- `&[u8]` -> `Vec<u8>` (UniFFI uses owned types)
- `&[&str]` -> `Vec<String>`
- Generic `<S: AsRef<[i32]> + Ord>` -> concrete `Vec<Vec<i32>>`
- `std::path::Path` -> `String` (convert in wrapper)
- `Vec<&[u8]>` (from `alg_cdc_chunks`) -> `Vec<Vec<u8>>` (copy to owned)
- `Result<T, IsccError>` -> `Result<T, IsccUniError>`

## Verification

- `cargo build -p iscc-uniffi` compiles successfully
- `cargo test -p iscc-uniffi` passes (at minimum: any doc tests or unit tests)
- `cargo clippy -p iscc-uniffi -- -D warnings` is clean
- `cargo build -p iscc-lib` still passes (no regressions)
- `cargo test -p iscc-lib` still passes (316+ tests)

## Done When

All verification criteria pass — the UniFFI scaffolding crate compiles, all 32 Tier 1 symbols are
exposed via UniFFI proc macros, and the existing workspace builds and tests are unaffected.
