## 2026-02-24 — Build iscc-ffi as wasm32-wasip1 module with alloc/dealloc helpers

**Done:** Added `iscc_alloc` and `iscc_dealloc` extern "C" functions to the FFI crate for WASM host
memory management. The crate now compiles as a wasm32-wasip1 module, producing a ~10.5MB debug
`.wasm` binary from the existing `cdylib` crate type.

**Files changed:**

- `crates/iscc-ffi/src/lib.rs`: Added `iscc_alloc(size) -> *mut u8` and `iscc_dealloc(ptr, size)`
    under a `// --- Memory allocation helpers (for WASM host) ---` section, placed between
    `clear_last_error` and `result_to_c_string`. Both use `#[unsafe(no_mangle)]` matching the
    crate's edition 2024 style. `iscc_alloc` returns a dangling non-null pointer for `size == 0`.
    `iscc_dealloc` guards against both null pointers and zero-size frees.

**Verification:**

- [x] `rustup target add wasm32-wasip1` succeeds
- [x] `cargo build -p iscc-ffi --target wasm32-wasip1` compiles without errors
- [x] `target/wasm32-wasip1/debug/iscc_ffi.wasm` exists (10,992,662 bytes)
- [x] `cargo test -p iscc-ffi` passes all 62 existing tests
- [x] `cargo clippy -p iscc-ffi -- -D warnings` clean
- [x] `grep 'fn iscc_alloc'` returns exactly 1 match
- [x] `grep 'fn iscc_dealloc'` returns exactly 1 match
- [x] `mise run check` passes all 14 pre-commit hooks

**Next:** The WASM module is ready for Go/wazero integration. The next step would be creating the
`packages/go/` directory with the Go wrapper code that loads this `.wasm` module and calls the
exported FFI functions through wazero.

**Notes:** No surprises. The existing `crate-type = ["cdylib", "staticlib"]` in Cargo.toml works
without modification for WASM — cargo produces the `.wasm` from the `cdylib` target automatically.
The debug binary is ~10.5MB; release + wasm-opt would reduce this significantly but is explicitly
out of scope for this step.
