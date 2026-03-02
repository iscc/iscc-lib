# Next Work Package

## Step: Add gen_sum_code_v0 examples to all 6 howto guides

## Goal

Add a "Sum-Code" subsection with a minimal working code example to the Code Generation section of
each per-language howto guide. This is the last remaining documentation gap — all 6 guides currently
have 9 gen function subsections but none for `gen_sum_code_v0`.

## Scope

- **Create**: none
- **Modify**: `docs/howto/rust.md`, `docs/howto/python.md`, `docs/howto/nodejs.md`,
    `docs/howto/wasm.md`, `docs/howto/java.md`, `docs/howto/go.md`
- **Reference**:
    - `crates/iscc-wasm/tests/unit.rs` (WASM `gen_sum_code_v0` takes `&[u8]`, not file path)
    - `crates/iscc-jni/java/src/main/java/io/iscc/iscc_lib/IsccLib.java` (Java signature)
    - `packages/go/code_sum.go` (Go signature)
    - `crates/iscc-lib/src/api.rs` (Rust signature)
    - `crates/iscc-py/src/lib.rs` (Python binding signature)
    - `crates/iscc-napi/src/lib.rs` (Node.js binding signature)

## Not In Scope

- Adding `gen_sum_code_v0` benchmarks or benchmark documentation
- Updating result types tables in rust.md or python.md (SumCodeResult is already listed there)
- Modifying code, tests, or any non-howto documentation files
- Adding new sections beyond the Code Generation subsection (e.g., no new "Streaming" or "Advanced"
    subsections for Sum-Code)
- Updating the count of gen functions in howto guide introductions or other prose — if they say
    "nine" in running text, that's a separate cleanup task, not part of adding the example
    subsection
- Addressing issue #16 (feature flags) — separate low-priority step

## Implementation Notes

**Insertion point:** Add `### Sum-Code` subsection immediately AFTER the existing `### ISCC-CODE`
subsection in the Code Generation section of each file.

**Pattern to follow:** Each subsection has:

1. `### Sum-Code` heading
2. One-line description: "Generate a composite ISCC-CODE from a file in a single pass:" (adjust for
    WASM: "from raw bytes")
3. Fenced code block with language-appropriate example

**Key API differences by language:**

| Language | Function                                | Input                | Result type                                                  |
| -------- | --------------------------------------- | -------------------- | ------------------------------------------------------------ |
| Rust     | `gen_sum_code_v0(path, 64, false)`      | `&Path`              | `SumCodeResult { iscc, datahash, filesize }`                 |
| Python   | `gen_sum_code_v0("file.bin")`           | `str \| os.PathLike` | `SumCodeResult` with `.iscc`, `.datahash`, `.filesize`       |
| Node.js  | `gen_sum_code_v0("file.bin")`           | `string` path        | object with `iscc`, `datahash`, `filesize`                   |
| WASM     | `gen_sum_code_v0(data)`                 | `Uint8Array`         | object with `iscc`, `datahash`, `filesize`                   |
| Java     | `IsccLib.genSumCodeV0(path, 64, false)` | `String` path        | `SumCodeResult` with `.iscc()`, `.datahash()`, `.filesize()` |
| Go       | `iscc.GenSumCodeV0(path, 64, false)`    | `string` path        | `*SumCodeResult` with `.Iscc`, `.Datahash`, `.Filesize`      |

**WASM is the outlier:** No filesystem access in browser — takes `Uint8Array` (raw bytes) instead of
a file path. Include a brief comment explaining this difference. The WASM howto already uses
`TextEncoder` for data encoding in other examples — follow the same pattern.

**Example style:** Match the tone and coding style of adjacent subsections (ISCC-CODE,
Instance-Code). Use the same import style, variable naming, and output printing pattern as existing
examples in each file. Keep examples to 5-10 lines excluding imports.

**File-based examples:** For Rust/Python/Node.js/Java/Go, the example needs a file path. Use a
simple example like `std::fs::write` (Rust), `Path("example.bin").write_bytes(...)` (Python),
`fs.writeFileSync` (Node.js), `Files.write` (Java), `os.WriteFile` (Go) to create a sample file,
then call `gen_sum_code_v0` on it. Alternatively, reference a hypothetical file path with a comment
— match whatever pattern feels most natural for the language. Look at how existing examples in each
guide handle data setup.

## Verification

- `grep -c "### Sum-Code" docs/howto/rust.md docs/howto/python.md docs/howto/nodejs.md docs/howto/wasm.md docs/howto/java.md docs/howto/go.md`
    — all 6 files show count of 1
- `grep -l "gen_sum_code_v0\|GenSumCodeV0\|genSumCodeV0" docs/howto/*.md | wc -l` — returns 6
- `uv run zensical build 2>&1 | tail -1` — exits 0 (docs site builds successfully)
- `mise run format` exits 0 (formatting clean)

## Done When

All 6 howto guides have a `### Sum-Code` subsection with a working code example in the Code
Generation section, and the documentation site builds without errors.
