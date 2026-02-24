# Next Work Package

## Step: Implement all 22 remaining JNI bridge functions

## Goal

Complete the JNI bridge crate by implementing the remaining 22 Tier 1 symbols as `extern "system"`
JNI functions, bringing `iscc-jni` from 1/23 to 23/23 coverage. This unblocks the subsequent Java
wrapper class, Maven build, and CI steps.

## Scope

- **Create**: (none)
- **Modify**: `crates/iscc-jni/src/lib.rs`
- **Reference**:
    - `crates/iscc-napi/src/lib.rs` — pattern reference for all 23 symbols (type mappings, error
        handling, streaming hasher lifecycle)
    - `crates/iscc-ffi/src/lib.rs` — opaque pointer pattern for streaming hashers
        (`Box::into_raw`/`Box::from_raw`, finalize-once via `Option<Inner>`)
    - `crates/iscc-lib/src/lib.rs` — Tier 1 API signatures and types

## Not In Scope

- Java source tree (`io.iscc.iscc_lib.IsccLib` class, Maven/Gradle build) — separate step
- Java tests (requires JVM in devcontainer) — separate step
- CI job for Java — requires Java tests first
- Native library bundling (`META-INF/native/`) — separate step
- Returning structured results (all gen functions return `.iscc` string only, matching napi/wasm
    pattern)
- README or documentation updates for Java

## Implementation Notes

### Error handling helper

Implement `throw_and_default` as a real function (currently documented as a code template):

```rust
fn throw_and_default<T: Default>(env: &mut JNIEnv, msg: &str) -> T {
    let _ = env.throw_new("java/lang/IllegalArgumentException", msg);
    T::default()
}
```

Use this in all fallible functions (gen\_\*\_v0, iscc_decompose, sliding_window, alg_simhash,
soft_hash_video_v0, streaming hasher update/finalize).

### JNI type mappings

Follow these conversions consistently:

| Rust type                              | JNI parameter                               | Conversion                                              |
| -------------------------------------- | ------------------------------------------- | ------------------------------------------------------- |
| `&str`                                 | `JString`                                   | `env.get_string(&input)` → `.into()`                    |
| `Option<&str>`                         | `JString`                                   | check `.is_null()` before extracting                    |
| `&[u8]`                                | `jbyteArray`                                | `env.convert_byte_array(input)`                         |
| `&[i32]`                               | `jintArray`                                 | `env.get_array_length()` + `env.get_int_array_region()` |
| `&[&str]`                              | `JObjectArray`                              | loop + `get_object_array_element` + `get_string`        |
| `Vec<Vec<i32>>`                        | `JObjectArray` of `jintArray`               | nested extraction                                       |
| Return `String` → `jstring`            | `env.new_string(result)` → `.into_raw()`    |                                                         |
| Return `Vec<String>` → `jobjectArray`  | build `JObjectArray` via `new_object_array` |                                                         |
| Return `Vec<u8>` → `jbyteArray`        | `env.byte_array_from_slice(&result)`        |                                                         |
| Return `Vec<Vec<u8>>` → `jobjectArray` | array of `jbyteArray`                       |                                                         |

### Function groups

**9 gen\_\*\_v0 functions** — each takes JNI params, converts, calls `iscc_lib::gen_*_v0`, returns
`jstring` of the `.iscc` field. Use `throw_and_default` on error, returning
`JObject::null().into_raw()`.

**4 text utilities** (`text_clean`, `text_remove_newlines`, `text_trim`, `text_collapse`) —
infallible, take `JString`, return `jstring`.

**`sliding_window`** — takes `JString` + `jint`, returns `jobjectArray`. Throws on `width < 2`.

**`alg_simhash`** — takes `jobjectArray` of `jbyteArray`, returns `jbyteArray`. Throws on error.

**`alg_minhash_256`** — takes `jintArray` (u32 features as int), returns `jbyteArray`. Infallible.
Note: Java has no unsigned int — cast `jint` to `u32` with `as u32`.

**`alg_cdc_chunks`** — takes `jbyteArray` + `jboolean` + `jint`, returns `jobjectArray` of
`jbyteArray`.

**`soft_hash_video_v0`** — takes `jobjectArray` of `jintArray` + `jint`, returns `jbyteArray`.
Throws on error.

**`encode_base64`** — takes `jbyteArray`, returns `jstring`.

**`iscc_decompose`** — takes `JString`, returns `jobjectArray` of `jstring`. Throws on error.

### Streaming hashers

Use the opaque-pointer-as-jlong pattern (same as C FFI but with `jlong` instead of raw pointer):

- `dataHasherNew()` → creates `Box<DataHasherWrapper>`, returns `jlong` via
    `Box::into_raw() as jlong`
- `dataHasherUpdate(jlong ptr, jbyteArray data)` → casts back to `&mut`, calls `inner.update()`
- `dataHasherFinalize(jlong ptr, jint bits)` → takes ownership via `Box::from_raw()`, calls
    `inner.take().finalize()`, returns `jstring`
- `dataHasherFree(jlong ptr)` → drops via `Box::from_raw()` if non-zero

Same pattern for `InstanceHasher`. Use a wrapper struct with `Option<Inner>` for finalize-once
semantics (matching the C FFI `FfiDataHasher` pattern).

### JNI naming convention

All functions use the prefix `Java_io_iscc_iscc_1lib_IsccLib_` (the `_1` encodes the underscore in
`iscc_lib`). Use `#[unsafe(no_mangle)]` per Rust 2024 edition. Use `extern "system"` calling
convention.

### Import management

Add needed JNI types to the `use` declarations: `JString`, `JObject`, `JObjectArray`, `jstring`,
`jobjectArray`, `jbyteArray`, `jintArray`, `jint`, `jlong`, etc.

## Verification

- `cargo check -p iscc-jni` exits 0
- `cargo clippy -p iscc-jni -- -D warnings` exits 0
- `cargo clippy --workspace --all-targets -- -D warnings` exits 0
- `crates/iscc-jni/src/lib.rs` contains 23 `extern "system"` functions (1 existing + 22 new)
- All 9 gen function JNI names are present: `genMetaCodeV0`, `genTextCodeV0`, `genImageCodeV0`,
    `genAudioCodeV0`, `genVideoCodeV0`, `genMixedCodeV0`, `genDataCodeV0`, `genInstanceCodeV0`,
    `genIsccCodeV0`
- All 4 text utility JNI names are present: `textClean`, `textRemoveNewlines`, `textTrim`,
    `textCollapse`
- Streaming hasher JNI functions are present: `dataHasherNew`, `dataHasherUpdate`,
    `dataHasherFinalize`, `dataHasherFree`, `instanceHasherNew`, `instanceHasherUpdate`,
    `instanceHasherFinalize`, `instanceHasherFree`
- `throw_and_default` helper function exists (not just a doc template)

## Done When

All verification criteria pass — `iscc-jni` compiles cleanly with 23 Tier 1 symbols as JNI bridge
functions, workspace clippy is clean, and `throw_and_default` is a real function.
