## 2026-02-23 — Add 4 algorithm primitives to Node.js bindings

**Done:** Added `alg_simhash`, `alg_minhash_256`, `alg_cdc_chunks`, and `soft_hash_video_v0` as napi
functions in `crates/iscc-napi/src/lib.rs`, expanding Node.js bindings from 17/23 to 21/23 Tier 1
symbols. All 4 functions follow the existing thin-wrapper pattern with appropriate napi-rs type
conversions (Buffer for byte data, Vec<Buffer> for chunk output, Option for defaults).

**Files changed:**

- `crates/iscc-napi/src/lib.rs`: Added 4 napi functions under `// ── Algorithm primitives ──`
    section after `sliding_window`. Type mappings: `Vec<Buffer>` input for simhash (Buffer
    implements `AsRef<[u8]>`), `Vec<u32>` for minhash, `Buffer` + `Option<u32>` for cdc,
    `Vec<Vec<i32>>` + `Option<u32>` for soft_hash_video.
- `crates/iscc-napi/__tests__/functions.test.mjs`: Added 16 tests in 4 new `describe` blocks
    (alg_simhash: 4 tests, alg_minhash_256: 3 tests, alg_cdc_chunks: 5 tests, soft_hash_video_v0: 4
    tests). Tests cover empty input, determinism, output length, error throwing, and data integrity
    (chunks reassemble to original).

**Verification:**

- `cargo build -p iscc-napi` compiles cleanly
- `cargo clippy --workspace --all-targets -- -D warnings` is clean
- `node --test crates/iscc-napi/__tests__/functions.test.mjs` passes all 43 tests (27 existing + 16
    new)
- `node --test crates/iscc-napi/__tests__/` passes all 89 tests (73 existing + 16 new)
- `cargo test --workspace` passes 268 tests (no regressions)
- All 14 pre-commit hooks pass (`mise run check`)
- All 4 functions callable from JavaScript: `alg_simhash`, `alg_minhash_256`, `alg_cdc_chunks`,
    `soft_hash_video_v0`

**Next:** Node.js is now at 21/23 Tier 1 symbols. The remaining 2 are `DataHasher` and
`InstanceHasher` streaming classes — these require a different design pattern (napi-rs class with
state) and should be a separate step. Good candidates for next work:

1. **Add 4 algorithm primitives to WASM bindings** — replicate the same 4 functions in
    `crates/iscc-wasm/src/lib.rs` (WASM type mappings differ: `&[u8]`/`JsValue` with
    `serde_wasm_bindgen`)
2. **Add 4 algorithm primitives to C FFI bindings** — replicate in `crates/iscc-ffi/src/lib.rs` (C
    type mappings: `*const u8` + `usize`, output buffers with caller-provided allocation)
3. **Streaming hashers for Node.js** — `DataHasher`/`InstanceHasher` napi-rs classes with
    `new() -> update(Buffer) -> finalize() -> Result` pattern

**Notes:** The `alg_simhash` binding works cleanly because napi-rs `Buffer` implements
`AsRef<[u8]>`, so `&Vec<Buffer>` coerces to `&[Buffer]` which satisfies `&[impl AsRef<[u8]>]` — no
manual conversion needed. The `alg_cdc_chunks` return type requires `.to_vec()` on each chunk slice
since the returned `Vec<&[u8]>` borrows from the input data. The `cargo fmt` hook auto-fixed the
`soft_hash_video_v0` signature formatting (collapsed multiline parameters to single line).
