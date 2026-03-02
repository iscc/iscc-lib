# Next Work Package

## Step: Expose add_units/units in Go binding (issue #21)

## Goal

Add `addUnits bool` parameter to `GenSumCodeV0` and `Units []string` field to `SumCodeResult` in the
pure Go binding, completing the last binding for issue #21. This lets Go callers get the individual
Data-Code and Instance-Code ISCC strings from a single optimized call.

## Scope

- **Create**: (none)
- **Modify**:
    - `packages/go/code_sum.go` — add `addUnits bool` parameter, add `Units []string` field to
        `SumCodeResult`, conditionally populate when `addUnits=true`
    - `packages/go/code_sum_test.go` — update existing 4 tests to pass `false` as 4th arg, add 3 new
        tests (units enabled, units disabled, units content verification)
- **Reference**:
    - `crates/iscc-lib/src/code_sum.rs` — Rust core implementation for pattern reference
    - `crates/iscc-jni/src/lib.rs` — JNI binding for comparison (most recent units exposure)

## Not In Scope

- Updating docs (`rust-api.md`, `architecture.md`) — that's a separate step after all bindings done
- Closing issue #21 — the review agent handles issue resolution
- Renaming `TestPureGo*` test prefixes — cosmetic cleanup, not part of this work
- Any changes to other binding crates (all already complete)

## Implementation Notes

The Go function already computes `dataResult.Iscc` and `instanceResult.Iscc` internally (needed for
`GenIsccCodeV0`). The change is straightforward:

1. **`SumCodeResult` struct**: Add `Units []string` field. When `addUnits=false`, leave it `nil` (Go
    zero value). When `addUnits=true`, set to `[]string{dataResult.Iscc, instanceResult.Iscc}`.

2. **`GenSumCodeV0` signature**: Change from `(path string, bits uint32, wide bool)` to
    `(path string, bits uint32, wide bool, addUnits bool)`. This is a breaking change for Go
    callers but matches all other bindings. No default parameter support in Go.

3. **Conditional population**: After `dh.Finalize(bits)` and `ih.Finalize(bits)` but before
    constructing the result, check `addUnits`. If true, create the units slice from the two ISCC
    strings. Pattern from Rust core:

    ```go
    var units []string
    if addUnits {
        units = []string{dataResult.Iscc, instanceResult.Iscc}
    }
    ```

4. **Tests**: Update all 4 existing `GenSumCodeV0` call sites to pass `false` as the 4th argument.
    Add 3 new tests following the pattern from other bindings:

    - `TestGenSumCodeV0UnitsEnabled` — call with `addUnits=true`, verify `Units` is non-nil and has
        length 2
    - `TestGenSumCodeV0UnitsDisabled` — call with `addUnits=false`, verify `Units` is nil
    - `TestGenSumCodeV0UnitsContent` — call with `addUnits=true`, verify each unit string starts with
        "ISCC:" and that units match separate `GenDataCodeV0`/`GenInstanceCodeV0` calls

## Verification

- `cd packages/go && go test ./...` passes (all existing + 3 new tests)
- `cd packages/go && go vet ./...` clean
- `grep -c 'func Test' packages/go/code_sum_test.go` shows 7 (4 existing + 3 new)
- `grep 'Units' packages/go/code_sum.go` shows the field declaration and population logic

## Done When

All verification criteria pass: `go test` passes with 7 sum code tests (4 updated + 3 new), `go vet`
clean, and `SumCodeResult.Units` is correctly gated by `addUnits`.
