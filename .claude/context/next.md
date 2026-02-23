# Next Work Package

## Step: Add 4 algorithm primitives to Node.js bindings

## Goal

Expand Node.js bindings from 17/23 to 21/23 Tier 1 symbols by adding `alg_simhash`,
`alg_minhash_256`, `alg_cdc_chunks`, and `soft_hash_video_v0`. These are the complex-type-signature
functions that require careful napi-rs type mapping (byte buffers, nested arrays, owned vs borrowed
data). Node.js goes first as the design template; WASM and C FFI will replicate in follow-up steps.

## Scope

- **Modify**: `crates/iscc-napi/src/lib.rs` — add 4 napi functions
- **Modify**: `crates/iscc-napi/__tests__/functions.test.mjs` — add unit tests for all 4 functions
- **Reference**: `crates/iscc-py/src/lib.rs` (PyO3 equivalents — lines 238–273),
    `crates/iscc-lib/src/simhash.rs` (`alg_simhash` signature), `crates/iscc-lib/src/minhash.rs`
    (`alg_minhash_256` signature), `crates/iscc-lib/src/cdc.rs` (`alg_cdc_chunks` signature),
    `crates/iscc-lib/src/lib.rs` (`soft_hash_video_v0` at line 530)

## Not In Scope

- Adding these 4 functions to WASM or C FFI bindings — those are separate follow-up steps
- `DataHasher`/`InstanceHasher` streaming classes — separate step with different design pattern
- Structured return types for gen functions (returning objects instead of plain `.iscc` strings)
- Changes to the Rust core crate — all 4 functions are already exported and tested
- Conformance test additions — the existing conformance tests cover gen functions; algorithm
    primitive tests are unit tests

## Implementation Notes

### Type mappings

Each function wraps the corresponding `iscc_lib` function with napi-rs type conversions:

1. **`alg_simhash`** — `(hash_digests: Vec<Buffer>) -> Buffer`

    - Rust signature: `alg_simhash(hash_digests: &[impl AsRef<[u8]>]) -> Vec<u8>`
    - napi-rs `Buffer` implements `AsRef<[u8]>`, so `iscc_lib::alg_simhash(&hash_digests)` works
        directly (pass `&Vec<Buffer>` which coerces to `&[Buffer]`)
    - Convert `Vec<u8>` result to `Buffer` via `.into()`
    - Infallible — no error mapping needed

2. **`alg_minhash_256`** — `(features: Vec<u32>) -> Buffer`

    - Rust signature: `alg_minhash_256(features: &[u32]) -> Vec<u8>`
    - Pass `&features` directly
    - Convert `Vec<u8>` result to `Buffer` via `.into()`
    - Infallible — no error mapping needed

3. **`alg_cdc_chunks`** — `(data: Buffer, utf32: bool, avg_chunk_size: Option<u32>) -> Vec<Buffer>`

    - Rust signature: `alg_cdc_chunks(data: &[u8], utf32: bool, avg_chunk_size: u32) -> Vec<&[u8]>`
    - Default `avg_chunk_size` to 1024 via `.unwrap_or(1024)`
    - The return type `Vec<&[u8]>` borrows from `data` — convert each chunk to owned `Buffer` via
        `.iter().map(|c| Buffer::from(c.to_vec())).collect()`
    - Infallible — no error mapping needed

4. **`soft_hash_video_v0`** —
    `(frame_sigs: Vec<Vec<i32>>, bits: Option<u32>) -> napi::Result<Buffer>`

    - Rust signature: `soft_hash_video_v0(&[Vec<i32>], u32) -> IsccResult<Vec<u8>>`
    - This follows the exact same pattern as `gen_video_code_v0` (which already takes
        `Vec<Vec<i32>>`)
    - Default `bits` to 64 via `.unwrap_or(64)`
    - Map error with `.map_err(|e| napi::Error::from_reason(e.to_string()))`
    - Convert `Vec<u8>` result to `Buffer` via `.into()`

### Naming and placement

Use `#[napi(js_name = "...")]` for all 4 functions to preserve snake_case naming (matching the
existing pattern). Place them after the existing `sliding_window` function at the end of the file,
grouped under an `// ── Algorithm primitives ──` section comment.

### Tests

Add to `__tests__/functions.test.mjs` in a new `describe('algorithm primitives', ...)` section:

- **`alg_simhash`**: Feed known byte buffers, verify output is a Buffer of expected length. Test
    empty input returns 32 zero bytes. Test with single digest returns same bytes back
- **`alg_minhash_256`**: Feed a `[u32]` array, verify output is a 32-byte Buffer. Test with known
    feature values
- **`alg_cdc_chunks`**: Feed known data, verify chunks concatenate back to original. Test empty
    input returns one empty chunk. Test with `utf32=true`
- **`soft_hash_video_v0`**: Feed frame signatures (reuse from conformance test vectors), verify
    output is a Buffer of length `bits/8`. Test empty input throws error

Use Node.js built-in `node:test` + `node:assert` (zero extra dependencies), matching existing test
patterns.

## Verification

- `cargo build -p iscc-napi` compiles without errors
- `cargo clippy --workspace --all-targets -- -D warnings` is clean
- `node --test crates/iscc-napi/__tests__/functions.test.mjs` passes all existing + new tests
- `node --test crates/iscc-napi/__tests__/` passes all tests (both conformance and function suites)
- `cargo test --workspace` passes with 268+ tests (no regressions in other crates)
- All 4 new functions callable from JavaScript: `alg_simhash`, `alg_minhash_256`, `alg_cdc_chunks`,
    `soft_hash_video_v0`

## Done When

All verification criteria pass: 4 new napi functions compile, clippy is clean workspace-wide, and
all Node.js tests (existing 73 + new algorithm primitive tests) pass via `node --test`.
