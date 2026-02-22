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
