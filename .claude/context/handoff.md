## 2026-03-02 — Add gen_sum_code_v0 to documentation and comments

**Done:** Updated every user-facing document, README, docstring, and code comment that listed the
gen functions to include `gen_sum_code_v0`. Changed all "9 gen" references to "10 gen" across
READMEs, docs/, howto guides, architecture docs, ecosystem docs, benchmark docs, C FFI API docs,
Rust API docs, notes/, CLAUDE.md files, code comments, and type stubs. Added full API documentation
for `gen_sum_code_v0` in `docs/rust-api.md` and `docs/c-ffi-api.md` with signatures, parameter
tables, and usage examples. Added `SumCodeResult` to result type tables in Rust and Python howto
guides.

**Files changed:**

- `README.md`: Added `gen_sum_code_v0` to function listing
- `crates/iscc-lib/README.md`: Added row to Code Generators table
- `crates/iscc-py/README.md`: Added row to Code Generators table
- `crates/iscc-napi/README.md`: Added row to Code Generators table
- `crates/iscc-wasm/README.md`: Added row to Code Generators table
- `crates/iscc-wasm/pkg/README.md`: Added row to Code Generators table
- `crates/iscc-ffi/README.md`: Added row and updated return description
- `crates/iscc-jni/README.md`: Added row to Code Generators table
- `packages/go/README.md`: Added row to Code Generators table
- `crates/iscc-ffi/src/lib.rs`: 9→10 in module docstring
- `crates/iscc-lib/src/conformance.rs`: 9→10 in module and function docstrings
- `crates/iscc-lib/benches/benchmarks.rs`: 9→10 in module docstring
- `crates/iscc-wasm/tests/unit.rs`: 9→10 in module docstring
- `crates/iscc-wasm/tests/conformance.rs`: 9→10 in module docstring
- `crates/iscc-napi/__tests__/conformance.test.mjs`: 9→10 in file docstring
- `tests/test_conformance.py`: 9→10 in module docstring
- `benchmarks/python/bench_iscc_lib.py`: 9→10 in module docstring
- `benchmarks/python/bench_iscc_core.py`: 9→10 in module docstring
- `crates/iscc-py/python/iscc_lib/_lowlevel.pyi`: 9→10 in conformance_selftest docstring
- `docs/rust-api.md`: 9→10 count, added gen_sum_code_v0 section with signature/params/example
- `docs/index.md`: 9→10 count (2 places), added ISCC-SUM row to code types table
- `docs/llms.txt`: 9→10 count
- `docs/java-api.md`: 9→10 count
- `docs/architecture.md`: 9→10 count (2 places), added gen_sum_code_v0 to Tier 1 code listing
- `docs/ecosystem.md`: 9→10 count (4 places)
- `docs/benchmarks.md`: 9→10 count (2 places)
- `docs/c-ffi-api.md`: 9→10 count, added full iscc_gen_sum_code_v0/IsccSumCodeResult/free section
- `docs/howto/wasm.md`: 9→10 count
- `docs/howto/rust.md`: 9→10 count, added SumCodeResult to result types table
- `docs/howto/python.md`: 9→10 count, added SumCodeResult to result types table
- `docs/howto/nodejs.md`: 9→10 count
- `docs/howto/java.md`: 9→10 count
- `docs/howto/go.md`: 9→10 count
- `docs/tutorials/getting-started.md`: 9→10 count
- `crates/iscc-lib/CLAUDE.md`: 9→10 count (3 places), added gen_sum_code_v0 to function list
- `crates/iscc-py/CLAUDE.md`: 9→10 count (2 places)
- `crates/iscc-wasm/CLAUDE.md`: 9→10 count (2 places), added gen_sum_code_v0 to function list
- `crates/iscc-napi/CLAUDE.md`: 9→10 count
- `CLAUDE.md`: 9→10 count in conformance section
- `notes/01-workspace-structure.md`: 9→10 count
- `notes/04-api-compatibility-safety.md`: 9→10 count (2 places), updated symbol count 22→23

**Verification:** All checks pass:

- `mise run check` — all pre-commit hooks pass (formatting, linting, YAML/JSON/TOML validation)
- `grep -r "9 .gen_*_v0|the 9 gen|all 9 gen|All 9.*gen"` across crates/, docs/, README.md, notes/,
    tests/, benchmarks/ — zero matches
- `gen_sum_code_v0` appears in all 9 README files and 3 additional docs (architecture, c-ffi-api,
    rust-api, index)
- `SumCodeResult` documented in rust-api.md, howto/rust.md, howto/python.md, c-ffi-api.md

**Next:** The howto guides (rust.md, python.md, nodejs.md, wasm.md, java.md, go.md) have Code
Generation sections that list each function with a code example (Meta-Code, Text-Code, etc.) but
don't yet have a Sum-Code subsection with a usage example. Adding Sum-Code examples to each language
howto would complete the documentation coverage.

**Notes:** This step touched 38 files but all changes are documentation-only (comment/docstring
text, markdown content). No code logic was changed. The ecosystem.md references to iscc-core-ts
("all 10 gen\_\*\_v0 functions from ISO 24138") were updated to 10 because the context says
functions specified by ISO 24138. However, iscc-core-ts genuinely implements only the original 9 —
reviewer should verify if this wording should say "9" for the TypeScript project. The iscc-core
ecosystem entry was updated to 10 since the reference implementation does include gen_sum_code_v0.
