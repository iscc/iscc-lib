## 2026-11-18 — Review of: Go `EncodeSeq` / `DecodeSeq` (ISCC-SEQ on the pure-Go port)

**Verdict:** PASS_WITH_NOTES

**Summary:** The Go port follows the Rust `read_unit` / `encode_seq` / `decode_seq` step for step.
It uses raw `uint32` header helpers under typed wrappers that behave as before, so SubType ≥ 256 and
Version 1 round-trip. Go and Rust behave the same: a corpus check on 3,296 `EncodeSeq` and 30,009
`DecodeSeq` inputs, compared through the Python binding, found 0 differences. The tests missed two
Rust cases. With no SEMANTIC unit in any vector or test, dropping SEMANTIC from `seqMainTypes` kept
every seq test green. Review added `TestSeqMainTypes` and `TestReadUnitOffsets128`, ported from
`test_seq.rs`. The first fails under that mutation.

**Verification:**

- [x] `CGO_ENABLED=0 go -C packages/go test -count=1 ./...` gives `ok`. `go vet` is clean and
    `gofmt -l packages/go` prints nothing, both before and after the review-added tests
- [x] The `EncodeSeq` (codec.go:879) and `DecodeSeq` (:919) signature greps print one line each.
    `grep -n 'func ReadUnit'` prints nothing (exit 1)
- [x] `seq_test.go` covers each required case with its own assertion: all 5
    `STRICT_BASE32_DIVERGENCES` rows (rejected, twins accepted), SubType 4679 and Version 1 round
    trips, a non-zero padding nibble (also at a second-unit offset), readUnit offsets
    `len`/`len+1`/`100`/`-1`, and `faaa-aaaaaaaa` → `28000000000000`
- [x] `AllocsPerRun(asciiUpper("GAAWDLQNG6SSDPF2")) == 0`. It goes through a package-level sink, so
    inlining cannot hide an allocation
- [x] Each runner fails on one mutated real vector, in its own test. There are 5 single-case failure
    arms per runner plus a positive control. `ConformanceSelftest()` returns `true` on the real data
- [x] `mise run check` exits 0 and modified no files
- [x] Probe: Go vs Rust (via `iscc_lib`) on whitespace, case-fold, multibase, prefix and display
    forms plus random bytes: 0 differences (212 and 487 accepted)
- [x] Gate scan of all 5 unpushed commits: no suppressions, skips, threshold or hook changes

**Issues found:**

- Fixed in review: two unported `test_seq.rs` cases (`seq_maintypes`, `read_unit_offsets_128`)
- Nit: the `Malformed ISCC-HEADER` error wraps an error that already starts with `iscc:`, so the
    message reads `iscc: Malformed … : iscc: decode header …`
- Nit: the strict-divergence test asserts only `err != nil`. The Python twin also pins the
    `base32 decode error` reason
- Nit: the `conformance.go` header says "every gen function". `GenSumCodeV0` has no vectors and no
    runner
- Nit: the runner failure-arm tests write `FAILED: …` lines to stderr. They show only under `-v`

**Codex review:** Three findings, all advisory, none adopted:

1. (P2) `bzqaq4yamaaqaaaaaaaaaaaaaaaaaaaa` (multibase `b` + `cc01` + unit) encodes as a different
    20-byte unit. iscc-core 1.5.0 and Rust give the same bytes, `0e6008…`, because the spec has
    each unit upper-cased before cleaning. This matches the reference, so review recorded it in
    decisions.md (2026-11-18) to keep Codex from re-flagging it on each surface.
2. (P2) An oversized unit string is base32-decoded in full before it is rejected. Rust does the
    same, and memory grows only linearly. Not filed.
3. (P3) A `null` seq section passes Go's selftest without running a case. Rust fails on `null` but
    also passes on `{}`. The data is embedded at compile time and `TestSeqSectionsRun` catches it.
    Not filed.

**Next:** The `normal` text-utils issue (`text_trim` / `text_remove_newlines` keep U+001C–U+001F):
next.md named it as the follow-up, and it is a Rust hot path, so `mise run bench:iai:check` must run
in the same step. Alternatives are the next ISCC-SEQ surface (Node, WASM, FFI, …; 2 of 11 are done)
or the small Python `encode_seq` docstring issue.

**Notes:**

- Deferred docs sweep (until all 11 surfaces bind seq): `packages/go/CLAUDE.md` still says "all 9
    gen functions" and "46 conformance vectors" and lists no seq functions. Add it to the 33 → 35
    sweep with the crate `CLAUDE.md`, `notes/04`, `docs/architecture.md` and the binding docs
- Recipe for the remaining surfaces: put a throwaway `zz_probe_test.go` in a `/tmp` copy of
    `packages/go` (it runs on `-run` families only, because conformance tests need `../../crates`).
    For bindings, run the same JSON corpus through the binding and compare with `iscc_lib`
- `decisions.md` is at 399 of 400 lines. The runner rotates the oldest entries on the next overrun
