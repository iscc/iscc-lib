# Next Work Package

## Step: Expand docs landing page to all 6 languages

## Goal

Add Node.js, Java, Go, and WASM tabs to the Quick Start section in `docs/index.md` and add Java and
Go rows to the Available Bindings table. This closes the target gap "All code examples use tabbed
multi-language format" before the v0.0.1 release.

## Scope

- **Create**: (none)
- **Modify**: `docs/index.md`
- **Reference**: `docs/howto/nodejs.md`, `docs/howto/java.md`, `docs/howto/go.md`,
    `docs/howto/wasm.md` (for idiomatic code patterns in each language)

## Not In Scope

- Fixing the existing Rust and Python Quick Start examples (they have minor inaccuracies — Rust
    shows `println!("{result}")` as "JSON string" when it's a struct; Python uses `json.loads` when
    the binding returns a dict-like object directly). These should be fixed but in a separate step
- Adding the `Key Features` bullet to mention Java and Go (currently says "Rust, Python, Node.js,
    WebAssembly, or C") — cosmetic, defer
- Fixing the WASM howto guide package name (`@iscc/iscc-wasm` → `@iscc/wasm`) — pre-existing issue
    in a different file
- Restructuring the page layout or adding new sections beyond Quick Start tabs and binding table
    rows

## Implementation Notes

Add 4 new tabs to the `=== "Language"` tabbed block under Quick Start. Follow
[pymdownx.tabbed](https://facelessuser.github.io/pymdown-extensions/extensions/tabbed/) syntax
(already used by the existing Rust/Python tabs). Each tab shows install command + minimal code
example.

**Tab order**: Rust, Python, Node.js, Java, Go, WASM (matches target.md ordering and Available
Bindings table).

**Code patterns per language** (derived from howto guides and agent memory):

- **Node.js** (`=== "Node.js"`): `npm install @iscc/lib`, then
    `import { gen_text_code_v0 } from "@iscc/lib"` — returns a string directly (not structured
    object)
- **Java** (`=== "Java"`): Maven dependency XML block or `System.loadLibrary` note, then
    `IsccLib.genTextCodeV0("Hello World", 64)` — returns a String. Note that Maven Central is not
    yet available (build from source). Use `import io.iscc.iscc_lib.IsccLib;`
- **Go** (`=== "Go"`): `go get github.com/iscc/iscc-lib/packages/go`, then show Runtime creation +
    `rt.GenTextCodeV0(ctx, "Hello World", 64)` — returns `(string, error)`. Go requires more
    boilerplate (runtime setup) so keep the example minimal but correct
- **WASM** (`=== "WASM"`): `npm install @iscc/wasm`, then
    `import init, { gen_text_code_v0 } from "@iscc/wasm"` + `await init()` — returns a string. Note
    the async init requirement

**Available Bindings table**: Add rows for Java (Maven Central, `io.iscc:iscc-lib`, note not yet
published) and Go (Go module, `go get github.com/iscc/iscc-lib/packages/go`).

**Consistency**: All 6 tabs should demonstrate the same function (`gen_text_code_v0` with "Hello
World") to make cross-language comparison easy.

## Verification

- `uv run zensical build` exits 0 (docs site builds without errors)
- `grep -c '=== "' docs/index.md` returns 6 (one tab per language)
- `grep 'Node.js' docs/index.md` matches (Node.js tab present)
- `grep 'Java' docs/index.md` matches (Java tab and binding table row present)
- `grep 'Go' docs/index.md` matches (Go tab and binding table row present)
- `grep 'WASM' docs/index.md` matches (WASM tab present)
- `grep 'go get' docs/index.md` matches (Go install command in table or tab)
- Available Bindings table has 7 rows (Rust, Python, Node.js, WASM, C/C++, Java, Go)
- `mise run check` passes (formatting, linting)

## Done When

All verification criteria pass — docs/index.md has 6 language tabs in Quick Start and 7 binding rows
in the table, and the docs site builds cleanly.
