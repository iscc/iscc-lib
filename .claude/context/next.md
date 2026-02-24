# Next Work Package

## Step: Add Rust how-to guide

## Goal

Create `docs/howto/rust.md` — a per-language usage guide for Rust developers — and wire it into the
documentation navigation. This closes one of the two remaining documentation gaps (the other being
nav icons) and brings the Documentation section closer to "met" status.

## Scope

- **Create**: `docs/howto/rust.md`
- **Modify**: `zensical.toml` (add `{ "Rust" = "howto/rust.md" }` entry to the How-to Guides nav
    section)
- **Reference**: `docs/howto/python.md` (structural template), `docs/rust-api.md` (API details),
    `crates/iscc-lib/src/types.rs` (result struct fields), `crates/iscc-lib/src/lib.rs` (public API
    surface and function signatures)

## Not In Scope

- Nav icons for top-level nav sections (separate step)
- Codec module (Tier 2) documentation — the how-to covers Tier 1 API only; codec details belong in
    the Rust API Reference page
- Tabbed multi-language code blocks — how-to guides are single-language by design (each language
    gets its own page)
- Adding new tests or modifying Rust source code
- Updating the Rust API Reference page (`docs/rust-api.md`) — keep them independent

## Implementation Notes

Follow the **exact section structure** of `docs/howto/python.md` (348 lines), adapted for Rust
idioms. Target approximately 280–350 lines. Sections in order:

1. **Header** — `# Rust` + one-line description matching the Python pattern ("A guide to using
    iscc-lib from Rust...")

2. **Installation** — `cargo add iscc-lib`. Note: pure Rust, no system dependencies.

3. **Code generation** — All 9 `gen_*_v0` functions with examples:

    - Each function returns `IsccResult<XxxCodeResult>` (not `IsccResult<String>` — the API reference
        page is outdated on this; use the actual types from `types.rs`)
    - Use `?` operator in examples (assume `fn main() -> Result<(), Box<dyn std::error::Error>>` or
        similar)
    - Show accessing result struct fields (e.g., `result.iscc`, `result.name`, `result.metahash`)
    - For `gen_data_code_v0` and `gen_instance_code_v0`, input is `&[u8]` (no file-like objects in
        Rust — that's the streaming section's job)
    - For `gen_video_code_v0`, input is `&[Vec<i32>]` (frame signatures)
    - For `gen_audio_code_v0`, input is `&[i32]` (Chromaprint fingerprint, signed)
    - For `gen_image_code_v0`, input is `&[u8]` (1024 bytes, 32×32 grayscale)
    - For `gen_mixed_code_v0`, input is `&[&str]` (ISCC code strings)
    - For `gen_iscc_code_v0`, input is `(&[&str], bool)` where bool is `wide`

4. **Structured results** — Explain Rust result structs vs Python dicts:

    - Table of result types and their fields (mirror the Python how-to table)
    - `#[non_exhaustive]` means new fields may appear in future versions
    - Pattern matching on `Option` fields (description, meta)
    - Show field access: `result.iscc`, `result.name`, etc.

5. **Streaming** — `DataHasher` and `InstanceHasher`:

    - `new() -> update(&[u8]) -> finalize() -> IsccResult<XxxCodeResult>`
    - Show reading a file in chunks with `std::fs::File` and `std::io::Read` trait
    - After `finalize()`, the hasher is consumed (Rust ownership enforces this — no runtime error
        needed)
    - Show that streaming produces identical results to one-shot

6. **Text utilities** — `text_clean`, `text_collapse`, `text_remove_newlines`, `text_trim`:

    - Same 4 functions as Python how-to with Rust syntax
    - `text_trim` takes `(&str, usize)` and returns `String`

7. **Algorithm primitives** — Brief section covering Tier 1 algorithm functions:

    - `sliding_window`, `alg_simhash`, `alg_minhash_256`, `alg_cdc_chunks`
    - One-line description + signature for each; no full examples needed
    - Mention these are building blocks used internally by `gen_*_v0`

8. **Conformance testing** — `conformance_selftest()` returns `bool`

9. **Error handling** — `IsccResult<T>` is `Result<T, IsccError>`, use `?` or `match`

For the `zensical.toml` nav change, insert `{ "Rust" = "howto/rust.md" }` as the **first** entry in
the How-to Guides list (before Python), since Rust is the primary/native language of iscc-lib.

## Verification

- `uv run zensical build` exits 0 (site builds successfully with the new page)
- `docs/howto/rust.md` exists and has 250+ lines
- `zensical.toml` nav contains `"Rust" = "howto/rust.md"` in the How-to Guides section
- `grep -c 'gen_.*_code_v0' docs/howto/rust.md` returns 9 or more (all gen functions documented)
- `grep -c 'DataHasher\|InstanceHasher' docs/howto/rust.md` returns 2 or more (streaming covered)

## Done When

All verification commands pass: the site builds, the Rust how-to page exists with all 9 gen
functions and streaming hashers documented, and the page appears in the How-to Guides navigation.
