## 2026-11-21 — Remove hand-written counts and line numbers from the specs and per-crate CLAUDE.md files

**Done:** Removed every in-scope count, tally, file size and `path:NN` reference from all 15
`.claude/context/specs/*.md` files with a hit and 11 of the 12 per-crate/package `CLAUDE.md` files
(jni and ffi had only keep-class numerals). Counts became "every binding", "all Tier 1 symbols",
"every release platform", "every `gen_*_v0` function with `data.json` vectors". Nothing was replaced
with a corrected number. All Tier 1 symbol names were kept.

**Files changed:**

- `.claude/context/specs/`: c-ffi-dx, ci-cd, cpp/dotnet/go/java/kotlin/nodejs/python/ruby/swift/wasm
    -bindings, edge-vectors, rust-core. In `rust-core.md` the `Total: … = 35` arithmetic is replaced
    by one line pointing at `scripts/check_tier1_docs.py`. The section openers ("Four text
    processing functions", "Five codec functions", "One diagnostic function", …) now read "These
    …"/"This …". Both `path:NN` refs keep the path only.
- `crates/{iscc-lib,iscc-napi,iscc-py,iscc-rb,iscc-wasm}/CLAUDE.md`,
    `packages/{cpp,dotnet,go,kotlin,swift}/CLAUDE.md`: per-category numerals dropped from the Tier 1
    lists, plus test/vector tallies, platform counts, `~3200 lines`. The iscc-rb "line 58 in
    extensiontask.rb" now names `Rake::ExtensionTask#define` (checked in vendored rake-compiler
    1.3.1, `def define` at :50, guard at :58).

**Verification:**

- Grep 1 (`path:NN`), grep 2 (count classes, `grep -v 'JUnit [0-9]'`) and grep 3 (`~N lines` /
    `line N in`) each print nothing (exit 1 from git grep/grep)
- Wrap-insensitive pass: each file joined with `tr '\n' ' '` and grepped for digit and spelled-out
    counts before the same nouns. The only hits were false positives ("Tier 1 symbols", "JUnit 5",
    issue URLs, "two surfaces" as a general phrase)
- `check_tier1_docs.py` (run after each gated file: `OK: 20 full and 2 exact`),
    `check_spec_paths.py` (`OK … 15 specs`) and `check_ci_job_table.py` (`OK: 21 CI jobs`) exit 0
- `uv run pytest tests/test_check_tier1_docs.py tests/test_check_ci_job_table.py tests/test_c_ffi_docs.py tests/test_ruby_gem_docs.py`
    gives 78 passed
- `mise run format`, then `mise run check` exits 0
- Commit stat: only specs and `CLAUDE.md` files, plus `handoff.md` and advance `MEMORY.md`, which
    the protocol requires in the same commit. `iterations.jsonl` (runner-owned, dirty) is not staged

**Next:** the file-list part of the drift issue: indentation trees and module tables in the
`CLAUDE.md` files and specs, which no gate parses. That step has to decide between removing them and
gating them (`check_spec_paths.py` checks only that the paths exist, not that the list is complete).

**Notes:**

- Kept on purpose (keep classes in next.md): split thresholds (`~1500`/`~2500`/`~500 lines`),
    algorithm/format parameters, quoted error text, `~1.1 MB` per addon, `~784 MB` toolchain
    download, `~30 MB` revisit trigger, CRAP `(30)` / `> 10%` gate config, `~46,000` crossings
    rationale. Kept because the items are listed right there: `cpp/CLAUDE.md` "Four … RAII guards",
    `ci-cd.md` "Two CI gates" (two subsections follow), `rust-core.md` "three validation checks",
    "three edits", "three checked-in artifacts", "not four"
- Dropped the opcode tally in `wasm-bindings.md` (5370 vs 1993). The claim stays: the SIMD build
    shows more `v128` opcodes than a build without the feature
- Corrected in passing because the old sentences depended on a count: `rust-core.md` said "the two
    experimental ISCC-IDv1 symbols … their doc comments", but only `gen_iscc_id_v1` exists. It now
    names that symbol. `rust-core.md` "Update every place that repeats the Tier 1 count" now reads
    "lists the Tier 1 symbols"
- The Python 9-vs-10 contradiction is gone: the spec now says "one typed result class per `gen_*_v0`
    function", and the crate file says "one typed subclass per `gen_*_v0` result"
- Out of scope, seen but not fixed: `iscc-rb/CLAUDE.md` quotes the rake-compiler guard as
    `@gem_spec && @gem_spec.platform == 'ruby'`, while vendored 1.3.1 reads
    `!@no_native && (@gem_spec && …)`. Versions (JUnit 5 in `java-bindings.md` vs JUnit 6 in the
    crate file) are untouched, as next.md asked
- Docs-only: no Rust source changed, so the CRAP, iai and semver gates are unaffected and there is
    no API break
