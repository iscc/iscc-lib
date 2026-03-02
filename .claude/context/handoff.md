## 2026-03-02 — Add GenSumCodeV0 to Go bindings

**Done:** Implemented `GenSumCodeV0` in Go bindings as the last binding (7 of 7) for issue #15.
Created `code_sum.go` with `SumCodeResult` struct and `GenSumCodeV0` function that performs
single-pass file I/O feeding both `DataHasher` and `InstanceHasher`, then composes the ISCC-CODE via
`GenIsccCodeV0`. Created `code_sum_test.go` with 4 tests.

**Files changed:**

- `packages/go/code_sum.go`: New file — `SumCodeResult` struct and `GenSumCodeV0(path, bits, wide)`
    function
- `packages/go/code_sum_test.go`: New file — 4 tests: equivalence, result fields, non-existent file
    error, wide mode

**Verification:** All 151 Go tests pass (147 existing + 4 new). `go vet ./...` clean.
`mise run check` passes — all 14 pre-commit hooks clean. Equivalence verified: `GenSumCodeV0` output
matches separate `GenDataCodeV0` + `GenInstanceCodeV0` + `GenIsccCodeV0` for the same file content.

**Next:** Issue #15 (`gen_sum_code_v0` propagation) is now complete across all 7 bindings: Rust core
✅, Python ✅, Node.js ✅, WASM ✅, C FFI ✅, Java ✅, Go ✅. Review agent should close issue #15.
Outstanding cleanup: update FFI module docstring count ("9→10 gen functions") in
`crates/iscc-ffi/src/lib.rs`.

**Notes:** Clean implementation with no surprises. Go's `io.EOF` handling pattern (process `n` bytes
before checking for EOF) works exactly as specified in next.md. The function follows the established
Go binding patterns (error wrapping, struct return, defer cleanup).
