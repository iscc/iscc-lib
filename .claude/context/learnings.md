# Learnings

Accumulated knowledge from CID iterations. Each review agent appends findings here.

## Architecture

- Core crate must be pure Rust (no PyO3 dependency) so it can publish to crates.io independently
- Use `crates/` directory pattern with `workspace.dependencies` for centralized version management
- Only `iscc::api` (Tier 1) is bound to foreign languages; internal modules use `pub(crate)`
- Sync core with streaming interface: `new() -> update(&[u8]) -> finalize() -> Result<T>`

## Reference Implementation

- Official conformance vectors:
    `https://raw.githubusercontent.com/iscc/iscc-core/master/iscc_core/data.json`
- Reference Python package: `iscc/iscc-core` on GitHub
- Prior Rust work in `bio-codes/iscc-sum`: CDC, BLAKE3, streaming pattern, 50-130x speedup over
    Python

## Tooling

- mise for tool versions and task running
- maturin + PyO3 for Python bindings (abi3-py310 for single wheel per platform)
- uv for Python environment management
- Release profile: `lto = true`, `codegen-units = 1`, `strip = true`, `panic = "abort"`

## Process

- `gen_meta_code_v0` in iscc-core has no `extra` parameter — only `name, description, meta, bits`
- Chromaprint feature vectors are `u32` (integer sequences), not `f32` — `gen_audio_code_v0` takes
    `&[u32]`
- `gen_instance_code_v0` accepts a `bits` parameter in the Python reference (default 64)
- `gen_iscc_code_v0` takes `(codes, wide: bool)` — Python uses `wide=False` for 128-bit vs 256-bit
    output
- ST_ISCC SubType values are 0-7 (TEXT=0..MIXED=4, SUM=5, NONE=6, WIDE=7) — they share values 0-4
    with ST_CC, making a unified Rust SubType enum with values 0-7 correct for header encoding
- `codec` module is Tier 2 (`pub mod codec`) per notes/04, not `pub(crate)` — Tier 2 items are
    public Rust API but not exposed through FFI bindings
- Conformance test pattern: `include_str!("../tests/data.json")` + `serde_json::Value` for flexible
    parsing; `"stream:"` prefix in test vectors denotes hex-encoded byte data (empty after prefix =
    empty bytes); `hex` crate decodes test vector data
- `gen_instance_code_v0` is the simplest gen function: BLAKE3 hash → `encode_component` → "ISCC:"
    prefix. Good first implementation to establish patterns before tackling CDC/MinHash complexity
- `soft_hash_meta_v0` interleaves name/description SimHash digests in 4-byte chunks (8 chunks total
    = 32 bytes); `gen_text_code_v0` uses the same `text_collapse` → `sliding_window(3)` →
    BLAKE3-per-ngram → `alg_simhash` pipeline but without the name/extra interleaving
- `gen_meta_code_v0` normalizes name with `text_clean → text_remove_newlines → text_trim(128)` and
    description with `text_clean → text_trim(4096)` (no newline removal for description)
