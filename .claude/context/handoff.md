## 2026-02-22 — Implement CDC module and gen_data_code_v0

**Done:** Created `cdc.rs` module with the CDC_GEAR lookup table, `alg_cdc_params`,
`alg_cdc_offset`, and `alg_cdc_chunks` functions. Implemented `gen_data_code_v0` in `lib.rs` using
CDC chunking + xxh32 hashing + MinHash to produce similarity-preserving Data-Codes. All 4
conformance vectors pass.

**Files changed:**

- `crates/iscc-lib/src/cdc.rs`: New module with CDC algorithm — gear rolling hash table (256 u32
    values from iscc-core), parameter calculation, two-phase cut point detection (strict then
    relaxed mask), and content-defined chunk splitting. Includes 10 unit tests.
- `crates/iscc-lib/src/lib.rs`: Registered `cdc` module, replaced `gen_data_code_v0` stub with
    implementation (CDC chunks → xxh32 features → MinHash → encode_component), replaced stub test
    with conformance test covering all 4 vectors.

**Verification:** 117 tests pass (up from 107 — added 10 CDC unit tests, replaced 1 stub test with 1
conformance test). `cargo clippy -p iscc-lib -- -D warnings` clean. `cargo fmt -p iscc-lib --check`
clean. No `unsafe` code. All 4 gen_data_code_v0 conformance vectors pass (empty, 2-byte, 1-byte,
8192-byte inputs at 64/128/256 bit lengths).

**Next:** 7/9 gen functions implemented. 2 stubs remain: `gen_image_code_v0` and
`gen_video_code_v0`. These likely use SimHash on feature vectors similar to `gen_audio_code_v0`.
State.md should be updated to reflect 7/9 gen functions and 117 tests.

**Notes:**

- The CDC implementation is non-streaming (takes all data at once), matching the `gen_data_code_v0`
    API. A streaming `DataHasher` pattern (like Python's `DataHasherV0`) could be added later for
    the Tier 2 API if needed.
- `wrapping_add` is used for the gear hash accumulator to handle u32 overflow. This matches the
    iscc-sum Rust implementation and passes all conformance vectors.
- The `utf32` parameter in `alg_cdc_chunks` is wired but unused by `gen_data_code_v0` (always
    false). It will be needed for `gen_text_code_v0` streaming variant if text CDC is ever
    implemented.
