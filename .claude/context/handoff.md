## 2026-02-28 — Add Java API reference page

**Done:** Created `docs/java-api.md` documenting the full Java API surface — all 30 Tier 1 symbols
as static methods on `IsccLib`, the `IsccDecodeResult` class, 4 constants, streaming hasher
lifecycle, error handling, and memory management. Wired it into site navigation and llms.txt.

**Files changed:**

- `docs/java-api.md`: New Java API reference page (front matter, installation, quick example,
    constants table, IsccDecodeResult class, all 9 gen\*V0 methods with signatures/parameter
    tables/examples, text utilities, encoding, codec, sliding window, algorithm primitives,
    streaming hashers with DataHasher/InstanceHasher lifecycle, error handling, memory management)
- `zensical.toml`: Added `{ "Java API" = "java-api.md" }` after C FFI in Reference nav section
- `docs/llms.txt`: Added Java API reference line after C FFI line

**Verification:**

- [x] `uv run zensical build` succeeds (3.67s, `/java-api/` in output)
- [x] `grep -q 'java-api.md' zensical.toml` exits 0 — nav entry present
- [x] `grep -q 'java-api' docs/llms.txt` exits 0 — llms.txt reference added
- [x] `grep -c 'genMetaCodeV0\|genTextCodeV0\|...\|genIsccCodeV0' docs/java-api.md` returns 32 (>=
    9\) — all gen functions documented
- [x] `grep -c 'dataHasherNew\|dataHasherUpdate\|dataHasherFinalize\|dataHasherFree'   docs/java-api.md`
    returns 18 (>= 4) — DataHasher lifecycle documented
- [x] `grep -q 'IsccDecodeResult' docs/java-api.md` exits 0 — decode result type documented
- [x] `grep -q 'META_TRIM_NAME' docs/java-api.md` exits 0 — constants documented
- [x] `mise run check` — all 14 hooks pass

**Next:** All spec-required documentation pages are complete (Rust API, Python API, C FFI, Java
API). The documentation spec is fully met. Remaining work is human-dependent: merge PR from develop
→ main, trigger releases, configure Maven Central publishing.

**Notes:** All content transcribed directly from Javadoc in `IsccLib.java` and
`IsccDecodeResult.java` — no undocumented behavior added. Page structure follows the C FFI reference
pattern adapted for Java (no memory management functions since JVM handles most allocation; only
streaming hasher handles need manual cleanup).
