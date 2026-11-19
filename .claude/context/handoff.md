## 2026-11-19 — Review of: Bind `encode_seq` / `decode_seq` in the Ruby bindings (+ locale-free conformance load)

**Verdict:** PASS_WITH_NOTES

**Summary:** The Ruby gem binds `encode_seq` (Array → binary String) and `decode_seq` (String →
Array of `ISCC:` units), registered unprefixed. The conformance harness reads `data.json` as
explicit UTF-8 and runs both seq sections, which also resolves the `normal` locale issue. ISCC-SEQ
is on 9 of 11 surfaces. The diff follows next.md exactly. Codex found a memory-safety bug in
Magnus's `Vec<T>` conversion, which this binding used before this step too (filed), and two weak
test fixtures (fixed in review).

**Verification:**

- [x] `bundle exec rake compile` + `env -u LANG -u LC_ALL bundle exec rake test`: exit 0, also run
    with `LC_CTYPE` unset. All 8 `TestConformance#test_{en,de}code_seq_*` and every `TestSeq#*` test
    run (138 runs after the review edits)
- [x] `cargo clippy -p iscc-rb -- -D warnings`: exit 0
- [x] `bundle exec standardrb`: exit 0 (also after the review edits)
- [x] `grep` shows `"encode_seq"` / `"decode_seq"` registrations at `lib.rs:573-574`
- [x] `test_conformance.rb` has `encoding: "UTF-8"` and reads both seq sections; `test_seq.rb` has
    the `"\0junk"` negative. They are the only two JSON fixture reads under `crates/iscc-rb`
- [x] `mise run check`: exit 0, tree clean apart from `iterations.jsonl`
- Probes: (1) a `to_str` that runs `arr.clear` + `GC.start` segfaults Ruby in `encode_seq` AND in
    the older `gen_mixed_code_v0`, and the non-mutating control returns normally. (2) iscc-core
    1.5.0 `encode_seq`: `EEAYYNMFG5XOELK2====` raises "Incorrect padding", `GAAAAAAAAA======` →
    `300000000000`

**Issues found:**

- Filed `normal`: Ruby `Vec<T>` parameters segfault when an element's `to_str` mutates the array.
    The cause is Magnus 0.8.2 `RArray::to_vec`, which keeps a borrowed slice while it calls back
    into Ruby. Every `Vec<_>` param is affected (`encode_seq`, `gen_mixed_code_v0`,
    `gen_iscc_code_v0`, …). Fix: take `RArray` + `into_iter()` (dups first), as `alg_simhash`
    already does
- Fixed (test-only): `test_encode_seq_padded_unit` used `ISCC:EEAYYNMFG5XOELK2====`, which even the
    padding-tolerant reference rejects, so it could not see a padding regression. Now it uses
    `ISCC:GAAAAAAAAA======` and has a control `test_encode_seq_padding_control` (`300000000000`)
- Fixed (test-only): the four-unit fixture is selected by name (`test_0001_four_units`), not by JSON
    key order
- Not filed: JNI `encodeSeqRejectsPadding` (`IsccLibTest.java:750`) has the same weak `EEAY…====`
    literal. The core and the Python/Go suites pin padding, so this is low value
- Deleted resolved issue: "Ruby conformance test depends on the process locale"

**Codex review:** Ran two reviews. The HEAD one covered only the handoff/memory fixup and found
nothing. The run on `79844a9` reported P1 (Magnus `Vec<String>` re-entrancy → segfault: reproduced,
pre-existing, filed), P2 (padding fixture does not separate strict from reference behavior: verified
against iscc-core 1.5.0, fixed) and P3 (fixture picked by position: fixed).

**Next:** UniFFI seq (Swift + Kotlin, 3 regenerated artifacts, hard-coded per-section counts), the
last missing surface. Its padding negative must use `ISCC:GAAAAAAAAA======` with control
`ISCC:GAAAAAAAAA` → `300000000000`, not `EEAY…====`. After UniFFI, do the 33→35 doc sweep. For Ruby
it covers `crates/iscc-rb/README.md`, `docs/howto/ruby.md`, and the Key Files table in
`crates/iscc-rb/CLAUDE.md`, which lacks `test_iscc_id.rb` and `test_seq.rb`. The Magnus `Vec<T>` fix
is a self-contained single-file Ruby step and can go before or after.

**Notes:**

- Ruby keeps an embedded NUL through `Vec<String>`, and the core rejects it (like JNI). UniFFI is
    the last surface whose NUL handling has not been probed
- The Magnus bug sits in a safe public Magnus API (`impl TryConvert for Vec<T>`), so it may deserve
    an upstream report to `matsadler/magnus`. That is the human's call; no GitHub issue was filed
- Bindings-only step: no core or hot-path change, so iai and CRAP are not affected, and there is no
    `iscc-lib` API change
