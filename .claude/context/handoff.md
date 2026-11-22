## 2026-11-22 — `alg_minhash_256` returns an error for an empty feature list (Rust core API break)

**Done:** The core `alg_minhash_256` now returns `IsccResult<Vec<u8>>` and rejects `[]` with
`InvalidInput("features must not be empty")`. The 3 internal callers (`soft_hash_text_v0`,
`gen_data_code_v0`, `DataHasher::finalize`) use the `pub(crate)` unchecked `alg_minhash_256_inner`.
All 7 binding crates map the error the way their `alg_simhash` does. **API-BREAK:** approved in
decisions.md (2026-11-22). No other signature changed.

**Files changed** (core commit 0d2a6544, then one commit per binding up to 6037d8c9):

- `crates/iscc-lib/src/minhash.rs`: checked public fn plus `_inner`. The empty test now asserts
    `Err(InvalidInput)` and the exact message; a `matches_inner` test was added. The private
    `minhash` `unwrap_or(MAXH)` and its unit test are unchanged
- `crates/iscc-lib/src/{lib.rs,streaming.rs}`: 3 call sites switched to `_inner`
- `crates/iscc-lib/tests/test_algorithm_primitives.rs` (empty → error, `.unwrap()`s; the crate-root
    import check now calls `&[1]`), `benches/iai_benches.rs` (`.unwrap()` inside `black_box`)
- Bindings: `iscc-py` (`PyResult`, `PyValueError`), `iscc-napi` (`from_reason`), `iscc-wasm`
    (`JsError`), `iscc-ffi` (`match` + `set_last_error`; `///` reworded), `iscc-jni`
    (`throw_and_default`), `iscc-rb` (`to_magnus_err`), `iscc-uniffi` (`Result<_, IsccUniError>`,
    plus a new empty-list unit test). Each wrapper's doc comment names the error
- Regenerated: `iscc.h` + `NativeMethods.g.cs` (together, because the `///` changed); UniFFI
    `iscc_uniffi.kt` and `iscc_uniffi.swift` (Swift `algMinhash256` is now `throws`).
    `iscc_uniffiFFI.h` matches HEAD once whitespace is normalized. `.crap-baseline.json` regenerated
- Tests: `crates/iscc-ffi/src/tests.rs` (`(NULL,0)` → core error, not "must not be NULL"),
    `IsccLibTest.java` (empty → `IllegalArgumentException` with the message),
    `packages/cpp/tests/test_iscc.cpp` 42b (`assert_throws_with(…, "features must not be empty")`),
    `tests/test_algo.py` (`pytest.raises(ValueError, match=…)`). `test_iscc.c` needed no edit: its
    `CHECK_NULL_EMPTY_BUF` row still passes (both calls return NULL, no NULL rejection)
- Docs: `_lowlevel.pyi` (`:raises ValueError:`), the `algMinhash256` Javadoc, `docs/rust-api.md`
    (signature + example with `.unwrap()` and `is_err()`), and the minhash line in
    `howto/{rust,   python,nodejs,wasm,java,kotlin,swift,ruby,dotnet}.md`, `c-ffi-api.md`,
    `java-api.md` and `ruby-api.md`. `specs/rust-core.md`: table row and "Verified when" bullet

**Verification:**

- [x] `cargo test -p iscc-lib`: default, `--no-default-features` and `--features text-processing`
    all pass. `cargo test -p iscc-ffi -p iscc-uniffi`: 121 + 31 passed.
    `cargo clippy --workspace --all-targets -- -D warnings` is clean
- [x] WASM `wasm-pack test --node … --features conformance`: 11 + 8 + 136 passed. napi in a `/tmp`
    copy of the tracked tree: `npm ci && npx napi build --platform && npm test` gave 243 pass and 0
    fail, and `alg_minhash_256([])` throws `invalid input: features must not be empty`. JNI
    `mvn clean test`: exit 0, 117 + 8 tests and 0 failures (`algMinhash256EmptyFeatures` passed).
    Python (after `maturin develop`): 924 passed. Ruby `rake compile test`: 175 runs, 0 failures,
    and `[]` raises `RuntimeError: invalid input: features must not be empty`. .NET: 155 passed,
    with `LD_LIBRARY_PATH=target/debug` as in CI. C test: 150 passed. C++ test (built with `g++`):
    121 passed. Kotlin `./gradlew test`: exit 0
- [x] After `cargo build -p iscc-ffi -p iscc-uniffi`, the ci.yml freshness step body (extracted with
    PyYAML, run with `bash -e`) exits 0, and
    `git diff --exit-code crates/iscc-ffi/include/iscc.h packages/dotnet/Iscc.Lib/NativeMethods.g.cs`
    exits 0
- [x] The `xff` grep finds only simhash tests: ffi tests.rs 701-732, Java 1158 (the `algSimhash`
    section), primitives 108-109, cpp 473, and test_algo.py 26/53. No hit tests `alg_minhash_256`
- [x] On a fresh `mise run coverage`,
    `cargo crap … --baseline .crap-baseline.json --fail-regression --fail-above` reports 0
    regressed. Against the HEAD~ baseline, `alg_minhash_256` scored `+1.0` (cc 1→2), as expected.
    `mise run bench:iai:check` exits 0: `bench_minhash_256` +0.00%, every gen row unchanged or lower
- [x] `mise run check` exits 0, and `uv run pytest tests/test_rust_api_signatures.py` gives 5 passed

**Next:** the Go port (`AlgMinhash256` → `([]byte, error)`, same message), then the
`alg_minhash_256` section in `data-edge.json` as a 12-surface fan-out. That fan-out pins the
empty-list case on napi, WASM, Ruby, .NET, Swift and Kotlin, which have no direct test yet.

**Notes:**

- The regenerated CRAP baseline also records 3 `tests/test_data_edge.rs` helpers (`u8_arg`,
    `header_args`, `call_encode_component`) that the committed baseline lacked, and moves line
    numbers that earlier steps shifted. These were NEW and moved rows, not regressions
- The UniFFI generator leaves trailing whitespace and writes a stray `iscc_uniffiFFI.modulemap` into
    `Sources/IsccLib/`. I deleted the modulemap, and prek normalized the rest (the Kotlin file
    needed two `check` passes). The committed outputs differ from HEAD only in the minhash function
    and its checksum
- The Swift `throws` and the Kotlin `@Throws` on `algMinhash256` break source compatibility for
    Swift and Kotlin callers. No call site in this repo's Swift or Kotlin tests or harnesses uses it
