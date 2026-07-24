# Handoff

## 2026-07-24 — Reject trailing bytes in Rust core `iscc_decode`

**Done:** Added a `tail.len() > nbytes` rejection branch to `iscc_decode` in the Rust core,
mirroring the two-branch Go pattern from iter 120. Inputs whose decoded body exceeds the expected
digest length now return `Err(IsccError::InvalidInput("decoded body too long: ..."))` instead of
being silently truncated. Docstring updated to state the exact-length requirement, and one focused
test added. All 11 delegating bindings inherit the fix automatically.

**Files changed:**

- `crates/iscc-lib/src/lib.rs`: second guard branch after the untouched "too short" branch (distinct
    branches, "too short" message intact); docstring summary + `# Errors` now document that both
    truncated and trailing-byte inputs are rejected; new test
    `test_iscc_decode_rejects_trailing_bytes` in the inline test module.

**Verification:**

- `cargo test -p iscc-lib` — 321 passed (270 unit + 28 + 22 integration + 1 doctest), 0 failed
- `cargo test -p iscc-lib --all-features` — same counts, all green (feature-gated paths covered)
- `cargo clippy -p iscc-lib --all-features -- -D warnings` — clean
- `cargo fmt -p iscc-lib --check` — clean
- New test asserts: `"ISCC:AAAZXZ6OU74YAZIM" + "AA"` base32-decodes to more bytes than canonical
    (sanity-checked in the test via `codec::decode_base32` before asserting), `iscc_decode` on it
    returns `Err(InvalidInput)` with message containing `"too long"`, and the canonical form still
    decodes to its 8-byte digest — all pass
- `test_iscc_decode_truncated_input`, all `test_iscc_decode_round_trip_*`, and all conformance tests
    (`test_*_conformance_vectors`, `conformance_selftest` via `test_conformance_selftest`) stay
    green — `iscc_decompose` untouched per Not-In-Scope
- `mise run check` — all 15 pre-commit hooks Passed

**Next:** The `normal` `[review]` issue "Rust core `iscc_decode` silently accepts trailing bytes" is
now resolved — review agent should close it. Remaining CID-doable v0.6.0 targets: #49 aarch64 Python
wheels and the project-wide dependency review/refresh. Release-reliability issues (npm OIDC
migration, single-registry re-trigger) stay human-gated. Do NOT cut v1.0.0 or flip the Semver gate —
both held by Titusz.

**Notes:**

- Signature-neutral robustness fix: `iscc_decode` is Tier 1 but its signature/return shape is
    unchanged — only previously-accepted malformed input is now rejected. No API break.
- The Goal's headline example `ISCC:MAIGHFECJMOPMIABAA` is an ISCC-IDv1 (header version 1). The Rust
    core's `codec::Version` enum only has `V0`, so that exact string is rejected at the header level
    (`invalid Version: 1`) before the length check — it was never decodable by the Rust core. The
    alias gap in Rust applied to all V0 codes; the test therefore uses next.md's suggested canonical
    V0 Meta-Code `ISCC:AAAZXZ6OU74YAZIM` + `"AA"` suffix (16→18 base32 chars = 10→11 decoded bytes,
    one trailing byte). ISCC-IDv1 support exists only in the experimental Go `iscc_id.go`.
- After both guards `tail.len() == nbytes`, so the existing `tail[..nbytes].to_vec()` is now a full
    copy — left as-is (no behavior change, minimal diff).
- Not a benchmarked hot path (`iscc_decode` is not in `gen_*_v0`/hashing/CDC/MinHash benches) — no
    bench run needed.
- `.claude/context/iterations.jsonl` is runner-owned; left unstaged.
