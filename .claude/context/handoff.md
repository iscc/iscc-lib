## 2026-02-24 — Review of: Build iscc-ffi as wasm32-wasip1 module with alloc/dealloc helpers

**Verdict:** PASS

**Summary:** The advance agent added `iscc_alloc` and `iscc_dealloc` helper functions to the FFI
crate, enabling WASM host memory management for Go/wazero bindings. The implementation is clean,
well-documented, correctly handles edge cases (null pointers, zero-size allocations), and compiles
to a ~10.5MB wasm32-wasip1 module. All existing tests pass unchanged.

**Verification:**

- [x] `rustup target add wasm32-wasip1` succeeds (idempotent)
- [x] `cargo build -p iscc-ffi --target wasm32-wasip1` compiles without errors
- [x] `ls target/wasm32-wasip1/debug/iscc_ffi.wasm` — file exists (10,992,662 bytes)
- [x] `cargo test -p iscc-ffi` passes all 62 existing tests
- [x] `cargo clippy -p iscc-ffi -- -D warnings` clean
- [x] `grep 'fn iscc_alloc'` returns exactly 1 match
- [x] `grep 'fn iscc_dealloc'` returns exactly 1 match
- [x] `mise run check` passes all 14 pre-commit hooks
- [x] No quality gate circumvention in diff

**Issues found:**

- (none)

**Next:** Create the `packages/go/` directory with the Go module scaffold and wazero bridge. This is
step 2 of the Go bindings multi-step plan: set up `go.mod`, embed the `.wasm` binary, and implement
the wazero runtime initialization + basic memory helpers that call `iscc_alloc`/`iscc_dealloc`.

**Notes:** The existing `crate-type = ["cdylib", "staticlib"]` works without modification for WASM
targets. The `thread_local!` macro in the FFI crate (for error storage) compiles for wasm32-wasip1
since WASM is single-threaded. Debug binary is ~10.5MB; release + wasm-opt optimization is a
separate future step.
