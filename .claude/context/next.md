# Next Work Package

## Step: Build iscc-ffi as wasm32-wasip1 module with alloc/dealloc helpers

## Goal

Enable the C FFI crate to compile as a WASI reactor module suitable for wazero consumption. This is
the foundational step for Go bindings — without a working `.wasm` binary, no Go code can proceed.

## Scope

- **Create**: (none)
- **Modify**: `crates/iscc-ffi/src/lib.rs` (add `iscc_alloc` and `iscc_dealloc` functions)
- **Reference**: `crates/iscc-ffi/Cargo.toml`, `crates/iscc-ffi/src/lib.rs` (existing FFI patterns),
    `.claude/context/target.md` (Go bindings section), `notes/02-language-bindings.md`

## Not In Scope

- Creating `packages/go/` directory or any Go wrapper code
- Adding a CI job for WASM/WASI builds or Go tests
- Optimizing the `.wasm` binary size (LTO, wasm-opt, etc.)
- Changing existing FFI function signatures or return types
- Building in release mode or configuring wasm-opt profiles
- Addressing the normal-priority FFI video frame allocation issue (separate concern)

## Implementation Notes

**alloc/dealloc helpers** — The Go/wazero host needs to allocate memory inside the WASM module to
pass strings and byte buffers across the boundary. Add two `extern "C"` functions:

```rust
/// Allocate `size` bytes of WASM-side memory. Returns a pointer the host can
/// write into. The host must call `iscc_dealloc` to free this memory.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn iscc_alloc(size: usize) -> *mut u8 {
    let layout = std::alloc::Layout::from_size_align(size, 1).unwrap();
    unsafe { std::alloc::alloc(layout) }
}

/// Free `size` bytes of WASM-side memory at `ptr`, previously allocated by
/// `iscc_alloc`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn iscc_dealloc(ptr: *mut u8, size: usize) {
    if ptr.is_null() { return; }
    let layout = std::alloc::Layout::from_size_align(size, 1).unwrap();
    unsafe { std::alloc::dealloc(ptr, layout) };
}
```

**Placement** — Add these at the top of `lib.rs`, right after the `clear_last_error` function and
before the `result_to_c_string` helper. Group them under a comment section
`// --- Memory allocation helpers (for WASM host) ---`.

**WASM target setup** — Before building, install the target: `rustup target add wasm32-wasip1`. The
existing `crate-type = ["cdylib", "staticlib"]` in Cargo.toml works — cargo produces a `.wasm` file
from the `cdylib` target.

**Null pointer guard** — `iscc_dealloc` must check for null before calling `dealloc` (calling
`std::alloc::dealloc` with null is UB). The `iscc_alloc` function should handle `size == 0` by
returning a non-null dangling pointer (use `std::alloc::Layout` which panics on zero size — guard
with an early return of `std::ptr::NonNull::dangling().as_ptr()` for size 0).

**Clippy** — The new functions use `unsafe` in an `unsafe fn`. Check the Rust edition in
`crates/iscc-ffi/Cargo.toml` — if edition 2024, use `#[unsafe(no_mangle)]` (matching the JNI crate);
if edition 2021, use `#[no_mangle]`. Match existing FFI crate style.

**Existing tests** — The alloc/dealloc functions don't need unit tests in this step (they're trivial
wrappers around `std::alloc`). What matters is that existing FFI tests continue to pass and the WASM
build succeeds.

## Verification

- `rustup target add wasm32-wasip1` succeeds (idempotent)
- `cargo build -p iscc-ffi --target wasm32-wasip1` compiles without errors
- `ls target/wasm32-wasip1/debug/iscc_ffi.wasm` — file exists
- `cargo test -p iscc-ffi` passes all 62 existing tests
- `cargo clippy -p iscc-ffi -- -D warnings` clean
- `grep 'fn iscc_alloc' crates/iscc-ffi/src/lib.rs` returns 1 match
- `grep 'fn iscc_dealloc' crates/iscc-ffi/src/lib.rs` returns 1 match

## Done When

All verification criteria pass: iscc-ffi compiles to wasm32-wasip1 with exported alloc/dealloc
helpers, existing tests still pass, and clippy is clean.
