# Next Work Package

## Step: Add gen_sum_code_v0 to all documentation and READMEs

## Goal

Update every user-facing reference that lists the gen\_\*\_v0 functions to include `gen_sum_code_v0`
(the 10th gen function). This closes the documentation gap identified after completing issue #15
across all 7 bindings.

## Scope

- **Create**: (none)
- **Modify**:
    - `README.md` — add `gen_sum_code_v0` to the Implementors Guide list (line ~212)
    - `crates/iscc-lib/README.md` — add row to Code Generators table
    - `crates/iscc-py/README.md` — add row to Code Generators table
    - `crates/iscc-napi/README.md` — add row to Code Generators table
    - `crates/iscc-wasm/README.md` — add row to Code Generators table
    - `crates/iscc-wasm/pkg/README.md` — keep in sync with `crates/iscc-wasm/README.md` (identical)
    - `crates/iscc-ffi/README.md` — add row to Code Generators table
    - `crates/iscc-ffi/src/lib.rs` — fix module docstring: "9 `gen_*_v0` functions" → "10"
    - `crates/iscc-jni/README.md` — add row to Code Generators table
    - `packages/go/README.md` — add row to Code Generators table
    - `crates/iscc-lib/src/conformance.rs` — update comment "all 9" → "all 10"
    - `crates/iscc-lib/benches/benchmarks.rs` — update comment "all 9" → "all 10"
    - `crates/iscc-wasm/tests/unit.rs` — update comment "the 9" → "the 10"
    - `crates/iscc-wasm/tests/conformance.rs` — update comment "all 9" → "all 10"
    - `docs/c-ffi-api.md` — add `iscc_gen_sum_code_v0` section after `iscc_gen_iscc_code_v0`; update
        "All 9" → "All 10"
    - `docs/benchmarks.md` — update "the 9" → "the 10" in description text
    - Any other `docs/*.md` files that list the 9 gen functions (check `docs/api.md`,
        `docs/rust-api.md`, `docs/architecture.md`, `docs/index.md`, `docs/ecosystem.md`,
        `docs/tutorials/getting-started.md`, `docs/howto/*.md`)
- **Reference**:
    - `crates/iscc-ffi/src/lib.rs` — for `iscc_gen_sum_code_v0` C API signature (already exists)
    - `crates/iscc-lib/src/api.rs` — for `gen_sum_code_v0` Rust signature
    - `crates/iscc-py/src/lib.rs` — for Python binding signature
    - `crates/iscc-napi/src/lib.rs` — for Node.js binding signature

## Not In Scope

- Adding `gen_sum_code_v0` to benchmark suite (criterion or pytest-benchmark) — separate performance
    step
- Updating `docs/benchmarks.md` with benchmark results for `gen_sum_code_v0` — no benchmarks exist
    yet for this function
- Modifying any functional code — this is a documentation-only step
- Adding new tests — all tests already pass
- Updating version numbers or releasing a new version
- Addressing issue #16 (feature flags) — separate low-priority step
- Rewriting any existing documentation sections — only add missing function references

## Implementation Notes

**Pattern for API tables:** Every per-crate README and the Go README has a "Code Generators" table
with 9 rows. Add `gen_sum_code_v0` as the 10th row, placed after `gen_iscc_code_v0` (it builds on
Data-Code + Instance-Code + ISCC-CODE). Use binding-appropriate naming:

| Binding | Function name          | Description                           |
| ------- | ---------------------- | ------------------------------------- |
| Rust    | `gen_sum_code_v0`      | Generate an ISCC-SUM from a file path |
| Python  | `gen_sum_code_v0`      | Generate an ISCC-SUM from a file path |
| Node.js | `gen_sum_code_v0`      | Generate an ISCC-SUM from a file path |
| WASM    | `gen_sum_code_v0`      | Generate an ISCC-SUM from raw bytes   |
| C FFI   | `iscc_gen_sum_code_v0` | Generate an ISCC-SUM from a file path |
| Java    | `genSumCodeV0`         | Generate an ISCC-SUM from a file path |
| Go      | `GenSumCodeV0`         | Generate an ISCC-SUM from a file path |

**Root README Implementors Guide** (line ~203-213): Add `gen_sum_code_v0` to the code block list.

**FFI module docstring** (`crates/iscc-ffi/src/lib.rs` line 3): Change "9" to "10".

**Code comments** in `conformance.rs`, `benchmarks.rs`, and WASM test files: simple "9" → "10"
string replacements.

**docs/c-ffi-api.md**: Add a new `### iscc_gen_sum_code_v0` section after `iscc_gen_iscc_code_v0`.
Read the actual C FFI function signature from `crates/iscc-ffi/src/lib.rs` to document it
accurately. The function returns an `IsccSumCodeResult` struct (not a plain string like other gen
functions) — document the struct fields and the `iscc_free_sum_code_result` cleanup function.

**WASM pkg/README.md**: This file is an exact copy of `crates/iscc-wasm/README.md`. After editing
the source README, copy the same changes to `pkg/README.md` to keep them in sync.

**Other docs pages**: Grep each file in the docs/ reference list to find where gen functions are
listed. Some pages may list functions in code examples, API tables, or prose — add gen_sum_code_v0
where it fits naturally. Don't add it to places that only show one or two example functions.

## Verification

- `grep -rl "gen_sum_code_v0\|GenSumCodeV0\|genSumCodeV0\|iscc_gen_sum_code_v0" README.md crates/*/README.md packages/go/README.md docs/`
    returns hits in all expected files
- `grep -r "9 .gen_\*_v0\|the 9 gen\|all 9 gen\|All 9.*gen" crates/ docs/ README.md` returns zero
    hits (all "9" references updated to "10")
- `cargo clippy -p iscc-ffi -- -D warnings` passes (docstring change doesn't break code)
- `diff crates/iscc-wasm/README.md crates/iscc-wasm/pkg/README.md` returns no output (files in sync)
- `mise run check` passes (formatting hooks clean)

## Done When

All verification criteria pass — every user-facing listing of gen\_\*\_v0 functions includes
`gen_sum_code_v0`, all "9 gen" comments/text are updated to "10", and no code behavior is changed.
