# Next Work Package

## Step: The first hasher `finalize()` consumes on every surface, whatever rejects the argument

## Goal

Pick up the `normal` issue "`@iscc/wasm` hasher `finalize()` stays usable after some rejected
arguments" and apply the decision "The first `finalize()` consumes a hasher on every surface,
whatever its outcome" (2026-11-22) to every binding that still checks arguments before it takes the
inner hasher. Probing found that WASM is not the only one: napi and JNI check `bits` first too, and
Python and Ruby convert their arguments before the body runs.

## Alternatives Considered

- **Chosen:** finalize-consumes fan-out. The decision is settled, no core change is needed, each
    surface has its own suite, and it closes one of the three `normal` issues that block v0.8.0.
- **Rejected:** `alg_minhash_256([])` error (`IsccResult` in the Rust core and `([]byte, error)` in
    Go). It is a core API break that every binding crate must follow, so it is a larger step. It
    comes next, alone, and its `data-edge.json` section follows as a fan-out.

## Scope

**Fan-out:** a hasher's first `finalize()` takes the inner hasher before any argument conversion or
check, so every rejected argument leaves it consumed. Applies to 5 surfaces: WASM, napi, JNI,
Python, Ruby.

- **Modify**: `crates/iscc-wasm/src/lib.rs`, `crates/iscc-napi/src/lib.rs`,
    `crates/iscc-jni/src/lib.rs`, `crates/iscc-py/src/lib.rs`, `crates/iscc-rb/src/lib.rs`
- **Modify (tests)**: `crates/iscc-wasm/tests/unit.rs`,
    `crates/iscc-napi/__tests__/functions.test.mjs`,
    `crates/iscc-jni/java/src/test/java/io/iscc/iscc_lib/IsccLibTest.java`,
    `tests/test_streaming.py`, the Ruby hasher tests under `crates/iscc-rb/test/`
- **Modify (docs)**: the hasher single-use sentence in `docs/howto/wasm.md` (line ~353),
    `docs/howto/nodejs.md` (~241), `docs/howto/python.md` (~315), `docs/howto/ruby.md` (~328),
    `docs/ruby-api.md` (~724), and `docs/java-api.md` (the two `IllegalStateException` notes,
    ~707/762). For one wording across the docs, update the same sentence in `dotnet.md`,
    `kotlin.md`, `swift.md` and `c-cpp.md` too. Copy the wording from `docs/howto/go.md:288-289`
- **Reference**: decisions.md (the 2026-11-22 finalize entry; "Ruby integer parameters keep Ruby's
    implicit `to_int` conversion", 2026-11-21); `crates/iscc-wasm/src/lib.rs:690-821` (current
    order)
- Each surface commits on its own (fan-out rule)

## Not In Scope

- `bits` above 256 and the `alg_minhash_256` empty-list error: these are separate `normal` steps. Do
    not add range checks here
- Go, .NET, C FFI, C++, Swift/Kotlin (UniFFI) and the Rust core already comply: their `bits` is
    unsigned by type, and the core rejects only after `take()`. Change no code there, and say why in
    the handoff (this is the "every other surface checked" part of the issue)
- No `data-edge.json` vector: hasher state cannot be expressed in JSON
- Do not change exception classes or messages a caller sees today (Python `OverflowError`/
    `TypeError`/`ValueError`, Ruby `RangeError`/`TypeError`, JNI `IllegalArgumentException`)
- `update()` argument handling is out of scope

## Implementation Notes

- **WASM** (3 hashers): move `opt_u32`/`opt_bool` after `self.inner.take()…?`.
- **napi** (DataHasher, InstanceHasher; napi has no SumHasher): moving `opt_u32` after `take()`
    covers `-1` and `1.5`. A non-number (`"x"`) is still rejected by napi's `Option<f64>` conversion
    before the body. To cover it, take `bits: Option<Unknown>` (napi 3.11,
    `napi::bindgen_prelude::Unknown`, `src/js_values/unknown.rs`) with
    `#[napi(ts_arg_type = "number | undefined | null")]`. After `take()`, read it as a number and
    route it through the existing `to_u32` helper. A non-number gets a `bits must be a number`-style
    error. Treat `undefined` and `null` as the default 64, as today. The generated `index.d.ts` must
    keep `finalize(bits?: number | undefined | null)`, so diff the built file's `finalize` lines
    against the current build.
- **JNI** (2 hashers): order becomes zero-handle check → `take()` (else `throw_state_error`) →
    `non_negative(bits)` → `finalize`. A zero handle still throws before anything else.
- **Python** (3 `_lowlevel` hashers): PyO3 converts `bits: u32` and `wide`/`add_units: bool` before
    the body. Take them as `&Bound<'_, PyAny>`, or as `Option<…>` with `signature = (bits=None, …)`
    mapping `None` to the old defaults, and `.extract::<u32>()`/`.extract::<bool>()` after `take()`.
    `extract` raises the same exception types as today. `_lowlevel.pyi` keeps `bits: int = 64`.
- **Ruby** (2 hashers, `_finalize`): take `bits: Value` and convert with `u32::try_convert` after
    `take()`. That is the conversion Magnus applied before, so `to_int` behaviour is unchanged.
- **Tests, per surface and hasher:** case A, an argument the binding rejects (WASM/napi `-1` and
    `"x"`, JNI `-1`, Python `-1`, Ruby `-1`); case B, a `bits` the core rejects (`13`). After each,
    a valid `finalize(64)` must raise the surface's "already finalized" error. Use a fresh hasher
    for each case, one bad case per hasher (learnings: one bad case per call). For SumHasher in WASM
    and Python, also cover a rejected `wide` (`"x"`).
- Binding-only step: neither CRAP nor iai is touched (no `crates/iscc-lib` change).

## Verification

- WASM: `wasm-pack test --node crates/iscc-wasm --features conformance` passes, including the new
    consumption tests for all 3 hashers
- napi, in a `git archive HEAD` copy under `/tmp` (9p `link()` issue):
    `npm ci && npx napi build --platform && npm test` passes. The built `index.d.ts` still types
    both `finalize` methods as `(bits?: number | undefined | null)`
- JNI: `cargo build -p iscc-jni && mvn -f crates/iscc-jni/java/pom.xml test` passes
- Python: `uv run maturin develop -m crates/iscc-py/Cargo.toml && uv run pytest tests/` passes (new
    cases in `tests/test_streaming.py`). `uv run ty check` is clean
- Ruby: `LANG=C.UTF-8 mise exec -- bundle exec rake compile test` in `crates/iscc-rb` passes
- `cargo clippy --workspace --all-targets -- -D warnings` and `mise run check` exit 0
- `grep -il "even when"` over `docs/howto/{wasm,nodejs,python,ruby}.md` and `docs/java-api.md` lists
    all 5 files

## Done When

All 7 verification criteria pass, and the handoff names each of the 12 surfaces with how it meets
the rule (fixed here, or compliant by type).
