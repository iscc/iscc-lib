# Next Work Package

## Step: Add WASM conformance tests runnable with wasm-pack test --node

## Goal

Add conformance tests for the `iscc-wasm` crate that validate all 9 `gen_*_v0` WASM-exported
functions against the vendored `data.json` test vectors, running in a Node.js WASM runtime via
`wasm-pack test --node`. This closes the "WASM has no tests" gap identified in state.md.

## Scope

- **Create**: `crates/iscc-wasm/tests/conformance.rs`
- **Modify**: `crates/iscc-wasm/Cargo.toml` (add dev-dependencies), root `Cargo.toml` (add
    `wasm-bindgen-test` to workspace dependencies)
- **Reference**: `crates/iscc-napi/__tests__/conformance.test.mjs` (JS test structure),
    `crates/iscc-lib/src/lib.rs` lines 793+ (Rust conformance test pattern with `include_str!` and
    `serde_json::Value`), `crates/iscc-wasm/src/lib.rs` (WASM function signatures)

## Implementation Notes

### Dev-dependencies setup

Add `wasm-bindgen-test` to root `Cargo.toml` workspace dependencies:

```toml
wasm-bindgen-test = "0.3"
```

Add dev-dependencies to `crates/iscc-wasm/Cargo.toml`:

```toml
[dev-dependencies]
wasm-bindgen-test = { workspace = true }
serde_json = { workspace = true }
serde-wasm-bindgen = { workspace = true }
hex = { workspace = true }
```

Note: `serde-wasm-bindgen` is already in `[dependencies]` but also needed in dev-dependencies
context for creating `JsValue` test inputs. Actually, since it's already a regular dependency, tests
can use it — no need to duplicate. Only add `wasm-bindgen-test`, `serde_json`, and `hex` as
dev-dependencies.

### Test file pattern (`crates/iscc-wasm/tests/conformance.rs`)

Use `wasm_bindgen_test` crate with `#[wasm_bindgen_test]` attribute on each test function. Load test
data with `include_str!` (compile-time embedding — works in WASM, no filesystem access needed):

```rust
use wasm_bindgen::JsValue;
use wasm_bindgen_test::*;

// Load data.json at compile time — path is relative to the test file
const DATA_JSON: &str = include_str!("../../iscc-lib/tests/data.json");
```

The path from `crates/iscc-wasm/tests/conformance.rs` to `crates/iscc-lib/tests/data.json` is
`../../iscc-lib/tests/data.json`.

### Testing functions with JsValue parameters

Three functions accept `JsValue` for complex array types: `gen_video_code_v0`, `gen_mixed_code_v0`,
`gen_iscc_code_v0`. In tests, create `JsValue` from Rust data using
`serde_wasm_bindgen::to_value()`:

```rust
let codes_js: JsValue = serde_wasm_bindgen::to_value(&codes_vec).unwrap();
let result = iscc_wasm::gen_mixed_code_v0(codes_js, Some(bits)).unwrap();
```

### Test structure

Write one `#[wasm_bindgen_test]` function per gen function (9 total), each iterating over all test
cases in the corresponding `data.json` section. Follow the same parsing patterns as the Rust core
tests:

- `"stream:"` prefix → hex-decode to `Vec<u8>`, pass as `&[u8]`
- Null/empty string inputs → `None` for `Option<String>` parameters
- JSON object meta → `serde_json::to_string()` then pass as `Some(String)`
- Image pixels → `inputs[0]` is array of u8 values → collect to `Vec<u8>`, pass as `&[u8]`
- Audio cv → `inputs[0]` is array of i32 values → collect to `Vec<i32>`, pass directly
- Video frame_sigs → `inputs[0]` is array of arrays → convert to `JsValue` via serde
- Compare only `outputs.iscc` (the ISCC string output) — same as Node.js tests

### Running tests

Install wasm-pack if needed: `curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh`

Then: `wasm-pack test --node crates/iscc-wasm`

### Important edge cases

- `gen_image_code_v0` test vectors have pixels as JSON array of integers — must be converted to
    `Vec<u8>` then passed as `&[u8]` (wasm-bindgen accepts `&[u8]` directly)
- `gen_iscc_code_v0` has an implicit `wide=false` default (not present in test vector inputs) — pass
    `None` for wide parameter
- Empty description (`""`) should be passed as `None`, matching the Node.js/Python test behavior
- The `include_str!` path must work relative to the test file location within the workspace

### Also: remove unused `js-sys` dependency

The review noted `js-sys` is declared but unused. Since the tests don't need it either, remove it
from `crates/iscc-wasm/Cargo.toml` dependencies and from `[workspace.dependencies]` in root
`Cargo.toml`.

## Verification

- `wasm-pack test --node crates/iscc-wasm` passes with 46 conformance tests (same count as Node.js)
- `cargo test -p iscc-lib` still passes (143 tests — no regression)
- `cargo clippy --workspace --all-targets -- -D warnings` is clean
- `cargo fmt --all --check` is clean
- `js-sys` dependency removed from both `crates/iscc-wasm/Cargo.toml` and root `Cargo.toml`
- No test uses mocks — all tests validate against real `data.json` conformance vectors

## Done When

All 9 `gen_*_v0` WASM functions pass conformance tests via `wasm-pack test --node`, the unused
`js-sys` dependency is removed, and all existing quality gates pass without regression.
