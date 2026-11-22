## 2026-11-22 — The first hasher `finalize()` consumes on every surface, whatever rejects the argument

**Done:** WASM, napi, JNI, Python and Ruby hashers now take the inner hasher before any argument
conversion or check, so a rejected `bits` (or `wide`/`add_units`) leaves the hasher finalized. Each
surface got consumption tests (binding-rejected and core-rejected `bits`, one bad case per fresh
hasher), and the single-use sentence now reads the same in 10 doc pages (wording from `go.md`). One
commit per surface, then a final commit with the shared docs, handoff and memory.

**Files changed:**

- `crates/iscc-wasm/src/lib.rs`: 3 hashers call `opt_u32`/`opt_bool` after `take()`; doc comments
- `crates/iscc-wasm/tests/unit.rs`: replaced `test_rejected_finalize_leaves_hasher_usable` (it
    pinned the old behaviour) with 10 tests: `-1`, `"x"` and `13` per hasher, plus SumHasher `wide`
- `crates/iscc-napi/src/lib.rs`: `bits: Option<Unknown<'_>>` with
    `ts_arg_type = "number | undefined | null"`; new private `opt_number` (type check, then a
    SAFETY-commented `cast::<f64>`) runs after `take()`, then the existing `opt_u32`
- `crates/iscc-napi/__tests__/functions.test.mjs`: replaced the "leaves the hasher usable" test with
    6 (`-1`, `"x"`, `13` × 2 hashers)
- `crates/iscc-jni/src/lib.rs`: order is now zero handle → `take()` → `non_negative(bits)`
- `IsccLibTest.java`: replaced `hasherFinalizeRejectsNegativeBits` (old behaviour) with 4 tests
    (`-1` and `13` × 2 hashers) and an `assertConsumed` helper
- `crates/iscc-py/src/lib.rs`: private `Deferred<'py>` enum (see Notes); 3 hashers convert after
    `take()`; explicit `text_signature` keeps `($self, bits=64[, wide=False, add_units=False])`
- `tests/test_streaming.py`: 13 new tests (3 bad `bits` × 3 hashers, SumHasher `wide`/`add_units`,
    deferred error equals the typed-parameter error for `"x"`/`None`, defaults unchanged)
- `crates/iscc-rb/src/lib.rs`: `_finalize(bits: Value)` + `u32::try_convert` after `take()`
- `crates/iscc-rb/test/test_iscc_lib.rb`: 7 new tests (`-1`, `"x"`, `13` × 2 hashers, `to_int`)
- Docs: `howto/{wasm,nodejs,python,ruby,dotnet,kotlin,swift,c-cpp}.md`, `ruby-api.md`, `java-api.md`
    (warning box + both `*Finalize` notes)

**Verification:**

- WASM: `wasm-pack test --node crates/iscc-wasm --features conformance`: 11 + 8 + 136 passed, the 10
    `*_consumes` tests listed as ok
- napi (git-archive copy in `/tmp`, `npm ci && npx napi build --platform && npm test`): 243 pass, 0
    fail. Built `index.d.ts` diffed against a HEAD~ build: both lines are still
    `finalize(bits?: number | undefined | null): string`; only the JSDoc text changed
- JNI: `cargo build -p iscc-jni && mvn clean test`: IsccLibTest 117 + DataEdgeTest 8, 0 failures
- Python: `maturin develop` then `uv run pytest tests/`: 923 passed; `uv run ty check`: All checks
    passed
- Ruby: `LANG=C.UTF-8 mise exec -- bundle exec rake compile test`: 175 runs, 0 failures (the `.so`
    is newer than `src/lib.rs`)
- `cargo clippy --workspace --all-targets -- -D warnings` exit 0; `mise run check` exit 0
- `grep -il "even when"` over the 5 files lists all 5

**Every surface (12):**

- Fixed here: WASM, napi, JNI, Python, Ruby
- Already compliant, no code change: **Rust core**, where `finalize(self, bits)` moves the hasher.
    **C FFI**: `iscc_*_hasher_finalize` does the null check, then `take()`, then `finalize(bits)`;
    `bits` is `u32`. **C++** wraps the C call with `uint32_t bits`. **.NET** sets
    `_finalized = true` before the FFI call; `bits` is `uint`. **Swift/Kotlin (UniFFI)**:
    `guard.take()` comes before `finalize(bits)`, and `bits` is `u32` (`UInt32`/`UInt`). **Go** sets
    `h.finalized = true` first; `bits` is `uint32`

**Next:** the `alg_minhash_256([])` error (core API break, approved), then `bits` above 256.

**Notes:**

- Python: `Option<…>` with `bits=None` (next.md's alternative) would turn `finalize(None)` from
    `TypeError` into a 64-bit result. `Deferred` keeps every given value, `None` included, and
    converts it after `take()` with the same `extract`, so the types and messages do not change. It
    also re-adds PyO3 0.29's `__notes__` entry `while processing 'bits'`; a test checks that message
    and notes equal those of `gen_data_code_v0`'s typed `bits`
- napi: a non-number now raises `Invalid bits: expected a number`. It used to fail in napi's own
    `Option<f64>` conversion. `-1`/`1.5`/`2**32+64` messages are unchanged
- Ruby: the core error (`13`) is `RuntimeError`, the same class as "already finalized"; the test
    checks both messages
- `docs/howto/wasm.md` is worded "Each is single-use: …" because mdformat put a line break inside
    "even when" in the first wording, and the grep needs the phrase on one line
- napi `CLAUDE.md` type table has no row for `Option<Unknown>` (out of scope; one-line follow-up)
