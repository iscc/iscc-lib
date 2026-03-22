## 2026-03-22 — Fix XCFramework release cache key to include all build inputs

**Done:** Expanded the XCFramework build cache key in `release.yml` to include all files that affect
the output binary — build script, Swift FFI headers/modulemap, per-crate Cargo.toml files, and root
Cargo.toml. This prevents stale XCFramework artifacts from being published when these inputs change.

**Files changed:**

- `.github/workflows/release.yml`: Expanded `hashFiles()` in the `xcf-cache` key (line ~1269) from 2
    globs to 6, adding `crates/iscc-*/Cargo.toml`, `Cargo.toml`, `scripts/build_xcframework.sh`, and
    `packages/swift/Sources/iscc_uniffiFFI/**`

**Verification:**

- YAML validation exits 0 (valid YAML confirmed via `yaml.safe_load`)
- `grep -A5 'xcf-cache' | grep 'build_xcframework.sh'` — PASS
- `grep -A5 'xcf-cache' | grep 'iscc_uniffiFFI'` — PASS
- `grep -A5 'xcf-cache' | grep 'Cargo.toml'` — PASS (both root and per-crate globs present)
- `mise run format` — no changes (yamlfix already applied its preferred formatting)
- `mise run check` — all 15 pre-commit hooks pass

**Next:** Two normal-priority release workflow issues remain: (1) Swift `ref: main` race condition
(the `build-xcframework` job uses `GITHUB_REF_NAME` for version/tag instead of deriving from
`Cargo.toml`), and (2) root `Package.swift` CI smoke test. There's also the benchmarks documentation
gap (low priority). The Swift `ref:main` race condition is likely the most impactful remaining
issue.

**Notes:** yamlfix reformatted the multi-line `hashFiles()` call into a more compact 3-line format
(vs the 7-line format suggested in next.md). The content is identical — just different line
wrapping. This is fine since yamlfix is the authoritative formatter. No code changes needed beyond
the single YAML file edit — this was a pure CI configuration fix with no tests to write.
