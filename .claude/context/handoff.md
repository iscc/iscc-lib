## 2026-11-19 — Review of: .NET and C++ wrappers reject string inputs with an embedded NUL

**Verdict:** PASS

**Summary:** Both wrappers now reject a string containing `\0` before it reaches the C FFI. In .NET
the check sits in `ToNativeUtf8`, which is the only string-marshalling path (grepped every
non-generated `Iscc.Lib/*.cs`). In C++ the new `detail::checked_c_str` is now the only route from
`std::string` to a C call, apart from `encode_seq`, which has its own guard. Tests cover every case
next.md listed, plus `gen_sum_code_v0`'s path in C++. The issue is resolved and deleted from
issues.md.

**Verification:**

- [x] `cargo build -p iscc-ffi && dotnet test …`: 125/125 passed, including all 9 `NulInputTests`.
    The first run hit "apphost not found" because the background Codex run was building in the same
    `obj/`; the re-run was clean
- [x] C++ ASAN build (fresh `build/`) + `test_iscc`: exit 0, `76 passed, 0 failed`, no sanitizer
    report. Re-run after my line wrap: same result
- [x] `grep -n 'c_str()' iscc.hpp`: only L68 (inside `checked_c_str`) and L384 (`encode_seq`, right
    after its NUL guard)
- [x] `grep -n 'embedded NUL'`: matches `docs/howto/dotnet.md:447` and `docs/howto/c-cpp.md:428`
- [x] `mise run check`: exit 0, and no formatter rewrites
- Extra checks: a strict `g++ -std=c++17 -Wall -Wextra -Wpedantic -Werror -fsyntax-only` pass over
    `test_iscc.cpp` is clean. The .NET negatives cannot pass by accident: the new `ToNativeUtf8`
    check is the only code that produces an `IsccException` containing "embedded NUL", so all 8 fail
    against the old file. That means the pre-fix .NET run the advance agent skipped is not needed

**Issues found:**

- Minor fix (mine): wrapped the two new `iscc.hpp` lines that ran to 105 columns (`gen_meta_code_v0`
    and `gen_sum_code_v0`), so they fit in 100 like the rest of the header. No formatter config
    enforces this; it was my choice, and I rebuilt and re-ran the tests afterwards
- `packages/cpp/CLAUDE.md` and `packages/dotnet/CLAUDE.md` were edited although they are not in
    Scope. I accepted this: the old type-mapping row told agents to call `.c_str()` at the call
    site, which is exactly the bug pattern
- The .NET message names no parameter or index, while C++ `encode_seq` says "ISCC-UNIT at index i".
    This follows from having one shared check, as next.md prescribed. Noted, not filed
- The advance memory's new NUL line stops mid-sentence ("route new inputs"). That file belongs to
    the advance agent, so I did not edit it. learnings.md now states the full rule

**Codex review:** Main commit `8f6ba5d`: clean, no findings (it also reproduced the 76 C++ and 125
.NET passes). HEAD `7ba4763`: one P3, the truncated advance memory line above. Not actionable for
review.

**Next:** Nine `normal` issues remain. These are small and independent:

- `@iscc/wasm` license/description metadata (`crates/iscc-wasm/Cargo.toml` + the release patch step)
- Dependabot `soupsieve`/`setuptools` lock refresh (security alerts, dev-only)
- Ruby negative-Bignum truncation (another wrong-result bug)
- Python `encode_seq` docstrings (two docstrings)

The .NET pin-loop dedup issue touches the same `IsccLib.cs` region that was just edited.

Gate maintenance for define-next to weigh: nothing stops a future wrapper function from using a bare
`.c_str()` (C++) or calling `Encoding.UTF8.GetBytes` directly (.NET), which would bring the
truncation back. A small structural test pinning both helpers as the only marshalling paths would
catch it.

**Notes:** Binding-only change, so the CRAP, iai and semver gates are not affected. The C ABI stays
NUL-terminated (decisions.md 2026-11-19). No new decision entry: 2026-11-19 already covers this
change.
