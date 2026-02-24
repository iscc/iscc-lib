# Next Work Package

## Step: Create Java conformance tests

## Goal

Create JUnit 5 conformance tests that exercise all 9 `gen_*_v0` JNI functions against the official
`data.json` test vectors, validating the entire Rust→JNI→Java bridge end-to-end.

## Scope

- **Create**: `crates/iscc-jni/java/src/test/java/io/iscc/iscc_lib/IsccLibTest.java`
- **Modify**: `crates/iscc-jni/java/pom.xml` (add Gson test dependency, Surefire plugin with
    `java.library.path`)
- **Reference**:
    - `crates/iscc-napi/__tests__/conformance.test.mjs` — existing conformance test pattern to mirror
    - `crates/iscc-lib/tests/data.json` — the authoritative test vectors
    - `crates/iscc-jni/java/src/main/java/io/iscc/iscc_lib/IsccLib.java` — Java API signatures
    - `crates/iscc-jni/src/lib.rs` — JNI bridge for understanding parameter mapping

## Not In Scope

- Native library loader class (extracts platform `.so`/`.dll`/`.dylib` from `META-INF/native/`) —
    that's a separate packaging step
- Java CI job in `.github/workflows/ci.yml` — needs working tests first, add in a follow-up step
- README Java sections or `docs/howto/java.md` — documentation comes after tests validate the bridge
- Testing streaming hashers (`DataHasher`/`InstanceHasher`) via JNI opaque handles — focus on the 9
    `gen_*_v0` functions first; streaming tests can follow
- Refactoring `IsccLib.java` or the JNI bridge — the test validates the existing code as-is
- Testing text utilities, algorithm primitives, or codec functions — conformance vectors only cover
    the 9 gen functions

## Implementation Notes

**JSON parsing:** Add `com.google.code.gson:gson` as a `<scope>test</scope>` dependency in
`pom.xml`. Gson handles the nested arrays in `data.json` (video frame sigs are `int[][]`, mixed
codes are `String[]`) with minimal boilerplate. Use `JsonObject`/`JsonArray` for traversal.

**data.json path:** Maven runs tests from the `pom.xml` directory (`crates/iscc-jni/java/`). The
relative path to data.json is `../../iscc-lib/tests/data.json`. Read it with
`Files.readString(Path.of("../../iscc-lib/tests/data.json"))` in a `@BeforeAll` setup method.

**Native library path:** Add `maven-surefire-plugin` to `pom.xml` with:

```xml
<plugin>
  <groupId>org.apache.maven.plugins</groupId>
  <artifactId>maven-surefire-plugin</artifactId>
  <version>3.5.2</version>
  <configuration>
    <argLine>-Djava.library.path=${project.basedir}/../../../target/debug</argLine>
  </configuration>
</plugin>
```

This points at the workspace's `target/debug/` directory where `cargo build -p iscc-jni` places
`libiscc_jni.so` (Linux) / `libiscc_jni.dylib` (macOS) / `iscc_jni.dll` (Windows).

**Test structure:** Use JUnit 5 `@TestFactory` returning `Collection<DynamicTest>` — this is the
idiomatic JUnit 5 pattern for data-driven tests and produces readable test names matching the JSON
keys (e.g., `test_0001_title_only`). One `@TestFactory` method per gen function, all in a single
test class.

**Input mapping per function:**

- `genMetaCodeV0(name, description, meta, bits)` — `inputs[0]` is name (String), `inputs[1]` is
    description (String, empty string → pass `""` since the JNI bridge handles it), `inputs[2]` is
    meta (nullable JSON value — if it's a JSON object, serialize with sorted keys; if null, pass
    `null`; if string, pass as-is), `inputs[3]` is bits (int)
- `genTextCodeV0(text, bits)` — `inputs[0]` String, `inputs[1]` int
- `genImageCodeV0(pixels, bits)` — `inputs[0]` is a JSON array of ints (0-255) → convert to `byte[]`
    (cast each int to byte)
- `genAudioCodeV0(cv, bits)` — `inputs[0]` is JSON array of signed ints → `int[]`
- `genVideoCodeV0(frameSigs, bits)` — `inputs[0]` is `int[][]` (array of frame signature arrays)
- `genMixedCodeV0(codes, bits)` — `inputs[0]` is `String[]` (array of ISCC code strings)
- `genDataCodeV0(data, bits)` — `inputs[0]` is `"stream:<hex>"` → decode hex to `byte[]`
- `genInstanceCodeV0(data, bits)` — same `"stream:<hex>"` decoding
- `genIsccCodeV0(codes, wide)` — `inputs[0]` is `String[]`, no `wide` in test vectors → pass `false`

**Meta argument handling:** When `inputs[2]` is a `JsonObject` (not a string and not null),
serialize it with sorted keys using `TreeMap` to sort entries before `Gson.toJson()`. When it's
`JsonNull` or null, pass `null`. When it's a `JsonPrimitive` string, pass `getAsString()`.

**Stream hex decoding:** Parse `"stream:<hex>"` by stripping the `"stream:"` prefix, then converting
the hex string to `byte[]` using `HexFormat.of().parseHex()` (Java 17+). Empty hex after prefix
means empty `byte[]` (`new byte[0]`).

**Build prerequisite:** Before running `mvn test`, the native library must be built with
`cargo build -p iscc-jni`. The verification commands chain these.

## Verification

- `cargo build -p iscc-jni` exits 0 (native library built)
- `mvn test -f crates/iscc-jni/java/pom.xml` passes all conformance tests
- Test output shows all 9 `gen_*_v0` function groups with individual test vectors passing
- `cargo clippy --workspace --all-targets -- -D warnings` remains clean

## Done When

All 9 `gen_*_v0` conformance test groups pass via `mvn test` after building the native library with
`cargo build -p iscc-jni`, and workspace clippy remains clean.
