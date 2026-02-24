# Target State — iscc-lib

High-performance polyglot implementation of ISO 24138:2024 (ISCC). Every component below is verified
by concrete criteria that agents can check.

## Rust Core Crate — `iscc-lib` on crates.io

A pure Rust library (no binding dependencies) publishable to crates.io as
[`iscc-lib`](https://crates.io/crates/iscc-lib). Name is available.

Detailed spec: `.claude/context/specs/rust-core.md`

**Tier 1 API** — 22 public symbols bound in all languages:

- 9 `gen_*_v0` functions with structured return types (matching iscc-core dict fields)
- 4 text utilities: `text_clean`, `text_remove_newlines`, `text_trim`, `text_collapse`
- 4 algorithm primitives: `sliding_window`, `alg_minhash_256`, `alg_cdc_chunks`, `alg_simhash`
- 1 soft hash: `soft_hash_video_v0`
- 1 encoding utility: `encode_base64`
- 1 codec operation: `iscc_decompose`
- 2 streaming types: `DataHasher`, `InstanceHasher`
- 1 diagnostic: `conformance_selftest`

**Tier 2 API** — `codec` module (Rust-only, not bound): `MainType`, `SubType`, `Version` enums,
`encode_header`, `decode_header`, `encode_base32`, `decode_base32`, `encode_component`,
`encode_length`, `decode_length`, `encode_units`.

**Verified when:**

- `cargo test -p iscc-lib` passes with all conformance vectors from `iscc-core/data.json`
- Output of every `gen_*_v0` function matches `iscc-core` reference for every test vector
- All Tier 1 functions produce output matching their iscc-core counterparts
- `DataHasher` / `InstanceHasher` produce identical results to `gen_data_code_v0` /
    `gen_instance_code_v0` for the same input
- `cargo clippy -p iscc-lib -- -D warnings` clean
- `cargo fmt -p iscc-lib --check` clean
- No `unsafe` without documented justification
- Crate has zero binding dependencies (no PyO3, napi, wasm-bindgen)

## Python Bindings — `iscc_lib` on PyPI

A Python package installable from PyPI as [`iscc_lib`](https://pypi.org/project/iscc-lib/), a
drop-in replacement for `iscc-core`. Name is available. (You already own `iscc` and `iscc-core` on
PyPI.)

Detailed spec: `.claude/context/specs/python-bindings.md`

**Verified when:**

- `pip install .` succeeds and exposes all 9 `gen_*_v0` functions
- All functions return `dict` with the same keys and values as iscc-core
- Streaming functions accept both `bytes` and file-like objects
- `pytest` passes the same conformance vectors from Python
- `ruff check` and `ruff format --check` clean
- Single wheel per platform (abi3-py310)

## Node.js Bindings — `@iscc/lib` on npm

An npm package [`@iscc/lib`](https://www.npmjs.com/package/@iscc/lib) exposing all 9 entrypoints as
native addon via napi-rs. Published under the `@iscc` npm org.

**Verified when:**

- `npm test` passes conformance vectors from JavaScript
- Package installs cleanly via `npm install`

## WASM Bindings — `@iscc/wasm` on npm

A browser-compatible WASM package [`@iscc/wasm`](https://www.npmjs.com/package/@iscc/wasm) exposing
all 9 entrypoints via wasm-bindgen. Published under the same `@iscc` npm scope.

**Verified when:**

- Conformance tests pass in a WASM runtime
- Package builds with `wasm-pack`

## C FFI

A C-compatible shared library with generated headers, enabling Go/C# and other language bindings.

**Verified when:**

- cbindgen generates valid C headers
- A C test program can call the entrypoints and get correct results

## Java Bindings — Maven Central

A Java library published to Maven Central (e.g., `io.iscc:iscc-lib`) with bundled native libraries
for all supported platforms. Java/JVM developers add a single dependency to their `pom.xml` or
`build.gradle` and get idiomatic Java access to all ISCC functions — no manual native library
management required.

**Architecture:**

- JNI bridge crate (`iscc-jni`) generates the native interface from the Rust core
- Java wrapper provides idiomatic API (e.g., `IsccLib.genMetaCodeV0("title")`)
- Platform-specific native libraries (linux-x64, linux-aarch64, macos-x64, macos-aarch64,
    windows-x64) bundled inside the JAR under `META-INF/native/`
- Loader class extracts and loads the correct native library at runtime

**Dev environment:** Requires JDK 17+ and Maven (or Gradle) in the devcontainer. Add to
`.devcontainer/Dockerfile` when work on Java bindings begins.

**Verified when:**

- `mvn test` (or `gradle test`) passes conformance vectors from Java
- JAR installs cleanly via Maven/Gradle dependency declaration
- Native libraries load correctly on Linux, macOS, and Windows
- All 9 `gen_*_v0` functions are accessible with idiomatic Java types
- Devcontainer includes JDK and build tool for Java development

## README

The repository README (`README.md`) is the project's public-facing entry point, written for
developers who want to adopt ISCC in their ecosystem. It follows the same structural pattern as the
`iscc/iscc-core` README but targets a polyglot audience — Rust, Python, Java, Node.js, WASM, and C
developers.

**Structure** (modeled after iscc-core README):

- **Badges**: CI status, crate/package version badges for all published packages
- **Tagline**: one-line description emphasizing polyglot, high-performance, ISO 24138
- **Key Features**: similarity-preserving, multi-level identification, self-describing, ISO
    standardized, polyglot (Rust + Python + Java + Node.js + WASM + C FFI), conformance-tested
- **What is the ISCC**: brief explanation of ISCC purpose and capabilities (reuse iscc-core text)
- **What is iscc-lib**: explains this is a high-performance polyglot implementation, relationship to
    `iscc-core` reference, and which ecosystems it serves
- **ISCC Architecture**: architecture diagram (reuse iscc-core diagram or link to docs site)
- **ISCC MainTypes**: table of main types (reuse from iscc-core)
- **Installation**: per-language install instructions (Rust/cargo, Python/pip, Java/Maven,
    Node.js/npm, WASM/npm) — use tabbed or sectioned format
- **Quick Start**: minimal code examples showing `gen_meta_code_v0` in each language
- **Implementors Guide**: link to conformance test vectors and the 9 `gen_*_v0` entry points (same
    list as iscc-core), link to documentation site for detailed per-language guides
- **Documentation**: link to `lib.iscc.codes`
- **Contributing**: contribution guidelines, link to developer chat
- **License**: Apache-2.0
- **Maintainers**: @titusz

**Audience**: The README does NOT describe internal development workflows (CID loop, dev container
setup, quality gates). Those belong in the documentation site under a Development section.

**Verified when:**

- README exists and renders correctly on GitHub
- Contains per-language installation instructions (Rust, Python, Java, Node.js, WASM)
- Contains per-language quick start code examples
- Links to documentation site (`lib.iscc.codes`)
- Does not contain development workflow content (CID loop, dev container, pre-commit hooks)
- Lists the 9 `gen_*_v0` conformance entry points
- Includes ISCC architecture diagram and MainTypes table

## Documentation

Project documentation hosted at lib.iscc.codes with ISCC branding and agent-friendly features
matching `iscc/iscc-usearch` at `usearch.iscc.codes`.

Detailed spec: `.claude/context/specs/documentation.md`

**Verified when:**

- Site builds and deploys via GitHub Pages
- ISCC branding (colors, logo, favicon, dark mode) matches iscc-usearch
- Covers Rust API, Python API, architecture, and per-language how-to guides
- All code examples use tabbed multi-language format (Python, Rust, Java, Node.js, WASM)
- Copy-page feature and `llms-full.txt` generation for agent consumption
- Navigation follows Diátaxis framework (tutorials, howto, explanation, reference)
- Development section covers: dev container setup, CID workflow, quality gates, project structure

## Benchmarks

Performance comparisons against the Python reference implementation.

**Verified when:**

- criterion benchmarks exist for all Rust `gen_*_v0` functions
- pytest-benchmark compares Python bindings vs `iscc-core`
- Speedup factors published in documentation

## CI/CD and Publishing

Automated quality gates and publishing pipelines.

**Verified when:**

- All quality gates run automatically on push/PR
- crates.io and PyPI publishing via OIDC trusted publishing (no API keys)
- All CI workflows green

## Key Resources

- **Existing Rust code**: `bio-codes/iscc-sum` — production-quality, optimized Rust for Data-Code
    and Instance-Code (CDC, MinHash, BLAKE3, streaming). Use as foundation, adapt rather than
    rewrite from Python. Available via deepwiki MCP.
- **Python reference**: `iscc/iscc-core` — the conformance baseline. Available via deepwiki MCP.
- **Conformance vectors**:
    `https://raw.githubusercontent.com/iscc/iscc-core/master/iscc_core/data.json`
- **Architecture notes**: @notes/00-overview.md — design decisions, tooling stack, workspace layout.
