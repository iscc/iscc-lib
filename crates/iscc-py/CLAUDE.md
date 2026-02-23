# CLAUDE.md — iscc-py

PyO3/maturin Python bindings for the `iscc-lib` Rust core, published to PyPI as `iscc-lib`.

## Crate Role

- Binding crate in the hub-and-spoke model: depends on `iscc-lib` (pure Rust core), adds PyO3
    wrappers
- Published to PyPI only (`publish = false` in Cargo.toml prevents crates.io publishing)
- The Python package `iscc_lib` is a drop-in replacement for `iscc-core` (the reference
    implementation)
- Callers replace `import iscc_core as ic` with `import iscc_lib as ic` for identical behavior

## abi3-py310 Constraint

- PyO3 `abi3-py310` feature is set in workspace root `Cargo.toml`
- Produces a single wheel per platform instead of one per Python minor version
- Minimum Python version: `>=3.10` (declared in `pyproject.toml`)
- Do NOT add features that require newer CPython APIs or per-version builds

## Module Layout

```
crates/iscc-py/
  Cargo.toml              # cdylib crate, lib name = "_lowlevel"
  pyproject.toml           # maturin build backend, module-name = "iscc_lib._lowlevel"
  src/
    lib.rs                 # All PyO3 bindings: 21 #[pyfunction]s + 2 #[pyclass]es + #[pymodule]
  python/iscc_lib/
    __init__.py            # Public API: wrapper functions, IsccResult classes, re-exports
    _lowlevel.pyi          # Type stubs for the native Rust extension module
    _lowlevel.abi3.so      # Build artifact (gitignored in CI, present after maturin develop)
    py.typed               # PEP 561 marker for type checker discovery
```

### Rust layer (`src/lib.rs`)

- Single file containing all bindings (no sub-modules)
- `#[pymodule(name = "_lowlevel")]` entry point registers all symbols
- All `gen_*_v0` functions return `PyDict` (not structured Rust types)
- Errors map to `PyValueError` via `.map_err(|e| PyValueError::new_err(e.to_string()))`
- Streaming types (`DataHasher`, `InstanceHasher`) use `Option<inner>` pattern for one-shot finalize

### Python layer (`python/iscc_lib/__init__.py`)

- Imports `_lowlevel` functions as private names (e.g., `_gen_meta_code_v0`)
- Wraps each in a public function that returns a typed `IsccResult` subclass
- `gen_data_code_v0` and `gen_instance_code_v0` accept `bytes | BinaryIO` (call `.read()` for
    streams)
- `DataHasher` and `InstanceHasher` are Python wrapper classes that delegate to `_lowlevel`
    counterparts
- Algorithm functions and utilities re-exported directly (no wrapping needed)

## Type Mapping: Rust to Python

| Rust type (core API)                                      | PyO3 binding                                                            | Python public API                                  |
| --------------------------------------------------------- | ----------------------------------------------------------------------- | -------------------------------------------------- |
| `MetaCodeResult` struct                                   | `PyDict` with `iscc`, `name`, `metahash`, optional `description`/`meta` | `MetaCodeResult(IsccResult)`                       |
| `TextCodeResult` struct                                   | `PyDict` with `iscc`, `characters`                                      | `TextCodeResult(IsccResult)`                       |
| `DataCodeResult` struct                                   | `PyDict` with `iscc`                                                    | `DataCodeResult(IsccResult)`                       |
| `InstanceCodeResult` struct                               | `PyDict` with `iscc`, `datahash`, `filesize`                            | `InstanceCodeResult(IsccResult)`                   |
| `IsccCodeResult` struct                                   | `PyDict` with `iscc`                                                    | `IsccCodeResult(IsccResult)`                       |
| `MixedCodeResult` struct                                  | `PyDict` with `iscc`, `parts`                                           | `MixedCodeResult(IsccResult)`                      |
| `ImageCodeResult` / `AudioCodeResult` / `VideoCodeResult` | `PyDict` with `iscc`                                                    | Corresponding `IsccResult` subclass                |
| `iscc_lib::Error`                                         | `PyValueError`                                                          | `ValueError`                                       |
| `Vec<u8>` (from `soft_hash_video_v0`)                     | `PyBytes`                                                               | `bytes`                                            |
| `DataHasher` / `InstanceHasher`                           | `#[pyclass]` with `Option<inner>`                                       | Python wrapper class accepting `bytes \| BinaryIO` |

### IsccResult hierarchy

- `IsccResult(dict)` base class with `__getattr__` for attribute-style access
- 9 typed subclasses with class-level annotations for IDE completion
- `isinstance(result, dict)` is `True` -- full dict protocol compatibility
- `json.dumps(result)` works without custom serializer

## API Compatibility Rules

- Every `gen_*_v0` function must return a dict with the exact same keys and value types as
    `iscc-core`
- Dict keys are the contract: `iscc`, `name`, `metahash`, `description`, `meta`, `characters`,
    `parts`, `datahash`, `filesize`
- Optional keys (`description`, `meta`) must be absent (not `None`) when not provided
- Bindings must NOT define semantics -- they translate the core API only
- Conformance vectors from `crates/iscc-lib/tests/data.json` are the correctness baseline
- All 9 `gen_*_v0` functions must match iscc-core output for every vector

## Build Commands

```bash
# Development build (compiles Rust, installs into current venv)
maturin develop -m crates/iscc-py/Cargo.toml

# Run Python tests
pytest

# Run all tests (Rust + Python)
mise run test

# Lint
mise run lint

# Format
mise run format
```

## Test Patterns

- Tests live in `/workspace/iscc-lib/tests/` (project root), not inside the crate
- `test_conformance.py` -- parametrized tests for all 9 `gen_*_v0` against `data.json` vectors
- `test_smoke.py` -- basic functionality, IsccResult hierarchy, attribute access, BinaryIO streaming
- `test_text_utils.py` -- text utility functions
- `test_algo.py` -- algorithm primitives
- `test_streaming.py` -- DataHasher and InstanceHasher streaming
- All tests use `from iscc_lib import ...` (the public Python API, not `_lowlevel`)
- Tests verify both `result["key"]` dict access and `result.key` attribute access
- Stream tests verify `BytesIO` input produces identical output to `bytes` input

### Conformance verification

- `data.json` is vendored at `crates/iscc-lib/tests/data.json` (no network access in tests)
- Streaming inputs encoded as `stream:<hex>` in vectors -- decoded with `bytes.fromhex()`
- `conformance_selftest()` function runs all vectors from Rust and returns `True`/`False`

## Publishing Constraints

- OIDC trusted publishing to PyPI (no long-lived API keys)
- One wheel per platform via `abi3-py310`
- Build matrix: Linux (i686, x86_64, armv7l, aarch64), macOS (universal2), Windows (x64, x86)
- sdist also published
- Tag-triggered: `git tag v*` triggers `.github/workflows/build-wheels.yml`
- Release profile: `lto = true`, `codegen-units = 1`, `strip = true`, `panic = "abort"`

## Common Pitfalls

- Do NOT add PyO3 or any binding dependency to the `iscc-lib` core crate
- Do NOT return Rust enums directly to Python (they break when variants are added)
- Do NOT expose `pub(crate)` internal modules through bindings -- only Tier 1 crate-root functions
- Do NOT add keys to result dicts beyond what iscc-core returns
- Do NOT return `None` for optional keys -- omit the key entirely (no `set_item` call)
- Do NOT change `_lowlevel` to return structured types -- it must return plain `PyDict`
- Do NOT add `__init__` params to `_lowlevel` DataHasher/InstanceHasher -- stream handling is in the
    Python wrapper
- Do NOT let Rust panics cross FFI boundary -- always convert errors with `.map_err()`
- Do NOT use `serde_json` for dict construction -- use `PyDict::new()` and `set_item()`
- When adding a Tier 1 function: add to `lib.rs` (#[pyfunction] + register in module),
    `_lowlevel.pyi` (stub), `__init__.py` (wrapper + re-export + `__all__`), and tests
- The `sliding_window` width < 2 check is in the PyO3 layer (not core) to raise `ValueError`
