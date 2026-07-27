---
name: unicode-freeze
description: Unicode 16.0.0 freeze rule — U+FFFF sentinel map design, table regen, boundary fixture, pending work
metadata:
  type: project
---

# Unicode 16.0.0 freeze rule (sentinel map, iter 148, RULED)

- `text_clean`/`text_collapse` MAP 16.0-unassigned code points to `UNASSIGNED_SENTINEL` (`U+FFFF`,
    permanent noncharacter: forever `Cn`, `ccc = 0`, no decomposition) inside the fused iterator
    BEFORE normalization. NOT a delete-filter (iter-133 design failed 42/140 sequence cases —
    changes adjacency, unblocks composition/`Final_Sigma`) and NOT a category override (`U+A7F1`
    decomposes to `S` under Unicode 17 tables before the filter sees it).
- The category filters (`is_c_category`/`is_cmp_category`) stay exactly as the reference defines
    them — `U+FFFF` is `Cn`, so they remove the sentinel where the reference removes unassigned code
    points. **Why:** IEP-0003 conformance = output-equivalence with the reference; this preserves
    composition-blocking + `Final_Sigma` context while making output table-version invariant.
    Authority: `specs/rust-core.md` requirement 1; `decisions.md` 2026-07-26.
- Table: `crates/iscc-lib/src/utils/unicode16.rs` (731 ranges, 819,533 cps; regen:
    `uv run --script scripts/gen_unicode16_unassigned.py` — PEP 723, pins `unicodedata2==16.0.0`;
    data output is docstring-independent).
- Boundary fixture (iter 141, sequences iter 149): `crates/iscc-lib/tests/unicode_boundary.json` (7
    `text_clean` + 5 `text_collapse` cases: 4 single code points per section + 4 sequence vectors) +
    `tests/test_unicode_boundary.rs` (`SEQUENCE_VECTORS` const pins
    input/expected/delete-filter-output in source; 2 ungated guards + 2 gated vector tests with
    counts derived `4 + rows`) — propagation source for bindings.
- Six sentinel regression tests live in `utils.rs` `mod tests` (composition-block, jamo, final
    sigma, decomposition leak, normalizer pass-through, literal U+FFFF).
- User-facing page: `docs/unicode.md` (sentinel mechanism + "How much does this matter?" + both
    boundary-vector tables).
- Propagation slice 1 (iter 150): Python `tests/test_unicode_boundary.py` reads the canonical
    fixture by relative path (12 vectors + metadata guard, all green); Go
    `packages/go/unicode_boundary_test.go` embeds the vendored byte-identical copy
    `packages/go/testdata/unicode_boundary.json` (`cp` + `cmp`, never Write) and skips exactly 3
    table-dependent cases via a `"<section>/<case>"`-keyed skip map (U+1FAE9 both fns, U+113C5
    `text_clean` only — the `text_collapse` U+113C5 case PASSES because Go removes the mark as `C`
    where 16.0 removes it as `M`, same output `ab`). All 4 sequence vectors pass in Go (iter-147
    `Final_Sigma` fix). Guard test asserts every skip key names an existing fixture case. GOTCHA:
    `t.Skipf(reason)` with a variable trips go vet's printf check — use `t.Skipf("%s", reason)`.
- Propagation slice 2 (iter 151): WASM `crates/iscc-wasm/tests/unicode_boundary.rs` (`include_str!`
    of the canonical fixture, 3 `#[wasm_bindgen_test]` fns — metadata guard + two section loops,
    ungated by the `conformance` feature) and Ruby `crates/iscc-rb/test/test_unicode_boundary.rb`
    (`define_method` per case, fresh `BOUNDARY_JSON` / `BOUNDARY_DATA` constants — `rake test` loads
    all files in one process, reusing `test_conformance.rb` names warns). All 12 vectors + guard
    green on both, no skips. The Ruby `.so` needs `bundle exec rake compile` first — a stale
    extension fails the U+A7F1 rows only (U+0378 is `Cn` in every Unicode version and never
    discriminates staleness).
- Propagation slice 3 (iter 153): napi `crates/iscc-napi/__tests__/unicode_boundary.test.mjs`
    (canonical-path `readFileSync`, metadata guard + two `describe` loops, snake_case exports) and
    JNI `crates/iscc-jni/java/.../UnicodeBoundaryTest.java` (gson `@BeforeAll` load of
    `../../iscc-lib/tests/unicode_boundary.json`, 1 `@Test` guard + 2 `@TestFactory` sections,
    camelCase `IsccLib.textClean`/`textCollapse`). All 12 vectors + guard green, no skips. Local
    native artifacts go stale silently — rebuild first (`npx napi build --platform` from
    crates/iscc-napi; `cargo build -p iscc-jni`) and probe freshness with
    `text_clean("a"+U+A7F1+"b") == "ab"` (a stale build returns `aSb`; U+0378 never discriminates).
    `mvn -o -B test` works fully offline against cached `~/.m2`.
- Propagation slice 4 (iter 154): C# `packages/dotnet/Iscc.Lib.Tests/UnicodeBoundaryTests.cs`
    (`Lazy<JsonElement>` + `[Fact]` guard + 2 `[Theory]`/`[MemberData]`; fixture reaches the test
    output dir via a csproj
    `<Content Include="..\..\..\crates\iscc-lib\tests\unicode_boundary.json"   Link="testdata\unicode_boundary.json">`
    item — NO tracked copy, `bin/` gitignored) and Kotlin
    `packages/kotlin/src/test/.../UnicodeBoundaryTest.kt` (gson lazy load from
    `System.getProperty("iscc.fixtureDir")`, set via one `systemProperty(...)` line in
    `build.gradle.kts`'s `tasks.withType<Test>` block — also NO tracked copy). Both 13/13 green.
    Neither surface needed `VENDORED_COPIES` changes. Kotlin GOTCHA: a plain `@Test` loop shows only
    3 tests in the Gradle XML — use `@TestFactory`/`DynamicTest` so each vector is a `testcase` and
    `tests="13"` is checkable; run `./gradlew cleanTest test --offline` locally (bare `test` can be
    UP-TO-DATE and not execute).
- Case-property freeze (iter 156): `text_collapse` lowercases via `to_lowercase_unicode16` in
    `utils.rs` — decides `Final_Sigma` from vendored `Cased`/`Case_Ignorable` tables in
    `utils/unicode16_case.rs` (152 ranges/4,311 cps + 452 ranges/2,749 cps; regen:
    `uv run --script scripts/gen_unicode16_case.py`, PEP 723 `requires-python = "==3.14.*"`, derives
    both properties behaviourally from `str.lower()` sigma probes, asserts
    `unidata_version == "16.0.0"`), pre-substitutes each `Σ` (ς iff prev non-ignorable is cased and
    next non-ignorable is not), THEN delegates to std `to_lowercase()` so the compiler's 17.0-table
    `Final_Sigma` branch can never fire. **Why:** rustc 1.97 ships Unicode 17.0, which reclassified
    `U+0295` `Ll`→`Lo`, so bare `str::to_lowercase()` makes output a function of the rustc version
    (was the single sweep divergence: 3 comparisons, 1 code point). `CASED_RANGES` = `Cased` minus
    `Case_Ignorable` (unobservable — scan skips ignorables first). Table placement: lowercase runs
    AFTER sentinel map + NFD, so context is scanned on the NFD'd string. Sweep evidence: 17,793,024
    comparisons (8 contexts × 2 fns × 1,112,064 scalars) — 3 divergences before, **0 after**.
- Sweep gate (iter 157, hardened iter 158, spec criterion 4 CLOSED): `scripts/unicode_sweep.py` —
    permanent fail-closed differential gate, oracle = installed `iscc_core` on CPython 3.14 (uniform
    16.0.0 tables). 1,112,064 scalars × 8 `CONTEXTS` × 2 `FUNCTION_PAIRS` = 17,793,024 comparisons,
    ~60 s with a --release extension. Guards: `check_rebuilt` (`--rebuilt` caller assertion — a bare
    `uv run scripts/unicode_sweep.py` fails closed; only the mise task and CI job pass the flag,
    right after their unconditional release build), `check_oracle` (unidata must be 16.0.0),
    `check_extension_fresh` (`.so` mtime vs newest `*.rs` under iscc-lib/src + iscc-py/src; empty or
    missing source dirs also fail closed), scalar/comparison count asserts BEFORE the success line
    `TOTAL 17793024 comparisons, 0 divergences` (byte-frozen format — do not change). `sweep()`
    returns `SweepResult(comparisons, divergences, samples)` — counts every divergence, retains at
    most `MAX_REPORTED_DIVERGENCES` (20) samples so a broad regression can't OOM CI. Run via
    `mise run unicode:sweep` (rebuilds first) or CI job `unicode-sweep` (21st job, CPython 3.14 +
    `--release`, standalone because the 3.10 matrix leg could only skip). Tests
    `tests/test_unicode_sweep.py` (14) load it via importlib; `sweep()` reads module globals at call
    time so the oracle is monkeypatchable.
- Propagation slice 5 (iter 159): C FFI — `scripts/gen_ffi_boundary_vectors.py` (PEP 723, stdlib
    only) renders the canonical fixture into `crates/iscc-ffi/tests/unicode_boundary_vectors.h`
    (pure-ASCII C header; non-ASCII UTF-8 bytes as **3-digit octal escapes** — NEVER `\x`, C hex
    escapes are greedy/unbounded). `test_iscc.c` includes it via quoted `#include` (sibling dir, no
    new `-I` in the CI gcc line), runs 3 metadata guards + 12 vectors → 80 passed. Drift gate
    `tests/test_gen_ffi_boundary_vectors.py` (6 tests): render == tracked header byte-exact,
    mutation fires, wrong-version/empty-section fail closed. Header is NOT in `VENDORED_COPIES`
    (derived, not byte-identical — the no-op gate is its equivalent). GOTCHA: compare
    `ISCC_UNICODE_DATA_VERSION` through a `const char *` local so `-Waddress` never sees a literal
    vs NULL.
- Pending: 2 binding surfaces (C++ — no `cmake` in container; Swift — no `swift` toolchain in
    container, would add a tracked vendored copy that MUST be registered in `VENDORED_COPIES` of
    `tests/test_vendored_fixtures.py`).
- GOTCHA (iter 149): writing `\uXXXX` escape text into the ASCII-escaped fixture via the Edit tool
    decodes it into literal UTF-8 chars. Write fixture JSON with Python
    (`json.dumps(..., ensure_ascii=True, indent=2)` + trailing newline round-trips the file
    byte-identically); verify with `raw.isascii()` + numeric `ord()` checks. Rust `\u{XXXX}` escapes
    via Write tool land fine.
- CRAP trap for `tests/test_unicode_boundary.rs`: baseline has exactly 2 non-`#[test]` fns
    (`boundary_data`, `run_boundary_section`); any new free helper gets pessimistic-0 coverage — put
    new assertions inside `#[test]` fns only.
