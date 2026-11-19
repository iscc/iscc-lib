## 2026-11-19 — .NET and C++ wrappers reject string inputs with an embedded NUL

**Done:** Both wrappers now reject a NUL-bearing string before marshalling. .NET: `ToNativeUtf8`
throws `IsccException("string input contains an embedded NUL character")`. C++: a new
`detail::checked_c_str()` throws `IsccError` with the same message and is the only path from
`std::string` to `const char*`, apart from `encode_seq`, which keeps its own indexed guard. Both
howto pages now document the restriction.

**Files changed:**

- `packages/cpp/include/iscc/iscc.hpp`: added `detail::checked_c_str`. 17 call sites route through
    it: text_clean/remove_newlines/trim/collapse, json_to_data_url, iscc_decode, iscc_decompose,
    sliding_window, gen_meta_code_v0 (name + optional description/meta), gen_text_code_v0, the
    gen_mixed/gen_iscc element loops, and the gen_sum_code_v0 path
- `packages/dotnet/Iscc.Lib/IsccLib.cs`: NUL check in `ToNativeUtf8` (with `<remarks>`/`<exception>`
    docs). The `EncodeSeq` `<exception>` doc now lists the embedded NUL case. The array methods
    still convert every element before the first `GCHandle.Alloc`, so the throw cannot leak a pin
- `packages/dotnet/Iscc.Lib.Tests/NulInputTests.cs` (new): 9 facts, 1 positive control
    (`GenTextCodeV0("Hello World")` == `ISCC:EAASKDNZNYGUUF5A`) + 8 NUL negatives (GenTextCodeV0,
    TextClean, IsccDecompose, IsccDecode, EncodeSeq, GenMetaCodeV0 description, GenIsccCodeV0
    element, GenMixedCodeV0 element). Each asserts `IsccException` with "embedded NUL" in the
    message
- `packages/cpp/tests/test_iscc.cpp`: test block 46 with a literal-length sanity check, a positive
    control, and 9 negatives (the same 8 as .NET plus a gen_sum_code_v0 path). They use
    `assert_throws_with(..., "embedded NUL", ...)` and `std::string("Hello\0World", 11)`-style
    literals
- `docs/howto/dotnet.md`, `docs/howto/c-cpp.md` (C++ "Error handling"): one sentence each with the
    words "embedded NUL"
- `packages/cpp/CLAUDE.md`, `packages/dotnet/CLAUDE.md`: these are outside the Scope list. I updated
    the type-mapping rows (they said `.c_str()` at call site), the string-marshalling bullet, and
    the test-file layout entry, so the agent guidance matches the code

**Verification:**

- `cargo build -p iscc-ffi && dotnet test packages/dotnet/Iscc.Lib.Tests/ -e LD_LIBRARY_PATH=$PWD/target/debug`:
    exit 0, `Passed: 125, Failed: 0`. That total includes the 9 new facts
- C++ ASAN build (`uv run --with cmake cmake -B build ... -DSANITIZE_ADDRESS=ON`, then `--build`,
    then `LD_LIBRARY_PATH=../../target/debug ./build/tests/test_iscc`): exit 0,
    `76 passed, 0 failed`, no ASAN report in the output
- Non-vacuity probe (C++): I compiled the new test file with g++ against the HEAD (pre-fix)
    `iscc.hpp` in `/tmp/nulprobe`. Result: `68 passed, 8 failed`. Every new negative fails except
    `encode_seq`, which was already guarded. gen_text, text_clean, decompose, decode, meta and mixed
    report "nothing thrown". gen_iscc and gen_sum fail with the wrong message, because the truncated
    input hits a different core error. I did not run the same probe for .NET, but the issue entry
    documents the prefix successes on .NET
- `grep -n 'c_str()' packages/cpp/include/iscc/iscc.hpp` prints line 68 (`return s.c_str();` inside
    `checked_c_str`) and line 384 (`encode_seq`, after its own NUL guard). Nothing else
- `grep -n 'embedded NUL' docs/howto/dotnet.md docs/howto/c-cpp.md` matches dotnet.md:447 and
    c-cpp.md:428
- `mise run check`: exit 0, run after `mise run format`

**Next:** Two remaining `normal` items are small. One is the `@iscc/wasm` license/description
metadata. The other is the Ruby negative-Bignum truncation in `iscc-rb`. The .NET pin-loop dedup
issue (3 copies, NULL for `[]`) touches the same `IsccLib.cs` region I just edited, so it would be a
natural follow-up.

**Notes:**

- The C++ `encode_seq` message ("ISCC-UNIT at index i contains an embedded NUL character") also
    contains "embedded NUL", so a single needle works for every C++ string input
- The .NET message carries no parameter name or index: `ToNativeUtf8` is shared and does not know
    them. .NET `EncodeSeq` therefore does not say which unit was bad, unlike C++. I accepted this to
    keep the check in one place, as next.md prescribed
- In C++ `gen_meta_code_v0`, `description`/`meta` are checked before `name`. Each string is checked
    on its own, so the order only decides which input gets reported when several contain a NUL
- Binding-only change. No hot path, CRAP or iai gate is affected
