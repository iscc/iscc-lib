## 2026-11-20 — data-edge.json fan-out, group 3 — C and C++ harnesses via one generated header

**Done:** Recovered and adapted the `ea69169` generator as `scripts/gen_data_edge_vectors.py`. It
renders `data-edge.json` into `crates/iscc-ffi/tests/data_edge_vectors.h` (C99 + C++17, positional
initialisers, explicit-length strings, a resolved `c`/`cpp` expectation per row, per-section and
total count macros). A pytest gate fails on a stale header. The C test and the C++ test now run
every vector through one check function per section plus a loop, a total-row tally and negative
tests. Three commits, one per surface: `09fde2e` generator, `021e917` C, `9c19587` C++.

**Files changed:**

- `scripts/gen_data_edge_vectors.py` (new): PEP 723, octal escape rule, ASCII/LF output; fails
    closed on an unknown/empty/missing section, bad arity or type, and a bad `except` key or value
- `crates/iscc-ffi/tests/data_edge_vectors.h` (new, generated)
- `tests/test_gen_data_edge_vectors.py` (new, 13 tests): fresh render == tracked bytes; ASCII/LF/no
    `\x`; count macros derived from the fixture; each name rendered once; a mutated expectation
    changes the render; unknown section / bad arity / bad `except` key / empty section raise
    `SystemExit`; the NUL text row renders c=skip, cpp=error; the empty-list sentinel
- `crates/iscc-ffi/tests/test_iscc.c`: `check_edge_*` per section, `RUN_EDGE_SECTION` loop (uses
    `ASSERT_EQ`, prints `SKIP:` for skip rows), tally vs `ISCC_EDGE_TOTAL_COUNT`, 3 negatives
    (accepted rejection, `iscc` mismatch, embedded NUL: `strlen != len`, whose expected `iscc` is
    the hash of `"Hello"`, so only the length check catches it)
- `packages/cpp/tests/test_iscc.cpp`: `check_edge_*` (only `iscc::IsccError` counts as a rejection;
    any other exception fails), `run_edge_section` template, tally, 2 negatives
- `packages/cpp/tests/CMakeLists.txt`: PRIVATE `target_include_directories` on `test_iscc` only
- `crates/iscc-ffi/CLAUDE.md`, `packages/cpp/CLAUDE.md`: name the header, the loop and the
    regenerate command, with no vector counts

**Verification:**

- `uv run pytest tests/test_gen_data_edge_vectors.py tests/test_data_edge.py tests/test_vendored_fixtures.py`:
    100 passed. `mise run check` exit 0 (pre-commit hooks only). I also ran
    `prek run --hook-stage pre-push --files <both .py>`: clippy, cargo test, ty, Ruff S/C901 and
    pytest all Passed
- Regenerating with `uv run --script scripts/gen_data_edge_vectors.py` leaves
    `git status --porcelain crates/iscc-ffi/tests` empty
- The no-vector-name `grep -nE …` over both harnesses prints nothing (exit 1)
- C (ci.yml gcc line after `cargo build -p iscc-ffi`): exit 0, 96 passed. A script derived from the
    fixture found all 14 non-skip `PASS: data_edge/…` lines and 3 negative PASS lines. 2 SKIP lines
    are the two `c: skip` rows
- C++ (`/tmp/cpp-edge`, ci.yml flags, absolute `FFI_LIB_DIR`, ASAN on): exit 0, 95 passed, 0
    `AddressSanitizer` lines, all 16 `PASS: data_edge/…` lines and 2 negative PASS lines
- Probes with a real regression, using a header rendered from a mutated fixture copy:
    - C: dropping `except.c` from the NUL text row → that row FAILs. A wrong `encode_seq` output →
        FAIL. Deleting the `gen_iscc_code_v0` loop → tally FAIL "got 15, expected 16", exit 1
    - C++: dropping `except.cpp` → FAIL. A wrong decode maintype → FAIL, exit 1
- `gcc -std=c99 -Wall -Wextra -pedantic` and `g++ -std=c++17 -Wall -Wextra -pedantic` syntax checks
    print no warnings
- Caveat: only Linux gcc/g++ ran here, so the macOS/Windows compilers have not seen the header

**Next:** v0.8.0 "Shared edge-case vectors run everywhere" should now be met on all 12 surfaces.
update-state should confirm the CI `c-ffi` and `cpp` jobs are green on the pushed sha. Cheap
`normal` follow-up: the Python `encode_seq` docstring issue.

**Notes:**

- Required by next.md: the C++ `gen_mixed_code_v0`/`gen_iscc_code_v0` wrappers pass `ptrs.data()`
    (NULL for an empty vector), while `encode_seq` uses `detail::safe_data`. The empty-list rows
    pass anyway, because the FFI NULL guard rejects the input before the core's count check. Not
    fixed (out of scope)
- Deviations from next.md:
    - `escape_c_bytes` also escapes `?` as `\?` so no trigraph can form. `\?` is valid in both C99 and
        C++17, and a test pins it
    - The generator also fails closed on an empty/missing section (no zero-length arrays) and on input
        type mismatches
    - It keeps its own `SURFACE_IDS` copy, a second list next to `tests/test_data_edge.py`. Drift
        fails closed, but both lists need maintaining
- `iscc-ffi` is `publish = false`, so the header never ships (`cargo package --list` would include
    `tests/`, but the crate is never published)
- `mise run check` runs only the pre-commit stage. The pre-push gates need
    `prek run --hook-stage pre-push`
