## 2026-11-22 — Go `EncodeComponent` rejects a digest shorter than `bit_length/8`

**Done:** Go's public `EncodeComponent` keeps its MainType/SubType/Version checks (same order), then
rejects `len(digest) < bitLength/8` with `iscc: digest length N < bit_length/8 (M)` (Rust wording),
then calls `encodeComponentInternal`. The clamping body (MTIscc rejection, length/header encoding,
clamped append) moved into `encodeComponentInternal`, mirroring Rust's `codec::encode_component`.
All 9 gen call sites now use the internal helper. Rule recorded in decisions.md.

**Files changed:**

- `packages/go/codec.go`: split `EncodeComponent` (validation + length check) and
    `encodeComponentInternal` (clamping encode); both doc comments updated
- `packages/go/code_content_{audio,image,mixed,text,video}.go`, `code_data.go`, `code_instance.go`,
    `code_meta.go` (2 sites): `EncodeComponent(uint8(MTx), uint8(STy), uint8(VSV0), …)` →
    `encodeComponentInternal(MTx, STy, VSV0, …)`, one call each, same arguments (perl regex swap)
- `packages/go/codec_test.go`: `TestCodecEncodeComponentRejectsShortDigest` (4 bytes/64 bits, full
    message asserted), `TestCodecEncodeComponentRejectsEmptyDigest` (0 bytes/32 bits,
    `digest length`), `TestCodecEncodeComponentExactLengthDigest` (8 bytes/64 bits, body
    round-trips)
- `.claude/context/decisions.md`: entry `encode_component rejects a digest shorter`

**Verification:**

- `go test -count=1 ./...` → `ok`; `go vet ./...` and `GOARCH=386 go vet ./...` clean;
    `CGO_ENABLED=0 GOARCH=386 go test -count=1 ./...` → `ok`; `gofmt -l .` empty
- `grep -n "EncodeComponent(" packages/go/*.go | grep -v _test.go` → only
    `codec.go:480:func EncodeComponent(…)` (the doc line has no `(`)
- `grep -n "encode_component rejects a digest shorter" .claude/context/decisions.md` → line 389
- `mise run check` → exit 0 (first run: mdformat rewrapped the decision entry; second run clean)
- Temporary probe (`zz_probe_test.go`, run then deleted, `ls zz*` → no matches): data
    `ISCC:GAEHA6W7RKLGNVVVKKJZC3YRAK357XX6X7HKIOLPOBY7BC5QBONX2BKZ`, text
    `ISCC:EAEHA2J6QMX4ZG2CUO7YPWBB4MECFBWWVJDPMQLTDS4CDX62B55UGXO4` —
    `GenDataCodeV0([]byte("abc"), 512)` and `GenTextCodeV0("hello", 512)` equal next.md's two codes
    exactly
- Decision claims re-measured: iscc-core 1.5.0 `encode_component(0,0,0,64,b"\x01\x02\x03\x04")` →
    `AAAQCAQDAQ`; Python binding same call →
    `ValueError: invalid input: digest length 4 < bit_length/8 (8)`

**Next:** the `encode_component` section in `data-edge.json` (12-surface fan-out) can now pass on
Go: short digest → error, exact digest → code.

**Notes:**

- **Finding for review to file:** gen `bits` above 256 (e.g. 512) yields a header claiming 512 bits
    over a 32-byte body on every surface and in iscc-core (probe above). Not pinned by any committed
    test, per next.md; behaviour not chosen
- Decompose call sites lose no validation: they pass types from `decodeHeader`, which already
    range-checks MainType/SubType/Version (codec.go:257-266)
- Go-only change: no Rust touched, CRAP/iai gates unaffected; no public API signature changed
