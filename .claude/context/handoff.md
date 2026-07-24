# Handoff

## 2026-07-24 — Review of: Reject trailing bytes in Rust core `iscc_decode`

**Verdict:** PASS

**Summary:** The advance agent added a second `tail.len() > nbytes` "too long" rejection branch to
`iscc_decode` in `crates/iscc-lib/src/lib.rs`, mirroring the two-branch Go pattern from iter 120 and
closing the codec-wide trailing-byte alias gap for the stability-committed core and all 11
delegating bindings. Scope is exactly next.md's — 6 added guard/error lines, a docstring update, and
one focused test; the "too short" branch and `iscc_decompose` are untouched.

**Verification:**

- [x] `cargo test -p iscc-lib` passes — 321 total (270 unit incl. new
    `test_iscc_decode_rejects_trailing_bytes`, 28+22 integration, 1 doctest), 0 failed
- [x] `cargo test -p iscc-lib --all-features` passes — same counts, feature-gated paths green
- [x] `cargo clippy -p iscc-lib --all-features --all-targets -- -D warnings` clean (only
    pre-existing transitive `proc-macro-error2` future-incompat note, unrelated to diff)
- [x] `cargo fmt -p iscc-lib --check` passes — exit 0
- [x] New test asserts both: padded canonical (`ISCC:AAAZXZ6OU74YAZIM` + `"AA"`) →
    `Err(InvalidInput)` containing `"too long"`, AND canonical form still decodes to its 8-byte
    digest — PASS
- [x] `test_iscc_decode_truncated_input` (asserts `"too short"` substring, line 1554) + all
    `test_iscc_decode_round_trip_*` stay green — "too short" branch intact
- [x] Conformance holds — `test_conformance_selftest_passes`, all `test_gen_*_v0_conformance`,
    `test_decompose_conformance_roundtrip` PASS; `iscc_decompose` untouched per Not-In-Scope
- [x] `mise run check` — all 15 pre-commit hooks Passed

**Issues found:**

- (none) — implementation is correct, minimal, and conformance-safe. Resolved and deleted the
    `normal` `[review]` issue "Rust core `iscc_decode` silently accepts trailing bytes" after
    verifying the fix.

**Codex review:** Clean. Confirmed "The new guard correctly rejects decoded bodies with trailing
bytes while preserving canonical inputs and existing short-body validation. The full Rust test
suite, clippy, and formatting checks pass." No actionable findings.

**Next:** The Rust-core trailing-byte hardening is complete — the decode alias gap is now closed in
both Go and the Rust core (all 11 bindings inherit it). Remaining CID-doable v0.6.0 targets: #49
aarch64 Python wheels (native `ubuntu-24.04-arm` runner, plan at
`.claude/plans/restore-linux-aarch64-python-wheels.md`) and the project-wide dependency
review/refresh (both `normal` `[human]`, both actionable). Release-reliability issues (npm OIDC
migration, single-registry re-trigger) stay human-gated. Do NOT cut v1.0.0 or flip the Semver gate —
both held by Titusz.

**Notes:**

- Signature-neutral robustness fix on a Tier 1 stability-committed symbol — signature/return shape
    unchanged, only previously-accepted malformed input is now rejected. Rationale recorded in
    `decisions.md` (2026-07-24, "Tightening decode input-validation is not a SemVer break") so the
    eventual `cargo-semver-checks` enforcing gate has the human judgment on record: input-domain
    narrowing is invisible to signature-based semver tooling.
- The Goal's headline example `ISCC:MAIGHFECJMOPMIABAA` is an ISCC-IDv1 (header version 1) that the
    Rust core rejects at the header level (`codec::Version` has V0 only) before the length check —
    it was never decodable by the Rust core. The alias gap in Rust applied to all V0 codes; the test
    correctly uses a canonical V0 Meta-Code + `"AA"` suffix. ISCC-IDv1 support remains Go-only.
- No benchmarked hot path touched — no bench run needed.
