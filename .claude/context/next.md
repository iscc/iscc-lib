# Next Work Package

## Step: Update docs for 4-parameter gen_sum_code_v0

## Goal

Update all documentation files that still reference the old 3-parameter
`gen_sum_code_v0(path, bits, wide)` signature to reflect the current 4-parameter signature with
`add_units` and the `units` result field. This unblocks the Documentation section from "partially
met" to "met."

## Scope

- **Create**: (none)
- **Modify**:
    - `docs/architecture.md` — line 131: add `add_units` to pseudocode signature
    - `docs/rust-api.md` — lines 270-296: update signature, add `add_units` parameter row, mention
        `units` field in description and `SumCodeResult` struct, update code example to pass 4th arg
    - `docs/howto/rust.md` — line 166: add `false` 4th argument to code example
    - `docs/c-ffi-api.md` — lines 328-362: update C signature to include `bool add_units`, add
        parameter row, add `units` field (`char **units`) to `IsccSumCodeResult` struct docs, update
        `iscc_free_sum_code_result` doc to mention freeing `units`
    - `docs/howto/c-cpp.md` — lines 101, 231: add `false` 4th argument to both C code examples
- **Reference**:
    - `crates/iscc-lib/src/lib.rs` lines 968-975 — actual Rust signature
    - `crates/iscc-lib/src/types.rs` lines 98-107 — `SumCodeResult` struct definition
    - `crates/iscc-ffi/include/iscc.h` lines 31-52, 528-531 — actual C signature and struct

## Not In Scope

- Updating Python/Node.js/WASM howto examples (they use keyword args or defaults — `add_units`
    defaults to `false` and the examples are correct as-is)
- Updating per-crate READMEs (they reference `gen_sum_code_v0` by name only, not signature)
- Updating `docs/index.md` (only has function name in table, not signature)
- Regenerating `llms.txt` / `llms-full.txt` (automated by build pipeline)
- Any source code changes

## Implementation Notes

### docs/architecture.md (line 131)

Change:

```
pub fn gen_sum_code_v0(path, bits, wide) -> IsccResult<SumCodeResult>
```

to:

```
pub fn gen_sum_code_v0(path, bits, wide, add_units) -> IsccResult<SumCodeResult>
```

### docs/rust-api.md (lines 270-296)

1. **Signature** (line 275): change to
    `pub fn gen_sum_code_v0(path: &Path, bits: u32, wide: bool, add_units: bool) -> IsccResult<SumCodeResult>`
2. **Parameter table** (after line 282): add row for `add_units` | `bool` |
    `Include individual Data-Code and Instance-Code ISCC strings in the result`
3. **Description** (lines 284-286): update to mention `units` field: "Returns a `SumCodeResult` with
    `iscc`, `datahash`, `filesize`, and optionally `units` (when `add_units` is `true`)."
4. **Code example** (line 292): `gen_sum_code_v0(Path::new("document.pdf"), 64, false, false)?;`

### docs/howto/rust.md (line 166)

Change:

```rust
let result = gen_sum_code_v0(Path::new("example.bin"), 64, false)?;
```

to:

```rust
let result = gen_sum_code_v0(Path::new("example.bin"), 64, false, false)?;
```

### docs/c-ffi-api.md (lines 328-362)

1. **C signature** (lines 328-332): add `bool add_units` parameter
2. **Parameter table** (after line 339): add row for `add_units` | `bool` |
    `Include individual Data-Code and Instance-Code strings in the result`
3. **Struct** (lines 344-349): add `char **units` field with doc:
    `NULL-terminated array of Data-Code and Instance-Code ISCC strings, or NULL when add_units is false`
4. **Free doc** (line 361): update to mention releasing `units` array

### docs/howto/c-cpp.md (lines 101, 231)

Both C examples need `false` added as 4th argument:

- Line 101: `iscc_gen_sum_code_v0("document.pdf", 64, false, false);`
- Line 231: `iscc_gen_sum_code_v0("file.bin", 64, false, false);`

## Verification

- `grep -n 'gen_sum_code_v0.*bits.*wide' docs/rust-api.md docs/architecture.md docs/howto/rust.md docs/c-ffi-api.md docs/howto/c-cpp.md | grep -v add_units`
    returns no matches (all signatures include `add_units`)
- `grep -c 'add_units' docs/rust-api.md` returns at least 2 (signature + parameter table row)
- `grep -c 'add_units' docs/c-ffi-api.md` returns at least 2 (signature + parameter table row)
- `grep 'units' docs/c-ffi-api.md | grep -c 'char \*\*'` returns at least 1 (struct field)
- `uv run zensical build` exits 0 (docs site builds cleanly)
- `mise run check` passes (pre-commit hooks: formatting, YAML, TOML, markdown)

## Done When

All verification criteria pass — every `gen_sum_code_v0` reference in docs shows the 4-parameter
signature with `add_units`, and the documentation site builds cleanly.
