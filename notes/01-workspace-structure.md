# Workspace Structure and Cargo Design

Repository layout patterns and Cargo workspace configuration for `iscc-lib`.

## Repository Structure Patterns

### minijinja (focused approach)

Flat workspace — each crate is a top-level directory:

```
minijinja/
├── Cargo.toml              # Workspace root
├── Makefile                 # Task runner
├── CLAUDE.md               # Dev guidance
├── CONTRIBUTING.md
├── scripts/
│   └── bump-version.sh     # Version bumping
├── minijinja/              # Core library (crate)
├── minijinja-cli/          # CLI binary
├── minijinja-contrib/      # Extra filters/functions
├── minijinja-autoreload/   # Dev auto-reload
├── minijinja-py/           # Python bindings (PyO3 + maturin)
│   ├── Cargo.toml
│   ├── pyproject.toml
│   ├── Makefile
│   ├── src/                # Rust source (PyO3 extension)
│   │   ├── lib.rs
│   │   ├── environment.rs
│   │   ├── typeconv.rs
│   │   └── error_support.rs
│   ├── python/minijinja/   # Pure Python wrapper
│   │   ├── __init__.py
│   │   └── _internal.py
│   └── tests/
├── minijinja-js/           # JS/WASM bindings (wasm-pack)
│   ├── Cargo.toml
│   └── package.json
├── minijinja-cabi/         # C bindings (cbindgen)
├── examples/
├── benchmarks/
└── .github/workflows/
    ├── tests.yml
    ├── clippy.yml
    ├── rustfmt.yml
    ├── release.yml          # cargo-dist → crates.io + GitHub Releases
    └── build-wheels.yml     # maturin → PyPI
```

### kreuzberg (comprehensive approach)

Separated `crates/` and `packages/` directories:

```
kreuzberg/
├── Cargo.toml              # Workspace root (workspace.dependencies)
├── pyproject.toml           # Python project root
├── package.json             # Node.js project root
├── Taskfile.yml             # Task runner (go-task)
├── .pre-commit-config.yaml  # Multi-language linting
├── .sdkmanrc                # Tool versions (sdkman)
├── pnpm-lock.yaml
├── uv.lock
├── scripts/
│   ├── sync_versions.py                   # Cross-language version sync
│   └── publish/
│       ├── prepare-release-metadata.sh
│       ├── validate-version-consistency.sh
│       ├── check_pypi.sh
│       ├── check_npm.sh
│       ├── check_cratesio.sh
│       ├── check_maven.sh
│       ├── check_nuget.sh
│       ├── publish-cargo-crate.sh
│       ├── publish-node-packages.sh
│       ├── publish-gems-direct.sh
│       ├── upload-cli-binaries.sh
│       ├── upload-go-libraries.sh
│       └── go/tag-and-push-go-module.sh
├── crates/                  # All Rust crates
│   ├── kreuzberg/           # Core library
│   ├── kreuzberg-ffi/       # C FFI layer
│   ├── kreuzberg-py/        # Python (PyO3)
│   ├── kreuzberg-node/      # Node.js (napi-rs)
│   ├── kreuzberg-wasm/      # Browser WASM (wasm-bindgen)
│   ├── kreuzberg-tesseract/ # OCR integration
│   └── kreuzberg-cli/       # CLI binary
├── packages/                # Non-Rust language packages
│   ├── python/              # Python package (pyproject.toml)
│   ├── ruby/                # Ruby gem
│   ├── go/v4/               # Go module
│   ├── java/                # Maven package
│   ├── csharp/              # NuGet package
│   ├── php/                 # Composer package
│   └── typescript/          # npm package
├── .task/                   # Taskfile includes
│   ├── languages/           # Per-language task definitions
│   │   ├── rust.yml
│   │   ├── python.yml
│   │   ├── node.yml
│   │   └── ...
│   ├── workflows/           # Cross-language orchestration
│   │   ├── build.yml
│   │   ├── test.yml
│   │   └── lint.yml
│   └── tools/               # Tool management tasks
├── e2e/                     # End-to-end tests
├── docs/
├── examples/
└── .github/workflows/
    ├── ci-python.yaml
    ├── ci-node.yaml
    ├── ci-csharp.yaml
    ├── ci-cli.yaml
    ├── ci-validate.yaml
    └── publish.yaml          # Unified multi-registry release
```

### Pattern Comparison

**minijinja pattern** (flat crates):

- Each binding is a Cargo workspace member at the top level
- Simpler to navigate, lower cognitive overhead
- Works well with up to ~8 crates
- Python package files live inside the crate directory

**kreuzberg pattern** (crates/ + packages/):

- Rust crates in `crates/`, language packages in `packages/`
- Clear separation between Rust compilation units and distribution packages
- Scales to many languages without cluttering the root
- Necessary when non-Rust packages need their own build tooling (Maven, .NET, Composer)

**iscc-lib uses kreuzberg's `crates/` pattern** — cleaner once you have 4+ binding crates.

## Cargo Workspace Design

### Workspace Root Cargo.toml

Both projects use a virtual workspace (no package defined at root). kreuzberg uses centralized
dependency management; minijinja manages versions per-crate.

**kreuzberg pattern (recommended)** — centralized workspace dependencies:

```toml
# Cargo.toml (workspace root)
[workspace]
resolver = "2"
members = [
  "crates/iscc",
  "crates/iscc-py",
  "crates/iscc-node",
  "crates/iscc-wasm",
  "crates/iscc-ffi",
  "crates/iscc-cli",
]

[workspace.package]
version = "0.1.0"
edition = "2024"
rust-version = "1.85"
license = "MIT"
repository = "https://github.com/iscc/iscc-lib"

[workspace.dependencies]
# Shared dependencies declared once
blake3 = "1"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
thiserror = "2"
tokio = { version = "1", features = ["rt", "macros"] }
```

### Member Crate Referencing Workspace Dependencies

```toml
# crates/iscc-py/Cargo.toml
[package]
name = "iscc-py"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
publish = false               # Python package published to PyPI, not crates.io

[lib]
name = "iscc"
crate-type = ["cdylib"]

[dependencies]
iscc = { path = "../iscc" }
pyo3 = { version = "0.23", features = ["abi3-py310"] }

# Shared deps from workspace
serde.workspace = true
serde_json.workspace = true
```

### Crate Architecture (Hub-and-Spoke)

```
                    ┌──────────┐
                    │ iscc-cli │
                    └────┬─────┘
                         │
  ┌──────────┐    ┌──────┴──────┐    ┌───────────┐
  │ iscc-py  │───→│    iscc     │←───│ iscc-node │
  │  (PyO3)  │    │   (core)   │    │ (napi-rs) │
  └──────────┘    └──────┬──────┘    └───────────┘
                         │
               ┌─────────┼─────────┐
               │         │         │
          ┌────┴───┐ ┌───┴────┐ ┌──┴───────┐
          │iscc-ffi│ │iscc-wasm│ │  tests   │
          │  (C)   │ │ (WASM) │ │(internal)│
          └────────┘ └────────┘ └──────────┘
```

The core crate (`iscc`) is a pure Rust library with no FFI concerns. Each binding crate depends on
the core and translates its API to the target language. This keeps the core crate clean and
testable.

### Phase 1 Directory Layout

```
iscc-lib/
├── Cargo.toml               # Virtual workspace
├── clippy.toml              # Complexity threshold
├── mise.toml                 # Tool versions + tasks
├── pyproject.toml            # Python project (uv)
├── zensical.toml             # Documentation config
├── .pre-commit-config.yaml   # prek hooks
├── deny.toml                 # cargo-deny config
├── scripts/
│   └── sync_versions.py
├── overrides/
│   └── main.html             # Doc template overrides
├── docs/                     # Documentation (zensical → lib.iscc.codes)
│   ├── index.md
│   ├── stylesheets/extra.css
│   ├── assets/
│   ├── reference/api.md
│   └── development/contributing.md
├── crates/
│   ├── iscc/                 # Core Rust library
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── meta.rs       # Meta-Code
│   │       ├── semantic.rs   # Semantic-Code
│   │       ├── content.rs    # Content-Code
│   │       ├── data.rs       # Data-Code
│   │       ├── instance.rs   # Instance-Code
│   │       └── codec.rs      # Base32 encoding
│   └── iscc-py/              # Python bindings
│       ├── Cargo.toml
│       ├── pyproject.toml
│       ├── src/lib.rs
│       └── python/iscc_lib/
│           ├── __init__.py
│           └── py.typed
└── .github/workflows/
    ├── ci.yml                # Test Rust + Python
    ├── release.yml           # cargo-dist → crates.io
    ├── wheels.yml            # maturin → PyPI
    └── docs.yml              # zensical → GitHub Pages
```
