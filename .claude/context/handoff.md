## 2026-02-24 — Create per-crate READMEs for iscc-lib, iscc-py, and iscc-napi

**Done:** Created three registry-facing README.md files for the primary publishable crates
(crates.io, PyPI, npm) with all required sections. Updated `crates/iscc-lib/Cargo.toml` to point
`readme` to the local `README.md` and added `readme = "README.md"` to
`crates/iscc-py/pyproject.toml`.

**Files changed:**

- `crates/iscc-lib/README.md`: Created with Rust-specific content — badges (CI, crate, license),
    experimental notice, ISCC explanation, `cargo add` installation, Rust quick start example, API
    overview table with all 9 gen functions plus utilities/codec/streaming
- `crates/iscc-py/README.md`: Created with Python-specific content — badges (CI, PyPI, license),
    experimental notice, `pip install` installation, Python quick start with dict-style access,
    notes on BinaryIO support for streaming functions
- `crates/iscc-napi/README.md`: Created with Node.js-specific content — badges (CI, npm, license),
    package name `@iscc/lib`, `npm install` installation, quick start showing string return type
    (napi bindings return strings directly, not objects), note about Buffer inputs
- `crates/iscc-lib/Cargo.toml`: Changed `readme` from `"../../README.md"` to `"README.md"`
- `crates/iscc-py/pyproject.toml`: Added `readme = "README.md"` under `[project]`

**Verification:** All checks pass:

- All three README files exist (verified with `test -f`)
- Each README has 6 H2 sections (What is ISCC, Installation, Quick Start, API Overview, Links,
    License)
- `Cargo.toml` and `pyproject.toml` both contain `readme = "README.md"`
- `mise run check` passes all 14 pre-commit hooks
- `cargo clippy --workspace --all-targets -- -D warnings` is clean

**Next:** Create per-crate READMEs for batch 2 (iscc-wasm, iscc-ffi, iscc-jni), address performance
issues from issues.md (DataHasher buffer allocation, codec header parsing), or start Go bindings.

**Notes:** The napi Node.js quick start correctly reflects that `gen_meta_code_v0` returns a string
directly (not an object), matching the actual napi binding implementation which uses
`.map(|r| r.iscc)`. All three READMEs use consistent section ordering and formatting. No
`package.json` change was needed for npm (auto-detects README.md). Each README is concise (65-80
lines) and tailored to its registry audience.
