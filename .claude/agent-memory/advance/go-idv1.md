---
name: go-idv1
description: Go-only experimental ISCC-IDv1 support and exact body-length decode guards
metadata:
  type: project
---

# Go ISCC-IDv1 + decode guards

- Experimental ISCC-IDv1 in `packages/go/iscc_id.go` (`EncodeIsccID`/`DecodeIsccID`, iter 119, #43);
    `codec.go` `decodeHeader` accepts Version=1 ONLY for MainType ID (`VSV1` const).
- Go `IsccDecode` (iter 120) and Rust Tier 1 `iscc_decode` (iter 121) both enforce EXACT body length
    — two-branch "too short"/"too long" guards.
- Rust `codec::Version` has V0 only, so IDv1 codes (`MAIG...`) are Go-only.
- Go CI job runs only `go test`+`go vet` — no gofmt gate. The go1.26 gofmt alignment drift in
    codec_test.go + conformance.go was fixed (gofmt -w) in iter 147; tree is gofmt-clean now.
- `TextCollapse` (utils.go) uses per-call `cases.Lower(language.Und)` for Final_Sigma conformance
    (iter 147). NEVER hoist the Caser to a package-level var — Casers are stateful/not
    goroutine-safe, and hoisting measured no faster (~140ns construction).
