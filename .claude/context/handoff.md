## 2026-02-27 — Review of: Implement pure Go GenDataCodeV0 and GenInstanceCodeV0

**Verdict:** PASS

**Summary:** Clean implementation of `GenDataCodeV0` and `GenInstanceCodeV0` as pure Go gen
functions with all 7 conformance vectors passing (4 data + 3 instance). `DataHasher` and
`InstanceHasher` streaming structs follow the Python/Rust reference correctly. The WASM bridge type
rename (`DataHasher`→`WasmDataHasher`, `InstanceHasher`→`WasmInstanceHasher`) was a minimal,
necessary adaptation to resolve Go package-level name collisions.

**Verification:**

- [x] `cd packages/go && go build ./...` exits 0
- [x] `cd packages/go && go test -run TestPureGoGenDataCodeV0 -count=1 -v` — 4/4 data vectors PASS
- [x] `cd packages/go && go test -run TestPureGoGenInstanceCodeV0 -count=1 -v` — 3/3 instance
    vectors PASS
- [x] `cd packages/go && go vet ./...` exits 0
- [x] `cd packages/go && go test ./...` — all tests pass (pure Go + WASM bridge)
- [x] `cd packages/go && go test -race -run "TestPureGo(GenDataCodeV0|GenInstanceCodeV0)" -count=1`
    — race detector clean
- [x] `grep -c 'func GenDataCodeV0' packages/go/code_data.go` returns 1
- [x] `grep -c 'func GenInstanceCodeV0' packages/go/code_instance.go` returns 1
- [x] `grep -c 'type DataHasher struct' packages/go/code_data.go` returns 1
- [x] `grep -c 'type InstanceHasher struct' packages/go/code_instance.go` returns 1
- [x] `mise run check` — all 14 hooks pass

**Issues found:**

- (none)

**Codex review:** Codex flagged that `DataHasher.Finalize()` mutates internal state (appends to
`chunkFeatures`, clears `tail`), making it non-idempotent — calling Finalize twice would produce
different results. This mirrors the Python reference (`DataHasherV0._finalize()` sets
`self.tail = None`) and Rust (which consumes `self` on finalize). The single-use-finalize pattern is
by design. Not a blocking issue, but worth noting for documentation if a public streaming API guide
is written later.

**Next:** Implement `GenImageCodeV0` and `GenAudioCodeV0` as the next gen function sub-step. Image
code needs DCT + WTA-Hash (both already implemented). Audio code needs SimHash on Chromaprint
features (SimHash already implemented). 5 conformance vectors for image + 3 for audio = 8 total
vectors. After these two, only `GenVideoCodeV0`, `GenMixedCodeV0`, and `GenIsccCodeV0` remain.

**Notes:** The WASM bridge type rename is a one-time adaptation. Future gen functions that share
names with WASM bridge functions (`GenImageCodeV0`, `GenVideoCodeV0`, etc.) will NOT need renames
because the WASM bridge wraps them as `Runtime.GenImageCodeV0()` (method on `Runtime` struct) while
the pure Go versions are package-level functions — Go resolves these differently. Only struct type
names (like `DataHasher`/`InstanceHasher`) had actual collisions.
