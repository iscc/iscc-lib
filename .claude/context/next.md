# Next Work Package

## Step: Create standalone C example with CMakeLists.txt (issue #23)

## Goal

Create a self-contained C example program and CMake build file in `crates/iscc-ffi/examples/` that
demonstrates the streaming ISCC-SUM use case — reading a file in chunks, feeding both DataHasher and
InstanceHasher, composing the ISCC-CODE, and printing the result. This gives C/C++ developers a
copy-pasteable starting point for integration.

## Scope

- **Create**: `crates/iscc-ffi/examples/iscc_sum.c`, `crates/iscc-ffi/examples/CMakeLists.txt`
- **Modify**: (none)
- **Reference**: `crates/iscc-ffi/include/iscc.h` (API signatures),
    `crates/iscc-ffi/tests/test_iscc.c` (existing patterns for hasher lifecycle, temp files, linking
    flags), `.claude/context/specs/c-ffi-dx.md` §2 (spec requirements)

## Not In Scope

- Adding CI steps for the example (CI already tests via `test_iscc.c`; example CI can follow later)
- Creating the `docs/howto/c-cpp.md` guide (issue #22, separate step)
- Installing cmake in the devcontainer (cmake is not available; verify CMakeLists.txt structure via
    file content inspection)
- Adding the example to any CI workflow or Makefile
- Modifying any existing source files

## Implementation Notes

**`iscc_sum.c`** — a complete program that:

1. Takes a file path as `argv[1]` (exit with usage message if missing)
2. Opens the file with `fopen(path, "rb")`
3. Creates both hashers: `iscc_data_hasher_new()` + `iscc_instance_hasher_new()`
4. Reads the file in a loop using a 4MB buffer (`iscc_io_read_size()`) — calls
    `iscc_data_hasher_update()` and `iscc_instance_hasher_update()` on each chunk
5. Finalizes both: `iscc_data_hasher_finalize(dh, 64)` and `iscc_instance_hasher_finalize(ih, 64)`
6. Composes ISCC-CODE: builds a 2-element `const char*` array `{data_code, instance_code}` and
    calls `iscc_gen_iscc_code_v0(codes, 2, false)`
7. Prints: ISCC-CODE, Data-Code, Instance-Code (one per line, labeled)
8. Frees everything: strings via `iscc_free_string()`, hashers via `_free()` (in correct order —
    finalize before free)
9. Handles errors at each step: checks NULL returns, prints `iscc_last_error()`, exits non-zero
10. Include `#include "iscc.h"` (relative — works with `-I` flag)

Use comments explaining each step. Keep it readable for a C developer who has never seen ISCC.
Follow C89/C99 compatible style (declare variables at block start) for maximum portability.

**`CMakeLists.txt`** — minimal cmake build:

```cmake
cmake_minimum_required(VERSION 3.14)
project(iscc_sum_example C)
add_executable(iscc_sum iscc_sum.c)
target_include_directories(iscc_sum PRIVATE ${CMAKE_CURRENT_SOURCE_DIR}/../include)
target_link_libraries(iscc_sum PRIVATE iscc_ffi)
```

Include a comment block at the top explaining how to build:

```
# Build against a pre-built iscc-ffi library:
#   cmake -B build -DCMAKE_PREFIX_PATH=/path/to/iscc-ffi
#   cmake --build build
#
# Build against a Cargo-built library (from repo root):
#   cargo build -p iscc-ffi --release
#   cmake -B crates/iscc-ffi/examples/build \
#         -S crates/iscc-ffi/examples \
#         -DCMAKE_LIBRARY_PATH=target/release
#   cmake --build crates/iscc-ffi/examples/build
```

Add `build/` to comments as a generated directory (no need for .gitignore — it won't be created in
CI).

**gcc verification command** (mirrors CI compilation pattern from `.github/workflows/ci.yml`):

```sh
cargo build -p iscc-ffi
gcc -o /tmp/iscc_sum_example crates/iscc-ffi/examples/iscc_sum.c \
    -I crates/iscc-ffi/include -L target/debug -liscc_ffi -lpthread -ldl -lm
echo "Hello ISCC" > /tmp/iscc_test_file.bin
LD_LIBRARY_PATH=target/debug /tmp/iscc_sum_example /tmp/iscc_test_file.bin
```

The output should show 3 labeled lines with ISCC codes (each starting with `ISCC:`).

## Verification

- `test -f crates/iscc-ffi/examples/iscc_sum.c` exits 0
- `test -f crates/iscc-ffi/examples/CMakeLists.txt` exits 0
- `grep 'iscc_data_hasher_update' crates/iscc-ffi/examples/iscc_sum.c` exits 0 (streaming pattern)
- `grep 'iscc_instance_hasher_update' crates/iscc-ffi/examples/iscc_sum.c` exits 0 (dual-hasher)
- `grep 'iscc_gen_iscc_code_v0' crates/iscc-ffi/examples/iscc_sum.c` exits 0 (composition)
- `grep 'fread' crates/iscc-ffi/examples/iscc_sum.c` exits 0 (file chunk loop)
- `grep 'cmake_minimum_required' crates/iscc-ffi/examples/CMakeLists.txt` exits 0
- `cargo build -p iscc-ffi` succeeds
- gcc compiles the example:
    `gcc -o /tmp/iscc_sum_example crates/iscc-ffi/examples/iscc_sum.c -I crates/iscc-ffi/include -L target/debug -liscc_ffi -lpthread -ldl -lm`
    exits 0
- Example runs and prints ISCC codes:
    `echo "Hello ISCC" > /tmp/iscc_test_file.bin && LD_LIBRARY_PATH=target/debug /tmp/iscc_sum_example /tmp/iscc_test_file.bin`
    prints lines containing `ISCC:`
- `cargo clippy -p iscc-ffi -- -D warnings` clean (no regressions)
- `cargo test -p iscc-lib` passes (310 tests — no regressions)

## Done When

All verification criteria pass: both files exist, the example compiles with gcc, runs against a test
file producing valid ISCC output on stdout, and existing tests/clippy remain clean.
