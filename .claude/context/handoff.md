## 2026-03-02 — Add gen_sum_code_v0 to Java JNI bindings

**Done:** Implemented `genSumCodeV0` in the Java JNI bindings. Created `SumCodeResult.java`
(immutable result class with `iscc`, `datahash`, `filesize` fields), added the JNI bridge function
`Java_io_iscc_iscc_1lib_IsccLib_genSumCodeV0` in `lib.rs` following the `isccDecode` object-return
pattern, and declared the `genSumCodeV0` native method in `IsccLib.java`. Added 4 tests: equivalence
with manual composition, result field validation, non-existent file error, and narrow vs wide mode
comparison.

**Files changed:**

- `crates/iscc-jni/java/src/main/java/io/iscc/iscc_lib/SumCodeResult.java`: Created — immutable Java
    result class with `String iscc`, `String datahash`, `long filesize`
- `crates/iscc-jni/src/lib.rs`: Added `Java_io_iscc_iscc_1lib_IsccLib_genSumCodeV0` JNI bridge
    function returning `jobject` via `env.find_class()` + `env.new_object()`
- `crates/iscc-jni/java/src/main/java/io/iscc/iscc_lib/IsccLib.java`: Added
    `genSumCodeV0(String path, int bits, boolean wide)` native method declaration with Javadoc
- `crates/iscc-jni/java/src/test/java/io/iscc/iscc_lib/IsccLibTest.java`: Added 4 `@Test` methods
    for genSumCodeV0

**Verification:**

- `cargo clippy -p iscc-jni -- -D warnings` — clean, no warnings
- `cargo test -p iscc-jni` — 0 Rust tests (crate has no Rust unit tests, only JNI exports)
- `cd crates/iscc-jni/java && mvn test` — 62 tests, 0 failures (58 existing + 4 new)
- `mise run check` — all 14 pre-commit hooks passed

**Next:** Propagate `gen_sum_code_v0` to Go bindings (`packages/go/`). This is the last binding (7
of 7) and requires a pure Go reimplementation since Go doesn't wrap Rust. After Go, issue #15 is
complete. Also consider bundling the module docstring function count update ("9→10 gen functions")
in `crates/iscc-ffi/src/lib.rs` with a cleanup step.

**Notes:** The JNI constructor signature `(Ljava/lang/String;Ljava/lang/String;J)V` uses `J` for
Java `long` (64-bit signed), matching `filesize: u64` from Rust cast as `jlong`. Java's
`Long.MAX_VALUE` (2^63-1) is sufficient for any realistic file size. The `wide` parameter uses
`jboolean` (which is `u8` in the jni crate), compared as `wide != 0` to get a Rust `bool`.
