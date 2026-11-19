## 2026-11-19 — Bind `encode_seq` / `decode_seq` in the Java JNI bindings

**Done:** Added `Java_io_iscc_iscc_1lib_IsccLib_encodeSeq` (`String[]` → `byte[]`) and
`Java_io_iscc_iscc_1lib_IsccLib_decodeSeq` (`byte[]` → `String[]`) to `crates/iscc-jni/src/lib.rs`.
Both reuse `extract_string_array` / `build_string_array` / `byte_array_from_slice` in the crate's
`match` → `throw_and_default` style, so core errors reach Java as `IllegalArgumentException` with
the core message. Matching `native` declarations with Javadoc sit after `isccDecompose` in
`IsccLib.java`. ISCC-SEQ is now on 8 of 11 surfaces.

**Files changed:**

- `crates/iscc-jni/src/lib.rs`: two `extern "system"` exports after `isccDecompose`
- `crates/iscc-jni/java/src/main/java/io/iscc/iscc_lib/IsccLib.java`:
    `public static native byte[] encodeSeq(String[] units)` and
    `public static native String[] decodeSeq(byte[] data)`. The Javadoc covers the optional `ISCC:`
    prefix in any case, strict base32 (padding, spare bits, non-ASCII case-fold aliases) and the
    `ISCC:`-prefixed output
- `crates/iscc-jni/java/src/test/java/io/iscc/iscc_lib/IsccLibTest.java`: a `decodeBytes`
    (`bytes:<hex>`) helper; `@TestFactory` `encodeSeq` and `decodeSeq` over the data.json sections;
    `seqByteRoundtrip` (`encodeSeq(decodeSeq(x)) == x` over the `decode_seq` vectors); a
    `[Ljava.lang.String;` class check; 6 negative tests with one bad input each (an empty array
    checked for the "at least one ISCC-UNIT" message, `unit + "\0junk"`, the `ſ` alias
    `gaawdlqng6ſsdpf2`, `EEAYYNMFG5XOELK2====`, empty bytes, and a truncated `test_0002` vector)

**Verification:**

- `cargo build -p iscc-jni` and `cargo clippy -p iscc-jni -- -D warnings` both finish clean
- `mvn -q -f crates/iscc-jni/java/pom.xml clean test > /tmp/mvn.log; echo $?` gives `0` (unpiped,
    re-run after review of the first piped run). Surefire reports `tests="103"` failures 0, errors
    0, skipped 0. 19 test cases have "seq" in the name: 3×4 dynamic plus 7 static
- `-Xcheck:jni`: surefire run with `-DargLine="-Xcheck:jni -Djava.library.path=$PWD/target/debug"`
    exits 0, and there are no `.dumpstream` files. A direct `java -Xcheck:jni` run of a `/tmp` probe
    (encodeSeq/decodeSeq success path plus both error paths) printed no `WARNING in native method`
- Javadoc's "ISCC-CODE, ID or FLAKE are rejected" matches the `SEQ_MAINTYPES` doc (codec.rs:740).
    Python `encode_seq([<ISCC-CODE>])` fails with "MainType 5 is not permitted in an ISCC-SEQ"
- `mise run format` and then `mise run check` both exit 0
- Checked the negative inputs against the Python binding first: the ASCII control `gaawdlqng6ssdpf2`
    encodes fine (`300161ae0d37a521bcba`), the `ſ` variant fails with "invalid length at 16", and
    `====` fails with "invalid symbol at 16". So each negative test fails for its intended reason,
    not because the base unit is broken

**Next:** Ruby seq (`crates/iscc-rb`), then UniFFI (Swift/Kotlin, which hard-code per-section
conformance counts). Keep asking each one for a `unit + "\0junk"` negative test. After that comes
the 33→35 doc sweep, which must now also cover `crates/iscc-jni/README.md`, `docs/howto/java.md`,
`docs/java-api.md` and the `crates/iscc-jni/CLAUDE.md` test-pattern list (it doesn't mention the
`bytes:` helper or the seq tests).

**Notes:**

- **JNI embedded NUL is now probed:** a `/tmp` Java probe against `target/classes` shows the NUL
    survives the JNI boundary. `encodeSeq({unit + "\0junk"})` fails with
    `invalid input: base32 decode error: invalid symbol at 55` (index 55 = the NUL after the 55-char
    cleaned unit). `genTextCodeV0("Hello\0World", 64)` = `ISCC:EAASKDNZNYGUUF5A` ≠ `"Hello"`'s
    `ISCC:EAAWSPUDF7GJWQVD`. So JNI does NOT share the .NET/C++ truncation defect, and the NUL issue
    can stay scoped to the C-FFI wrappers
- Binding-only step: no core, FFI or hot-path change, so iai and CRAP are unaffected. No null-input
    policy was added (out of scope)
