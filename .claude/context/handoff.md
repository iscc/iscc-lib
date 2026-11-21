## 2026-11-21 — Review of: Remove the C++ `safe_data` sentinel and the .NET `PinnedRows` padding

**Verdict:** PASS

**Summary:** Both workarounds are gone. `iscc.hpp` passes `.data()` at all 21 call sites and
`PinnedRows` sizes its arrays to `Count`. Two non-test code files changed, the FFI, `iscc.h` and
`NativeMethods.g.cs` are untouched, and no result changes. The deleted pytest pinned a rule that the
human decision removed, so its removal weakens no gate.

**Verification:**

- [x] `grep -rn "safe_data" packages/ tests/ docs/` prints nothing (exit 1), and
    `grep -n "Math.Max" packages/dotnet/Iscc.Lib/IsccLib.cs` prints nothing — both exit 1
- [x] C++ under ASAN passes — `libasan.so.8` linked, `test_iscc` exit 0, "118 passed, 0 failed",
    including `alg_simhash(one empty inner row) does not report NULL`
- [x] `dotnet test packages/dotnet/Iscc.Lib.Tests/ -e LD_LIBRARY_PATH=$PWD/target/debug` — 155
    passed, 0 failed, including the `ArrayInputTests` empty cases and the new empty-row case
- [x] `uv run pytest` passes — 817 passed; nothing references the deleted file
- [x] `mise run check` exits 0

Two probes beyond next.md, both on claims the advance handoff left unchecked:

- `std::vector<uint8_t>{}.data()` is `nullptr` on this toolchain, also for an inner row. So the C++
    suite really sends `(NULL, 0)` for empty vectors and for the new empty-inner-row case
- `GCHandle.Alloc(Array.Empty<byte>(), Pinned).AddrOfPinnedObject()` is non-null (`dotnet fsi`). So
    the sentence in `packages/dotnet/CLAUDE.md` ("an empty inner row passes its own pointer") is
    true, and the .NET empty-inner-row test guards against a regression without reaching the NULL
    path. Only the outer `[]` reaches it in .NET, and the existing `ArrayInputTests` cover that

**Issues found:**

- Fixed by me: the comment above test block 42b said empty vectors "never reach the NULL path",
    which this step made false. Reworded (comment only; C++ suite rebuilt and re-run, 118 passed)
- Not filed (style): a few call sites in `iscc.hpp` keep a line break that the shorter `.data()`
    form no longer needs; the `encode_component` assertion matches on "NULL" and the new one on
    "must not be NULL"
- No new issue filed. The issue "C++ `safe_data` and .NET `PinnedRows` padding …" is resolved and
    deleted

**Codex review:** no findings. Codex confirmed that every affected FFI entry point accepts a NULL
pointer with length 0, and ran 121 Rust FFI tests, the 118 C++ checks under ASAN/UBSAN and the 155
.NET tests.

**Next:** the generated-artifacts gate (issue "Gates test generated and built artifacts that nothing
refreshes or diffs"): fail pre-push pytest on a stale `_lowlevel.abi3.so`, add a
`git diff --exit-code` for `NativeMethods.g.cs` after the FFI build in CI, and a regenerate-and-diff
step for the UniFFI outputs (compare modulo trailing whitespace). The UniFFI outputs are in a
known-clean state since iteration 272, so the gate starts green.

**Notes:**

- decisions.md: the 2026-11-20 entry on .NET jagged arrays is retitled ("null check first, one
    `PinnedRows` helper") and its padding sentence reworded. The 2026-11-21 triage entry superseded
    that half; the null-check half stays binding. No new entry
- learnings.md: the "Binding input-marshalling traps" bullet no longer calls the workarounds
    redundant; it now says both wrappers pass `(NULL, 0)` through and that only C++ sends a NULL
    inner row. The file stays at 200 lines, so the next new bullet needs an equal cut
- `crates/iscc-ffi/tests/data_edge_vectors.h` still says an empty list "points at a one-element
    sentinel array". That is the generated C test table (C has no zero-length arrays), not the C++
    wrapper, and it is correct as it stands
- No core change: CRAP, iai and the feature matrix are not affected. `detail::safe_data` lived in
    the `detail` namespace, so its removal is no API break for C++ users
- 14 `normal` and 6 `low` issues were open before this step; 13 `normal` remain
