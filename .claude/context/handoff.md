## 2026-02-24 — Review of: Create per-crate READMEs for iscc-lib, iscc-py, and iscc-napi

**Verdict:** PASS

**Summary:** Three registry-facing README.md files created for the primary publishable crates
(crates.io, PyPI, npm). Each README has all required sections with registry-specific content,
correct API examples, and proper manifest integration. Clean documentation-only change with no code
modifications.

**Verification:**

- [x] `test -f crates/iscc-lib/README.md` exits 0
- [x] `test -f crates/iscc-py/README.md` exits 0
- [x] `test -f crates/iscc-napi/README.md` exits 0
- [x] `grep -q 'readme = "README.md"' crates/iscc-lib/Cargo.toml` exits 0
- [x] `grep -q 'readme = "README.md"' crates/iscc-py/pyproject.toml` exits 0
- [x] Each README has 6 H2 sections (all return 6)
- [x] `cargo clippy --workspace --all-targets -- -D warnings` clean
- [x] `mise run check` passes all 14 pre-commit hooks

**Issues found:**

- (none)

**Next:** Create per-crate READMEs for batch 2 (iscc-wasm, iscc-ffi, iscc-jni). These are the
secondary crates — iscc-wasm publishes to npm as `@iscc/wasm`, iscc-ffi is not published separately
(no registry metadata needed), iscc-jni targets Maven Central. Follow the same template and section
structure established in batch 1.

**Notes:** All three READMEs are 70-75 lines, slightly under the 80-120 range suggested in next.md,
but they're complete and well-structured for registry pages. The Node.js quick start correctly uses
CommonJS `require()` and shows the string return pattern. Python README includes the BinaryIO note
for streaming functions. Rust README mentions the Tier 2 codec module appropriately.
