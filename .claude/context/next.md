# Next Work Package

## Step: Add Rust API documentation page

## Goal

Create a Rust API reference page for the documentation site, covering all 9 public `gen_*_v0`
functions and the Tier 2 `codec` module. This fills the last content gap in the target's
documentation requirement ("Covers Rust API, Python API, and architecture").

## Scope

- **Create**: `docs/rust-api.md`
- **Modify**: `zensical.toml` (add `rust-api.md` to `nav` array)
- **Reference**: `crates/iscc-lib/src/lib.rs` (function signatures, doc comments),
    `crates/iscc-lib/src/codec.rs` (public types: `MainType`, `SubType`, `encode_component`,
    `decode_header`), `docs/api.md` (Python API page — match the style/structure), `docs/index.md`
    (quick start Rust example to stay consistent)

## Implementation Notes

Create `docs/rust-api.md` as a hand-written reference page (no auto-generation — Rust doc comments
are the source, but zensical/mkdocstrings only supports Python). Structure:

1. **Intro paragraph** — explain this is the pure Rust crate (`cargo add iscc-lib`), all functions
    return `IsccResult<String>` containing JSON, link to docs.rs for full rustdoc.

2. **Functions section** — document all 9 `gen_*_v0` functions with:

    - Function signature (Rust syntax-highlighted code block)
    - Brief description (from the doc comments in lib.rs)
    - Parameter table (name, type, description, default where applicable)
    - Return value description
    - Short usage example

    Function signatures to document (from lib.rs):

    - `gen_meta_code_v0(name: &str, description: Option<&str>, meta: Option<&str>, bits: u32) -> IsccResult<String>`
    - `gen_text_code_v0(text: &str, bits: u32) -> IsccResult<String>`
    - `gen_image_code_v0(pixels: &[u8], bits: u32) -> IsccResult<String>`
    - `gen_audio_code_v0(cv: &[i32], bits: u32) -> IsccResult<String>`
    - `gen_video_code_v0(frame_sigs: &[Vec<i32>], bits: u32) -> IsccResult<String>`
    - `gen_mixed_code_v0(codes: &[&str], bits: u32) -> IsccResult<String>`
    - `gen_data_code_v0(data: &[u8], bits: u32) -> IsccResult<String>`
    - `gen_instance_code_v0(data: &[u8], bits: u32) -> IsccResult<String>`
    - `gen_iscc_code_v0(codes: &[&str], wide: bool) -> IsccResult<String>`

3. **Types section** — document the public types from `codec` module:

    - `IsccError` enum (and `IsccResult<T>` alias)
    - `MainType` and `SubType` enums (brief table of variants)
    - Note that `codec` is Tier 2 — available to Rust users but not exposed via FFI

4. **Error handling** — brief section explaining `IsccResult` pattern and `IsccError::InvalidInput`

Style guidelines:

- Match the tone of `docs/api.md` (concise, practical)
- Use `===` tabbed examples only if showing multi-language — here just use plain Rust code blocks
- Use admonitions (`!!! note`, `!!! tip`) sparingly for important callouts
- Keep each function's documentation to ~10-15 lines (signature + description + params + example)

For `zensical.toml`, insert `"rust-api.md"` into the nav array after `"architecture.md"` and before
`"api.md"`, so the order is: index → architecture → rust-api → api (general → specific, Rust before
Python).

## Verification

- `docs/rust-api.md` exists and contains documentation for all 9 `gen_*_v0` functions
- `zensical.toml` nav includes `"rust-api.md"`
- All function signatures in the docs match the actual signatures in `lib.rs`
- `uv run zensical build` succeeds without errors
- The built site contains the Rust API page (check `site/rust-api/index.html` exists)
- No broken internal links in the docs

## Done When

The Rust API documentation page builds successfully with zensical, covers all 9 `gen_*_v0` functions
with correct signatures, and is linked in the site navigation.
