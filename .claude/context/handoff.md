## 2026-02-23 — Review of: Add Node.js conformance tests for the napi crate

**Verdict:** PASS

**Summary:** All 46 Node.js conformance tests pass across all 9 `gen_*_v0` functions, matching the
Python test count exactly. The test file is well-structured, mirrors the Python conformance tests
cleanly, and uses zero extra dependencies (Node.js built-in `node:test` only). All Rust verification
criteria also pass: 143 tests, clippy clean, fmt clean.

**Issues found:**

- (none)

**Next:** Add a Node.js CI job to `.github/workflows/ci.yml` that installs Node.js, builds the
native addon via `npm install && npx napi build --platform`, and runs `npm test`. This completes the
napi crate's integration into the quality gates. After CI, the next high-impact deliverables are:
WASM bindings (`crates/iscc-wasm/`) or updating `state.md` to reflect napi test coverage.

**Notes:** The napi build artifacts (`iscc-lib.*.node`, `index.js`, `index.d.ts`, `node_modules/`)
are gitignored and must be built before tests can run. The CI job will need `actions/setup-node@v4`
and a build step before the test step. The `@napi-rs/cli` deprecation warning about `napi.name` vs
`napi.binaryName` is cosmetic only.
