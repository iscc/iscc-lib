# Next Work Package

## Step: Implement Java NativeLoader class

## Goal

Create a `NativeLoader` class that automatically extracts the platform-specific native library from
the JAR's `META-INF/native/` directory and loads it, eliminating the need for manual
`-Djava.library.path` configuration. Update `IsccLib` to use it, with fallback to
`System.loadLibrary` for development/CI environments.

## Scope

- **Create**: `crates/iscc-jni/java/src/main/java/io/iscc/iscc_lib/NativeLoader.java`
- **Modify**: `crates/iscc-jni/java/src/main/java/io/iscc/iscc_lib/IsccLib.java` (static initializer
    and Javadoc)
- **Reference**: `crates/iscc-jni/java/pom.xml`, `crates/iscc-jni/README.md`,
    `crates/iscc-jni/java/src/test/java/io/iscc/iscc_lib/IsccLibTest.java`

## Not In Scope

- Bundling actual native binaries into the JAR (that's a CI/build pipeline step for a future
    iteration — the NativeLoader's JAR extraction path won't activate until binaries are bundled)
- Maven Central publishing configuration (Sonatype staging plugin, GPG signing, POM metadata)
- Multi-platform CI matrix for cross-compiling native libraries
- Updating the how-to guide (`docs/howto/java.md`) — defer until bundling is also done
- Adding new test classes — existing 49 conformance tests verify the loader works via fallback
- Changing NativeLoader to use `IllegalStateException` for state errors — tracked separately in
    issues.md

## Implementation Notes

**NativeLoader pattern** (well-established in JNI projects like sqlite-jdbc, netty-tcnative):

1. **Platform detection**: Detect OS via `System.getProperty("os.name")` and arch via
    `System.getProperty("os.arch")`. Normalize to canonical names:

    - OS: `linux`, `macos`, `windows` (detect via `os.name` prefix/contains)
    - Arch: `x86_64`, `aarch64` (normalize `amd64` → `x86_64`)

2. **Library naming**: Platform-specific library filename:

    - Linux: `libiscc_jni.so`
    - macOS: `libiscc_jni.dylib`
    - Windows: `iscc_jni.dll`

3. **Resource path**: `META-INF/native/{os}-{arch}/{libname}` (e.g.,
    `META-INF/native/linux-x86_64/libiscc_jni.so`)

4. **Loading strategy** (try in order): a. Try loading from JAR resource — use
    `NativeLoader.class.getResourceAsStream("/" + path)` to locate the resource. If found, extract
    to a temp file in a unique temp directory (`Files.createTempDirectory("iscc-jni-")`), call
    `System.load(absolutePath)`, mark temp file and directory for `deleteOnExit()` b. Fall back to
    `System.loadLibrary("iscc_jni")` — this covers dev/CI environments where the library is on
    `java.library.path` c. If both fail, throw `UnsatisfiedLinkError` with a descriptive message
    listing both attempted paths and the detected OS/arch

5. **Thread safety**: The `load()` method is `public static synchronized` with a
    `private static  volatile boolean loaded` guard. Since `IsccLib`'s static initializer calls it,
    class loading guarantees single execution, but the synchronized guard protects against direct
    calls from user code

6. **Temp file handling**: Use `Files.createTempDirectory("iscc-jni-")` to create an isolated
    directory per JVM instance. Copy the resource to a file with the original library name inside
    this directory (keeping the correct extension is important on some platforms). Use
    `file.deleteOnExit()` and `dir.deleteOnExit()` (directory deletion only works when empty, but
    the file gets deleted first)

**IsccLib changes**:

- Replace `System.loadLibrary("iscc_jni")` with `NativeLoader.load()` in the static block
- Update the class Javadoc to mention `NativeLoader` instead of `System.loadLibrary`

**Important**: Use `NativeLoader.class.getResourceAsStream()` (NOT
`ClassLoader.getSystemClassLoader()`) — this works correctly in shaded/fat JARs and OSGi containers
where resources are loaded from the calling class's classloader.

**CI compatibility**: The existing CI job sets `-Djava.library.path=target/debug` via Surefire
plugin, so the fallback `System.loadLibrary("iscc_jni")` path will be used. The JAR-extraction path
won't activate (no native libs in META-INF yet) but won't cause errors — it simply catches the
`NullPointerException`/`IOException` from `getResourceAsStream` returning null and falls through to
the working fallback.

## Verification

- `cd crates/iscc-jni && cargo build` succeeds (Rust side unchanged)
- `cd crates/iscc-jni/java && mvn test` passes all 49 existing tests (loader fallback path works)
- `test -f crates/iscc-jni/java/src/main/java/io/iscc/iscc_lib/NativeLoader.java` exits 0
- `grep 'NativeLoader.load' crates/iscc-jni/java/src/main/java/io/iscc/iscc_lib/IsccLib.java` exits
    0
- `grep -c 'System.loadLibrary' crates/iscc-jni/java/src/main/java/io/iscc/iscc_lib/IsccLib.java`
    outputs `0` (no direct loadLibrary in IsccLib)
- `grep 'META-INF/native' crates/iscc-jni/java/src/main/java/io/iscc/iscc_lib/NativeLoader.java`
    exits 0

## Done When

All 6 verification checks pass: NativeLoader.java exists with META-INF/native extraction logic,
IsccLib.java delegates to NativeLoader.load() with no direct System.loadLibrary, and all 49 existing
Maven tests pass via the fallback loading path.
