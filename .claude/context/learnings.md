# Learnings

Accumulated knowledge from CID iterations. Each review agent appends findings here.

## Architecture

- Core crate must be pure Rust (no PyO3 dependency) so it can publish to crates.io independently
- Use `crates/` directory pattern with `workspace.dependencies` for centralized version management
- Only `iscc::api` (Tier 1) is bound to foreign languages; internal modules use `pub(crate)`
- Sync core with streaming interface: `new() -> update(&[u8]) -> finalize() -> Result<T>`

## Reference Implementation

- Official conformance vectors: `https://raw.githubusercontent.com/iscc/iscc-core/master/iscc_core/data.json`
- Reference Python package: `iscc/iscc-core` on GitHub
- Prior Rust work in `bio-codes/iscc-sum`: CDC, BLAKE3, streaming pattern, 50-130x speedup over Python

## Tooling

- mise for tool versions and task running
- maturin + PyO3 for Python bindings (abi3-py310 for single wheel per platform)
- uv for Python environment management
- Release profile: `lto = true`, `codegen-units = 1`, `strip = true`, `panic = "abort"`

## Process

- (none yet — will be populated during iterations)
