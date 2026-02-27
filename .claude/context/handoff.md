## 2026-02-27 — Update crates/iscc-lib/CLAUDE.md for 30-symbol Tier 1 API

**Done:** Updated the per-crate CLAUDE.md to reflect the current 30-symbol Tier 1 API. All 5
sections identified in next.md were updated: binding crate list (added iscc-jni), symbol count
(22→30), Tier 1 list (added json_to_data_url, encode_component, iscc_decode, 4 constants), Tier 2
list (removed encode_component), and dependencies (added serde_json_canonicalizer). Also fixed the
module layout table which still listed encode_component under codec.rs Tier 2 description.

**Files changed:**

- `crates/iscc-lib/CLAUDE.md`: Updated binding crate list, symbol count, Tier 1/Tier 2 API lists,
    dependencies section, and module layout table

**Verification:** All 7 grep verification checks pass:

- `grep '30 symbols'` → match (Tier 1 heading)
- `grep 'json_to_data_url'` → match (in Tier 1 encoding utilities)
- `grep 'iscc_decode'` → match (in Tier 1 codec operations)
- `grep -c 'encode_component'` → exactly 2 (Tier 1 + Common Pitfalls, NOT in Tier 2)
- `grep 'META_TRIM_NAME'` → match (in Tier 1 algorithm constants)
- `grep 'serde_json_canonicalizer'` → match (in Dependencies section)
- `grep 'iscc-jni'` → match (in binding crate list)
- `mise run check` → all 14 hooks pass

**Next:** All documentation is now accurate. The project can focus on remaining issues: dict
auto-serialization for gen_meta_code_v0 (#5), encode_component API promotion (#6), iscc_decode
implementation (#7), or constants exposure (#8) — though #6-#8 are already implemented in Rust, the
issues may track binding-level work.

**Notes:** The module layout table (line 19) also referenced encode_component in the codec.rs
description — this was an additional stale reference not called out in next.md's 5 update points.
Fixed it to keep the table consistent with the Tier 1/2 lists. mdformat auto-fixed table column
widths on first run.
