## 2026-02-28 — Convert getting-started tutorial to tabbed multi-language examples

**Done:** Converted the getting-started tutorial from Python-only to tabbed multi-language format
with 7 tabbed sections covering all 6 languages (Python, Rust, Node.js, Java, Go, WASM). Sections
converted: install, conformance selftest, gen_meta_code_v0, iscc_decompose, gen_text_code_v0,
gen_instance_code_v0, and InstanceHasher streaming.

**Files changed:**

- `docs/tutorials/getting-started.md`: Replaced Python-only prerequisites with language-agnostic
    statement. Converted all code examples to `=== "Language"` tabbed format (7 groups × 6 languages
    = 42 tabs). Kept JSON serialization as Python-only admonition. Updated Next Steps to link all 6
    language howto guides. Code snippets pulled from per-language howto guides with consistent
    variable naming ("The Neverending Story" metadata example).

**Verification:**

- `grep -c '=== "Python"'` → 7 (≥ 5 ✓)
- `grep -c '=== "Rust"'` → 7 (≥ 5 ✓)
- `grep -c '=== "Go"'` → 7 (≥ 5 ✓)
- `uv run zensical build` → exits 0 ✓
- `mise run format` → no changes ✓
- `mise run check` → all 14 hooks pass ✓

**Next:** The landing page `docs/index.md` has a stale Go example using the old WASM-bridge API
(`NewRuntime`/`ctx` pattern) instead of the current pure Go API (`iscc.GenTextCodeV0` directly).
This should be updated to match the howto guide. Also, the doc spec tab order says "Python, Rust,
Java, Node.js, WASM" but the landing page uses "Rust, Python, Node.js, Java, Go, WASM" and the
tutorial now uses "Python, Rust, Node.js, Java, Go, WASM" — these should be standardized.

**Notes:** The Go code in `docs/index.md` Quick Start section still shows the old WASM-bridge API
(`iscc.NewRuntime(ctx)`, `rt.GenTextCodeV0(ctx, ...)`). This is out of scope for the current work
package but should be fixed. Filed mentally — the review agent may want to create an issue.
