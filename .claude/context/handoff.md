## 2026-11-20 — Python `encode_seq` docstrings name the non-ASCII case-fold divergence from iscc-core

**Done:** All three Python-visible `encode_seq` docstrings now say that strict base32 decoding
rejects `=` padding, non-zero spare bits and non-ASCII case-fold aliases (`ſ`, `ß`), and that
`iscc_core` accepts them. The wording follows napi. A pytest checks all three copies.

**Files changed:**

- `crates/iscc-py/python/iscc_lib/__init__.py`: the wrapper docstring names three items. "accepts
    both" became "accepts them"
- `crates/iscc-py/python/iscc_lib/_lowlevel.pyi`: the stub docstring names three items (reST double
    backticks) and adds "(`iscc_core` accepts them)"
- `crates/iscc-py/src/lib.rs`: the `///` doc on `#[pyfunction] fn encode_seq` (this becomes
    `_lowlevel.encode_seq.__doc__`) names three items plus the `iscc_core` note
- `tests/test_encode_seq_docs.py` (new): 4 tests. Each of wrapper, extension and `.pyi` (read with
    `ast.get_docstring`) must contain `padding`, `spare bits`, `case-fold` and `iscc_core` after
    whitespace normalization. A fourth test checks that the helper flags the old two-item sentence

**Verification:**

- `uv run maturin develop -m crates/iscc-py/Cargo.toml`: exit 0 ("Installed iscc-lib-0.7.0")
- `uv run pytest tests/test_encode_seq_docs.py tests/test_data_edge.py -q`: 81 passed
- `uv run python -c "import iscc_lib; d = iscc_lib.encode_seq.__doc__; assert 'case-fold' in d and 'spare bits' in d"`:
    printed ok
- `cargo clippy -p iscc-py -- -D warnings`: clean
- `mise run check`: exit 0, and the tree is clean apart from the staged changes and the runner's
    `iterations.jsonl`
- `uv run prek run --hook-stage pre-push --files <3 py files>`: exit 0 (ruff S/C901, ty, pytest)

**Next:** the `wasm-bindings.md` spec drift (`normal`, a file-list fact), then the derived-facts
parity check (`normal` [human]).

**Notes:**

- Ruff raised no `RUF001`/`RUF002` on `ſ`/`ß`, so the literal characters stay in the docstrings and
    no code points had to be spelled out in words
- Only docstrings changed, so there is no CRAP or iai effect
- `mise run check` ran before the test file was tracked, so the file was not checked then. At commit
    time ruff format reformatted it (it wrapped the long `_PYI` line). The commit hooks then passed
    on the reformatted file, and the 4 tests still pass
