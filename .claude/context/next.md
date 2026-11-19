# Next Work Package

## Step: npm packages ship a license and a plain-text description

## Goal

Fix the `normal` issue "`@iscc/wasm` on npm has no license and a README-badge description". Probing
showed the same badge-markdown description on `@iscc/lib` (`npm view @iscc/lib description`), so
both npm manifests get a real description in this step. Then npm and license scanners stop treating
`@iscc/wasm` as unlicensed.

## Alternatives Considered

- **Chosen:** npm metadata for both packages. It affects every consumer of a published artifact
    (licence scanning), it is a two-file manifest change, and a fresh `wasm-pack` build verifies it
    locally.
- **Rejected:** Ruby negative-Bignum truncation. It is a real wrong-result bug, but it only fires
    for inputs from -2**63 to -(2**62)-1. It goes next, and it needs its own Ruby rebuild and test
    cycle.

## Scope

- **Modify**: `crates/iscc-wasm/Cargo.toml`, `crates/iscc-napi/package.json`
- **Reference**: `.github/workflows/release.yml` (`build-wasm`, "Set npm package name and version"
    step, L428-439), root `Cargo.toml` `[workspace.package]`, `crates/iscc-wasm/README.md` and
    `crates/iscc-napi/README.md` (taglines)

## Not In Scope

- Editing `release.yml`. Its patch step reads the generated JSON, changes only `name`, `repository`
    and `version`, and writes it back, so fields that come from `Cargo.toml` survive. Keep
    `Cargo.toml` as the single source of truth.
- Copying a `LICENSE` file into `crates/iscc-wasm/`. If wasm-pack warns about a missing licence
    file, that warning is informational.
- Adding `keywords`, `homepage` or other npm fields beyond `license` and `description`.
- Editing the published 0.7.0 packages on npm. The fix ships with the next release.
- A pytest that re-reads `Cargo.toml` with tomllib. It would only restate the manifest.

## Implementation Notes

- **WASM**: wasm-pack builds `pkg/package.json` from the crate manifest. Add to `[package]` in
    `crates/iscc-wasm/Cargo.toml`:
    - `license.workspace = true` (the workspace value is `Apache-2.0`)
    - a crate-specific `description` literal. Do NOT use `description.workspace = true`, because the
        workspace text says "Rust implementation", which is the wrong pitch for a browser package.
- wasm-pack 0.13.1 already resolves `version.workspace = true`, so it probably resolves `license`
    the same way. That is unverified. If the generated JSON has no `license`, use the literal
    `license = "Apache-2.0"`.
- Exact WASM description string:
    `Browser-compatible WebAssembly bindings for ISO 24138:2024 International Standard Content Code (ISCC)`
- **napi**: `crates/iscc-napi/package.json` is committed and published as-is. Add a `"description"`
    key right after `"license"` with the exact string:
    `High-performance Node.js bindings for ISO 24138:2024 International Standard Content Code (ISCC)`
- The `crates/iscc-wasm/pkg/` already on disk is stale (version 0.5.0, built in July) and
    gitignored. Only a fresh build proves the fix.
- Manifest-only change: the CRAP, iai and semver gates are unaffected.

## Verification

- `wasm-pack build --target web --dev crates/iscc-wasm` exits 0. A `--release` build also needs
    `RUSTFLAGS` with SIMD; `--dev` skips wasm-opt, and the manifest mapping is the same for both
- `node -e` on the freshly generated `crates/iscc-wasm/pkg/package.json` asserts `license` equals
    `"Apache-2.0"` and `description` equals the exact WASM string above
- `node -e` on `crates/iscc-napi/package.json` asserts `description` equals the exact napi string
    above and `license` is still `"Apache-2.0"`
- `cargo metadata --no-deps --format-version 1` parses, and for `iscc-wasm` it shows
    `"license": "Apache-2.0"`
- `mise run check` exits 0

## Done When

A fresh wasm-pack build produces a `package.json` with the Apache-2.0 licence and the plain-text
WASM description, the napi manifest has its description, and `mise run check` is green.
