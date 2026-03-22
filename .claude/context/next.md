# Next Work Package

## Step: Add root Package.swift manifest smoke test to CI

## Goal

Add a CI step that validates the consumer-facing root `Package.swift` manifest parses correctly,
addressing the normal-priority issue "CI does not exercise root Package.swift." This catches
structural regressions (syntax errors, invalid target names, broken paths) in the manifest that real
SPM consumers resolve.

## Scope

- **Create**: (none)
- **Modify**: `.github/workflows/ci.yml` (add a step to the `swift` job that runs
    `swift package dump-package` at the repo root)
- **Reference**: `Package.swift` (root manifest structure), `.claude/context/issues.md` (issue
    description), `.github/workflows/ci.yml` (existing `swift` job, lines 232-252)

## Not In Scope

- Full `swift package resolve` at the repo root — the checksum is `PLACEHOLDER` on develop, so
    binary-target resolution would fail. Manifest parsing validation is the appropriate check
- Changing the root `Package.swift` checksum or URL pattern
- Adding a separate CI job for the root manifest — reuse the existing `swift` macOS-14 job
- Modifying the release workflow
- Validating that `releaseTag` matches `Cargo.toml` version — this is already handled by
    `version_sync.py --check` in the Version Consistency CI job

## Implementation Notes

The existing `swift` CI job (lines 232-252) runs on `macos-14` and already has:

1. `actions/checkout@v4`
2. `dtolnay/rust-toolchain@stable`
3. `rust-cache`
4. `cargo build -p iscc-uniffi`
5. `swift build` (in `packages/swift/`)
6. `swift test` (in `packages/swift/`)

**Add a new step** after checkout (early, before the expensive Rust build) that validates the root
`Package.swift`:

```yaml
  - name: Validate root Package.swift manifest
    run: swift package dump-package
```

No `working-directory` needed — the default is the repo root, which is exactly where the
consumer-facing `Package.swift` lives.

**Why `dump-package`:**

- Parses the `Package.swift` and outputs its structure as JSON
- Validates Swift syntax, target names, dependency references, platform requirements
- Does NOT attempt to download binary targets (so PLACEHOLDER checksum is fine)
- Fast (< 1 second) — no network, no compilation
- Available on all macOS runners with Swift toolchain

**Placement:** Insert after the `Swatinem/rust-cache@v2` step (line 238) and before the "Build
UniFFI native library" step (line 239-240). The rust-toolchain and rust-cache steps are cheap and we
want them available for subsequent steps regardless. This validates manifest structure early and
fails fast without wasting time on Rust compilation.

**Output verification:** The step succeeds (exit 0) if the manifest parses. It fails (exit 1) with a
Swift error if there's a syntax or structural problem. The JSON output goes to stdout but doesn't
need to be captured.

## Verification

- `python3 -c "import yaml; yaml.safe_load(open('.github/workflows/ci.yml'))"` exits 0 (valid YAML)
- `grep -c 'dump-package' .github/workflows/ci.yml` returns `1` (new step exists)
- `grep -B2 'dump-package' .github/workflows/ci.yml | grep -c 'working-directory'` returns `0` (runs
    at repo root, not in a subdirectory)
- `grep -B5 'dump-package' .github/workflows/ci.yml | grep 'Validate root'` confirms the step name
    references the root manifest
- The `dump-package` step appears BEFORE the "Build UniFFI native library" step in the `swift` job
    (visual inspection of step ordering in the YAML)
- `mise run format` produces no changes

## Done When

All verification criteria pass — the `swift` CI job includes a manifest-parsing smoke test for the
root `Package.swift` that runs before the expensive build steps.
