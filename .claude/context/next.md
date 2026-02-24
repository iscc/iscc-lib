# Next Work Package

## Step: Create Java wrapper class and Maven build config

## Goal

Create the Java-side source tree for the JNI bindings: a Java class with all 29 native method
declarations matching the Rust JNI bridge, a Maven `pom.xml` for compilation, and JDK 17 in the
devcontainer. This establishes the Java compilation target that all subsequent Java work (tests, CI,
packaging) builds on.

## Scope

- **Create**: `crates/iscc-jni/java/pom.xml`,
    `crates/iscc-jni/java/src/main/java/io/iscc/iscc_lib/IsccLib.java`
- **Modify**: `.devcontainer/Dockerfile`
- **Reference**: `crates/iscc-jni/src/lib.rs` (all 29 JNI function signatures),
    `crates/iscc-jni/Cargo.toml`

## Not In Scope

- `NativeLoader.java` for extracting platform-specific `.so`/`.dll`/`.dylib` from `META-INF/native/`
    — that's a packaging concern for JAR distribution and belongs in a future step
- Java tests (`src/test/java/`) — separate step after the wrapper compiles
- Java CI job in `ci.yml` — needs tests first
- Idiomatic Java wrapper classes (e.g., `DataHasher`/`InstanceHasher` as Java classes with
    try-with-resources) — start with raw native methods, add sugar later
- Maven/Gradle publishing config (signing, Sonatype, Maven Central) — deferred
- `mise.toml` tool entries for JDK/Maven — mise isn't used for Java in CI, and the devcontainer
    provides the tools directly
- README or docs updates for Java — separate step

## Implementation Notes

**Devcontainer:** Add `openjdk-17-jdk-headless maven` to the `apt-get install` line in the
Dockerfile's system packages section. For the current session, the advance agent should run
`sudo apt-get update && sudo apt-get install -y openjdk-17-jdk-headless maven` before attempting
`mvn compile`.

**IsccLib.java:** Package `io.iscc.iscc_lib`. All methods are `public static native`. The class has
a static initializer block `static { System.loadLibrary("iscc_jni"); }`. The 29 native methods must
match the JNI function names in `lib.rs` exactly (Java name is the part after
`Java_io_iscc_iscc_1lib_IsccLib_`):

| JNI method name          | Java signature                                                                 |
| ------------------------ | ------------------------------------------------------------------------------ |
| `conformanceSelftest`    | `boolean conformanceSelftest()`                                                |
| `genMetaCodeV0`          | `String genMetaCodeV0(String name, String description, String meta, int bits)` |
| `genTextCodeV0`          | `String genTextCodeV0(String text, int bits)`                                  |
| `genImageCodeV0`         | `String genImageCodeV0(byte[] pixels, int bits)`                               |
| `genAudioCodeV0`         | `String genAudioCodeV0(int[] cv, int bits)`                                    |
| `genVideoCodeV0`         | `String genVideoCodeV0(int[][] frameSigs, int bits)`                           |
| `genMixedCodeV0`         | `String genMixedCodeV0(String[] codes, int bits)`                              |
| `genDataCodeV0`          | `String genDataCodeV0(byte[] data, int bits)`                                  |
| `genInstanceCodeV0`      | `String genInstanceCodeV0(byte[] data, int bits)`                              |
| `genIsccCodeV0`          | `String genIsccCodeV0(String[] codes, boolean wide)`                           |
| `textClean`              | `String textClean(String text)`                                                |
| `textRemoveNewlines`     | `String textRemoveNewlines(String text)`                                       |
| `textTrim`               | `String textTrim(String text, int nbytes)`                                     |
| `textCollapse`           | `String textCollapse(String text)`                                             |
| `encodeBase64`           | `String encodeBase64(byte[] data)`                                             |
| `isccDecompose`          | `String[] isccDecompose(String isccCode)`                                      |
| `slidingWindow`          | `String[] slidingWindow(String seq, int width)`                                |
| `algSimhash`             | `byte[] algSimhash(byte[][] hashDigests)`                                      |
| `algMinhash256`          | `byte[] algMinhash256(int[] features)`                                         |
| `algCdcChunks`           | `byte[][] algCdcChunks(byte[] data, boolean utf32, int avgChunkSize)`          |
| `softHashVideoV0`        | `byte[] softHashVideoV0(int[][] frameSigs, int bits)`                          |
| `dataHasherNew`          | `long dataHasherNew()`                                                         |
| `dataHasherUpdate`       | `void dataHasherUpdate(long ptr, byte[] data)`                                 |
| `dataHasherFinalize`     | `String dataHasherFinalize(long ptr, int bits)`                                |
| `dataHasherFree`         | `void dataHasherFree(long ptr)`                                                |
| `instanceHasherNew`      | `long instanceHasherNew()`                                                     |
| `instanceHasherUpdate`   | `void instanceHasherUpdate(long ptr, byte[] data)`                             |
| `instanceHasherFinalize` | `String instanceHasherFinalize(long ptr, int bits)`                            |
| `instanceHasherFree`     | `void instanceHasherFree(long ptr)`                                            |

Add a Javadoc class comment explaining that this is the low-level JNI interface to `iscc-lib`. Each
native method should have a brief Javadoc comment. Group methods into sections (gen functions, text
utilities, encoding, codec, sliding window, algorithm primitives, streaming hashers) matching the
Rust source.

**pom.xml:** Minimal Maven config:

- `groupId`: `io.iscc`
- `artifactId`: `iscc-lib`
- `version`: `0.0.1-SNAPSHOT`
- `packaging`: `jar`
- JDK 17 source/target via `maven.compiler.source`/`maven.compiler.target` properties = `17`
- JUnit 5 (`junit-jupiter`) in test scope (prepares for future test step)
- No plugins beyond `maven-compiler-plugin`
- UTF-8 encoding via `project.build.sourceEncoding` property

## Verification

- `sudo apt-get install -y openjdk-17-jdk-headless maven` succeeds (or JDK already present)
- `mvn compile -f crates/iscc-jni/java/pom.xml` exits 0
- `grep -c 'native ' crates/iscc-jni/java/src/main/java/io/iscc/iscc_lib/IsccLib.java` outputs `29`
- `cargo clippy --workspace --all-targets -- -D warnings` still passes (no Rust changes, confirm no
    regression)
- `.devcontainer/Dockerfile` contains `openjdk-17-jdk-headless` and `maven`

## Done When

`mvn compile` succeeds on the Java wrapper class containing all 29 native method declarations that
match the JNI bridge signatures in `crates/iscc-jni/src/lib.rs`, and the devcontainer Dockerfile
includes JDK 17 and Maven.
