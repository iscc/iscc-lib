## 2026-11-21 — Realign the specs and crate CLAUDE.md files with the shipped code

**Done:** Rewrote every statement the audit issue cites, in the present tense, to match the tree. No
code file changed. Where a sentence next to a cited block said the same stale thing, I fixed it as
well (listed under Notes).

**Files changed:**

- `.claude/context/specs/rust-core.md`: IDv1 Part 1 now says what shipped. `Version` has `V0`/`V1`
    (`#[non_exhaustive]`) and `validate_version` admits `(any, V0)` and `(Id, V1)`, which matches
    codec.rs:105-130. The failing-output block is replaced by live `iscc_lib` output. Also fixed
    "once Part 1 lands", and "Codec changes this requires" → "Codec rules for Version 1" (present
    tense)
- `.claude/context/specs/python-bindings.md`: the "GIL Release for Text/Video" section is now in the
    present tense. Text, video, video_flat, soft_hash_video and soft_hash_video_flat run inside
    `py.detach` (I checked the `.detach(` sites at iscc-py lib.rs:140/192/219/243/609). The
    `extract_frame_sigs` caveat stays, as a constraint that holds
- `.claude/context/specs/go-bindings.md`: removed `io.Reader` from 3 places (:13 is now
    "`Push`/`Finalize` hashers for streaming"). The "exist only on `develop`" paragraph is replaced
    (see Notes)
- `.claude/context/specs/wasm-bindings.md`: "Rust Bridge Layer" now shows the real
    `gen_meta_code_v0`, copied verbatim from iscc-wasm lib.rs:181-191. It has snake_case exports,
    `JsValue` + `unchecked_optional_param_type` + `opt_u32`, and returns a `String`. The text names
    `gen_sum_code_v0` → `WasmSumCodeResult` as the exception. Two "Key Differences" bullets were
    also wrong and are fixed: the WASM `gen_sum_code_v0` takes bytes, not a path, and serde is used
    only to deserialize array inputs
- `crates/iscc-{napi,py,wasm}/CLAUDE.md`: `iscc_lib::Error` → `iscc_lib::IsccError`; mdformat
    realigned the tables
- `packages/go/CLAUDE.md`: the per-file unit-test list is replaced by a rule on file names
    (`<name>_test.go` sibling, `code_*_test.go`). I checked that every non-test `.go` file has a
    sibling test file. Publishing now says the proxy needs `packages/go/vX.Y.Z`, which `release.yml`
    creates on a `workflow_dispatch` with a `version` input; the "Create tags" step at :112-115 is
    gated by `if: inputs.version != ''` on `prepare-release`. The FFI claim now says only C++ and
    .NET wrap `iscc-ffi`

**Verification:**

- All 6 grep criteria in next.md print nothing, and `grep -n 'packages/go/v' packages/go/CLAUDE.md`
    prints line 126
- `uv run scripts/check_spec_paths.py` → "OK: every tree entry and backticked path in 15 specs
    resolves."
- `mise run format` rc=0, and `mise run check` rc=0 (all hooks passed)
- `git diff --stat`: only the 8 scoped files, plus the runner's `iterations.jsonl`, which I did not
    stage
- IDv1 decode output, live: `uv run python -c "...iscc_decode('ISCC:MAIGHFECJMOPMIAB')"` →
    `(<MT.ID: 6>, <ST.NONE: 0>, <VS.V1: 1>, 0, b'c\x94\x82K\x1c\xf6 \x01')`. The spec shows it as
    `MT.ID, ST.NONE, VS.V1`

**Next:** `conformance_selftest`'s single section runner (Rust + Go, plus `.crap-baseline.json` in
the same step), as handoff and next.md proposed.

**Notes:**

- I departed from next.md on Go: it says "the old symbols were removed in 0.6.0", but that is false.
    `git grep 'func EncodeIsccID\|func GenIsccIDV1' <tag> -- packages/go` finds no IDv1 API in
    v0.3.0–v0.5.0, and only `GenIsccIDV1` in v0.6.0 and v0.7.0. So the spec now says that neither
    old symbol was ever in a tagged release and that `GenIsccIDV1` first shipped in 0.6.0. The
    "update issue #43 when this lands" sentence is dropped
- Out of scope, not touched: `crates/iscc-py/CLAUDE.md` "Test Patterns" still lists test files one
    by one (`test_new_symbols.py`, …). This is the same kind of list the 2026-11-21 decision forbids
    in per-crate CLAUDE.md files, and it is likely stale. It is a candidate for a later sweep
- The `iscc_core.iscc_decode` reference example in rust-core.md is kept unchanged
