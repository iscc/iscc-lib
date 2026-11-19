## 2026-11-19 — Bind `encode_seq` / `decode_seq` in the Ruby bindings (+ locale-free conformance load)

**Done:** Added Magnus `encode_seq(Vec<String>) -> RString` (binary) and
`decode_seq(RString) -> Vec<String>`, registered unprefixed next to `iscc_decode`. Header symbol
list is now 35 of 35. The Ruby conformance harness reads data.json as explicit UTF-8 and runs both
seq sections. `test_seq.rb` adds the byte round trip, type checks and 6 one-input negatives (plus
controls). ISCC-SEQ is on 9 of 11 surfaces.

**Files changed:**

- `crates/iscc-rb/src/lib.rs`: `encode_seq` / `decode_seq` bridge fns (copy-before-call slice,
    errors through `to_magnus_err`), two `define_module_function` registrations (L573-574), header
    33→35
- `crates/iscc-rb/test/test_conformance.rb`: `File.read(DATA_JSON, encoding: "UTF-8")`,
    `decode_bytes` helper (raises unless `bytes:` prefix), `encode_seq` + `decode_seq` loops, header
    comment
- `crates/iscc-rb/test/test_seq.rb` (new): fixture guard (4 vectors, named key), byte round trip
    `encode_seq(decode_seq(x)) == x` over every `decode_seq` vector, `Encoding::BINARY`, Array of
    `ISCC:` Strings; negatives: `[]` ("at least one ISCC-UNIT"), `"\0junk"` suffix, `ſ` alias,
    `====` padding, `"".b` ("Empty ISCC-SEQ"), truncated by one byte ("Truncated ISCC-UNIT")

**Verification:**

- `bundle exec rake compile` exit 0; `env -u LANG -u LC_ALL bundle exec rake test`: 137 runs, 402
    assertions, 0 failures/errors/skips. `TESTOPTS=-v` lists all 8
    `TestConformance#test_{en,de}code_seq_*` and all 12 `TestSeq#*` tests
- `cargo clippy -p iscc-rb -- -D warnings` exit 0; `bundle exec standardrb` exit 0
- `grep` shows `"encode_seq"` / `"decode_seq"` registrations at lib.rs L573/L574
- `mise run format` + `mise run check` exit 0, clean tree afterwards (except runner-owned
    iterations.jsonl)
- Probes. (1) Each negative fails for the intended reason: `"\0junk"` → "invalid symbol at 16", `ſ`
    → "invalid length at 16", `====` → "invalid symbol at 16". Controls are committed tests: the
    ASCII `gaawdlqng6ssdpf2` encodes to 10 bytes, and unpadded `ISCC:EEAYYNMFG5XOELK2` encodes to
    `21018c3585376ee22d5a` (the test_0004 prefix). (2) Locale: the HEAD version of
    `test_conformance.rb`, run with LANG/LC_ALL/LC_CTYPE unset, aborts with
    `Encoding::InvalidByteSequenceError "\xC3" on US-ASCII` in bundled json 2.21.1 (the probe copy
    was deleted). The new file passes in the same environment

**Next:** UniFFI seq (Swift + Kotlin; hard-coded per-section conformance counts), then the 33→35 doc
sweep. For Ruby that sweep covers `crates/iscc-rb/README.md`, `crates/iscc-rb/CLAUDE.md` (Key Files
table lacks `test_iscc_id.rb` and `test_seq.rb`) and `docs/howto/ruby.md`.

**Notes:**

- Deviation: my first unpadded control expected 9 bytes, which was wrong. The 64-bit unit takes a
    2-byte header (MainType 2, SubType 1 → `2101`), so the control now asserts the exact test_0004
    prefix bytes
- Ruby keeps the embedded NUL through `Vec<String>`, so the reject comes from the core, as in JNI
- System ruby + json (no bundler) parses the file fine under US-ASCII. Only the bundled json 2.21.1
    aborts. The locale issue is real for the `bundle exec rake test` path that CI and the gates use
- Bindings-only step: no core or hot-path change, so iai and CRAP are not affected. No API change to
    `iscc-lib`
