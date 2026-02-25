## 2026-02-25 — Create Java how-to guide and add to navigation

**Done:** Created `docs/howto/java.md` (321 lines) — the last remaining how-to guide — covering all
10 required sections: front matter, intro, installation (Maven + build-from-source), setup
(System.loadLibrary + java.library.path), code generation (all 9 gen\*V0 methods), streaming
(DataHasher/InstanceHasher with try-finally memory management), text utilities, algorithm
primitives, conformance testing, and error handling. Added Java navigation entry to `zensical.toml`
after the Go entry.

**Files changed:**

- `docs/howto/java.md`: Created complete Java how-to guide following the Go guide structural
    template, adapted for Java's static API (no runtime object), JNI setup, camelCase method names,
    opaque `long` handles for streaming, and `byte[]`/`int[]`/`int[][]` types
- `zensical.toml`: Added `{ "Java" = "howto/java.md" }` entry after Go in How-to Guides nav

**Verification:** All 10 verification criteria pass:

- `test -f docs/howto/java.md` — file exists
- `grep 'io.iscc'` — Maven coordinates present (groupId and import)
- `grep 'System.loadLibrary'` — native library setup documented
- `grep 'genMetaCodeV0'` — code generation documented (3 matches)
- `grep 'dataHasherNew'` — streaming documented (2 matches)
- `grep 'textClean'` — text utilities documented
- `grep 'conformanceSelftest'` — conformance testing documented
- `grep '"Java"' zensical.toml` — nav entry present
- `grep 'howto/java.md' zensical.toml` — nav path correct
- `uv run zensical build` — site builds with `/howto/java/` in output
- All 14 pre-commit hooks pass

**Next:** All 6 how-to guides are complete (Rust, Python, Node.js, WASM, Go, Java). Consider
addressing tracked issues from issues.md: codec optimization, DataHasher buffer allocation,
`__version__` attribute, or the TypeScript port evaluation.

**Notes:** The Java guide is 321 lines, within the target range of 300-400 (Go: 388, Rust: 356,
Python: 353, WASM: 338, Node.js: 281). Key Java-specific adaptations vs Go guide: no "Runtime setup"
section (static methods), replaced with "Setup" section explaining `System.loadLibrary` and
`java.library.path`; streaming uses opaque `long` handles with explicit try-finally instead of Go's
`defer`; `genIsccCodeV0` exposes `boolean wide` parameter (Go hardcodes to false). All API
signatures verified against `IsccLib.java`. Used `icon: lucide/coffee` (Java's coffee cup).
