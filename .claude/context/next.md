# Next Work Package

## Step: Implement gen_sum_code_v0 + SumCodeResult in Rust core

## Goal

Add `gen_sum_code_v0` and `SumCodeResult` to the Rust core crate — the 32nd and final Tier 1 symbol.
This function performs single-pass file I/O that feeds both `DataHasher` (CDC/MinHash) and
`InstanceHasher` (BLAKE3) from the same read buffer, then composes the final ISCC-CODE internally.
This is the first function in the crate that introduces file I/O.

## Scope

- **Create**: (none)
- **Modify**:
    - `crates/iscc-lib/src/types.rs` — add `SumCodeResult` struct
    - `crates/iscc-lib/src/lib.rs` — add `gen_sum_code_v0` function + export + tests
- **Reference**:
    - `crates/iscc-lib/src/streaming.rs` — `DataHasher` and `InstanceHasher` API
    - `crates/iscc-lib/src/types.rs` — existing result type patterns
    - `.claude/context/specs/rust-core.md` — spec for signature and verification criteria

## Not In Scope

- Propagating `gen_sum_code_v0` to any binding crate (Python, Node.js, WASM, C FFI, Java, Go) —
    separate steps after core lands
- Updating README, per-crate READMEs, or documentation site for the new function
- Adding `units: Vec<String>` field to `SumCodeResult` (optional field per spec — defer until
    bindings need it or add if trivial)
- Benchmarks for `gen_sum_code_v0` (add after bindings to benchmark full pipeline)
- WASM design decisions for path-based I/O — not relevant for core crate

## Implementation Notes

**SumCodeResult** — add to `types.rs` following the existing pattern (`#[non_exhaustive]`,
`#[derive(Debug, Clone, PartialEq, Eq)]`):

```rust
/// Result of [`gen_sum_code_v0`](crate::gen_sum_code_v0).
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct SumCodeResult {
    /// Composite ISCC-CODE string (e.g., `"ISCC:KAC..."`).
    pub iscc: String,
    /// Hex-encoded BLAKE3 multihash (`"1e20..."`) of the file.
    pub datahash: String,
    /// Byte length of the file.
    pub filesize: u64,
}
```

**gen_sum_code_v0** — add to `lib.rs` near the other `gen_*_v0` functions:

1. Open file with `std::fs::File::open(path)`, map I/O errors to `IsccError::InvalidInput`
2. Create `DataHasher::new()` and `InstanceHasher::new()`
3. Read loop: `let mut buf = vec![0u8; IO_READ_SIZE];` — read chunks, feed both hashers
4. Finalize both: `data_hasher.finalize(bits)?` and `instance_hasher.finalize(bits)?`
5. Compose ISCC-CODE: call `gen_iscc_code_v0(&[&data_result.iscc, &instance_result.iscc], wide)?`
6. Return
    `SumCodeResult { iscc: iscc_result.iscc, datahash: instance_result.datahash, filesize: instance_result.filesize }`

**Error handling**: Use `IsccError::InvalidInput` for file-not-found and read errors — this matches
the existing error type. The format should include the OS error message (e.g.,
`"Cannot open file: No such file or directory (os error 2)"`).

**Tests** — write test functions (not a test class) in `lib.rs`'s `#[cfg(test)]` module:

1. **Equivalence test**: Write known bytes to a temp file (`std::env::temp_dir()` +
    `std::fs::write`), call `gen_sum_code_v0`, compare against separate
    `gen_data_code_v0(data, bits)` + `gen_instance_code_v0(data, bits)` →
    `gen_iscc_code_v0(&[data_iscc, instance_iscc], wide)`. Assert `iscc`, `datahash`, and
    `filesize` all match.
2. **Empty file**: Test with a zero-byte file — should still produce valid ISCC.
3. **File not found**: Assert `gen_sum_code_v0` returns `Err` for a nonexistent path.
4. **Wide mode**: Test with `wide=true` — verify output differs from `wide=false`.
5. **Different bit lengths**: Test at least `bits=64` and `bits=128`.

Use `std::fs::write` + `std::fs::remove_file` for temp files (no new dev dependencies needed). Clean
up temp files even on test failure — or use unique filenames under `std::env::temp_dir()`.

**Export**: Add `pub use` or ensure `gen_sum_code_v0` is `pub fn` at crate root. `SumCodeResult` is
already picked up by `pub use types::*`.

## Verification

- `cargo test -p iscc-lib` passes (303 existing + ≥5 new tests)
- `cargo clippy -p iscc-lib -- -D warnings` clean
- `cargo fmt -p iscc-lib --check` clean
- `iscc_lib::gen_sum_code_v0` is importable from crate root
- `iscc_lib::SumCodeResult` is importable from crate root
- Test asserts
    `gen_sum_code_v0(path, 64, false).iscc == gen_iscc_code_v0(&[data_iscc, instance_iscc], false).iscc`
    for the same file content
- Test asserts `gen_sum_code_v0(nonexistent_path, 64, false)` returns `Err`

## Done When

All verification criteria pass — `gen_sum_code_v0` and `SumCodeResult` are implemented, tested, and
exported from the Rust core crate with equivalence to the two-pass approach verified.
