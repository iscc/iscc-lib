## 2026-03-02 — Expose add_units/units in Go binding (issue #21)

**Done:** Added `addUnits bool` parameter to `GenSumCodeV0` and `Units []string` field to
`SumCodeResult` in the pure Go binding. When `addUnits=true`, Units contains the individual
Data-Code and Instance-Code ISCC strings; when `false`, Units is nil. Updated all 4 existing test
call sites and added 3 new tests.

**Files changed:**

- `packages/go/code_sum.go`: Added `Units []string` field to `SumCodeResult`, added `addUnits bool`
    parameter to `GenSumCodeV0`, conditional population logic after finalization
- `packages/go/code_sum_test.go`: Updated 4 existing tests to pass `false` as 4th arg, added
    `TestGenSumCodeV0UnitsEnabled`, `TestGenSumCodeV0UnitsDisabled`, `TestGenSumCodeV0UnitsContent`

**Verification:**

- `cd packages/go && go test ./...` — all tests pass (7 sum code tests: 4 updated + 3 new)
- `cd packages/go && go vet ./...` — clean, no warnings
- `grep -c 'func Test' packages/go/code_sum_test.go` — 7 ✅
- `grep 'Units' packages/go/code_sum.go` — shows field declaration and population logic ✅
- `mise run check` — 14/14 pre-commit hooks pass

**Next:** Issue #21 is now complete across all 7 bindings (Rust core → Python → Node.js → WASM → C
FFI → JNI → Go). Next steps: update documentation (`rust-api.md`, `architecture.md`) to reflect the
`add_units`/`units` additions across all bindings, then close issue #21.

**Notes:** Straightforward implementation. The Go binding already computed `dataResult.Iscc` and
`instanceResult.Iscc` internally, so the change only required threading the `addUnits` parameter and
conditionally building the `[]string` slice. No surprises or technical debt.
