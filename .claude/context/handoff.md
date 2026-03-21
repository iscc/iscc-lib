## 2026-03-21 — Fix Swift CI module name mismatch

**Done:** Renamed the SPM FFI target from `IsccLibFFI` to `iscc_uniffiFFI` across directory name,
`Package.swift`, and `module.modulemap` so it matches the `#if canImport(iscc_uniffiFFI)` /
`import iscc_uniffiFFI` in the UniFFI-generated Swift code.

**Files changed:**

- `packages/swift/Sources/IsccLibFFI/` → `packages/swift/Sources/iscc_uniffiFFI/` (directory rename)
- `packages/swift/Package.swift`: changed 3 occurrences of `IsccLibFFI` to `iscc_uniffiFFI` (target
    name, path, dependency)
- `packages/swift/Sources/iscc_uniffiFFI/module.modulemap`: changed `module IsccLibFFI` to
    `module iscc_uniffiFFI`

**Verification:**

- `grep -c 'iscc_uniffiFFI' packages/swift/Package.swift` → 3 — PASS
- `grep -c 'IsccLibFFI' packages/swift/Package.swift` → 0 — PASS
- `head -1 .../module.modulemap` → `module iscc_uniffiFFI {` — PASS
- `test -d packages/swift/Sources/iscc_uniffiFFI` → exists — PASS
- `test ! -d packages/swift/Sources/IsccLibFFI` → gone — PASS
- `cargo clippy --workspace --exclude iscc-rb --all-targets -- -D warnings` — clean
- `mise run check` — all hooks pass (pre-commit + pre-push)

**Next:** Push to verify the Swift CI job passes on GitHub Actions. This is the real validation
since Swift tests can't run on the Linux devcontainer. If CI is green, next work should be Swift
docs + README integration (`docs/howto/swift.md`, version sync, etc.) or closing the Swift issue in
issues.md and starting Kotlin bindings.

**Notes:** No surprises. The fix is purely structural — 3 string replacements + 1 directory rename.
No code logic changed. No tests affected (conformance tests are in `Tests/IsccLibTests/` which is
unchanged — they import `IsccLib` not the FFI target directly).
