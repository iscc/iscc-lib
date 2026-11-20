# Next Work Package

## Step: `alg_simhash` rejects an empty list in the Rust core and the Go port

## Goal

This picks up the first half of the issue "`alg_simhash` accepts an empty list; C++ list wrappers
pass NULL for an empty vector" (decisions.md 2026-11-20). The public `alg_simhash` in the Rust core
and `AlgSimhash` in Go return an error for `[]`. Generator output stays the same, and every test or
doc comment that pins the old 32-zero-byte answer flips in this same step, so the tree stays green.

## Alternatives Considered

- **Chosen:** the `alg_simhash` core + Go change. It is the only open behaviour change the human has
    decided, it has to land first and alone, and two later steps depend on it (the data-edge vector,
    then the C++ wrappers). Iterations 221-224 were all docs or tooling, so this is the user-facing
    pick.
- **Rejected:** the gate-leniency issue in `scripts/check_spec_paths.py` (the handoff's "Next"). It
    is small and tooling-only, nothing depends on it except a later derived-facts reuse, and it can
    follow this step.

## Scope

The behaviour change lives in 6 files: `simhash.rs`, four Go files, and `iscc-ffi/src/lib.rs` (its
doc comment and in-file test). napi, WASM, Python, JNI, .NET, Ruby and UniFFI pass the core error
through with no source change. **This is not the fan-out**: their only edits are the tests that
pinned the old value, plus comment-only doc fixes.

- **Modify (behaviour):** `crates/iscc-lib/src/simhash.rs`; `packages/go/simhash.go`,
    `packages/go/code_content_audio.go`, `packages/go/code_content_mixed.go`,
    `packages/go/code_meta.go`
- **Modify (comment-only):** `crates/iscc-ffi/src/lib.rs:1181`, `crates/iscc-napi/src/lib.rs:384`,
    `crates/iscc-py/src/lib.rs:561`, `crates/iscc-py/python/iscc_lib/_lowlevel.pyi:284`,
    `crates/iscc-wasm/src/lib.rs:435`, the `alg_simhash` row in `.claude/context/specs/rust-core.md`
    (Tier 1 table, about line 90)
- **Regenerate (tool output):** `crates/iscc-ffi/include/iscc.h`,
    `packages/dotnet/Iscc.Lib/NativeMethods.g.cs`, `.crap-baseline.json`
- **Tests to flip to a rejection:** `simhash.rs` `test_alg_simhash_empty` (about line 181);
    `crates/iscc-lib/tests/test_algorithm_primitives.rs:72`, plus line 294
    (`test_flat_crate_root_imports` unwraps an empty call, so give it one digest);
    `crates/iscc-ffi/src/lib.rs` `test_alg_simhash_empty` (about line 2282);
    `packages/go/simhash_test.go` `TestSimhashEmpty`; `tests/test_algo.py:10`;
    `crates/iscc-napi/__tests__/functions.test.mjs:194`; `crates/iscc-wasm/tests/unit.rs:392`;
    `crates/iscc-jni/java/src/test/java/io/iscc/iscc_lib/IsccLibTest.java:806-809`;
    `packages/dotnet/Iscc.Lib.Tests/ArrayInputTests.cs:63`, `SmokeTests.cs:336`
- **Reference:** `crates/iscc-lib/src/lib.rs` (generators call `alg_simhash_inner`, the pattern Go
    copies); decisions.md 2026-11-20 "`alg_simhash` rejects an empty list"

## Not In Scope

- The `alg_simhash` vector in `data-edge.json`. The file has no `alg_simhash` section yet, and a new
    section fails every harness without a handler, so it is the next step (fan-out).
- The five `packages/cpp/include/iscc/iscc.hpp` wrappers that pass `ptrs.data()` (the second half of
    the issue).
- New empty-list tests on Ruby, Swift, Kotlin, C or C++. None of them pins the old value today, and
    the data-edge vector will cover them.
- Changing `alg_simhash_inner`. Its internal contract (32 zeros for `[]`) stays, because the
    generators rely on it.

## Implementation Notes

- **Rust:** check for an empty list at the top of `alg_simhash` and return `IsccError::InvalidInput`
    with a message that names the empty list (e.g. "hash_digests must not be empty"). Leave
    `alg_simhash_inner` alone and update the public doc comment.
- **Go:** split the function. An unexported `algSimhash(hashDigests [][]byte) []byte` keeps the
    current body, including 32 zero bytes for empty input. The public `AlgSimhash` validates (empty
    → error, then the length check) and calls it. Move **all** eight `AlgSimhash` call sites in the
    three generator files to `algSimhash` and drop their discarded `_` errors. Don't leave two forms
    in place. The `gen_audio_code_v0` vector `test_0000_empty_64` (empty input) in data.json already
    guards the unchanged generator output.
- **FFI:** keep the `num_digests == 0` branch in `iscc_alg_simhash`. It now forwards the core's
    error, which is the intended route. The in-file test must assert `data` is NULL and that
    `iscc_last_error` holds the core message, not "must not be NULL". Regenerate `iscc.h` with the
    `ci.yml` command
    (`cbindgen --config crates/iscc-ffi/cbindgen.toml --crate iscc-ffi --output crates/iscc-ffi/include/iscc.h`),
    then `cargo build -p iscc-ffi` to rewrite `NativeMethods.g.cs`, and commit both.
- **Flipped tests:** assert the rejection on the raw call alone (learnings: one bad case per call,
    no conversions inside `raises`). Python `ValueError`, napi/WASM throw/`Err`, Java
    `IllegalArgumentException`, .NET `IsccException`. Keep the intent of `ArrayInputTests.cs:63`
    (the empty list reaches the core): assert the message is not the NULL-pointer one.
- **Gates:** the new branch raises `alg_simhash`'s CRAP score (cc 4 → 5). The regression gate runs
    only in CI, so run `mise run crap:baseline` (it depends on `coverage`) and commit
    `.crap-baseline.json`. iai is not affected: no bench calls `alg_simhash`, and
    `alg_simhash_inner` is unchanged.

## Verification

- `cargo test -p iscc-lib -p iscc-ffi` passes, and
    `cargo clippy --workspace --exclude iscc-rb -- -D warnings` is clean
- `(cd packages/go && go test ./... && go vet ./...)` passes, and
    `grep -rn "AlgSimhash(" packages/go/code_*.go` prints nothing
- `uv run pytest tests/test_algo.py` passes
- The napi suite (`npm test` in `crates/iscc-napi`, after a fresh build) and the WASM suite
    (`wasm-pack test --node crates/iscc-wasm`) pass
- `mvn -f crates/iscc-jni/java/pom.xml clean test` and `dotnet test packages/dotnet` pass
- After running the cbindgen command and `cargo build -p iscc-ffi`, `git status --porcelain` shows
    no change to `iscc.h` or `NativeMethods.g.cs`, and `cargo crap … --fail-regression --fail-above`
    (copy the line from `ci.yml`) exits 0 against the committed baseline

## Done When

`alg_simhash([])` returns an error in the Rust core and in Go, the generator output is unchanged,
and all six verification criteria pass.
