## 2026-02-23 — Add Node.js conformance tests for the napi crate

**Done:** Created JavaScript conformance tests for all 9 `gen_*_v0` napi-rs bindings using Node.js
built-in `node:test` runner. All 46 conformance vectors pass, matching the Python test count. Added
`test` script to `package.json`.

**Files changed:**

- `crates/iscc-napi/__tests__/conformance.test.mjs`: Conformance test file covering all 9 gen
    functions (16+5+3+5+3+2+4+3+5 = 46 vectors). Mirrors Python test structure with `describe`/`it`
    blocks per function. Includes `decodeStream` for hex-encoded buffer inputs and
    `jsonSortedStringify` for meta dict conversion.
- `crates/iscc-napi/package.json`: Added `"test": "node --test __tests__/conformance.test.mjs"`
    script.

**Verification:**

- `npm test` in `crates/iscc-napi/`: 46 passed, 0 failed (9 suites, 46 tests)
- `cargo test -p iscc-lib`: 143 passed, 0 failed (no regression)
- `cargo clippy --workspace --all-targets -- -D warnings`: clean
- `cargo fmt --all --check`: clean

**Next:** Add a Node.js CI job to `.github/workflows/ci.yml` that builds the native addon and runs
`npm test`. This would complete the napi crate integration. After that, WASM bindings
(`crates/iscc-wasm/`) or Phase 1 pytest-benchmark (iscc_lib vs iscc-core comparison) would be the
next deliverables.

**Notes:** The napi build generates `iscc-lib.linux-x64-gnu.node`, `index.js`, and `index.d.ts` in
the crate root — these are gitignored build artifacts. The `node_modules/` directory from
`npm install` is also a build artifact. The test file uses zero extra dependencies (only Node.js
built-in `node:test` and `node:assert`). The `@napi-rs/cli` prints a deprecation warning about
`napi.name` vs `napi.binaryName` — cosmetic only, doesn't affect functionality.
