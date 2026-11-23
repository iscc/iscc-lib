# CLAUDE.md — iscc-wasm

Browser-compatible WASM bindings for iscc-lib, published as `@iscc/wasm` on npm.

## Crate Role

- Spoke crate in the hub-and-spoke model: depends on `iscc-lib` (pure Rust core), adds
    `wasm-bindgen` annotations
- Thin wrapper layer only -- all logic lives in `iscc-lib`; this crate translates types and maps
    errors
- `publish = false` in Cargo.toml -- published to npm via `wasm-pack`, never to crates.io
- Stability tier: "best-effort" -- versioned by npm package version, not SemVer-governed like the
    Rust core

## Code Organization

- `src/lib.rs` contains every exported function. No submodules -- keep it flat until the file
    exceeds ~500 lines.
- Tests use `wasm_bindgen_test` and run in a Node.js WASM runtime.

## Type Mapping (Rust to JS via wasm-bindgen)

| Rust type               | JS type                                  | How                                                                                                 |
| ----------------------- | ---------------------------------------- | --------------------------------------------------------------------------------------------------- |
| `&str`                  | `string`                                 | Direct wasm-bindgen conversion                                                                      |
| `String`                | `string`                                 | Direct wasm-bindgen conversion                                                                      |
| `Option<String>`        | `string \| undefined`                    | wasm-bindgen handles Option natively                                                                |
| `JsValue` (opt. number) | `number \| null` (optional)              | `unchecked_optional_param_type`; `opt_u32` (`null`/`undefined` → default)                           |
| `JsValue` (number)      | `number`                                 | `unchecked_param_type = "number"`; checked by `number`, narrowed by `to_u8`/`to_u32`/`to_i32`       |
| `JsValue` (boolean)     | `boolean` / `boolean \| null` (optional) | Checked by `boolean` / `opt_bool` (`as_bool()`; `null`/`undefined` → default)                       |
| `&[u8]`                 | `Uint8Array`                             | Direct wasm-bindgen conversion                                                                      |
| `Vec<u8>`               | `Uint8Array`                             | Direct wasm-bindgen conversion                                                                      |
| `Vec<String>`           | `string[]`                               | Direct wasm-bindgen conversion                                                                      |
| `JsValue` (numbers)     | `ArrayLike<number>`                      | Via `unchecked_param_type`; elements checked by `number_vec`, narrowed by `to_i32_vec`/`to_u32_vec` |
| `Vec<Vec<i32>>`         | `JsValue`                                | Use `serde_wasm_bindgen::from_value` to deserialize                                                 |
| `Vec<Uint8Array>`       | `Uint8Array[]`                           | Return-side byte chunks via `js_sys::Uint8Array`                                                    |
| `Result<T, JsError>`    | throws on Err                            | `JsError::new(&e.to_string())`                                                                      |

**Key rules:**

- Use `serde_wasm_bindgen` for nested collections (`Vec<Vec<T>>`, `Vec<String>` from JS arrays).
    Never use `serde_json` in the WASM path -- it adds ~50KB.
- Errors are always `Result<T, JsError>`. Map `iscc_lib::IsccError` with
    `.map_err(|e| JsError::new(&e.to_string()))`.
- Most gen functions return the `iscc` string field from the core result struct, not the full
    struct. Exception: `gen_sum_code_v0` returns a `WasmSumCodeResult` struct with `iscc`,
    `datahash`, `filesize`, and optional `units` fields.

## Published Target and Initialization

`@iscc/wasm` is ONE `wasm-pack build --target web` build, published from `release.yml`. No other
target is built or published. The build is an ES module whose initializer loads `iscc_wasm_bg.wasm`
once; every environment calls it before any other export:

- Browsers and bundlers (webpack, Vite): a bare `await init()` fetches the `.wasm` file by URL
- Node.js: a bare `init()` fails (Node's `fetch` cannot read `file:` URLs), so pass the bytes:
    `initSync({ module: readFileSync(require.resolve("@iscc/wasm/iscc_wasm_bg.wasm")) })`. Prefer
    `iscc-napi` (`@iscc/lib`) for Node.js server use

`docs/howto/wasm.md` → "Setup" shows the three forms; `tests/test_wasm_docs.py` guards the docs.

The crate targets `wasm32-unknown-unknown` (browser WASM). WASI is not supported. Evaluate the WASM
Component Model only when WIT tooling is stable, browsers support components natively, and `jco`
output size is competitive with `wasm-pack`.

## Build Commands

```bash
# Development build (native ESM, the published target)
wasm-pack build crates/iscc-wasm --target web

# Release build, exactly as the build-wasm job of release.yml runs it
RUSTFLAGS="-C target-feature=+simd128" \
    wasm-pack build --target web --release crates/iscc-wasm --features conformance
```

The release profile uses
`wasm-opt = ["-O3", "--enable-simd", "--enable-bulk-memory", "--enable-nontrapping-float-to-int"]`
for maximum runtime speed optimization (configured via
`[package.metadata.wasm-pack.profile.release]` in Cargo.toml).

BLAKE3's hand-written `wasm32` SIMD backend is wired up with two compile-time pieces:

1. The `blake3/wasm32_simd` **Cargo feature** (required to activate the backend) — enabled by a
    direct `blake3 = { workspace = true, features = ["wasm32_simd"] }` dependency in this crate's
    Cargo.toml. The dep exists solely for feature unification (no `use blake3` in `lib.rs`); it
    feature-unifies onto `iscc-lib`'s blake3 for the wasm-only build and must not be removed as
    "unused". blake3's build.rs emits the gating `blake3_wasm32_simd` cfg only when this feature is
    set on a `wasm32` target, so native builds are unaffected.
2. `RUSTFLAGS="-C target-feature=+simd128"` — set by release and CI build steps. blake3's backend
    functions carry `#[target_feature(enable = "simd128")]`, so they compile even without it (a
    plain `wasm-pack build` succeeds); the global flag additionally enables `simd128` across the
    whole crate so the surrounding data-path code auto-vectorizes and blake3's SIMD functions
    inline optimally. `--enable-simd` lets `wasm-opt` accept the resulting `v128` instructions.

## Test Commands

```bash
# Run all WASM tests in Node.js runtime (including conformance_selftest)
wasm-pack test --node crates/iscc-wasm --features conformance

# Run without the conformance feature (skips conformance_selftest unit test)
wasm-pack test --node crates/iscc-wasm

# Run specific test file (--test goes to cargo, BEFORE the `--`; after `--` the
# wasm-bindgen-test-runner rejects it and only accepts a positional name filter)
wasm-pack test --node crates/iscc-wasm --features conformance --test conformance
wasm-pack test --node crates/iscc-wasm --test unit
```

- Tests use `#[wasm_bindgen_test]` attribute, not `#[test]`.
- Conformance vectors are loaded at compile time via
    `include_str!("../../iscc-lib/tests/data.json")`.
- `conformance.rs` reads vectors through the strict readers in `tests/common/mod.rs` (shared with
    `data_edge.rs`): a wrong JSON type or an out-of-range number fails the test, never cast or
    truncated. Each section's case count comes from `data.json`; an empty section fails.
- `data_edge.rs` hardcodes no vector count: adding a vector to a handled section needs no edit.
- No network access in tests. All test data is vendored.

## Publishing (npm via wasm-pack + npm publish)

- Package name: `@iscc/wasm` under the `@iscc` npm scope
- Auth: OIDC trusted publishing (configured per package on npmjs.com for `release.yml`)
- Workflow: the `build-wasm` job of `release.yml` (a `workflow_dispatch` run with a `version` input
    or the `npm` checkbox) runs
    `wasm-pack build --target web --release crates/iscc-wasm --features conformance`, then a
    post-build step patches the generated `pkg/package.json` to set `name` to `@iscc/wasm` and
    `version` to the root `Cargo.toml` version. Publishing runs `npm publish --access public` from
    the `pkg/` directory (trusted publishing adds provenance).
- There is no `package.json` in the crate root. The `pkg/package.json` is auto-generated by
    `wasm-pack build` and patched by CI before publishing.

## Exported API Surface

All Tier 1 symbols are bound (including `gen_iscc_id_v1`), plus the result structs and streaming
types listed below. Every `#[wasm_bindgen]` export in `lib.rs` maps 1:1 to an `iscc_lib` public
symbol:

- **Gen functions:** `gen_meta_code_v0`, `gen_text_code_v0`, `gen_image_code_v0`,
    `gen_audio_code_v0`, `gen_video_code_v0`, `gen_mixed_code_v0`, `gen_data_code_v0`,
    `gen_instance_code_v0`, `gen_iscc_code_v0`, `gen_sum_code_v0`
- **ISCC-IDv1 mint function (experimental):** `gen_iscc_id_v1` (returns the ISCC string directly,
    not a wrapper struct)
- **Text utils:** `text_clean`, `text_remove_newlines`, `text_trim`, `text_collapse`
- **Algorithm primitives:** `sliding_window`, `alg_simhash`, `alg_minhash_256`, `alg_cdc_chunks`
- **Constants:** `META_TRIM_NAME`, `META_TRIM_DESCRIPTION`, `META_TRIM_META`, `IO_READ_SIZE`,
    `TEXT_NGRAM_SIZE` (exposed as getter functions — wasm-bindgen does not support `#[wasm_bindgen]`
    on `pub const`)
- **Soft hash:** `soft_hash_video_v0`
- **Encoding:** `encode_base64`, `json_to_data_url`
- **Codec:** `encode_component`, `iscc_decode` (with `IsccDecodeResult` struct), `iscc_decompose`,
    `encode_seq` (`units` accepts any JS iterable of strings), `decode_seq`
- **Diagnostic:** `conformance_selftest` (feature-gated behind `conformance` Cargo feature)
- **Result structs:** `WasmSumCodeResult` (returned by `gen_sum_code_v0`, with `iscc`, `datahash`,
    `filesize`, and optional `units` fields) and `IsccDecodeResult` (returned by `iscc_decode`)
- **Streaming types:** `DataHasher`, `InstanceHasher`, `SumHasher`

`DataHasher`, `InstanceHasher`, and `SumHasher` are bound as `#[wasm_bindgen]` structs with
`constructor`, `update()`, and `finalize()` methods. They use the `Option<Inner>` finalize-once
pattern (consistent with the Python, napi, and JNI binding crates). `SumHasher` wraps the core
`iscc_lib::streaming::SumHasher` (reached via its full module path — it is intentionally not a
crate-root Tier 1 re-export) and its `finalize(bits?, wide?, add_units?)` returns the shared
`WasmSumCodeResult`.

## Common Pitfalls

- **Do not use `serde_json` at runtime.** Use `serde_wasm_bindgen::from_value` for nested JS inputs.
    `serde_json` is a dev-dependency only (for parsing test vectors).
- **Do not return bytes through `serde_wasm_bindgen::to_value`.** It turns `Vec<u8>` into a plain
    `number[]`. Return `Vec<u8>` (one `Uint8Array`) or `Vec<js_sys::Uint8Array>` (`Uint8Array[]`).
- **Do not add logic in this crate.** All computation belongs in `iscc-lib`. This crate is a
    translation layer.
- **Do not expose Rust structs directly.** Return primitive types (`String`, `Vec<u8>`,
    `Vec<String>`) or `JsValue`. Most gen functions return the `.iscc` string field, not the full
    result struct. Exception: `gen_sum_code_v0` returns `WasmSumCodeResult`, and `iscc_decode`
    returns `IsccDecodeResult` — both use `#[wasm_bindgen(getter_with_clone)]`.
- **Do not panic across WASM boundary.** Every fallible path must return `Result<T, JsError>`.
    Panics in WASM abort the runtime.
- **Do not add `#[wasm_bindgen]` to internal helpers.** Only Tier 1 API functions get the
    annotation.
- **Do not use `Option<&str>`.** wasm-bindgen does not support `Option<&str>` -- use
    `Option<String>` and convert with `.as_deref()`.
- **Never take a number, a `bool` or a `Vec` of numbers as an input.** The JS glue converts them
    (`>>> 0` for `u32`, `null` → 0 and `"2"` → 2 for `f64`, truthiness for `bool`, typed-array
    copies for `Vec<f64>`/`Vec<i32>`), so a wrong value turns into a different valid argument. Take
    a `JsValue`, type-check it with the helpers at the top of `src/lib.rs` (`number`, `opt_number`,
    `number_vec`, `boolean`, `opt_bool`), then narrow through the range-checked `to_*` helpers. Keep
    the TypeScript type with `unchecked_param_type` (`"number"`, `"boolean"`, `"ArrayLike<number>"`)
    or, for a trailing optional parameter, `unchecked_optional_param_type` (`"number | null"`,
    `"boolean | null"`). Nested arrays (`gen_video_code_v0`) take `JsValue` +
    `serde_wasm_bindgen::from_value`, which rejects non-`i32` numbers itself.
- **An `Option<T>` parameter before a `JsValue` override needs its own TypeScript override.**
    wasm-bindgen marks an `Option<T>` optional only when every later parameter is, and a `JsValue`
    with `unchecked_optional_param_type` does not count. Give it
    `unchecked_optional_param_type = "string | null"` (`gen_meta_code_v0`), or it loses its `?` in
    the `.d.ts`. After a signature change, diff the built `.d.ts` against a build of the previous
    commit.
- **Read a JS value in a helper only through an import marked `catch`** (`Reflect::get`,
    `Reflect::get_u32`). A JS exception from a getter or a proxy must not cross a Rust frame:
    `Array::from`, the `Array` index getter and `try_iter` have no `catch`, and a few thousand such
    throws leave every export failing with `memory access out of bounds`.
- **`#[wasm_bindgen_test]` calls the Rust function directly and skips the JS glue.** Prove a
    marshalling change from Node as well, on a release `--target web` build ("Build Commands")
    loaded with `initSync({ module })`.
- **A new `data.json` section needs a test in `conformance.rs`.** The core's `SECTIONS` table test
    fails on an unhandled section; the WASM harness has no such guard of its own.
- **`crate-type` must include `cdylib`.** The `rlib` target is included for test compilation. Do not
    remove either.
