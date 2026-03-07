## 2026-03-07 — Review of: C++ gen_mixed_code_v0 test + nested vector null-safety

**Verdict:** PASS

**Summary:** The advance agent added a `safe_data` overload for `int32_t` vectors and applied it to
all three nested vector iteration loops (`alg_simhash`, `soft_hash_video_v0`, `gen_video_code_v0`).
Added `gen_mixed_code_v0` smoke test to the C++ test suite. All changes are minimal, correct, and
follow established patterns. 53 tests pass including ASAN.

**Verification:**

- [x] `cargo build -p iscc-ffi` succeeds
- [x] C++ test suite: 53 passed, 0 failed (52 existing + 1 new `gen_mixed_code_v0`) — verified with
    both normal and ASAN builds
- [x] `grep -c 'gen_mixed_code_v0' packages/cpp/tests/test_iscc.cpp` → 3 (≥1 required)
- [x] `grep -c 'safe_data' packages/cpp/include/iscc/iscc.hpp` → 10 (≥5 required: 2 overloads + 8
    uses)
- [x] `cargo clippy --workspace --all-targets -- -D warnings` — clean
- [x] `mise run check` — all 15 hooks pass

**Issues found:**

- (none)

**Codex review:** Codex confirmed correctness: null inner pointers for empty nested vectors now
match the Rust FFI's non-null requirement for zero-length slices. Mixed-Code smoke test is
consistent with existing API patterns. No follow-up fixes needed.

**Next:** The only remaining `normal` priority C++ item is package manager manifests (vcpkg.json,
conanfile.py). After that, only `low` priority issues remain (Swift, Kotlin, logos) which the CID
loop skips. Consider whether the package manager manifests step is worth doing or whether to signal
idle.

**Notes:** The `gen_mixed_code_v0` test correctly generates two content codes of different subtypes
(text + image) as input, which exercises the mixed-code combination path. The `safe_data` int32_t
overload mirrors the existing uint8_t pattern exactly. C++ test count is now 53 (34 numbered test
blocks, some producing multiple assertions).
