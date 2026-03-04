# Next Work Package

## Step: Fix stale Go code examples in README and howto guide

## Goal

Fix two verified documentation bugs where Go code examples use outdated function signatures that
would cause compilation errors for developers following the docs. The README Go quickstart uses a
completely obsolete WASM-based runtime pattern, and the howto guide's `GenSumCodeV0` example is
missing a required parameter.

## Scope

- **Create**: (none)
- **Modify**:
    - `README.md` — rewrite Go quickstart section (lines 160-188) to use pure Go package-level
        function pattern matching the actual `GenMetaCodeV0` signature
    - `docs/howto/go.md` — fix `GenSumCodeV0` call on line 206 to include the missing `addUnits`
        parameter
- **Reference**:
    - `packages/go/code_meta.go` — actual `GenMetaCodeV0` signature:
        `func GenMetaCodeV0(name string, description, meta *string, bits uint32) (*MetaCodeResult, error)`
    - `packages/go/code_sum.go` — actual `GenSumCodeV0` signature:
        `func GenSumCodeV0(path string, bits uint32, wide bool, addUnits bool) (*SumCodeResult, error)`
    - `docs/howto/go.md` lines 30-50 — correct Go quickstart pattern (use as model for README fix)

## Not In Scope

- Rewriting any other language's quickstart examples in the README
- Adding new Go examples or expanding the Go howto guide beyond the signature fix
- Creating a `docs/go-api.md` API reference page (no Go API page exists yet — separate step)
- Updating the Go section format or structure — only fix the code accuracy

## Implementation Notes

**README.md Go quickstart** (lines 160-188): The current code shows:

```go
ctx := context.Background()
rt, err := iscc.NewRuntime(ctx)
defer rt.Close(ctx)
code, err := rt.GenMetaCodeV0(ctx, "ISCC Test Document!", nil, nil, 64)
```

This pattern is from when Go used WASM via wazero. The Go bindings are now pure Go with
package-level functions. Replace with the same pattern used in `docs/howto/go.md` lines 30-35:

```go
result, err := iscc.GenMetaCodeV0("ISCC Test Document!", nil, nil, 64)
```

Remove the `"context"` import. Keep the `iscc` import path as
`github.com/iscc/iscc-lib/packages/go`. Print `result.Iscc` (not bare `code`).

**docs/howto/go.md line 206**: Change:

```go
result, err := iscc.GenSumCodeV0("example.bin", 64, false)
```

to:

```go
result, err := iscc.GenSumCodeV0("example.bin", 64, false, false)
```

This adds the missing `addUnits bool` 4th parameter.

## Verification

- `grep -c 'NewRuntime\|rt\.Close\|rt\.Gen' README.md` returns 0 (no stale runtime pattern)
- `grep 'GenMetaCodeV0' README.md` shows package-level call matching actual signature
- `grep 'GenSumCodeV0' docs/howto/go.md` shows 4-argument call matching actual signature
- `uv run zensical build` exits 0 (docs site builds cleanly)
- `mise run check` passes (all pre-commit hooks green)

## Done When

All five verification criteria pass — Go code examples in README.md and docs/howto/go.md match
actual function signatures in packages/go/.
