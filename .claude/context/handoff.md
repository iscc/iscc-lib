## 2026-03-01 — Implement gen_sum_code_v0 + SumCodeResult in Rust core

**Done:** Added `SumCodeResult` struct to `types.rs` and `gen_sum_code_v0` function to `lib.rs`. The
function opens a file, reads it in `IO_READ_SIZE` chunks, feeds both `DataHasher` and
`InstanceHasher` from the same buffer, then composes the final ISCC-CODE. This is the 32nd and final
Tier 1 symbol for the Rust core crate.

**Files changed:**

- `crates/iscc-lib/src/types.rs`: Added `SumCodeResult` struct with `iscc`, `datahash`, `filesize`
    fields
- `crates/iscc-lib/src/lib.rs`: Added `gen_sum_code_v0(path, bits, wide)` function + 7 test
    functions

**Verification:** All quality gates pass:

- `cargo test -p iscc-lib`: 310 tests pass (303 existing + 7 new gen_sum_code_v0 tests)
- `cargo clippy -p iscc-lib -- -D warnings`: clean
- `cargo fmt -p iscc-lib --check`: clean
- `mise run check`: all 14 pre-commit hooks pass
- `gen_sum_code_v0` and `SumCodeResult` are importable from crate root (via `pub fn` and
    `pub use types::*`)
- Equivalence verified:
    `gen_sum_code_v0(path, bits, wide).iscc == gen_iscc_code_v0(&[data_iscc, instance_iscc], wide).iscc`
    for same content
- Error case verified: nonexistent path returns `Err` with "Cannot open file" message
- Wide mode verified: 64-bit codes produce same result for narrow/wide (below threshold), 128-bit
    codes produce different results
- Both 64-bit and 128-bit tested with equivalence assertion

**Next:** Propagate `gen_sum_code_v0` + `SumCodeResult` to binding crates. Start with Python
bindings (`crates/iscc-py/`) since it's the primary consumer, then Node.js, WASM, C FFI, Java, and
Go. The Go binding will need a pure-Go implementation (not a Rust binding wrapper).

**Notes:** No surprises. The implementation is straightforward — `DataHasher` and `InstanceHasher`
compose cleanly in a single read loop. `SumCodeResult` follows the exact same `#[non_exhaustive]`
pattern as all other result types. The `units: Vec<String>` field was deferred per next.md scope.
