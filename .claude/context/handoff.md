## 2026-02-27 — Review of: Update crates/iscc-lib/CLAUDE.md for 30-symbol Tier 1 API

**Verdict:** PASS

**Summary:** Clean, accurate documentation-only update. All 5 sections from next.md were updated
correctly: binding crate list (+iscc-jni), symbol count (22→30), Tier 1 list (+8 symbols), Tier 2
list (−encode_component), and dependencies (+serde_json_canonicalizer). The advance agent also fixed
a bonus stale reference in the module layout table (encode_component in codec.rs description).

**Verification:**

- [x] `grep '30 symbols' crates/iscc-lib/CLAUDE.md` — match on Tier 1 heading
- [x] `grep 'json_to_data_url' crates/iscc-lib/CLAUDE.md` — match in Tier 1 encoding utilities
- [x] `grep 'iscc_decode' crates/iscc-lib/CLAUDE.md` — match in Tier 1 codec operations
- [x] `grep -c 'encode_component' crates/iscc-lib/CLAUDE.md` → exactly 2 (Tier 1 + Common Pitfalls)
- [x] `grep 'META_TRIM_NAME' crates/iscc-lib/CLAUDE.md` — match in Tier 1 algorithm constants
- [x] `grep 'serde_json_canonicalizer' crates/iscc-lib/CLAUDE.md` — match in Dependencies section
- [x] `grep 'iscc-jni' crates/iscc-lib/CLAUDE.md` — match in binding crate list
- [x] No Rust compilation or test changes — docs-only commit
- [x] `mise run check` — all 14 hooks pass

**Issues found:**

- (none)

**Codex review:** Codex flagged [P2] that `codec::encode_component` is still `pub fn` in codec.rs
but was removed from the Tier 2 list. This is by design: there are two `encode_component` functions
— the Tier 1 wrapper in `lib.rs` (takes `u8` integers for FFI) and the Tier 2 original in codec.rs
(takes Rust enums). The Tier 2 list correctly omits it because Tier 2 means "Rust-only, not exposed
through bindings" and `encode_component` IS now bound in all languages. Same pattern as
`iscc_decompose` (also in codec.rs, also re-exported at crate root, only listed in Tier 1).

**Next:** All internal documentation is now accurate. The project can focus on remaining issues in
issues.md: dict auto-serialization for `gen_meta_code_v0` (#5), `encode_component` API promotion
(#6), `iscc_decode` implementation (#7), or constants exposure (#8). Note that #6-#8 describe
features already implemented in Rust and bindings — these issues may be closeable after verification
that all binding-level work is complete.

**Notes:** The module layout table still has a minor pre-existing quirk: `iscc_decompose` is listed
in the codec.rs row description but it's a Tier 1 re-export (same pattern as the now-fixed
`encode_component`). Not worth a dedicated work package — can be cleaned up opportunistically.
