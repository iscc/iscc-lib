## 2026-02-24 — Review of: JNI jint validation and local reference frame safety

**Verdict:** PASS

**Summary:** Added negative `jint` validation guards at 3 call sites and
`push_local_frame`/`pop_local_frame` safety wrappers at 5 loops — exactly as specified. 3 new JUnit
5 tests verify the exceptions. All 49 Java tests (46 conformance + 3 negative-value) pass, clippy
clean, all 14 pre-commit hooks pass.

**Verification:**

- [x] `cargo test -p iscc-jni` passes — 0 Rust unit tests (expected, JNI tested via Java)
- [x] `cargo clippy -p iscc-jni -- -D warnings` clean
- [x] `cd crates/iscc-jni/java && mvn test` passes — 49 tests, 0 failures
- [x] `grep -c 'push_local_frame' crates/iscc-jni/src/lib.rs` returns 5
- [x] `grep -c 'pop_local_frame' crates/iscc-jni/src/lib.rs` returns 5
- [x] `grep -cP 'nbytes < 0|width < 0|avg_chunk_size < 0' crates/iscc-jni/src/lib.rs` returns 3
- [x] `mise run check` passes all 14 pre-commit hooks

**Issues found:**

- (none)

**Next:** Both JNI robustness issues resolved and deleted from issues.md. Remaining normal-priority
candidates: napi version skew, napi npm packaging, napi unnecessary clone, wasm silent null return,
FFI video frame copy, codec header parsing optimization, DataHasher allocation overhead. Low
priority: alg_dct power-of-two, alg_wtahash bounds check, JNI exception types, Python __version__,
Python module docstring, WASM conformance_selftest size, WASM stale CLAUDE.md, TypeScript port
evaluation.

**Notes:** `pop_local_frame` is `unsafe` in jni crate v0.21 (Rust 2024 edition). Each `unsafe` block
has a SAFETY comment. Helper functions propagate frame errors via `?`. The `algSimhash` and
`algCdcChunks` functions use `let _ =` for the pop result (JVM cleans up on native return). On early
error paths after `push_local_frame`, the JVM handles cleanup automatically.
