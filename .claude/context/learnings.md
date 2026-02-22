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
- Chromaprint feature vectors are signed `i32` (not `u32` or `f32`) — `gen_audio_code_v0` takes
    `&[i32]` because conformance vectors include negative values
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
    = 32 bytes)
- `gen_text_code_v0` uses MinHash (NOT SimHash):
    `text_collapse → sliding_window(13) → xxh32 →   alg_minhash_256`. The `xxhash-rust` crate
    (feature `xxh32`) provides the hash function. The `minhash` module is ported from
    `bio-codes/iscc-sum` with MPA/MPB constants inlined
- `gen_data_code_v0` also uses MinHash (`alg_minhash_256`) plus CDC — the minhash module is shared
- `gen_meta_code_v0` normalizes name with `text_clean → text_remove_newlines → text_trim(128)` and
    description with `text_clean → text_trim(4096)` (no newline removal for description)
- `soft_hash_audio_v0` uses multi-stage SimHash: overall 4B + quarters 4×4B + sorted-thirds 3×4B =
    32B. Python reference uses `more_itertools.divide` (not `numpy.array_split`), but semantics are
    identical (first `len % n` parts get one extra element)
- `alg_simhash` output length matches input digest length — 4-byte digests in → 4-byte SimHash out.
    This makes it reusable for audio (4B digests) vs text/meta (32B BLAKE3 digests)
- `gen_mixed_code_v0` takes `&[&str]` (ISCC code strings, optional "ISCC:" prefix). The
    `soft_hash_codes_v0` helper prepares nbytes-length entries from `raw[0]` (header first byte) +
    body truncated to `nbytes-1`, then feeds to `alg_simhash`. Zero-padding handles short bodies
