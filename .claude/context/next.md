# Next Work Package

## Step: Promote soft_hash_video_v0 to Tier 1 public API

## Goal

Promote `soft_hash_video_v0` from private `fn` to `pub fn` as a Tier 1 public API symbol. This
brings the promoted symbol count from 17 to 18 of 22 and exposes the video similarity hashing
primitive that downstream consumers need for building custom video fingerprinting pipelines.

## Scope

- **Create**: none
- **Modify**: `crates/iscc-lib/src/lib.rs` (change `fn soft_hash_video_v0` →
    `pub fn   soft_hash_video_v0`), `crates/iscc-lib/tests/test_algorithm_primitives.rs` (add
    `soft_hash_video_v0` integration tests)
- **Reference**: `crates/iscc-lib/src/lib.rs` (lines 520-544, current implementation),
    `crates/iscc-lib/tests/test_algorithm_primitives.rs` (existing test pattern),
    `reference/iscc-core/iscc_core/code_content_video.py` (Python reference for behavior)

## Implementation Notes

This is simpler than prior promotions because `soft_hash_video_v0` is defined directly in `lib.rs`
(not in a submodule), so no module visibility change or re-export is needed — just the function
visibility.

1. In `lib.rs` line 524: change `fn soft_hash_video_v0` → `pub fn soft_hash_video_v0`
2. Add a doc-comment line noting this is a Tier 1 public API function (the existing docstring is
    fine, just ensure it's `///` not `//`)
3. No `pub use` re-export needed — the function is already at crate root scope

Add integration tests to `test_algorithm_primitives.rs`:

- **Basic hashing**: call `soft_hash_video_v0` with known frame signatures, verify it returns a
    digest of `bits/8` bytes length
- **Default 64-bit output**: verify `bits=64` returns 8-byte digest
- **256-bit output**: verify `bits=256` returns 32-byte digest
- **Deduplication**: verify that duplicate frame signatures don't change the result (pass same sigs
    twice, compare with single copy)
- **Empty input error**: verify empty `frame_sigs` slice returns `IsccError::InvalidInput`
- **Consistency with gen_video_code_v0**: verify that calling `soft_hash_video_v0` with the same
    inputs used by `gen_video_code_v0` conformance vectors produces a digest that matches the body
    portion of the encoded video code (extract body via `codec::decode_header`)
- **Flat import path**: verify `iscc_lib::soft_hash_video_v0` compiles

Run workspace-wide clippy (`cargo clippy --workspace --all-targets -- -D warnings`) before
finishing, per learnings about newer lints only surfacing in `--all-targets` mode.

## Verification

- `cargo test -p iscc-lib` passes (all existing 188 tests + new soft_hash_video integration tests)
- `cargo clippy --workspace --all-targets -- -D warnings` is clean
- `iscc_lib::soft_hash_video_v0` is callable from integration tests
- Function returns correct-length digest (8 bytes for bits=64, 32 bytes for bits=256)
- Empty input returns appropriate error

## Done When

All verification criteria pass: tests green, clippy clean, `soft_hash_video_v0` accessible as a
public function from integration tests, and output matches expected digest lengths.
