# Next Work Package

## Step: Scaffold iscc-py crate with PyO3/maturin and expose gen_instance_code_v0

## Goal

Create the Python binding crate (`crates/iscc-py/`) with the maturin/PyO3 toolchain and expose
`gen_instance_code_v0` as the first function. This validates the entire build pipeline end-to-end
(Rust → native module → Python import → correct output) and establishes the pattern for all 9
functions.

## Scope

- **Create**: `crates/iscc-py/Cargo.toml`, `crates/iscc-py/src/lib.rs`,
    `crates/iscc-py/pyproject.toml`, `crates/iscc-py/python/iscc/__init__.py`,
    `crates/iscc-py/python/iscc/py.typed`
- **Modify**: root `Cargo.toml` (add `iscc-py` to workspace members, add `pyo3` to
    workspace.dependencies), `crates/iscc-lib/src/lib.rs` (remove unused `NotImplemented` variant)
- **Reference**: `notes/02-language-bindings.md` (PyO3 patterns, `_lowlevel` module, abi3-py310),
    `notes/03-async-and-streaming.md` (sync core principle), root `pyproject.toml` (dev tooling
    config), `crates/iscc-lib/src/lib.rs` (function signatures)

## Implementation Notes

### Crate structure

Follow the `_lowlevel` pattern from `notes/02-language-bindings.md`:

```
crates/iscc-py/
├── Cargo.toml              # cdylib, depends on iscc-lib + pyo3
├── pyproject.toml           # maturin build backend
├── src/
│   └── lib.rs              # #[pymodule] → iscc._lowlevel
└── python/iscc/            # Pure Python wrapper
    ├── __init__.py          # Re-exports from _lowlevel
    └── py.typed             # PEP 561 marker
```

### Root Cargo.toml changes

- Add `"crates/iscc-py"` to `workspace.members`
- Add `pyo3 = { version = "0.23", features = ["abi3-py310"] }` to `workspace.dependencies`

### crates/iscc-py/Cargo.toml

```toml
[package]
name = "iscc-py"
version.workspace = true
edition.workspace = true
publish = false          # Published via PyPI, not crates.io

[lib]
name = "_lowlevel"
crate-type = ["cdylib"]

[dependencies]
iscc-lib = { path = "../iscc-lib" }
pyo3 = { workspace = true, features = ["extension-module"] }
```

### crates/iscc-py/pyproject.toml

```toml
[build-system]
requires = ["maturin>=1.0,<2.0"]
build-backend = "maturin"

[project]
name = "iscc-lib"
requires-python = ">=3.10"
classifiers = [
  "Programming Language :: Rust",
  "Programming Language :: Python :: Implementation :: CPython",
]

[tool.maturin]
features = ["pyo3/extension-module"]
python-source = "python"
module-name = "iscc._lowlevel"
```

### crates/iscc-py/src/lib.rs

- Use `#[pymodule(name = "_lowlevel")]` (PyO3 0.23+ syntax)
- Expose `gen_instance_code_v0(data: &[u8], bits: Option<u32>) -> PyResult<String>` with `bits`
    defaulting to 64 (matching iscc-core Python API defaults)
- Convert `IsccError` to `PyErr` using `pyo3::exceptions::PyValueError`
- Keep it minimal — one function to validate the pipeline

### crates/iscc-py/python/iscc/__init__.py

- Import and re-export `gen_instance_code_v0` from `iscc._lowlevel`
- Use `__all__` to declare public API
- Add module docstring

### crates/iscc-py/python/iscc/py.typed

- Empty file (PEP 561 marker for type checkers)

### Test update

Update `tests/test_smoke.py` to test the exposed function via
`from iscc import gen_instance_code_v0`. Verify the result matches the known conformance value:
`gen_instance_code_v0(b"", 64)` should return `"ISCC:IAA26E2JXH27TING"`.

### Cleanup

Also remove the unused `IsccError::NotImplemented` variant from `crates/iscc-lib/src/lib.rs` as
noted in the handoff — all 9 gen functions are implemented, so this variant is dead code.

### Build pipeline

The advance agent must install maturin into the dev environment. Check if it's already available via
`uv pip install maturin` or add it to pyproject.toml dev dependencies. Build with
`maturin develop --manifest-path crates/iscc-py/Cargo.toml` to install the module into the active
Python environment.

### Important: maturin manifest-path

When running `maturin develop`, point it at the iscc-py crate specifically:
`maturin develop --manifest-path crates/iscc-py/Cargo.toml`

## Verification

- `cargo build -p iscc-py` compiles successfully
- `maturin develop --manifest-path crates/iscc-py/Cargo.toml` installs the module
- `python -c "from iscc import gen_instance_code_v0; print(gen_instance_code_v0(b'', 64))"` prints
    `ISCC:IAA26E2JXH27TING`
- `pytest tests/test_smoke.py` passes with the new conformance test
- `cargo clippy -p iscc-py -- -D warnings` is clean
- `cargo test -p iscc-lib` still passes all 143 tests (after `NotImplemented` removal)
- `IsccError::NotImplemented` variant is removed from iscc-lib

## Done When

The advance agent is done when `maturin develop` builds successfully, the Python import works, the
conformance value is correct, and pytest passes.
