## 2026-02-27 — Review of: Add Codec operations and Constants sections to binding howto guides

**Verdict:** PASS_WITH_NOTES

**Summary:** Clean documentation-only update adding "Codec operations" and "Constants" sections to
all 4 binding howto guides (Python, Node.js, Java, WASM). Content is well-structured, follows the Go
template, and uses language-idiomatic examples. One factual error fixed during review: WASM
constants were documented with lowercase names but the actual `js_name` exports use uppercase.

**Verification:**

- [x] `grep -c 'encode_component\|encodeComponent\|EncodeComponent' docs/howto/python.md` → 2
- [x] `grep -c 'iscc_decode\|isccDecode' docs/howto/python.md` → 3
- [x] `grep -c 'META_TRIM_NAME' docs/howto/python.md` → 2
- [x] `grep -c 'core_opts' docs/howto/python.md` → 7
- [x] `grep -c 'encode_component' docs/howto/nodejs.md` → 2
- [x] `grep -c 'iscc_decode' docs/howto/nodejs.md` → 3
- [x] `grep -c 'META_TRIM_NAME' docs/howto/nodejs.md` → 2
- [x] `grep -c 'encodeComponent' docs/howto/java.md` → 1
- [x] `grep -c 'isccDecode' docs/howto/java.md` → 2
- [x] `grep -c 'META_TRIM_NAME' docs/howto/java.md` → 1
- [x] `grep -c 'encode_component' docs/howto/wasm.md` → 2
- [x] `grep -c 'iscc_decode' docs/howto/wasm.md` → 3
- [x] `grep -c 'meta_trim_name' docs/howto/wasm.md` → 2 (now via uppercase `META_TRIM_NAME`)
- [x] `uv run zensical build` → exits 0
- [x] `mise run check` → all 14 hooks pass

**Issues found:**

- Fixed: WASM constants section used lowercase function names (`meta_trim_name()`) but the actual
    `wasm_bindgen` exports use uppercase via `js_name = "META_TRIM_NAME"`. Corrected to uppercase
    `META_TRIM_NAME()` etc.
- Fixed: Advance agent memory note incorrectly stated WASM constants use lowercase — corrected.
- Note: `softHashVideoV0` appears in both "Algorithm primitives" and "Other codec methods" sections
    in the Java guide. The existing "Algorithm primitives" section was not in scope for
    modification, and next.md required including it in Codec operations. Cosmetic duplication only.
- Note: Node.js guide mixes ESM `import` (existing sections) with CommonJS `require()` (new Codec
    and Constants sections). This was explicitly specified in next.md and mirrors the Go guide's
    pattern of showing different import styles. Could confuse users in ESM-only projects — consider
    unifying to ESM in a future doc refresh.

**Codex review:** Codex identified the same WASM constant naming issue (P1, fixed), the ESM/CJS
mixing in Node.js (P2, by-design per next.md spec but noted), and the advance agent memory
inaccuracy (P3, fixed).

**Next:** All 6 language binding howto guides now have complete documentation coverage (30/30 Tier 1
symbols with Codec operations and Constants). The project can focus on remaining issues in
issues.md: dict auto-serialization for `gen_meta_code_v0` (issue #5), `encode_component` API
promotion (issue #6), `iscc_decode` implementation (issue #7), or constants exposure (issue #8).

**Notes:** The next.md spec incorrectly specified WASM constant names as lowercase — always verify
documented API names against actual `js_name`/`#[pyfunction]`/`#[napi]` attributes in the binding
source code, not just the spec.
