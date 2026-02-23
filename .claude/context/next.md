# Next Work Package

## Step: Add C test program and CI job for iscc-ffi

## Goal

Create a minimal C test program that links against `iscc-ffi` and validates correctness of the FFI
entrypoints. This satisfies the target criterion "A C test program can call the entrypoints and get
correct results" and adds a dedicated CI job to verify the C FFI on every push.

## Scope

- **Create**: `crates/iscc-ffi/tests/test_iscc.c` (C test program)
- **Modify**: `.github/workflows/ci.yml` (add C FFI CI job)
- **Reference**: `crates/iscc-ffi/src/lib.rs` (function signatures and expected values from unit
    tests), `crates/iscc-ffi/cbindgen.toml` (header generation config), `crates/iscc-ffi/Cargo.toml`
    (crate-type: cdylib + staticlib)

## Implementation Notes

### C test program (`crates/iscc-ffi/tests/test_iscc.c`)

A self-contained C program that tests the FFI entrypoints against known expected values. Uses the
same expected values already validated in the Rust unit tests in `lib.rs`.

**Test cases to include** (values taken from the Rust unit tests):

1. `iscc_gen_meta_code_v0("Die Unendliche Geschichte", NULL, NULL, 64)` → `"ISCC:AAAZXZ6OU74YAZIM"`
2. `iscc_gen_meta_code_v0("Die Unendliche Geschichte", "Von Michael Ende", NULL, 64)` →
    `"ISCC:AAAZXZ6OU4E45RB5"`
3. `iscc_gen_text_code_v0("Hello World", 64)` → `"ISCC:EAASKDNZNYGUUF5A"`
4. `iscc_gen_image_code_v0(<1024 zero bytes>, 1024, 64)` → `"ISCC:EEAQAAAAAAAAAAAA"`
5. `iscc_gen_instance_code_v0(<empty bytes>, 0, 64)` → `"ISCC:IAA26E2JXH27TING"`
6. `iscc_gen_data_code_v0("Hello World", 11, 64)` → starts with `"ISCC:"`
7. Error handling: `iscc_gen_text_code_v0(NULL, 64)` returns `NULL`, `iscc_last_error()` returns
    non-NULL
8. Error cleared on success: after an error, a successful call makes `iscc_last_error()` return
    `NULL`
9. `iscc_free_string(NULL)` does not crash (no-op test)

**Structure pattern:**

```c
#include "iscc.h"
#include <stdio.h>
#include <string.h>
#include <stdlib.h>

static int tests_passed = 0;
static int tests_failed = 0;

#define ASSERT_STR_EQ(actual, expected, test_name) ...
#define ASSERT_NULL(ptr, test_name) ...
#define ASSERT_NOT_NULL(ptr, test_name) ...

int main(void) {
    // ... test cases ...
    printf("%d passed, %d failed\n", tests_passed, tests_failed);
    return tests_failed > 0 ? 1 : 0;
}
```

Each test should call `iscc_free_string()` on every non-NULL result. The program exits with 0 on all
tests passing, non-zero on any failure.

### CI job (`.github/workflows/ci.yml`)

Add a new `c-ffi` job (after the existing `wasm` job) that:

1. Checks out the code
2. Sets up Rust toolchain (stable) + cache
3. Installs cbindgen: `cargo install cbindgen`
4. Builds the FFI crate: `cargo build -p iscc-ffi` (builds both cdylib and staticlib)
5. Generates the C header: `cbindgen --crate iscc-ffi -o crates/iscc-ffi/tests/iscc.h`
6. Compiles the test program:
    `gcc -o test_iscc crates/iscc-ffi/tests/test_iscc.c -I crates/iscc-ffi/tests -L target/debug -liscc_ffi -lpthread -ldl -lm`
7. Runs it: `LD_LIBRARY_PATH=target/debug ./test_iscc`

**Library naming note:** On Linux, `cargo build -p iscc-ffi` produces `target/debug/libiscc_ffi.so`
(cdylib) and `target/debug/libiscc_ffi.a` (staticlib). The linker flag is `-liscc_ffi`. The cdylib
name uses underscores (Cargo converts hyphens to underscores in library names).

**System libraries:** The Rust runtime requires `-lpthread -ldl -lm` when statically linking. For
dynamic linking (cdylib), these may not be needed but including them ensures portability.

### Header generation approach

Do NOT commit the generated header. Generate it in CI via `cbindgen`. This avoids staleness issues
and is the standard approach for cbindgen-based projects. The test program includes the generated
header from the same directory.

## Verification

- The C test program compiles with gcc against the cbindgen-generated header
- The C test program runs successfully (exit code 0) and prints all tests passing
- All existing CI jobs remain green (no regression)
- The new `c-ffi` CI job passes
- At least 7 test cases cover: meta, text, image, instance, data, error handling, and memory freeing

## Done When

The advance agent is done when the C test program exists, compiles, runs with all tests passing
locally, and the CI workflow includes a `c-ffi` job that builds, compiles, and runs the C test.
