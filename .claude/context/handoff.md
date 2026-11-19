## 2026-11-19 — Review of: Tier-1 33→35 doc/count sweep + `encode_seq`/`decode_seq` doc entries

**Verdict:** PASS_WITH_NOTES

**Summary:** Docs-only fan-out (plus one uniffi module-doc comment) across 36 files, within the
declared `**Fan-out:**` scope and nothing from Not In Scope. Every Tier-1 count reads 35. All 11
how-to pages and the 4 hand-written API pages document `encode_seq`/`decode_seq`, and every
signature matches its source. Each re-derived count (30 pyfunctions, 58 Go vectors = 50 + 4 + 4, 19
Swift/Kotlin methods, napi 5 codec) checks out. Review fixed two small doc nits Codex found.

**Verification:**

- [x] Stale-count grep prints nothing (exit 1). It is non-vacuous: the same grep on `d3e10fa` finds
    8 hits
- [x] Seq-coverage `grep -LiE "encode_?seq"` prints no filenames. The rendered HTML for all 11
    howtos carries seq (case-insensitive grep)
- [x] Manual signature review: all 12 surfaces match the Implementation Notes table and the source
    (`codec.rs:818/857`, `index.d.ts`, `IsccLib.java:288/300`, `codec.go:879/919`,
    `IsccLib.cs:488/542`, `iscc.h:645/670`, `iscc.hpp:366/384`, rb `lib.rs:329/340`, Swift/Kotlin
    generated files). Error contracts were also checked: Rust/C/Java/Ruby/.NET exception types and
    excluded MainTypes. No fabricated return types
- [x] `uv run zensical build`: exit 0, "No issues found" (`gen_llms_full.py` re-run afterwards)
- [x] `mise run check` exit 0 with no rewrites. `cargo build -p iscc-uniffi` finished. Workspace
    clippy `-D warnings` exit 0
- [x] Extra probes:
    - The C++ seq howto example, compiled with `g++ -Wall -Wextra -Werror` against `iscc.hpp`, printed
        `34 / 2007a88248c5 / ISCC:… / true`
    - The C example (with the prefixed type) printed the same, with `decoded[1] == NULL`
    - The Ruby example on a fresh `.so` printed `34`, `"2007a88248c5"`, ASCII-8BIT, round trip `true`
    - A tree-wide grep for stale `3 codec` / `46 vectors` / `9 test methods` / `27 #[pyfunction]`
        found nothing

**Issues found:**

- Fixed in review:
    - `docs/java-api.md:508` used `Arrays.equals` without an import → `java.util.Arrays.equals`,
        matching the page's own `java.util.Arrays.copyOf`
    - The Swift/Kotlin `CLAUDE.md` tree comments said `gen_iscc_id_v1` comes from `data.json`. It does
        not; it is a hard-coded golden test. Reworded
- The new `iscc_encode_seq`/`iscc_decode_seq` C snippets use the unprefixed `IsccByteBuffer`. This
    follows page style as next.md directed, and the existing `docs/c-ffi-api.md` type-name issue
    covers it
- `docs/ruby-api.md` says invalid input raises `RuntimeError`. That holds for codec errors only:
    `encode_seq([nil])` raises `TypeError` (probed). The page-wide "all methods raise
    `RuntimeError`" sentence predates this step. Nit, not filed
- Nit: the new C++ seq snippet lacks `#include <iostream>`. Its neighbouring codec/IDv1 snippets
    omit it too (page convention)
- Nit: the Swift/Kotlin "9 ISCC-SEQ methods … negatives" includes one positive control
    (`testEncodeSeqPaddingControl`)

**Codex review:** Run on `f386584`, the main commit; HEAD is handoff-only. Six findings:

- Java `Arrays` import (P3) and the Swift/Kotlin IDv1 wording (P3): valid, fixed in review
- C `IsccByteBuffer` (P2): already tracked; out of scope by next.md
- .NET `EncodeSeq` accepts `unit + "\0junk"`, so the "strict" wording overstates (P2): already
    tracked in the .NET/C++ NUL-truncation issue; fixing the behaviour makes the docs true
- Ruby exception contract (P2): verified `TypeError`; pre-existing page wording, see above
- C++ `<iostream>` (P3): page convention

**Next:** the Ruby Magnus `Vec<T>` segfault fix (`normal`, single file `crates/iscc-rb/src/lib.rs`):
take `RArray` and iterate with `into_iter()`, as `alg_simhash` already does. `encode_seq` and
`gen_mixed_code_v0` are affected call sites. That fix rewrites exactly the argument conversion
behind the `TypeError`-vs-`RuntimeError` contract. Have its tests pin which exception class a
non-String element raises, and correct the `docs/ruby-api.md` Error Handling sentence in the same
step if the class stays `TypeError`.

**Notes:**

- ISCC-SEQ is now bound and documented on all 11 surfaces. The remaining seq-adjacent issues are
    behaviour bugs (Ruby segfault, .NET/C++ NUL truncation, napi Buffer leak, the Python docstring)
    and the C-page type-name question (rename vs alias is a design call)
- Commit `4503ed0` changed the c-cpp howto prose to `iscc_IsccByteBuffer`, so the howto and
    `docs/c-ffi-api.md` now differ in naming until that issue is decided
- learnings.md: the two "Tier 1 = 33 / sweep pending" bullets now read 35 / done at 211
