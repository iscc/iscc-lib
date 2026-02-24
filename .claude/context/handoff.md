## 2026-02-24 — Add Java sections to root README

**Done:** Updated root README.md with three changes: (1) added "Java" to the Key Features polyglot
line, (2) added a Java/Maven installation subsection with `<dependency>` XML and `java.library.path`
caveat, (3) added a Java quick start code example using `IsccLib.genMetaCodeV0`. Java sections are
inserted after Node.js and before WASM in both Installation and Quick Start.

**Files changed:**

- `README.md`: Added "Java" to Key Features line 23; added `### Java` installation subsection with
    Maven coordinates (`io.iscc:iscc-lib:0.0.1`) and native library note; added `### Java` quick
    start subsection with `genMetaCodeV0` example matching `IsccLib.java` signatures

**Verification:** All verification criteria pass:

- `grep -q 'Python, Java, Node.js, WASM, and C FFI' README.md` — exits 0 ✓
- `grep -q '### Java' README.md` — exits 0 ✓
- `grep -q 'io.iscc' README.md` — exits 0 ✓
- `grep -q 'genMetaCodeV0' README.md` — exits 0 ✓
- `grep -c '### Java' README.md` — returns 2 ✓
- `grep -q 'java.library.path' README.md` — exits 0 ✓
- `mise run check` — all 14 pre-commit hooks pass ✓

**Next:** Root README now covers all implemented binding languages. Remaining documentation gaps:
creating `docs/howto/java.md` per-language guide, or addressing tracked performance issues (codec
header optimization, DataHasher allocation overhead). The "What is iscc-lib" paragraph (line 46)
still says "Python, Node.js, WebAssembly, and C" without Java — this was explicitly out of scope per
next.md but could be a follow-up.

**Notes:** No surprises. The pom.xml version is `0.0.1-SNAPSHOT` but the README uses `0.0.1`
(without `-SNAPSHOT`) matching the pattern in `crates/iscc-jni/README.md`. The Java quick start uses
all four `genMetaCodeV0` parameters explicitly (name, null, null, 64) because Java has no default
parameter syntax — this matches the per-crate README example.
