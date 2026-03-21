## 2026-03-21 — Add missing language tabs to docs landing page

**Done:** Added 5 new language tabs (Ruby, C#, C++, Swift, Kotlin) to the Quick Start section of
`docs/index.md`, reordered existing tabs to match target spec order (Python, Rust, Ruby, Node.js,
WASM, Go, Java, C#, C++, Swift, Kotlin), and added Swift and Kotlin rows to the Available Bindings
table.

**Files changed:**

- `docs/index.md`: Added 5 Quick Start tabs with install + code examples; added 2 rows (Swift,
    Kotlin) to Available Bindings table between C/C++ and WebAssembly; updated Multi-language
    feature bullet to include Swift and Kotlin; reordered tabs to match target spec

**Verification:**

- `grep -c '=== "' docs/index.md` → 11 (was 6) ✓
- `grep '=== "Ruby"' docs/index.md` → match ✓
- `grep '=== "C#"' docs/index.md` → match ✓
- `grep '=== "C++"' docs/index.md` → match ✓
- `grep '=== "Swift"' docs/index.md` → match ✓
- `grep '=== "Kotlin"' docs/index.md` → match ✓
- `grep -c "Swift" docs/index.md` → 3 (≥2) ✓
- `grep -c "Kotlin" docs/index.md` → 3 (≥2) ✓
- `uv run zensical build` → exits 0 ✓
- `mise run format` → no changes ✓
- `mise run check` → all 15 hooks pass ✓

**Next:** The remaining `normal` issue is the Swift `GITHUB_REF_NAME` bug which is blocked on human
review. The only other issue (`low`) is docs logos. Consider creating a PR from develop → main if
this is the last docs cleanup step, or look for other docs/test gaps to address.

**Notes:**

- Tab order was changed from the original (Python, Rust, Node.js, Java, Go, WASM) to match target
    spec order (Python, Rust, Ruby, Node.js, WASM, Go, Java, C#, C++, Swift, Kotlin). This is a
    cosmetic reordering — no content was lost.
- mdformat auto-adjusted the Available Bindings table column widths and wrapped the Multi-language
    bullet to fit line length — expected behavior.
- No test files needed — this is a docs-only change with no code modifications.
