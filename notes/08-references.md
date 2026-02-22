# References

All tool and project links for the iscc-lib polyglot architecture.

## Projects Studied

- [mitsuhiko/minijinja](https://github.com/mitsuhiko/minijinja) — Template engine with Python, JS/WASM, C bindings
- [kreuzberg-dev/kreuzberg](https://github.com/kreuzberg-dev/kreuzberg) — Document intelligence with 10+ language bindings
- [bio-codes/iscc-sum](https://github.com/bio-codes/iscc-sum) — Existing ISCC Data-Code + Instance-Code Rust implementation with Python bindings

## Tools

- [mise](https://mise.jdx.dev/) — Polyglot tool version manager and task runner
- [cargo-dist](https://github.com/axodotdev/cargo-dist) — Rust release automation
- [maturin](https://github.com/PyO3/maturin) — Build Python packages from Rust
- [PyO3](https://pyo3.rs/) — Rust bindings for Python
- [napi-rs](https://napi.rs/) — Rust bindings for Node.js (N-API)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/) — Rust→WASM packaging
- [cbindgen](https://github.com/mozilla/cbindgen) — Generate C headers from Rust
- [uv](https://docs.astral.sh/uv/) — Fast Python package manager
- [pnpm](https://pnpm.io/) — Efficient Node.js package manager
- [ruff](https://docs.astral.sh/ruff/) — Python linter and formatter
- [ty](https://github.com/astral-sh/ty) — Fast Rust-based Python type checker (Astral)
- [biome](https://biomejs.dev/) — JS/TS linter and formatter
- [prek](https://github.com/j178/prek) — Rust-based pre-commit framework (drop-in replacement)
- [mdformat-mkdocs](https://github.com/KyleKing/mdformat-mkdocs) — Markdown formatter for MkDocs projects
- [zensical](https://github.com/zensical/zensical) — Rust-powered documentation site generator (Material for MkDocs successor)
- [napi-rs/package-template](https://github.com/napi-rs/package-template) — Template for napi-rs npm packages

## Binding Generators

- [UniFFI](https://mozilla.github.io/uniffi-rs/) — Automated multi-language binding generator (Mozilla)
- [Diplomat](https://github.com/rust-diplomat/diplomat) — Zero-copy binding generator (ICU4X team)

## Release and API Stability

- [cargo-semver-checks](https://github.com/obi1kenobi/cargo-semver-checks) — Lint for semver violations in Rust APIs
- [release-plz](https://release-plz.ieni.dev/) — Automated Rust release management

## Security and Supply Chain

- [cargo-deny](https://embarkstudios.github.io/cargo-deny/) — License, advisory, and dependency checks for Rust
- [bandit](https://bandit.readthedocs.io/) — Security linter for Python
- [actions/attest-build-provenance](https://github.com/actions/attest-build-provenance) — SLSA provenance for GitHub releases

## Wasm Component Model

- [cargo-component](https://github.com/bytecodealliance/cargo-component) — Build Wasm Components from Rust
- [jco](https://github.com/bytecodealliance/jco) — JavaScript toolchain for Wasm Components

## FFI and ABI Safety

- [The Rustonomicon: FFI](https://doc.rust-lang.org/nomicon/ffi.html) — Rust FFI safety guidelines
- [The Rust FFI Omnibus](https://jakegoulding.com/rust-ffi-omnibus/) — Practical Rust↔C patterns
- [The Big Book of FFI](https://nrc.github.io/big-book-ffi/) — Survey of Rust FFI tools and tradeoffs

## Trusted Publishing

- [crates.io: Trusted publishing](https://crates.io/docs/trusted-publishing) — OIDC-based publishing for crates.io

## Benchmarking

- [criterion](https://github.com/bheisler/criterion.rs) — Statistical microbenchmarking for Rust
- [pytest-benchmark](https://pytest-benchmark.readthedocs.io/) — pytest plugin for Python benchmarks
- [CodSpeed](https://codspeed.io/) — Continuous benchmarking in CI (criterion + pytest-benchmark support)
- [Bencher](https://bencher.dev/) — Continuous benchmarking and regression detection

## Conformance Vectors

- [ISCC Conformance Vectors](https://core.iscc.codes/conformance/) — Conformance suite overview and encoding notes
- [`iscc/iscc-core` conformance data](https://github.com/iscc/iscc-core/blob/master/iscc_core/data.json) — Canonical `data.json` in repo
- [`data.json` (raw)](https://raw.githubusercontent.com/iscc/iscc-core/master/iscc_core/data.json) — Raw JSON for automation
