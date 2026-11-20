## 2026-11-20 — Review of: `alg_simhash` rejects an empty list in the Rust core and the Go port

**Verdict:** PASS_WITH_NOTES

**Summary:** The public `alg_simhash` (Rust) and `AlgSimhash` (Go) now return "hash_digests must not
be empty" for `[]`. The generators keep their output through `alg_simhash_inner` and the new
unexported Go `algSimhash`. Every test and doc comment that pinned the old 32-zero answer was
flipped, and the work matches next.md and the 2026-11-20 decision. Codex found nothing.

**Verification:**

- [x] `cargo test -p iscc-lib -p iscc-ffi` passes;
    `cargo clippy --workspace --exclude iscc-rb -- -D warnings` is clean — all suites ok (303 unit
    tests in the core), clippy clean, also with `--all-targets`
- [x] `go test ./...` and `go vet ./...` pass in `packages/go`, and
    `grep -rn "AlgSimhash(" packages/go/code_*.go` prints nothing — confirmed, `gofmt -l` empty
- [x] `uv run pytest tests/test_algo.py` passes — 18 passed
- [x] napi suite after a fresh build, and `wasm-pack test --node crates/iscc-wasm` — 179/179, and
    109 WASM tests passed (11, 5 and 93 per target)
- [x] `mvn -f crates/iscc-jni/java/pom.xml clean test` and the .NET suite pass — 108 run, 0
    failures; 150 passed (run as `dotnet test packages/dotnet/Iscc.Lib.Tests/`, see Notes)
- [x] cbindgen + `cargo build -p iscc-ffi` leave `iscc.h` and `NativeMethods.g.cs` unchanged, and
    the CI CRAP line exits 0 — `git status` clean after the regen; gate: 0 regressed, 0 new, 145
    unchanged

Beyond next.md: `mise run check` exit 0; `cargo test -p iscc-lib` with `--no-default-features` and
`--all-features` green; a per-function diff of `.crap-baseline.json` shows `alg_simhash` 4 → 5 as
the only changed score, plus 26 functions from `tests/test_data_edge.rs` that the old baseline
lacked (highest 20, under the limit of 30).

**Issues found:**

- `docs/howto/rust.md` listed `sliding_window` and `alg_simhash` without their `IsccResult` return
    type (older than this step). Fixed in the review commit, together with one sentence on each
    function's error case.
- The diff touches 10 hand-edited source files where the cap is 6. Four of them (napi, py, `.pyi`,
    wasm) changed only a doc comment, and next.md said so. I passed this and recorded the reading in
    decisions.md ("Comment-only edits and regenerated files do not count against the 6-file cap").
    Titusz may overrule it.
- advance added a "Verified when" bullet to `specs/rust-core.md` where next.md asked only for the
    table row. The issue is `[human]` with a `**Spec:**` field and the bullet restates the human
    decision, so it stays.
- WASM's flipped test checks only `is_err()`, like the other rejection tests in that file. Not
    filed.

**Codex review:** No actionable findings. Codex ran the Rust core/FFI, Go and .NET tests itself (all
green); it could not collect the Python tests in its own environment.

**Next:** The `alg_simhash` section in `data-edge.json`: one `[]` vector with `error`, a handler in
every harness, the validator and the generated C header (`gen_data_edge_vectors.py`). This is a
fan-out — the behaviour is fixed by the core and by decisions.md 2026-11-20. Then the five C++
`ptrs.data()` wrappers, then the gate-leniency issue in `scripts/check_spec_paths.py`.

**Notes:**

- The issue entry is rewritten to the two remaining parts. New fact for the C++ step:
    `iscc_alg_simhash` handles `num_digests == 0` before its NULL checks, so C and C++ already
    report the core message for `alg_simhash({})`. The data-edge vector therefore cannot pin that
    wrapper's pointer; the C++ step needs its own test per wrapper. The other four FFI functions
    have no such branch.
- next.md's `dotnet test packages/dotnet` has no project file to run. Use the `ci.yml` form:
    `dotnet test packages/dotnet/Iscc.Lib.Tests/ -e LD_LIBRARY_PATH=$PWD/target/debug` after
    `cargo build -p iscc-ffi`.
- next.md counted eight generator call sites in Go; there are seven (audio 3, mixed 1, meta 3).
- The unpushed range also holds the `cid(human)` commit 32d6686 (review model and effort,
    `sanitize_env`). It weakens no gate.
- `cargo-crap` 0.2.2 and `cargo-llvm-cov` are now installed in `~/.cargo/bin` in this container.
