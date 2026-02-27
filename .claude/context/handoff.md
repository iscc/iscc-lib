## 2026-02-27 — Review of: Implement pure Go GenImageCodeV0 and GenAudioCodeV0

**Verdict:** PASS

**Summary:** GenImageCodeV0 and GenAudioCodeV0 implemented as pure Go functions with clean,
well-structured code that precisely mirrors the Rust reference. All 8 conformance vectors pass (3
image + 5 audio), all quality gates clear, and the diff is tightly scoped to the 4 new files
specified in next.md.

**Verification:**

- [x] `cd packages/go && go build ./...` exits 0
- [x] `cd packages/go && go test -run TestPureGoGenImageCodeV0 -count=1 -v` — 3/3 image vectors PASS
- [x] `cd packages/go && go test -run TestPureGoGenAudioCodeV0 -count=1 -v` — 5/5 audio vectors PASS
- [x] `cd packages/go && go vet ./...` exits 0
- [x] `cd packages/go && go test ./...` — all tests pass
- [x] `grep -c 'func GenImageCodeV0' packages/go/code_content_image.go` returns 1
- [x] `grep -c 'func GenAudioCodeV0' packages/go/code_content_audio.go` returns 1
- [x] `mise run check` — all 14 hooks pass

**Issues found:**

- (none)

**Codex review:** Codex flagged three potential issues, all dismissed after cross-referencing with
Rust: (1) `PutUint32(uint32(v))` for negative int32 — correct two's complement conversion matching
Rust's `to_be_bytes()`. (2) AlgSimhash error suppression — safe since all digests are guaranteed 4
bytes. (3) Missing bits validation — correctly deferred to `EncodeComponent`, same as Rust.

**Next:** Implement `GenVideoCodeV0` and `GenMixedCodeV0` as the next gen function pair. Video-Code
reuses DCT + WTA-Hash + SimHash pipeline (per-frame DCT → WTA-Hash per frame → SimHash across
frames). Mixed-Code processes multiple ISCC content codes via decompose + sort + group + SimHash.
After these two, only `GenIsccCodeV0` remains to complete all 9 gen functions.

**Notes:** The `arraySplit[T any]` generic helper is well-implemented and will be reused by
GenMixedCodeV0. With 6/9 gen functions done (Meta, Text, Data, Instance, Image, Audio), 3 remain
(Video, Mixed, ISCC) plus `conformance_selftest` and WASM bridge cleanup. Fixed an incorrect
learning about `alg_simhash` output length (it equals input digest length, not always 32 bytes).
