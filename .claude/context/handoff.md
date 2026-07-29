## 2026-07-29 — Review of: Rename Go `EncodeIsccID` → `GenIsccIDV1`, delete `DecodeIsccID` / `IsccIDv1Result`

**Verdict:** PASS

**Summary:** The advance agent replaced the superseded `EncodeIsccID(realm, hubID, timestamp)` with
the canonical `GenIsccIDV1(timestamp, hubID, realm) (*IsccIdResult, error)`, deleted `DecodeIsccID`
and `IsccIDv1Result`, and moved decode to the generic `IsccDecode` + digest-unpack recipe. One
non-test/non-doc file touched (scope-clean), tests use the real golden vector, all gates green.

**Verification:**

- [x] `CGO_ENABLED=0 go test -count=1 ./...` in `packages/go/` — `ok`, all pass.
- [x] `go vet ./...` — clean (exit 0).
- [x] `! grep -rq 'DecodeIsccID\|IsccIDv1Result' packages/go` — no matches (also none in `docs/`).
- [x] Golden `TestGenIsccIDV1KnownVector`:
    `GenIsccIDV1(1751831876325218,1,0).ISCC ==   "ISCC:MAIGHFECJMOPMIAB"` — PASS (ran `-run`
    verbose).
- [x] `grep -q GenIsccIDV1 docs/howto/go.md && … README.md` — both present.
- [x] `mise run check` — all prek pre-commit hooks Passed; no context files reformatted.
- [x] **Probe:** `TestGenIsccIDV1KnownVectorExtraction` round-trips the golden vector through the
    documented `IsccDecode` + `binary.BigEndian.Uint64` recipe back to (ts, hub, realm) — PASS.

**Issues found:**

- (none material) — Minor: the `docs/howto/go.md` §Experimental snippet uses `binary.BigEndian`
    without showing the `encoding/binary` import in that block. Illustrative howto snippet against a
    stdlib package; not misleading, not filed.

**Codex review:** Clean — "preserves the existing encoding and validation behavior while matching
the specified signature and result shape; tests and documentation updated consistently, Go test and
vet pass." No actionable findings.

**Next:** #43 fan-out continues. (a) Mint `gen_iscc_id_v1` on napi/wasm/ffi/jni/rb/uniffi (each its
own step; FFI needs `iscc.h` regen + freshness gate); (b) decode round-trip — widen each surface's
version enum (Python `VS` IntEnum lists only `V0`) to accept `V1` + add an
`iscc_decode(gen_iscc_id_v1(...))` test; (c) the Tier-1 32→33 doc/count sweep (see #43 for the exact
stale sites incl. three non-Markdown sources). Go sub-item now fully closed.

**Notes:** `codec.go` decode logic untouched — `IsccDecode` already accepts `Id` Version 1. No
API-break: both removed symbols existed only on `develop`, never in a tagged release. Per the spec,
`iscc/iscc-monitor` should be told the final Go shape (update #43) before deleting its interim port
— a human/tracking action, out of code scope.
