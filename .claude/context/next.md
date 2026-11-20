# Next Work Package

## Step: Python `encode_seq` docstrings name the non-ASCII case-fold divergence from iscc-core

## Goal

Resolve the `normal` [human] issue "Python `encode_seq` docstrings omit the case-fold divergence
from iscc-core". All three Python-surface `encode_seq` docstrings must name non-ASCII case-fold
aliases (`ſ`, `ß`, ...) next to padding and spare bits as inputs iscc-lib rejects and iscc_core
accepts. Python is the last surface whose `encode_seq` doc names only two of the three differences.

## Alternatives Considered

- **Chosen:** Python `encode_seq` docstrings. The handoff names it next, it is a user-facing
    published docstring (`help(iscc_lib.encode_seq)`, IDE stubs), and it is small and clean.
- **Rejected:** `wasm-bindings.md` spec drift (`normal`). Also small, but it is a CID-internal spec
    line with no user impact. Next in line. The derived-facts parity check (`normal`) is the largest
    remaining item and comes after both. Dependabot `soupsieve` waits on Titusz.

## Scope

- **Modify**:
    - `crates/iscc-py/python/iscc_lib/__init__.py`: the `encode_seq` wrapper docstring (L120-122)
    - `crates/iscc-py/python/iscc_lib/_lowlevel.pyi`: the `encode_seq` stub docstring (L56-57)
    - `crates/iscc-py/src/lib.rs`: the `///` doc on `#[pyfunction] fn encode_seq` (L515-519). PyO3
        turns it into `_lowlevel.encode_seq.__doc__`, the third Python-visible copy (finish the
        pattern on the surface)
- **Create**: a small pytest (e.g. `tests/test_encode_seq_docs.py`) asserting all three docstrings
    name the case-fold aliases. Tests do not count toward the file budget
- **Reference**: `crates/iscc-napi/src/lib.rs` L83-88 and `crates/iscc-wasm/src/lib.rs` ~L350 (the
    wording other surfaces use); `docs/howto/python.md` L453; decisions.md 2026-11-18 "Codec input
    is upper-cased ASCII-only"; `tests/test_data_edge.py` L54-56 (the pinned vectors)

## Not In Scope

- Docstrings of `iscc_decode`, `iscc_decompose` or `decode_seq`. The reference `iscc_decode` rejects
    non-ASCII aliases too, so there is no divergence to document there (learnings: "the reference's
    reject set differs per entry point")
- Any behaviour change, any change to `data-edge.json` or other surfaces' docs. They already state
    the divergence
- `docs/howto/python.md`. It already says "upper-casing is ASCII-only"
- The bare-C-name nits in `packages/cpp/` and the .NET `ArgumentNullException` doc gap from the last
    handoff

## Implementation Notes

- Wording follows napi: "Base32 decoding is strict: `=` padding, non-zero spare bits and non-ASCII
    case-fold aliases (`ſ`, `ß`) are rejected (`iscc_core` accepts them)." Keep each file's markup
    style: the `.pyi` uses double-backtick reST literals; `__init__.py` and the Rust `///` use
    single backticks. The `__init__.py` text currently ends "(`iscc_core` accepts both)", so "both"
    must become "all three" or "them".
- Non-ASCII characters in the docstrings are fine (UTF-8 sources; napi and wasm already carry `ſ`).
    If ruff flags `RUF001`/`RUF002` (ambiguous characters) for `ſ`, name the code points in words
    ("long s U+017F, sharp s U+00DF") instead of suppressing the rule (decisions.md 2026-07-25:
    findings are fixed at the source, never suppressed).
- The test reads `iscc_lib.encode_seq.__doc__` and `iscc_lib._lowlevel.encode_seq.__doc__` at
    runtime. The second needs a rebuilt extension:
    `uv run maturin develop -m crates/iscc-py/Cargo.toml`. For the `.pyi`, parse the file with `ast`
    and take `ast.get_docstring` of the `encode_seq` FunctionDef. Assert on a stable substring such
    as `"case-fold"` plus `"iscc_core"`. Do not assert an exact sentence.
- Binding-only docstring step: it trips neither the CRAP nor the iai gate.

## Verification

- `uv run maturin develop -m crates/iscc-py/Cargo.toml` exits 0, then
    `uv run pytest tests/test_encode_seq_docs.py` exits 0 with at least 3 assertions covering the
    wrapper, the extension and the `.pyi` docstrings
- `uv run python -c "import iscc_lib; d = iscc_lib.encode_seq.__doc__; assert 'case-fold' in d and 'spare bits' in d"`
    exits 0
- `uv run pytest tests/test_data_edge.py` still exits 0
- `cargo clippy -p iscc-py -- -D warnings` clean
- `mise run check` exits 0 and leaves the tree clean

## Done When

All three Python `encode_seq` docstrings name non-ASCII case-fold aliases next to padding and spare
bits as inputs iscc_core accepts and iscc-lib rejects, and the verification commands pass.
