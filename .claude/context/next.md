# Next Work Package

## Step: Go `EncodeComponent` rejects a digest shorter than `bit_length/8`

## Goal

Picks up the `normal` [audit] issue "Go `EncodeComponent` truncates a short digest where every
Rust-backed surface raises". Go's public `EncodeComponent` returns an `iscc:` error for a short
digest, as the Rust core's Tier 1 `encode_component` does. The rule goes into decisions.md, so the
next step can pin it with a 12-surface `encode_component` vector in `data-edge.json`.

## Alternatives Considered

- **Chosen:** the Go behaviour change plus the recorded rule, Go only. A change to the Go port goes
    first and alone, and the shared vector cannot pass on Go until this lands.
- **Rejected:** Go fix and `data-edge.json` section in one step. That combines a port behaviour
    change with a 12-surface fan-out, which the sizing rules forbid. Also rejected: making the gen
    functions reject `bits` beyond their digest. That is a separate behaviour, shared with iscc-core
    (see Implementation Notes), and is not ours to decide here.

## Scope

- **Modify**: `packages/go/codec.go`: the rule lives here (the only file whose behaviour changes).
- **Modify** (one-call swap, no behaviour change): `packages/go/code_content_audio.go`,
    `code_content_image.go`, `code_content_mixed.go`, `code_content_text.go`,
    `code_content_video.go`, `code_data.go`, `code_instance.go`, `code_meta.go` (2 call sites).
    Decision 2026-11-20 counts toward the cap only files whose behaviour changes, so this is 1
    behaviour file. Review confirms the 8 swaps are neutral from the diff and the probe below.
- **Modify**: `.claude/context/decisions.md`: append the rule.
- **Tests**: `packages/go/codec_test.go`.
- **Reference**: `crates/iscc-lib/src/lib.rs` (`encode_component`, ~lines 202-230; test near 2155),
    `crates/iscc-lib/src/codec.rs:542-564` (Tier 2 `codec::encode_component`, which clamps),
    `reference/iscc-core/iscc_core/codec.py:43-45`.

## Not In Scope

- The `encode_component` section in `data-edge.json` and its 12 harnesses. That is the next step (a
    fan-out). Do not touch `data-edge.json` or any vendored copy.
- Changing what any `Gen*` function returns for `bits` above its digest length (e.g. 512). Every
    surface and iscc-core agree there today. Go must keep matching them.
- Rust core changes. The core already rejects short digests (`lib.rs`), and `codec.rs` keeps
    clamping for internal callers.
- Changing the order of the existing MainType/SubType/Version checks in `EncodeComponent`.

## Implementation Notes

Mirror the Rust split exactly:

- Rust Tier 1 `encode_component` (lib.rs) = enum validation + the digest-length check, then
    `codec::encode_component`.
- `codec::encode_component` (codec.rs) = the ISCC MainType rejection + length/header encoding +
    `&digest[..nbytes.min(digest.len())]` (clamps). The Rust gen functions and decompose call this
    one.

In Go:

1. Move today's clamping body (MTIscc rejection, `encodeLength`, `encodeHeader`, clamped append)
    into `encodeComponentInternal` (typed `MainType`/`SubType`/`Version`). Keep it clamping, and
    keep its doc comment true.
2. `EncodeComponent` keeps its MainType/SubType/Version validation. Then it rejects
    `len(digest) < int(bitLength/8)` with
    `fmt.Errorf("iscc: digest length %d < bit_length/8 (%d)", len(digest), bitLength/8)` (same
    wording as Rust), then calls `encodeComponentInternal`. Update its doc comment to state the
    short-digest error.
3. Every `Gen*` call site (the 9 calls in the 8 files above) switches from
    `EncodeComponent(uint8(MTx), uint8(STy), uint8(VSV0), bits, d)` to
    `encodeComponentInternal(MTx, STy, VSV0, bits, d)`. This matches Rust, where the gen functions
    use `codec::encode_component`, and it gives the surface one form, not two. The decompose call
    sites in `codec.go` already use the internal helper.

Measured while scoping. On the Python binding (Rust-backed) and on iscc-core 1.5.0,
`gen_data_code_v0(b"abc", 512)` returns
`ISCC:GAEHA6W7RKLGNVVVKKJZC3YRAK357XX6X7HKIOLPOBY7BC5QBONX2BKZ` and `gen_text_code_v0("hello", 512)`
returns `ISCC:EAEHA2J6QMX4ZG2CUO7YPWBB4MECFBWWVJDPMQLTDS4CDX62B55UGXO4`. Both have a 512-bit header
over a 32-byte body. Go must still return exactly these after the change. Do not commit a test that
pins this output: the behaviour has not been chosen. Run it as a temporary probe, delete it, and
paste the output into the handoff. Name it in the handoff as a finding for review to file (gen
`bits` above 256 gives a header that claims more bits than the body on every surface and in
iscc-core). iscc-core `encode_component(0,0,0,64,b"\x01\x02\x03\x04")` returns `AAAQCAQDAQ` (it
slices). The decision must say that Rust and Go diverge from it here on purpose.

**Decision entry** (`## 2026-11-22 — encode_component rejects a digest shorter than bit_length/8`):
the Tier 1 `encode_component` raises on every surface, where iscc-core slices. Why: a header that
claims more bits than the body has is a malformed unit, and eleven surfaces already raise. Gen
functions keep the reference's behaviour: the internal encode path clamps. A shared vector follows
in the next step.

Tests (codec_test.go): a short digest (4 bytes, 64 bits) returns an error that contains
`digest length`. A zero-length digest with 32 bits is rejected. An exact-length digest still encodes
(an existing test may already cover this). One bad case per call. This is a Go-only step, so the
Rust CRAP and iai gates are untouched.

## Verification

- `cd packages/go && go test ./...` passes, including the new short-digest tests
- `cd packages/go && go vet ./...` clean and `CGO_ENABLED=0 GOARCH=386 go test ./...` passes
- `grep -n "EncodeComponent(" packages/go/*.go | grep -v _test.go` prints only the definition and
    doc-comment lines in `codec.go` (no gen call site uses the public function)
- The handoff shows the probe output: Go `GenDataCodeV0([]byte("abc"), 512)` and
    `GenTextCodeV0("hello", 512)` equal the two codes above
- `grep -n "encode_component rejects a digest shorter" .claude/context/decisions.md` matches
- `mise run check` exits 0

## Done When

Go's public `EncodeComponent` rejects a short digest with an `iscc:` error, the Go gen paths still
clamp (output unchanged), the rule is in decisions.md, and every check above passes.
