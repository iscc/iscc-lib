## 2026-02-25 — Expand docs landing page to all 6 languages

**Done:** Added Node.js, Java, Go, and WASM tabs to the Quick Start section in `docs/index.md` and
added Java and Go rows to the Available Bindings table. All 6 tabs demonstrate `gen_text_code_v0`
with "Hello World" for consistent cross-language comparison.

**Files changed:**

- `docs/index.md`: Added 4 new Quick Start tabs (Node.js, Java, Go, WASM) after existing Rust and
    Python tabs. Added Java (Maven Central planned, build from source) and Go (Go module) rows to
    the Available Bindings table between Node.js and WebAssembly. Table now has 7 rows.

**Verification:**

- `uv run zensical build` exits 0 — docs site builds without errors
- `grep -c '=== "' docs/index.md` returns 6 — one tab per language
- `grep 'Node.js' docs/index.md` matches — Node.js tab and binding table row present
- `grep 'Java' docs/index.md` matches — Java tab and binding table row present
- `grep 'Go' docs/index.md` matches — Go tab and binding table row present
- `grep 'WASM' docs/index.md` matches — WASM tab present
- `grep 'go get' docs/index.md` matches — Go install command in both tab and table
- Available Bindings table has 7 rows (Rust, Python, Node.js, Java, Go, WebAssembly, C/C++)
- `mise run check` — all 14 pre-commit hooks pass

**Next:** The "All code examples use tabbed multi-language format" gap from target.md is now closed.
The existing Rust and Python Quick Start examples have minor inaccuracies noted in next.md's "Not In
Scope" (Rust shows struct as "JSON string", Python uses `json.loads` unnecessarily) — these could be
fixed in a follow-up step. The project appears ready for the v0.0.1 release PR from `develop` to
`main`.

**Notes:** mdformat auto-reformatted the JS `import` statements in Node.js and WASM tabs to
multi-line style (one import per line with braces). This is the formatter's standard behavior for
code blocks inside markdown — the resulting code is functionally identical and renders correctly on
the docs site. The Go tab has more boilerplate than other languages (runtime setup with context) but
the example is kept minimal while remaining correct per the howto guide patterns.
