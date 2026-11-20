# Next Work Package

## Step: C++ wrappers pass non-NULL pointers for every vector input

## Goal

Close the issue "C++ list wrappers pass NULL for an empty vector": every `std::vector` input in
`iscc.hpp` reaches the FFI through `detail::safe_data`, so `{}` reports the core's own error instead
of the FFI's "must not be NULL". This leaves one pointer form on the C++ surface.

## Alternatives Considered

- **Chosen:** C++ `safe_data` everywhere. It is the handoff's Next, it touches one header plus its
    test file, and it closes a v0.8.0-blocking `normal` issue.
- **Rejected:** `scripts/check_spec_paths.py` leniency (3 sub-fixes plus replay-test rework). It is
    a larger change to a gate script, and the handoff puts it second.

## Scope

- **Modify**: `packages/cpp/include/iscc/iscc.hpp`; `packages/cpp/CLAUDE.md` (the Type Mapping rows
    that say "`.data()` + `.size()`, or `detail::safe_data()`", and the "Empty vector data pointers"
    pitfall: change both to "always `detail::safe_data()`")
- **Tests**: `packages/cpp/tests/test_iscc.cpp`
- **Reference**: `crates/iscc-ffi/src/lib.rs` (`iscc_gen_video_code_v0` :418,
    `iscc_gen_mixed_code_v0` :466, `iscc_gen_iscc_code_v0` :571, `iscc_alg_simhash` :1201,
    `iscc_soft_hash_video_v0` :1363); `.github/workflows/ci.yml` job `cpp`

## Not In Scope

- Reordering the NULL and count checks in `crates/iscc-ffi`. That changes FFI behaviour and moves
    `iscc.h`, `NativeMethods.g.cs` and the C tests. The wrapper is the fix.
- The raw-pointer overloads (`encode_base64(const uint8_t*, size_t)`, `gen_data_code_v0(ptr, len)`,
    `DataHasher::update(const uint8_t*, size_t)`, …). The caller owns that pointer.
- The `rust-core.md` Tier 1 rows for `alg_simhash` and `sliding_window`. The reviewer who closes the
    issue corrects them.
- Any `data-edge.json` edit. No row depends on the NULL path.

## Implementation Notes

There are nine vector-backed `.data()` call sites that pass through to the FFI (line numbers as of
today):

| Line     | Wrapper                                 | Argument(s)                     | Defect today?                |
| -------- | --------------------------------------- | ------------------------------- | ---------------------------- |
| 421      | `alg_simhash`                           | `ptrs`, `lens`                  | no (FFI short-circuits on 0) |
| 476      | `soft_hash_video_v0`                    | `ptrs`, `lens`                  | yes                          |
| 546      | `gen_video_code_v0`                     | `ptrs`, `lens`                  | yes                          |
| 559      | `gen_mixed_code_v0`                     | `ptrs` (`vector<const char*>`)  | yes                          |
| 605      | `gen_iscc_code_v0`                      | `ptrs` (`vector<const char*>`)  | yes                          |
| 330      | `encode_component`                      | `digest` (`vector<uint8_t>`)    | no (FFI checks length first) |
| 431      | `alg_minhash_256`                       | `features` (`vector<uint32_t>`) | no (FFI checks length first) |
| 697, 747 | `DataHasher` / `InstanceHasher::update` | `data` (`vector<uint8_t>`)      | no (FFI checks length first) |

Route all nine through `detail::safe_data`. The five that are not defects today still change: under
the finish-the-pattern rule (decisions.md 2026-11-19), two pointer forms on one surface are the
defect.

The three existing overloads cover only `uint8_t`, `int32_t` and `const char*`. The new sites also
need `const uint8_t*`, `const int32_t*`, `size_t` and `uint32_t` elements. Use **one** helper form,
not seven copy-pasted overloads. A single
`template <typename T> const T* safe_data(const std::vector<T>& v)` with a
`static const T sentinel{}` replaces all three and keeps every call site unchanged; advance may pick
another form if it fits better. Keep the doc comment, and keep the sentinel read-only, because the
FFI never dereferences it when the count is 0.

Core messages for `{}` (the FFI NULL messages are `"<param> must not be NULL"`):

- `gen_mixed_code_v0({})` → contains `"at least 2 Content-Codes"`
- `gen_iscc_code_v0({})` → contains `"at least 2 ISCC unit codes"`
- `gen_video_code_v0({})` and `soft_hash_video_v0({}, 64)` → contain
    `"frame_sigs must not be empty"`. Use this full needle: `"must not be"` alone also matches the
    NULL message.
- `alg_simhash({})` → contains `"must not be empty"`. It is green before the fix too. Add it for
    completeness; the source-grep criterion below pins its pointer.

Add one `assert_throws_with` test per wrapper (helper at test_iscc.cpp:88), next to the existing
`encode_seq({})` test (#42). Also add positive `{}` round trips for the non-defect siblings:
`alg_minhash_256({})` returns 32 bytes, `encode_component(..., {})` throws the core's own error or
returns a value (assert that it does not mention NULL), and
`DataHasher().update(std::vector<uint8_t>{})` followed by `finalize` equals `gen_data_code_v0({})`.

This step is binding-only: neither the CRAP gate nor the iai gate trips. Locally, cmake comes from
`uv run --with cmake cmake …` (or `uvx cmake`).

## Verification

- `cargo build -p iscc-ffi`, then in `packages/cpp` the CI `cpp` job's commands
    (`cmake -B build -DCMAKE_BUILD_TYPE=Debug -DFFI_LIB_DIR=../../target/debug -DSANITIZE_ADDRESS=ON`,
    `cmake --build build`, `LD_LIBRARY_PATH=../../target/debug ./build/tests/test_iscc`) exit 0
    with 0 failed
- The test output shows a passing `{}` test for each of `gen_mixed_code_v0`, `gen_iscc_code_v0`,
    `gen_video_code_v0`, `soft_hash_video_v0` and `alg_simhash`, each checking the core message
    listed above
- `grep -n '\.data()' packages/cpp/include/iscc/iscc.hpp | grep -v 'v\.data()'` prints nothing: the
    only `.data()` left is inside the `safe_data` helper (keep its parameter named `v`)
- `uv run prek run --files packages/cpp/include/iscc/iscc.hpp packages/cpp/tests/test_iscc.cpp packages/cpp/CLAUDE.md`
    exits 0
- `packages/cpp/CLAUDE.md` no longer describes `.data()` as an accepted way to pass an input vector

## Done When

All five verification criteria pass on the working tree.
