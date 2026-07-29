## 2026-07-29 — Mint `gen_iscc_id_v1` on the JNI (Java) surface

**Done:** Exported `genIsccIdV1(long timestamp, int hubId, int realm) -> String` from the JNI
surface (7th of 11 in the #43 IDv1 fan-out), bringing `crates/iscc-jni` to 33 usable Tier 1 symbols.
The Rust wrapper range-validates `hubId`/`realm` before narrowing to `u16`/`u8`, then delegates to
`iscc_lib::gen_iscc_id_v1`, which re-validates and maps `Err` → `IllegalArgumentException`.

**Files changed:**

- `crates/iscc-jni/src/lib.rs`: added `Java_io_iscc_iscc_1lib_IsccLib_genIsccIdV1` wrapper (mirrors
    `genTextCodeV0`; narrowing guards for `hubId` 0-65535 / `realm` 0-255, `timestamp as u64`
    unguarded per next.md rationale).
- `crates/iscc-jni/java/.../IsccLib.java`: added `public static native String genIsccIdV1(...)`
    decl.
- `crates/iscc-jni/java/.../IsccLibTest.java` (test): added golden, invalid-realm, and decode
    round-trip `@Test`s.

**Verification:**

- `cargo build -p iscc-jni` — clean; `cargo clippy -p iscc-jni --all-targets -- -D warnings` —
    clean; `cargo fmt -p iscc-jni --check` — clean.
- `mvn clean test -f crates/iscc-jni/java/pom.xml` — **96 tests, 0 failures** (83 IsccLibTest incl.
    3 new + 13 UnicodeBoundaryTest); ran `cargo build -p iscc-jni` first so surefire's
    `java.library.path=target/debug` resolves the native lib.
- `mise run check` — all prek pre-commit hooks Passed.
- (probe) Golden `ISCC:MAIGHFECJMOPMIAB` for `(1751831876325218, 1, 0)` asserted green by the
    passing `genIsccIdV1Golden` test — matches the frozen cross-surface `iscc-core` oracle.

**Next:** #43 fan-out continues on the next typed-int surface — **rb** (Magnus), then
uniffi→Swift/Kotlin, dotnet C# consumer (regenerate `NativeMethods.g.cs` in-step), cpp. One surface
per step. After the fan-out, the deferred Tier-1 32→33 doc/count sweep across the stale sites in
#43.

**Notes:** JNI `isccDecode` returns `version` as a plain `int`, so the decode round-trip works today
with no `IsccDecodeResult`/decode-path change (as next.md scoped). No public-API break, no hot path
touched. jni docs carry no numeric symbol count, so no doc edit here (mirrors the ffi slice).
