# Next Work Package

## Step: .NET and C++ wrappers reject string inputs with an embedded NUL

## Goal

Fix the `normal` issue ".NET and C++ wrappers silently truncate string inputs at an embedded NUL"
using the approach Titusz chose (decisions.md 2026-11-19): each wrapper rejects a NUL-bearing string
before marshalling, so it never returns a prefix's ISCC as a success. .NET throws `IsccException`,
which matches the .NET howto's "methods throw `IsccException` on invalid input" and mirrors C++
`IsccError`.

## Alternatives Considered

- **Chosen:** NUL rejection in .NET and C++. Today's decision unblocked it, and it fixes a
    silent-wrong-result bug: the worst failure class among the 10 open normals.
- **Rejected:** `@iscc/wasm` license/description metadata. It is tiny and only takes effect at the
    next release, so it costs nothing to do next. The Ruby negative-Bignum fix also waits: it is
    another wrong-result bug, but on a much narrower input range.

## Scope

- **Modify**: `packages/dotnet/Iscc.Lib/IsccLib.cs`, `packages/cpp/include/iscc/iscc.hpp`,
    `docs/howto/dotnet.md`, `docs/howto/c-cpp.md`
- **Tests (outside budget)**: `packages/dotnet/Iscc.Lib.Tests/SmokeTests.cs` (or a new
    `NulInputTests.cs`), `packages/cpp/tests/test_iscc.cpp`
- **Reference**: the issue entry in issues.md (it has the probe inputs and the expected codes),
    `packages/dotnet/CLAUDE.md`, the existing C++ `encode_seq` NUL guard (`iscc.hpp` ~L364-375) and
    its test #43 in `test_iscc.cpp`

## Not In Scope

- Any change to `crates/iscc-ffi`, `iscc.h` or `NativeMethods.g.cs`. The C ABI stays NUL-terminated.
- The .NET pin-loop dedup and the `[]`/null handling in `GenMixedCodeV0`/`GenIsccCodeV0`. Both
    belong to a separate issue, so do not touch the pin loops beyond what the NUL check needs.
- `docs/c-ffi-api.md` type names, the JNI/Ruby/UniFFI surfaces (these keep the NUL, and the core
    rejects it), and any length-carrying FFI redesign (rejected in the decision).

## Implementation Notes

- **.NET:** `ToNativeUtf8` is the only string-marshalling path in `Iscc.Lib` (checked: no other
    `GetBytes`/`StringToCoTaskMemUTF8` in non-generated files). Add the check there:
    `s.Contains('\0')` → throw `IsccException` with a clear message, e.g. "string input contains an
    embedded NUL character". The three array methods already convert every element before the first
    `GCHandle.Alloc` (the loop before `try`), so a throw cannot leak pinned handles. Keep it that
    way. Update the `<exception>` XML docs where a method already lists them (`EncodeSeq`).
- **C++:** add one `detail` helper that throws `IsccError` if the string contains `'\0'` and returns
    `c_str()`, e.g. `inline const char* checked_c_str(const std::string& s)`. Route every
    `std::string` → C call through it, including the `std::optional` description/meta in
    `gen_meta_code_v0` (`->c_str()`) and the element loops in `gen_mixed_code_v0`,
    `gen_iscc_code_v0` and `gen_sum_code_v0`'s `path`. `encode_seq` keeps its own guard and its
    "ISCC-UNIT at index i" message, because test #43 pins `"index 1"`.
- **C++ test literals:** `std::string("Hello\0World")` stops at the NUL in the constructor. Use
    `std::string("Hello\0World", 11)` or `s + '\0' + "junk"`. Use the existing `assert_throws_with`
    with a needle taken from the helper's message.
- **Test matrix (per wrapper):** at least one gen function (`gen_text_code_v0`), one text function
    (`text_clean`), one codec function (`iscc_decompose` or `iscc_decode`), `encode_seq`, one
    optional argument (`gen_meta_code_v0` with a NUL in `description`) and one array element
    (`gen_mixed_code_v0` or `gen_iscc_code_v0`). Add one positive control showing that NUL-free
    input still returns the documented code.
- **Docs:** in the "Error handling" sections of `docs/howto/dotnet.md` and `docs/howto/c-cpp.md`
    (C++ part), add a sentence saying that string inputs with an embedded NUL (`\0`) are rejected
    rather than truncated. Use the words "embedded NUL".
- Binding-only step, so neither the CI-only CRAP gate nor the iai Perf gate is affected.
- Tooling: `dotnet` is on `$PATH`. Run cmake as `uv run --with cmake cmake`.

## Verification

- `cargo build -p iscc-ffi && dotnet test packages/dotnet/Iscc.Lib.Tests/ -e LD_LIBRARY_PATH=$PWD/target/debug`
    passes, including the new NUL tests
- In `packages/cpp`,
    `uv run --with cmake cmake -B build -DCMAKE_BUILD_TYPE=Debug -DFFI_LIB_DIR=../../target/debug -DSANITIZE_ADDRESS=ON`,
    then `uv run --with cmake cmake --build build`, then
    `LD_LIBRARY_PATH=../../target/debug ./build/tests/test_iscc` exits 0 with no ASAN report
- Every line that `grep -n 'c_str()' packages/cpp/include/iscc/iscc.hpp` prints is either inside the
    NUL-check helper or in `encode_seq` right after its own NUL guard
- `grep -n 'embedded NUL' docs/howto/dotnet.md docs/howto/c-cpp.md` matches in both files
- `mise run check` exits 0

## Done When

All five verification criteria pass, and a NUL-bearing string reaching any .NET or C++ string input
raises `IsccException` / `IsccError` instead of returning a result.
