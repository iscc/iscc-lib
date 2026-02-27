# Next Work Package

## Step: Add Codec operations and Constants sections to binding howto guides

## Goal

Add "Codec operations" and "Constants" documentation sections to the Python, Node.js, Java, and WASM
howto guides, achieving cross-language documentation parity with `docs/howto/go.md` which is
currently the only guide covering all 30/30 Tier 1 symbols.

## Scope

- **Create**: (none)
- **Modify**: `docs/howto/python.md`, `docs/howto/nodejs.md`, `docs/howto/java.md`,
    `docs/howto/wasm.md`
- **Reference**: `docs/howto/go.md` (template for section structure and content),
    `crates/iscc-py/python/iscc_lib/__init__.py` (Python API surface and naming),
    `crates/iscc-napi/src/lib.rs` (Node.js API names),
    `packages/java/src/main/java/io/iscc/IsccLib.java` (Java API names),
    `crates/iscc-wasm/src/lib.rs` (WASM API names)

## Not In Scope

- Adding "Algorithm primitives" sections to Python, Node.js, or WASM guides (separate step)
- Adding Codec/Constants to `docs/howto/rust.md` (Rust developers use `cargo doc`)
- Modifying any source code — this is documentation-only
- Updating `zensical.toml` navigation (sections are within existing pages, no new pages)
- Restructuring existing sections in any guide

## Implementation Notes

Use `docs/howto/go.md` lines 365–437 as the structural template. Each guide gets two new sections
inserted **before** the "Conformance testing" section:

### "Codec operations" section

Cover these 6 functions with language-idiomatic code examples:

1. **`encode_component`** — construct an ISCC unit from raw header fields and digest
2. **`iscc_decode`** — parse an ISCC unit string back into header components and digest
3. **`iscc_decompose`** — split a composite ISCC-CODE into individual unit codes
4. **`encode_base64`** — encode bytes to base64
5. **`json_to_data_url`** — convert JSON string to data URL
6. **`soft_hash_video_v0`** — compute video similarity hash from frame signatures

Structure: Show encode + decode as a paired example (like Go guide), then decompose, then list
remaining 3 as bullet points with signatures.

### "Constants" section

Cover 4 algorithm constants with a single code block showing how to import/access them:

- `META_TRIM_NAME` = 128
- `META_TRIM_DESCRIPTION` = 4096
- `IO_READ_SIZE` = 4,194,304
- `TEXT_NGRAM_SIZE` = 13

### Language-specific details

**Python:** Functions use `snake_case`. `iscc_decode` returns `tuple[MT, ST, VS, int, bytes]` with
IntEnum-typed values. Constants are top-level module attributes. Also mention `core_opts`
SimpleNamespace for iscc-core API parity.

**Node.js:** Functions use `snake_case` (via `js_name`). `iscc_decode` returns an `IsccDecodeResult`
object. Constants are exported as `META_TRIM_NAME` etc. Use `const { ... } = require("@iscc/lib")`
import style.

**Java:** Methods use `camelCase` on `IsccLib` class: `encodeComponent`, `isccDecode` (returns
`IsccDecodeResult`), `isccDecompose`, `encodeBase64`, `jsonToDataUrl`, `softHashVideoV0`. Constants
are `public static final int` on `IsccLib`.

**WASM:** Functions use `snake_case`. `iscc_decode` returns an `IsccDecodeResult` object. Constants
are getter functions: `meta_trim_name()`, etc.

### Description update

Update each guide's front matter `description:` and opening paragraph to mention codec operations
and constants coverage.

## Verification

- `grep -c 'encode_component\|encodeComponent\|EncodeComponent' docs/howto/python.md` returns ≥ 1
- `grep -c 'iscc_decode\|isccDecode' docs/howto/python.md` returns ≥ 1
- `grep -c 'META_TRIM_NAME' docs/howto/python.md` returns ≥ 1
- `grep -c 'core_opts' docs/howto/python.md` returns ≥ 1
- `grep -c 'encode_component' docs/howto/nodejs.md` returns ≥ 1
- `grep -c 'iscc_decode' docs/howto/nodejs.md` returns ≥ 1
- `grep -c 'META_TRIM_NAME' docs/howto/nodejs.md` returns ≥ 1
- `grep -c 'encodeComponent' docs/howto/java.md` returns ≥ 1
- `grep -c 'isccDecode' docs/howto/java.md` returns ≥ 1
- `grep -c 'META_TRIM_NAME' docs/howto/java.md` returns ≥ 1
- `grep -c 'encode_component' docs/howto/wasm.md` returns ≥ 1
- `grep -c 'iscc_decode' docs/howto/wasm.md` returns ≥ 1
- `grep -c 'meta_trim_name' docs/howto/wasm.md` returns ≥ 1
- `uv run zensical build` exits 0 (docs site builds successfully)
- `mise run check` passes (all pre-commit/pre-push hooks clean)

## Done When

All 4 binding howto guides have "Codec operations" and "Constants" sections with idiomatic code
examples, the docs site builds cleanly, and all verification grep checks pass.
