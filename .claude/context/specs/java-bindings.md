# Spec: Java Bindings — JNI Bridge with Maven Central Publishing

Java/JVM bindings for `iscc-lib` published as `io.iscc:iscc-lib` on Maven Central. Uses JNI (Java
Native Interface) to call into the Rust `iscc-lib` core crate, with platform-specific native
libraries bundled inside the JAR under `META-INF/native/`.

## Architecture

**Two-layer design** (matching the Python/Ruby binding pattern):

1. **Rust JNI bridge** (`crates/iscc-jni/src/lib.rs`): `extern "system"` functions with JNI-mangled
    names that call `iscc-lib` core and return JNI types (`jstring`, `jobject`, `jbyteArray`).
    Single file, ~1060 lines.
2. **Java wrapper** (`crates/iscc-jni/java/src/main/java/io/iscc/iscc_lib/`): Static API class
    (`IsccLib`), result types (`SumCodeResult`, `IsccDecodeResult`), and automatic native library
    loader (`NativeLoader`). Provides idiomatic Java DX with Javadoc.

**Why JNI (not JNA or Panama FFI):**

- JNI is the standard, universally supported native interface for all JVM versions >= 8
- No runtime dependency on third-party FFI library
- Direct Rust-to-Java type marshalling via the `jni` crate (v0.21)
- Panama Foreign Function API requires Java 22+ — JNI ensures JDK 17+ (LTS) compatibility

## Crate/Package Structure

```
crates/iscc-jni/
├── Cargo.toml                              # cdylib, depends on iscc-lib + jni
├── README.md                               # Per-crate README for Maven Central
├── src/
│   └── lib.rs                              # JNI bridge (~1060 lines)
└── java/
    ├── pom.xml                             # Maven project (io.iscc:iscc-lib)
    └── src/
        ├── main/java/io/iscc/iscc_lib/
        │   ├── IsccLib.java                # Static API class with native methods
        │   ├── NativeLoader.java           # Platform-aware native library loader
        │   ├── SumCodeResult.java          # Result type for genSumCodeV0
        │   └── IsccDecodeResult.java       # Result type for isccDecode
        └── test/java/io/iscc/iscc_lib/
            └── IsccLibTest.java            # JUnit 5 conformance tests
```

## Rust JNI Bridge Layer

### JNI Function Naming

JNI mangled names encode the Java package. For `io.iscc.iscc_lib.IsccLib`, the underscore in
`iscc_lib` is encoded as `_1`:

```
Java_io_iscc_iscc_1lib_IsccLib_<methodName>
```

### Error Handling

- **`throw_and_default<T>`**: Throws `IllegalArgumentException` for invalid input, returns default
- **`throw_state_error<T>`**: Throws `IllegalStateException` for invalid state (e.g., finalized
    hasher)

Both use `env.throw_new()` — no Rust panics cross the JNI boundary.

### Streaming Hashers (Opaque Pointer Pattern)

`DataHasher` and `InstanceHasher` use `Box::into_raw()` cast to `jlong`:

```rust
struct JniDataHasher { inner: Option<iscc_lib::DataHasher> }
// new() → Box::into_raw() → jlong
// update() → cast jlong back → call inner.update()
// finalize() → Option::take() (one-shot semantics)
// free() → Box::from_raw() reclaims memory
```

## Java Wrapper Layer

### Static API Class (`IsccLib.java`)

All methods are `public static native`. Native library loaded eagerly via
`static { NativeLoader.load(); }`.

**Constants** as `public static final`:

| Constant                | Value       |
| ----------------------- | ----------- |
| `META_TRIM_NAME`        | `128`       |
| `META_TRIM_DESCRIPTION` | `4096`      |
| `META_TRIM_META`        | `128_000`   |
| `IO_READ_SIZE`          | `4_194_304` |
| `TEXT_NGRAM_SIZE`       | `13`        |

### Target DX

```java
import io.iscc.iscc_lib.IsccLib;
import io.iscc.iscc_lib.SumCodeResult;

// Code generation
String iscc = IsccLib.genMetaCodeV0("Title", "Description", null, 64);

// File-based sum code
SumCodeResult result = IsccLib.genSumCodeV0("/path/to/file.bin", 64, false, true);
System.out.println(result.iscc);       // "ISCC:KAC..."
System.out.println(result.datahash);   // "1e20..."
System.out.println(result.filesize);   // 123456

// Streaming
long ptr = IsccLib.dataHasherNew();
try {
    IsccLib.dataHasherUpdate(ptr, chunk);
    String code = IsccLib.dataHasherFinalize(ptr, 64);
} finally {
    IsccLib.dataHasherFree(ptr);
}
```

## Native Library Loading

`NativeLoader.java` implements two strategies:

1. **JAR resource extraction** (production): Extracts from `META-INF/native/{os}-{arch}/{libname}`
    to temp directory, loads via `System.load()`
2. **System library path** (dev/CI): Falls back to `System.loadLibrary("iscc_jni")` using
    `java.library.path`

| OS / Arch        | Resource path                                     |
| ---------------- | ------------------------------------------------- |
| linux / x86_64   | `META-INF/native/linux-x86_64/libiscc_jni.so`     |
| linux / aarch64  | `META-INF/native/linux-aarch64/libiscc_jni.so`    |
| macOS / arm64    | `META-INF/native/macos-aarch64/libiscc_jni.dylib` |
| macOS / x86_64   | `META-INF/native/macos-x86_64/libiscc_jni.dylib`  |
| windows / x86_64 | `META-INF/native/windows-x86_64/iscc_jni.dll`     |

## Distribution / Publishing (Maven Central)

**Coordinates:** `io.iscc:iscc-lib`

**Maven Central requirements** (handled by `release` profile in `pom.xml`):

1. Source JAR (`maven-source-plugin`)
2. Javadoc JAR (`maven-javadoc-plugin`)
3. GPG signing (`maven-gpg-plugin`)
4. Central publishing (`central-publishing-maven-plugin` with `autoPublish=true`)

**Secrets:**

| Secret                  | Purpose                                   |
| ----------------------- | ----------------------------------------- |
| `MAVEN_USERNAME`        | Sonatype Central Portal token username    |
| `MAVEN_PASSWORD`        | Sonatype Central Portal token password    |
| `MAVEN_GPG_PRIVATE_KEY` | ASCII-armored GPG private key for signing |
| `MAVEN_GPG_PASSPHRASE`  | GPG key passphrase                        |

## CI Integration

### CI Job (ci.yml)

```yaml
java:
  runs-on: ubuntu-latest
  steps:
    - uses: actions/checkout@v4
    - uses: dtolnay/rust-toolchain@stable
    - uses: actions/setup-java@v4
      with:
        distribution: temurin
        java-version: '17'
    - run: cargo build -p iscc-jni
    - run: mvn test -f crates/iscc-jni/java/pom.xml
```

Maven Surefire sets `java.library.path` to `target/debug` for CI testing.

### Release Jobs

1. **`build-jni`**: Cross-compile native libraries for 5 platforms
2. **`assemble-jar`**: Bundle all native libraries into JAR under `META-INF/native/`
3. **`publish-maven`**: GPG sign and deploy to Maven Central (with version-already-published check)

## DevContainer Setup

```dockerfile
RUN apt-get update && apt-get install -y --no-install-recommends \
    openjdk-17-jdk-headless maven \
    && apt-get clean && rm -rf /var/lib/apt/lists/*
```

## Version Sync

| Target                         | What is synced                          |
| ------------------------------ | --------------------------------------- |
| `crates/iscc-jni/java/pom.xml` | `<version>X.Y.Z</version>`              |
| `crates/iscc-jni/README.md`    | Maven `<version>` in dependency snippet |

## Documentation

- **How-to guide**: `docs/howto/java.md`
- **API reference**: `docs/java-api.md`
- **Per-crate README**: `crates/iscc-jni/README.md`

## Verification Criteria

- [ ] `mvn test` passes (65 tests) with all conformance vectors
- [ ] All 10 `gen_*_v0` functions return correct ISCC strings matching `data.json`
- [ ] All 32 Tier 1 symbols accessible from Java
- [ ] `DataHasher` and `InstanceHasher` streaming types work correctly
- [ ] Streaming hashers throw `IllegalStateException` after finalization
- [ ] Invalid inputs throw `IllegalArgumentException` (not Rust panics)
- [ ] `NativeLoader` extracts correct native library from JAR on all 5 platforms
- [ ] JAR published to Maven Central with source and Javadoc JARs + GPG signatures
- [ ] Duplicate version publish detected and skipped gracefully
- [ ] Version synced from root `Cargo.toml` via `mise run version:sync`
- [ ] Per-crate README renders correctly
- [ ] Documentation site includes Java how-to guide and API reference
