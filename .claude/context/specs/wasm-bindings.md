# Spec: WASM Bindings — Browser/Node via wasm-bindgen

A browser-compatible WASM package `@iscc/wasm` providing all ISCC functions via wasm-bindgen. Runs
in any WASM-capable environment (browsers, Deno, Cloudflare Workers, Node.js via WASM).

## Architecture

**Single-layer design:**

1. **Rust bridge** (`crates/iscc-wasm/src/lib.rs`): wasm-bindgen annotated functions that call
    `iscc-lib` core and return JavaScript types via WASM interop. All Tier 1 symbols exported.
2. **Generated JS glue** (`pkg/`): wasm-pack generates JavaScript wrapper and TypeScript
    declarations for the WASM module.

**Why separate from Node.js native addon:**

- `@iscc/lib` (napi-rs) — native performance, Node.js only, requires platform-specific binaries
- `@iscc/wasm` (wasm-bindgen) — portable, runs everywhere WASM runs (browsers, edge), ~2-5x slower
    than native but zero platform dependencies

## Crate Structure

```
crates/iscc-wasm/
├── Cargo.toml              # cdylib, depends on iscc-lib + wasm-bindgen
├── src/
│   └── lib.rs              # wasm-bindgen bridge (all Tier 1 symbols)
├── README.md               # Per-crate README for npm
├── tests/
│   └── conformance.rs      # wasm-pack test conformance against data.json
└── pkg/                    # wasm-pack build output (generated)
```

The tree lists the load-bearing entries, not every file. `pkg/` is wasm-pack build output and is not
tracked.

## Rust Bridge Layer

Functions use a bare `#[wasm_bindgen]` attribute, so every export keeps its snake_case Rust name
(`gen_meta_code_v0`, never a camelCase `js_name`). Optional numbers and booleans arrive as
`JsValue`, keep their TypeScript type through `unchecked_optional_param_type`, and are type-checked
in Rust by the private `opt_u32` / `opt_bool` helpers at the top of `crates/iscc-wasm/src/lib.rs`.
Gen functions return the `iscc` string:

```rust
#[wasm_bindgen]
pub fn gen_meta_code_v0(
    name: &str,
    #[wasm_bindgen(unchecked_optional_param_type = "string | null")] description: Option<String>,
    #[wasm_bindgen(unchecked_optional_param_type = "string | null")] meta: Option<String>,
    #[wasm_bindgen(unchecked_optional_param_type = "number | null")] bits: JsValue,
) -> Result<String, JsError> {
    let bits = opt_u32(&bits, 64, "bits")?;
    iscc_lib::gen_meta_code_v0(name, description.as_deref(), meta.as_deref(), bits)
        .map(|r| r.iscc)
        .map_err(|e| JsError::new(&e.to_string()))
}
```

The exception is `gen_sum_code_v0`, which returns a `WasmSumCodeResult` object (`iscc`, `datahash`,
`filesize`, optional `units`).

### Streaming Hashers

`DataHasher` and `InstanceHasher` are exposed as JavaScript classes. The WASM bridge uses the same
`Option<inner>` pattern as napi-rs for finalize-once semantics.

A `SumHasher` class (GitHub issue #37) exposes single-pass streaming ISCC-SUM (Data-Code +
Instance-Code) over the shared core `streaming::SumHasher` struct, so browser consumers feed each
chunk once instead of `update()`-ing both `DataHasher` and `InstanceHasher` separately (which copies
every byte into WASM linear memory twice). `SumHasher.finalize(bits?, wide?, add_units?)` returns a
`WasmSumCodeResult` (same shape as `gen_sum_code_v0`: `iscc`, `datahash`, `filesize`, optional
`units`), dropping the manual `gen_iscc_code_v0([dataCode, instanceCode])` assembly on the JS side.
The core struct is shared with the PyO3 binding (see `python-bindings.md` → "Streaming SumHasher").

### Key Differences from Node.js Binding

- Uses `&[u8]` / `Vec<u8>` mapped to `Uint8Array` (not Node.js `Buffer`)
- `gen_sum_code_v0` takes a `Uint8Array`, not a file path (no filesystem in a browser context)
- No native filesystem access — streaming via `update()` calls only
- `serde-wasm-bindgen` deserializes nested array inputs (frame signatures, code lists); structured
    results such as `WasmSumCodeResult` are `#[wasm_bindgen]` structs

## Build Process

```bash
wasm-pack build crates/iscc-wasm --target bundler   # For bundlers (webpack, vite)
wasm-pack build crates/iscc-wasm --target web        # For direct browser use
wasm-pack build crates/iscc-wasm --target nodejs     # For Node.js (testing)
```

### WASM SIMD (`simd128`)

GitHub: https://github.com/iscc/iscc-lib/issues/42

Published release builds enable WASM SIMD so `blake3` uses its `wasm32` SIMD backend instead of the
portable scalar fallback. Two compile-time pieces are involved (no runtime detection):

1. The `blake3/wasm32_simd` **Cargo feature** (required to activate the backend) — a direct
    `blake3 = { workspace = true, features = ["wasm32_simd"] }` dependency on `crates/iscc-wasm`.
    blake3's build.rs emits the gating `blake3_wasm32_simd` cfg (which makes `Platform::detect()`
    return `WASM32_SIMD`) only when this feature is set on a `wasm32` target; the `simd128`
    target-feature alone leaves blake3 on `Platform::Portable`.
2. The `simd128` target-feature (`RUSTFLAGS="-C target-feature=+simd128"`) — set on the release
    build. blake3's backend functions carry `#[target_feature(enable = "simd128")]`, so they
    compile even without it; the global flag additionally enables `simd128` across the whole crate
    so the surrounding data-path code (gear CDC / xxh32 / minhash) also auto-vectorizes and
    blake3's SIMD functions inline optimally:

```bash
RUSTFLAGS="-C target-feature=+simd128" \
    wasm-pack build --target web --release crates/iscc-wasm --features conformance
```

and `wasm-opt` must accept SIMD in `crates/iscc-wasm/Cargo.toml`:

```toml
[package.metadata.wasm-pack.profile.release]
wasm-opt = [
  "-O3",
  "--enable-simd",
  "--enable-bulk-memory",
  "--enable-nontrapping-float-to-int",
]
```

Applies to every published target (`web`, plus `bundler` if published). BLAKE3 is the Instance-Code
/ `datahash` leg of `gen_sum_code_v0` / `SumHasher` and gains the most; the Data leg (gear CDC /
xxh32 / minhash) benefits less. `simd128` is baseline in all current browsers (Chrome/Edge ≥91,
Firefox ≥89, Safari ≥16.4) and Node ≥16 — a single SIMD build is fine, no scalar fallback artifact.
Build-flag + Cargo-feature change only, no source changes: conformance output is byte-identical.

**Verified when:**

- Release workflow builds `@iscc/wasm` with `simd128` enabled
    (`RUSTFLAGS="-C target-feature=+simd128"`) for all published targets
- `wasm-opt` flags in `crates/iscc-wasm/Cargo.toml` include `--enable-simd`
- `wasm-pack test --node crates/iscc-wasm --features conformance` passes on the SIMD build
- Published `.wasm` binary contains SIMD instructions — the backend is definitively wired in
    (`cargo tree -p iscc-wasm --target wasm32-unknown-unknown -i blake3 -f "{p} {f}"` shows blake3's
    `wasm32_simd` feature, so `Platform::detect()` returns `WASM32_SIMD` at compile time), and the
    release `.wasm` disassembly shows more `v128` opcodes (`wasm-tools print`) than a build without
    the Cargo feature. (`wasm-validate --enable-simd` is NOT sufficient evidence — it merely permits
    SIMD and passes scalar modules too.)

## Distribution / Publishing

- **Scope**: `@iscc/wasm` under the `@iscc` npm organization
- **Authentication**: OIDC trusted publishing, like the Node.js binding
- **Package type**: Single universal package (no platform-specific variants)
- **Size**: ~200-400KB for the `.wasm` binary

## CI Integration

### CI Job (ci.yml)

```yaml
wasm:
  runs-on: ubuntu-latest
  steps:
    - uses: actions/checkout@v4
    - uses: dtolnay/rust-toolchain@stable
      with:
        targets: wasm32-unknown-unknown
    - uses: taiki-e/install-action@wasm-pack
    - run: wasm-pack test --node crates/iscc-wasm
```

## Version Sync

The crate has no committed `package.json`. wasm-pack generates `pkg/package.json` from
`crates/iscc-wasm/Cargo.toml` (version, `license`, `description`), and release.yml's "Set npm
package name and version" step patches `name`, `repository` and `version` (read from the root
`Cargo.toml`). The Cargo manifests are therefore the single source of the npm version, `license` and
`description`; see ci-cd.md's "Automatic propagation" section.

## Documentation

- **How-to guide**: `docs/howto/wasm.md`
- **Per-crate README**: `crates/iscc-wasm/README.md` for npm

## Verification Criteria

- `wasm-pack test --node` passes all conformance vectors
- All Tier 1 symbols accessible from JavaScript/TypeScript
- Package builds with `wasm-pack build`
- `DataHasher` and `InstanceHasher` streaming types work in WASM
- `SumHasher` streaming type works in WASM (single-pass ISCC-SUM, output matches two-hasher pattern)
- TypeScript declarations provide accurate type information
- Package installs cleanly via `npm install @iscc/wasm`
- Works in browser environments (no Node.js-specific APIs)
- npm package version comes from root `Cargo.toml` (wasm-pack output patched by release.yml)
- Per-crate README renders correctly on npmjs.com
