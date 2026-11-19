## 2026-11-19 — Review of: Bind `encode_seq` / `decode_seq` in the C++ header-only wrapper

**Verdict:** PASS

**Summary:** `iscc::encode_seq` / `iscc::decode_seq` landed in `iscc.hpp` with RAII ownership
(`UniqueByteBuffer` / `UniqueStringArray` + `check_ptr`), an empty-array sentinel so the core's "at
least one ISCC-UNIT" error is the one that comes through, and an embedded-NUL guard that throws
before marshalling. ISCC-SEQ is now on 7 of 11 surfaces. The tests hard-code data.json bytes, check
the byte round trip in the correct direction (`encode_seq(decode_seq(x)) == x`), and use one bad
input per negative case. Scope was exactly the two files next.md named.

**Verification:**

- [x] `cargo build -p iscc-ffi` + `cmake -B build-seq … -DSANITIZE_ADDRESS=ON` + `cmake --build`
    exit 0. There were no compiler warnings, only bind-mount clock-skew notices from gmake. The
    CMake target sets no `-Wall`, so I also compiled `test_iscc.cpp` with
    `g++ -Wall -Wextra -Wpedantic -Wshadow -Wconversion -Werror`, and it was clean
- [x] `LD_LIBRARY_PATH=../../target/debug ./build-seq/tests/test_iscc` exits 0: 65 passed, 0 failed,
    no ASAN report. It prints PASS lines for the encode_seq vector, the decode_seq units, the
    test_0004 byte round trip and all four negative cases
- [x] `iscc.hpp:366` declares `std::vector<uint8_t> encode_seq(const std::vector<std::string>&)` and
    `iscc.hpp:384` declares `std::vector<std::string> decode_seq(const std::vector<uint8_t>&)`
- [x] Test 42 asserts that `encode_seq({})` throws `IsccError` containing "at least one ISCC-UNIT".
    Test 43 asserts that a unit with an embedded `'\0'` throws (needle "index 1")
- [x] `git status --porcelain crates/iscc-ffi packages/dotnet` is empty
- [x] `mise run check` exits 0
- [x] Extra probes: (1) the doc comment's claim holds — a lowercase unit, a hyphenated `iscc:` unit
    and a bare unit all give the same bytes; non-UTF-8 and `""` units give clean `IsccError`s
    (ASAN-built probe). (2) Advance's report that C++ also truncates at an embedded NUL reproduces:
    `gen_text_code_v0("Hello\0World")` gives `"Hello"`'s code, and
    `iscc_decompose(code + '\0' + "junk")` returns 4 units without throwing. I widened the NUL issue
    to cover C++
- [x] The unpushed range (update-state, define-next, advance) has no lint suppressions, skips or
    hook changes

**Issues found:**

- The embedded-NUL truncation now spans .NET and C++ (15 unguarded `c_str()` sites). I rewrote the
    issue to cover both wrappers, including the new disagreement between surfaces: C++ `encode_seq`
    rejects `unit + "\0junk"` while .NET `EncodeSeq` accepts the prefix
- Stale C++ docs are deferred to the sweep as next.md planned, so nothing was filed. They are listed
    under **Next**
- (no defects in the diff itself)

**Codex review:** clean, with no findings. It confirmed the empty-input error is preserved, the NUL
check runs before truncation could happen, the RAII release is correct, and the suite passed with
warnings treated as errors.

**Next:** JNI seq (`crates/iscc-jni` + the Java wrapper and its tests), then Ruby, then UniFFI
(Swift/Kotlin, which hard-code per-section conformance counts). Ask each remaining surface for a
`unit + "\0junk"` negative test, so the cross-surface NUL behaviour is pinned as the fan-out
proceeds. Those three bind the core directly rather than through the C FFI, so they should reject it
the way iscc-core does. After all 11 surfaces, the 33→35 doc sweep must cover:

- **C++:** `packages/cpp/CLAUDE.md` ("35+ test cases" twice; the type map has no
    `const std::vector<std::string>&` → `const char* const*` row with its sentinel `safe_data`
    overload), `packages/cpp/README.md`, `docs/howto/c-cpp.md`
- **.NET:** `packages/dotnet/CLAUDE.md` (still says "all 9 gen functions (50 vectors total)", and
    `SeqTests.cs` is missing from File Layout and Test Patterns), `packages/dotnet/README.md`,
    `docs/howto/dotnet.md`
- **Everywhere else:** the crate `CLAUDE.md` files, `notes/00`, `notes/04`, `docs/architecture.md`
    and `docs/c-ffi-api.md`, which has no seq entries, plus its open type-name issue

**Notes:**

- **Pending human decision, not escalated yet:** the NUL issue's fix (reject NUL at each wrapper's
    choke point, or switch to a length-delimited FFI ABI) is Titusz's call. I did not set the HUMAN
    REVIEW marker, because it pauses the loop and four mechanical seq surfaces remain. If the
    fan-out finishes and this is still the blocking `normal` issue, escalate it then
- Binding-only step: no core, FFI or hot-path change, so iai and CRAP are unaffected
- A `/tmp` C++ probe needs only
    `g++ -std=c++17 -fsanitize=address -Ipackages/cpp/include -Icrates/iscc-ffi/include x.cpp -Ltarget/debug -liscc_ffi`
    and then `LD_LIBRARY_PATH=target/debug`. It is quicker than cmake for one-off checks
