## 2026-11-18 — Review of: Bind `EncodeSeq` / `DecodeSeq` in the .NET wrapper

**Verdict:** PASS_WITH_NOTES

**Summary:** `IsccLib.EncodeSeq(string[])` and `IsccLib.DecodeSeq(ReadOnlySpan<byte>)` wrap the
existing P/Invoke declarations correctly, and the generated files are untouched. They cover the null
and empty-array guards, use the stack sentinel that lets the core's empty-list message through, and
free native memory through the `Consume*` helpers. The harness runs both data.json seq sections plus
the byte round trip `EncodeSeq(DecodeSeq(x)) == x`. ISCC-SEQ now covers 6 of 11 surfaces. The review
found one pre-existing problem that affects the whole wrapper (embedded NUL, below), filed as an
issue rather than blocking.

**Verification:**

- [x] `cargo build -p iscc-ffi && dotnet restore … --locked-mode && dotnet build … --no-restore`:
    Build succeeded, 0 Warning(s), 0 Error(s)
- [x] `dotnet test … --no-restore -e LD_LIBRARY_PATH=$PWD/target/debug`: exit 0, 116/116 passed. The
    detailed log lists `ConformanceTests.EncodeSeq` ×4, `DecodeSeq` ×4, `SeqRoundTripBytes` ×4 and
    10 `SeqTests`. Per-class counts: 50 `ConformanceTests.Gen*` rows (no rows lost), 44
    `SmokeTests`, 22 seq tests
- [x] `IsccLib.cs` declares `public static byte[] EncodeSeq(string[] units)` (L488) and
    `public static string[] DecodeSeq(ReadOnlySpan<byte> data)` (L542)
- [x] `EncodeSeq([])` throws `IsccException` containing "at least one ISCC-UNIT", asserted by
    `SeqTests.EncodeSeq_Empty_ThrowsAtLeastOneUnit`
- [x] `git status --porcelain packages/dotnet/Iscc.Lib/NativeMethods.g.cs crates/iscc-ffi` is empty
    after the build
- [x] `mise run check`: exit 0, no files rewritten
- [x] Extra: the unpushed range (define-next, update-state, advance) contains no lint suppressions
    or skipped tests. All five tracked `data.json` copies are identical and have 4+4 seq vectors

**Issues found:**

- **Embedded NUL truncates every .NET string input** (filed, `normal`). Probed:
    `EncodeSeq([unit + "\0junk"])` and `IsccDecompose(code + "\0junk")` succeed using only the text
    before the NUL; Python raises "base32 decode error". `GenTextCodeV0("Hello\0World")` returns the
    code for `"Hello"`, `ISCC:EAAWSPUDF7GJWQVD`, while Python returns `ISCC:EAASKDNZNYGUUF5A`.
    Cause: `ToNativeUtf8` keeps the `\0`, and the FFI's `CStr` stops reading there. This step
    followed the prescribed pattern, so it is not a regression
- **Three copies of the string-array pin loop**, plus NULL-on-empty in `GenIsccCodeV0` /
    `GenMixedCodeV0` (filed, `normal`). next.md deferred this on purpose; the entry keeps it from
    getting lost
- Nit: `ConsumeByteBuffer` casts `buf.len` to `int` without a check, so an output over 2 GiB would
    be cut short silently. That would take hundreds of millions of units, and the helper is shared
    and pre-existing. Not filed

**Codex review:** reviewed the main commit `e79a035`, since HEAD only holds handoff text. Two
findings. P2 (embedded NUL makes `EncodeSeq` encode only the text before the NUL) is real; probing
showed the whole wrapper is affected, filed as above. P3 (unchecked `nuint`→`int` length cast in
`ConsumeByteBuffer`) is valid in theory but needs an output of several GiB; noted above, not filed.

**Next:** C++ `iscc.hpp` seq, as planned. It needs RAII ownership of the returned `IsccByteBuffer`
and string array, the byte round trip over the `decode_seq` vectors, and one bad input per negative
test. **Watch:** `iscc.hpp` passes `std::string::c_str()` to the FFI at 15 sites, so it probably has
the same embedded-NUL truncation (unprobed). Either probe `unit + '\0' + "junk"` in that step and
widen the NUL issue, or sequence the NUL fix first so C++ seq does not copy the problem. Then JNI,
Ruby and UniFFI seq, then the 33→35 doc sweep. The sweep includes `packages/dotnet/CLAUDE.md`, which
still says "all 9 gen functions (50 vectors total)" and does not list `SeqTests.cs` in File Layout
or Test Patterns, plus `packages/dotnet/README.md` and `docs/howto/dotnet.md`.

**Notes:** binding-only step: no core, FFI or hot-path changes, so iai and CRAP are unaffected.
`maturin develop --release` was rebuilt before the push. A `/tmp` console project with a
`ProjectReference` to `Iscc.Lib.csproj` makes a cheap .NET probe harness; run its dll with
`LD_LIBRARY_PATH=target/debug`.
