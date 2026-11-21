# Next Work Package

## Step: Reject video frame signatures of unequal length in the Rust core and the Go port

## Goal

Fix the behaviour behind the `normal` issue "Core video hashing panics or returns a wrong digest on
frame signatures of unequal length": `soft_hash_video_v0` (and so `gen_video_code_v0`) returns
`InvalidInput` instead of panicking (which aborts the host process in release builds) or returning a
digest that depends on input order. The Go port has the same defect and is fixed in the same step.

## Alternatives Considered

- **Chosen:** core + Go video-length fix. It is the only open bug that kills the host process, and
    the core changes behaviour, so it lands alone and first, before any vector fan-out.
- **Rejected:** Go JCS escaping fix (`code_meta.go`). It is also a correctness bug, but a soft one
    (wrong Meta-Code, no crash). It comes next. After it, ONE fan-out step adds the two new
    `data-edge.json` sections (`gen_video_code_v0` error vector + `gen_meta_code_v0` JCS vector) to
    all 12 harnesses. A new section is an unsplittable 12-surface fan-out (learnings 226), and core
    and Go behaviour changes are never combined with a fan-out, so the vector is deliberately left
    for that step.

## Scope

- **Modify**: `crates/iscc-lib/src/lib.rs` (`soft_hash_video_v0`, its doc comment and unit tests),
    `packages/go/code_content_video.go` (`SoftHashVideoV0` + doc comment), `docs/rust-api.md`
    (`soft_hash_video_v0` / `gen_video_code_v0` error sentence), `.crap-baseline.json` (regenerated)
- **Tests**: `crates/iscc-lib/tests/test_algorithm_primitives.rs` and/or `lib.rs` `mod tests`;
    `packages/go/code_content_video_test.go`
- **Reference**: `reference/iscc-core/iscc_core/code_content_video.py`,
    `crates/iscc-lib/src/wtahash.rs` (`alg_wtahash` already rejects < 380 columns),
    `crates/iscc-py/src/lib.rs` `extract_frame_sigs` (the existing Python-side check)

## Not In Scope

- Adding the `data-edge.json` vector or touching any binding harness. That is the follow-up fan-out
    step (see Alternatives). Do not add a section to `data-edge.json` here: every harness fails
    closed on an unknown section.
- Changing the Python binding's `extract_frame_sigs` length check. It has to stay, because it builds
    a flat buffer with `chunks_exact(frame_len)`, and it already raises `ValueError` like the core
    error. Error text is never matched across surfaces (`edge-vectors.md`).
- Requiring exactly 380 columns, or changing the empty-input or `bits` handling.
- The other `normal` issues (Go JCS, FFI NULL, integer wrap, docs, pytest time).

## Implementation Notes

- Validate **before** deduplication: walk all of `frame_sigs` (input order) and compare each `len()`
    with `frame_sigs[0]`. On the first mismatch return
    `IsccError::InvalidInput(format!("frame_sigs[{i}] has length {n}, expected {first}"))`. Then the
    dedup and sum code can keep `cols = frame_sigs[0].len()` safely. Go mirrors this with
    `fmt.Errorf` and the same wording.
- This deliberately deviates from iscc-core. The reference `zip(*sigs)` truncates to the shortest
    signature, so `[380-long, 381-long]` gives a digest there and a shorter-than-380 one raises
    `IndexError`. The human issue makes rejection the resolution. Say so in the doc comment in one
    line. The follow-up vector will carry a `diverges` marker.
- Tests, Rust and Go each, **one bad case per call** (learnings): (a) a longer later signature
    (`[vec![0;380], vec![0;381]]`) → `Err`, no panic; (b) a shorter later signature (`[380, 379]`) →
    `Err`; (c) the same pair in reversed order → `Err` (proves the result no longer depends on
    order); (d) `gen_video_code_v0` with a ragged input → `Err`; (e) equal-length duplicates still
    produce the existing conformance result (existing tests cover this; keep them green). Assert the
    `InvalidInput` variant on the raw call.
- **CRAP (CI-only gate):** the new branch in `soft_hash_video_v0` and the lines it moves in `lib.rs`
    re-score functions. Regenerate the baseline in this step with `mise run crap:baseline` (it runs
    `mise run coverage` first) and commit `.crap-baseline.json`. Never widen epsilon/threshold.
- **iai:** `bench_video_code.frames_10x380` (baseline 27,801 Ir) runs `gen_video_code_v0`. A
    10-element length pass costs a few dozen Ir, far below 10 %. Still run the gate locally (needs
    valgrind + `iai-callgrind-runner@0.16.1`, see learnings), and do not touch `.iai-baseline.json`
    unless the gate fails.
- `docs/rust-api.md` lines ~157 and ~467 say which inputs give `InvalidInput`. Add the ragged case.

## Verification

- `cargo test -p iscc-lib` passes, including the new ragged-input tests (a)–(d), and none of them
    panics
- `(cd packages/go && go test ./... && go vet ./...)` passes, including Go tests for (a)–(d)
- `cargo clippy -p iscc-lib -- -D warnings` and `cargo fmt -p iscc-lib --check` are clean
- `cargo llvm-cov -p iscc-lib --lcov --output-path lcov.info && cargo crap --lcov lcov.info --baseline .crap-baseline.json --fail-regression --fail-above`
    exits 0 against the committed baseline
- `mise run bench:iai:check` exits 0
- `grep -n "frame_sigs\[" crates/iscc-lib/src/lib.rs packages/go/code_content_video.go` shows the
    length-mismatch error in both files

## Done When

Ragged frame signatures make both the Rust core and the Go port return an error (no panic, no
digest), and every verification command above exits 0.
