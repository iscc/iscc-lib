## 2026-03-21 — Review of: Fix Swift CI module name mismatch

**Verdict:** PASS

**Summary:** The advance agent correctly renamed the SPM FFI target from `IsccLibFFI` to
`iscc_uniffiFFI` across directory name, `Package.swift`, and `module.modulemap`. The fix is minimal,
focused, and addresses the exact root cause of the failing Swift CI job (silent conditional import
failure due to module name mismatch).

**Verification:**

- [x] `grep -c 'iscc_uniffiFFI' packages/swift/Package.swift` → 3 — PASS
- [x] `grep -c 'IsccLibFFI' packages/swift/Package.swift` → 0 — PASS
- [x] `head -1 .../module.modulemap` → `module iscc_uniffiFFI {` — PASS
- [x] `test -d packages/swift/Sources/iscc_uniffiFFI` → exists — PASS
- [x] `test ! -d packages/swift/Sources/IsccLibFFI` → gone — PASS
- [x] `cargo clippy --workspace --exclude iscc-rb --all-targets -- -D warnings` — clean
- [x] `mise run check` — 15/15 hooks pass

**Issues found:**

- (none)

**Codex review:** Codex timed out (~2 min wait on a trivial 3-line rename diff). Skipped — manual
review is comprehensive for this scope.

**Next:** Push will trigger CI — the Swift job on `macos-14` should now pass since the module name
matches the UniFFI-generated `#if canImport(iscc_uniffiFFI)`. If CI is green (all 15 jobs), the next
work package should be **Swift docs + README integration**: `docs/howto/swift.md` how-to guide,
README Swift install/quickstart tab, `packages/swift/CLAUDE.md`, and version sync for
`Constants.swift`. After Swift is fully complete, close the Swift issue and begin Kotlin bindings.

**Notes:**

- The last CI run (23379967641) failed on the Swift job due to this exact module name mismatch. This
    push should fix it
- No source code references to the old `IsccLibFFI` name remain in the codebase (only context/memory
    files describing the fix)
- State.md still describes the pre-fix state — update-state will reconcile after CI results
