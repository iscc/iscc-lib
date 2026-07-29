# Next Work Package

## Step: Mint `gen_iscc_id_v1` on the JNI (Java) surface

## Goal

Continue the #43 IDv1 fan-out: expose the experimental `gen_iscc_id_v1` minting function on the
Java/JNI surface (7th of 11), bringing `crates/iscc-jni` to 33 usable Tier 1 symbols. This is a live
v0.6.0 release blocker (issue "ISCC-IDv1 is unsupported outside Go").

## Alternatives Considered

- **Chosen:** JNI IDv1 minting — the handoff's explicit next surface and the next typed-int slice;
    unblocks one of the 5 remaining fan-out surfaces with a self-contained, well-precedented change.
- **Rejected:** the Tier-1 32→33 doc/count sweep — deliberately deferred until the fan-out completes
    (issues.md scopes it as its own slice); running it now would churn count text while symbols are
    still landing.

## Scope

- **Modify**: `crates/iscc-jni/src/lib.rs` (add the `genIsccIdV1` JNI wrapper)
- **Modify**: `crates/iscc-jni/java/src/main/java/io/iscc/iscc_lib/IsccLib.java` (native decl)
- **Modify (test, excluded from budget)**:
    `crates/iscc-jni/java/src/test/java/io/iscc/iscc_lib/IsccLibTest.java`
- **Reference**: `crates/iscc-jni/src/lib.rs:200-221` (`genTextCodeV0` String-returning pattern),
    `crates/iscc-jni/CLAUDE.md` (type mapping, `_1` mangling, `throw_and_default`),
    `.claude/context/specs/rust-core.md` §"ISCC-IDv1 Operations" (signature, validation, golden)

## Not In Scope

- The Tier-1 32→33 doc/count sweep across CLAUDE.md/README/docs — separate #43 slice. jni docs carry
    no numeric symbol count, so no doc edit is needed here (mirrors the ffi slice).
- Any enum-widening or decode change: JNI `isccDecode` already returns `version` as a plain `int`,
    so `iscc_decode(gen_iscc_id_v1(...))` round-trips today — do NOT touch `IsccDecodeResult` or the
    decode path.
- The other 4 remaining surfaces (rb, uniffi→Swift/Kotlin, dotnet C# consumer, cpp).
- Adding a "now"/clock convenience — the core is clock-free; `timestamp` stays a required argument.

## Implementation Notes

- Java signature: `public static native String genIsccIdV1(long timestamp, int hubId, int realm);`
    JNI Rust name `Java_io_iscc_iscc_1lib_IsccLib_genIsccIdV1` (note the `_1` for `iscc_lib`).
- Params arrive as `jlong`/`jint`/`jint`. **Narrowing hazard — validate before casting to core's
    `(u64, u16, u8)`:** a `jint` can wrap into the valid range (`65537 as u16 == 1` would masquerade
    as a valid hub). Reject via `throw_and_default` when `hubId` is outside `0..=u16::MAX` or
    `realm` outside `0..=u8::MAX`, THEN cast. `timestamp as u64` needs no guard: a negative `jlong`
    maps to a huge value core rejects, and every valid ts (`< 2^52`) is a positive `long`. Core's
    `gen_iscc_id_v1` re-validates the semantic ranges (ts/hub/realm) in order and returns `Err` →
    `throw_and_default` maps it to `IllegalArgumentException`.
- Body mirrors `genTextCodeV0`: call `iscc_lib::gen_iscc_id_v1(ts, hub, realm)`, on `Ok` return
    `env.new_string(result.iscc)`, on `Err` `throw_and_default(env, &e.to_string())`.
- Tests (JUnit `@Test`): golden
    `assertEquals("ISCC:MAIGHFECJMOPMIAB", IsccLib.genIsccIdV1(   1751831876325218L, 1, 0))`; realm
    error
    `assertThrows(IllegalArgumentException.class,   () -> IsccLib.genIsccIdV1(1751831876325218L, 1, 2))`;
    round-trip decoding the golden via `IsccLib.isccDecode(...)` asserting `version == 1`,
    `subtype == 0` (realm), `maintype == 6`.

## Verification

- `cargo build -p iscc-jni` clean; `cargo clippy -p iscc-jni --all-targets -- -D warnings` clean;
    `cargo fmt -p iscc-jni --check` clean.
- `cd crates/iscc-jni/java && mvn clean test` passes (run `cargo build -p iscc-jni` first; surefire
    sets `java.library.path` to `target/debug`) — new golden, realm-error, and decode round-trip
    tests plus all prior tests green.
- `mise run check` — all prek pre-commit hooks pass.
- (probe) Golden `ISCC:MAIGHFECJMOPMIAB` for `(1751831876325218, 1, 0)` matches the frozen
    cross-surface `iscc-core` oracle value.

## Done When

`genIsccIdV1` is exported from `IsccLib`, mints the golden ISCC, throws on invalid realm, its output
round-trips through `isccDecode` at Version 1, and all cargo + mvn + prek gates pass.
