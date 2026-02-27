## 2026-02-27 — Add Codec operations and Constants sections to binding howto guides

**Done:** Added "Codec operations" and "Constants" documentation sections to all 4 binding howto
guides (Python, Node.js, Java, WASM), achieving cross-language documentation parity with the Go
guide. Each guide covers encode/decode, decompose, additional codec functions, and algorithm
constants with language-idiomatic code examples.

**Files changed:**

- `docs/howto/python.md`: Added Codec operations section (encode/decode with IntEnum types,
    decompose, 3 bullet-point functions), Constants section (direct imports + core_opts namespace),
    updated front matter description and opening paragraph
- `docs/howto/nodejs.md`: Added Codec operations section (encode/decode with IsccDecodeResult,
    decompose, 3 bullet-point functions using require() style), Constants section (destructured
    exports), updated front matter description and opening paragraph
- `docs/howto/java.md`: Added Codec operations section (camelCase methods on IsccLib,
    IsccDecodeResult with int/byte[] fields, decompose, 3 bullet-point methods), Constants section
    (static final int on IsccLib), updated front matter description
- `docs/howto/wasm.md`: Added Codec operations section (encode/decode with Uint8Array,
    IsccDecodeResult, decompose, 3 bullet-point functions), Constants section (getter functions),
    updated front matter description and opening paragraph

**Verification:** All 14 grep checks pass (all ≥ 1). `uv run zensical build` exits 0.
`mise run check` passes all 14 hooks. mdformat auto-formatted some alignment in code comments
(removed extra alignment spaces in constants sections) — changes are clean.

**Next:** All 6 language binding howto guides now have full documentation coverage (30/30 Tier 1
symbols). The project can focus on other target milestones — the remaining issues in issues.md (dict
auto-serialization for gen_meta_code_v0, or adding Algorithm primitives sections to Python, Node.js,
and WASM guides).

**Notes:** The Python guide uniquely documents the `core_opts` SimpleNamespace and IntEnum return
types from `iscc_decode` — these are Python-specific API features not present in other bindings. The
Node.js guide uses `require()` import style in the Codec sections as specified in next.md, while the
existing Code generation sections use ESM `import` — this mirrors the Go guide's pattern of showing
different import styles in different sections.
