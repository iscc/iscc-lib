## 2026-11-22 — Review of: `alg_minhash_256` returns an error for an empty feature list (Rust core API break)

**Verdict:** PASS_WITH_NOTES

**Summary:** The Rust core's `alg_minhash_256` now returns `IsccResult<Vec<u8>>` and rejects an
empty list with `features must not be empty`. All 7 binding crates pass the error on the way their
`alg_simhash` wrapper does, and C++, .NET, Swift and Kotlin get it through the C FFI and UniFFI.
Every criterion passed in my own run, and no gen output or perf row moved. **API-BREAK:** approved
beforehand (decisions.md 2026-11-22, "`alg_minhash_256` rejects an empty feature list").

**Verification:**

- [x] `cargo test -p iscc-lib` (default, `--no-default-features`, `--features text-processing`, also
    `--all-features`), `cargo test -p iscc-ffi -p iscc-uniffi` (121 + 31) and
    `cargo clippy --workspace --all-targets -- -D warnings` pass. `test_alg_minhash_256_empty`
    asserts `InvalidInput` and the exact message
- [x] Binding suites pass: WASM 11 + 8 + 136; napi in a `/tmp` archive copy 243 pass; JNI 117 + 8;
    Python 924; Ruby 175 runs; .NET 155; C test 150; C++ (cmake, ASAN) 121; Kotlin `./gradlew test`
    exit 0
- [x] The UniFFI freshness step body from `ci.yml` exits 0 after
    `cargo build -p iscc-ffi -p iscc-uniffi`, and `git diff --exit-code` on `iscc.h` and
    `NativeMethods.g.cs` exits 0. A fresh `cbindgen` run equals the committed header
- [x] The `xff` grep prints simhash tests only; no hit tests `alg_minhash_256`
- [x] On a fresh `mise run coverage`, the CI CRAP line reports 0 regressed, 0 new, 185 unchanged.
    `mise run bench:iai:check` exits 0: `bench_minhash_256` +0.00%, `bench_text_code` −0.06%, both
    `bench_data_code` rows −3.5% against the baseline (the gen functions call the unchecked helper,
    which is the old code)
- [x] `mise run check` exits 0, and `uv run pytest tests/test_rust_api_signatures.py` gives 5 passed

Two checks beyond the list: napi and Ruby have no test of the empty list, so I called them by hand.
Both report `invalid input: features must not be empty` (Ruby as `RuntimeError`), as their how-to
pages say.

**Issues found:**

- Scope: the step changed code in 10 non-test files (3 core, 7 binding `lib.rs`), over the 6-file
    cap, and next.md declared neither escape valve. I passed it and recorded why in decisions.md ("A
    core signature change lands with the binding edits the compiler forces"): the workspace does not
    compile between the core change and the binding edits, so no smaller step exists. Titusz may
    want to overrule or to put this case into the define-next protocol
- Not filed (nit): the XML doc of .NET `AlgMinhash256` (`IsccLib.cs`) and the `///` of C++
    `alg_minhash_256` (`iscc.hpp`) do not name the error. The how-to pages do. The data-edge fan-out
    touches both surfaces and can add the line
- The issue in issues.md is rewritten to what is left: the Go port and the shared vector section

**Codex review:** one run over the whole advance range (`--base` at the define-next commit). No
actionable findings: it confirmed that the error reaches every Rust-backed binding and that the
internal callers behave as before.

**Next:** the Go port. `AlgMinhash256` becomes `([]byte, error)` with the message
`features must not be empty`; the internal Go callers switch to an unchecked helper, as in Rust;
`TestMinhash256Empty`, `docs/howto/go.md` and the row in `specs/go-bindings.md` follow. After that,
the `alg_minhash_256` section in `data-edge.json` as one 12-surface fan-out (an error row and a
non-empty row). The other `normal` issue (`bits` above 256) comes after these two.

**Notes:**

- Swift is the only surface I could not build here (macOS CI job only). The generated
    `algMinhash256` is now `throws`. No call site exists in `packages/swift`, `packages/kotlin` or
    the docs examples, so nothing should break; update-state should still read the `swift` job of
    this push
- The `semver` CI job will report the break. It is `continue-on-error`, so CI stays green
- Go and the Rust core now disagree on `alg_minhash_256([])` (Go: 32 bytes of `0xFF`). That is the
    open issue, and it holds until the Go step lands
- The UniFFI Swift generator writes a stray `iscc_uniffiFFI.modulemap` into
    `packages/swift/Sources/IsccLib/`. Delete it after every regeneration; the freshness gate does
    not see it
- The regenerated CRAP baseline also took in 3 test helpers of `tests/test_data_edge.rs` and line
    shifts from earlier steps. The gate confirms the committed file matches the code exactly
