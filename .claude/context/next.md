# Next Work Package

## Step: `alg_minhash_256` returns an error for an empty feature list (Rust core API break)

## Goal

Picks up the `normal` issue "`alg_minhash_256([])` returns 32 bytes of `0xFF` instead of an error"
for the Rust core and every binding built on it. **API-BREAK:** the core signature becomes
`fn alg_minhash_256(features: &[u32]) -> IsccResult<Vec<u8>>`, as decisions.md (2026-11-22,
"`alg_minhash_256` rejects an empty feature list") approves.

## Alternatives Considered

- **Chosen:** Rust core change plus the binding edits it forces. The other surfaces cannot land
    before it, and the binding crates stop compiling the moment the core signature changes.
- **Rejected:** the `bits` above 256 issue first. It changes every gen function, every hasher and
    the codec on 12 surfaces, so it is much larger. The minhash break is small, and the handoff puts
    it first.
- **Rejected:** Go port in the same step. Go is a second core implementation (`AlgMinhash256`
    becomes `([]byte, error)`), and it is its own next step. After that comes the shared
    `alg_minhash_256` section in `data-edge.json`, one fan-out over all 12 surfaces.

## Scope

The core change cannot land on its own, because the workspace does not compile until every binding
handles the `Result`. Each binding edit maps the error exactly as that crate's `alg_simhash` already
does, so no surface needs a decision that is still open. Behaviour files: `minhash.rs` + 7 binding
`lib.rs` (one call each). The internal callers in `lib.rs`/`streaming.rs` switch to an unchecked
helper, a call swap that leaves behaviour unchanged. .NET (`ConsumeByteBuffer`) and C++ (null check
and `check_error`) already raise FFI errors; Swift and Kotlin take the regenerated UniFFI output.
Commit the core first, then one commit per binding.

- **Modify**: `crates/iscc-lib/src/minhash.rs`, `crates/iscc-lib/src/lib.rs`,
    `crates/iscc-lib/src/streaming.rs`, `crates/iscc-py/src/lib.rs`, `crates/iscc-napi/src/lib.rs`,
    `crates/iscc-wasm/src/lib.rs`, `crates/iscc-ffi/src/lib.rs`, `crates/iscc-jni/src/lib.rs`,
    `crates/iscc-rb/src/lib.rs`, `crates/iscc-uniffi/src/lib.rs`
- **Regenerate**: `crates/iscc-ffi/include/iscc.h` + `packages/dotnet/Iscc.Lib/NativeMethods.g.cs`
    (only if a `///` on the export changes, and then both together); the 3 UniFFI outputs
    (`iscc_uniffi.kt`, `iscc_uniffi.swift`, `iscc_uniffiFFI.h`); `.crap-baseline.json`
- **Docs/comments**: `crates/iscc-py/python/iscc_lib/_lowlevel.pyi` (`:raises ValueError:`), the
    `algMinhash256` Javadoc in `IsccLib.java`, `docs/rust-api.md` (signature + example),
    `docs/howto/rust.md`, and the `alg_minhash_256` entry on every other API/how-to page that lists
    it (`git grep -n -i "minhash256\|minhash_256" docs`, except `docs/howto/go.md`);
    `.claude/context/specs/rust-core.md` rows at ~88 and ~102
- **Tests** (outside the budget): `crates/iscc-lib/src/minhash.rs` tests,
    `crates/iscc-lib/tests/test_algorithm_primitives.rs` (~146 and ~299),
    `crates/iscc-ffi/src/tests.rs` (~784), `crates/iscc-ffi/tests/test_iscc.c` (~891, check it still
    holds), `IsccLibTest.java` (~1181), `packages/cpp/tests/test_iscc.cpp` (~702),
    `tests/test_algo.py` (~72), `crates/iscc-uniffi/src/lib.rs` tests
- **Reference**: `crates/iscc-lib/src/simhash.rs` (`alg_simhash` / `alg_simhash_inner`), each
    binding's `alg_simhash` wrapper, `reference/iscc-core/iscc_core/minhash.py:14-16`

## Not In Scope

- Go (`packages/go/minhash.go`, `minhash_test.go`, `docs/howto/go.md`, `specs/go-bindings.md`): the
    next step
- The `alg_minhash_256` section in `data-edge.json` and its 12 harness handlers (Java's `Map.of` →
    `Map.ofEntries`): the fan-out after Go
- New empty-list tests on surfaces that have none today (napi, WASM, Ruby, .NET, Swift, Kotlin): the
    data-edge section pins them. Only rewrite the tests that pin `0xFF` (and the ones that no longer
    compile)
- The `bits` above 256 issue, and the `low` finalize-test issue

## Implementation Notes

- Core: mirror simhash. The public `alg_minhash_256` returns
    `Err(IsccError::InvalidInput("features must not be empty".into()))` for `[]`, otherwise
    `Ok(<unchecked>)`. Add a `pub(crate)` unchecked helper (for example `alg_minhash_256_inner`) and
    switch the 3 internal callers (`soft_hash_text_v0`, `gen_data_code_v0`, `DataHasher::finalize`)
    to it. They always pass at least one feature, so no gen output and no iai gen row moves. Keep or
    drop the private `minhash` `unwrap_or(MAXH)` fallback, as you like; its own unit test may stay.
- Use the exact message `features must not be empty` on all surfaces. The data-edge vectors and the
    C++ test (`assert_throws_with(..., "must not be empty", ...)`) will match on it.
- FFI: `match` like `iscc_alg_simhash`; `(NULL, 0)` goes through `slice_or_empty`, so it reaches the
    empty-list error, not a NULL rejection. The C `CHECK_NULL_EMPTY_BUF` row and C++ 42b must still
    show "no NULL rejection".
- JNI: `Err(e) => throw_and_default(env, &e)`; Ruby `map_err(to_magnus_err)`; napi
    `napi::Error::from_reason(e.to_string())`; WASM `JsError::new`; UniFFI `?`; PyO3 `PyValueError`.
    The Python return type becomes `PyResult<Vec<u8>>`.
- Regenerate the UniFFI outputs with the commands in the `kotlin` job of `.github/workflows/ci.yml`
    ("Check UniFFI bindings freshness"). The generated Swift `algMinhash256` becomes `throws`.
- CRAP: the new branch raises `alg_minhash_256` from cc 1 to 2, which the CI-only
    `--fail-regression` flags. Run `mise run coverage && mise run crap:baseline` in this step.
- napi builds fail in the 9p checkout; build and test in a `git archive HEAD` copy under `/tmp`
    (learnings).
- `iai_benches.rs` `bench_minhash_256` wraps a `Result` now; `.unwrap()` it inside `black_box`.

## Verification

- `cargo test -p iscc-lib` (also `--no-default-features` and `--features text-processing`),
    `cargo test -p iscc-ffi -p iscc-uniffi` and
    `cargo clippy --workspace --all-targets -- -D warnings` pass; a core test asserts
    `alg_minhash_256(&[])` is `Err(IsccError::InvalidInput(_))` with the message above
- Binding suites pass: `wasm-pack test --node crates/iscc-wasm --features conformance`; napi
    `npm ci && npx napi build --platform && npm test` in a `/tmp` archive copy;
    `cargo build -p iscc-jni && mvn -f crates/iscc-jni/java/pom.xml test`; `maturin develop` +
    `uv run pytest tests/`; `LANG=C.UTF-8 mise exec -- bundle exec rake compile test` in
    `crates/iscc-rb`; `dotnet test packages/dotnet/Iscc.Lib.Tests`; the C test and the C++ test
    program; Kotlin `./gradlew test`
- After `cargo build -p iscc-ffi -p iscc-uniffi`, the UniFFI freshness step body from `ci.yml` exits
    0, and
    `git diff --exit-code crates/iscc-ffi/include/iscc.h packages/dotnet/Iscc.Lib/NativeMethods.g.cs`
    exits 0 once the step's changes are committed
- `git grep -n -i 'xff' -- tests/test_algo.py crates/iscc-lib/tests/test_algorithm_primitives.rs crates/iscc-jni/java/src/test packages/cpp/tests crates/iscc-ffi/src/tests.rs`
    prints no line that tests `alg_minhash_256` (inspect each hit)
- `cargo crap --lcov lcov.info --baseline .crap-baseline.json --fail-regression` exits 0 on a fresh
    `mise run coverage`, and `mise run bench:iai:check` exits 0
- `mise run check` exits 0, and `uv run pytest tests/test_rust_api_signatures.py` passes

## Done When

The advance agent is done when all six verification criteria pass, and the core and the 7 binding
crates report the error with the message `features must not be empty` for an empty feature list.
