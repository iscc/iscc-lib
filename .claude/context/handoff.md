## 2026-07-29 — Mint `gen_iscc_id_v1` on the C FFI surface

**Done:** Exported `iscc_gen_iscc_id_v1(timestamp: u64, hub_id: u16, realm: u8) -> *mut c_char` from
`crates/iscc-ffi`, regenerated the committed `iscc.h` via cbindgen, and added golden + error C test
cases. First of the 6 remaining typed-int #43 fan-out surfaces; the header freshness gate is now
exercised.

**Files changed:**

- `crates/iscc-ffi/src/lib.rs`: new `#[unsafe(no_mangle)]` extern after `iscc_gen_iscc_code_v0`;
    body is `clear_last_error()` +
    `result_to_c_string(iscc_lib::gen_iscc_id_v1(...).map(|r| r.iscc))`. Typed ints, no `checked()`
    guard (core re-checks ranges).
- `crates/iscc-ffi/include/iscc.h`: cbindgen-regenerated (verbatim CI command); +21 lines declaring
    the new symbol.
- `crates/iscc-ffi/tests/test_iscc.c`: test 29 golden
    (`iscc_gen_iscc_id_v1(1751831876325218ULL,1,0) == "ISCC:MAIGHFECJMOPMIAB"`) + test 30 error
    (`realm=2` → `ASSERT_NULL` + `iscc_last_error()` non-NULL); renumbered old test 29 → 31.

**Verification:**

- `cargo build -p iscc-ffi` — clean. `cargo clippy -p iscc-ffi --all-targets -- -D warnings` —
    clean. `cargo fmt -p iscc-ffi --check` — clean.
- cbindgen regen is idempotent (`git diff --stat iscc.h` stays +21 vs HEAD; grep confirms
    `char *iscc_gen_iscc_id_v1(uint64_t, uint16_t, uint8_t)` at iscc.h:386).
- `gcc … test_iscc.c -I include -L target/debug -liscc_ffi …` compiles;
    `LD_LIBRARY_PATH=target/debug /tmp/test_iscc` → **83 passed, 0 failed**, incl. golden + realm=2
    error + error-message assertion.
- `mise run check` — all prek pre-commit hooks Passed.

**Next:** #43 fan-out continues on the next typed-int surface — jni IDv1 minting (handoff-sequenced
after ffi), then rb, uniffi→Swift/Kotlin, dotnet, cpp — one per step. The dotnet step regenerates
`NativeMethods.g.cs` (csbindgen already picks up the new symbol on `cargo build` but it is unused
until then). After the fan-out, the Tier-1 32→33 doc/count sweep.

**Notes:** No `checked()` guard by design — `gen_iscc_id_v1` re-validates all three ranges and
returns `IsccError::InvalidInput`, which `result_to_c_string` maps to NULL + last_error. No pointer
args, so no NULL guard. `NativeMethods.g.cs` was NOT regenerated/committed (out of scope; the symbol
is unused in dotnet until its own step). Header +21 lines is the committed delta, not drift.
