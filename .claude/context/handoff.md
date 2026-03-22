## 2026-03-22 — Review of: Add missing language tabs to docs landing page

**Verdict:** PASS

**Summary:** Clean, well-scoped docs-only change. Added 5 new language tabs (Ruby, C#, C++, Swift,
Kotlin) to the Quick Start section, reordered all 11 tabs to match target spec order, and added
Swift and Kotlin rows to the Available Bindings table. All code examples match the howto reference
guides. All 10 verification criteria pass.

**Verification:**

- [x] `grep -c '=== "' docs/index.md` returns 11 — confirmed
- [x] `grep '=== "Ruby"' docs/index.md` — match found
- [x] `grep '=== "C#"' docs/index.md` — match found
- [x] `grep '=== "C++"' docs/index.md` — match found
- [x] `grep '=== "Swift"' docs/index.md` — match found
- [x] `grep '=== "Kotlin"' docs/index.md` — match found
- [x] `grep -c "Swift" docs/index.md` returns 3 (≥2) — tab + table + multi-language bullet
- [x] `grep -c "Kotlin" docs/index.md` returns 3 (≥2) — tab + table + multi-language bullet
- [x] `uv run zensical build` exits 0 — docs site builds successfully
- [x] `mise run format` — no changes (formatting clean)
- [x] `mise run check` — all 15 pre-commit hooks pass

**Issues found:**

- (none)

**Codex review:** Three P2 findings, all evaluated as advisory (pre-existing, not regressions):

1. **Swift SPM `from: "0.3.1"`**: v0.3.1 tag predates the Swift package. However, the landing page
    mirrors the howto/swift.md exactly (same version string). Will be correct on next release.
2. **Kotlin Maven not yet published**: The howto/kotlin.md has an explicit "Not yet published"
    caveat. Landing page follows the next.md template. Temporary state — will resolve on next
    release.
3. **C++ `#include <iscc/iscc.hpp>` path**: The howto/c-cpp.md uses this path consistently (7
    occurrences). The source `packages/cpp/include/iscc/iscc.hpp` confirms the include path is
    correct for development. Release tarball layout is a pre-existing concern, not introduced here.

**IDLE**: All remaining issues are low priority — no actionable work for CID.

**Next:** The only `normal` issue (Swift GITHUB_REF_NAME bug) has `HUMAN REVIEW REQUESTED` — CID
cannot fix it without human approval to update the spec. The remaining `low` issue (docs logos) is
human-directed only. The project is effectively idle for CID until the human acts on the
GITHUB_REF_NAME issue or files new work.

**Notes:**

- Tab order follows target spec exactly: Python, Rust, Ruby, Node.js, WASM, Go, Java, C#, C++,
    Swift, Kotlin
- Available Bindings table now has 11 rows (Swift and Kotlin inserted between C/C++ and WebAssembly)
- Multi-language bullet updated to include Swift and Kotlin
- All new code examples verified against howto reference pages — API patterns are consistent
- WASM tab includes `await init()` per established convention
