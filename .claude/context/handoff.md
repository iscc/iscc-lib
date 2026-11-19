## 2026-11-19 — Bind `encode_seq` / `decode_seq` in the C++ header-only wrapper

**Done:** I added `iscc::encode_seq(const std::vector<std::string>&) -> std::vector<uint8_t>` and
`iscc::decode_seq(const std::vector<uint8_t>&) -> std::vector<std::string>` to `iscc.hpp`, in the
codec section after `iscc_decompose`. They use `UniqueByteBuffer` / `UniqueStringArray` RAII plus
`check_ptr`. `encode_seq` throws `IsccError` ("ISCC-UNIT at index N contains an embedded NUL
character") before marshalling any unit that contains `'\0'`. It passes a non-null sentinel array
when `units` is empty, so the core's "at least one ISCC-UNIT" error reaches the caller. ISCC-SEQ now
covers 7 of 11 surfaces.

**Files changed:**

- `packages/cpp/include/iscc/iscc.hpp`: new `detail::safe_data(const std::vector<const char*>&)`
    overload (returns a static `nullptr` slot for empty vectors), `encode_seq` and `decode_seq`
- `packages/cpp/tests/test_iscc.cpp`: `from_hex` and `assert_throws_with` (message substring)
    helpers. Tests 39-45: the encode_seq `test_0001_four_units` full bytes, the decode_seq
    `test_0001` units, the decode_seq `test_0004` byte round trip `encode_seq(decode_seq(x)) == x`,
    and one bad input per negative test: empty encode, embedded NUL, empty decode, truncated decode

**Verification:**

- `cargo build -p iscc-ffi`: OK. In `packages/cpp`,
    `uv run --with cmake cmake -B build-seq …   -DSANITIZE_ADDRESS=ON` gave cfg=0 and
    `cmake --build build-seq` gave exit 0, with no compiler output beyond the progress lines. The
    CMake test target sets no `-Wall`, so I also ran
    `g++ -std=c++17 -Wall -Wextra -Wpedantic -Werror -fsyntax-only` on `test_iscc.cpp` (which
    includes `iscc.hpp`). Result: clean
- `LD_LIBRARY_PATH=../../target/debug ./build-seq/tests/test_iscc`: exit 0, "65 passed, 0 failed",
    no FAIL lines, no ASAN report. PASS lines: `encode_seq(test_0001_four_units) bytes`,
    `decode_seq(test_0001_four_units) units`, `encode_seq(decode_seq(test_0004)) byte round trip`,
    `encode_seq(empty) throws 'at least one ISCC-UNIT'`,
    `encode_seq(unit with embedded NUL) throws`, `decode_seq(empty) throws 'Empty ISCC-SEQ'`,
    `decode_seq(truncated) throws 'Truncated ISCC-UNIT'`
- Mutation probes (header copied to /tmp, test rebuilt against the copy):
    - NUL guard disabled:
        `FAIL: encode_seq(unit with embedded NUL) throws — expected IsccError,   nothing thrown`
    - `safe_data(ptrs)` replaced by `ptrs.data()`:
        `FAIL: encode_seq(empty) … message "units must   not be NULL" lacks "at least one ISCC-UNIT"`
    - So both guards are pinned by a test
- `git status --porcelain crates/iscc-ffi packages/dotnet`: empty
- `mise run check`: exit 0. `mise run format` rewrote nothing

**Next:** continue the seq fan-out: JNI (`crates/iscc-jni`), then Ruby, then UniFFI (Swift/Kotlin).
Then the 33→35 doc sweep, which must cover `packages/cpp/CLAUDE.md` ("35+ test cases", the type map
has no `std::vector<const char*>` sentinel row), `packages/cpp/README.md` and `docs/howto/c-cpp.md`.
It also covers the .NET doc items from the last review.

**Notes:**

- **NUL probe (report-only, widen the NUL issue to C++):** existing C++ `c_str()` sites truncate at
    an embedded NUL, the same as .NET. `iscc::gen_text_code_v0(std::string("Hello\0World", 11))` →
    `ISCC:EAAWSPUDF7GJWQVD`, identical to `gen_text_code_v0("Hello")`.
    `iscc::iscc_decompose(code + '\0' + "junk")` returns 2 units instead of throwing. Probe source:
    `/tmp/cppprobe/probe.cpp`, not committed. I did not change these sites (out of scope)
- The `units` sentinel is a `detail::safe_data` overload rather than a local static, which matches
    the two existing overloads. `decode_seq` reuses the existing `uint8_t` `safe_data`. The FFI
    checks `len == 0` first, so the core's "Empty ISCC-SEQ" error comes through
- No core, FFI or hot-path change, so iai and CRAP are unaffected. `iscc.h` and `NativeMethods.g.cs`
    are untouched
