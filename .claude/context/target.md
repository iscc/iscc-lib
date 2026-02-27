# Target State — iscc-lib

High-performance polyglot implementation of ISO 24138:2024 (ISCC). Every component below is verified
by concrete criteria that agents can check.

## Core Principle — Developer Experience First

Developer experience is the highest priority for every language binding. Each binding must feel
**idiomatic to its target language** — not like a thin FFI wrapper or a mechanical port. Developers
should be able to install with their language's standard package manager and use the API with
patterns natural to their ecosystem.

**Guiding rules:**

- **Zero-friction install**: `cargo add`, `pip install`, `npm install`, `go get`, Maven dependency —
    one command, no external toolchains or manual library management required
- **Idiomatic API surface**: naming conventions, error handling, types, and patterns follow each
    language's conventions (e.g., `snake_case` in Python/Rust, `camelCase` in JS, `PascalCase` in
    Go/Java, `Result<T>` in Rust, exceptions in Python/Java, `error` returns in Go)
- **No cgo, no JNI manual setup, no DLL hell**: prefer pure-language packaging (WASM via wazero for
    Go, bundled native libs in JARs for Java, abi3 wheels for Python) over approaches that leak C
    toolchain requirements to the consumer
- **Self-contained packages**: all platform-specific native code is embedded or bundled — the
    developer never downloads or configures shared libraries separately
- **Conformance parity**: every binding passes the same conformance test vectors, so developers can
    trust that switching languages doesn't change behavior

## Rust Core Crate — `iscc-lib` on crates.io

A pure Rust library (no binding dependencies) publishable to crates.io as
[`iscc-lib`](https://crates.io/crates/iscc-lib). Initial experimental version `0.0.1` is being
published to all registries.

Detailed spec: `.claude/context/specs/rust-core.md`

**Tier 1 API** — 30 public symbols bound in all languages:

- 9 `gen_*_v0` functions with structured return types (matching iscc-core dict fields)
- 4 text utilities: `text_clean`, `text_remove_newlines`, `text_trim`, `text_collapse`
- 4 algorithm primitives: `sliding_window`, `alg_minhash_256`, `alg_cdc_chunks`, `alg_simhash`
- 1 soft hash: `soft_hash_video_v0`
- 2 encoding utilities: `encode_base64`, `json_to_data_url`
- 3 codec operations: `iscc_decompose`, `encode_component`, `iscc_decode`
- 4 algorithm constants: `META_TRIM_NAME`, `META_TRIM_DESCRIPTION`, `IO_READ_SIZE`,
    `TEXT_NGRAM_SIZE`
- 2 streaming types: `DataHasher`, `InstanceHasher`
- 1 diagnostic: `conformance_selftest`

**Tier 2 API** — `codec` module (Rust-only, not bound): `MainType`, `SubType`, `Version` enums,
`encode_header`, `decode_header`, `encode_base32`, `decode_base32`, `encode_length`,
`decode_length`, `encode_units`.

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

## Go Bindings — Go module

A pure Go module consumable via `go get` (e.g., `go get github.com/iscc/iscc-lib/packages/go`)
providing idiomatic, first-class Go access to all ISCC functions. This is a native Go implementation
— not an FFI wrapper or WASM bridge.

**Architecture:**

- Pure Go implementation of all ISCC algorithms (CDC, MinHash, SimHash, DCT, WTA-Hash)
- No CGO, no WASM, no embedded binaries — just Go source code
- Idiomatic Go API with Go naming conventions, `error` returns, `[]byte` slices, and `io.Reader`
    support for streaming
- Lives in this repository under `packages/go/` as a Go sub-module
- Validated against the same conformance test vectors as all other bindings

**Why pure Go (not WASM/wazero bridge):**

- **Zero distribution friction** — `go get` fetches source code only, no binary artifacts in git
- **Native performance** — compiled to machine code, no WASM interpreter overhead. BLAKE3 and xxHash
    Go libraries have SIMD-optimized implementations
- **First-class debugging** — Go developers can step into ISCC code, profile it, read the source
- **Cross-compilation works** — `GOOS=linux GOARCH=arm64 go build` just works
- **No build artifacts in git** — the WASM approach required committing a ~700KB binary to the
    repository, polluting git history and weakening large-file guards

**Go dependencies (all well-maintained, pure Go):**

- `github.com/zeebo/blake3` — BLAKE3 cryptographic hash
- `github.com/cespare/xxhash/v2` or equivalent — xxHash for feature hashing
- `golang.org/x/text/unicode/norm` — Unicode NFKC/NFD normalization
- `encoding/base32`, `encoding/base64`, `encoding/hex`, `encoding/json` — standard library

**Implementation scope** (~6,300 lines of Rust to port):

- Codec: header encode/decode, base32, component encoding, ISCC decomposition
- Text utilities: Unicode normalization, cleaning, trimming, collapsing
- Algorithms: CDC (gear rolling hash), MinHash (64-dim universal hash), SimHash (bit-vote), DCT
    (Nayuki fast recursive), WTA-Hash (video fingerprinting)
- 9 `gen_*_v0` code generation functions
- Streaming: `DataHasher` and `InstanceHasher` with `io.Reader` support
- Conformance selftest

**Verified when:**

- `go test ./...` passes all conformance vectors from `iscc-core/data.json`
- Output of every `gen_*_v0` function matches `iscc-core` reference for every test vector
- Package installs cleanly via `go get` with no external dependencies beyond Go modules
- No cgo required (`CGO_ENABLED=0` works)
- Cross-compilation works (`GOOS`/`GOARCH` combinations)
- All 30 Tier 1 symbols are accessible with idiomatic Go types and error handling
- API uses Go conventions: `GenMetaCodeV0`, `GenTextCodeV0`, exported types with PascalCase, `error`
    return values
- No binary artifacts committed to the repository
- `check-added-large-files` threshold at 256KB (no need for inflated limits)
- `go vet ./...` clean

## README

The repository README (`README.md`) is the project's public-facing entry point, written for
developers who want to adopt ISCC in their ecosystem. It follows the same structural pattern as the
`iscc/iscc-core` README but targets a polyglot audience — Rust, Python, Java, Go, Node.js, WASM, and
C developers.

**Structure** (modeled after iscc-core README):

- **Experimental notice**: prominent warning that the library is v0.0.x and APIs may change
- **Badges**: CI status, crate/package version badges for all published packages
- **Tagline**: one-line description emphasizing polyglot, high-performance, ISO 24138
- **Key Features**: similarity-preserving, multi-level identification, self-describing, ISO
    standardized, polyglot (Rust + Python + Java + Node.js + WASM + C FFI), conformance-tested
- **What is the ISCC**: brief explanation of ISCC purpose and capabilities (reuse iscc-core text)
- **What is iscc-lib**: explains this is a high-performance polyglot implementation, relationship to
    `iscc-core` reference, and which ecosystems it serves
- **ISCC Architecture**: architecture diagram (reuse iscc-core diagram or link to docs site)
- **ISCC MainTypes**: table of main types (reuse from iscc-core)
- **Installation**: per-language install instructions (Rust/cargo, Python/pip, Java/Maven, Go/go
    get, Node.js/npm, WASM/npm) — use tabbed or sectioned format
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
- Contains per-language installation instructions (Rust, Python, Java, Go, Node.js, WASM)
- Contains per-language quick start code examples
- Links to documentation site (`lib.iscc.codes`)
- Does not contain development workflow content (CID loop, dev container, pre-commit hooks)
- Lists the 9 `gen_*_v0` conformance entry points
- Includes ISCC architecture diagram and MainTypes table

## Per-Crate READMEs

Each publishable crate and binding package has its own `README.md` tailored to its registry
audience. These READMEs are what developers see on crates.io, PyPI, npm, and Maven Central — they
must stand alone without requiring the reader to visit the repository.

**Crates and their registries:**

| Crate / Package    | Registry                   | Audience                |
| ------------------ | -------------------------- | ----------------------- |
| `crates/iscc-lib`  | crates.io                  | Rust developers         |
| `crates/iscc-py`   | PyPI                       | Python developers       |
| `crates/iscc-napi` | npm                        | Node.js developers      |
| `crates/iscc-wasm` | npm                        | Browser/WASM developers |
| `crates/iscc-ffi`  | (not published separately) | C/C# integrators        |
| `crates/iscc-jni`  | Maven Central              | Java/JVM developers     |
| `packages/go`      | Go module proxy            | Go developers           |

**Each per-crate README contains:**

- **Package name and tagline**: what this package is, one line
- **Badges**: registry version, CI status, license
- **What is ISCC**: brief explanation (2-3 sentences, can be shared text)
- **Installation**: registry-specific install command (e.g., `cargo add iscc-lib`,
    `pip install   iscc-lib`, `npm install @iscc/lib`)
- **Quick start**: minimal code example in the crate's target language showing `gen_meta_code_v0`
- **API overview**: list of the 9 `gen_*_v0` functions and key utilities available in this binding
- **Links**: link to documentation site (`lib.iscc.codes`), repository, and the ISCC specification
- **License**: Apache-2.0

**Registry integration:**

- Rust: `Cargo.toml` `readme` field points to crate-local `README.md`
- Python: `pyproject.toml` `readme` field points to crate-local `README.md`
- Node.js/WASM: `package.json` `readme` field (npm auto-detects `README.md` in package root)
- Java: Maven Central does not render READMEs, but the file serves as the crate's documentation
    entry point and is linked from the POM's `<description>` and `<url>` fields

**Verified when:**

- Every publishable crate directory contains a `README.md`
- Each README has installation instructions specific to its registry
- Each README has a quick start code example in the target language
- Registry metadata (`Cargo.toml`, `pyproject.toml`, `package.json`) references the local README
- READMEs render correctly on their respective registry pages

## Documentation

Project documentation hosted at lib.iscc.codes with ISCC branding and agent-friendly features
matching `iscc/iscc-usearch` at `usearch.iscc.codes`.

Detailed spec: `.claude/context/specs/documentation.md`

**Verified when:**

- Site builds and deploys via GitHub Pages
- ISCC branding (colors, logo, favicon, dark mode) matches iscc-usearch
- Covers Rust API, Python API, architecture, and per-language how-to guides
- All code examples use tabbed multi-language format (Python, Rust, Java, Go, Node.js, WASM)
- Copy-page feature and `llms-full.txt` generation for agent consumption
- Navigation follows Diátaxis framework (tutorials, howto, explanation, reference)
- Development section covers: dev container setup, CID workflow, quality gates, project structure

## Benchmarks

Performance comparisons against the Python reference implementation.

**Verified when:**

- criterion benchmarks exist for all Rust `gen_*_v0` functions
- pytest-benchmark compares Python bindings vs `iscc-core`
- Speedup factors published in documentation

## CI/CD, Dev Tooling, and Publishing

Automated quality gates, developer tooling, and selective publishing pipelines.

Detailed spec: `.claude/context/specs/ci-cd.md`

**Verified when:**

- All quality gates run automatically on push/PR (Rust, Python, Node.js, WASM, C FFI)
- `workflow_dispatch` with per-registry checkboxes (crates.io, PyPI, npm) works
- Tag push `v*.*.*` triggers all publish jobs
- crates.io and PyPI publishing via OIDC trusted publishing (no API keys)
- npm publishing via `NPM_TOKEN` secret for `@iscc/lib` and `@iscc/wasm`
- Publishing an already-published version skips gracefully
- All CI workflows green
- All packages versioned at `0.0.1` (experimental) with coordinated version across manifests

## Key Resources

- **Existing Rust code**: `bio-codes/iscc-sum` — production-quality, optimized Rust for Data-Code
    and Instance-Code (CDC, MinHash, BLAKE3, streaming). Use as foundation, adapt rather than
    rewrite from Python. Available via deepwiki MCP.
- **Python reference**: `iscc/iscc-core` — the conformance baseline. Available via deepwiki MCP.
- **Conformance vectors**:
    `https://raw.githubusercontent.com/iscc/iscc-core/master/iscc_core/data.json`
- **Architecture notes**: @notes/00-overview.md — design decisions, tooling stack, workspace layout.
