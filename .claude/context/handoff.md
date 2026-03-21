## 2026-03-21 — Add root Package.swift and fix Swift install documentation

**Done:** Created a root-level `Package.swift` that mirrors the subdirectory manifest with adjusted
paths (no test target), enabling SPM dependency resolution via `https://github.com/iscc/iscc-lib`.
Updated all three Swift documentation files to use version `0.3.1` and honestly document the
build-from-source requirement for the native `libiscc_uniffi` library.

**Files changed:**

- `Package.swift` (new): Root SPM manifest with paths pointing to `packages/swift/Sources/`. Two
    targets (`iscc_uniffiFFI`, `IsccLib`), one library product, no test target
- `README.md`: Swift install section updated from `0.3.0` to `0.3.1`, added note about native
    library requirement with link to howto guide
- `packages/swift/README.md`: Version `0.3.0` to `0.3.1`, added build-from-source prerequisite note
    in Installation section, updated Requirements to list Rust toolchain instead of "Pre-built
    XCFramework", expanded Building from Source section with clearer step-by-step instructions
- `docs/howto/swift.md`: Version `0.3.0` to `0.3.1`, restructured Installation section -- promoted
    build-from-source from a collapsible tip to a subsection ("Build the native library") with
    clone/build/link instructions, noted XCFramework distribution is planned for future release

**Verification:** All 11 verification criteria pass:

- `Package.swift` exists with correct `swift-tools-version` header
- Paths point to `packages/swift/Sources/IsccLib` and `packages/swift/Sources/iscc_uniffiFFI`
- No `testTarget` in root manifest
- Version `0.3.1` present in all three docs, `0.3.0` removed from all three
- `cargo build` documented in both `packages/swift/README.md` and `docs/howto/swift.md`
- `cargo clippy --workspace --all-targets -- -D warnings` exits 0
- All 15 pre-commit hooks pass (`mise run check`)

**Next:** The two Swift packaging issues (SPM URL resolution + install documentation) are now
resolved. The remaining open work is likely XCFramework binary distribution (enabling zero-friction
install without Rust toolchain), but that's a larger infrastructure step. Check `issues.md` and
`target.md` for the next priority.

**Notes:**

- The two `Package.swift` files coexist without conflict: SPM always reads from root for dependency
    resolution; `cd packages/swift && swift build` uses the subdirectory one. CI is unaffected -- it
    uses `working-directory: packages/swift`
- The root manifest intentionally omits the test target -- conformance tests have a `data.json`
    resource dependency that would need path adjustment, and SPM consumers don't need tests
- The `.linkedLibrary("iscc_uniffi")` linker setting is preserved in the root manifest -- it
    accurately reflects the native library dependency
