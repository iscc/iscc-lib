## 2026-02-27 — Propagate 7 Tier 1 symbols to Java JNI

**Done:** Added 4 constants (`META_TRIM_NAME`, `META_TRIM_DESCRIPTION`, `IO_READ_SIZE`,
`TEXT_NGRAM_SIZE`) as `public static final int` fields, 3 new JNI native methods (`jsonToDataUrl`,
`encodeComponent`, `isccDecode`), and a new `IsccDecodeResult` Java class to the Java JNI binding.
Java JNI now exposes 30/30 Tier 1 symbols.

**Files changed:**

- `crates/iscc-jni/src/lib.rs`: Added 3 new `extern "system"` JNI functions — `jsonToDataUrl`
    (string-in/string-out), `encodeComponent` (scalar args + byte array → string with range
    validation), `isccDecode` (string → `IsccDecodeResult` Java object via `env.new_object`). Added
    `jobject` to imports.
- `crates/iscc-jni/java/src/main/java/io/iscc/iscc_lib/IsccLib.java`: Added 4
    `public static final int` constants before the static initializer, 3 native method declarations
    (`jsonToDataUrl`, `encodeComponent`, `isccDecode`).
- `crates/iscc-jni/java/src/main/java/io/iscc/iscc_lib/IsccDecodeResult.java`: New class with 5
    `public final` fields (`maintype`, `subtype`, `version`, `length`, `digest`) and a single
    constructor.
- `crates/iscc-jni/java/src/test/java/io/iscc/iscc_lib/IsccLibTest.java`: Added 7 new tests —
    constants verification (4 assertions), `jsonToDataUrl` (2 tests: plain JSON + JSON-LD),
    `encodeComponent` (1 test), `isccDecode` (1 known value test), `isccDecode` invalid input error
    test, and encode→decode roundtrip test.

**Verification:**

- `cargo build -p iscc-jni` succeeds
- `cargo clippy -p iscc-jni --all-targets -- -D warnings` clean
- `cd crates/iscc-jni/java && mvn test` passes — 58 tests (51 existing + 7 new), 0 failures
- `grep -c 'extern "system"' crates/iscc-jni/src/lib.rs` shows 33 (32 functions + 1 doc comment
    mention — the spec expected 32 but didn't account for the line-3 doc comment)
- `grep -c 'static final int' IsccLib.java` shows 4
- `grep 'IsccDecodeResult' IsccLib.java` finds the `isccDecode` native method
- `mise run check` passes — all 14 pre-commit hooks pass

**Next:** All 30 Tier 1 symbols are now propagated to Python, Node.js, WASM, C FFI, and Java JNI
bindings. Remaining: Go/wazero binding (23/30 → 30/30). The define-next agent should propagate the 7
new symbols to the Go binding.

**Notes:** The `isccDecode` JNI function constructs a Java `IsccDecodeResult` object on the Rust
side using `env.find_class` + `env.new_object` with constructor signature `(IIII[B)V`. The
`encodeComponent` function validates jint ranges (0-255 for mtype/stype/version, ≥0 for bitLength)
before casting. The verification grep count of 33 vs expected 32 is harmless — the extra match is
the doc comment on line 3 of the module, not an actual function.
