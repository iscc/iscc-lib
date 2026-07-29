# Next Work Package

## Step: Mint `gen_iscc_id_v1` on the C++ surface

## Goal

Add idiomatic `iscc::gen_iscc_id_v1(uint64_t, uint16_t, uint8_t)` + `IsccIdResult` to the
header-only C++ wrapper, advancing cpp 32→33 Tier 1 symbols. This is the **11th and final** minting
surface of the #43 IDv1 fan-out.

## Alternatives Considered

- **Chosen:** C++ IDv1 minting — the single remaining minting surface for #43 (v0.6.0 blocker);
    every other surface is done, and the FFI symbol it wraps already ships (iter 182).
- **Rejected:** the repo-wide Tier-1 32→33 doc/count sweep — the deferred *follow-up* to the fan-out
    (issues.md `#43`), best done once after the last minting surface lands, not before.

## Scope

- **Modify**: `packages/cpp/include/iscc/iscc.hpp` (add `IsccIdResult` struct + wrapper),
    `packages/cpp/tests/test_iscc.cpp` (add golden, error-path, round-trip cases)
- **Reference**: `crates/iscc-ffi/include/iscc.h:386`
    (`char *iscc_gen_iscc_id_v1(uint64_t, uint16_t, uint8_t)`),
    `crates/iscc-ffi/tests/test_iscc.c:474-484` (C golden + realm-error), existing `MetaCodeResult`
    \+ `gen_meta_code_v0` in `iscc.hpp` (pattern to mirror)

## Not In Scope

- The repo-wide Tier-1 32→33 doc/count sweep (stale `32` in CLAUDE.md/README, API-doc entries) —
    separate step under #43 after this lands. Do **not** touch count text or the cpp README here.
- No `checked()`/range guard in the wrapper: exact-width unsigned args mean callers cannot pass
    out-of-width values; core re-validates in ts→hub→realm order (matches ffi/dotnet pattern).
- Regenerating `iscc.h` or `NativeMethods.g.cs` — the FFI symbol already exists, no Rust change; a
    `cargo build -p iscc-ffi` must leave both tracked files unchanged.

## Implementation Notes

- Add a result struct next to the other `*CodeResult` structs:
    `struct IsccIdResult { std::string iscc; };`
- Add the wrapper in the "Gen functions" section, mirroring `gen_meta_code_v0`: delegate to
    `iscc_gen_iscc_id_v1(timestamp, hub_id, realm)`, wrap the returned `char*` in
    `detail::UniqueString`, `detail::check_ptr(s.ptr)` (throws `IsccError` on NULL / out-of-range),
    return `IsccIdResult{s.to_string()}`. No default arguments — all three params are required.
- Tests to add in `test_iscc.cpp` (use existing `assert_str_eq` / `assert_true` + the
    try/catch-`threw` idiom already used for `iscc_decode` invalid near line 209):
    - golden: `gen_iscc_id_v1(1751831876325218ULL, 1, 0).iscc == "ISCC:MAIGHFECJMOPMIAB"`
    - error: `gen_iscc_id_v1(1751831876325218ULL, 1, 2)` throws `iscc::IsccError` (invalid realm)
    - round-trip: `iscc_decode("ISCC:MAIGHFECJMOPMIAB")` succeeds and `.version == 1` — C++
        `DecodeResult.version` is a raw `uint8_t`, so no enum widening is needed and this passes
        as-is.

## Verification

- `cargo build -p iscc-ffi` then
    `git diff --quiet -- crates/iscc-ffi/include/iscc.h   packages/dotnet/Iscc.Lib/NativeMethods.g.cs`
    (both clean — no FFI change)
- `uv run --with cmake cmake -B packages/cpp/build -DFFI_LIB_DIR=target/debug packages/cpp` then
    `uv run --with cmake cmake --build packages/cpp/build` succeeds
- `LD_LIBRARY_PATH=target/debug ./packages/cpp/build/tests/test_iscc` prints `0 failed`, exits 0
- `grep -c gen_iscc_id_v1 packages/cpp/include/iscc/iscc.hpp` ≥ 1; `IsccIdResult` present
- `mise run check` passes (prek hygiene + parity hooks clean on the working tree)

## Done When

The C++ wrapper exposes `gen_iscc_id_v1` returning `IsccIdResult`, the C++ test suite passes with
golden/error/round-trip coverage, and the FFI header + P/Invoke files are unchanged by a rebuild.
