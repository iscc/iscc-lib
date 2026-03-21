## 2026-03-21 — Review of: Add Package.swift version sync and update Swift install docs

**Verdict:** PASS

**Summary:** Clean, minimal implementation. Two functions added to `version_sync.py` following
existing patterns exactly, and `docs/howto/swift.md` manual build section replaced with a
collapsible "Build from source" tip. All 5 verification criteria pass, all 15 pre-commit hooks pass,
clippy workspace clean.

**Verification:**

- [x] `uv run scripts/version_sync.py --check` exits 0 — 16 targets all match (was 15)
- [x] `grep -c "Package.swift" scripts/version_sync.py` returns 4 (≥2)
- [x] `grep "planned for a future release" docs/howto/swift.md` — no matches (old text removed)
- [x] `grep "Build from source" docs/howto/swift.md` — match found (new collapsible section present)
- [x] `mise run check` — all 15 pre-commit hooks pass

**Issues found:**

- Resolved: "Swift package does not vend the native library" (`normal` `[human]`) — deleted from
    issues.md. XCFramework build script, root Package.swift, release workflow, version sync, and
    docs all complete.

**Codex review:** Two P2 findings, both evaluated as advisory:

1. **`cargo build` local dev path misleading**: The collapsible tip suggests
    `cargo build -p  iscc-uniffi --release` as an alternative, but then says "toggle
    `useLocalFramework = true`" which expects an XCFramework at `target/ios/`. Valid docs accuracy
    point, but in a collapsible advanced-user tip — the primary `build_xcframework.sh` path is
    correct. Low priority.
2. **Version sync before checksum exists**: Bumping `releaseTag` without updating `releaseChecksum`
    could cause SPM resolution failures. Dismissed — this is by design. The release workflow
    updates the checksum after building. Development branches are not consumed as SPM packages.

**Next:** The only `normal` issue remaining is the Swift `GITHUB_REF_NAME` bug, which has
`HUMAN REVIEW REQUESTED` — CID cannot fix it without human approval to update the spec. The other
issue (docs logos) is `low`. The define-next agent should recognize the `HUMAN REVIEW REQUESTED`
flag blocks autonomous progress on the `GITHUB_REF_NAME` issue.

**Notes:**

- Cleaned up triplicated CI/CD entries in learnings.md (183→171 lines)
- Version sync now covers 16 targets — state.md will reflect this on next update-state run
- Swift XCFramework deliverables are now complete except for the `GITHUB_REF_NAME` spec fix
- The `GITHUB_REF_NAME` issue needs human to either approve the spec change or downgrade to `low`
