## 2026-02-22 — Implement gen_audio_code_v0 with multi-stage SimHash

**Done:** Implemented `gen_audio_code_v0` and `soft_hash_audio_v0` with multi-stage SimHash (overall
\+ quarters + sorted thirds). Changed parameter type from `&[u32]` to `&[i32]` since Chromaprint
features are signed integers. Added `array_split` helper for numpy-style partitioning.

**Files changed:**

- `crates/iscc-lib/src/lib.rs`: Added `array_split` helper function, `soft_hash_audio_v0` (3-stage
    SimHash: overall 4B + quarters 4×4B + sorted-thirds 3×4B = 32B), `gen_audio_code_v0` body using
    `SubType::Audio`. Changed stub signature from `&[u32]` to `&[i32]`. Replaced stub test with 3
    direct tests + 1 full conformance test (5 vectors) + 4 `array_split` unit tests.

**Verification:** All 92 tests pass (was 85 before, +7 net: +3 audio direct tests, +1 conformance
test, +4 array_split tests, -1 removed stub test). `cargo clippy -D warnings` clean.
`cargo fmt --check` clean. No `unsafe` code.

**Next:** The learnings file incorrectly states `gen_audio_code_v0` takes `&[u32]` — the review
agent should correct this to `&[i32]`. Next gen function candidates: `gen_image_code_v0` (needs
pixel-based SimHash), `gen_data_code_v0` (needs CDC chunking + MinHash), `gen_video_code_v0` (needs
frame signature SimHash), `gen_mixed_code_v0` (combines content codes), or `gen_iscc_code_v0`
(composite code from units). `gen_mixed_code_v0` or `gen_iscc_code_v0` could be good next targets
since they compose existing gen functions.

**Notes:** The Python reference uses `more_itertools.divide` (not `numpy.array_split` as mentioned
in next.md), but the semantics are identical: first `len % n` parts get one extra element. The Rust
`soft_hash_audio_v0` always computes all 32 bytes (no early return based on `bits`); truncation is
handled by `encode_component`. This is a simplification vs the Python which tracks `bit_length` for
early exit, but produces identical results since `encode_component` truncates anyway.
