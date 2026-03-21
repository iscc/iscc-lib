# Next Work Package

## Step: Kotlin documentation — howto guide, package README/CLAUDE.md, root README integration

## Goal

Create all Kotlin documentation artifacts (howto guide, package README, package CLAUDE.md) and
integrate Kotlin into the root README and docs navigation. This completes the documentation sub-task
of the Kotlin bindings issue, leaving only the release workflow for a future step.

## Scope

- **Create**: `docs/howto/kotlin.md`, `packages/kotlin/README.md`, `packages/kotlin/CLAUDE.md`
- **Modify**: `README.md` (add Kotlin install + quickstart sections), `zensical.toml` (add nav
    entry), `scripts/gen_llms_full.py` (add ORDERED_PAGES entry)
- **Reference**: `docs/howto/swift.md` (template for howto guide — both are UniFFI-generated),
    `packages/swift/README.md` (template for package README), `packages/swift/CLAUDE.md` (template
    for package CLAUDE.md), `packages/kotlin/build.gradle.kts` (for install/dependency syntax),
    `packages/kotlin/src/main/kotlin/uniffi/iscc_uniffi/iscc_uniffi.kt` (for API details/naming)

## Not In Scope

- Release workflow (`release.yml` maven-kotlin job) — separate step
- Kotlin/Native or KMP-specific documentation — current binding is JVM-only
- Fixing Swift SPM install instructions — separate normal-priority issue
- Modifying any Kotlin source code, build config, or tests
- Adding Kotlin to the polyglot badge line (already listed in README line 26)

## Implementation Notes

### docs/howto/kotlin.md (~400-425 lines)

Follow `docs/howto/swift.md` structure exactly (both are UniFFI-generated bindings, very parallel):

- **Frontmatter**: `icon: lucide/compass`, description mentioning Kotlin

- **Intro paragraph**: mention UniFFI-generated bindings, `uniffi.iscc_uniffi` package, JNA loading

- **Installation**: Gradle (Kotlin DSL) dependency block:

    ```kotlin
    dependencies {
        implementation("io.iscc:iscc-lib-kotlin:0.3.1")
    }
    ```

    Note: the package is not yet published to Maven Central — add a note about building from source
    for now (cargo build + gradlew test). Include the JNA native lib path requirement

- **Sections** (mirror Swift howto, adapting syntax):

    1. Installation (Gradle dependency + build from source)
    2. Meta-Code generation (`genMetaCodeV0` — UniFFI camelCase)
    3. Text-Code generation
    4. Image-Code, Audio-Code, Video-Code, Mixed-Code generation
    5. Data-Code generation (streaming via `DataHasher`)
    6. Instance-Code generation (streaming via `InstanceHasher`)
    7. ISCC-Code generation
    8. ISCC-SUM generation (`genSumCodeV0`)
    9. Codec operations (decompose, encode, decode)
    10. Text utilities
    11. Algorithm primitives
    12. Constants
    13. Conformance selftest

- **Kotlin API naming**: UniFFI generates camelCase function names (`genMetaCodeV0`, `textClean`,
    `slidingWindow`, etc.). Result types are data classes in `uniffi.iscc_uniffi` package (e.g.,
    `MetaCodeResult`, `TextCodeResult`). Check the generated `iscc_uniffi.kt` for exact names and
    signatures.

- **Streaming pattern**: `DataHasher()` / `InstanceHasher()` objects with `.push(data)` and
    `.finalize()` methods (same as Swift but Kotlin syntax)

- **Byte data**: Kotlin uses `ByteArray` (not `List<Byte>`). UniFFI maps `Vec<u8>` to
    `kotlin.ByteArray`

- **Error handling**: UniFFI throws `IsccUniException` — use try/catch in examples

### packages/kotlin/README.md (~70 lines)

Follow `packages/swift/README.md` structure:

- Package name + tagline
- Installation (Gradle dependency)
- Usage example (`genMetaCodeV0`)
- API overview (10 gen functions + utilities)
- Build from source instructions (cargo build + gradlew)
- Links to docs site, repository, ISO spec
- License: Apache-2.0

### packages/kotlin/CLAUDE.md (~100-110 lines)

Follow `packages/swift/CLAUDE.md` structure:

- Package role (UniFFI-generated Kotlin/JVM bindings)
- File layout (build.gradle.kts, src/main/kotlin, src/test/kotlin, gradle wrapper)
- Build and test commands (./gradlew build, ./gradlew test)
- Key details: JNA loading, JVM-only (no Kotlin/Native), UniFFI naming conventions
- Conformance testing (JUnit 5 + Gson, data.json in src/test/resources/)
- Version sync target (gradle.properties)

### Root README.md modifications

Add Kotlin sections in the same position pattern as Swift (between Swift and WASM):

**Installation section** — add after Swift, before WASM:

```
### <img src="https://cdn.simpleicons.org/kotlin/7F52FF" width="20" height="20" alt="Kotlin"> Kotlin
```

With Gradle Kotlin DSL dependency block.

**Quick Start section** — add after Swift, before WASM:

```
### <img src="https://cdn.simpleicons.org/kotlin/7F52FF" width="20" height="20" alt="Kotlin"> Kotlin
```

With import + genMetaCodeV0 example using named parameters.

### zensical.toml

Add after Swift entry in the nav How-To Guides section:

```toml
{ "Kotlin" = "howto/kotlin.md" },
```

### scripts/gen_llms_full.py

Add after `"howto/swift.md"` in ORDERED_PAGES list:

```python
("howto/kotlin.md",)
```

## Verification

- `test -f docs/howto/kotlin.md` — howto guide exists
- `test -f packages/kotlin/README.md` — package README exists
- `test -f packages/kotlin/CLAUDE.md` — package CLAUDE.md exists
- `grep -q 'kotlin' zensical.toml` — nav entry present
- `grep -q 'kotlin' scripts/gen_llms_full.py` — ORDERED_PAGES entry present
- `grep -q 'Kotlin' README.md` — root README has Kotlin sections
- `grep -c 'Kotlin' README.md` returns at least 4 (install heading, quickstart heading, polyglot
    line, at least one more)
- `uv run python scripts/gen_llms_full.py && test -f docs/llms-full.txt` — llms-full.txt regenerates
    successfully
- `mise run check` — all pre-commit hooks pass (formatting, linting)

## Done When

All verification criteria pass — Kotlin documentation is complete with howto guide, package
README/CLAUDE.md, root README integration, and docs nav/llms entries.
