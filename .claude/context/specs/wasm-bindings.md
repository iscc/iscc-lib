# Spec: WASM Bindings — Browser/Node via wasm-bindgen

A browser-compatible WASM package `@iscc/wasm` providing all ISCC functions via wasm-bindgen. Runs
in any WASM-capable environment (browsers, Deno, Cloudflare Workers, Node.js via WASM).

## Architecture

**Single-layer design:**

1. **Rust bridge** (`crates/iscc-wasm/src/lib.rs`): wasm-bindgen annotated functions that call
    `iscc-lib` core and return JavaScript types via WASM interop. All 32 Tier 1 symbols exported.
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
│   └── lib.rs              # wasm-bindgen bridge (all 32 Tier 1 symbols)
├── package.json            # npm package config (@iscc/wasm)
├── README.md               # Per-crate README for npm
├── tests/
│   └── conformance.rs      # wasm-pack test conformance against data.json
└── pkg/                    # wasm-pack build output (generated)
    ├── iscc_wasm.js
    ├── iscc_wasm.d.ts
    ├── iscc_wasm_bg.wasm
    └── package.json
```

## Rust Bridge Layer

Functions use `#[wasm_bindgen]` attribute. Return types are serialized to `JsValue` via serde:

```rust
#[wasm_bindgen(js_name = "genMetaCodeV0")]
pub fn gen_meta_code_v0(
    name: &str,
    description: Option<String>,
    meta: Option<String>,
    bits: Option<u32>,
) -> Result<JsValue, JsError> {
    let result = iscc_lib::gen_meta_code_v0(name, ...)?;
    Ok(serde_wasm_bindgen::to_value(&result)?)
}
```

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
- File path operations (`gen_sum_code_v0`) not available in browser context
- No native filesystem access — streaming via `update()` calls only
- `serde-wasm-bindgen` for complex type serialization (vs napi-rs's `#[napi(object)]`)

## Build Process

```bash
wasm-pack build crates/iscc-wasm --target bundler   # For bundlers (webpack, vite)
wasm-pack build crates/iscc-wasm --target web        # For direct browser use
wasm-pack build crates/iscc-wasm --target nodejs     # For Node.js (testing)
```

### WASM SIMD (`simd128`)

GitHub: https://github.com/iscc/iscc-lib/issues/42

Published release builds enable WASM SIMD so `blake3` uses its `wasm32` SIMD backend instead of the
portable scalar fallback. On wasm the backend is selected at **compile time** via
`target_feature = "simd128"` (no runtime detection), so the flag must be set on the release build:

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
Pure build-flag change: conformance output is byte-identical.

**Verified when:**

- [ ] Release workflow builds `@iscc/wasm` with `simd128` enabled
    (`RUSTFLAGS="-C target-feature=+simd128"`) for all published targets
- [ ] `wasm-opt` flags in `crates/iscc-wasm/Cargo.toml` include `--enable-simd`
- [ ] `wasm-pack test --node crates/iscc-wasm --features conformance` passes on the SIMD build
- [ ] Published `.wasm` binary contains SIMD instructions (e.g. `wasm-objdump` /
    `wasm-validate --enable-simd` evidence, or a documented before/after `SumHasher` throughput
    measurement on a few-MB buffer)

## Distribution / Publishing

- **Scope**: `@iscc/wasm` under the `@iscc` npm organization
- **Authentication**: Same `NPM_TOKEN` as Node.js binding
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

| Target                          | What is synced       |
| ------------------------------- | -------------------- |
| `crates/iscc-wasm/package.json` | `"version": "X.Y.Z"` |

## Documentation

- **How-to guide**: `docs/howto/wasm.md`
- **Per-crate README**: `crates/iscc-wasm/README.md` for npm

## Verification Criteria

- [ ] `wasm-pack test --node` passes all conformance vectors
- [ ] All 32 Tier 1 symbols accessible from JavaScript/TypeScript
- [ ] Package builds with `wasm-pack build`
- [ ] `DataHasher` and `InstanceHasher` streaming types work in WASM
- [x] `SumHasher` streaming type works in WASM (single-pass ISCC-SUM, output matches two-hasher
    pattern)
- [ ] TypeScript declarations provide accurate type information
- [ ] Package installs cleanly via `npm install @iscc/wasm`
- [ ] Works in browser environments (no Node.js-specific APIs)
- [ ] Version synced from root `Cargo.toml` via `mise run version:sync`
- [ ] Per-crate README renders correctly on npmjs.com
