## 2026-11-20 — Review of: Python `encode_seq` docstrings name the non-ASCII case-fold divergence from iscc-core

**Verdict:** PASS

**Summary:** All three Python-visible `encode_seq` docstrings now name the three inputs iscc-lib
rejects and iscc_core accepts: the `__init__.py` wrapper, the PyO3 `///` doc that becomes
`_lowlevel.encode_seq.__doc__`, and the `.pyi` stub. They follow the napi wording, and a new pytest
gates all three. The change is small and limited to the scope (3 non-test files plus 1 test), and no
gate was weakened. The `normal` [human] issue is resolved and deleted. The spec (`rust-core.md`
"Base32 input is strict") already stated the behaviour, so it needed no edit.

**Verification:**

- [x] `uv run maturin develop -m crates/iscc-py/Cargo.toml` exits 0, then
    `uv run pytest tests/test_encode_seq_docs.py` exits 0 with ≥3 assertions: 4 passed (wrapper,
    extension, `.pyi` read via `ast.get_docstring`, plus a self-check of the helper against the old
    two-item sentence)
- [x] `python -c "… assert 'case-fold' in d and 'spare bits' in d"`: printed `ok`
- [x] `uv run pytest tests/test_data_edge.py`: 77 passed
- [x] `cargo clippy -p iscc-py -- -D warnings`: clean
- [x] `mise run check`: exit 0, and the tree stayed clean (only the runner's `iterations.jsonl`
    changed)
- [x] Probe: iscc-core 1.5.0 (`--python 3.13`) gives `ſ` and `ß` → `300161ae0d37a521bcba` (the ASCII
    twin), so the claim "iscc_core accepts them" is true for the named examples

**Issues found:**

- Wording nit, not filed: "case-fold alias" is the project's term (spec, decision, napi, wasm, Go,
    JNI), but the reference accepts only characters whose `str.upper()` is ASCII base32. The capital
    sharp s `ẞ` (U+1E9E) case-folds to `ss`, yet iscc_core rejects it too ("should contain only
    ASCII characters"). Callers get no wrong result, since both libraries reject it. The `docs/`
    pages already use the more precise "upper-casing is ASCII-only". Recorded in learnings.

**Codex review:** One P2 finding: "case-fold" overstates which aliases the reference accepts, so the
docstrings should say "characters that upper-case to ASCII". The finding is correct (I reproduced
it), but the term is the spec's and every other surface's vocabulary. Changing only Python would
make the surfaces disagree, so I treat it as the wording nit above rather than a defect.

**Next:** the `wasm-bindings.md` spec drift (`normal` [review]): drop the
`crates/iscc-wasm/package.json` tree line, and drop the Version Sync row or point it at the real
path (`Cargo.toml` → wasm-pack → release.yml). This is a file-list fact, so the spec edit needs no
escalation. After that comes the derived-facts parity check (`normal` [human]). Dependabot:
`soupsieve` waits on Titusz, but `setuptools` 84.1.0 declares `>=3.10` and could be bumped alone
(`uv lock --upgrade-package setuptools`, untested).

**Notes:**

- The remaining `normal` issues are: wasm spec drift, derived-facts parity, and Dependabot (partly
    human-gated). Everything else is `low`.
- The larger-step trial (decisions.md 2026-11-19) is evaluated after iteration 224. This iteration
    is one more PASS in that window.
