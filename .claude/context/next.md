# Next Work Package

## Step: Add npm publishing jobs to release workflow

## Goal

Add CI jobs to `.github/workflows/release.yml` that build and publish `@iscc/lib` (napi-rs native
addon) and `@iscc/wasm` (wasm-bindgen package) to npm when a version tag is pushed. This closes the
last code-level gap in the target state — everything else remaining is admin action (GitHub Pages
enablement).

## Scope

- **Modify**: `.github/workflows/release.yml` — add `build-napi`, `publish-npm-lib`, `build-wasm`,
    `publish-npm-wasm` jobs
- **Modify**: `crates/iscc-napi/package.json` — add platform triples for napi-rs cross-platform
    support
- **Reference**: `.github/workflows/ci.yml` (existing Node.js/WASM build patterns),
    `crates/iscc-napi/package.json` (current config), `notes/06-build-cicd-publishing.md`

## Implementation Notes

### napi-rs (`@iscc/lib`) publishing

Use the standard napi-rs cross-platform build matrix pattern:

1. **Build matrix job (`build-napi`)**: Build native `.node` binaries on each platform:

    - `ubuntu-latest` for `x86_64-unknown-linux-gnu`
    - `ubuntu-latest` + cross for `aarch64-unknown-linux-gnu`
    - `macos-14` for `aarch64-apple-darwin` (Apple Silicon)
    - `macos-13` for `x86_64-apple-darwin`
    - `windows-latest` for `x86_64-pc-windows-msvc`

    Each matrix entry: checkout → setup Node 20 → setup Rust → `npm install` →
    `npx napi build --platform --release` (from `crates/iscc-napi/`) → upload `*.node` as artifact.

2. **Update `crates/iscc-napi/package.json`**: Add platform triples configuration:

    ```json
    "napi": {
      "name": "iscc-lib",
      "triples": {
        "defaults": true,
        "additional": ["aarch64-unknown-linux-gnu"]
      }
    }
    ```

3. **Publish job (`publish-npm-lib`)**: Depends on `build-napi`. Download all `.node` artifacts into
    the `crates/iscc-napi/` directory → `npx napi prepublish -t npm` →
    `npm publish --provenance --access public`. Use `NPM_TOKEN` secret for authentication (npm OIDC
    provenance via `--provenance` flag requires `id-token: write` permission).

### wasm-bindgen (`@iscc/wasm`) publishing

1. **Build job (`build-wasm`)**: Install wasm-pack →
    `wasm-pack build --target web --release crates/iscc-wasm` → post-process
    `crates/iscc-wasm/pkg/package.json` to set `"name": "@iscc/wasm"` and correct version → upload
    `crates/iscc-wasm/pkg/` as artifact.

    wasm-pack generates `pkg/` with `package.json`, `.wasm`, `.js`, `.d.ts` files. The generated
    `package.json` uses the Cargo crate name (`iscc-wasm`) — we need to patch it to the scoped npm
    name (`@iscc/wasm`). Use `node -e` or `sed` to update the name field after build.

2. **Publish job (`publish-npm-wasm`)**: Depends on `build-wasm`. Download `pkg/` artifact →
    `cd pkg && npm publish --provenance --access public`. Same `NPM_TOKEN` secret.

### Authentication

- npm publishing uses `NPM_TOKEN` repository secret (standard pattern for npm)
- Add `NODE_AUTH_TOKEN: ${{ secrets.NPM_TOKEN }}` env var to publish steps
- Use `actions/setup-node@v4` with `registry-url: https://registry.npmjs.org` to configure `.npmrc`
    automatically
- Add `permissions: id-token: write` for npm provenance attestation

### Important details

- All npm publish jobs should have `needs:` dependency on their respective build jobs
- All npm jobs should be independent of the crates.io/PyPI jobs (parallel execution)
- Use `if: startsWith(github.ref, 'refs/tags/v')` on publish jobs to prevent accidental publishing
    from `workflow_dispatch` (build-only for testing)
- The `concurrency` group already prevents overlapping releases
- For the linux-arm64 cross-compilation, use `uraimo/run-on-arch-action@v2` or QEMU +
    `--target aarch64-unknown-linux-gnu`. Alternatively, use the napi-rs docker cross-compile
    pattern with `ghcr.io/napi-rs/napi-rs/nodejs-rust:lts-debian-aarch64`
- Existing crates.io and PyPI jobs must remain unchanged

## Verification

- `release.yml` is valid YAML (no syntax errors)
- The workflow has jobs for: `publish-crates-io`, `build-wheels`, `build-sdist`, `publish-pypi`,
    `build-napi`, `publish-npm-lib`, `build-wasm`, `publish-npm-wasm`
- napi-rs build matrix covers at least: linux-x64, linux-arm64, macos-arm64, windows-x64
- WASM build uses `wasm-pack build` with appropriate target
- npm publish steps use `--access public` (required for scoped packages)
- `NPM_TOKEN` secret is referenced for authentication
- `id-token: write` permission present on publish jobs (for provenance)
- `crates/iscc-napi/package.json` has platform triples configured
- Existing crates.io and PyPI jobs are unchanged

## Done When

The advance agent is done when `release.yml` contains complete npm publishing jobs for both
`@iscc/lib` and `@iscc/wasm`, the napi package.json has cross-platform triples, and all verification
criteria above pass.
