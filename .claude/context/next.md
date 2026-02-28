# Next Work Package

## Step: Fix stale Go example on landing page

## Goal

Replace the WASM-bridge Go code snippet on the documentation landing page (`docs/index.md`) with the
current pure Go API pattern. The existing snippet uses `NewRuntime`/`ctx` which no longer exists
after the pure Go rewrite — it confuses developers and contradicts the Go howto guide.

## Scope

- **Create**: none
- **Modify**: `docs/index.md` (lines 114-122 — the Go tab code block)
- **Reference**: `docs/howto/go.md` (lines 60-67 — the canonical `GenTextCodeV0` example)

## Not In Scope

- Tab order standardization across pages (low priority issue, needs human review for canonical
    order)
- Updating any other doc pages — only the landing page Go tab is stale
- Adding error handling to other language tabs for consistency (keep existing style)

## Implementation Notes

Replace the Go code block in the `=== "Go"` tab (lines 114-122) with the pure Go API pattern. The
current stale code:

```go
import iscc "github.com/iscc/iscc-lib/packages/go"

ctx := context.Background()
rt, _ := iscc.NewRuntime(ctx)
defer rt.Close(ctx)

code, _ := rt.GenTextCodeV0(ctx, "Hello World", 64)
fmt.Println(code) // "ISCC:EAA..."
```

Replace with the pure Go API matching the howto guide:

```go
import iscc "github.com/iscc/iscc-lib/packages/go"

result, _ := iscc.GenTextCodeV0("Hello World", 64)
fmt.Println(result.Iscc) // "ISCC:EAA..."
```

Key differences: no `context.Background()`, no `NewRuntime`, no `defer rt.Close`, function is
package-level not a method, returns a struct with `.Iscc` field (not a plain string).

Keep the import line unchanged — it's correct. Keep the install command (`go get ...`) unchanged.
Preserve the indentation style (4-space indent under the tab).

## Verification

- `grep -c "NewRuntime" docs/index.md` returns 0
- `grep -c "context.Background" docs/index.md` returns 0
- `grep "GenTextCodeV0" docs/index.md` shows the pure Go API pattern (no `ctx` parameter)
- `grep "result.Iscc" docs/index.md` returns a match
- `uv run zensical build` exits 0
- `mise run format` produces no changes (run format before committing)

## Done When

All verification criteria pass — the landing page Go tab uses the pure Go API with no references to
the stale WASM-bridge pattern.
