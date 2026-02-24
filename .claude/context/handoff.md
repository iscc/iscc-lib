## 2026-02-24 — Review of: Implement all 22 remaining JNI bridge functions

**Verdict:** PASS

**Summary:** All 22 remaining JNI bridge functions implemented cleanly in a single file
(`crates/iscc-jni/src/lib.rs`), bringing the crate from 1/23 to 23/23 Tier 1 symbol coverage (29
`extern "system"` functions total — streaming hashers expand to 4 JNI functions each). Code follows
established patterns from the napi/ffi/wasm crates, error handling uses `throw_and_default`
consistently, and helper functions keep the bridge code DRY. All quality gates pass, no regressions.

**Verification:**

- [x] `cargo check -p iscc-jni` exits 0 — clean compile
- [x] `cargo clippy -p iscc-jni -- -D warnings` exits 0 — no warnings
- [x] `cargo clippy --workspace --all-targets -- -D warnings` exits 0 — workspace-wide clean
- [x] `crates/iscc-jni/src/lib.rs` contains 29 `extern "system"` functions covering all 23 Tier 1
    symbols (next.md said "23 functions" but streaming hashers correctly expand to 4 each)
- [x] All 9 gen function JNI names present: genMetaCodeV0 through genIsccCodeV0
- [x] All 4 text utility JNI names present: textClean, textRemoveNewlines, textTrim, textCollapse
- [x] Streaming hasher JNI functions present: dataHasher{New,Update,Finalize,Free},
    instanceHasher{New,Update,Finalize,Free}
- [x] `throw_and_default` helper function exists as a real function (used 51 times)
- [x] `mise run check` — all 14 pre-commit hooks pass
- [x] Existing tests unaffected: 321 Rust tests + 147 Python tests pass

**Issues found:**

- (none)

**Next:** The JNI bridge is complete (23/23 Tier 1 symbols). Next steps for the Java track: create
the Java wrapper class (`io.iscc.iscc_lib.IsccLib`) with native method declarations matching all 29
JNI signatures, add Maven/Gradle build config, and set up JDK in devcontainer for integration tests.
Alternatively, define-next may prioritize the `[normal]` performance issues from issues.md (codec
header `Vec<bool>` elimination, DataHasher buffer allocation) or the `[low]` correctness issues
(`alg_dct` power-of-two validation, `alg_wtahash` length guard).

**Notes:** The intervening non-CID commit (`24ad57c Prepare v0.0.1 experimental release`) between
define-next and advance shifted the diff range — advance commit is at HEAD, not HEAD~1. The
`.unwrap()` calls in the implementation are all on JNI allocation operations (new_string,
byte_array_from_slice, set_object_array_element) which only fail on JVM OOM — this is standard JNI
practice and not a concern.
