# Next Work Package

## Step: Add GenSumCodeV0 to Go bindings

## Goal

Implement `GenSumCodeV0` in the Go bindings (`packages/go/`) — the last binding (7 of 7) for issue
#15. This completes `gen_sum_code_v0` propagation across all language bindings.

## Scope

- **Create**: `packages/go/code_sum.go`, `packages/go/code_sum_test.go`
- **Modify**: none (the new function is additive; no existing files need changes)
- **Reference**:
    - `crates/iscc-lib/src/lib.rs` lines 967–998 (Rust `gen_sum_code_v0`)
    - `crates/iscc-lib/src/types.rs` lines 98–105 (`SumCodeResult` struct)
    - `packages/go/code_data.go` (Go `DataHasher`, `GenDataCodeV0` pattern)
    - `packages/go/code_instance.go` (Go `InstanceHasher`, `GenInstanceCodeV0` pattern)
    - `packages/go/code_iscc.go` (Go `GenIsccCodeV0`)
    - `packages/go/code_data_test.go` (Go test pattern for gen functions)

## Not In Scope

- Updating the FFI module docstring count ("9→10 gen functions") in `crates/iscc-ffi/src/lib.rs` —
    that's a separate cleanup step
- README or docs updates mentioning `gen_sum_code_v0` in Go — tracked for a follow-up step
- Closing issue #15 — the review agent handles that after verification
- Adding conformance vectors for `gen_sum_code_v0` to `data.json` — the reference implementation has
    no `gen_sum_code_v0` vectors; equivalence is verified against the two-pass approach

## Implementation Notes

**`code_sum.go`:**

1. Define `SumCodeResult` struct with fields: `Iscc string`, `Datahash string`, `Filesize uint64`
    (mirrors `InstanceCodeResult` fields)
2. Implement `GenSumCodeV0(path string, bits uint32, wide bool) (*SumCodeResult, error)`:
    - Open file with `os.Open(path)`, defer close
    - Create `NewDataHasher()` and `NewInstanceHasher()`
    - Read file in a loop using `IoReadSize`-sized buffer (4 MB), feeding both hashers via `.Push()`
    - Handle `io.EOF`: Go's `os.File.Read` returns `(n, io.EOF)` when it reads the last chunk — must
        process `n` bytes before breaking. The idiomatic pattern is:
        ```
        for {
            n, err := f.Read(buf)
            if n > 0 {
                dh.Push(buf[:n])
                ih.Push(buf[:n])
            }
            if err == io.EOF {
                break
            }
            if err != nil {
                return nil, fmt.Errorf("iscc: read file: %w", err)
            }
        }
        ```
    - Call `dh.Finalize(bits)` and `ih.Finalize(bits)` to get data and instance results
    - Compose ISCC-CODE via `GenIsccCodeV0([]string{dataResult.Iscc, instanceResult.Iscc}, wide)`
    - Return
        `&SumCodeResult{Iscc: isccResult.Iscc, Datahash: instanceResult.Datahash, Filesize: instanceResult.Filesize}`
    - Wrap file open/read errors in descriptive error messages

**`code_sum_test.go`:**

Write 4 tests mirroring other binding test suites:

1. **Equivalence test** (`TestGenSumCodeV0Equivalence`) — create a temp file with known bytes, call
    `GenSumCodeV0`, also call `GenDataCodeV0` + `GenInstanceCodeV0` + `GenIsccCodeV0` separately on
    the same data, verify ISCC codes match, and `datahash`/`filesize` match instance result
2. **Result fields test** (`TestGenSumCodeV0ResultFields`) — verify `SumCodeResult` has non-empty
    `Iscc` (starts with "ISCC:"), `Datahash` (starts with "1e20"), and correct `Filesize`
3. **Non-existent file error** (`TestGenSumCodeV0ErrorNonExistent`) — call with a path that doesn't
    exist, verify non-nil error
4. **Wide mode test** (`TestGenSumCodeV0WideMode`) — call with `bits=128, wide=true` and
    `bits=128, wide=false`, verify ISCC outputs differ but `datahash` and `filesize` are identical
    (must use bits=128 since 64-bit codes produce identical wide/non-wide output)

Use `os.CreateTemp` for temp files with `defer os.Remove(f.Name())` for cleanup.

**Pattern notes:**

- Go file I/O is idiomatic: `os.Open` + `defer f.Close()` + `f.Read(buf)` loop
- The function accepts `string` path (not `[]byte` like other Go gen functions) because it does its
    own file I/O — same pattern as the Rust version
- Error wrapping: use `fmt.Errorf("iscc: ...: %w", err)` consistent with other Go gen functions
- Package docstring follows pattern: `// Pure Go implementation of ISCC-SUM generation...`

## Verification

- `cd packages/go && go test -run TestGenSumCodeV0 -v` passes (4 new tests)
- `cd packages/go && go test ./...` passes (147 existing + 4 new = 151 tests)
- `cd packages/go && go vet ./...` clean
- `GenSumCodeV0` equivalence: output matches separate `GenDataCodeV0` + `GenInstanceCodeV0` +
    `GenIsccCodeV0` for the same file content

## Done When

All verification criteria pass: `GenSumCodeV0` exists in Go with `SumCodeResult`, all 151 Go tests
pass including 4 new tests for equivalence/fields/error/wide, and `go vet` is clean.
