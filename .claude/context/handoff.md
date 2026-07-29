## 2026-07-29 — Review of: Tier-1 32→33 doc/count sweep + `gen_iscc_id_v1` API-doc entries

**Verdict:** PASS_WITH_NOTES

**Summary:** The docs-only sweep bumped every shipped-artifact Tier-1 count to 33, added a
`gen_iscc_id_v1` per-symbol entry + decode recipe to the four hand-maintained API pages, and added
an "ISCC-IDv1 (experimental)" mint+decode example to the 9 howto pages that lacked one. Golden value
and decode recipe verified against `iscc-core` 1.3.0; all next.md verification passes. Two Codex P2
findings on factual doc claims — one (a fabricated wasm `IsccIdResult` struct) I fixed directly; the
other (pre-existing c-ffi type-name/header mismatch) I filed as an issue.

**Verification:**

- [x] Stale-count grep (next.md cmd) → empty (exit 1); broader `\b32\b`+word-form sweep also clean.
- [x] API pages carry the entry — `gen_iscc_id_v1` (rust/ruby), `genIsccIdV1` (java),
    `iscc_gen_iscc_id_v1` (c-ffi) all listed.
- [x] All 11 howto pages reference IDv1 minting (`grep -riLE` prints no filenames).
- [x] `uv run zensical build` → "No issues found", exit 0.
- [x] `mise run check` → all prek hooks Passed (mdformat, docs page-list parity, CI job parity).
- [x] **Extra probe** — golden `gen_iscc_id_v1(1751831876325218,1,0) == "ISCC:MAIGHFECJMOPMIAB"`
    confirmed via `iscc-core`; decode recipe (`version==1`, `ts=n>>12`, `hub=n&0xFFF`,
    `realm=subtype`; realm 1 → `MEIGH…`) verified; all 15 doc golden strings consistent.
- [x] **Extra probe** — Rust `iscc_decode` returns `(u8,u8,u8,u8,Vec<u8>)`, matching the documented
    5-tuple destructure; testnet/mainnet realm labels match `iscc_id.py:46`.

**Issues found:**

- **Fixed here:** `crates/iscc-wasm/CLAUDE.md` claimed `gen_iscc_id_v1` returns an `IsccIdResult`
    struct and listed it as a 3rd result struct — the wasm surface returns `Result<String, JsError>`
    (bare string) and no such struct exists. Corrected to "returns the ISCC string directly" and
    restored the accurate "2 result structs / 3 streaming types" framing. (Codex P2 #2.)
- **Filed (issues.md, `normal` [review]):** `docs/c-ffi-api.md` documents all FFI structs under
    unprefixed names (`IsccDecodeResult`), but cbindgen emits `iscc_IsccDecodeResult` in `iscc.h`,
    so no snippet — including the new IDv1 example — compiles verbatim. Pre-existing and page-wide,
    not a regression from this sweep (the example follows the page's own convention). (Codex P2 #1.)

**Codex review:** Both findings above are genuine and were verified against source. #2 was a
this-iteration fabrication (fixed); #1 is a pre-existing whole-page gap (tracked). Codex found no
issue with the count bumps or the API/howto entries.

**Next:** #43 (ISCC-IDv1) is now **fully closed** — mint + decode on all 11 surfaces and the doc
sweep are done. No v0.6.0 release blockers remain among CID-doable work. Remaining open `normal`
issues are all non-release-gating: (1) Ruby `gen_iscc_id_v1` validation order for `> i64::MAX`
inputs; (2) codec `iscc_clean` divergence (two-surface Rust+Go, the most substantive follow-up); (3)
iai ASCII-only text benchmarks; (4) the new c-ffi-api type-name gap. The go1.27 boundary-table bump
stays trigger-gated (~Aug 2026). Suggest define-next pick the codec `iscc_clean` gap or the Ruby
validation-order fix.

**Notes:**

- The wasm **howto** example was already correct (treats the return as a string) — only the
    per-crate CLAUDE.md prose was wrong; the two are now consistent.
- Scope was clean: docs/notes/agent-prose only, no source/test/signature changes. next.md's
    `**Fan-out:**` valve covers the >3-file docs edit; each surface got a mechanically identical
    count bump + template example (wasm CLAUDE.md's struct-count restructure was the only
    non-mechanical edit and was honestly flagged in the advance handoff).
- learnings.md compressed 202→197 lines (under budget); resolved #43 sweep issue deleted.
- CI reminder unchanged: `zensical build` wipes `site/`, so `gen_llms_full.py` must run after it in
    docs CI.
