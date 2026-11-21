## 2026-11-21 — Review of: User-docs accuracy batch — pkg-config version, Sum-Code prefix, llms.txt/ISO scope, Publish section, Ruby Float

**Verdict:** PASS_WITH_NOTES

**Summary:** The docs-only batch does what next.md asked, and every factual claim I re-derived
holds: the release facts against `release.yml`, the `KUA` prefix from Python, the Ruby conversions
from a rebuilt extension. Five `normal` issues are closed. The scope missed sibling sentences that
carry the same false claims; I fixed two comment lines directly and filed the rest as one issue.

**Verification:**

- [x] `grep -rnE 'ISCC:KA[AC]\.\.\.' docs/howto docs/java-api.md crates/iscc-jni/java/src/main`
    prints nothing; measured again: `gen_iscc_code_v0([data, instance])` and `gen_sum_code_v0` both
    return `ISCC:KUABGAWXH2VYUQWIVYLQGD2NZH6TM` (`wide=True` with 64-bit units returns the same)
- [x] `grep -nE '^Version: [0-9]' docs/howto/c-cpp.md` prints nothing
- [x] The ISO-scope grep on `docs/index.md` and `docs/benchmarks.md` prints nothing; `docs/llms.txt`
    names Ruby, .NET, C++, Swift and Kotlin. iscc-core 1.5.0 has no `gen_sum_code_v0` (checked)
- [x] The tag-push grep on `docs/development.md` and the `Force-Update Tag Pattern` grep on the
    Swift spec print nothing. Publish section checked job by job against `release.yml` (needs,
    conditions, permissions, secrets, exists-check steps, the eight input names): all correct
- [x] `docs/ruby-api.md` → Error Handling states the Float conversion; both `lib.rs` doc comments
    match; the advance edits under `crates/` are comment lines only. Ruby probe re-run: all eleven
    results as the handoff reported, plus `Float::INFINITY` → `FloatDomainError`
- [x] `uv run zensical build` exits 0, the two pytest files pass (18), `mise run check` exits 0

**Issues found:**

- Fixed here (step 9, comment or prose only): `crates/iscc-lib/src/types.rs` and
    `crates/iscc-wasm/src/lib.rs` still said `"ISCC:KAC..."` on the Sum-Code result's `iscc` field;
    `docs/ruby-api.md` now says "a `NaN` or infinite Float"
- Filed `normal`: "`docs/ecosystem.md` and two other pages still count 10 standard functions". The
    step fixed the sentence on three pages; `ecosystem.md` (four places, plus six of eleven
    languages), `architecture.md` → "How It Works" and `howto/nodejs.md` → "Code generation" kept it
- Filed `low`, untriaged: "target.md and the Swift spec criterion say the release workflow
    force-updates the tag". next.md excluded both lines and nothing tracked them; `target.md` needs
    Titusz
- Not filed (nits): `docs/benchmarks.md` says "all benchmarks measure the 9 functions", while the
    criterion suite also times `gen_sum_code_v0` (the page shows no row for it). In
    `docs/development.md` the XCFramework row sits under "once `prepare-release` has finished",
    although that job uploads it

**Codex review:** ran over both advance commits (`--base 47738d0`); no findings. It confirmed the
release instructions against the workflow.

**Next:** the JNI `NativeLoader` fallback stays next in line (Java test plus a `-Dos.arch=riscv64`
probe on a built JAR). A cheap alternative is one docs-only step that closes both
`docs/architecture.md` entries and the new `ecosystem.md` entry: three files, no code. Scope it with
`git grep -nE '10 .gen_' -- docs` and read each hit, because six of the eleven hits are correct
library-wide counts.

**Notes:**

- Closed: Ruby Float docs, pkg-config version, Sum-Code prefix, `development.md` tag push, and the
    `[audit]` language-coverage entry. Left: 6 `normal` (JNI loader, architecture sections, one-shot
    duplication, `@iscc/wasm` Node docs, `alg_minhash_256([])`, ecosystem leftovers) and 8 `low`
- Titusz's Ruby decision lived only in the deleted issue, so it is now a decisions.md entry ("Ruby
    integer parameters keep Ruby's implicit `to_int` conversion")
- decisions.md is at 400 lines and learnings.md at 200: the next entry in either needs a cut or a
    rotation in the same commit
- `zensical build` wiped `site/`; nothing tracked depends on it
