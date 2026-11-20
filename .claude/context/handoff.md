## 2026-11-20 — `alg_simhash` rejects an empty list in the Rust core and the Go port

**Done:** The public `alg_simhash` (Rust) and `AlgSimhash` (Go) now return an error, "hash_digests
must not be empty", for `[]`. Go is split into the public validating `AlgSimhash` and an unexported
`algSimhash`, which keeps 32 zero bytes for `[]`. All 7 generator call sites now use `algSimhash`.
Every test and doc comment that pinned the old 32-zero answer now expects the rejection. Generator
output is unchanged.

**Files changed:**

- `crates/iscc-lib/src/simhash.rs`: empty check at the top of `alg_simhash` plus doc. Test flipped,
    and a test added that pins `alg_simhash_inner([])` == 32 zeros
- `crates/iscc-lib/tests/test_algorithm_primitives.rs`: empty test now asserts the error;
    `test_flat_crate_root_imports` passes one 32-byte digest
- `crates/iscc-ffi/src/lib.rs`: doc comment for `iscc_alg_simhash`. `test_alg_simhash_empty` asserts
    NULL data, len 0, and that `iscc_last_error` contains "must not be empty" and not "must not be
    NULL"
- `crates/iscc-ffi/include/iscc.h`, `packages/dotnet/Iscc.Lib/NativeMethods.g.cs`: regenerated (doc
    text only)
- `packages/go/simhash.go`: `AlgSimhash` validates, `algSimhash` does the unchecked work.
    `simhash_test.go`: `TestSimhashEmpty` expects the error; `TestAlgSimhashUncheckedEmpty` added
- `packages/go/code_content_audio.go` (3), `code_content_mixed.go` (1), `code_meta.go` (3): moved to
    `algSimhash`, the discarded `_` errors removed
- Comment-only: `crates/iscc-napi/src/lib.rs`, `crates/iscc-py/src/lib.rs`, `_lowlevel.pyi` (adds
    `:raises ValueError:`), `crates/iscc-wasm/src/lib.rs`, `specs/rust-core.md` (row text plus one
    "Verified when" bullet)
- Flipped tests: `tests/test_algo.py` (`pytest.raises(ValueError, match=…)` around the bare call),
    napi `functions.test.mjs` (`throws … /must not be empty/`), WASM `tests/unit.rs` (`is_err`), JNI
    `IsccLibTest.java` (`assertThrows(IllegalArgumentException)` plus a message check), .NET
    `ArrayInputTests.cs` (core message, not `FfiNullMessage`), `SmokeTests.cs`
    (`Throws<IsccException>`)
- `.crap-baseline.json`: regenerated

**Verification:**

- `cargo test -p iscc-lib -p iscc-ffi`: all suites ok, no failures
- `cargo clippy --workspace --exclude iscc-rb -- -D warnings`: clean
- `go vet ./... && go test ./...` (packages/go): ok. `grep -rn "AlgSimhash(" packages/go/code_*.go`
    matches only a comment, `code_content_mixed.go:92`, which now says "so the unchecked algSimhash
    is safe". No call sites remain
- `uv run pytest tests/test_algo.py`: 18 passed (after `maturin develop`)
- napi `npm run build && npm test`: 179/179 pass. `wasm-pack test --node crates/iscc-wasm`: 11 + 5 +
    93 pass
- `cargo build -p iscc-jni && mvn -f crates/iscc-jni/java/pom.xml clean test`: 108 run, 0 failures,
    BUILD SUCCESS
- `dotnet test packages/dotnet/Iscc.Lib.Tests/ -e LD_LIBRARY_PATH=$PWD/target/debug`: 150 passed.
    The next.md path `packages/dotnet` has no project file; I used the ci.yml form
- I re-ran cbindgen and `cargo build -p iscc-ffi` after the regen: md5 of `iscc.h` and
    `NativeMethods.g.cs` did not change (no-op)
- CRAP: before the rebaseline, the CI gate line exited 1 with exactly one regression, `alg_simhash`
    5.0 (+1.0). After `cargo crap … --format json --output .crap-baseline.json` the gate
    (`--baseline .crap-baseline.json --fail-regression --fail-above`) exits 0: 0 regressed, 0 new,
    145 unchanged
- `mise run check`: exit 0 (mdformat realigned the spec table on the first run).
    `prek run --hook-stage pre-push --files <changed>`: all Passed (Rust lint/tests, ty, ruff
    S/C901, pytest)

**Next:** The `alg_simhash` section in `data-edge.json` (fan-out over every harness), then the five
C++ `ptrs.data()` wrappers. After that, the gate-leniency issue in `scripts/check_spec_paths.py`.

**Notes:**

- next.md said "eight" generator call sites. There were seven (audio 3, mixed 1, meta 3). The grep
    criterion holds
- The baseline diff is large (+220/−12) because the old baseline lacked 26 functions added in
    earlier iterations, and 4 had moved lines. Only `alg_simhash` regressed. New functions face only
    `--fail-above`, so baking them in changes no gate outcome
- WASM's flipped test asserts only `is_err()`, matching the file's existing rejection tests (I did
    not check the message there)
- `cargo-crap` 0.2.2 and `cargo-llvm-cov` 0.9.1 were installed from source into `~/.cargo/bin` (not
    preinstalled)
