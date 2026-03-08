## 2026-03-08 — Fix C++ gen_audio_code_v0 NULL pointer crash on empty vector

**Done:** Fixed the `gen_audio_code_v0` wrapper in `iscc.hpp` to use `detail::safe_data(cv)` instead
of `cv.data()`, preventing a NULL pointer crash when passed an empty `std::vector<int32_t>`. Added
an empty-vector smoke test (test 35) that verifies the output matches the expected ISCC code.

**Files changed:**

- `packages/cpp/include/iscc/iscc.hpp`: Changed `cv.data()` to `detail::safe_data(cv)` on line 472
    in `gen_audio_code_v0`
- `packages/cpp/tests/test_iscc.cpp`: Added test 35 — `gen_audio_code_v0` with empty vector, asserts
    result equals `ISCC:EIAQAAAAAAAAAAAA`

**Verification:**

- `cmake --build build-ci && ./build-ci/tests/test_iscc` passes with 54 tests (was 53)
- Empty-vector test produces `ISCC:EIAQAAAAAAAAAAAA` (matching C# behavior)
- `cargo clippy --workspace --all-targets -- -D warnings` clean
- `mise run check` — all 15 hooks pass
- `detail::safe_data` call site count is 9 (was 8). Note: next.md predicted 11→10 which included the
    2 definition sites — the actual `detail::safe_data` *call* count is 9
- ASAN enabled during test run — no memory errors detected

**Next:** Only `low` priority issues remain (Swift bindings, Kotlin bindings, language logos,
version sync for vcpkg/conan manifests). The CID loop should signal idle or pick up the next
available issue from issues.md.

**Notes:** The next.md predicted `grep -c 'detail::safe_data'` would return 11 (was 10), but the
actual count is 9 (was 8). The discrepancy is because the 2 function *definitions* of `safe_data` in
the `detail` namespace don't include the `detail::` prefix qualifier. The fix itself is correct —
single token change, matching the exact pattern used by all other functions.
