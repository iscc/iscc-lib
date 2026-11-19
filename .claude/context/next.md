# Next Work Package

## Step: Bind `encode_seq` / `decode_seq` in the C++ header-only wrapper

## Goal

Add `iscc::encode_seq` and `iscc::decode_seq` to `packages/cpp/include/iscc/iscc.hpp`, with RAII
ownership of the returned FFI buffer and string array. This brings ISCC-SEQ to 7 of 11 surfaces for
v0.8.0. The wrapper must not copy the `c_str()` embedded-NUL truncation that the open .NET issue
describes.

## Alternatives Considered

- **Chosen:** C++ seq. It continues the fan-out (FFI → .NET → C++), which needs only one header and
    its smoke test, and it is the handoff's "Next".
- **Rejected:** fix embedded-NUL truncation across .NET and C++ first. The issue says choosing
    between "reject NUL" and "length-delimited FFI ABI" is a design decision, and that needs
    sign-off before it is scoped. C++ seq can avoid the problem on its own, because every seq unit
    containing a NUL is already an error in iscc-core (base32 decode error).
- **Rejected:** small `normal` issues (Python docstring, WASM npm metadata, `uv.lock`). Finish the
    coherent seq fan-out first; these remain open and block the release.

## Scope

- **Modify**: `packages/cpp/include/iscc/iscc.hpp`
- **Modify (tests)**: `packages/cpp/tests/test_iscc.cpp`
- **Reference**: `crates/iscc-ffi/include/iscc.h` (`iscc_encode_seq` L645, `iscc_decode_seq` L670),
    `crates/iscc-ffi/src/lib.rs` L1075 (null check order), `crates/iscc-ffi/tests/test_iscc.c` cases
    31/32 (the seq case pinned in iter 205), `crates/iscc-lib/tests/data.json` (`encode_seq` /
    `decode_seq` sections, `bytes:<hex>` values), `.claude/context/specs/rust-core.md` → "encode_seq
    / decode_seq" (canonical, since `cpp-bindings.md` has no seq section)

## Not In Scope

- The 15 existing `c_str()` call sites and the NUL issue's design choice. Leave them untouched.
- The NULL-on-empty flaw in the existing `gen_mixed_code_v0` / `gen_iscc_code_v0` wrappers.
- A JSON parser or fixture generator for C++ tests, and a raw-pointer `decode_seq` overload.
- Docs and the 33→35 count (`packages/cpp/CLAUDE.md`, `README.md`, `docs/howto/c-cpp.md`). The sweep
    step owns these.
- Java, Ruby, Swift and Kotlin seq.

## Implementation Notes

- Signatures, placed in the "Codec functions" section next to `iscc_decompose`:
    `inline std::vector<uint8_t> encode_seq(const std::vector<std::string>& units)` and
    `inline std::vector<std::string> decode_seq(const std::vector<uint8_t>& data)`.
- `encode_seq` uses the existing `detail::UniqueByteBuffer` guard, then `check_ptr`-style error
    handling on `buf.data`, then `.to_vec()`. `decode_seq` uses `detail::UniqueStringArray`,
    `check_ptr` and `.to_vec()`, passing `detail::safe_data(data)` and `data.size()`. The FFI checks
    `len == 0` before NULL, so the core's empty-input error still comes through.
- **Empty-vector trap:** `std::vector<const char*>{}.data()` may be NULL, and `iscc_encode_seq`
    checks `units.is_null()` first. That would surface "units must not be NULL" instead of the
    core's "at least one ISCC-UNIT". Pass a non-null sentinel pointer when `units` is empty. No
    `safe_data` overload exists for `const char*`: add one in `detail`, or use a local static.
- **Embedded NUL:** before marshalling, throw `iscc::IsccError` if any unit contains `'\0'`
    (`find('\0') != npos`). The message should name the unit index. This matches iscc-core, which
    rejects such a unit, so it settles nothing about the open NUL issue.
- The C++ side has no data.json reader, so hard-code literal vectors, following C case 31. Use
    `encode_seq` `test_0001_four_units`: assert the full expected bytes from data.json, not just the
    length. Also hard-code at least one `decode_seq` input as a byte literal and assert the bytes
    round trip `encode_seq(decode_seq(x)) == x`. Coverage of all 4+4 vectors already comes through
    `conformance_selftest` (test 1), which runs the seq sections in the Rust core.
- Negative tests use one bad input per call, catching `iscc::IsccError` and checking a message
    substring: `encode_seq({})` → "at least one ISCC-UNIT"; `encode_seq({unit + '\0' + "junk"})`
    throws; `decode_seq({})` throws; a truncated `decode_seq` input (drop the last byte of a valid
    seq) throws. Read the error strings in `crates/iscc-lib/src/codec.rs`, not from this note.
- Optional and report-only: when the test binary is built, probe one existing site, e.g.
    `iscc::text_clean("a\0b")` or `iscc_decompose(code + '\0' + "junk")`. Put one handoff line on
    whether it truncates, so review can widen the NUL issue to C++. Do not fix it.
- Binding-only step: no core, FFI or hot-path change, so neither the iai nor the CRAP gate is
    affected. `iscc.h` and `NativeMethods.g.cs` must not change.
- Local cmake: `uv run --with cmake cmake …`. Use a dedicated build dir (e.g. `build-seq`), because
    `packages/cpp/build*` dirs already exist.

## Verification

- `cargo build -p iscc-ffi`, then in `packages/cpp`:
    `uv run --with cmake cmake -B build-seq -DCMAKE_BUILD_TYPE=Debug -DFFI_LIB_DIR=../../target/debug -DSANITIZE_ADDRESS=ON`
    and `uv run --with cmake cmake --build build-seq` exit 0 with no warnings from `iscc.hpp`
- `LD_LIBRARY_PATH=../../target/debug ./build-seq/tests/test_iscc` exits 0 with no FAIL lines and no
    ASAN report, and prints PASS lines for the encode_seq vector, the decode_seq byte round trip and
    each negative case
- `iscc.hpp` declares `encode_seq(const std::vector<std::string>&)` returning
    `std::vector<uint8_t>`, and `decode_seq(const std::vector<uint8_t>&)` returning
    `std::vector<std::string>`
- A test asserts that `encode_seq({})` throws `IsccError` whose message contains "at least one
    ISCC-UNIT", and another asserts that a unit with an embedded `'\0'` throws
- `git status --porcelain crates/iscc-ffi packages/dotnet` is empty
- `mise run check` exits 0

## Done When

All six verification criteria pass, with `encode_seq` / `decode_seq` callable from C++ and pinned by
the vector, byte round trip and negative tests above.
