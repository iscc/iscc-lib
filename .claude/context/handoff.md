## 2026-03-07 — C++ gen_mixed_code_v0 test + nested vector null-safety

**Done:** Added `safe_data` overload for `int32_t` vectors and applied it to all three nested vector
iteration loops (`alg_simhash`, `soft_hash_video_v0`, `gen_video_code_v0`). Added
`gen_mixed_code_v0` smoke test to the C++ test suite.

**Files changed:**

- `packages/cpp/include/iscc/iscc.hpp`: Added `detail::safe_data(const std::vector<int32_t>&)`
    overload. Changed `d.data()` → `detail::safe_data(d)` in `alg_simhash`, `f.data()` →
    `detail::safe_data(f)` in `soft_hash_video_v0` and `gen_video_code_v0`
- `packages/cpp/tests/test_iscc.cpp`: Added test 33 (`gen_mixed_code_v0`) using text + image content
    codes as input. Renumbered old test 33 (DataHasher move) to 34

**Verification:**

- `cargo build -p iscc-ffi` — success
- C++ test suite: 53 passed, 0 failed (52 existing + 1 new `gen_mixed_code_v0`)
- `grep -c 'gen_mixed_code_v0' packages/cpp/tests/test_iscc.cpp` → 3 (≥1 required)
- `grep -c 'safe_data' packages/cpp/include/iscc/iscc.hpp` → 10 (≥5 required: 2 overloads + 8 uses)
- `cargo clippy --workspace --all-targets -- -D warnings` — clean
- `mise run check` — all 15 hooks pass

**Next:** The remaining C++ issue item is package manager manifests (vcpkg.json, conanfile.py,
pkg-config). After that, only low-priority issues remain (Swift, Kotlin, logos).

**Notes:** The `safe_data` pattern for `uint8_t` was already used in 3 top-level functions
(`encode_base64`, `gen_image_code_v0`, `gen_data_code_v0`, `gen_instance_code_v0`,
`alg_cdc_chunks`). The `int32_t` overload follows the same sentinel pattern. The existing `uint8_t`
overload was already correctly applied to `alg_simhash`'s inner elements — corrected: it was NOT
applied to `alg_simhash`'s inner `d.data()` until this change. All three nested-vector functions now
use `safe_data` consistently.
