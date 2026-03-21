# Next Work Package

## Step: Add Swift CI job on macOS runner

## Goal

Add a `swift` CI job to `ci.yml` that builds the UniFFI native library and runs `swift test` on a
macOS runner. This is the only way to validate that the Swift conformance tests (50 vectors across 9
functions) actually pass — they cannot run in the Linux devcontainer.

## Scope

- **Create**: (none)
- **Modify**: `.github/workflows/ci.yml` — add `swift:` job
- **Reference**:
    - `packages/swift/Package.swift` — SPM manifest with `linkedLibrary("iscc_uniffi")`
    - `packages/swift/Tests/IsccLibTests/ConformanceTests.swift` — 9 test methods, 50 vectors
    - `.claude/context/specs/swift-bindings.md` — CI section with example YAML
    - `.github/workflows/ci.yml` — existing job patterns (esp. `c-ffi`, `dotnet`, `cpp`)

## Not In Scope

- `docs/howto/swift.md` how-to guide (separate step after CI is green)
- README Swift installation/quickstart sections (separate step)
- Version sync (`Constants.swift`, `version_sync.py`) — separate step
- `packages/swift/CLAUDE.md` — separate step
- XCFramework pre-built binaries — future release step
- Release workflow (`release.yml`) Swift publishing — separate step
- SwiftFormat/SwiftLint integration — premature before CI validates tests pass
- Adding `iscc-uniffi` to the Rust CI clippy/test job (it already runs in workspace)

## Implementation Notes

**Runner:** Use `macos-14` (Apple Silicon M1, has Xcode 15+ with Swift 5.9+ pre-installed). This
matches the `swift-tools-version: 5.9` in `Package.swift`.

**Job structure** (follow existing patterns from `c-ffi`, `dotnet`, `cpp`):

```yaml
swift:
  name: Swift (swift build, swift test)
  runs-on: macos-14
  steps:
    - uses: actions/checkout@v4
    - uses: dtolnay/rust-toolchain@stable
    - uses: Swatinem/rust-cache@v2
    - name: Build UniFFI native library
      run: cargo build -p iscc-uniffi
    - name: Build Swift package
      run: >
        swift build
        -Xlinker -L${{ github.workspace }}/target/debug
      working-directory: packages/swift
    - name: Run Swift tests
      run: >
        swift test
        -Xlinker -L${{ github.workspace }}/target/debug
        -Xlinker -rpath
        -Xlinker ${{ github.workspace }}/target/debug
      working-directory: packages/swift
```

**Key details:**

1. **Debug build** for speed — consistent with `c-ffi`, `dotnet`, `cpp` jobs (none use `--release`)
2. **Linker flags**: `-Xlinker -L<path>` tells the linker where to find `libiscc_uniffi.dylib`. The
    `-Xlinker -rpath -Xlinker <path>` embeds the library search path into the test binary so it
    finds the dylib at runtime without needing `DYLD_LIBRARY_PATH` (which can be stripped by SIP on
    macOS)
3. **Library name**: `cargo build -p iscc-uniffi` produces `target/debug/libiscc_uniffi.dylib` on
    macOS. The `Package.swift` already has `.linkedLibrary("iscc_uniffi")` which SPM translates to
    `-liscc_uniffi`
4. **Separate build + test steps** for clearer error diagnosis (build failure vs test failure)
5. **`Swatinem/rust-cache@v2`** works on macOS runners — no compatibility issues
6. **Place the job** after `cpp:` and before `bench:` (keeps binding jobs grouped before bench)

**If `-rpath` alone doesn't work** (in case `swift test`'s XCTest runner doesn't propagate it), add
`env: DYLD_LIBRARY_PATH: ${{ github.workspace }}/target/debug` to the test step as fallback. Try
`-rpath` first — it's the cleaner solution.

## Verification

- `grep -q 'macos-14' .github/workflows/ci.yml` exits 0 (macOS runner present)
- `grep -q 'swift test' .github/workflows/ci.yml` exits 0 (swift test command present)
- `grep -q 'cargo build -p iscc-uniffi' .github/workflows/ci.yml` exits 0 (UniFFI build step)
- `python3 -c "import yaml; yaml.safe_load(open('.github/workflows/ci.yml'))"` exits 0 (valid YAML)
- `cargo clippy --workspace --exclude iscc-rb --all-targets -- -D warnings` passes (no regressions)

## Done When

All verification commands pass and the `swift:` job is properly defined in `ci.yml` with macOS
runner, UniFFI library build, and `swift test` execution with correct linker flags.
