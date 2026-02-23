## 2026-02-23 — Add architecture overview documentation page

**Done:** Created `docs/architecture.md` with all 5 required sections (hub-and-spoke crate model,
workspace layout, internal module structure, streaming pattern, conformance testing). Added
`architecture.md` to the `zensical.toml` nav between `index.md` and `api.md`.

**Files changed:**

- `docs/architecture.md`: New architecture overview page with Mermaid diagram, workspace tree,
    module table, streaming pattern explanation, and conformance testing matrix
- `zensical.toml`: Added `architecture.md` to nav list between `index.md` and `api.md`

**Verification:** All 143 Rust tests pass, clippy clean (no warnings), cargo fmt clean. Content
accuracy verified against actual `lib.rs` module structure (7 modules: codec as Tier 2, cdc/dct/
minhash/simhash/utils/wtahash as pub(crate) internal). Crate names match actual workspace members
(iscc-lib, iscc-py, iscc-napi, iscc-wasm, iscc-ffi). Mermaid diagram uses valid `graph TD` syntax
with proper node/edge definitions.

**Next:** The documentation site now has its three core pages (index, architecture, API reference).
Remaining documentation gaps: benchmark results page showing Rust vs Python performance data. Beyond
docs, OIDC trusted publishing workflows for crates.io, PyPI, and npm would move the project closer
to release-readiness.

**Notes:** The architecture page documents the Tier 1 API as pub functions in the crate root
(lib.rs) rather than in a separate `api` module — this matches the actual implementation, which
differs slightly from the notes/04 design document that references `iscc::api`. The admonition
(`!!! note`) uses Material for MkDocs syntax which is already configured in zensical.toml. Used
Unicode `—` for em dashes per the learnings about smartsymbols.
