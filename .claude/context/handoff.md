# Handoff

## 2026-07-24 — Review of: Go ISCC-IDv1 encode/decode (issue #43)

**Verdict:** PASS_WITH_NOTES

**Summary:** The advance agent added experimental ISCC-IDv1 support to the pure-Go binding
(`EncodeIsccID`/`DecodeIsccID` + `IsccIDv1Result`, a `VSV1` const, and a MainType-ID-only Version=1
relaxation in `decodeHeader`). The implementation is a faithful port of iscc-core's `gen_iscc_id_v1`
(verified against `reference/iscc-core/iscc_core/iscc_id.py`: `body = (timestamp << 12) | hub_id`,
header MT=ID/ST=realm/V=1/len=0, base32 + `ISCC:` prefix), the public `EncodeComponent` Version>0
contract is untouched, scope matches next.md exactly, and all tests + quality gates are green. Codex
surfaced one pre-existing (not regression) input-hardening gap, now filed as an issue.

**Verification:**

- [x] `go test ./...` passes (all existing + 8 new tests) — green from `packages/go/`
- [x] `CGO_ENABLED=0 go test ./...` passes (pure-Go invariant holds) — green
- [x] `go vet ./...` clean — exit 0, no output
- [x] `DecodeIsccID("ISCC:MAIGHFECJMOPMIAB")` and no-prefix variant → realm 0 / hub 1 / ts
    1751831876325218 — asserted by `TestDecodeIsccIDKnownVector` (both prefix forms)
- [x] `EncodeIsccID(0, 1, 1751831876325218)` → `"ISCC:MAIGHFECJMOPMIAB"` —
    `TestEncodeIsccIDKnownVector`
- [x] Boundary round-trips (hubID 0/4095, realm 0/1, ts 2^52−1) — `TestIsccIDRoundTripBoundaries`
    (18 combinations)
- [x] `IsccDecode` accepts ID/Version=1 → Maintype 6, Version 1, 8-byte digest —
    `TestIsccDecodeAcceptsIDv1`
- [x] Version>0 still rejected for non-ID MainType — `TestDecodeHeaderRejectsVersion1ForNonID` (+
    `TestDecodeHeaderRejectsVersion2ForID` pins the exact-match semantics)
- [x] `EncodeIsccID` returns `iscc:`-prefixed error for ts≥2^52, hubID≥4096, realm∉{0,1} —
    `TestEncodeIsccIDValidation`
- [x] Both functions carry an "experimental" doc-comment marker and appear in
    `packages/go/README.md` — verified (3 markers in `iscc_id.go`, 2 README rows)
- [x] `mise run check` — all 15 pre-commit hooks Passed

**Issues found:**

- Codex \[P2\]: `DecodeIsccID`/`IsccDecode` silently accept trailing bytes (e.g.
    `ISCC:MAIGHFECJMOPMIABAA` decodes identically to the canonical form). Verified **pre-existing
    and codec-wide** (a Data-Code with `+"AA"` is accepted the same way) — a hardening gap, not a
    regression from this work. Filed as a `normal` `[review]` issue with a scoped fix
    recommendation.
- Doc freshness (fixed directly): added the two new experimental functions to
    `packages/go/CLAUDE.md`'s "Codec Functions (exported)" list.

**Codex review:** One actionable P2 — trailing-byte acceptance in `DecodeIsccID`. Assessed as a real
robustness gap but pre-existing in `IsccDecode` (affects every MainType), so it does not block this
PR; filed for a follow-up iteration. No other findings.

**Next:** #43 is done and its spec checkboxes are ticked. Suggested next work: pick up the new
`normal` `[review]` issue — tighten Go `IsccDecode` to reject bodies whose length ≠ the
header-declared `nbytes` (re-run `ConformanceSelftest` to confirm no vendored vector relies on
padding, and check `IsccDecompose` is unaffected). Remaining v0.6.0 `normal` backlog otherwise: #49
aarch64 Python wheels, project-wide dependency review/refresh, npm OIDC migration, and the broken
single-registry re-trigger in `release.yml`.

**Notes:**

- Pre-existing gofmt drift under go1.26.1 on two untouched files (`codec_test.go`, `conformance.go`)
    — struct-tag/comment alignment rules changed in newer gofmt. CI pins go 1.23 and runs only
    `go test` + `go vet` (no gofmt gate), so CI is unaffected. The advance agent's three new files
    are gofmt-clean. Not worth a tracked issue (local-toolchain artifact only).
- The GitHub issue #43 stays open per its "close on release" note (v0.6.0 not yet cut); only the
    local `issues.md` tracking entry was deleted since the code work is complete.
- learnings.md trimmed to 200 lines (archived the completed blake3 WASM SIMD investigation to
    `learnings-archive.md`; added an ISCC-IDv1 algorithm entry).
