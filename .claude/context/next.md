# Next Work Package

## Step: Update architecture and development docs for JNI and Go bindings

## Goal

Update `docs/architecture.md` and `docs/development.md` to include the JNI (Java) and Go binding
crates, which were added in iterations 5-7 but never reflected in these documentation pages. Both
pages currently show only 4 binding crates when there are actually 6 (+ Go via WASM/wazero).

## Scope

- **Create**: (none)
- **Modify**: `docs/architecture.md`, `docs/development.md`
- **Reference**: `crates/iscc-jni/src/lib.rs`, `packages/go/iscc.go`, `docs/howto/java.md`,
    `docs/howto/go.md`, `zensical.toml` (for current nav structure)

## Not In Scope

- Adding new Reference nav pages (Java API reference, C FFI reference) — those are separate steps
- Changing the getting-started tutorial or adding tabbed multi-language examples
- Modifying any Rust, Python, or other source code
- Updating the README or per-crate READMEs

## Implementation Notes

### architecture.md changes

1. **Mermaid diagram**: Add `JNI` and `GO` nodes. JNI depends on CORE directly. Go depends on CORE
    indirectly via WASM (Go uses wazero to run the WASM binary compiled from `iscc-ffi`). The
    diagram should show this relationship: `GO --> WASM_BIN --> CORE` where WASM_BIN represents the
    compiled WASI binary from `iscc-ffi`. Or more accurately, Go depends on the FFI crate's WASM
    output. Keep the diagram simple — something like:

    ```
    JNI["iscc-jni<br/><small>Java · JNI</small>"] --> CORE
    GO["Go<br/><small>wazero · WASM</small>"] -.-> FFI
    ```

    The dotted arrow for Go (via WASM) distinguishes it from the direct Rust dependency of other
    binding crates.

2. **Workspace layout tree**: Add `iscc-jni/` under `crates/` and `packages/go/` as a top-level
    directory alongside `crates/`. Include key files (`Cargo.toml`, `src/lib.rs`, `java/`
    subdirectory with `pom.xml`, `IsccLib.java`, etc. for JNI; `iscc.go`, `iscc_test.go`, `go.mod`
    for Go).

3. **Crate summary table**: Add rows for `iscc-jni` (produces JNI shared library, build tool cargo,
    published to Maven Central) and `packages/go` (produces Go module, build tool cargo+wasm-pack
    for the embedded binary, published to pkg.go.dev).

4. **Streaming pattern / Per-Binding Adaptation table**: Add rows for Java (JNI sync API,
    `DataHasher`/`InstanceHasher` via JNI) and Go (sync API via wazero WASM calls,
    `UpdateFrom(ctx, io.Reader)` support for streaming).

5. **Conformance test matrix table**: Add rows for Java (`mvn test`, `data.json` via relative path
    from test resources) and Go (`go test`, embedded test vectors).

### development.md changes

1. **Project structure tree**: Add `iscc-jni/` under `crates/` (with `src/lib.rs` and `java/`
    subdirectory) and `packages/go/` as a peer of `crates/`.

2. **Crate summary table**: Add rows for `iscc-jni` and `packages/go` matching the pattern of
    existing rows.

3. **Included Tools table**: Add JDK 17 and Go entries if they're provided by the devcontainer
    (check `.devcontainer/Dockerfile` if unsure).

### Style guidance

- Match the existing style and formatting of both pages exactly
- Use the same icon/description YAML front matter format (already present)
- Keep descriptions concise and consistent with adjacent entries
- For Go, emphasize the "pure Go, no cgo" aspect that's central to the design (wazero)
- For Java, mention JNI + bundled native libraries pattern

## Verification

- `uv run zensical build` exits 0 (site builds with updated pages)
- `grep -q 'iscc-jni' docs/architecture.md` (JNI mentioned in architecture)
- `grep -q 'wazero' docs/architecture.md` (Go/wazero mentioned in architecture)
- `grep -q 'iscc-jni' docs/development.md` (JNI mentioned in development)
- `grep -q 'packages/go' docs/development.md` (Go mentioned in development)
- `grep -c 'iscc-jni\|packages/go\|wazero\|Java\|Go' docs/architecture.md` returns at least 5
    (multiple references across diagram, layout, tables)
- `mise run format` produces no unstaged changes (formatting clean)

## Done When

All verification criteria pass — both docs pages reflect the full set of 6 binding crates (Python,
Node.js, WASM, C FFI, JNI, Go) in their diagrams, layout trees, and summary tables.
