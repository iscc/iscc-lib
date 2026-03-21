# Next Work Package

## Step: Add root Package.swift and fix Swift install documentation

## Goal

Fix the two open Swift packaging issues by adding a root-level `Package.swift` (so SPM URL
resolution works with `https://github.com/iscc/iscc-lib`) and updating all install documentation to
accurately describe the current build-from-source requirement for the native library.

## Scope

- **Create**: `Package.swift` (repo root)
- **Modify**: `README.md` (Swift install section ~lines 133-141), `packages/swift/README.md`
    (install section ~lines 7-17), `docs/howto/swift.md` (install section ~lines 14-48)
- **Reference**: `packages/swift/Package.swift` (existing subdirectory manifest — the template),
    `packages/swift/CLAUDE.md` (build commands and layout), `.github/workflows/ci.yml` (Swift CI job
    ~line 232, uses `working-directory: packages/swift`)

## Not In Scope

- XCFramework binary target builds — that's a separate, larger step requiring CI infrastructure to
    build universal frameworks, upload as release assets, and reference as `.binaryTarget`
- CI workflow changes — existing CI uses `working-directory: packages/swift` and will continue to
    use the subdirectory Package.swift unchanged
- Removing or modifying `packages/swift/Package.swift` — it remains the local dev/CI manifest
- Adding Swift to the release workflow (currently SPM uses Git tags, no registry publish needed)
- Resolving issue 2 fully (native library vending) — this step documents the requirement honestly;
    full fix requires XCFramework support in a future step

## Implementation Notes

### Root Package.swift

Create a root-level `Package.swift` mirroring the subdirectory manifest with adjusted paths:

```swift
// swift-tools-version: 5.9
import PackageDescription

let package = Package(
    name: "IsccLib",
    products: [
        .library(name: "IsccLib", targets: ["IsccLib"]),
    ],
    targets: [
        .target(
            name: "iscc_uniffiFFI",
            path: "packages/swift/Sources/iscc_uniffiFFI",
            publicHeadersPath: ".",
            linkerSettings: [
                .linkedLibrary("iscc_uniffi"),
            ]
        ),
        .target(
            name: "IsccLib",
            dependencies: ["iscc_uniffiFFI"],
            path: "packages/swift/Sources/IsccLib"
        ),
    ]
)
```

Key decisions:

- **Omit the test target** — tests stay in `packages/swift/` for CI. SPM consumers don't need them
- **Keep `.linkedLibrary("iscc_uniffi")`** — accurate; the native library IS required
- **swift-tools-version: 5.9** — matches the subdirectory manifest
- **No conflict with subdirectory Package.swift** — SPM dependency resolution only looks at root;
    `cd packages/swift && swift build` uses the subdirectory one

### Documentation Updates (3 doc files)

All three docs need the same conceptual change:

1. **Fix version**: Change `from: "0.3.0"` to `from: "0.3.1"` everywhere (Swift package was added in
    0.3.1, not 0.3.0)

2. **Add build-from-source prerequisite**: After the SPM dependency snippet, add a clear note
    explaining that the native `libiscc_uniffi` library must be built from source with
    `cargo build -p iscc-uniffi --release`, and the linker must be told where to find it via
    `-Xlinker -L<path-to-target/release>`

3. **README.md** (lines 133-141): Keep concise — show SPM dependency snippet with correct version,
    add a brief note about the native library requirement and link to the howto guide for details

4. **packages/swift/README.md** (lines 7-17): Expand the Requirements section to list the native
    library build as a prerequisite. The "Building from Source" section already exists but should
    be referenced more prominently from the install section

5. **docs/howto/swift.md** (lines 14-48): Promote the "Build from source" admonition from a tip to
    the primary installation path. The existing content is good — restructure so the
    build-from-source steps are the main flow, with a note that XCFramework distribution (enabling
    zero-friction install) is planned for a future release

### Two Package.swift files coexist correctly

- Root `Package.swift` — for SPM consumers who add `https://github.com/iscc/iscc-lib`
- `packages/swift/Package.swift` — for CI and local development
    (`working-directory: packages/swift`)
- No conflict: SPM resolution always starts from root; local builds start from current directory

## Verification

- `test -f Package.swift` — root Package.swift exists
- `head -1 Package.swift | grep -q 'swift-tools-version'` — valid Swift manifest
- `grep -q 'packages/swift/Sources/IsccLib' Package.swift` — paths point to subdirectory
- `grep -q 'packages/swift/Sources/iscc_uniffiFFI' Package.swift` — FFI target path correct
- `! grep -q 'testTarget' Package.swift` — no test target in root manifest
- `grep -q '0.3.1' README.md` — Swift version updated from 0.3.0
- `! grep -q 'from: "0.3.0"' README.md` — old version removed
- `! grep -q 'from: "0.3.0"' packages/swift/README.md` — old version removed
- `! grep -q 'from: "0.3.0"' docs/howto/swift.md` — old version removed
- `grep -q 'cargo build' packages/swift/README.md` — build-from-source in install section
- `grep -q 'cargo build' docs/howto/swift.md` — build-from-source documented
- `cargo clippy --workspace --all-targets -- -D warnings` exits 0 (no Rust changes, sanity check)

## Done When

All verification criteria pass: root Package.swift exists with correct subdirectory paths, all three
documentation files have accurate Swift install instructions referencing version 0.3.1 and the
build-from-source requirement, and no CI regressions.
