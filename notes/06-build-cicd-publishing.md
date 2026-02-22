# Binary Size, CI/CD, and Publishing

Release optimization, CI testing matrices, and multi-registry publishing workflows.

## Binary Size and Release Optimization

Binary size matters for Python wheels (download time), Node native addons (install time), and WASM
bundles (page load). A few Cargo and build settings make a large difference.

### Rust Release Profile

iscc-sum already uses these settings — they should be standard for all iscc-lib crates:

```toml
# Cargo.toml (workspace root)
[profile.release]
opt-level = 3       # Maximum optimization
lto = true           # Link-time optimization (or "thin" for faster builds)
codegen-units = 1    # Better optimization at cost of compile time
strip = true         # Strip debug symbols
panic = "abort"      # Smaller binary, no unwinding overhead
```

**Trade-off**: `lto = true` + `codegen-units = 1` significantly increases compile time. Use
`lto = "thin"` in CI for faster builds while keeping most of the size benefit.

### Per-Target Guidance

**Python wheels:**
- Use `abi3-py310` to publish one wheel per platform instead of one per Python version
- Avoid pulling in `openssl`, `reqwest`, or `tokio` — each adds hundreds of KB
- The `strip = true` setting removes debug symbols automatically

**Node.js native addons:**
- Disable default features in the core crate dependency to avoid pulling unused code
- Use `#[napi]` on the minimal surface only — each exported function increases binding overhead

**WASM bundles:**
- Add `wasm-opt` optimization in the build:
  ```toml
  [package.metadata.wasm-pack.profile.release]
  wasm-opt = ["-Os"]  # Optimize for size
  ```
- Avoid `serde_json` in WASM code paths — it adds ~50KB. Use `serde-wasm-bindgen` instead for
  direct JsValue conversion.
- Consider `#[cfg(target_arch = "wasm32")]` to gate WASM-specific code and exclude desktop-only
  dependencies.

**CLI binaries:**
- Use `cargo-dist` which applies `strip` and UPX compression automatically
- Consider `opt-level = "s"` (optimize for size) instead of `3` if binary size matters more than
  peak throughput

## CI/CD Testing Matrix

**minijinja** tests across multiple Rust versions and targets:

```yaml
# .github/workflows/tests.yml
jobs:
  test-latest:      # Stable Rust, latest
  test-nightly:     # Nightly, no lockfile
  test-32bit:       # armv5te-unknown-linux-gnueabi (Rust 1.70)
  test-stable:      # MSRV (1.70)
  test-wasi:        # wasm32-wasip1 via WasmTime
  test-python:      # Python bindings
  test-js:          # JS/WASM bindings
  test-cli-linux:   # CLI on Linux
  test-cli-windows: # CLI on Windows
```

**kreuzberg** uses per-language CI workflows:

```yaml
# Separate workflow files
ci-validate.yaml   # General validation
ci-python.yaml     # Python-specific
ci-node.yaml       # Node.js-specific
ci-csharp.yaml     # C#-specific
ci-cli.yaml        # CLI binary
```

## Publishing to Package Registries

### Approach 1: minijinja (separate workflows, tag-triggered)

Two independent workflows, both triggered by the same git tag:

```
git tag v2.12.0 && git push --tags
    │
    ├── release.yml (cargo-dist)
    │   ├── Plan release
    │   ├── Build binaries (macOS ARM64/x64, Linux x64/ARM64, Windows)
    │   ├── Create GitHub Release with artifacts
    │   └── Publish to crates.io
    │
    └── build-wheels.yml (maturin)
        ├── Build macOS universal2 wheel
        ├── Build Linux wheels (i686, x86_64, armv7l, aarch64)
        ├── Build Windows wheels (x64, x86)
        ├── Build sdist
        └── Publish to PyPI
```

**cargo-dist workflow** (auto-generated):

```yaml
# .github/workflows/release.yml
on:
  push:
    tags:
      - '**[0-9]+.[0-9]+.[0-9]+*'

jobs:
  plan:
    runs-on: ubuntu-latest
    outputs:
      tag: ${{ steps.plan.outputs.tag }}
    steps:
      - uses: actions/checkout@v4
      - uses: axodotdev/cargo-dist-action@v0

  build-local-artifacts:
    needs: plan
    strategy:
      matrix:
        include:
          - { runner: macos-14,       target: aarch64-apple-darwin }
          - { runner: macos-13,       target: x86_64-apple-darwin }
          - { runner: ubuntu-22.04,   target: x86_64-unknown-linux-gnu }
          - { runner: ubuntu-22.04,   target: aarch64-unknown-linux-gnu }
          - { runner: windows-2022,   target: x86_64-pc-windows-msvc }
    steps:
      - uses: actions/checkout@v4
      - uses: axodotdev/cargo-dist-action@v0

  host:
    needs: [plan, build-local-artifacts]
    steps:
      - uses: axodotdev/cargo-dist-action@v0  # Upload to GitHub Release + crates.io
```

**maturin wheel build** (cross-platform matrix):

```yaml
# .github/workflows/build-wheels.yml
on:
  push:
    tags: ['*']

jobs:
  linux:
    runs-on: ubuntu-latest
    strategy:
      matrix:
        target: [i686, x86_64, armv7l, aarch64]
    steps:
      - uses: PyO3/maturin-action@v1
        with:
          target: ${{ matrix.target }}
          args: --release --out dist
          manylinux: auto

  macos:
    runs-on: macos-14
    steps:
      - uses: PyO3/maturin-action@v1
        with:
          target: universal2-apple-darwin
          args: --release --out dist

  windows:
    runs-on: windows-latest
    strategy:
      matrix:
        target: [x64, x86]
    steps:
      - uses: PyO3/maturin-action@v1
        with:
          target: ${{ matrix.target }}
          args: --release --out dist

  publish:
    needs: [linux, macos, windows]
    runs-on: ubuntu-latest
    permissions:
      id-token: write  # Trusted publishing (OIDC)
    steps:
      - uses: actions/download-artifact@v4
      - uses: pypa/gh-action-pypi-publish@release/v1
```

### Approach 2: kreuzberg (unified workflow, multi-registry)

Single `publish.yaml` orchestrating all registries:

```
workflow_dispatch / GitHub Release published
    │
    ├── 1. Prepare metadata (extract version, validate tag)
    ├── 2. Validate version consistency (all manifests match)
    ├── 3. Check registries (skip if already published)
    │   ├── check-pypi
    │   ├── check-npm
    │   ├── check-cratesio
    │   ├── check-maven
    │   ├── check-nuget
    │   ├── check-packagist
    │   └── check-hex
    ├── 4. Build (parallel matrix builds)
    │   ├── python-wheels (maturin, multi-platform)
    │   ├── node-bindings (napi build, multi-platform)
    │   ├── cli-binaries (cargo build, multi-platform)
    │   ├── go-ffi-libraries
    │   └── elixir-precompiled
    ├── 5. Publish (per-registry)
    │   ├── crates.io (OIDC trusted publishing)
    │   ├── PyPI (OIDC trusted publishing)
    │   ├── npm (NPM_TOKEN secret)
    │   ├── RubyGems (OIDC trusted publishing)
    │   ├── Maven Central (GPG + credentials)
    │   ├── NuGet (NUGET_API_KEY)
    │   ├── Packagist (PACKAGIST_API_TOKEN)
    │   └── Hex.pm (HEX_API_KEY)
    └── 6. Finalize (update release notes, Go module tag)
```

## Authentication Methods by Registry

| Registry | Auth Method | Details |
|----------|-------------|---------|
| **crates.io** | OIDC trusted publishing | `rust-lang/crates-io-auth-action@v1`, no API key needed |
| **PyPI** | OIDC trusted publishing | `pypa/gh-action-pypi-publish@release/v1`, no API key needed |
| **npm** | Token (secret) | `NODE_AUTH_TOKEN` repository secret |
| **RubyGems** | OIDC trusted publishing | `rubygems/configure-rubygems-credentials@v1` |
| **Maven** | GPG + credentials | Traditional signing and upload |
| **NuGet** | API key (secret) | `NUGET_API_KEY` repository secret |

**Trusted publishing (OIDC)** is preferred where available — it eliminates long-lived API keys and
ties authentication to the GitHub Actions workflow identity. crates.io, PyPI, and RubyGems all
support it.

**Trusted publishing caveat**: some registries constrain which GitHub Actions triggers are allowed
for OIDC-based publishing (e.g., only `push`/`workflow_dispatch`, not `pull_request_target`). Treat
this as part of your release design early so you don't have to refactor workflows later.

## napi-rs Cross-Platform Build Matrix

The napi-rs template provides a complete CI setup for building native Node.js addons:

```yaml
# Build matrix for @napi-rs/cli
jobs:
  build:
    strategy:
      matrix:
        include:
          - host: macos-latest
            target: aarch64-apple-darwin
            build: pnpm build --target aarch64-apple-darwin
          - host: macos-latest
            target: x86_64-apple-darwin
            build: pnpm build --target x86_64-apple-darwin
          - host: windows-latest
            target: x86_64-pc-windows-msvc
            build: pnpm build --target x86_64-pc-windows-msvc
          - host: ubuntu-latest
            target: x86_64-unknown-linux-gnu
            build: pnpm build --target x86_64-unknown-linux-gnu
          - host: ubuntu-latest
            target: aarch64-unknown-linux-gnu
            build: |
              sudo apt-get install -y gcc-aarch64-linux-gnu
              pnpm build --target aarch64-unknown-linux-gnu

  publish:
    needs: build
    steps:
      - uses: actions/download-artifact@v4
      - run: npx napi artifacts  # Moves binaries into platform packages
      - run: npm publish --access public
```

Each platform produces a `.node` file that gets packaged into a platform-specific npm package
(e.g., `@iscc/lib-darwin-arm64`). The main `@iscc/lib` package uses `optionalDependencies` to
pull in only the matching platform binary at install time.
