# Next Work Package

## Step: Bind `encode_seq` / `decode_seq` on the C FFI

## Goal

Export `iscc_encode_seq` / `iscc_decode_seq` from `iscc-ffi` and commit the regenerated `iscc.h` and
`NativeMethods.g.cs`. That brings ISCC-SEQ to 5 of 11 surfaces and unblocks the .NET and C++ seq
steps, which build on this C ABI.

## Alternatives Considered

- **Chosen:** C FFI seq. It is the critical-path gap for v0.8.0: two more surfaces depend on it, and
    the handoff names it.
- **Rejected:** the `uv.lock` Dependabot alerts (`soupsieve`/`setuptools`, `normal`). They affect
    only the dev environment and are small enough to slot in later. The feature fan-out comes first
    while the FFI context is fresh.

## Scope

- **Modify**: `crates/iscc-ffi/src/lib.rs` (2 exports, update the module doc's codec list, tests in
    `mod tests`), `crates/iscc-ffi/Cargo.toml` (add `[dev-dependencies] serde_json`, which is
    already a workspace dep), `crates/iscc-ffi/tests/test_iscc.c` (C tests)
- **Regenerate + commit (generated, not counted)**: `crates/iscc-ffi/include/iscc.h` (cbindgen),
    `packages/dotnet/Iscc.Lib/NativeMethods.g.cs` (csbindgen rewrites it on every
    `cargo build -p iscc-ffi`)
- **Reference**: `iscc_gen_iscc_code_v0` (input loop over `*const *const c_char` + count),
    `iscc_encode_component` (the NULL/len-0 slice guard), `iscc_decompose` +
    `vec_to_c_string_array`, `vec_to_byte_buffer` in `crates/iscc-ffi/src/lib.rs`;
    `encode_seq`/`decode_seq` docs in `crates/iscc-lib/src/codec.rs`; the CI `c-ffi` job in
    `.github/workflows/ci.yml` (copy its cbindgen and gcc lines exactly)

## Not In Scope

- The .NET `IsccLib.cs` wrapper and the C++ `iscc.hpp` wrapper. They are the next steps; only the
    generated `NativeMethods.g.cs` changes here
- `docs/c-ffi-api.md`, the READMEs, `crates/iscc-ffi/CLAUDE.md` counts, and the Tier 1 33→35 doc
    sweep (deferred until every surface binds seq)
- The open c-ffi-api type-name issue
- Any edit to `crates/iscc-lib`, and splitting `lib.rs` into modules
- New free functions or result structs: the existing ones fit

## Implementation Notes

- Signatures:
    - `iscc_encode_seq(units: *const *const c_char, num_units: usize) -> IsccByteBuffer`: free with
        `iscc_free_byte_buffer`; on error `.data` is NULL
    - `iscc_decode_seq(data: *const u8, data_len: usize) -> *mut *mut c_char`: a NULL-terminated
        array, freed with `iscc_free_string_array`; NULL on error
    - On every error path, call `set_last_error`
- Input handling: copy `iscc_gen_iscc_code_v0` exactly. A NULL `units` errors even when the count is
    0; a NULL element errors with its index; invalid UTF-8 errors. Pass the `&[&str]` straight to
    `iscc_lib::encode_seq`, which itself rejects an empty list.
- For `decode_seq`, copy the `iscc_encode_component` guard: `data_len == 0` → `&[]`, and the core
    then rejects empty input. NULL with `data_len > 0` errors.
- The core never returns an empty `Vec` from either function, so the error signals are unambiguous.
- Write the doc comments in the house style (`# Parameters` / `# Returns` / `# Safety`). Mention
    that units may omit `ISCC:` and may be lowercase, and that base32 decoding is strict (no
    padding, zero spare bits, ASCII-only case folding).
- **Rust tests (the FFI conformance harness):**
    - Load `include_str!("../../iscc-lib/tests/data.json")` with `serde_json`. Run every case in the
        `encode_seq` and `decode_seq` sections through the FFI functions: expected values are
        `bytes:<hex>`, so add a small hex helper in the tests. Compare the full content: bytes for
        encode, the unit list for decode.
    - Add a round trip (decode of encode).
    - Assert that the walk saw a non-zero number of cases per section. Do not pin an exact count.
    - Negatives, one bad input per call: NULL `units`, `num_units == 0`, a NULL element, a malformed
        unit, an ISCC-CODE unit (excluded MainType), `data_len == 0`, truncated bytes, and NULL `data`
        with a non-zero length. Each negative asserts the null result AND a non-NULL
        `iscc_last_error()`.
- **C tests in `test_iscc.c`:** continue the numbered sequence after test 30 and use the existing
    `ASSERT_*` macros.
    - Encode the 4 unit strings from `data.json` `encode_seq` → `test_0001_four_units` (already
        canonical `ISCC:` uppercase).
    - Decode the buffer. Assert a count of 4 and `strcmp`-equal units.
    - Free both with the library free functions.
    - Add one negative: `num_units = 0` returns `.data == NULL` and `iscc_last_error()` is non-NULL.
    - Full vector coverage lives in the Rust test above and in `iscc_conformance_selftest`. The C test
        checks the ABI and ownership. That is why decision 2026-07-27's generated-header mechanism,
        which applied to the since-removed Unicode fixture, is not needed here.
- Run the C test under valgrind:
    `LD_LIBRARY_PATH=target/debug valgrind --leak-check=full --errors-for-leak-kinds=definite ./test_iscc`.
    A definite leak or invalid access traced to the seq calls is a failure. Report any
    pre-existing finding in the handoff; do not fix it.
- Regenerate the header with the exact cbindgen command from `ci.yml`. Then run
    `cargo build -p iscc-ffi` and commit `NativeMethods.g.cs` in the same commit. Pre-push clippy
    rejects a push if the build modified a tracked file.
- This step touches no core, text or codec hot path, so neither the iai Perf gate nor the CRAP gate
    (both core-only) can move.

## Verification

- `cargo test -p iscc-ffi` passes, including the data.json seq-vector walk, the round trip and the
    one-input-per-call negatives
- `cargo clippy -p iscc-ffi --all-targets -- -D warnings` and `cargo fmt -p iscc-ffi --check` are
    clean
- The header is fresh. Running
    `cbindgen --config crates/iscc-ffi/cbindgen.toml --crate iscc-ffi --output /tmp/iscc_fresh.h`
    then `diff /tmp/iscc_fresh.h crates/iscc-ffi/include/iscc.h` exits 0, and `iscc.h` declares both
    `iscc_encode_seq` and `iscc_decode_seq`
- The C test passes. After `cargo build -p iscc-ffi`, the `ci.yml` gcc line plus
    `LD_LIBRARY_PATH=target/debug ./test_iscc` reports 0 failures, including the seq round trip
- `NativeMethods.g.cs` contains both symbols. After `cargo build -p iscc-ffi`,
    `git status --porcelain packages/dotnet/Iscc.Lib/NativeMethods.g.cs crates/iscc-ffi/include/iscc.h`
    is empty, and `dotnet build packages/dotnet/Iscc.Lib/Iscc.Lib.csproj` exits 0
- `mise run check` exits 0

## Done When

All six verification criteria pass with the two exports, their tests, and both regenerated artifacts
committed together.
