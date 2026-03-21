# Next Work Package

## Step: Add Kotlin conformance tests for all 9 gen\_\*\_v0 functions

## Goal

Add JUnit 5 conformance tests to the Kotlin JVM package that verify all 9 `gen_*_v0` functions
against the vendored `data.json` test vectors (50 vectors total). This is the highest-impact next
step for the Kotlin bindings issue — proving the UniFFI-generated Kotlin code produces correct
results.

## Scope

- **Create**: `packages/kotlin/src/test/kotlin/uniffi/iscc_uniffi/ConformanceTest.kt`,
    `packages/kotlin/src/test/resources/data.json` (copy from `crates/iscc-lib/tests/data.json`)
- **Modify**: `packages/kotlin/build.gradle.kts` (add JUnit 5 + Gson test dependencies)
- **Reference**: `packages/swift/Tests/IsccLibTests/ConformanceTests.swift` (UniFFI sibling
    pattern), `crates/iscc-jni/java/src/test/java/io/iscc/iscc_lib/IsccLibTest.java` (JVM JSON
    parsing pattern), `packages/kotlin/src/main/kotlin/uniffi/iscc_uniffi/iscc_uniffi.kt` (API
    signatures)

## Not In Scope

- CI job for Kotlin — that's a separate step after tests pass locally
- Smoke/unit tests beyond conformance vectors — keep scope minimal
- `io.iscc.lib` idiomatic wrapper package — tests call UniFFI-generated functions directly
- Version sync, documentation, README, or release workflow — all later steps
- KMP/Kotlin Native targets — JVM only for now

## Implementation Notes

**Dependencies to add to `build.gradle.kts`:**

```kotlin
testImplementation("org.junit.jupiter:junit-jupiter:5.11.4")
testImplementation("com.google.gson:gson:2.11.0")
```

The `useJUnitPlatform()` and `jvmArgs` for `java.library.path` are already present in
`build.gradle.kts`.

**Test file structure:** Follow the Swift conformance tests pattern — one test method per `gen_*_v0`
function, each loading vectors from the corresponding JSON key. Use Kotlin idioms but keep the
structure parallel to Swift for consistency across UniFFI bindings.

**UniFFI-generated function signatures (all in `uniffi.iscc_uniffi` package):**

- `genMetaCodeV0(name: String, description: String?, meta: String?, bits: UInt): MetaCodeResult`
- `genTextCodeV0(text: String, bits: UInt): TextCodeResult`
- `genImageCodeV0(pixels: ByteArray, bits: UInt): ImageCodeResult`
- `genAudioCodeV0(cv: List<Int>, bits: UInt): AudioCodeResult`
- `genVideoCodeV0(frameSigs: List<List<Int>>, bits: UInt): VideoCodeResult`
- `genMixedCodeV0(codes: List<String>, bits: UInt): MixedCodeResult`
- `genDataCodeV0(data: ByteArray, bits: UInt): DataCodeResult`
- `genInstanceCodeV0(data: ByteArray, bits: UInt): InstanceCodeResult`
- `genIsccCodeV0(codes: List<String>, wide: Boolean): IsccCodeResult`

All result types have an `iscc: String` field.

**Key type mappings for data.json parsing:**

- `bits` → `UInt` (Gson reads as Number, convert with `.toInt().toUInt()`)
- `pixels` (image) → `ByteArray` (JSON array of 0-255 ints, each cast to byte)
- `cv` (audio) → `List<Int>` (JSON array of i32 values)
- `frameSigs` (video) → `List<List<Int>>` (JSON 2D array of i32 values)
- `"stream:<hex>"` prefix → `ByteArray` (hex decode after stripping prefix, use `HexFormat`)
- `meta` → `String?` (null, string, or JSON object serialized with sorted keys via `TreeMap`)
- `gen_iscc_code_v0` vectors have no `wide` parameter — always pass `false`

**Expected vector counts:** 20+5+3+5+3+2+4+3+5 = 50 total.

**data.json:** Copy `crates/iscc-lib/tests/data.json` to
`packages/kotlin/src/test/resources/data.json`. Load in tests via
`javaClass.classLoader.getResourceAsStream("data.json")`.

**Runtime prerequisite:** `cargo build -p iscc-uniffi` must run before `./gradlew test` to produce
`libiscc_uniffi.so` in `target/debug/`. The `build.gradle.kts` already sets `java.library.path` to
`../../target/debug`.

## Verification

- `cargo build -p iscc-uniffi` exits 0 (produces `target/debug/libiscc_uniffi.so`)
- `cd packages/kotlin && ./gradlew test` exits 0 with all 9 test methods passing (50 vectors)
- `./gradlew test` output shows `9 tests successful`
- `test -f packages/kotlin/src/test/resources/data.json` exits 0
- `test -f packages/kotlin/src/test/kotlin/uniffi/iscc_uniffi/ConformanceTest.kt` exits 0

## Done When

All 9 conformance test methods pass against the vendored data.json vectors using `./gradlew test`,
confirming the UniFFI-generated Kotlin bindings produce correct ISCC codes.
