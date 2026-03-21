# Next Work Package

## Step: Fix Swift CI module name mismatch

## Goal

Fix the failing Swift CI job by renaming the SPM FFI target from `IsccLibFFI` to `iscc_uniffiFFI` so
it matches the `#if canImport(iscc_uniffiFFI)` / `import iscc_uniffiFFI` in the UniFFI-generated
Swift code. This restores CI to green (all 15 jobs passing).

## Scope

- **Create**: (none)
- **Modify**: `packages/swift/Package.swift`, `packages/swift/Sources/IsccLibFFI/module.modulemap`
- **Rename**: `packages/swift/Sources/IsccLibFFI/` → `packages/swift/Sources/iscc_uniffiFFI/`
- **Reference**: `packages/swift/Sources/IsccLib/iscc_uniffi.swift` (lines 10-11 — the
    `canImport`/`import` that dictates the required module name)

## Not In Scope

- Editing the generated `iscc_uniffi.swift` file (option (b) — we use option (a) instead)
- Swift docs (`docs/howto/swift.md`), README Swift sections, or version sync
- `packages/swift/CLAUDE.md` creation
- XCFramework or release distribution setup
- Any Kotlin bindings work

## Implementation Notes

The root cause is a module name mismatch. UniFFI generates Swift code that does:

```swift
#if canImport(iscc_uniffiFFI)
import iscc_uniffiFFI
```

But the SPM target is named `IsccLibFFI` with `module IsccLibFFI` in the modulemap. The conditional
import silently fails, leaving all FFI symbols unresolved (~40 "cannot find" errors).

**Fix (3 changes):**

1. **Rename directory**: `Sources/IsccLibFFI/` → `Sources/iscc_uniffiFFI/`

2. **Update `Package.swift`** — change 3 occurrences of `IsccLibFFI` to `iscc_uniffiFFI`:

    - Line 11: target name `"IsccLibFFI"` → `"iscc_uniffiFFI"`
    - Line 12: path `"Sources/IsccLibFFI"` → `"Sources/iscc_uniffiFFI"`
    - Line 20: dependency `"IsccLibFFI"` → `"iscc_uniffiFFI"`

3. **Update `module.modulemap`** (inside the renamed directory):

    - Line 1: `module IsccLibFFI` → `module iscc_uniffiFFI`

This is the recommended fix from state.md — option (a): rename the SPM target to match the generated
code's expectation, avoiding edits to generated code.

## Verification

- `grep -c 'iscc_uniffiFFI' packages/swift/Package.swift` outputs `3`
- `grep -c 'IsccLibFFI' packages/swift/Package.swift` outputs `0`
- `head -1 packages/swift/Sources/iscc_uniffiFFI/module.modulemap` contains `module iscc_uniffiFFI`
- `test -d packages/swift/Sources/iscc_uniffiFFI` exits 0 (directory exists)
- `test ! -d packages/swift/Sources/IsccLibFFI` exits 0 (old directory gone)
- `cargo clippy --workspace --exclude iscc-rb --all-targets -- -D warnings` clean
- `mise run check` passes (all pre-commit/pre-push hooks)

## Done When

All verification criteria pass — the module name mismatch is resolved and the Swift package
structure is consistent with the UniFFI-generated imports.
