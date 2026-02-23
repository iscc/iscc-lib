## 2026-02-23 — Review of: Promote soft_hash_video_v0 to Tier 1 public API

**Verdict:** PASS

**Summary:** Clean, minimal promotion of `soft_hash_video_v0` from private `fn` to `pub fn`. Exactly
one line changed in `lib.rs`. 5 well-structured integration tests added covering all cases specified
in next.md (basic 64-bit, 256-bit, deduplication, empty input error, consistency with
`gen_video_code_v0`). The consistency test is particularly good — it extracts the body from the gen
function's encoded output and verifies it matches the primitive's raw digest. All 193 tests pass
(143 unit + 28 algorithm primitives + 22 text utils), clippy clean workspace-wide, all pre-commit
hooks pass.

**Issues found:**

- (none)

**Next:** Promoted symbol count is now 18 of 22 Tier 1 targets. The remaining 4 symbols all require
new implementation (not visibility changes): `encode_base64`, `iscc_decompose`,
`DataHasher`/`InstanceHasher` streaming types, and `conformance_selftest`. `encode_base64` is likely
the simplest — it wraps the existing `encode_base32` pattern but for base64 encoding.
`iscc_decompose` is a codec utility that splits an ISCC code into its constituent unit codes. Either
of these codec utilities would be a good next step before tackling the more complex streaming types.

**Notes:** The promotion pattern is now fully proven with 6 successful applications (4 text utils, 4
algo primitives, 1 video hash). All remaining Tier 1 work involves net-new implementation.
`state.md` should be updated to reflect 18/22 promoted symbols.
