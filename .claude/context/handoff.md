## 2026-11-21 — Reject video frame signatures of unequal length in the Rust core and the Go port

**Done:** `soft_hash_video_v0` (and so `gen_video_code_v0`) in the Rust core and `SoftHashVideoV0` /
`GenVideoCodeV0` in the Go port validate every frame signature's length against `frame_sigs[0]`
before deduplication. The first mismatch returns `InvalidInput` / an error with the wording
`frame_sigs[{i}] has length {n}, expected {first}`. The check is a small private helper in each
(`check_frame_sig_lengths` / `checkFrameSigLengths`). The doc comments name the deliberate deviation
from iscc-core in one line.

**Files changed:**

- `crates/iscc-lib/src/lib.rs`: helper `check_frame_sig_lengths`, called after the empty check in
    `soft_hash_video_v0`; error sentences in both doc comments; 5 unit tests (longer later, shorter
    later, reversed order, first-mismatch index, `gen_video_code_v0` ragged), each one bad case per
    call, each asserting `InvalidInput` plus the exact message on the raw call
- `packages/go/code_content_video.go`: `checkFrameSigLengths` + call in `SoftHashVideoV0`; both doc
    comments
- `packages/go/code_content_video_test.go`: 6 tests, the same 5 cases plus an equal-length duplicate
    check
- `docs/rust-api.md`: ragged-input `InvalidInput` sentence for `gen_video_code_v0` and
    `soft_hash_video_v0`
- `.crap-baseline.json`: regenerated with `mise run crap:baseline`. The diff covers only
    `soft_hash_video_v0` (cc 4→5) and the added `check_frame_sig_lengths` (cc 3, CRAP 3.005)

**Verification:**

- `cargo test -p iscc-lib`: all suites ok (lib 308 passed, including the 5 new tests and
    `test_gen_video_code_v0_conformance`)
- Mutation check: with the helper call neutralised, the 5 new Rust tests FAIL and the Go run panics
    `index out of range [380] with length 380` in `TestSoftHashVideoV0RejectsLongerLaterSig`.
    Restored afterwards
- `go test ./... && go vet ./...` in `packages/go`: ok. `gofmt -l .` printed nothing
- `cargo clippy -p iscc-lib -- -D warnings` and `--all-targets`: clean.
    `cargo fmt -p iscc-lib --check`: ok
- `cargo crap --lcov lcov.info --baseline .crap-baseline.json --fail-regression --fail-above`: exit
    0
- `mise run bench:iai:check`: exit 0, 16 benches within 10 %. `frames_10x380` 27,801 → 27,757 Ir
    (-0.16 %). `.iai-baseline.json` untouched
- `mise run check`: exit 0
- `grep -n "frame_sigs\[" …` shows the error at lib.rs:737 and code_content_video.go:43

**Next:** the Go JCS escaping fix (`code_meta.go`), then the single 12-harness fan-out that adds the
`gen_video_code_v0` ragged error vector (with `diverges`) and the `gen_meta_code_v0` JCS vector to
`data-edge.json`.

**Notes:**

- No public API change: same signatures, only a new error case.
- The bindings were not touched. They all delegate to the core, so they now raise their usual
    InvalidInput mapping for ragged input. The Python `extract_frame_sigs` check rejects first, as
    before. I grepped the binding tests for 379/381-length video inputs: the only hits are
    `(1..381)` / `range(1, 381)`, which are 380 elements long, so no test relied on ragged input
    succeeding.
- valgrind is not in the devcontainer. I installed it with
    `apt-get update && apt-get install   valgrind`, then
    `cargo install --locked iai-callgrind-runner@0.16.1`, because `cargo binstall` is not installed.
    That replaces the learnings recipe.
