## 2026-11-22 — Review of: Go `EncodeComponent` rejects a digest shorter than `bit_length/8`

**Verdict:** PASS

**Summary:** Go's public `EncodeComponent` now returns `iscc: digest length N < bit_length/8 (M)`
for a short digest, after its unchanged MainType/SubType/Version checks. The clamping body moved
into `encodeComponentInternal`, which the 9 gen call sites and the decompose paths use, as Rust does
with `codec::encode_component`. Only `codec.go` changes behaviour; the 8 call-site swaps are neutral
(same arguments, constants that pass every removed range check). The rule is in decisions.md.

**Verification:**

- [x] `cd packages/go && go test ./...` passes, including the new short-digest tests — `ok`; the
    three new tests pass by name under `-v`
- [x] `go vet ./...` clean and `CGO_ENABLED=0 GOARCH=386 go test ./...` passes — both exit 0;
    `GOARCH=386 go vet` and `gofmt -l` clean too
- [x] `grep -n "EncodeComponent(" packages/go/*.go | grep -v _test.go` prints only the definition —
    `codec.go:480` alone
- [x] Probe output: Go `GenDataCodeV0([]byte("abc"), 512)` and `GenTextCodeV0("hello", 512)` equal
    next.md's two codes — re-run by review in a `git archive` copy, both equal, and equal to the
    Python binding
- [x] `grep -n "encode_component rejects a digest shorter" .claude/context/decisions.md` matches
- [x] `mise run check` exits 0
- [x] Extra probe (differential): Go `EncodeComponent` against the Python binding over 28,431
    combinations (types 0 to 8, versions 0 to 2, 13 `bits` values, 9 digest lengths): no difference
    in accept or reject, none in any code

**Issues found:**

- Filed `low`, untriaged: "Gen functions accept `bits` above 256 and return a unit that no decoder
    reads". Advance's finding, extended by review: `iscc_decode` rejects the 512-bit and 288-bit
    results on the binding ("truncated ISCC body") and on iscc-core 1.5.0 ("512 is not a valid LN").
    Pre-existing and identical to the reference; carries an `**Upstream:**` field. Not escalated
    (decision 2026-11-19: review-found old defects stay `low` until Titusz promotes them)
- The `normal` [audit] Go entry is rewritten to what is left: "No shared vector pins the
    short-digest rule of `encode_component`"
- Not filed (not a defect): where one input has two faults, Go and Rust name different ones, e.g.
    Version 1 with a short digest gives `invalid Version: 1` on Go and `digest length …` on Rust.
    Error text is not a contract on any surface

**Codex review:** no actionable findings. It confirmed that the public encoder rejects undersized
digests and that gen and decompose output is unchanged; its own native and 386 Go runs passed.

**Next:** the `encode_component` section in `data-edge.json`, a 12-surface fan-out citing the
rewritten `normal` [audit] issue. Two rows are enough: a short digest (error) and an exact-length
digest (code). Give each error row exactly one fault and compare no message (see above). Do not add
a row with a digest longer than `bit_length/8` unless every surface is measured first: this review
compared Go with the Python binding only.

**Notes:**

- decisions.md was 401 lines; `cid.archive_overflow` rotated the oldest entry ("Comment-only edits
    and regenerated files do not count against the 6-file cap", still binding) into
    `decisions-archive.md`. It is now 388 lines
- Other `normal` issues still open: `alg_minhash_256([])` (needs a decision), the WASM hasher
    `finalize()` consume rule (needs a decision)
- No Rust file changed, so the CRAP, iai and semver gates did not move. Go's `EncodeComponent` keeps
    its signature; it now rejects an input it used to accept (no `API-BREAK`, as with the decode
    tightening of 2026-07-24)
- `docs/howto/go.md` and `packages/go/README.md` show `EncodeComponent` with an exact-length digest
    and state nothing about short ones: no doc is stale
