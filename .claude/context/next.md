# Next Work Package

## Step: Fix stale WASM CLAUDE.md documentation

## Goal

Update `crates/iscc-wasm/CLAUDE.md` to accurately reflect that DataHasher and InstanceHasher are
fully bound and the total exported symbol count is 23 (not 22). This resolves the
`[low] iscc-wasm: Stale CLAUDE.md says DataHasher/InstanceHasher not yet bound` issue.

## Scope

- **Create**: (none)
- **Modify**: `crates/iscc-wasm/CLAUDE.md`
- **Reference**: `crates/iscc-wasm/src/lib.rs` (to verify the actual exports)

## Not In Scope

- Adding new WASM bindings or changing any Rust code
- Updating other crate CLAUDE.md files (e.g., iscc-jni Javadoc mismatch is a separate concern)
- Creating `crates/iscc-ffi/README.md` (separate task)
- Any changes to `src/lib.rs` or test files

## Implementation Notes

Three changes needed in `crates/iscc-wasm/CLAUDE.md`:

1. **Line 117**: Change "All 22 Tier 1 symbols are bound" → "All 23 Tier 1 symbols are bound, plus 2
    streaming types". The 23 count matches the Tier 1 API definition in target.md and the actual
    exports in `lib.rs`.

2. **Lines 130-131**: Replace the paragraph that says "DataHasher and InstanceHasher (streaming
    types) are not yet bound. Binding stateful types requires `#[wasm_bindgen]` on a struct with
    constructor/method annotations." with accurate documentation stating they ARE bound as
    `#[wasm_bindgen]` structs with constructor, `update()`, and `finalize()` methods. Mention the
    `Option<Inner>` finalize-once pattern (consistent with Python/napi/JNI crates).

3. **Add DataHasher/InstanceHasher to the export list** (around lines 128-129): Add a line for the 2
    streaming types, e.g., "**2 streaming types:** `DataHasher`, `InstanceHasher`".

Keep edits minimal and factual. Do not rewrite unrelated sections.

## Verification

- `grep -c "not yet bound" crates/iscc-wasm/CLAUDE.md` outputs `0`
- `grep "All 23 Tier 1" crates/iscc-wasm/CLAUDE.md` matches (confirms updated count)
- `grep "DataHasher" crates/iscc-wasm/CLAUDE.md` shows them listed as bound exports
- `grep "InstanceHasher" crates/iscc-wasm/CLAUDE.md` shows them listed as bound exports
- No Rust source files modified (only CLAUDE.md changed)

## Done When

All four grep verification checks pass and the only modified file is `crates/iscc-wasm/CLAUDE.md`.
