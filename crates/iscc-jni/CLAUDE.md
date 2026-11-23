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

## Code Organization

Single Rust file crate. All JNI bridge functions, helper extractors, and streaming hasher wrappers
live in `src/lib.rs`. Do not split into submodules unless the file exceeds ~1500 lines.

## Rust-to-JNI Type Mapping

| Rust / iscc-lib type  | JNI boundary type          | Java type       | Notes                                                                              |
| --------------------- | -------------------------- | --------------- | ---------------------------------------------------------------------------------- |
| `String`              | `JString`                  | `String`        | `JString::try_to_string(env)` for input, `env.new_string()` for output             |
| `Option<String>`      | `JString` (nullable)       | `String` (null) | Check `.is_null()` before `try_to_string()`                                        |
| `&[u8]` / `Vec<u8>`   | `JByteArray`               | `byte[]`        | Extract via `env.convert_byte_array()`, return via `env.byte_array_from_slice()`   |
| `&[i32]` / `Vec<i32>` | `JIntArray`                | `int[]`         | Extract via `extract_int_array`                                                    |
| `Vec<Vec<i32>>`       | `JObjectArray<JIntArray>`  | `int[][]`       | Extract via `extract_int_array_2d`                                                 |
| `Vec<String>`         | `JObjectArray<JString>`    | `String[]`      | Extract via `extract_string_array`, build via `build_string_array`                 |
| `Vec<Vec<u8>>`        | `JObjectArray<JByteArray>` | `byte[][]`      | Build with `JObjectArray::<JByteArray>::new` + `set_element`                       |
| `u32`                 | `jint`                     | `int`           | Cast `jint as u32`; Java has no unsigned int                                       |
| `u8`                  | `jint`                     | `int`           | Validate range 0-255 before casting                                                |
| `bool`                | `jboolean`                 | `boolean`       | `jboolean` is Rust `bool` (jni-sys 0.4); pass through directly                     |
| `usize`               | `jint`                     | `int`           | Validate non-negative before casting                                               |
| `jlong`               | `jlong`                    | `long`          | Used for opaque pointers (streaming hashers)                                       |
| `Result<T>`           | throws in Java             | exception       | `throw_and_default` -> `IllegalArgumentException` (`OutOfMemoryError` on NoMemory) |
| struct result         | `JObject`                  | custom class    | Construct via `env.new_object()` with a `jni_sig!` signature                       |

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

- **Conformance tests**: JUnit 6 `@TestFactory` with `DynamicTest` -- parametrized over vendored
    `data.json` vectors at `crates/iscc-lib/tests/data.json`; each `gen_*_v0` function must match
    the `iscc` output field for every test case; `encodeSeq`/`decodeSeq` are parametrized over the
    `encode_seq`/`decode_seq` sections (raw bytes, plus the byte round trip
    `encodeSeq(decodeSeq(x)) == x`)
- **Unit tests**: `@Test` methods for codec round-trips (`encodeComponent` -> `isccDecode`),
    `jsonToDataUrl`, `genSumCodeV0` equivalence, streaming hasher state validation, constants
    verification, and negative-input validation
- Tests use JUnit 6 assertions (`assertEquals`, `assertThrows`, `assertNotNull`, etc.)
- Test-only dependencies: `junit-jupiter` 6.1.2, `gson` 2.14.0 (for JSON vector parsing)
- No mocking -- tests call real Rust functions through JNI
- `stream:<hex>` prefixed byte data decoded via `HexFormat.of().parseHex()`
- `meta` inputs that are JSON objects are serialized with sorted keys via `TreeMap` before passing
    to Rust

### Edge-case tests (`DataEdgeTest.java`)

- Runs every vector of `crates/iscc-lib/tests/data-edge.json` through a section → caller table;
    `except.java` is the only deviation and an unknown section fails. A rejection must be an
    `IllegalArgumentException` from the raw call; no vector count is hardcoded

### Streaming hasher tests

- Verify `IllegalStateException` is thrown when calling `update()` or `finalize()` on an
    already-finalized hasher
- Every test uses `try/finally` to ensure `*Free()` is called even if assertions fail

## Error Handling

- Every fallible bridge function wraps its body in `EnvUnowned::with_env(|env| ...)` and maps the
    outcome with `.resolve::<ThrowRuntimeExAndDefault>()`; the policy checks for a pending exception
    first, so it only fires for a genuine panic or unhandled `Err`
- **`throw_and_default<T: Default, E: JavaError + ?Sized>`**: takes the error itself (`&e`, or a
    `&str` message -- never `&e.to_string()`); the `JavaError` trait (impls for
    `jni::errors::Error`, `iscc_lib::IsccError`, `str`) picks the class. `JniError::NoMemory` throws
    a fresh `OutOfMemoryError` (jni's `jni_call_with_catch*` macros catch and clear the JVM's
    original and return `NoMemory`); everything else throws `IllegalArgumentException` with the
    `Display` text. Returns `Ok(T::default())` -- the exception stays pending and the JVM raises it
    in Java
- **`throw_state_error<T: Default>`**: throws `IllegalStateException` for invalid hasher state
    (update/finalize after finalize)
- **Pending-exception guard**: both helpers return the default without throwing when
    `env.exception_check()` is true (a `jni_call_post_check_ex!` call such as `set_region` leaves
    the exception pending) -- `ThrowNew` over a pending exception breaks the JNI spec
- **Null handle**: `*Update`/`*Finalize` throw `IllegalArgumentException` for a zero handle before
    dereferencing it; a stale (freed, non-zero) handle is undetectable without a handle registry
- **OOM test**: `IsccLibTest.allocationFailureSurfacesAsOutOfMemoryError` runs `OomProbe` (test
    source) in a child JVM with `-Xmx16m -Xcheck:jni` -- `encodeSeq` over 600,000 units asks for a
    ~20 MB `byte[]` that cannot fit; never shrink the surefire JVM's heap instead
- JNI environment errors (string conversion, array access) are caught and propagated as exceptions
- Rust never panics across the JNI boundary -- `with_env` wraps the body in `catch_unwind`, and all
    fallible paths use `match` with error propagation

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

JNI local references are limited per frame (~512). Array extraction and construction loops wrap each
iteration in `env.with_local_frame(16, |env| ...)` to prevent reference table overflow when
processing large arrays. All data is copied into a Rust-owned `Vec` (or stored into the JVM-side
result array) before the closure returns and the frame pops.

### Result Object Construction

Functions returning structured results (`isccDecode`, `genSumCodeV0`) construct Java objects via
`env.new_object()` with compile-time `jni_sig!` descriptor signatures:

- `IsccDecodeResult`: `"(IIII[B)V"` -- four ints + byte array
- `SumCodeResult`: `"(Ljava/lang/String;Ljava/lang/String;J[Ljava/lang/String;)V"` -- two strings +
    long + string array

### Native Library Loading

`NativeLoader.java` implements a two-strategy loading sequence:

1. Extract platform-specific binary from JAR at `META-INF/native/{os}-{arch}/{libname}` to a temp
    directory and load via `System.load()`. Skipped when `detectOs()` or `detectArch()` returns
    `null` (unrecognized platform)
2. Fall back to `System.loadLibrary("iscc_jni")` (dev/CI and every platform without a bundled
    binary). Also reached when the bundled binary fails to extract or load (`IOException` or
    `UnsatisfiedLinkError`, e.g. musl Linux); that first error is attached via `addSuppressed` to
    the final `UnsatisfiedLinkError`

No throw may precede strategy 2. The loader tests run `LoaderProbe` in a child JVM (overridden
`os.name`/`os.arch`, or a garbage bundled library prepended to the classpath), because the test JVM
has already loaded the library.

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
- Version must stay in sync with workspace version -- coordinated by `scripts/version_sync.py`
    (pom.xml version updated separately)

## Common Pitfalls

- **Do not add business logic** -- this crate is a thin translation layer; all computation belongs
    in `iscc-lib`
- **Do not forget the `_1` encoding** -- Java package `iscc_lib` has an underscore, which JNI
    mangles as `_1` in function names; omitting it causes `UnsatisfiedLinkError` at runtime
- **Do not return raw Rust types** -- all data must be converted to `#[repr(transparent)]` JNI
    reference types (`JString`, `JByteArray`, `JObject`); returning Rust `String` or `Vec` is
    undefined behavior
- **Do not use `Vec<u8>` for byte returns** -- use `env.byte_array_from_slice()` to create a
    `JByteArray` owned by the JVM (zero-copy on the Rust side: it writes the `&[u8]` directly via
    `set_region`)
- **Do not skip local frame management** -- loops that create JNI local references (strings, arrays)
    must wrap each iteration in `with_local_frame` to avoid reference table overflow
- **Do not panic across JNI** -- any panic in Rust crashes the JVM; always convert errors to Java
    exceptions via `throw_and_default` and never use `.unwrap()` on fallible operations
- **Watch for signed/unsigned mismatch** -- Java has no unsigned integer types; `jint` (i32) must be
    cast to `u32`/`u8` with explicit range validation (e.g., `mtype` 0-255, `nbytes` non-negative)
- **Watch for `jboolean` semantics** -- with jni-sys 0.4, `jboolean` is Rust `bool` and passes
    through directly (no `!= 0` conversion needed)
- **Watch for null JString** -- optional string parameters (e.g., `description`, `meta`) arrive as
    null `JString`; always check `.is_null()` before calling `try_to_string()`
- **Free handles exactly once** -- streaming hasher handles (`jlong`) returned by `*New()` must be
    freed by `*Free()`; double-free is undefined behavior, never-free is a memory leak
- **Do not commit `java/target/`** -- Maven build artifacts are gitignored
- **Panic = abort in release** -- the workspace release profile uses `panic = "abort"`, so any panic
    terminates the JVM process immediately
