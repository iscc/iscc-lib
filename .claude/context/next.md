# Next Work Package

## Step: Remove the C++ `safe_data` sentinel and the .NET `PinnedRows` padding

## Goal

Picks up the `normal` issue "C++ `safe_data` and .NET `PinnedRows` padding guard a NULL rejection
the FFI no longer has". Since iteration 255 the C FFI reads `(NULL, 0)` as an empty input on every
pointer+length parameter, inner rows included. Titusz decided on 2026-11-21 to remove both
workarounds (decisions.md, "Triage: …"), so each wrapper passes its pointers straight through.

## Alternatives Considered

- **Chosen:** remove both workarounds. It is already decided, touches only the wrappers, and the
    empty-input tests on both surfaces already pin the behaviour
- **Rejected:** the generated-artifacts gate (stale `.so`, undiffed `NativeMethods.g.cs`/UniFFI
    outputs). It is bigger (3 files across hooks and CI) and needs CI wiring. It comes next. The
    UniFFI half is in a known-clean state, so waiting costs nothing

## Scope

- **Modify**: `packages/cpp/include/iscc/iscc.hpp` (delete `detail::safe_data`; every call site uses
    `.data()`)
- **Modify**: `packages/dotnet/Iscc.Lib/IsccLib.cs` (`PinnedRows`: size `Pointers`/`Lengths` to
    `count`, drop the `max(Count, 1)` wording in the remarks and the two property docs)
- **Modify (docs)**: `packages/cpp/CLAUDE.md` (two table rows at :34-35, the "Empty vector data
    pointers" pitfall at :150-154), `packages/dotnet/CLAUDE.md` ("Empty spans" bullet at :154-155)
- **Delete (test)**: `tests/test_cpp_safe_data.py`. It pins the single-`.data()` rule, and that rule
    goes away with this step
- **Reference**: `crates/iscc-ffi/src/lib.rs` (`slice_or_empty` :258, `rows_or_empty` :276-305),
    decisions.md "The C FFI reads `(NULL, 0)` …",
    `packages/dotnet/Iscc.Lib.Tests/ArrayInputTests.cs`, `packages/cpp/tests/test_iscc.cpp`
    (:606-710 empty-input block)

## Not In Scope

- Any change to `iscc-ffi`, `iscc.h` or `NativeMethods.g.cs`. The FFI behaviour is settled, and no
    FFI symbol changes, so nothing gets regenerated
- The ArgumentNullException null checks in `PinnedRows` (`ThrowIfAnyNull`). The 2026-11-20 decision
    keeps them
- Rewording the 2026-11-20 decision in decisions.md. The 2026-11-21 triage entry already records
    that the padding goes
- The generated-artifacts gate, the `alg_minhash_256` empty-input decision, and the C++ 42b
    assertion that pins `alg_minhash_256({})` returning 32 bytes. Leave that test as it is
- Changing `checked_c_str` or any string-input marshalling

## Implementation Notes

- C++: replace each `detail::safe_data(x)` with `x.data()`. That covers the input vectors, the
    `ptrs`/`lens` arrays built inside list wrappers, the inner rows (`ptrs.push_back(d.data())`) and
    the two hasher `update(const std::vector<uint8_t>&)` overloads. An empty vector may yield
    `nullptr` with size 0, which the FFI reads as empty. Afterwards `grep -c safe_data` on the
    header must find nothing
- .NET: `fixed` over a zero-length `nint[]`/`nuint[]` yields a null pointer. Pass it with `Count`
    (0). `_handles` is already sized `count`. Single-array call sites (`fixed (byte* p = data)`)
    have no padding and stay as they are
- Coverage that goes away with the sentinel: an empty INNER row now reaches the FFI as `(NULL, 0)`
    in C++. Add one C++ assertion in the 42/42b block: a list input holding one empty inner row
    (e.g. `iscc::alg_simhash({std::vector<uint8_t>{}})`) must not surface "must not be NULL". It may
    return or throw the core's error. Use the existing helpers and check only that the message is
    not the NULL message. Add the same for .NET in `ArrayInputTests.cs`
    (`AlgSimhash(new[] { Array.Empty<byte>() })`, compare against its NULL-message constant)
- Update the CLAUDE.md docs to say what is now true: input vectors pass `.data()` directly, and
    `(NULL, 0)` is an empty input. Remove the reference to the deleted pytest
- No core change, so no CRAP or iai impact. ASAN runs in the `cpp` CI job. Run it locally as well

## Verification

- `grep -rn "safe_data" packages/ tests/ docs/` prints nothing (exit 1), and
    `grep -n "Math.Max" packages/dotnet/Iscc.Lib/IsccLib.cs` prints nothing
- C++ under ASAN passes (from `packages/cpp`, after `cargo build -p iscc-ffi`):
    `uv run --with cmake cmake -B build -DCMAKE_BUILD_TYPE=Debug -DFFI_LIB_DIR=../../target/debug -DSANITIZE_ADDRESS=ON`,
    `uv run --with cmake cmake --build build`,
    `LD_LIBRARY_PATH=../../target/debug ./build/tests/test_iscc` exits 0 with the new empty-row
    assertion
- `dotnet test packages/dotnet/Iscc.Lib.Tests/ -e LD_LIBRARY_PATH=$PWD/target/debug` passes,
    including the existing `ArrayInputTests` empty cases and the new empty-row case
- `uv run pytest` passes (the deleted test file is gone, nothing else references it)
- `mise run check` exits 0

## Done When

The advance agent is done when both workarounds are gone, the C++ (ASAN) and .NET suites pass with
their empty-input tests plus the new empty-inner-row cases, and the docs describe the direct pointer
passing.
