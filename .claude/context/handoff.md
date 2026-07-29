## 2026-07-29 — Mint `gen_iscc_id_v1` on the Node.js (napi) binding

**Done:** Added the experimental `gen_iscc_id_v1` minting export to `crates/iscc-napi` (bare-string
convention, `f64` timestamp), bringing the napi surface from 32 to 33 Tier 1 symbols. No
version-enum widening needed — napi's `iscc_decode` returns `version` as a bare `u8`, so V1 decode
round-trips today.

**Files changed:**

- `crates/iscc-napi/src/lib.rs`: added `#[napi(js_name = "gen_iscc_id_v1")]` wrapper
    `(timestamp: f64, hub_id: u16, realm: u8) -> napi::Result<String>` delegating to
    `iscc_lib::gen_iscc_id_v1(... as u64, ...).map(|r| r.iscc)`.
- `crates/iscc-napi/__tests__/iscc_id_v1.test.mjs` (new): golden vector, realm-1 accept, three
    validation-throws (ts=2^52, hub=4096, realm=2), and `iscc_decode` round-trip.

**Verification:**

- `npm run build:debug` — compiled clean.
- `npm test` — **158 pass / 0 fail** (existing suites + new `iscc_id_v1.test.mjs`).
- Golden `gen_iscc_id_v1(1751831876325218, 1, 0) === "ISCC:MAIGHFECJMOPMIAB"` — holds.
- `index.d.ts:149` declares
    `export declare function gen_iscc_id_v1(timestamp: number, hubId: number, realm: number): string`.
- Round-trip: `iscc_decode(...)` → `maintype 6`, `version 1`, `subtype 0` — asserted, passes.
- `cargo clippy -p iscc-napi --all-targets -- -D warnings` — clean.
- `cargo fmt -p iscc-napi --check` — clean.
- `mise run check` (prek pre-commit, all files) — all hooks Passed.

**Next:** #43 fan-out continues — mint `gen_iscc_id_v1` on the remaining surfaces: wasm, ffi (needs
`iscc.h` regen + freshness gate), jni, rb, uniffi (one per step). Then the Tier-1 32→33 doc/count
sweep across the stale sites named in #43.

**Notes:** napi CLAUDE.md/README carry no symbol count, so no doc edit needed here (per
Not-In-Scope). `index.d.ts` shows the JS-idiomatic camelCase param `hubId` in the type declaration
while the JS runtime call uses positional args, so this is cosmetic (napi-derive camelCases `.d.ts`
param identifiers only; the runtime export name stays `gen_iscc_id_v1` via js_name).
`index.js`/`index.d.ts`/`*.node` are gitignored build artifacts, not committed.
