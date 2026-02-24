## 2026-02-24 — Implement all 22 remaining JNI bridge functions

**Done:** Implemented all 22 remaining JNI bridge functions in `crates/iscc-jni/src/lib.rs`,
bringing the crate from 1/23 Tier 1 symbols to full coverage (29 `extern "system"` functions total —
23 Tier 1 symbols expand to 29 JNI functions because each streaming hasher requires 4 functions:
new/update/finalize/free). Added `throw_and_default` as a real function plus 5 helper functions for
type conversion (`extract_int_array`, `extract_byte_array`, `extract_int_array_2d`,
`extract_string_array`, `build_string_array`).

**Files changed:**

- `crates/iscc-jni/src/lib.rs`: Added 22 new `extern "system"` JNI functions (9 gen, 4 text utils,
    encode_base64, iscc_decompose, sliding_window, alg_simhash, alg_minhash_256, alg_cdc_chunks,
    soft_hash_video_v0, 8 streaming hasher functions) plus `throw_and_default` error helper and 5
    type-conversion helpers

**Verification:**

- `cargo check -p iscc-jni` exits 0 — clean compile
- `cargo clippy -p iscc-jni -- -D warnings` exits 0 — no warnings
- `cargo clippy --workspace --all-targets -- -D warnings` exits 0 — workspace-wide clean
- All 9 gen function JNI names present: genMetaCodeV0 through genIsccCodeV0
- All 4 text utility names present: textClean, textRemoveNewlines, textTrim, textCollapse
- All 8 streaming hasher functions present: dataHasher{New,Update,Finalize,Free},
    instanceHasher{New,Update,Finalize,Free}
- `throw_and_default` helper function exists as a real function (not doc template)
- `mise run check` — all 14 pre-commit hooks pass
- Existing tests unaffected: 206 unit + 31 integration + 22 text util tests pass

**Next:** The JNI bridge is complete. Next steps for the Java binding track: create the Java wrapper
class (`io.iscc.iscc_lib.IsccLib`) with native method declarations matching all 29 JNI function
signatures, add Maven/Gradle build configuration, and set up JDK in the devcontainer for Java
integration tests. Alternatively, define-next may prioritize the `[normal]` performance issues from
issues.md (codec header Vec<bool> elimination, DataHasher buffer allocation) which would benefit all
binding crates.

**Notes:** The jni 0.21 crate's typed wrapper API (JByteArray, JIntArray) requires explicit
conversion from raw JNI types — helper functions `extract_byte_array` and `extract_int_array` wrap
the `unsafe { JFoo::from_raw() }` + API call pattern to keep each bridge function clean. The
`alg_cdc_chunks` return type uses `"[B"` class descriptor for `byte[][]` (array of byte arrays). No
tests were added in this crate since JNI functions require a running JVM — testing will happen via
Java integration tests in a subsequent step.
