## 2026-02-27 — Update Go docs for 30/30 Tier 1 symbols

**Done:** Updated `docs/howto/go.md` with two new sections (Codec operations and Constants) covering
all 7 newly added Go symbols (EncodeBase64, JsonToDataUrl, EncodeComponent, IsccDecode,
IsccDecompose, SoftHashVideoV0, and 4 constants). Updated `packages/go/README.md` to replace stale
"planned" text with a complete API overview organized into 6 subsections (Text Processing, Algorithm
Primitives, Codec Operations, Streaming, Constants, Diagnostics) covering all 30/30 Tier 1 symbols.

**Files changed:**

- `docs/howto/go.md`: Added "Codec operations" section with encode/decode roundtrip and decompose
    examples, and "Constants" section listing all 4 package-level constants
- `packages/go/README.md`: Replaced stale Utilities subsection with 6 organized table-based
    subsections covering the full symbol set

**Verification:**

- `grep -c 'EncodeComponent' docs/howto/go.md` → 1 ✓
- `grep -c 'IsccDecode' docs/howto/go.md` → 2 ✓
- `grep -c 'IsccDecompose' docs/howto/go.md` → 1 ✓
- `grep -c 'JsonToDataUrl' docs/howto/go.md` → 1 ✓
- `grep -c 'EncodeBase64' docs/howto/go.md` → 1 ✓
- `grep -c 'SoftHashVideoV0' docs/howto/go.md` → 1 ✓
- `grep -c 'MetaTrimName' docs/howto/go.md` → 1 ✓
- `grep 'are planned' packages/go/README.md` → no matches ✓
- `grep -c 'EncodeComponent' packages/go/README.md` → 1 ✓
- `grep -c 'DataHasher' packages/go/README.md` → 2 ✓
- `uv run zensical build` → exits 0 ✓
- `mise run check` → all 14 hooks pass ✓

**Next:** All Go documentation gaps are closed (30/30 Tier 1 symbols documented). The project could
address remaining items in issues.md or other target milestones — e.g., dict auto-serialization for
`gen_meta_code_v0`, or adding codec/constants documentation sections to the Java and Python howto
guides for parity.

**Notes:** The next.md referenced `docs/howto/java.md` and `docs/howto/python.md` for
codec/constants section patterns, but neither has those sections yet. I followed the existing
pattern in go.md (Algorithm primitives section style) for the Codec operations section and used a
simple code-comment format for the Constants section. The README tables were reformatted by mdformat
during the format pass (proper column alignment).
