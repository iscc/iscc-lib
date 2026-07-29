# Next Work Package

## Step: Mint `gen_iscc_id_v1` on the C FFI surface

## Goal

Continue the #43 ISCC-IDv1 fan-out: expose `iscc_gen_iscc_id_v1` from `crates/iscc-ffi`, regenerate
the committed C header, and cover it with the golden vector — the first of the 6 remaining typed-int
surfaces, and the one that gates the header-freshness CI check.

## Alternatives Considered

- **Chosen:** FFI IDv1 minting — the handoff-designated next fan-out surface; it also underpins the
    downstream dotnet/cpp surfaces and is the only one needing `iscc.h` regen + a freshness gate.
- **Rejected:** jni IDv1 minting — equally ready, but the handoff sequences ffi first; deferring it
    keeps one surface per step and the header gate exercised now rather than later.

## Scope

- **Modify**: `crates/iscc-ffi/src/lib.rs` — add the `iscc_gen_iscc_id_v1` extern
- **Modify**: `crates/iscc-ffi/include/iscc.h` — regenerate via cbindgen (generated artifact)
- **Modify**: `crates/iscc-ffi/tests/test_iscc.c` — add golden + error assertions (test file)
- **Reference**: `crates/iscc-ffi/src/lib.rs:290-307` (meta pattern), `:571-593` (numeric-arg
    pattern), `result_to_c_string` at `:118`; `crates/iscc-lib/src/lib.rs:1064` (core signature);
    `crates/iscc-ffi/tests/test_iscc.c:94-155` (main structure)

## Not In Scope

- Any other surface (jni, rb, uniffi, dotnet, cpp) — each is its own step.
- The Tier-1 32→33 doc/count sweep across stale sites (separate #43 step). The ffi CLAUDE.md/README
    carry no numeric count, so no doc edit is needed here.
- Touching `packages/dotnet` csbindgen output — the new symbol is unused there; regenerate it in the
    dotnet step.
- Any decode/version-enum change — FFI decode already inherits core's generic path (no wrapper
    enum).

## Implementation Notes

- Signature:
    `pub unsafe extern "C" fn iscc_gen_iscc_id_v1(timestamp: u64, hub_id: u16, realm: u8)   -> *mut c_char`.
    Typed ints, no JS-coercion hazard — core `gen_iscc_id_v1` re-checks all ranges; do NOT add a
    `checked()` guard (that pattern is napi/wasm-only).
- Body mirrors the other gens: `clear_last_error();` then
    `result_to_c_string(iscc_lib::gen_iscc_id_v1(timestamp, hub_id, realm).map(|r| r.iscc))`. No
    pointer args → no NULL guard needed. Add the `# Safety` doc line for consistency (`no_mangle`).
- Regenerate the header verbatim with the CI command:
    `cbindgen --config crates/iscc-ffi/cbindgen.toml --crate iscc-ffi --output   crates/iscc-ffi/include/iscc.h`
    — do not hand-edit `iscc.h`.
- C test: assert golden `iscc_gen_iscc_id_v1(1751831876325218ULL, 1, 0) == "ISCC:MAIGHFECJMOPMIAB"`
    (use `ASSERT_STR_EQ`, `iscc_free_string` after), and one error case (e.g. `realm = 2` →
    `ASSERT_NULL` + `iscc_last_error()` non-NULL).

## Verification

- `cargo build -p iscc-ffi` succeeds; `cargo clippy -p iscc-ffi --all-targets -- -D warnings` clean;
    `cargo fmt -p iscc-ffi --check` clean.
- Re-running the cbindgen command above leaves `git diff --exit-code crates/iscc-ffi/include/iscc.h`
    empty (header committed and fresh), and `iscc.h` contains `iscc_gen_iscc_id_v1`.
- `gcc -o /tmp/test_iscc crates/iscc-ffi/tests/test_iscc.c -I crates/iscc-ffi/include -L   target/debug -liscc_ffi -lpthread -ldl -lm`
    compiles, and `LD_LIBRARY_PATH=target/debug /tmp/test_iscc` exits 0 with the golden + error
    assertions passing.
- `mise run check` (prek hooks) passes.

## Done When

`iscc_gen_iscc_id_v1` is exported from the FFI crate, `iscc.h` is fresh and committed, and the C
test program passes the golden vector and error case with all gates green.
