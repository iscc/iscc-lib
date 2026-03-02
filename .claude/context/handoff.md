## 2026-03-02 — Add gen_sum_code_v0 examples to all 6 howto guides

**Done:** Added a `### Sum-Code` subsection with a minimal working code example to the Code
Generation section of each per-language howto guide (rust.md, python.md, nodejs.md, wasm.md,
java.md, go.md). Each subsection follows the existing pattern: heading, one-line description, and a
fenced code block showing file creation, `gen_sum_code_v0` call, and result field printing.

**Files changed:**

- `docs/howto/rust.md`: Added Sum-Code subsection after ISCC-CODE with
    `gen_sum_code_v0(Path, 64, false)` example
- `docs/howto/python.md`: Added Sum-Code subsection with `gen_sum_code_v0("example.bin")` example
    using pathlib for file setup
- `docs/howto/nodejs.md`: Added Sum-Code subsection with `gen_sum_code_v0("example.bin")` example
    using `writeFileSync` for file setup
- `docs/howto/wasm.md`: Added Sum-Code subsection with `gen_sum_code_v0(data)` taking `Uint8Array`
    (no filesystem), includes explanatory note about WASM's in-memory data model
- `docs/howto/java.md`: Added Sum-Code subsection with `IsccLib.genSumCodeV0(path, 64, false)`
    example using `Files.write` for file setup, accessing result via `.iscc()`, `.datahash()`,
    `.filesize()` methods
- `docs/howto/go.md`: Added Sum-Code subsection with `iscc.GenSumCodeV0(path, 64, false)` example
    using `os.WriteFile` for file setup, accessing result via `.Iscc`, `.Datahash`, `.Filesize`
    fields

**Verification:**

- `grep -c "### Sum-Code"` across all 6 howto files: each returns 1 ✓
- `grep -l "gen_sum_code_v0\|GenSumCodeV0\|genSumCodeV0" docs/howto/*.md | wc -l`: returns 6 ✓
- `uv run zensical build` exits 0 (docs site builds successfully) ✓
- `mise run format` exits 0 (formatting clean after mdformat pass) ✓
- `mise run check` passes (14/14 hooks pass) ✓

**Next:** All howto guides now have complete Sum-Code examples. The documentation work for
gen_sum_code_v0 is fully complete across READMEs, docs pages, and howto guides. Remaining work:
address issue #16 (feature flags for minimal builds, low priority).

**Notes:** mdformat auto-adjusted some inline comment spacing in the JavaScript code blocks (e.g.,
`// "ISCC:KAA..."` → consolidated onto same line). This is cosmetic and consistent with how mdformat
already formatted adjacent examples in those files. No substantive content was changed by the
formatter.
