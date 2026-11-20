## 2026-11-20 — Review of: .NET — one private helper pins every jagged-array input; empty inputs reach the core

**Verdict:** PASS_WITH_NOTES

**Summary:** A private `PinnedRows : IDisposable` in `IsccLib.cs` now pins the inner arrays for all
six jagged-array methods. A null array or element throws `ArgumentNullException` before anything is
pinned, and `[]` reaches the core's error on the four methods that used to report the FFI's "must
not be NULL". The code is short, frees every handle on all paths, and changes no public signature.
`ArrayInputTests.cs` adds 20 focused tests. The resolved `normal` issue is deleted.

**Verification:**

- [x] `cargo build -p iscc-ffi` + `dotnet test … -e LD_LIBRARY_PATH=…/target/debug` exits 0: 150
    passed, 0 failed; the per-test log shows all 20 `ArrayInputTests` passing
- [x] `git status --porcelain` on `NativeMethods.g.cs` and `iscc.h` prints nothing after the build
- [x] Reviewer-read: all six methods (`GenVideoCodeV0`, `SoftHashVideoV0`, `AlgSimhash`,
    `GenMixedCodeV0`, `GenIsccCodeV0`, `EncodeSeq`) route through `PinnedRows`; the only
    `GCHandle.Alloc` in `packages/dotnet` is inside `PinnedRows.Pin`
- [x] `dotnet build --no-incremental …Iscc.Lib.Tests.csproj`: `0 Warning(s)`, `0 Error(s)`. I forced
    a full rebuild because an incremental build would pass this check without compiling anything
- [x] `mise run check` exits 0
- [x] Gate integrity: no suppression, skip or threshold change anywhere in `@{upstream}..HEAD`

**Issues found:**

- The advance handoff says putting the null check before the NUL check "matches the old `EncodeSeq`
    precedence". It does not. The old loop checked each element for null and then for NUL, in order,
    so `EncodeSeq(["bad\0unit", null])` threw `IsccException`; it now throws
    `ArgumentNullException`. This is a small behaviour change and an acceptable one. I recorded it
    in decisions.md (2026-11-20) together with the `max(Count, 1)` padding, which replaced next.md's
    stack sentinel
- `docs/howto/dotnet.md` "Error handling" names only `IsccException` and
    `InvalidOperationException`. The `ArgumentNullException` on null arrays is documented in the XML
    docs of all six methods but not in that section. Handoff note only, no issue: null was never
    documented, and `EncodeSeq` threw `ArgumentNullException` before this step
- Nit: `PinStrings` checks for nulls twice (once itself, once inside `Pin`). It costs almost
    nothing, so it is not worth changing

**Codex review:** Ran over the full advance range (`--base d588f04`); all 150 tests pass and the
handles are freed on every path. Two P3s: (1) the `Lengths` array is allocated but never used for
the three string methods. Dismissed: it is one shared path, and the cost is a single `nuint[]` per
call. (2) The advance handoff got the precedence wrong. Confirmed, see the first item above.

**Next:** The `normal` issues that CID can work on now are: Python `encode_seq` docstrings (two
docstrings), the `wasm-bindings.md` spec fact (delete one tree line, realign one Version Sync row),
`docs/c-ffi-api.md` type names (needs a compile check against `iscc.h`), and the derived-facts
parity check, which is the largest of the four. Skip the C++ `iscc.hpp` empty-vector entry: it is
`low` and untriaged, so CID must not pick it, even though it is the same defect on another surface.

**Notes:**

- Dependabot entry: I confirmed define-next's probe and recorded it in the issue. Every fixed
    `soupsieve` release (3.0.2 up to 4.1.2) needs `requires-python >=3.11.5`, but the lock resolves
    for `>=3.10`, so that part needs Titusz. `setuptools` 84.1.0 needs `>=3.10` and could move on
    its own, but the entry is resolved only when both are fixed. I did not escalate, because other
    `normal` work is available
- No Rust source changed, so CRAP, iai and semver are unaffected
