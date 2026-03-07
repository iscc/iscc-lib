# Next Work Package

## Step: C++ gen_mixed_code_v0 test + nested vector null-safety

## Goal

Close the two remaining C++ code quality gaps: add the missing `gen_mixed_code_v0` smoke test and
harden nested vector marshaling against nullptr from empty inner vectors. This leaves only package
manager manifests as the final C++ issue item.

## Scope

- **Create**: (none)
- **Modify**:
    - `packages/cpp/include/iscc/iscc.hpp` — add `safe_data` overloads for inner vector elements in
        `alg_simhash`, `soft_hash_video_v0`, `gen_video_code_v0`
    - `packages/cpp/tests/test_iscc.cpp` — add `gen_mixed_code_v0` smoke test
- **Reference**:
    - `crates/iscc-ffi/src/lib.rs` lines 466-488 (gen_mixed_code_v0 FFI implementation)
    - `crates/iscc-ffi/include/iscc.h` line 305 (C signature)
    - `packages/cpp/include/iscc/iscc.hpp` lines 42-45 (existing `safe_data` for `uint8_t`)

## Not In Scope

- Package manager manifests (vcpkg.json, conanfile.py, pkg-config) — separate step
- Adding conformance vector loading from data.json to C++ tests (C++ uses `conformance_selftest()`)
- Refactoring the test framework (e.g., switching to GoogleTest)
- Documentation updates (no API changes, just bug hardening)

## Implementation Notes

### gen_mixed_code_v0 test

The function takes `vector<string>` of ISCC content code strings. To test it:

1. Generate two or more content codes using existing gen functions (e.g., `gen_text_code_v0` and
    `gen_image_code_v0` — these already have tests with known working inputs)
2. Pass the resulting ISCC strings to `gen_mixed_code_v0`
3. Assert the result starts with `"ISCC:"`

Follow the existing test pattern (see tests 31-32 for audio/video — use `assert_starts_with`).
Insert after the gen_video_code_v0 test (currently test 32) and before DataHasher move test (33).
Renumber subsequent tests.

### Nested vector null-safety

Three functions iterate over inner vectors and call `.data()` which returns nullptr for empty
vectors on some libstdc++ implementations:

1. **`alg_simhash`** (line 349-364): iterates `vector<vector<uint8_t>>`, pushes `d.data()`
2. **`soft_hash_video_v0`** (line 402-418): iterates `vector<vector<int32_t>>`, pushes `f.data()`
3. **`gen_video_code_v0`** (line 471-485): iterates `vector<vector<int32_t>>`, pushes `f.data()`

Fix: Add a `safe_data` overload for `int32_t` vectors in `detail` namespace (matching the existing
`uint8_t` pattern at line 42). Then use `safe_data(d)` / `safe_data(f)` instead of `d.data()` /
`f.data()` in the three iteration loops. For `alg_simhash`, use the existing `uint8_t` overload.

```cpp
// In detail namespace, add alongside existing safe_data:
inline const int32_t* safe_data(const std::vector<int32_t>& v) {
    static const int32_t sentinel = 0;
    return v.empty() ? &sentinel : v.data();
}
```

Then update the three loops to use `detail::safe_data(d)` / `detail::safe_data(f)` instead of
`d.data()` / `f.data()`.

## Verification

- `cargo build -p iscc-ffi` succeeds (prerequisite for C++ tests)
- C++ test suite compiles and passes with new test:
    ```
    cd packages/cpp && cmake -B build -S tests && cmake --build build && ./build/test_iscc
    ```
    Expected: 53 passed (52 existing + 1 new), 0 failed
- `grep -c 'gen_mixed_code_v0' packages/cpp/tests/test_iscc.cpp` returns ≥1
- `grep -c 'safe_data' packages/cpp/include/iscc/iscc.hpp` returns ≥5 (was 3: definition + 2 uses;
    now: 2 overloads + 5 uses)
- `cargo clippy --workspace --all-targets -- -D warnings` clean (no Rust changes, but verify)

## Done When

All verification criteria pass: C++ test suite reports 53 passed / 0 failed with the new
gen_mixed_code_v0 test, and all three nested vector functions use safe_data for inner elements.
