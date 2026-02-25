# Next Work Package

## Step: Create Java how-to guide and add to navigation

## Goal

Create `docs/howto/java.md` — the last remaining how-to guide — and add its navigation entry to
`zensical.toml`. This completes the documentation set for all 6 language bindings.

## Scope

- **Create**: `docs/howto/java.md`
- **Modify**: `zensical.toml` (add Java entry to How-to Guides navigation)
- **Reference**: `docs/howto/go.md` (structural template — follow the same section pattern),
    `crates/iscc-jni/java/src/main/java/io/iscc/iscc_lib/IsccLib.java` (API signatures and Javadoc),
    `crates/iscc-jni/java/pom.xml` (Maven coordinates and JDK version),
    `crates/iscc-jni/java/src/test/java/io/iscc/iscc_lib/IsccLibTest.java` (usage examples from
    tests)

## Not In Scope

- Native library loader class (extracts `.so`/`.dll`/`.dylib` from JAR) — tracked separately
- Maven Central publishing configuration — not yet wired
- Gradle build examples — Maven is the primary build tool; Gradle can be added later
- Fixing the `IllegalArgumentException` vs `IllegalStateException` issue — tracked in issues.md
- Creating a Java API reference page (like `api.md` for Python) — future work
- Changing the `io.iscc:iscc-lib` dependency coordinates (they match `pom.xml` as-is)

## Implementation Notes

Follow the Go how-to guide (`docs/howto/go.md`) as the structural template. The Java guide should
have these sections:

1. **Front matter** — `icon: lucide/coffee`, `description:` line for Java
2. **Intro paragraph** — explain that `iscc-lib` provides Java bindings via JNI with a native
    shared library loaded via `System.loadLibrary("iscc_jni")`
3. **Installation** — Maven dependency snippet (`io.iscc:iscc-lib:0.0.1`). Note: currently requires
    building from source and setting `java.library.path` — Maven Central publishing is not yet
    available. Show the `cargo build -p iscc-jni` + `mvn test` build-from-source workflow
4. **Setup** — Since Java uses static methods (no runtime object like Go), this section covers the
    `System.loadLibrary` call and `java.library.path` configuration
5. **Code generation** — All 9 `gen*V0` methods with Java examples. Use `IsccLib.java` for exact
    signatures. All gen functions return `String` (the ISCC code). Use `null` for optional
    parameters (description, meta). Default `bits` is 64
6. **Streaming** — `DataHasher` and `InstanceHasher` lifecycle using opaque `long` handles:
    `dataHasherNew()` → `dataHasherUpdate(ptr, data)` → `dataHasherFinalize(ptr, bits)` →
    `dataHasherFree(ptr)`. Emphasize the try-finally pattern for `*Free` calls
7. **Text utilities** — `textClean`, `textRemoveNewlines`, `textTrim`, `textCollapse`
8. **Algorithm primitives** — `algSimhash`, `algMinhash256`, `algCdcChunks`, `softHashVideoV0`
9. **Conformance testing** — `conformanceSelftest()` returning boolean
10. **Error handling** — `IllegalArgumentException` thrown on invalid input; try-catch pattern

Key differences from Go guide:

- No "Runtime setup" section (Java uses static methods, not a runtime object)
- Instead, a "Setup" section explaining `System.loadLibrary` and build-from-source
- Streaming hashers use opaque `long` handles (not struct types) — must emphasize memory management
    with try-finally
- All method names are camelCase (Java convention), not PascalCase (Go convention)
- `byte[]` instead of `[]byte`, `int[]` instead of `[]int32`, `String[]` instead of `[]string`

Insert the Java nav entry in `zensical.toml` after the Go entry:

```toml
{ "Java" = "howto/java.md" },
```

Target length: ~300-400 lines (consistent with existing guides: Go 388, Rust 356, Python 353, WASM
338, Node.js 281).

Run `mise run format` before committing to satisfy pre-commit hooks (mdformat, etc.).

## Verification

- `test -f docs/howto/java.md` exits 0
- `grep 'io.iscc' docs/howto/java.md` exits 0 (Maven coordinates present)
- `grep 'System.loadLibrary' docs/howto/java.md` exits 0 (native library setup documented)
- `grep 'genMetaCodeV0' docs/howto/java.md` exits 0 (code generation documented)
- `grep 'dataHasherNew' docs/howto/java.md` exits 0 (streaming documented)
- `grep 'textClean' docs/howto/java.md` exits 0 (text utilities documented)
- `grep 'conformanceSelftest' docs/howto/java.md` exits 0 (conformance testing documented)
- `grep '"Java"' zensical.toml` exits 0 (nav entry present)
- `grep 'howto/java.md' zensical.toml` exits 0 (nav path correct)
- `uv run zensical build` exits 0 (site builds successfully with Java page)

## Done When

All 10 verification commands exit 0, confirming the Java how-to guide exists with all required
sections and the navigation entry is live.
