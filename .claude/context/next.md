# Next Work Package

## Step: Fix XCFramework release cache key to include all build inputs

## Goal

Expand the XCFramework build cache key in `release.yml` to include all files that affect the output
binary — preventing stale XCFramework artifacts from being published when build scripts, headers, or
UniFFI scaffolding change. Addresses the "XCFramework release cache key incomplete" normal issue.

## Scope

- **Create**: (none)
- **Modify**: `.github/workflows/release.yml` (line ~1269, the `key:` field of the XCFramework cache
    step)
- **Reference**: `scripts/build_xcframework.sh`, `packages/swift/Sources/iscc_uniffiFFI/`,
    `.claude/context/issues.md`

## Not In Scope

- Fixing the Swift `ref: main` race condition (separate issue, more complex architectural change)
- Adding a root `Package.swift` CI smoke test (separate issue)
- Changing the caching strategy (e.g., removing caching entirely) — just expand the key
- Modifying the build script itself or any non-cache-key parts of the workflow

## Implementation Notes

The current cache key at line 1269 is:

```yaml
key: xcf-${{ hashFiles('crates/iscc-*/src/**', 'Cargo.lock') }}
```

This misses several inputs that affect the XCFramework output:

1. **`scripts/build_xcframework.sh`** — changes to the build process (targets, flags, lipo
    configuration, zip method) produce different binaries
2. **`packages/swift/Sources/iscc_uniffiFFI/iscc_uniffiFFI.h`** and **`module.modulemap`** — these
    headers are copied into the XCFramework; stale headers cause compile failures for consumers
3. **`crates/iscc-uniffi/src/**`** — the UniFFI scaffolding crate source that generates the Swift
    bindings (already covered by `crates/iscc-*/src/**` glob since `iscc-*` matches `iscc-uniffi`)
4. **Root `Cargo.toml`** and per-crate **`Cargo.toml`** files — workspace dependency versions and
    feature flag changes affect compilation

The fix: expand the `hashFiles()` call to explicitly include these paths. Use a multi-line format
for readability:

```yaml
key: xcf-${{ hashFiles(
  'crates/iscc-*/src/**',
  'crates/iscc-*/Cargo.toml',
  'Cargo.lock',
  'Cargo.toml',
  'scripts/build_xcframework.sh',
  'packages/swift/Sources/iscc_uniffiFFI/**'
) }}
```

**Important YAML formatting note**: `hashFiles()` accepts multiple glob arguments as a single
function call. Keep the YAML valid — test with a YAML parser after editing.

Verify that `crates/iscc-*/src/**` already matches `crates/iscc-uniffi/src/` (it does, since the
glob `iscc-*` matches `iscc-uniffi`).

## Verification

- `python3 -c "import yaml; yaml.safe_load(open('.github/workflows/release.yml'))"` exits 0 (valid
    YAML)
- `grep -A5 'xcf-cache' .github/workflows/release.yml | grep 'build_xcframework.sh'` finds the build
    script in the cache key
- `grep -A5 'xcf-cache' .github/workflows/release.yml | grep 'iscc_uniffiFFI'` finds the headers in
    the cache key
- `grep -A5 'xcf-cache' .github/workflows/release.yml | grep 'Cargo.toml'` finds Cargo.toml in the
    cache key (beyond just Cargo.lock)
- `mise run format` produces no changes
- `mise run check` passes (all pre-commit hooks)

## Done When

All verification criteria pass — the XCFramework cache key includes build script, Swift headers,
Cargo.toml files, and all crate sources, ensuring cache invalidation on any input change.
