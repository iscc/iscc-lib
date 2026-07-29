# Next Work Package

## Step: Tier-1 32→33 doc/count sweep + `gen_iscc_id_v1` API-doc entries

## Goal

Close the last CID-doable v0.6.0 release blocker (issues.md "ISCC-IDv1 Tier-1 32→33 doc/count
sweep", GitHub #43): bring every shipped-artifact Tier-1 symbol count to 33 and add the
`gen_iscc_id_v1` per-symbol entry + IDv1 example to the hand-maintained API/howto docs. Purely
mechanical and non-functional — no source behaviour changes.

## Alternatives Considered

- **Chosen:** the doc/count sweep — the only remaining #43 item and an explicit v0.6.0 release
    blocker; the other four open normals are `[review]`/upstream-blocked, not release-gating.
- **Rejected:** the codec `iscc_clean` divergence normal — a real drop-in gap, but it needs a
    careful two-surface (Rust + hand-ported Go) code change with a multibase-prefix edge case, is
    not release-gating, and would not clear the last v0.6.0 blocker.

## Scope

**Fan-out:** the identical NN→33 count edit + `gen_iscc_id_v1` doc entry, applied across the doc
surfaces below. All targets are docs/notes/agent-prose (outside the 3-file code budget).

- **Modify (count → 33):** `crates/iscc-lib/CLAUDE.md:31` (32), `crates/iscc-wasm/CLAUDE.md:143`
    (30), `docs/ruby-api.md:9` (32), `docs/java-api.md:8` (30), `notes/00-overview.md:153` (22, plus
    the surrounding item-8 prose), `notes/04-api-compatibility-safety.md:21` (32),
    `.claude/agents/advance.md:146` (32). Also re-scan per-crate/package `README.md`/`CLAUDE.md` for
    any Tier-1 count phrasing missed here.
- **Modify (add `gen_iscc_id_v1` entry + field-extraction recipe):** `docs/rust-api.md`,
    `docs/java-api.md`, `docs/ruby-api.md`, `docs/c-ffi-api.md` (symbol name `iscc_gen_iscc_id_v1`).
- **Modify (add an IDv1 mint example, matching the existing go/ruby howto sections):**
    `docs/howto/rust.md`, `python.md`, `nodejs.md`, `wasm.md`, `java.md`, `kotlin.md`, `swift.md`,
    `dotnet.md`, `c-cpp.md` (9 pages; `go.md` and `ruby.md` already have it).
- **Reference:** `docs/howto/go.md:392-415` and `docs/howto/ruby.md` IDv1 sections (template);
    `.claude/agent-memory/define-next/iscc-idv1-fanout.md` (per-surface symbol names + decode
    recipe); state.md per-surface signatures.

## Not In Scope

- Any change to Rust/binding source, signatures, or test code — this is docs-only.
- Historical/record files: `iterations.jsonl`, `state.md`, agent memory, `*-archive.md`, and any
    `specs/*.md`/`target.md` (already at 33). Do not rewrite them.
- Adding a NEW docs page (would drag in `scripts/check_docs_nav.py` four-list parity) — edit
    existing pages only.
- The other open normals (Ruby wide-input order, codec `iscc_clean`, iai ASCII benchmarks).

## Implementation Notes

- Per-language mint symbol: `gen_iscc_id_v1` (rust/python/nodejs/wasm/c-cpp), `genIsccIdV1`
    (java/kotlin/swift), `GenIsccIdV1` (dotnet), `iscc_gen_iscc_id_v1` (c-ffi-api). Signature order
    is always `(timestamp, hub_id, realm)`; result carries a single ISCC-string field
    (`IsccIdResult`).
- Field-extraction / decode recipe (state it once per API page): `iscc_decode(iscc)` returns
    `version == 1` (a bare int/byte on every surface except Python's `VS.V1`), then bit-math on the
    8-byte BE body — `timestamp = n >> 12`, `hub_id = n & 0xFFF`, `realm = subtype`. There is NO
    dedicated IDv1 decoder; the generic `iscc_decode` covers it.
- Keep each howto example short and mark it "Experimental — not part of ISO 24138, may change in a
    minor release", mirroring `go.md`.
- Two source files the issue lists as stale already read 33 (`crates/iscc-uniffi/src/lib.rs:3`,
    `crates/iscc-rb/src/lib.rs:7`) — leave them; the verification grep confirms.
- Run `mise run format` before committing so mdformat's `--wrap 100` reflow doesn't reject the push.

## Verification

- Stale-count grep is empty:
    `grep -rniE "all (2[0-9]|3[0-2]) tier 1|\((2[0-9]|3[0-2]) symbols|(2[0-9]|3[0-2]) (public symbol|symbols at crate)|tier 1 \((2[0-9]|3[0-2]) symbol" docs/ notes/ crates/*/CLAUDE.md crates/*/README.md packages/ .claude/agents/advance.md`
    returns nothing.
- Each API page carries the new entry:
    `grep -l gen_iscc_id_v1 docs/rust-api.md docs/ruby-api.md && grep -l genIsccIdV1 docs/java-api.md && grep -l iscc_gen_iscc_id_v1 docs/c-ffi-api.md`
    lists all four.
- All 11 howto pages reference IDv1 minting: `grep -riLE "gen_?iscc_?id_?v1" docs/howto/*.md` prints
    no filenames.
- `uv run zensical build` exits 0 and reports "No issues found".
- `mise run check` passes (prek: mdformat/hygiene clean on all edited files).

## Done When

Every shipped-artifact Tier-1 count reads 33, the four API-doc pages plus all 11 howto pages carry a
`gen_iscc_id_v1` entry/example, and the verification commands above all pass.
