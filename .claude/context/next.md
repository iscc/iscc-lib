# Next Work Package

## Step: Add Java sections to root README

## Goal

Update the root README.md to include Java/Maven installation instructions, a Java quick start code
example, and fix the Key Features line to mention Java. This addresses three target gaps in one
coherent change: the Key Features inaccuracy ("Python, Node.js, WASM, and C FFI" — Java missing),
the missing Java installation section, and the missing Java quick start example.

## Scope

- **Create**: (none)
- **Modify**: `README.md`
- **Reference**:
    - `crates/iscc-jni/java/src/main/java/io/iscc/iscc_lib/IsccLib.java` — Java API surface and method
        signatures (camelCase names, parameter types)
    - `crates/iscc-jni/java/pom.xml` — Maven coordinates (`io.iscc`, `iscc-lib`, version)
    - `crates/iscc-jni/README.md` — per-crate README with Java quick start example for consistency

## Not In Scope

- Adding Go sections to the README — Go bindings are not started; adding placeholder sections for
    non-existent functionality would mislead developers
- Creating `docs/howto/java.md` — separate step, different file and nav config
- Java native loader class — code task, separate from documentation
- Updating the "What is iscc-lib" paragraph to mention Java — the current text says "Python,
    Node.js, WebAssembly, and C" which could be updated, but scoping to the three specific target
    gaps keeps this focused
- Adding Maven Central badge — package is not yet published to Maven Central, so no version badge
    exists

## Implementation Notes

**Key Features line (line 23):** Change "Python, Node.js, WASM, and C FFI" to "Python, Java,
Node.js, WASM, and C FFI" — add Java after Python (following the order used in target.md: "Rust +
Python + Java + Node.js + WASM + C FFI").

**Installation section:** Add a `### Java` subsection after Node.js (line 91) and before WASM. Use a
Maven `<dependency>` XML snippet:

```xml
<dependency>
  <groupId>io.iscc</groupId>
  <artifactId>iscc-lib</artifactId>
  <version>0.0.1</version>
</dependency>
```

Add a note that the native library must be on `java.library.path` (since the native loader class is
not yet implemented). This matches the caveat in `crates/iscc-jni/README.md`.

**Quick Start section:** Add a `### Java` subsection after Node.js and before WASM. Show
`genMetaCodeV0` usage with camelCase Java method names from `IsccLib.java`:

```java
import io.iscc.iscc_lib.IsccLib;

String result = IsccLib.genMetaCodeV0("ISCC Test Document!", null, null, 64);
System.out.println("Meta-Code: " + result);
```

**Section ordering:** Insert Java after Node.js and before WASM in both Installation and Quick Start
sections. This keeps the language-specific sections in logical order: native languages first (Rust,
Python, Node.js, Java), then WASM.

**Style:** Match the existing README tone and formatting. No emojis. Keep subsections concise — the
Installation subsection is 3-5 lines like the existing ones, the Quick Start subsection is a short
code block like the existing ones.

## Verification

- `grep -q 'Python, Java, Node.js, WASM, and C FFI' README.md` exits 0 (Key Features updated)
- `grep -q '### Java' README.md` exits 0 (Java subsections exist)
- `grep -q 'io.iscc' README.md` exits 0 (Maven groupId present)
- `grep -q 'genMetaCodeV0' README.md` exits 0 (Java quick start uses correct method name)
- `grep -c '### Java' README.md` returns 2 (one in Installation, one in Quick Start)
- `grep -q 'java.library.path' README.md` exits 0 (native library caveat present)
- `mise run check` passes all pre-commit hooks (formatting, linting)

## Done When

README.md contains Java installation (Maven dependency XML), Java quick start (genMetaCodeV0
example), updated Key Features line mentioning Java, and all verification commands pass.
