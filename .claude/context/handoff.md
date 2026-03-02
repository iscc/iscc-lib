## 2026-03-02 — Create standalone C example with CMakeLists.txt (issue #23)

**Done:** Created `crates/iscc-ffi/examples/iscc_sum.c` — a complete C program that reads a file in
chunks, feeds both DataHasher and InstanceHasher, composes the ISCC-CODE, and prints all three
codes. Also created `crates/iscc-ffi/examples/CMakeLists.txt` with build instructions for both
pre-built and Cargo-built library scenarios.

**Files changed:**

- `crates/iscc-ffi/examples/iscc_sum.c`: New C example program demonstrating streaming ISCC-SUM
- `crates/iscc-ffi/examples/CMakeLists.txt`: Minimal CMake build file with usage instructions

**Verification:**

- `test -f crates/iscc-ffi/examples/iscc_sum.c` — PASS
- `test -f crates/iscc-ffi/examples/CMakeLists.txt` — PASS
- `grep 'iscc_data_hasher_update'` — PASS (streaming pattern present)
- `grep 'iscc_instance_hasher_update'` — PASS (dual-hasher present)
- `grep 'iscc_gen_iscc_code_v0'` — PASS (composition present)
- `grep 'fread'` — PASS (file chunk loop present)
- `grep 'cmake_minimum_required'` — PASS
- `cargo build -p iscc-ffi` — PASS
- gcc compilation — PASS (compiled with `-I include -L target/debug -liscc_ffi -lpthread -ldl -lm`)
- Example execution — PASS (prints 3 labeled lines with `ISCC:` prefix codes)
- `cargo clippy -p iscc-ffi -- -D warnings` — PASS (clean)
- `cargo test -p iscc-lib` — PASS (310 tests: 256 + 31 + 22 + 1 doctest)
- `mise run check` — PASS (14/14 hooks)

**Next:** Issue #22 — Create the `docs/howto/c-cpp.md` how-to guide for C/C++ integration. The
example program and committed header are now in place to reference from the guide.

**Notes:** The example uses C89/C99 compatible style (variables declared at block start) for maximum
portability. Error handling covers every fallible step with proper cleanup. The CMakeLists.txt
includes comment blocks explaining both pre-built and Cargo-built library build workflows.
