# Next Work Package

## Step: Propagate 7 Tier 1 symbols to Java JNI

## Goal

Add the 7 remaining Tier 1 symbols (`encode_component`, `iscc_decode`, `json_to_data_url`, and 4
constants) to the Java JNI binding, bringing it from 23/30 to 30/30 — matching all other completed
bindings.

## Scope

- **Create**: `crates/iscc-jni/java/src/main/java/io/iscc/iscc_lib/IsccDecodeResult.java`
- **Modify**: `crates/iscc-jni/src/lib.rs`,
    `crates/iscc-jni/java/src/main/java/io/iscc/iscc_lib/IsccLib.java`
- **Tests**: `crates/iscc-jni/java/src/test/java/io/iscc/iscc_lib/IsccLibTest.java`
- **Reference**: `crates/iscc-ffi/src/lib.rs` (C FFI pattern for these 3 functions),
    `crates/iscc-napi/src/lib.rs` (Node.js `IsccDecodeResult` object pattern),
    `crates/iscc-jni/src/lib.rs` (existing JNI helpers: `throw_and_default`, `extract_byte_array`)

## Not In Scope

- Go binding propagation (separate step after Java is done)
- Maven Central publishing setup (requires human credentials on Sonatype)
- Updating `docs/howto/java.md` for the new symbols (separate docs step)
- Modifying the existing 29 JNI function signatures or error handling
- Adding `IsccDecodeResult` to the `pom.xml` (it's in the same package, auto-compiled by Maven)

## Implementation Notes

### Constants (4 symbols)

Add as `public static final int` fields in `IsccLib.java` — no JNI functions needed since these are
stable compile-time literals:

```java
public static final int META_TRIM_NAME = 128;
public static final int META_TRIM_DESCRIPTION = 4096;
public static final int IO_READ_SIZE = 4_194_304;
public static final int TEXT_NGRAM_SIZE = 13;
```

Place them at the top of the class, before the static initializer block.

### `jsonToDataUrl` (1 symbol)

String-in, string-out. Follows the exact pattern of `textClean`:

- **Rust JNI**: extract string with `env.get_string(&input)`, call `iscc_lib::json_to_data_url(&s)`,
    return via `env.new_string(result)`. Use `throw_and_default` on error (the Rust function returns
    `IsccResult`).
- **Java**: `public static native String jsonToDataUrl(String json);`

JNI function name: `Java_io_iscc_iscc_1lib_IsccLib_jsonToDataUrl`

### `encodeComponent` (1 symbol)

Scalar args + byte array → string:

- **Rust JNI**: Accept
    `mtype: jint, stype: jint, version: jint, bit_length: jint, digest: jbyteArray`. Validate jint
    ranges (0–255 for mtype/stype/version; ≥0 for bit_length) — use `throw_and_default` for
    out-of-range values. Use existing `extract_byte_array` for digest. Call
    `iscc_lib::encode_component(mtype as u8, stype as u8, version as u8, bit_length as u32, &digest)`.
    Return via `env.new_string(result)`.
- **Java**:
    `public static native String encodeComponent(int mtype, int stype, int version, int bitLength, byte[] digest);`

JNI function name: `Java_io_iscc_iscc_1lib_IsccLib_encodeComponent`

### `isccDecode` (1 symbol — most complex)

Returns structured data via a new Java class.

**Create `IsccDecodeResult.java`** in `io.iscc.iscc_lib` package:

```java
package io.iscc.iscc_lib;

public class IsccDecodeResult {
    public final int maintype;
    public final int subtype;
    public final int version;
    public final int length;
    public final byte[] digest;

    public IsccDecodeResult(int maintype, int subtype, int version, int length, byte[] digest) {
        this.maintype = maintype;
        this.subtype = subtype;
        this.version = version;
        this.length = length;
        this.digest = digest;
    }
}
```

**Rust JNI side**: Call `iscc_lib::iscc_decode(&s)` → get `(u8, u8, u8, u8, Vec<u8>)`. Construct the
Java object:

1. `env.find_class("io/iscc/iscc_lib/IsccDecodeResult")`
2. Create `jbyteArray` from `Vec<u8>` via `env.new_byte_array(len)` + `env.set_byte_array_region`
3. `env.new_object(class, "(IIII[B)V", &[JValue::Int(mt), JValue::Int(st), JValue::Int(vs), JValue::Int(li), JValue::Object(&byte_array)])`
4. Return `jobject`

Return type in the `extern "system"` signature is `jobject`. On error, `throw_and_default` returns
null.

- **Java**: `public static native IsccDecodeResult isccDecode(String isccUnit);`

JNI function name: `Java_io_iscc_iscc_1lib_IsccLib_isccDecode`

### Tests

Add to `IsccLibTest.java`:

1. **Constants** (4 assertions): verify `IsccLib.META_TRIM_NAME == 128`,
    `IsccLib.META_TRIM_DESCRIPTION == 4096`, `IsccLib.IO_READ_SIZE == 4_194_304`,
    `IsccLib.TEXT_NGRAM_SIZE == 13`
2. **`jsonToDataUrl`**: test with `{"key":"value"}` → verify starts with
    `data:application/json;base64,`; test with `{"@context":"..."}` → verify `application/ld+json`
    media type
3. **`encodeComponent`**: encode a known Meta-Code (mtype=0, stype=0, version=0, bit_length=64,
    8-byte digest) → verify output is a valid ISCC unit string
4. **`isccDecode`**: decode a known ISCC unit → verify all 5 fields match expected values. Test
    roundtrip: `encodeComponent` → `isccDecode` → fields match inputs
5. **Error cases**: `isccDecode` with invalid input → `assertThrows(IllegalArgumentException.class)`

Use the same test data patterns as the C FFI tests in `crates/iscc-ffi/tests/test_iscc.c` for
consistency.

## Verification

- `cargo build -p iscc-jni` succeeds
- `cargo clippy -p iscc-jni --all-targets -- -D warnings` clean
- `cd crates/iscc-jni/java && mvn test` passes (51 existing + new tests, 0 failures)
- `grep -c 'extern "system"' crates/iscc-jni/src/lib.rs` shows 32 (29 existing + 3 new)
- `grep -c 'static final int' crates/iscc-jni/java/src/main/java/io/iscc/iscc_lib/IsccLib.java`
    shows 4
- `grep 'IsccDecodeResult' crates/iscc-jni/java/src/main/java/io/iscc/iscc_lib/IsccLib.java` finds
    the `isccDecode` native method

## Done When

All verification criteria pass — Java JNI exposes 30/30 Tier 1 symbols with 4 constants, 3 new
native methods, a structured `IsccDecodeResult` class, and comprehensive tests.
