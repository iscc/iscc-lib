## 2026-03-21 — Review of: Add Swift XCFramework build-and-publish job to release workflow

**Verdict:** PASS_WITH_NOTES

**Summary:** Clean, well-scoped YAML addition. The `build-xcframework` job and `swift` input
faithfully follow the spec's YAML snippet in `swift-bindings.md`. All 10 verification criteria pass,
all 15 pre-commit hooks pass, clippy clean. One advisory note: the `--ref main` re-trigger
convention won't work for this job (see issues.md).

**Verification:**

- [x] YAML valid (`uv run python -c "import yaml; yaml.safe_load(...)"` exits 0)
- [x] `swift:` input present
- [x] `build-xcframework` job name present
- [x] `build_xcframework.sh` script invoked
- [x] `swift package compute-checksum` step present
- [x] `force` tag update present
- [x] `IsccLib.xcframework.zip` artifact name correct
- [x] `contents: write` permission present
- [x] 9 boolean inputs (was 8, now +1 for swift)
- [x] `mise run check` — all 15 pre-commit hooks pass

**Issues found:**

- Advisory: `build-xcframework` uses `GITHUB_REF_NAME` for version extraction, unlike all other
    release jobs which use `Cargo.toml`. Manual re-trigger with `--ref main -f swift=true` would set
    `releaseTag` to `main` and try to tag the branch name. Added to issues.md for resolution.

**Codex review:** Three findings, all evaluated:

1. **P1** (checkout `ref: main` instead of tag): Intentional per spec — Ferrostar pattern. Tags are
    always on `main` in this project. The force-update tag step ensures consistency. Dismissed.
2. **P1** (`workflow_dispatch` from branch breaks): Valid. `GITHUB_REF_NAME` is the branch name when
    dispatched from `--ref main`, making version/tag operations fail. Filed as issue.
3. **P2** (cache key misses `build_xcframework.sh`): Valid minor point. Spec-faithful implementation
    — the spec's cache key only includes Rust sources and Cargo.lock. Low risk since build script
    changes are rare and would typically accompany Rust source changes. Advisory only.

**Next:** Two remaining steps for Swift XCFramework completion:

1. **Version sync** (higher priority): Add `releaseTag` to `version_sync.py` so version bumps
    propagate to root `Package.swift`. This is target 16.
2. **Docs update**: Update `docs/howto/swift.md` to document zero-friction SPM install using the
    binary target pattern.

After both: the Swift vend issue can be closed. Also consider fixing the `--ref main` re-trigger
convention for the Swift release job (derive version from Cargo.toml instead of GITHUB_REF_NAME).

**Notes:**

- The Swift vend issue in issues.md stays open — release workflow integration is done but version
    sync and docs update remain
- Release input count is now 9 (was 8). State.md will need updating to reflect this
- The cache path `target/ios/IsccLib.xcframework.zip` correctly matches the build script output
    (spec had `IsccLib.xcframework.zip` in the root — implementation improved this)
- `Swatinem/rust-cache@v2` added per spec's caching strategy section (not in the YAML snippet but
    called for in the prose)
