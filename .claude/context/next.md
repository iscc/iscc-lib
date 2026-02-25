# Next Work Package

## Step: Create Go package README

## Goal

Create `packages/go/README.md` for the Go module proxy audience, matching the structure and quality
of existing per-crate READMEs (iscc-jni, iscc-napi, etc.). This is the first thing Go developers see
when they find the package via `pkg.go.dev` or `go get`.

## Scope

- **Create**: `packages/go/README.md`
- **Modify**: (none)
- **Reference**: `crates/iscc-jni/README.md`, `crates/iscc-napi/README.md`,
    `crates/iscc-lib/README.md` (for structure/tone), `packages/go/iscc.go` (for current API
    surface), `packages/go/go.mod` (for module path)

## Not In Scope

- Adding Go sections to the root `README.md` (separate step)
- Creating `docs/howto/go.md` how-to guide (separate step)
- Implementing the 12 remaining Go wrappers (text utilities, algorithm primitives, streaming)
- Adding Go badge to root README
- Modifying any Go source code

## Implementation Notes

Follow the established per-crate README pattern used by `crates/iscc-jni/README.md` and
`crates/iscc-napi/README.md`. Structure:

1. **Title**: `iscc-lib (Go)` — consistent with `iscc-lib (Java)` pattern
2. **Badges**: CI badge + License badge (no registry version badge yet — Go module proxy doesn't
    have a standard badge URL like npm/PyPI; skip for now)
3. **Experimental notice**: same blockquote used across all READMEs
4. **Tagline**: Go bindings via WASM/wazero — emphasize pure Go (no cgo), zero external
    dependencies for the consumer
5. **What is ISCC**: reuse the shared paragraph from other READMEs verbatim
6. **Installation**: `go get github.com/iscc/iscc-lib/packages/go` — note that the WASM binary is
    embedded (`//go:embed`), no manual setup needed
7. **Quick Start**: show `NewRuntime`, `GenMetaCodeV0`, `Close` — use `context.Background()`. Show
    the `defer rt.Close(ctx)` pattern. Example should compile
8. **API Overview**: list all 9 `Gen*CodeV0` functions in a table (same format as JNI README), then
    list currently available utilities (`ConformanceSelftest`, `TextClean`). Note that additional
    utilities (text processing, algorithm primitives, streaming hashers) are planned
9. **Architecture note**: brief mention that the package uses wazero (pure-Go WASM runtime) to
    execute Rust-compiled WASM — no cgo, no shared libraries, cross-compilation just works
10. **Links**: Documentation, Repository, ISCC Specification, ISCC Foundation — same as other
    READMEs
11. **License**: Apache-2.0

Key details to get right:

- Module path is `github.com/iscc/iscc-lib/packages/go` (from `go.mod`)
- Package name is `iscc` (from `package iscc` in `iscc.go`)
- Import as `iscc "github.com/iscc/iscc-lib/packages/go"`
- All methods are on `*Runtime` (not free functions)
- `GenMetaCodeV0` takes `(ctx, name, description, meta, bits)` where description/meta are `*string`
    (optional)
- Functions return `(string, error)` — idiomatic Go error handling
- The `context.Context` parameter is required on all methods

## Verification

- `test -f packages/go/README.md` exits 0 (file exists)
- `grep -q 'go get github.com/iscc/iscc-lib/packages/go' packages/go/README.md` exits 0 (install
    instruction present)
- `grep -q 'GenMetaCodeV0' packages/go/README.md` exits 0 (quick start uses correct function)
- `grep -q 'Apache-2.0' packages/go/README.md` exits 0 (license present)
- `grep -q 'wazero' packages/go/README.md` exits 0 (architecture mention present)
- `grep -q 'What is ISCC' packages/go/README.md` exits 0 (shared section present)
- `mise run check` passes (pre-commit hooks clean — mdformat will validate markdown formatting)

## Done When

All verification commands exit 0 and `mise run check` passes clean.
