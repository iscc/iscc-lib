# Language Bindings Architecture

Binding strategies for Python, Node.js, WASM, and C FFI, plus automated binding generators.

## Python (PyO3 + maturin) — Both Projects

This is the most mature Rust→Python binding ecosystem. Both projects use the same stack.

**Key pattern from minijinja**: expose a low-level Rust module (`_lowlevel`), then wrap it in pure
Python for a Pythonic API.

```
crates/iscc-py/
├── Cargo.toml              # PyO3 dependency, cdylib crate type
├── pyproject.toml           # maturin build backend
├── src/
│   ├── lib.rs              # #[pymodule] entry point → iscc._lowlevel
│   ├── code.rs             # ISCC-CODE Python class
│   └── units.rs            # ISCC-UNIT types
└── python/iscc/            # Pure Python wrapper
    ├── __init__.py          # Public API (imports from _lowlevel)
    ├── _internal.py         # Internal helpers
    └── py.typed             # PEP 561 marker
```

**pyproject.toml pattern:**

```toml
[build-system]
requires = ["maturin>=1.0,<2.0"]
build-backend = "maturin"

[project]
name = "iscc-lib"
requires-python = ">=3.10"
classifiers = ["Programming Language :: Rust"]

[tool.maturin]
features = ["pyo3/extension-module"]
python-source = "python"             # Where the pure-Python package lives (e.g., python/iscc/)
module-name = "iscc._lowlevel"
```

**Why this matters**: The `_lowlevel` pattern lets you provide a Pythonic API (with docstrings, type
hints, exceptions) while keeping the Rust module focused on raw FFI. Users get
`from iscc import Code` instead of dealing with Rust-flavored APIs.

**Implementation detail**: if you set `module-name = "iscc._lowlevel"`, the Rust `#[pymodule]`
should use `#[pyo3(name = "_lowlevel")]` so that import paths match the on-disk package layout.

**ABI choice (recommended default)**: strongly consider PyO3's `abi3` mode (e.g. `abi3-py310`) to
publish fewer wheels (one per platform instead of one per Python minor). Only opt out if you need
newer CPython APIs or depend on C extensions that require per-version builds.

## Node.js (napi-rs) — kreuzberg

napi-rs compiles to a native `.node` addon with zero overhead. This is what SWC, Biome, Lightning
CSS, Rspack, and Oxc all use.

**Key pattern**: platform-specific npm packages with a main package that selects the right binary.

```
crates/iscc-node/
├── Cargo.toml              # napi + napi-derive dependencies
├── src/
│   ├── lib.rs              # #[napi] exported functions
│   ├── code.rs
│   └── units.rs
├── package.json            # @napi-rs/cli build config
├── npm/                    # Per-platform packages (auto-generated)
│   ├── darwin-arm64/
│   │   └── package.json    # @iscc/lib-darwin-arm64
│   ├── darwin-x64/
│   │   └── package.json    # @iscc/lib-darwin-x64
│   ├── linux-x64-gnu/
│   │   └── package.json    # @iscc/lib-linux-x64-gnu
│   ├── linux-arm64-gnu/
│   │   └── package.json    # @iscc/lib-linux-arm64-gnu
│   └── win32-x64-msvc/
│       └── package.json    # @iscc/lib-win32-x64-msvc
└── __tests__/
```

**How platform selection works**: The main `@iscc/lib` package has `optionalDependencies` pointing
to each platform package. npm automatically installs only the matching one. The main package's JS
entry point detects the platform and requires the correct native binary.

**napi-rs vs WASM for Node.js**:

- napi-rs: native performance, filesystem access, threading, streaming — ideal for server-side
- WASM: universal but ~2-5x slower, no direct filesystem, limited threading — ideal for browsers

## Browser WASM (wasm-pack + wasm-bindgen) — Both Projects

```
crates/iscc-wasm/
├── Cargo.toml              # wasm-bindgen, publish = false
├── src/lib.rs              # #[wasm_bindgen] exports
└── package.json            # npm package (@iscc/wasm)
```

**Cargo.toml for WASM:**

```toml
[lib]
crate-type = ["cdylib"]

[dependencies]
iscc = { path = "../iscc", default-features = false }
wasm-bindgen = "0.2"
serde-wasm-bindgen = "0.6"

[package.metadata.wasm-pack.profile.release]
wasm-opt = ["-Os"] # Optimize for size
```

**Build targets** (from minijinja-js):

- `wasm-pack build --target bundler` — for webpack/vite
- `wasm-pack build --target web` — for native ESM in browsers
- `wasm-pack build --target nodejs` — fallback for Node.js without native addon

### Wasm Component Model — Decision Record

The Component Model (WASI Preview 2) defines a standard for polyglot Wasm modules with strictly
typed interfaces (WIT files). Tools like `cargo-component` (build) and `jco` (JavaScript
transpilation) can produce a single component that works across runtimes without separate `--target`
builds.

**Decision for iscc-lib:**

- **Now (v0–v1)**: Use `wasm-bindgen` + `wasm-pack`. Mature, browser-focused, well-documented.
- **Evaluate Component Model when** all three criteria are met:
    1. Stable WIT tooling (currently pre-1.0)
    2. Browser runtimes support components natively (currently requires polyfills)
    3. `jco` can produce bundles competitive with `wasm-pack` output size
- **Long-term**: A single component targeting both browser and WASI runtimes would eliminate the
    need for separate `--target` builds and could replace the `iscc-wasm` crate entirely.

Use `wasm-bindgen` for browser-first libraries. Evaluate the Component Model only if you target WASI
runtimes (Wasmtime, Wasmer), want one artifact for JS + Python + other hosts, or browser support is
secondary.

## C FFI (cbindgen) — Both Projects

kreuzberg uses a dedicated FFI crate; minijinja uses `minijinja-cabi`.

```
crates/iscc-ffi/
├── Cargo.toml              # staticlib + cdylib
├── src/lib.rs              # extern "C" functions
├── cbindgen.toml           # Header generation config
└── include/
    └── iscc.h              # Generated C header
```

The C FFI layer is the universal escape hatch — it enables Go (via cgo), Java (via JNI/Panama), C#
(via P/Invoke), PHP (via FFI extension), and any other language with C interop.

## Automated Binding Generators (UniFFI & Diplomat)

The per-language binding crates described above (PyO3, napi-rs, cbindgen) give you maximum control
and the most idiomatic APIs per language, but each crate is a separate maintenance surface. Two
tools offer an alternative: generate bindings for multiple languages from a single Rust API
definition.

**UniFFI** (Mozilla): Uses proc macros (`#[uniffi::export]`) or a UDL definition file to
auto-generate bindings for Python, Kotlin, Swift, and Ruby. Community plugins extend coverage to C#,
Go, and Node.js. Best suited when you need moderate-complexity APIs across many languages and can
accept slightly less idiomatic foreign-language wrappers in exchange for much lower maintenance.

**Diplomat** (ICU4X team): Uses `#[diplomat::bridge]` macros to generate C, C++,
JavaScript/TypeScript, Kotlin, and Python bindings. Designed for zero-copy, high-performance APIs
with strict ownership rules.

### Trade-offs vs. Bespoke Bindings

| Concern                  | Bespoke (PyO3/napi-rs)     | Automated (UniFFI/Diplomat)              |
| ------------------------ | -------------------------- | ---------------------------------------- |
| API idiom                | Fully native feel          | Good but generic                         |
| Maintenance              | One crate per language     | One definition for all                   |
| Performance              | Optimal (direct FFI)       | Near-native (extra indirection)          |
| Ecosystem maturity       | Very mature                | Maturing (UniFFI stable, Diplomat newer) |
| Deep runtime integration | Full (GIL, async, streams) | Limited to generated surface             |

**Recommendation for iscc-lib**: Start with bespoke PyO3 bindings for Python (the primary target
audience) where API ergonomics matter most. Evaluate UniFFI when adding the third or fourth language
binding — it can replace the C FFI crate and generate wrappers for Go, Kotlin, Swift, and C#
simultaneously, which is where the maintenance savings become significant.
