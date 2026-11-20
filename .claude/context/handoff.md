## 2026-11-20 — .NET — one private helper pins every jagged-array input; empty inputs reach the core

**Done:** A private nested `PinnedRows : IDisposable` in `IsccLib.cs` now pins inner arrays for all
six jagged-array methods (`GenVideoCodeV0`, `SoftHashVideoV0`, `AlgSimhash`, `GenMixedCodeV0`,
`GenIsccCodeV0`, `EncodeSeq`). A null array or element throws `ArgumentNullException` with the
parameter name (message `name[i] must not be null`). `[]` now passes non-NULL pointer and length
arrays, so the core's error surfaces. `EncodeSeq`'s special-cased empty branch is gone.

**Files changed:**

- `packages/dotnet/Iscc.Lib/IsccLib.cs`: added `PinnedRows`, with a `Pin<T>(T[]?[], paramName)`
    method for jagged arrays and a `PinStrings(string[], paramName)` method that converts with
    `ToNativeUtf8` first. The 6 method bodies now read
    `using PinnedRows rows = …; fixed (nint* p = rows.Pointers) [fixed (nuint* l = rows.Lengths)]`.
    Five methods gained `<exception cref="ArgumentNullException">` XML docs; `EncodeSeq` already had
    it
- `packages/dotnet/Iscc.Lib.Tests/ArrayInputTests.cs` (new): 20 `[Fact]`s:
    - 5 empty-input tests (the 4 methods next.md names + `EncodeSeq`). Each checks the core text is in
        the message and "must not be NULL" is not
    - `AlgSimhash([])` returns 32 zero bytes
    - 6 null-array tests asserting `ParamName`
    - 6 null-element tests with the null at index 2, after two valid rows, asserting `ParamName` and
        `name[2]`
    - 1 test that the method still works after a failed null-element call
    - 1 precedence test: a null element beats an earlier embedded NUL
- `packages/dotnet/CLAUDE.md`: rewrote the "Array parameter marshalling" bullets, added a
    jagged-array bullet under "Empty span sentinel", added `ArrayInputTests.cs` to File Layout, and
    the "GCHandle leaks" pitfall now points at the helper

**Verification:**

- `cargo build -p iscc-ffi` → ok. Afterwards,
    `git status --porcelain packages/dotnet/Iscc.Lib/NativeMethods.g.cs crates/iscc-ffi/include/iscc.h`
    printed nothing
- `dotnet build packages/dotnet/Iscc.Lib.Tests/Iscc.Lib.Tests.csproj`: `0 Warning(s)`, `0 Error(s)`
- `dotnet test packages/dotnet/Iscc.Lib.Tests/ -e LD_LIBRARY_PATH=$(pwd)/target/debug`: 150 passed,
    0 failed. With `--filter FullyQualifiedName~ArrayInputTests`: 20 passed. SmokeTests
    `AlgSimhash_EmptyInput_Returns32Bytes` and SeqTests are in the 150
- Power probe: I copied both projects to `/tmp`, restored HEAD's `IsccLib.cs` there and ran the
    filtered suite (19 tests, before I added the precedence test): **15 failed, 4 passed**. The 4
    that pass on the old code are the 3 `EncodeSeq` tests (it was already correct) and
    `AlgSimhash_Empty`. All gen/video/simhash empty and null tests are red on the old code
- `grep -n "GCHandle.Alloc" IsccLib.cs` → one hit, inside `PinnedRows.Pin`
- `mise run check` exit 0. `uv run prek run --hook-stage pre-push --files <3 files>` exit 0

**Next:** The C++ `iscc.hpp` empty-vector issue (`low`, untriaged) is the same defect on the other
C-FFI wrapper, five sites. The cheapest remaining `normal` items are still the Python `encode_seq`
docstrings and the `wasm-bindings.md` package.json spec-fact issue.

**Notes:**

- **Deviation from the next.md wording, same contract:** I did not use a stack sentinel for zero
    count. `PinnedRows` sizes `Pointers`/`Lengths` to `max(Count, 1)`, so `fixed` never yields NULL;
    the real count goes to the FFI as `rows.Count`. This keeps one path with no `count == 0` branch
    at any call site.
- Handle safety: null checks run over every row *before* any handle is allocated. The pin loop also
    has `catch { Dispose(); throw; }`, so an `Alloc` failure part-way frees the handles already
    taken. `Dispose` frees by index, so a second call is a no-op. Null checks also run before the
    `ToNativeUtf8` conversion, so for string inputs a null element always beats an embedded NUL in
    any other element. This matches the old `EncodeSeq` precedence and is pinned by
    `GenMixedCodeV0_NullAfterEmbeddedNul_ThrowsArgumentNull`.
- No public signature change, no Rust change: CRAP/iai/semver are untouched.
