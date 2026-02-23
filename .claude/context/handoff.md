## 2026-02-23 — Add Rust API documentation page

**Done:** Created `docs/rust-api.md` with documentation for all 9 `gen_*_v0` functions (signatures,
parameter tables, descriptions, usage examples) plus the Tier 2 `codec` module types (`IsccError`,
`MainType`, `SubType`) and error handling section. Updated `zensical.toml` nav to include the page
between architecture and Python API.

**Files changed:**

- `docs/rust-api.md`: Hand-written Rust API reference page covering all 9 public functions with
    correct signatures, parameter tables, return value descriptions, and code examples; plus codec
    types documentation and error handling guide
- `zensical.toml`: Added `"rust-api.md"` to nav array after `"architecture.md"` and before
    `"api.md"`

**Verification:** `uv run zensical build` succeeds — `site/rust-api/index.html` exists. All 9
function signatures verified against actual `lib.rs` source. `cargo fmt --check` clean,
`cargo clippy -- -D warnings` clean. No broken links (build produced no warnings).

**Next:** The documentation site now covers Rust API, Python API, and architecture — completing the
target's documentation requirement. Remaining gaps from state.md: (1) GitHub Pages enablement (admin
task, not code), (2) benchmark results page (optional content), (3) npm publishing workflow
(requires `@iscc` org setup on npm). The most impactful remaining code work would be updating
state.md to reflect completion of the Rust API docs and release workflow, then assessing whether the
target state has been reached.

**Notes:** The page is hand-written rather than auto-generated because zensical/mkdocstrings only
supports Python. Function signatures were cross-checked against lib.rs source to ensure accuracy.
The style matches the Python API page (concise, practical, parameter tables). Codec types are
documented as Tier 2 with a note about FFI unavailability.
