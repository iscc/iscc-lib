## 2026-02-23 — Add Python API reference page with docstrings

**Done:** Created the Python API reference documentation page using mkdocstrings with full
Sphinx-style docstrings for all 9 `gen_*_v0` functions. The page renders signatures with type
annotations, descriptions, parameter tables, and return value documentation.

**Files changed:**

- `crates/iscc-py/python/iscc_lib/_lowlevel.pyi`: Added Sphinx-style docstrings (`:param:`,
    `:return:`) for all 9 functions with descriptions derived from the Rust doc comments in `lib.rs`
- `docs/api.md`: Created API reference page with intro text, usage example, and `::: iscc_lib`
    mkdocstrings directive
- `zensical.toml`: Added `api.md` to nav, fixed `paths` from `crates/iscc-py/python/iscc_lib` to
    `crates/iscc-py/python` (parent directory so griffe can find the `iscc_lib` package), added
    `allow_inspection = false` to force stub-based static analysis, changed `show_root_heading` to
    `false`

**Verification:**

- `uv run zensical build` succeeds, produces `site/api/index.html`
- All 9 `gen_*_v0` functions documented with signatures, type annotations, descriptions, parameter
    tables, and return values
- `cargo test -p iscc-lib`: 143 passed, 0 failed
- `cargo clippy -p iscc-lib -- -D warnings`: clean
- `cargo fmt --check`: clean

**Next:** Add an architecture overview page (`docs/architecture.md`) or set up GitHub Pages
deployment workflow. The documentation site now has a landing page and a complete API reference —
deployment is the next high-value step.

**Notes:**

- The `paths` config in `zensical.toml` was incorrect (pointed to `iscc_lib` directory instead of
    parent `python/` directory). Fixed as part of this step.
- griffe (mkdocstrings backend) was reading docstrings from the compiled `_lowlevel.abi3.so` (PyO3
    embedded docstrings) instead of the `.pyi` stubs. The PyO3 docstrings lack parameter
    annotations. Setting `allow_inspection = false` forces griffe to use static analysis only,
    reading the rich Sphinx-style docstrings from `_lowlevel.pyi`.
- mdformat-mkdocs doesn't properly handle mkdocstrings `:::` directives with inline indented options
    — it collapses them into a single line and escapes underscores. Solved by: (1) using
    `::: iscc_lib` instead of `::: iscc_lib._lowlevel` (no underscore to escape), and (2) moving all
    mkdocstrings options to `zensical.toml` global config (changed `show_root_heading` from `true`
    to `false`), so no inline options are needed. The simple `::: iscc_lib` directive passes
    mdformat cleanly.
