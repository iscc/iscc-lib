## 2026-02-25 — Review of: Implement Java NativeLoader class

**Verdict:** PASS

**Summary:** Created `NativeLoader.java` with platform detection, JAR resource extraction, temp file
management, and `System.loadLibrary` fallback. Updated `IsccLib.java` to delegate to
`NativeLoader.load()`. Clean implementation following the well-established JNI native loader pattern
(sqlite-jdbc, netty-tcnative). All verification criteria met.

**Verification:**

- [x] `cd crates/iscc-jni && cargo build` succeeds — Rust side unchanged, compiles clean
- [x] `cd crates/iscc-jni/java && mvn test` — all 49 tests pass (loader fallback path works)
- [x] `test -f NativeLoader.java` exits 0 — file exists at expected path
- [x] `grep 'NativeLoader.load' IsccLib.java` exits 0 — delegation present in static initializer
- [x] `grep -c 'System.loadLibrary' IsccLib.java` outputs `0` — no direct loadLibrary in IsccLib
- [x] `grep 'META-INF/native' NativeLoader.java` exits 0 — extraction path present (4 matches)
- [x] All 14 pre-commit hooks pass
- [x] No quality gate circumvention in diff

**Issues found:**

- (none)

**Next:** The NativeLoader infrastructure is in place. The JAR extraction path won't activate until
native binaries are bundled into `META-INF/native/` during CI (a future iteration). Consider
addressing tracked issues: codec header parsing optimization (`[normal]`), DataHasher buffer
allocation (`[normal]`), iscc-ffi video frame allocation (`[normal]`), or lower-priority items like
`__version__` attribute, module docstring fix, WASM conformance feature gate, DCT power-of-two
validation, or the JNI `IllegalStateException` improvement.

**Notes:** Helper methods (`detectOs`, `detectArch`, `libraryFileName`) are package-private rather
than private, making them testable without reflection. The `try (stream)` pattern uses Java 9+
try-with-resources on an effectively-final variable, consistent with the Java 17 target. The
`volatile` + `synchronized` pattern is a correct (if conservative) thread-safety approach — the
`synchronized` on `load()` makes the volatile guard redundant for correctness but harmless and
documents intent clearly.
