## 2026-02-23 — Promote soft_hash_video_v0 to Tier 1 public API

**Done:** Changed `soft_hash_video_v0` from `fn` to `pub fn` in `lib.rs`, making it a Tier 1 public
API symbol. Added 5 integration tests covering basic hashing, 256-bit output, deduplication, empty
input error, and consistency with `gen_video_code_v0`. Updated the flat import verification test to
include the new symbol.

**Files changed:**

- `crates/iscc-lib/src/lib.rs`: Changed `fn soft_hash_video_v0` → `pub fn soft_hash_video_v0`
- `crates/iscc-lib/tests/test_algorithm_primitives.rs`: Added 5 `soft_hash_video_v0` tests, updated
    module docstring and flat import test to include the new symbol

**Verification:** All 193 tests pass (143 unit + 28 algorithm primitives + 22 text utils).
`cargo clippy --workspace --all-targets -- -D warnings` clean. `mise run check` passes all hooks.
`iscc_lib::soft_hash_video_v0` callable from integration tests. Output length verified: 8 bytes for
bits=64, 32 bytes for bits=256. Empty input returns `IsccError::InvalidInput`. Digest matches body
extracted from `gen_video_code_v0` conformance output.

**Next:** The remaining Tier 1 symbols to promote are: `encode_base64` (not implemented),
`iscc_decompose` (not implemented), `DataHasher`/`InstanceHasher` streaming types (not implemented),
and `conformance_selftest` (not implemented). All 4 remaining symbols require new implementation,
not simple visibility changes. `encode_base64` and `iscc_decompose` are likely the simplest next
targets as codec utility functions.

**Notes:** The Rust 1.93+ `cloned_ref_to_slice_refs` clippy lint triggered on `&[frame.clone()]`
patterns in tests — fixed by using `std::slice::from_ref(&frame)` as clippy suggested. No `pub use`
re-export was needed since the function is defined directly in `lib.rs` at crate root scope.
Promoted symbol count is now 18 of 22 Tier 1 targets.
