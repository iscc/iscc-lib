# Next Work Package

## Step: Add gen_sum_code_v0 to Java JNI bindings

## Goal

Propagate `gen_sum_code_v0` to the Java JNI bindings so Java developers can generate ISCC-SUM codes
with a single native call. This advances issue #15 (5 of 7 bindings complete; Java is 6th).

## Scope

- **Create**: `crates/iscc-jni/java/src/main/java/io/iscc/iscc_lib/SumCodeResult.java`
- **Modify**:
    - `crates/iscc-jni/src/lib.rs` — add JNI bridge function returning `SumCodeResult` Java object
    - `crates/iscc-jni/java/src/main/java/io/iscc/iscc_lib/IsccLib.java` — add `genSumCodeV0` native
        method declaration
    - `crates/iscc-jni/java/src/test/java/io/iscc/iscc_lib/IsccLibTest.java` — add tests
- **Reference**:
    - `crates/iscc-jni/src/lib.rs` lines 588-625 — `isccDecode` JNI bridge returning a Java object via
        `env.find_class()` + `env.new_object()`
    - `crates/iscc-jni/java/src/main/java/io/iscc/iscc_lib/IsccDecodeResult.java` — Java result class
        pattern (immutable, public final fields)
    - `crates/iscc-lib/src/lib.rs` line 967 — `gen_sum_code_v0` Rust implementation and tests

## Not In Scope

- Updating `crates/iscc-ffi/src/lib.rs` module docstring ("9→10 gen functions") — cosmetic, bundle
    with Go step or a separate cleanup step
- Go bindings (`packages/go/`) — that is the next step after Java
- README or documentation updates for gen_sum_code_v0 — deferred until all bindings complete
- Refactoring existing JNI gen functions to return structured results (they currently return
    `jstring`; that's a separate concern)
- Adding `gen_sum_code_v0` conformance vectors to `data.json` (none exist upstream)

## Implementation Notes

### 1. Create `SumCodeResult.java`

Follow the `IsccDecodeResult.java` pattern — simple immutable class with public final fields:

```java
package io.iscc.iscc_lib;

public class SumCodeResult {
    public final String iscc;
    public final String datahash;
    public final long filesize;

    public SumCodeResult(String iscc, String datahash, long filesize) {
        this.iscc = iscc;
        this.datahash = datahash;
        this.filesize = filesize;
    }
}
```

Use `long` for filesize (Java's 64-bit signed long; file sizes won't exceed `Long.MAX_VALUE`).

### 2. Add JNI bridge function in `lib.rs`

Add `Java_io_iscc_iscc_1lib_IsccLib_genSumCodeV0` following the `isccDecode` pattern (line 588) for
returning Java objects:

- Parameters: `(env: JNIEnv, _class: JClass, path: JString, bits: jint, wide: jboolean)`
- Return type: `jobject`
- Extract `path` string via `env.get_string()`, convert to `std::path::Path::new()`
- Call `iscc_lib::gen_sum_code_v0(std::path::Path::new(&path_str), bits as u32, wide != 0)`
- On success:
    1. Create Java strings for `iscc` and `datahash` via `env.new_string()`
    2. Find class: `env.find_class("io/iscc/iscc_lib/SumCodeResult")`
    3. Construct object:
        `env.new_object(class, "(Ljava/lang/String;Ljava/lang/String;J)V", &[iscc_jstr, datahash_jstr, filesize_jlong])`
    4. Return `obj.into_raw()`
- On error: `throw_and_default(&mut env, &e.to_string())`
- `jboolean` is `u8` in jni crate — compare `wide != 0` to get a Rust `bool`

### 3. Add native method in `IsccLib.java`

Add after the existing `genIsccCodeV0` declaration (~line 145):

```java
/**
 * Generate an ISCC-SUM code from a file path.
 *
 * Reads the file once, generating both Data-Code and Instance-Code in a single pass,
 * then composes the final ISCC-CODE.
 *
 * @param path file path to process
 * @param bits hash bit length (32, 64, 96, 128, 160, 192, 224, 256)
 * @param wide if true, use 256-bit combination (requires bits >= 128)
 * @return SumCodeResult with iscc, datahash, and filesize
 * @throws IllegalArgumentException on invalid input or file I/O error
 */
public static native SumCodeResult genSumCodeV0(String path, int bits, boolean wide);
```

### 4. Add tests in `IsccLibTest.java`

Since `data.json` has no `gen_sum_code_v0` vectors, write manual tests using temp files (matching
the Rust core test pattern). Use `java.io.File.createTempFile()` and `deleteOnExit()` for cleanup.
Add a new `@Test` method (not `@TestFactory` since there are no data-driven vectors):

- **Equivalence test**: Write known bytes to temp file, call `genSumCodeV0(path, 64, false)`, then
    compose the same result manually via
    `genDataCodeV0(bytes, 64) + genInstanceCodeV0(bytes, 64) → genIsccCodeV0(codes, false)`. Assert
    `result.iscc` equals the composed ISCC string. Assert `result.filesize` matches bytes length.
- **Result fields test**: Verify `result.datahash` is non-null and non-empty, `result.filesize > 0`
- **Error test**: Non-existent file path throws `IllegalArgumentException`
- **Wide mode test**: With `bits=128`, verify narrow and wide produce different `iscc` values but
    same `datahash` and `filesize`

## Verification

- `cargo test -p iscc-jni` passes (all existing + new JNI tests)
- `cargo clippy -p iscc-jni -- -D warnings` clean
- `cd crates/iscc-jni/java && mvn test` passes (all existing + new Java tests)
- `SumCodeResult.java` exists at expected path
- `genSumCodeV0` native method declared in `IsccLib.java`
- JNI bridge function `Java_io_iscc_iscc_1lib_IsccLib_genSumCodeV0` present in `lib.rs`

## Done When

All verification criteria pass: `cargo test -p iscc-jni` and `mvn test` both succeed with the new
`genSumCodeV0` function returning a `SumCodeResult` object, and clippy is clean.
