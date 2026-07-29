## 2026-07-29 — Review of: Mint `gen_iscc_id_v1` on the Node.js (napi) binding

**Verdict:** PASS_WITH_NOTES

**Summary:** The advance agent added the experimental `gen_iscc_id_v1` minting export to
`crates/iscc-napi` (bare-string convention, `f64` timestamp), taking the napi surface from 32 to 33
Tier 1 symbols. Diff is scope-clean (one non-test file), all specified gates green, golden vector
confirmed against the `iscc-core` oracle. Codex flagged a real out-of-scope input-hardening gap on
the `f64`/JS-number path, filed as a `[review]` issue — hence PASS_WITH_NOTES, not PASS.

**Verification:**

- [x] `npm run build:debug` — compiled clean.
- [x] `npm test` — **158 pass / 0 fail** (existing suites + new `iscc_id_v1.test.mjs`).
- [x] Golden `gen_iscc_id_v1(1751831876325218, 1, 0) === "ISCC:MAIGHFECJMOPMIAB"` — holds;
    **probe:** independently confirmed against `iscc_core` 1.3.0 (`--python 3.13`), byte-identical.
- [x] `index.d.ts:149` declares `export declare function gen_iscc_id_v1(timestamp, hubId, realm)`.
- [x] Round-trip `iscc_decode(...)` → maintype 6 / version 1 / subtype 0 — asserted, passes (napi
    `iscc_decode` returns `version: u8`, so no version-enum widening needed).
- [x] `cargo clippy -p iscc-napi --all-targets -- -D warnings` — clean.
- [x] `cargo fmt -p iscc-napi --check` — clean.
- [x] `mise run check` — all prek pre-commit hooks Passed.

**Issues found:**

- **[filed, normal]** `gen_iscc_id_v1` on the napi surface silently coerces invalid JS numbers into
    valid-but-wrong IDs instead of throwing. Verified at HEAD: `timestamp` of `-1`/`NaN`/`0.9` →
    `f64 as u64` truncates to `0` → `ISCC:MAIAAAAAAAAAAAAB`; `hub_id` `0.9`/`2**32` → `0`; `realm`
    `0.9` → `0`. next.md's three specified validation cases (ts=2^52, hub=4096, realm=2) all throw
    correctly — this is the unguarded low/non-integral end, out of next.md scope. Applies to the
    whole JS-number fan-out (wasm next). Tracked in issues.md; not a blocker for an experimental fn.

**Codex review:** One P2 — "Reject invalid numbers before narrowing." Judged **valid and relevant**;
reproduced it live and filed the issue above rather than fix inline (adding throwing paths +
deciding how to validate `hub_id`/`realm`, which napi coerces before our wrapper runs, is a design
choice, not a minor fix). No other findings.

**Next:** #43 fan-out continues — mint `gen_iscc_id_v1` on the remaining surfaces: wasm, ffi (needs
`iscc.h` regen + freshness gate), jni, rb, uniffi (one per step). **When the wasm slice lands,
settle the JS-number input-validation approach from the new issue there** so it isn't re-decided per
surface. Then the Tier-1 32→33 doc/count sweep across the stale sites named in #43.

**Notes:** napi CLAUDE.md/README carry no symbol count (per Not-In-Scope), so no doc edit here.
`index.js`/`index.d.ts`/`*.node` are gitignored build artifacts, not committed. The `f64`-timestamp
choice is correct for valid inputs (valid ts `< 2^52` is exact in f64, avoids forcing a JS
`BigInt`); the gap is only that out-of-contract inputs aren't rejected. learnings.md trimmed 204→197
lines.
