# Next Work Package

## Step: Add Go section and fix body text in root README

## Goal

Complete the root README by adding Go installation/quick-start sections, a Go Reference badge, and
fixing the "What is iscc-lib" body text to mention Java and Go — closing the last README gaps
identified in the state assessment.

## Scope

- **Create**: (none)
- **Modify**: `README.md`
- **Reference**: `packages/go/README.md` (for Go install command and quick-start code example),
    `packages/go/iscc.go` (for API shape)

## Not In Scope

- Adding a Maven Central badge (Java is not yet published to Maven Central; a dead badge is worse
    than no badge)
- Adding `docs/howto/go.md` or `docs/howto/java.md` how-to guides (separate documentation step)
- Adding Java/Go code to the tabbed code blocks on the documentation site
- Changing any code, tests, or CI configuration
- Updating per-crate READMEs (they are already complete)
- Fixing the Go `packages/go/README.md` "Additional utilities" note (it's out of date but belongs in
    a separate step)

## Implementation Notes

Make these changes to `README.md`:

1. **Add Go Reference badge** — after the npm badge line (line 7), add a Go Reference badge:
    `[![Go Reference](https://pkg.go.dev/badge/github.com/iscc/iscc-lib/packages/go.svg)](https://pkg.go.dev/github.com/iscc/iscc-lib/packages/go)`

2. **Fix "What is iscc-lib" body text** — line 46-47 currently says "Python, Node.js, WebAssembly,
    and C". Change to "Python, Java, Go, Node.js, WebAssembly, and C" to match the actual binding
    set.

3. **Add Go installation section** — between the Java and WASM sections (after the Java native
    library note, before the `### WASM` heading). Content: `### Go` heading followed by a bash code
    block containing `go get github.com/iscc/iscc-lib/packages/go`.

4. **Add Go quick start section** — between the Java and WASM quick-start sections. Use the compact
    quick-start code from `packages/go/README.md` (Runtime creation, GenMetaCodeV0 call, defer
    Close). The example should be complete and runnable (include `package main`, imports,
    `func main()`).

5. **Update Key Features bullet** — line 23 says "Python, Java, Node.js, WASM, and C FFI". Add Go:
    "Python, Java, Go, Node.js, WASM, and C FFI".

## Verification

- `grep 'pkg.go.dev' README.md` exits 0 (Go Reference badge present)
- `grep 'go get github.com/iscc/iscc-lib/packages/go' README.md` exits 0 (Go install command)
- `grep 'iscc.NewRuntime' README.md` exits 0 (Go quick-start example)
- `grep 'Python, Java, Go, Node.js, WebAssembly, and C' README.md` exits 0 ("What is iscc-lib" text
    fixed)
- `grep 'Python, Java, Go, Node.js, WASM, and C FFI' README.md` exits 0 (Key Features updated)
- `grep -c '### Go' README.md` outputs `2` (one in Installation, one in Quick Start)

## Done When

All six verification grep commands pass, confirming the Go sections are present, the body text
mentions all six binding ecosystems, and no other files were modified.
