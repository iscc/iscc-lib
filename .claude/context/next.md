# Next Work Package

## Step: Update crates/iscc-lib/CLAUDE.md for 30-symbol Tier 1 API

## Goal

Update the per-crate `CLAUDE.md` in `crates/iscc-lib/` to reflect the current 30-symbol Tier 1 API.
The file currently says "22 symbols at crate root" and has stale Tier 1/Tier 2 lists — this
misguides agents working on the crate. Fixing it ensures accurate developer guidance.

## Scope

- **Create**: (none)
- **Modify**: `crates/iscc-lib/CLAUDE.md`
- **Reference**: `crates/iscc-lib/src/lib.rs` (actual exports), `.claude/context/specs/rust-core.md`
    (authoritative 30-symbol table at lines 257-278)

## Not In Scope

- Modifying any Rust source code
- Updating the root `CLAUDE.md` (different file, different purpose)
- Deleting resolved issues from `issues.md` (review agent's job)
- Updating any documentation site pages (`docs/`)
- Changing module visibility or API surface — this is a docs-only step

## Implementation Notes

The file has several stale sections that all need updating together:

1. **Line 8 — binding crate list**: Currently says `iscc-py`, `iscc-napi`, `iscc-wasm`, `iscc-ffi`.
    Add `iscc-jni` (Java JNI binding crate added months ago).

2. **Line 31 — symbol count**: Change `22 symbols at crate root` → `30 symbols at crate root`.

3. **Tier 1 list (lines 33-42)**: Update to match the actual API:

    - Encoding utilities: add `json_to_data_url` (was 1 → now 2)
    - Codec operations: add `encode_component` and `iscc_decode` (was 1 → now 3)
    - Add new category: 4 algorithm constants (`META_TRIM_NAME`, `META_TRIM_DESCRIPTION`,
        `IO_READ_SIZE`, `TEXT_NGRAM_SIZE`)
    - Keep all existing entries unchanged

4. **Tier 2 list (lines 44-48)**: Remove `encode_component` (promoted to Tier 1). It still lives in
    `codec.rs` but is re-exported at crate root via a public wrapper in `lib.rs`.

5. **Dependencies section (lines 113-121)**: Add `serde_json_canonicalizer` — RFC 8785 (JCS)
    compliant JSON serialization, used for metadata canonicalization. Already in `Cargo.toml` but
    missing from the CLAUDE.md dependency listing.

Use `specs/rust-core.md` lines 257-278 as the authoritative reference for the complete Tier 1 table.
Cross-check against `lib.rs` `pub use` re-exports and `pub const` declarations.

## Verification

- `grep '30 symbols' crates/iscc-lib/CLAUDE.md` returns a match
- `grep 'json_to_data_url' crates/iscc-lib/CLAUDE.md` returns a match (in Tier 1 section)
- `grep 'iscc_decode' crates/iscc-lib/CLAUDE.md` returns a match (in Tier 1 section)
- `grep -c 'encode_component' crates/iscc-lib/CLAUDE.md` returns exactly 2 (once in Tier 1, once in
    Common Pitfalls — NOT in Tier 2)
- `grep 'META_TRIM_NAME' crates/iscc-lib/CLAUDE.md` returns a match
- `grep 'serde_json_canonicalizer' crates/iscc-lib/CLAUDE.md` returns a match
- `grep 'iscc-jni' crates/iscc-lib/CLAUDE.md` returns a match
- No Rust compilation or test changes (this is docs-only)

## Done When

All 7 grep verification checks pass, confirming the CLAUDE.md accurately reflects the current
30-symbol Tier 1 API with correct Tier 2 demotion and updated dependency list.
