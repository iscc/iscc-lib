## 2026-03-07 — Review of: Create C++ header-only wrapper (iscc.hpp) with all 32 Tier 1 symbols

**Verdict:** PASS_WITH_NOTES

**Summary:** The C++17 header-only wrapper `iscc.hpp` is well-implemented with clean RAII guards,
comprehensive error handling, and all 32 Tier 1 symbols properly wrapped. The 52-assertion smoke
test covers all major wrapper patterns and passes cleanly under AddressSanitizer. Scope discipline
is perfect — only `packages/cpp/` files were added, no quality gates weakened.

**Verification:**

- [x] `cargo build -p iscc-ffi` succeeds — compiled in 1.3s
- [x] C++ test compiles and all tests pass — 52 passed, 0 failed (CMake Debug build)
- [x] `conformance_selftest()` test passes in C++ test output — first test, PASS
- [x] At least one `gen_*_v0` function test passes with expected ISCC string — `gen_meta_code_v0`
    exact match: `ISCC:AAAZXZ6OU74YAZIM`
- [x] ASAN clean — rebuilt with `-DSANITIZE_ADDRESS=ON`, all 52 tests pass without ASAN errors
- [x] `mise run check` — all 15 pre-commit hooks pass
- [x] `cargo clippy --workspace --all-targets -- -D warnings` — clean

**Issues found:**

- `gen_mixed_code_v0` has no test coverage in `test_iscc.cpp` — all other 9 gen functions are
    tested. Minor gap, not a blocker since the wrapper follows the same pattern as
    `gen_video_code_v0`
- Unused `#include <memory>` in `iscc.hpp` — fixed directly (removed, build verified)
- Nested vector marshaling doesn't protect against empty inner elements (Codex P1 finding) — see
    Codex review section. Edge case, not blocking

**Codex review:** Found 3 P1 issues, all related to the same pattern: `alg_simhash`,
`soft_hash_video_v0`, and `gen_video_code_v0` marshal nested vectors (`vector<vector<T>>`) by
extracting `.data()` pointers from inner elements. Empty inner vectors yield `nullptr` on libstdc++,
which can cause the Rust FFI to abort (UB in `slice::from_raw_parts` with null). The existing
`safe_data()` helper covers top-level byte vectors but not nested elements. This is a valid
edge-case improvement — adding a templated `safe_data<T>` or per-element null checks would fix it.
Not blocking since empty inner vectors are semantically invalid input, but should be hardened in a
future iteration.

**Next:** Add CI job for C++ tests in `ci.yml` (the `cpp` job). Then bundle `iscc.hpp` in FFI
release tarballs (`release.yml`). Consider also adding the `gen_mixed_code_v0` test and nested
vector null-safety fix. The remaining items on the C++ issue (vcpkg.json, conanfile.py, find_package
support, README, docs) can follow as separate steps.

**Notes:** The C++ issue in issues.md should remain open — only the core wrapper and smoke test are
done. CI, release bundling, package manager manifests, and documentation still need separate steps.
The `cmake` and `g++` tools were installed ad-hoc in the devcontainer — CI will need
`apt-get install cmake` or equivalent.
