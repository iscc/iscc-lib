# CLAUDE.md -- iscc-jni

JNI bindings exposing the `iscc-lib` Rust core to Java/Kotlin/JVM languages, published as
`io.iscc:iscc-lib` on Maven Central.

## Crate Role

- Hub-and-spoke binding crate: depends on `iscc-lib` (pure Rust core), adds zero business logic
- Compiles to a platform-specific shared library (`cdylib`) loaded by the JVM via
    `System.loadLibrary`
- Published to Maven Central (not crates.io) -- `publish = false` in Cargo.toml
- Wraps all Tier 1 API symbols as `extern "system"` JNI functions with mangled names matching the
    Java class `io.iscc.iscc_lib.IsccLib`

## Module Layout

```
crates/iscc-jni/
  Cargo.toml                  # jni crate dep, cdylib
  src/
    lib.rs                    # All JNI wrapper functions (single file)
  java/
    pom.xml                   # Maven build: io.iscc:iscc-lib, Java 17
    src/main/java/io/iscc/iscc_lib/
      IsccLib.java            # Static native method declarations + constants
      IsccDecodeResult.java   # Result class for isccDecode (maintype, subtype, version, length, digest)
      SumCodeResult.java      # Result class for genSumCodeV0 (iscc, datahash, filesize, units)
      NativeLoader.java       # Platform-aware native library loader (JAR extraction + fallback)
    src/test/java/io/iscc/iscc_lib/
      IsccLibTest.java        # JUnit 5 conformance + unit tests
  README.md                   # Maven installation and API overview
```

Single Rust file crate. All JNI bridge functions, helper extractors, and streaming hasher wrappers
live in `src/lib.rs`. Do not split into submodules unless the file exceeds ~1500 lines.

## Rust-to-JNI Type Mapping

| Rust / iscc-lib type  | JNI boundary type              | Java type       | Notes                                                                |
| --------------------- | ------------------------------ | --------------- | -------------------------------------------------------------------- |
| `String`              | `JString` / `jstring`          | `String`        | `env.get_string()` for input, `env.new_string()` for output          |
| `Option<String>`      | `JString` (nullable)           | `String` (null) | Check `.is_null()` before `get_string()`                             |
| `&[u8]` / `Vec<u8>`   | `jbyteArray`                   | `byte[]`        | Extract via `extract_byte_array`, return via `byte_array_from_slice` |
| `&[i32]` / `Vec<i32>` | `jintArray`                    | `int[]`         | Extract via `extract_int_array`                                      |
| `Vec<Vec<i32>>`       | `JObjectArray` of `jintArray`  | `int[][]`       | Extract via `extract_int_array_2d`                                   |
| `Vec<String>`         | `JObjectArray` of `JString`    | `String[]`      | Extract via `extract_string_array`, build via `build_string_array`   |
| `Vec<Vec<u8>>`        | `JObjectArray` of `jbyteArray` | `byte[][]`      | Build with `find_class("[B")` + `new_object_array`                   |
| `u32`                 | `jint`                         | `int`           | Cast `jint as u32`; Java has no unsigned int                         |
| `u8`                  | `jint`                         | `int`           | Validate range 0-255 before casting                                  |
| `bool`                | `jboolean`                     | `boolean`       | Compare `!= 0` to convert to Rust `bool`                             |
| `usize`               | `jint`                         | `int`           | Validate non-negative before casting                                 |
| `jlong`               | `jlong`                        | `long`          | Used for opaque pointers (streaming hashers)                         |
| `Result<T>`           | throws in Java                 | exception       | `throw_and_default` -> `IllegalArgumentException`                    |
| struct result         | `jobject`                      | custom class    | Construct via `env.new_object()` with JNI descriptor signature       |

## Build Commands

```bash
# Build the native library (shared .so/.dylib/.dll)
cargo build -p iscc-jni
cargo build -p iscc-jni --release

# Build and test the Java side (requires cargo build first)
cd crates/iscc-jni/java && mvn test

# Maven passes -Djava.library.path=target/debug via surefire plugin config
# For release builds, adjust the library path accordingly
```

- `cargo build -p iscc-jni` produces `libiscc_jni.so` (Linux), `libiscc_jni.dylib` (macOS), or
    `iscc_jni.dll` (Windows) in `target/debug/` or `target/release/`
- Maven `surefire-plugin` sets `-Djava.library.path` to `target/debug` so the JVM can find the
    native library at test time
- Java source requires JDK 17+ (`maven.compiler.source=17`)

## Test Patterns

### Java tests (`IsccLibTest.java`)

- **Conformance tests**: JUnit 5 `@TestFactory` with `DynamicTest` -- parametrized over vendored
    `data.json` vectors at `crates/iscc-lib/tests/data.json`; each `gen_*_v0` function must match
    the `iscc` output field for every test case
- **Unit tests**: `@Test` methods for codec round-trips (`encodeComponent` -> `isccDecode`),
    `jsonToDataUrl`, `genSumCodeV0` equivalence, streaming hasher state validation, constants
    verification, and negative-input validation
- Tests use JUnit 5 assertions (`assertEquals`, `assertThrows`, `assertNotNull`, etc.)
- Test-only dependencies: `junit-jupiter` 5.11.4, `gson` 2.11.0 (for JSON vector parsing)
- No mocking -- tests call real Rust functions through JNI
- `stream:<hex>` prefixed byte data decoded via `HexFormat.of().parseHex()`
- `meta` inputs that are JSON objects are serialized with sorted keys via `TreeMap` before passing
    to Rust

### Streaming hasher tests

- Verify `IllegalStateException` is thrown when calling `update()` or `finalize()` on an
    already-finalized hasher
- Every test uses `try/finally` to ensure `*Free()` is called even if assertions fail

## Error Handling

- **`throw_and_default<T: Default>`**: throws `IllegalArgumentException` in Java, returns
    `T::default()` to satisfy the JNI return type (e.g., `null` for pointers, `0` for scalars)
- **`throw_state_error<T: Default>`**: throws `IllegalStateException` for invalid hasher state
    (update/finalize after finalize)
- Errors from `iscc_lib` are converted to exception messages via `.to_string()`
- JNI environment errors (string conversion, array access) are caught and propagated as exceptions
- Rust never panics across the JNI boundary -- all fallible paths use `match` with error propagation

## JNI-Specific Patterns

### Function Naming Convention

JNI requires mangled names encoding the Java package, class, and method. For
`io.iscc.iscc_lib.IsccLib`, the package `iscc_lib` contains an underscore encoded as `_1`:

```
Java_io_iscc_iscc_1lib_IsccLib_<methodName>
```

### Opaque Pointer Pattern (Streaming Hashers)

`DataHasher` and `InstanceHasher` use the pointer-as-`jlong` pattern:

1. `*New()` allocates via `Box::into_raw(Box::new(JniWrapper { inner: Some(hasher) }))` and returns
    the pointer cast to `jlong`
2. `*Update()` / `*Finalize()` cast `jlong` back to `*mut JniWrapper` and operate on the inner
    `Option`
3. `*Free()` reclaims memory via `Box::from_raw()` -- zero/null handle is a no-op
4. The `Option` wrapper enables finalize-once semantics: `finalize()` calls `.take()`, subsequent
    calls find `None` and throw `IllegalStateException`

### Local Reference Frame Management

JNI local references are limited per frame (~512). Array extraction and construction loops use
`env.push_local_frame(16)` / `env.pop_local_frame()` to prevent reference table overflow when
processing large arrays. All data is copied into Rust-owned `Vec` before popping the frame.

### Result Object Construction

Functions returning structured results (`isccDecode`, `genSumCodeV0`) construct Java objects via
`env.new_object()` with JNI type descriptor signatures:

- `IsccDecodeResult`: `"(IIII[B)V"` -- four ints + byte array
- `SumCodeResult`: `"(Ljava/lang/String;Ljava/lang/String;J[Ljava/lang/String;)V"` -- two strings +
    long + string array

### Native Library Loading

`NativeLoader.java` implements a two-strategy loading sequence:

1. Extract platform-specific binary from JAR at `META-INF/native/{os}-{arch}/{libname}` to a temp
    directory and load via `System.load()`
2. Fall back to `System.loadLibrary("iscc_jni")` (for dev/CI with `java.library.path`)

Loading is synchronized with a volatile guard to ensure exactly-once semantics. The static
initializer in `IsccLib` triggers `NativeLoader.load()` automatically on first class access.

## Publishing Constraints

- Maven coordinates: `io.iscc:iscc-lib` (groupId `io.iscc`, artifactId `iscc-lib`)
- Published to Maven Central via Sonatype Central Publishing (`central-publishing-maven-plugin`)
- Release profile (`-Prelease`) attaches sources JAR, javadoc JAR, and GPG signatures
- Platform-specific native libraries bundled in JAR under `META-INF/native/{os}-{arch}/`
- Supported platforms: `linux-x86_64`, `linux-aarch64`, `macos-x86_64`, `macos-aarch64`,
    `windows-x86_64`
- Library filenames: `libiscc_jni.so` (Linux), `libiscc_jni.dylib` (macOS), `iscc_jni.dll` (Windows)
- Version must stay in sync with workspace version -- coordinated by `scripts/sync_versions.py`
    (pom.xml version updated separately)

## Common Pitfalls

- **Do not add business logic** -- this crate is a thin translation layer; all computation belongs
    in `iscc-lib`
- **Do not forget the `_1` encoding** -- Java package `iscc_lib` has an underscore, which JNI
    mangles as `_1` in function names; omitting it causes `UnsatisfiedLinkError` at runtime
- **Do not return raw Rust types** -- all data must be converted to JNI-compatible types (`jstring`,
    `jbyteArray`, `jobject`); returning Rust `String` or `Vec` is undefined behavior
- **Do not use `Vec<u8>` for byte returns** -- use `env.byte_array_from_slice()` to create a
    `jbyteArray` owned by the JVM
- **Do not skip local frame management** -- loops that create JNI local references (strings, arrays)
    must use `push_local_frame` / `pop_local_frame` to avoid reference table overflow
- **Do not panic across JNI** -- any panic in Rust crashes the JVM; always convert errors to Java
    exceptions via `throw_and_default` and never use `.unwrap()` on fallible operations
- **Watch for signed/unsigned mismatch** -- Java has no unsigned integer types; `jint` (i32) must be
    cast to `u32`/`u8` with explicit range validation (e.g., `mtype` 0-255, `nbytes` non-negative)
- **Watch for `jboolean` semantics** -- JNI `jboolean` is `u8`, not Rust `bool`; use `!= 0` for
    conversion, never cast directly
- **Watch for null JString** -- optional string parameters (e.g., `description`, `meta`) arrive as
    null `JString`; always check `.is_null()` before calling `env.get_string()`
- **Free handles exactly once** -- streaming hasher handles (`jlong`) returned by `*New()` must be
    freed by `*Free()`; double-free is undefined behavior, never-free is a memory leak
- **Do not commit `java/target/`** -- Maven build artifacts are gitignored
- **Panic = abort in release** -- the workspace release profile uses `panic = "abort"`, so any panic
    terminates the JVM process immediately
