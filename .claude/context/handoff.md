## 2026-02-27 — Review of: Propagate 7 Tier 1 symbols to Java JNI

**Verdict:** PASS

**Summary:** Clean, focused implementation adding 4 constants, 3 new JNI native methods
(`jsonToDataUrl`, `encodeComponent`, `isccDecode`), and a new `IsccDecodeResult` Java class to the
Java JNI binding. All 30/30 Tier 1 symbols are now accessible via JNI. The implementation follows
existing patterns consistently and includes comprehensive tests.

**Verification:**

- [x] `cargo build -p iscc-jni` succeeds
- [x] `cargo clippy -p iscc-jni --all-targets -- -D warnings` clean — no warnings
- [x] `cd crates/iscc-jni/java && mvn test` passes — 58 tests (51 existing + 7 new), 0 failures
- [x] `grep -c 'pub extern "system" fn' crates/iscc-jni/src/lib.rs` shows 32 (29 existing + 3 new)
- [x] `grep -c 'static final int' IsccLib.java` shows 4
- [x] `grep 'IsccDecodeResult' IsccLib.java` finds the `isccDecode` native method
- [x] `mise run check` passes — all 14 pre-commit hooks pass

**Issues found:**

- (none)

**Codex review:** No correctness issues identified. Method naming/signatures match, object
construction for `IsccDecodeResult` is valid, jint range checks prevent lossy casts. Both Rust build
and Maven test suite pass.

**Next:** All 30 Tier 1 symbols are now propagated to Python, Node.js, WASM, C FFI, and Java JNI
bindings. Remaining: Go/wazero binding (23/30 → 30/30). The define-next agent should propagate the 7
new symbols to the Go binding.

**Notes:** Minor fix applied — removed duplicate `// ── Encoding ── (additional)` section comment in
`IsccLib.java`. The `isccDecode` JNI function constructs a Java `IsccDecodeResult` object on the
Rust side using `env.find_class` + `env.new_object` with constructor signature `(IIII[B)V`. The
`encodeComponent` validates jint ranges (0-255 for mtype/stype/version, ≥0 for bitLength) before
casting.
