# Next Work Package

## Step: Create per-crate READMEs (batch 2: iscc-wasm, iscc-jni)

## Goal

Create registry-facing README.md files for the two remaining publishable crates (`iscc-wasm` for npm
as `@iscc/wasm`, `iscc-jni` for Maven Central as `io.iscc:iscc-lib`). This completes per-crate
README coverage for all binding crates that publish to a package registry.

## Scope

- **Create**: `crates/iscc-wasm/README.md`, `crates/iscc-jni/README.md`
- **Modify**: (none — iscc-wasm has `publish = false` in Cargo.toml and publishes via npm/wasm-pack,
    so no Cargo.toml readme field needed; iscc-jni publishes via Maven, not crates.io)
- **Reference**:
    - `crates/iscc-lib/README.md` — batch 1 template (70 lines, 6 H2 sections)
    - `crates/iscc-napi/README.md` — batch 1 JS-ecosystem template
    - `crates/iscc-py/README.md` — batch 1 Python template (for streaming note pattern)
    - `crates/iscc-wasm/src/lib.rs` — WASM binding signatures
    - `crates/iscc-jni/java/src/main/java/io/iscc/iscc_lib/IsccLib.java` — Java API surface

## Not In Scope

- `crates/iscc-ffi/README.md` — iscc-ffi is not published to any registry (target spec says "not
    published separately"); lower priority, tackle only if all other README work is done
- Wiring `readme` fields in `Cargo.toml` — both crates have `publish = false` (they publish through
    npm and Maven respectively, not crates.io)
- Updating the root `README.md` with Java/Go sections — separate step
- Creating `docs/howto/java.md` — separate step
- Java native loader class — separate step
- Go bindings — not started yet, separate track

## Implementation Notes

Follow the exact template established by batch 1 (see `crates/iscc-lib/README.md` as the canonical
reference):

**Required H2 sections (6 total):** What is ISCC, Installation, Quick Start, API Overview, Links,
License.

**iscc-wasm README specifics:**

- Title: `@iscc/wasm`
- Badges: CI, npm (`@iscc/wasm`), License
- npm badge URL: `https://img.shields.io/npm/v/@iscc/wasm.svg`
- Experimental notice: same text as batch 1
- Tagline: mention browser-compatible WASM, built with Rust and wasm-bindgen
- Installation: `npm install @iscc/wasm`
- Quick start: ESM import pattern for WASM (not CommonJS `require()` — WASM typically uses async
    init). Show `gen_meta_code_v0` usage. Check `crates/iscc-wasm/src/lib.rs` for the exact export
    names
- Note that all code generators return ISCC strings directly (same pattern as iscc-napi)
- Note that both browser and Node.js targets are supported

**iscc-jni README specifics:**

- Title: `iscc-lib` (Java)
- Badges: CI, License (no Maven Central badge yet — package not yet published)
- Experimental notice: same text as batch 1
- Tagline: mention Java/JVM bindings via JNI, built with Rust
- Installation: Maven `<dependency>` XML snippet with `io.iscc` groupId, `iscc-lib` artifactId,
    version `0.0.1`
- Quick start: Java code showing `IsccLib.genMetaCodeV0("ISCC Test Document!")` — use the camelCase
    method names from `IsccLib.java`
- Note that the native library must currently be on `java.library.path` (no auto-loader yet)
- API Overview: list the 9 gen functions using Java camelCase names from IsccLib.java

**Shared "What is ISCC" text:** Reuse the exact paragraph from the batch 1 READMEs (all three use
identical text).

**Shared Links section:** Same 4 links as batch 1 (Documentation, Repository, ISCC Specification,
ISCC Foundation).

**Style notes:**

- Keep READMEs concise (70-80 lines each, matching batch 1). These are registry pages, not full docs
- Use consistent formatting across both READMEs — same section order, similar lengths
- No emojis. Use standard markdown headings and code blocks
- Use ISCC Foundation URL `https://iscc.io` (not other variants)

## Verification

- `test -f crates/iscc-wasm/README.md` exits 0
- `test -f crates/iscc-jni/README.md` exits 0
- `grep -c '^## ' crates/iscc-wasm/README.md` returns 6 (6 H2 sections)
- `grep -c '^## ' crates/iscc-jni/README.md` returns 6 (6 H2 sections)
- `grep -q '@iscc/wasm' crates/iscc-wasm/README.md` exits 0 (correct npm package name)
- `grep -q 'io.iscc' crates/iscc-jni/README.md` exits 0 (correct Maven groupId)
- `cargo clippy --workspace --all-targets -- -D warnings` clean (no code changes, sanity check)

## Done When

Both `crates/iscc-wasm/README.md` and `crates/iscc-jni/README.md` exist with 6 H2 sections each,
correct registry-specific installation instructions, quick start code examples, and all verification
commands pass.
