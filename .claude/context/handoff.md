## 2026-02-25 — Update architecture and development docs for JNI and Go bindings

**Done:** Updated `docs/architecture.md` and `docs/development.md` to include the JNI (Java) and Go
binding crates. Both pages now reflect all 6 binding targets (Python, Node.js, WASM, C FFI, JNI, Go)
in their Mermaid diagrams, workspace layout trees, crate summary tables, streaming adaptation
tables, and conformance test matrices.

**Files changed:**

- `docs/architecture.md`: Added JNI and Go nodes to Mermaid diagram (JNI → CORE direct, Go -.→ FFI
    dotted for WASM indirection), added `iscc-jni/` and `packages/go/` to workspace layout tree,
    added rows to crate summary table, per-binding adaptation table, and cross-language test matrix
- `docs/development.md`: Added JDK 17 + Maven and Go 1.24 to included tools table, added `iscc-jni/`
    and `packages/go/` to project structure tree, added rows to crate summary table

**Verification:** All criteria pass:

- `uv run zensical build` exits 0
- `grep -q 'iscc-jni' docs/architecture.md` — PASS
- `grep -q 'wazero' docs/architecture.md` — PASS
- `grep -q 'iscc-jni' docs/development.md` — PASS
- `grep -q 'packages/go' docs/development.md` — PASS
- `grep -c 'iscc-jni\|packages/go\|wazero\|Java\|Go' docs/architecture.md` returns 12 (≥5) — PASS
- `mise run format` produces no unstaged changes — PASS
- `mise run check` — all 14 hooks pass

**Next:** The CID loop is in maintenance mode. All core functionality, 7 language bindings, docs
site, CI/release workflows, ecosystem documentation, and architecture/development docs are complete.
Remaining work is human-gated: merge PR #3 from develop → main, configure publishing credentials,
and trigger a release.

**Notes:** Pure documentation change — no Rust, Python, or test code modified. The Go dotted arrow
(`-.->`) in the Mermaid diagram distinguishes it from direct Rust dependencies since Go uses wazero
to run the WASM binary compiled from `iscc-ffi`, not a direct crate dependency. mdformat
auto-reformatted table column widths (cosmetic-only).
