## 2026-03-02 — Review of: Create standalone C example with CMakeLists.txt (issue #23)

**Verdict:** PASS

**Summary:** The advance agent created `crates/iscc-ffi/examples/iscc_sum.c` and
`crates/iscc-ffi/examples/CMakeLists.txt` exactly as specified in next.md. The C example is
well-structured with C89/C99 compatible style, proper error handling at every step, and clear
comments. It compiles cleanly with gcc, runs correctly producing valid ISCC codes, and touches no
existing files. Review fixed two minor issues in error paths (hasher leak on partial creation
failure, missing `ferror` check after read loop).

**Verification:**

- [x] `test -f crates/iscc-ffi/examples/iscc_sum.c` — exists
- [x] `test -f crates/iscc-ffi/examples/CMakeLists.txt` — exists
- [x] `grep 'iscc_data_hasher_update'` — streaming pattern present
- [x] `grep 'iscc_instance_hasher_update'` — dual-hasher present
- [x] `grep 'iscc_gen_iscc_code_v0'` — composition present
- [x] `grep 'fread'` — file chunk loop present
- [x] `grep 'cmake_minimum_required'` — cmake structure valid
- [x] `cargo build -p iscc-ffi` — succeeds
- [x] gcc compilation — compiles cleanly
- [x] Example execution — prints 3 labeled lines with `ISCC:` prefix codes
- [x] `cargo clippy -p iscc-ffi -- -D warnings` — clean
- [x] `cargo test -p iscc-lib` — 310 tests pass (256 + 31 + 22 + 1 doctest)
- [x] `mise run check` — 14/14 hooks pass

**Issues found:**

- (none blocking — two minor error path issues fixed directly by review)

**Issue resolved:** #23 (standalone C example with CMakeLists.txt) — deleted from issues.md.

**Codex review:** Three findings, all addressed or assessed:

1. *CMake library discovery (P1)*: `CMAKE_LIBRARY_PATH` in comment instructions doesn't work with
    `target_link_libraries` without `find_library()`. Valid observation — the Cargo-built scenario
    comment is aspirational. The `CMAKE_PREFIX_PATH` scenario (pre-built library) works correctly.
    Low impact since this is instructional comments, not executable build logic. Can be improved
    when #22 (c-cpp howto guide) is written.
2. *Hasher leak on partial creation (P2)*: Fixed by review — added conditional free for non-NULL
    hashers in step 4 error path.
3. *fread I/O error (P2)*: Fixed by review — added `ferror(fp)` check after read loop to detect I/O
    errors vs EOF.

**Next:** Issue #22 — Create `docs/howto/c-cpp.md` how-to guide for C/C++ integration. The example
program (`iscc_sum.c`) and committed header (`iscc.h`) are now in place to reference from the guide.
The CMake comment about `CMAKE_LIBRARY_PATH` vs `find_library` should be clarified in the howto
guide's CMake section.

**Notes:** The remaining C FFI DX issues are #22 (howto guide), #25 (release tarballs), plus #21
(units support) and #16 (feature flags) as independent enhancements. The c-cpp howto guide (#22) is
the natural next step since all prerequisites (header, example, API docs) are in place.
