# Next Work Package

## Step: Swift documentation and README integration

## Goal

Complete the Swift bindings documentation: howto guide, README install/quickstart sections, docs nav
entry, ORDERED_PAGES registration, and per-package CLAUDE.md. This closes the remaining docs gap for
the Swift issue (CI is already green).

## Scope

- **Create**: `docs/howto/swift.md`, `packages/swift/CLAUDE.md`
- **Modify**: `README.md` (add Swift install + quickstart), `zensical.toml` (add Swift to nav),
    `scripts/gen_llms_full.py` (add `howto/swift.md` to ORDERED_PAGES)
- **Reference**: `docs/howto/dotnet.md` (structural template — same pattern for all howto guides),
    `packages/dotnet/CLAUDE.md` (structural template for per-package agent docs),
    `packages/swift/Sources/IsccLib/iscc_uniffi.swift` (Swift API signatures and types),
    `packages/swift/README.md` (existing Swift package README with usage examples)

## Not In Scope

- Version sync integration (`Constants.swift` + `scripts/version_sync.py` target) — separate step
- Kotlin bindings or documentation — depends on Swift being fully complete
- Swift API reference page in docs (e.g., `docs/swift-api.md`) — not present for other UniFFI
    bindings
- Updating the "Polyglot" bullet in Key Features beyond adding "Swift" to the language list

## Implementation Notes

### `docs/howto/swift.md`

Follow the exact structure of `docs/howto/dotnet.md`:

1. **Frontmatter**: `icon: lucide/compass`, description mentioning Swift
2. **Intro paragraph**: Swift bindings via UniFFI, `import IsccLib` to get started
3. **Installation**: SPM dependency in `Package.swift` (from `packages/swift/README.md`)
4. **Build from source** (collapsible tip): `cargo build -p iscc-uniffi` + `swift test` commands
5. **Code generation**: All 10 gen functions with Swift examples. Use Swift API names from
    `iscc_uniffi.swift`:
    - `genMetaCodeV0(name:description:meta:bits:)` → `MetaCodeResult`
    - `genTextCodeV0(text:bits:)` → `TextCodeResult`
    - `genImageCodeV0(pixels:bits:)` → `ImageCodeResult`
    - `genAudioCodeV0(cv:bits:)` → `AudioCodeResult`
    - `genVideoCodeV0(frameSigs:bits:)` → `VideoCodeResult`
    - `genMixedCodeV0(codes:bits:)` → `MixedCodeResult`
    - `genDataCodeV0(data:bits:)` → `DataCodeResult`
    - `genInstanceCodeV0(data:bits:)` → `InstanceCodeResult`
    - `genIsccCodeV0(codes:wide:)` → `IsccCodeResult`
    - `genSumCodeV0(path:bits:wide:addUnits:)` → `SumCodeResult`
6. **Structured results**: Table of result types with fields
7. **Streaming**: `DataHasher` and `InstanceHasher` — reference class (not struct, UniFFI objects
    are classes). Show `update(data:)` / `finalize(bits:)` pattern
8. **Codec operations**: `encodeComponent`, `isccDecode`, `isccDecompose`
9. **Text utilities**: `textClean`, `textRemoveNewlines`, `textTrim`, `textCollapse`
10. **Encoding utilities**: `encodeBase64`, `jsonToDataUrl`
11. **Algorithm primitives**: `slidingWindow`, `algSimhash`, `algMinhash256`, `algCdcChunks`,
    `softHashVideoV0`
12. **Constants**: getter functions `metaTrimName()`, `metaTrimDescription()`, `metaTrimMeta()`,
    `ioReadSize()`, `textNgramSize()` — note these are functions (UniFFI constraint), not
    properties
13. **Conformance testing**: `conformanceSelftest()`
14. **Error handling**: `IsccUniError` thrown by functions marked `throws`

Key Swift API patterns:

- All functions are camelCase free functions (not methods on a class)
- Binary data uses `Data` type (from Foundation)
- Functions that can fail use `throws` with `IsccUniError`
- Constants are getter functions (UniFFI can't export `const`)
- `DataHasher`/`InstanceHasher` are classes with `update(data:)`/`finalize(bits:)` methods

### `README.md`

Add Swift sections in two places:

1. **Installation** (after C / C++ section, before WASM): Use Swift logo from simpleicons
    (`https://cdn.simpleicons.org/swift/F05138`). Show SPM `Package.swift` dependency snippet
2. **Quick Start** (after C++ section, before WASM): Show `import IsccLib` +
    `try genMetaCodeV0(name:...)` example
3. **Key Features** "Polyglot" bullet: Add "Swift" to the language list

### `zensical.toml`

Add `{ "Swift" = "howto/swift.md" }` to the How-to Guides nav section (after C / C++, before the
closing bracket).

### `scripts/gen_llms_full.py`

Add `"howto/swift.md"` to `ORDERED_PAGES` list (after `"howto/c-cpp.md"`).

### `packages/swift/CLAUDE.md`

Follow `packages/dotnet/CLAUDE.md` structure: package role, file layout, build commands, test
patterns, publishing notes, common pitfalls. Key content:

- UniFFI-generated bindings (not P/Invoke)
- SPM distribution via Git tags (not registry upload)
- macOS runner required for CI (Swift not available on Linux CI)
- No `libiscc_uniffi` in devcontainer — Swift tests can only run on macOS

## Verification

- `test -f docs/howto/swift.md` exits 0
- `test -f packages/swift/CLAUDE.md` exits 0
- `grep -q 'Swift' zensical.toml` exits 0
- `grep -q 'howto/swift.md' zensical.toml` exits 0
- `grep -q 'howto/swift.md' scripts/gen_llms_full.py` exits 0
- `grep -q 'swift' README.md` exits 0 (Swift install section added)
- `grep -q 'genMetaCodeV0' docs/howto/swift.md` exits 0
- `grep -q 'DataHasher' docs/howto/swift.md` exits 0
- `grep -c '##' docs/howto/swift.md` returns at least 10 (matching dotnet.md section count)
- `cargo clippy --workspace --exclude iscc-rb --all-targets -- -D warnings` clean (no code changes,
    sanity check)

## Done When

All verification criteria pass — Swift howto guide, CLAUDE.md, README sections, nav config, and
ORDERED_PAGES entry are all in place and consistent.
