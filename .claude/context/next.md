# Next Work Package

## Step: Fix C++ gen_audio_code_v0 NULL pointer crash on empty vector

## Goal

Fix the `gen_audio_code_v0` wrapper in `iscc.hpp` that crashes when passed an empty
`std::vector<int32_t>` because `cv.data()` returns NULL on some implementations. This is a real
crash bug — the Rust core and C# binding both handle empty vectors correctly.

## Scope

- **Modify**: `packages/cpp/include/iscc/iscc.hpp` (line 472: `cv.data()` → `detail::safe_data(cv)`)
- **Modify**: `packages/cpp/tests/test_iscc.cpp` (add empty-vector smoke test)
- **Reference**: `packages/cpp/include/iscc/iscc.hpp` lines 42-56 (existing `safe_data` overloads)

## Not In Scope

- Fixing the Conan recipe (`conanfile.py`) — separate `normal` issue, separate step
- Updating .NET docs (`docs/howto/dotnet.md`) — separate `normal` issue
- Fixing "View as Markdown" 404 on docs site — separate `normal` issue
- Refactoring other functions that already use `safe_data` correctly
- Adding conformance test vectors for audio — existing smoke test pattern is sufficient

## Implementation Notes

The fix is a single token change on line 472 of `iscc.hpp`:

```cpp
// Before (line 472):
detail::UniqueString s(iscc_gen_audio_code_v0(cv.data(), cv.size(), bits));

// After:
detail::UniqueString s(iscc_gen_audio_code_v0(detail::safe_data(cv), cv.size(), bits));
```

The `detail::safe_data` helper already has an `int32_t` overload (line 49) that returns a static
empty array pointer instead of NULL for empty vectors. This is the same pattern used by
`gen_video_code_v0`, `alg_simhash`, `soft_hash_video_v0`, and other functions.

For the test, add a new numbered test block (35) after the existing DataHasher move semantics test
(34). The test should:

1. Call `iscc::gen_audio_code_v0({})` with an empty vector
2. Assert the result starts with `"ISCC:"` (matching the C# behavior which returns
    `ISCC:EIAQAAAAAAAAAAAA` for empty input)

Follow the existing test pattern: numbered comment, scoped block, `assert_starts_with` helper.

## Verification

- `cmake --build build-ci && ./build-ci/tests/test_iscc` passes with 54 tests (was 53)
- The empty-vector test produces `ISCC:EIAQAAAAAAAAAAAA` (matching C# behavior)
- `cargo clippy --workspace --all-targets -- -D warnings` clean
- `grep -c 'detail::safe_data' packages/cpp/include/iscc/iscc.hpp` returns 11 (was 10)

## Done When

All verification criteria pass — the empty-vector audio test succeeds and no regressions in the
existing 53 tests.
