## 2026-11-21 — Remove the C++ `safe_data` sentinel and the .NET `PinnedRows` padding

**Done:** Deleted `detail::safe_data` from `iscc.hpp`. All 21 call sites now pass `.data()` (input
vectors, `ptrs`/`lens` arrays, inner rows, both hasher `update(vector)` overloads). .NET
`PinnedRows` sizes `Pointers`/`Lengths` to `count`, and its remarks and property docs no longer say
`max(Count, 1)`. Removed `tests/test_cpp_safe_data.py`. Added one empty-inner-row test on each
surface and updated both CLAUDE.md files.

**Files changed:**

- `packages/cpp/include/iscc/iscc.hpp`: helper removed, `detail::safe_data(x)` → `x.data()`
- `packages/cpp/tests/test_iscc.cpp`: 42b now checks that `alg_simhash({std::vector<uint8_t>{}})`
    either returns or throws, and never with a message containing "must not be NULL"
- `packages/dotnet/Iscc.Lib/IsccLib.cs`: `new nint[count]` / `new nuint[count]`, docs rewritten
- `packages/dotnet/Iscc.Lib.Tests/ArrayInputTests.cs`: `AlgSimhash_EmptyInnerRow_NeverReportsNull`
    (uses `Record.Exception`, compares against `FfiNullMessage`)
- `packages/cpp/CLAUDE.md`: the two table rows now say `.data()` + `.size()`. The "Empty vector data
    pointers" pitfall now says `(NULL, 0)` is an empty input everywhere, and the pytest reference is
    gone
- `packages/dotnet/CLAUDE.md`: the "Empty spans" bullet describes `Count`-sized arrays
- `tests/test_cpp_safe_data.py`: deleted (`git rm`)

**Verification:**

- `grep -rn "safe_data" packages/ tests/ docs/` printed nothing (exit 1).
    `grep -n "Math.Max" packages/dotnet/Iscc.Lib/IsccLib.cs` printed nothing. A repo-wide grep for
    `test_cpp_safe_data` outside `.git`/`target`/`.claude` found nothing
- C++ ASAN: `cargo build -p iscc-ffi`, then cmake Debug with `-DSANITIZE_ADDRESS=ON` (build dir
    `/tmp/cppbuild273`; `ldd` shows `libasan.so.8` linked). `test_iscc` exit 0, "118 passed, 0
    failed", including `PASS: alg_simhash(one empty inner row) does not report NULL`
- `dotnet test packages/dotnet/Iscc.Lib.Tests/ -e LD_LIBRARY_PATH=$PWD/target/debug`: "Passed: 155,
    Failed: 0". With `--filter EmptyInnerRow` alone: 1 passed
- `uv run pytest -q`: `817 passed`
- `mise run check`: exit 0

**Next:** the generated-artifacts gate (stale `.so`, undiffed `NativeMethods.g.cs`/UniFFI outputs),
as next.md planned.

**Notes:**

- The core's `alg_simhash` accepts a single empty digest (the equal-length check needs ≥2 rows), so
    both new tests take the "returns" path. In C++ the empty inner row really does reach the FFI as
    `(NULL, 0)` (libstdc++ `data()` on an empty vector is nullptr). In .NET, `GCHandle` pinning an
    empty array most likely gives a non-null address, so that test guards against regressions but
    probably does not reach the NULL path. I did not check this on .NET
- No change to the FFI, `iscc.h` or `NativeMethods.g.cs`, and the core is untouched, so CRAP and iai
    are not affected. The `ThrowIfAnyNull` checks are kept. The `alg_minhash_256({})` 32-byte
    assertion is unchanged
- The learnings.md "Binding input-marshalling traps" bullet still calls `PinnedRows` padding and C++
    `safe_data` "redundant". Both are now gone, so the reviewer may want to trim that clause
