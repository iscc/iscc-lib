## 2026-02-24 — JNI jint validation and local reference frame safety

**Done:** Added negative `jint` validation guards at 3 call sites (`textTrim`, `slidingWindow`,
`algCdcChunks`) that throw `IllegalArgumentException` before unsafe casts. Wrapped all 5 JNI loops
with `push_local_frame(16)` / `pop_local_frame` to prevent local reference table overflow. Added 3
JUnit 5 `@Test` methods verifying the negative-value exceptions.

**Files changed:**

- `crates/iscc-jni/src/lib.rs`: Added `if param < 0` guards before 3 unsigned casts; added
    `push_local_frame`/`pop_local_frame` to 5 loops (`extract_int_array_2d`, `extract_string_array`,
    `build_string_array`, `algSimhash`, `algCdcChunks`)
- `crates/iscc-jni/java/src/test/java/io/iscc/iscc_lib/IsccLibTest.java`: Added 3 `@Test` methods
    (`textTrimNegativeNbytes`, `slidingWindowNegativeWidth`, `algCdcChunksNegativeAvgChunkSize`)
    using `assertThrows(IllegalArgumentException.class, ...)`

**Verification:**

- `cargo test -p iscc-jni` passes (0 Rust unit tests, as expected — JNI is tested via Java)
- `cargo clippy -p iscc-jni -- -D warnings` clean
- `cd crates/iscc-jni/java && mvn test` passes: 49 tests (46 conformance + 3 new negative-value)
- `grep -c 'push_local_frame' crates/iscc-jni/src/lib.rs` returns 5
- `grep -c 'pop_local_frame' crates/iscc-jni/src/lib.rs` returns 5
- `grep -cP 'nbytes < 0|width < 0|avg_chunk_size < 0' crates/iscc-jni/src/lib.rs` returns 3
- `mise run check` passes all 14 pre-commit hooks

**Next:** Both JNI robustness issues are now resolved. Remaining normal-priority candidates from
issues.md: napi version skew, napi npm packaging, napi unnecessary clone, wasm silent null return,
FFI video frame copy, codec header parsing optimization, DataHasher allocation overhead. Low
priority: alg_dct power-of-two, alg_wtahash bounds check, JNI exception types, Python __version__,
Python module docstring, WASM conformance_selftest size, WASM stale CLAUDE.md, TypeScript port
evaluation.

**Notes:** `pop_local_frame` is `unsafe` in jni crate v0.21 (Rust 2024 edition). Each call has a
SAFETY comment explaining why the pop is safe (all data copied to Rust-owned types before the frame
is released). For the 3 helper functions that return `Result`, the unsafe block wraps
`pop_local_frame` with `?` propagation. For `algSimhash` and `algCdcChunks` (which use
`throw_and_default`), the pop result is discarded with `let _ =` since errors are non-fatal (JVM
cleans up on native return). Initial `mvn test` failed because the old native library was cached —
`cargo build -p iscc-jni` must run before `mvn test` (documented in agent memory).
