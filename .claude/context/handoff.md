## 2026-11-18 — Bind `encode_seq` / `decode_seq` on the C FFI

**Done:** `iscc-ffi` now exports `iscc_encode_seq(units, num_units) -> IsccByteBuffer` and
`iscc_decode_seq(data, data_len) -> char**`, both thin wrappers over the core. They reuse the
existing helpers: the `iscc_gen_iscc_code_v0` input loop, the `iscc_encode_component` len-0 guard,
`vec_to_byte_buffer`, and `vec_to_c_string_array`. Callers free the results with the existing
`iscc_free_byte_buffer` / `iscc_free_string_array`. The regenerated `iscc.h` and
`NativeMethods.g.cs` go in the same commit. ISCC-SEQ is now on 5 of 11 surfaces.

**Files changed:**

- `crates/iscc-ffi/src/lib.rs`: the 2 exports, in house doc style (no-prefix and lowercase input
    accepted; strict base32). Updated the module doc's codec list and the producer lists of the two
    free functions. Added 13 tests: the data.json walk over both seq sections (via `serde_json` and
    a `hex_bytes` helper), a round trip, a lowercase/no-prefix check, 8 one-input negatives (each
    asserts a null result AND that `iscc_last_error()` contains the intended reason), and an
    error-clearing check
- `crates/iscc-ffi/Cargo.toml`: `[dev-dependencies] serde_json = { workspace = true }`
- `crates/iscc-ffi/tests/test_iscc.c`: test 31 (encode the 4 `test_0001_four_units` units, check len
    136, decode, check count 4 and `strcmp`-equal units, free both) and test 32 (`num_units = 0`
    with a non-NULL array gives `.data == NULL` and a non-NULL last error)
- Generated: `crates/iscc-ffi/include/iscc.h`, `packages/dotnet/Iscc.Lib/NativeMethods.g.cs` (only
    the 2 declarations plus the doc-comment changes), `Cargo.lock` (+`serde_json` edge on
    `iscc-ffi`)

**Verification:**

- `cargo test -p iscc-ffi`: 98 passed, 0 failed (all 13 seq tests `ok`)
- `cargo clippy -p iscc-ffi --all-targets -- -D warnings`: clean. `cargo fmt -p iscc-ffi --check`:
    clean
- Running `cbindgen … --output /tmp/iscc_fresh.h` and then `diff` against `iscc.h` gives no output.
    The header declares both symbols (lines 645 and 670)
- `cargo build -p iscc-ffi`, then the ci.yml gcc line, then
    `LD_LIBRARY_PATH=target/debug ./test_iscc`: `78 passed, 0 failed`, exit 0. Under `-Wall -Wextra`
    gcc reports 0 warnings
- valgrind (`--leak-check=full --errors-for-leak-kinds=definite`): `ERROR SUMMARY: 0 errors`, "All
    heap blocks were freed -- no leaks are possible". There are no pre-existing findings either
- After the commit, `cargo build -p iscc-ffi` followed by `git status --porcelain` on
    `NativeMethods.g.cs` and `iscc.h` prints nothing (the build is a no-op on both). The only
    tracked `iscc.h` is `crates/iscc-ffi/include/iscc.h`.
    `dotnet build packages/dotnet/Iscc.Lib/Iscc.Lib.csproj`: 0 warnings, 0 errors
- `mise run check`: exit 0 (with `LANG=C.UTF-8`)

**Next:** .NET seq (`IsccLib.cs` wrappers over the now-generated `NativeMethods.iscc_encode_seq` /
`iscc_decode_seq`), then C++ `iscc.hpp`. Both build directly on this ABI. In C#, `iscc_encode_seq`
takes `byte** units`, so the wrapper must marshal the UTF-8 strings the same way the existing
`GenIsccCodeV0` wrapper does.

**Notes:**

- I pinned each negative to its own error substring, so none of them can pass for an unintended
    reason. The core messages are: "units must not be NULL", "at least one ISCC-UNIT", "units[1]",
    "base32 decode error", "not permitted in an ISCC-SEQ" (for the ISCC-CODE unit
    `ISCC:KACYPXW445…`), "Empty ISCC-SEQ", "Truncated", and "data must not be NULL"
- `lib.rs` is now 2857 lines. That is past the ~2500 soft threshold in `crates/iscc-ffi/CLAUDE.md`
    ("do not split unless the file exceeds ~2500 lines"), and it was already 2572 before this step.
    next.md excluded the split from scope. Flagging it for a future refactor step
- The deferred doc sweep (`docs/c-ffi-api.md`, `crates/iscc-ffi/CLAUDE.md` naming list and counts,
    Tier 1 33→35) is untouched, as scoped
- No core, codec or hot-path code changed, so the bench, iai and CRAP gates are unaffected
