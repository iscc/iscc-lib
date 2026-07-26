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
- Pending: 5 binding surfaces (C FFI — needs a JSON reader or generated table, `tests/test_iscc.c`
    has no text coverage; C++ — no `cmake` in container; dotnet/kotlin/swift — the slice that adds
    tracked vendored copies, MUST register each in `VENDORED_COPIES` of
    `tests/test_vendored_fixtures.py` or the drift gate reds); full-code-space + sequence-class
    differential sweep (spec requirement 4).
- GOTCHA (iter 149): writing `\uXXXX` escape text into the ASCII-escaped fixture via the Edit tool
    decodes it into literal UTF-8 chars. Write fixture JSON with Python
    (`json.dumps(..., ensure_ascii=True, indent=2)` + trailing newline round-trips the file
    byte-identically); verify with `raw.isascii()` + numeric `ord()` checks. Rust `\u{XXXX}` escapes
    via Write tool land fine.
- CRAP trap for `tests/test_unicode_boundary.rs`: baseline has exactly 2 non-`#[test]` fns
    (`boundary_data`, `run_boundary_section`); any new free helper gets pessimistic-0 coverage — put
    new assertions inside `#[test]` fns only.
