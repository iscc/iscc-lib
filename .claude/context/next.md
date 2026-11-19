# Next Work Package

## Step: Bind `encode_seq` / `decode_seq` in the Java JNI bindings

## Goal

Expose ISCC-SEQ as `IsccLib.encodeSeq(String[]) → byte[]` and `IsccLib.decodeSeq(byte[]) → String[]`
in `crates/iscc-jni`, with the Java suite running both data.json seq sections. This brings ISCC-SEQ
to 8 of 11 surfaces (v0.8.0 release criterion).

## Alternatives Considered

- **Chosen:** JNI seq. It is the next unbound surface in the fan-out the handoff planned, uses 2
    source files, and JNI binds the core directly, so it also pins how a non-C-FFI surface treats an
    embedded NUL.
- **Rejected:** a small `normal` issue (Python `encode_seq` docstring, wasm npm metadata). Each is a
    one-file fix, but finishing the ISCC-SEQ feature set across the remaining 4 surfaces comes
    first; these fit after the fan-out, next to the 33→35 doc sweep.

## Scope

- **Modify**: `crates/iscc-jni/src/lib.rs` (two `extern "system"` exports),
    `crates/iscc-jni/java/src/main/java/io/iscc/iscc_lib/IsccLib.java` (two `native` declarations
    with Javadoc, in the Codec section after `isccDecompose`)
- **Tests**: `crates/iscc-jni/java/src/test/java/io/iscc/iscc_lib/IsccLibTest.java`
- **Reference**: `crates/iscc-jni/CLAUDE.md` (type map), `crates/iscc-lib/src/codec.rs:818`
    (`encode_seq(units: &[&str])`) and `:857` (`decode_seq(data: &[u8]) -> Vec<String>`),
    `.claude/context/specs/rust-core.md` → "encode_seq / decode_seq (ISCC-SEQ, IEP-0020)"

## Not In Scope

- `crates/iscc-jni/README.md`, `docs/howto/java.md`, `crates/iscc-jni/CLAUDE.md` and every 33→35
    count. These belong to the single doc sweep after all 11 surfaces bind seq
- Null-array / null-element handling. No existing JNI method tests null input, so don't add a policy
    here. Also leave out the `low` JNI streaming-hasher null-handle issue
- Ruby, Swift or Kotlin seq, and the .NET/C++ embedded-NUL design issue
- Refactoring `extract_string_array` / `build_string_array` (reuse them as they are)

## Implementation Notes

- Templates: `genMixedCodeV0` (L311) for `String[]` input via `extract_string_array` plus the
    `Vec<&str>` refs, `isccDecompose` (L798) for `String[]` output via `build_string_array`, and
    `algMinhash256` (L907) for a `JByteArray` return via `env.byte_array_from_slice`. Use the
    crate's `match` → `throw_and_default` style throughout, not `?`, so core errors reach Java as
    `IllegalArgumentException` carrying the core message
- Javadoc in `IsccLib.java` should say that `encodeSeq` accepts units with or without the `ISCC:`
    prefix in any case, and that base32 decoding is strict (`=` padding, non-zero spare bits and
    non-ASCII case-fold aliases are rejected). Also say that `decodeSeq` returns `ISCC:`-prefixed
    units
- data.json shape: `encode_seq` inputs are `[[unit, …]]` (one argument, a list) and outputs are
    `"bytes:<hex>"`. `decode_seq` inputs are `["bytes:<hex>"]` and outputs are a list of
    `ISCC:`-prefixed strings. 4 cases each, no error cases. The existing hex helper strips only
    `stream:`, so add a `bytes:` decoder (or generalise the helper)
- Tests to add (one bad input per `assertThrows`):
    - `@TestFactory` over `encode_seq` (assert `assertArrayEquals` on bytes) and over `decode_seq`
        (assert the unit list)
    - Byte round trip `encodeSeq(decodeSeq(x))` equals `x` over the `decode_seq` vectors (the spec's
        round trip is BYTES, not strings)
    - `decodeSeq` result `getClass().getName()` is `[Ljava.lang.String;` (the JVM type-checks no
        native return; see the `isccDecompose` test)
    - Negatives: `encodeSeq(new String[0])` throws with a message containing "at least one ISCC-UNIT";
        `unit + "\0junk"` throws (assert only the throw, not the message — NUL handling on JNI is
        unprobed; jni-rs reads modified UTF-8, so the NUL should survive to the core and be rejected
        there); a `ſ` case-fold alias unit (e.g. `gaawdlqng6ſsdpf2`) throws; an `=`-padded unit
        throws; `decodeSeq(new byte[0])` throws; a truncated `decode_seq` vector (drop the last byte)
        throws
- Rebuild the native lib before every `mvn` run. The `.so` is gitignored and goes stale silently.
    Optional JNI-safety probe:
    `mvn -f crates/iscc-jni/java/pom.xml test -DargLine="-Xcheck:jni -Djava.library.path=$PWD/target/debug"`
- Binding-only step: no core or hot-path change, so iai and CRAP are unaffected

## Verification

- `cargo build -p iscc-jni && mvn -f crates/iscc-jni/java/pom.xml clean test` exits 0
- `cargo clippy -p iscc-jni -- -D warnings` exits 0
- `IsccLib.java` declares `public static native byte[] encodeSeq(String[] units)` and
    `public static native String[] decodeSeq(byte[] data)`; `src/lib.rs` defines
    `Java_io_iscc_iscc_1lib_IsccLib_encodeSeq` and `Java_io_iscc_iscc_1lib_IsccLib_decodeSeq`
- `IsccLibTest.java` reads both the `encode_seq` and `decode_seq` sections of data.json and contains
    the byte round trip, the empty-array message assertion and the `"\0junk"` negative test
- `mise run check` exits 0

## Done When

All verification criteria pass, and the Java suite runs every data.json seq vector plus the listed
negative cases against the JNI exports.
