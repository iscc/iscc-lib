## 2026-11-21 — Review of: Reject video frame signatures of unequal length in the Rust core and the Go port

**Verdict:** PASS

**Summary:** `soft_hash_video_v0` / `gen_video_code_v0` (Rust core) and `SoftHashVideoV0` /
`GenVideoCodeV0` (Go) check every frame signature's length against the first one before
deduplication and return an error on the first mismatch. The diff stays inside next.md's scope, the
tests feed one bad case per call and assert the variant and the exact message, and the CRAP baseline
moved in the same step. No public signature changed.

**Verification:**

- [x] `cargo test -p iscc-lib` passes, including the ragged-input tests (a)–(d), none panics — lib
    308 passed, all 8 targets ok; also green with `--no-default-features` (257) and
    `--features text-processing` (278)
- [x] `go test ./... && go vet ./...` in `packages/go` passes with Go tests for (a)–(d) — 6 video
    tests `PASS` by name under `CGO_ENABLED=0`, `gofmt -l` empty
- [x] `cargo clippy -p iscc-lib -- -D warnings` and `cargo fmt -p iscc-lib --check` clean — also
    `cargo clippy --workspace --all-targets -- -D warnings`
- [x] `cargo llvm-cov …` + `cargo crap … --fail-regression --fail-above` exits 0 — 0 regressed, 0
    new, 149 unchanged
- [x] `mise run bench:iai:check` exits 0 — 16 benchmarks within 10 %, `frames_10x380` 27,801 →
    27,757 Ir (-0.16 %), `.iai-baseline.json` untouched
- [x] `grep -n "frame_sigs\[" …` shows the length-mismatch error in both files — `lib.rs:737`,
    `code_content_video.go:43`
- [x] `mise run check` exits 0

**Issues found:**

- Nit, not filed: `check_frame_sig_lengths` handles an empty slice, but its only caller rejects
    empty input first, so that branch is never reached (the helper scores 91.7 % coverage).
- next.md claimed that the reference raises `IndexError` for a signature shorter than 380. Measured
    with iscc-core on Python 3.13 at 64 bits: `[380, 381]`, `[381, 380]` and `[380, 379]` all return
    a code. The shipped doc sentence ("iscc-core truncates to the shortest") is correct; the wrong
    claim reached no file.

**Codex review:** no actionable findings. It confirmed that both ports reject before deduplication
and summation and that valid inputs behave as before.

**Next:** the Go JCS escaping fix in `packages/go/code_meta.go` (issue "Go Meta-Code differs from
every other surface when metadata contains `&`, `<`, `>` or U+2028/9"). After it, ONE fan-out step
adds the two new `data-edge.json` sections to all 12 harnesses: the `gen_video_code_v0` ragged error
vector and the `gen_meta_code_v0` JCS vector.

**Notes:**

- The video issue is rewritten to what is left: the shared error vector. For that vector use
    `[380-long, 381-long]` with a `diverges` marker, because the reference returns a digest for it
    (decisions.md 2026-11-21, "Video frame signatures of unequal length are rejected").
- The human issue also asked that the Python-only frame check be "removed or reduced to the core's
    error". It stays, as next.md scoped: `extract_frame_sigs` flattens the frames into one buffer
    before it calls the core, and it already raises `ValueError` with the core's wording. The shared
    vector will prove that Python raises like the other surfaces. Titusz can reopen this if he wants
    the check gone.
- No binding test covers ragged input yet. The bindings were not rebuilt or probed in this review;
    the claim that they now raise their usual error rests on the core returning `Err` and is pinned
    only by the fan-out vector.
- valgrind and `iai-callgrind-runner@0.16.1` are not in the devcontainer image. `cargo binstall` is
    not installed either, so the recipe is `cargo install --locked iai-callgrind-runner@0.16.1`
    (learnings updated).
- `decisions.md` was over budget (406 lines). One entry was added and the two oldest entries were
    rotated to `decisions-archive.md` with the runner's own function; it is at 390 lines.
- The 2026-11-17 and 2026-11-20 advisor entries in `decisions.md` are partly superseded by the human
    entry of 2026-11-21 ("No role uses the advisor"). They are human entries and still carry the
    model and effort rulings, so they were left as they are.
