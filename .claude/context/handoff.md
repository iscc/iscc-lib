## 2026-02-23 — Extended Rust core API surface (Titusz + Claude)

**Done:**

- Analyzed `iscc/iscc-sdk` imports from `iscc-core` to identify which functions iscc-lib must expose
    for iscc-sdk to replace its iscc-core dependency with iscc-lib
- Created `specs/rust-core.md` — detailed verifiable spec for the extended Rust core API
- Updated `target.md` to reference the new spec and expand the Rust core section

**Key findings from iscc-sdk analysis:**

- iscc-sdk uses `import iscc_core as ic` and calls 22+ distinct `ic.*` symbols
- The 9 `gen_*_v0` functions are the primary dependency (code generation)
- Text utilities (`text_clean`, `text_trim`, `text_remove_newlines`, `text_collapse`) are called
    directly by the SDK for metadata processing and granular text features
- Algorithm primitives (`sliding_window`, `alg_minhash_256`, `alg_cdc_chunks`) are used for granular
    text processing (independent of the gen functions)
- `soft_hash_video_v0` is called directly for granular video simprints
- `DataHasher` / `InstanceHasher` are used for streaming large files in `code_sum`
- `iscc_decompose` is used to split composite ISCC-CODEs into units
- `encode_base64` encodes simprint digests for granular features
- `conformance_selftest` is called by the SDK CLI
- `core_opts.*` (configuration) stays in the SDK — Rust core uses explicit parameters
- `Code` class and `IsccMeta` model stay in Python ecosystem (SDK / iscc-schema)

**What changed in target.md:**

- Rust core Tier 1 API expanded from 9 gen functions to 22 public symbols
- Added references to `specs/rust-core.md` for detailed verification criteria

**Priority order for implementation:**

1. Structured return types for existing 9 `gen_*_v0` functions (prerequisite for Python dict
    returns)
2. Promote text utilities to `pub` (minimal change — already implemented as `pub(crate)`)
3. Promote algorithm primitives to `pub` (already implemented as `pub(crate)`, need FFI-safe
    signatures for `alg_cdc_chunks` and `alg_simhash`)
4. `soft_hash_video_v0` — promote from private to `pub` (already implemented)
5. `encode_base64`, `iscc_decompose` — new functions
6. `DataHasher` / `InstanceHasher` — new streaming types
7. `conformance_selftest` — new diagnostic function
8. Update all 4 binding crates to expose the new Tier 1 API
9. Python API drop-in compatibility (dict returns, stream inputs)
10. Documentation refinement

**Next:** Read `specs/rust-core.md` and begin with structured return types (item 1), since that
unblocks both the Python dict returns and the additional fields needed by the streaming hashers.
