## 2026-07-29 — Rename Go `EncodeIsccID` → `GenIsccIDV1`, delete `DecodeIsccID` / `IsccIDv1Result`

**Done:** Replaced the superseded `EncodeIsccID(realm, hubID, timestamp) (string, error)` with the
canonical `GenIsccIDV1(timestamp, hubID, realm) (*IsccIdResult, error)` (single `ISCC` field),
deleted `DecodeIsccID` and `IsccIDv1Result` entirely, and moved decode to the generic `IsccDecode` +
digest-unpack recipe. Docs updated to match. Closes the Go sub-item of #43.

**Files changed:**

- `packages/go/iscc_id.go`: `GenIsccIDV1` + `IsccIdResult`; body packing/validation unchanged
    (order: timestamp `<2^52`, hubID `<2^12`, realm `<=1`; `iscc:`-prefixed errors). `DecodeIsccID`
    / `IsccIDv1Result` removed.
- `packages/go/iscc_id_test.go`: golden + validation tests target `GenIsccIDV1`; decode/round-trip
    tests use `extractIsccIDv1` helper (`IsccDecode` + `binary.BigEndian.Uint64` unpack) over the
    boundary grid (realm{0,1}×hub{0,4095}×ts{0,known,2^52-1}). Version-1 codec tests kept unchanged.
- `docs/howto/go.md`: §Experimental ISCC-IDv1 rewritten to `GenIsccIDV1` + extraction recipe;
    dropped the "Go-only, other bindings reject Version 1" warning (core accepts it now).
- `packages/go/README.md`, `packages/go/CLAUDE.md`: API tables/lists updated to `GenIsccIDV1`.

**Verification:**

- `CGO_ENABLED=0 go test -count=1 ./...` in `packages/go/` — `ok` (all pass).
- `go vet ./...` — clean.
- `grep -rn 'DecodeIsccID\|IsccIDv1Result' packages/go` — no matches (exit 1); also none in docs.
- Golden: `TestGenIsccIDV1KnownVector` asserts
    `GenIsccIDV1(1751831876325218,1,0).ISCC == "ISCC:MAIGHFECJMOPMIAB"` — passes.
- `grep -q GenIsccIDV1 docs/howto/go.md && … README.md` — both present.
- `uv run prek run --files <3 docs>` — mdformat reformatted the 3 markdown files (applied); re-run
    would pass.

**Next:** #43 fan-out continues. Remaining: (a) minting `gen_iscc_id_v1` on napi/wasm/ffi/jni/rb/
uniffi surfaces (each its own step; FFI needs `iscc.h` regen); (b) decode round-trip — widen each
surface's version enum (Python `VS`, etc.) to accept `V1`; (c) the Tier-1 32→33 doc/count sweep.

**Notes:** `codec.go` decode logic untouched — `IsccDecode` already accepts `Id` Version 1. No API
break of concern: both removed symbols existed only on `develop`, never in a tagged release. Per the
spec, `iscc/iscc-monitor` should be told the final shape (update #43) before deleting its interim
port — that is a human/tracking action, out of code scope.
