## 2026-11-21 — User-docs accuracy batch — pkg-config version, Sum-Code prefix, llms.txt/ISO scope, Publish section, Ruby Float

**Done:** Fixed all five docs issues in a docs-only pass. The pkg-config example uses a placeholder.
Every Sum-Code and `gen_iscc_code_v0([data, instance])` comment now shows `ISCC:KUA...`. `llms.txt`,
`index.md` and `benchmarks.md` state the 9 ISO functions plus the `gen_sum_code_v0` convenience.
`development.md` → Publish describes the workflow dispatch, warns against tagging by hand, and lists
every chain. The Swift spec heading and intro are corrected, and the Ruby docs and doc comments
describe the measured Float truncation. The only edits under `crates/` are comment lines.

**Files changed:**

- `docs/howto/{dotnet,go,java,kotlin,nodejs,python,ruby,rust,swift,wasm}.md`, `docs/java-api.md`:
    `"ISCC:KAA..."` → `"ISCC:KUA..."` (comment lines only)
- `crates/iscc-jni/java/src/main/java/io/iscc/iscc_lib/SumCodeResult.java`: Javadoc `KAC` → `KUA`
- `docs/howto/c-cpp.md`: `Version: X.Y.Z` plus a one-line note to use the installed release
- `docs/index.md`: intro and feature bullet say "9 `gen_*_v0` functions that ISO 24138 specifies"
    plus the `gen_sum_code_v0` convenience; the "10 functions implemented" count is kept
- `docs/benchmarks.md`: Methodology says 9 ISO functions; `gen_sum_code_v0` is not compared (true:
    the tables list 9 rows)
- `docs/llms.txt`: names all 11 surfaces, one install command each where there is one, and links
    `index.md#available-bindings` for Java/Kotlin/Swift/C/C++
- `docs/development.md`: Publish rewritten
    (`gh workflow run release.yml --ref main -f version=X.Y.Z` or the Actions page; Cargo.toml
    version must match; no hand tag; 12-row table; all 8 checkbox names; FFI job has no
    exists-check). The tree comment for `release.yml` is generic now
- `.claude/context/specs/swift-bindings.md`: heading "Release Process — Checksum Commit, Then Tag";
    line-9 summary and intro paragraph say the workflow commits first and tags once
- `docs/ruby-api.md` → Error Handling: describes the `to_int` truncation, plus the
    `TypeError`/`FloatDomainError` cases
- `crates/iscc-rb/src/lib.rs`: doc comments of `array_to_vec` and `array_to_i32_vec` (no code)

**Verification:**

- Measured prefixes (Python, after `mise run py:develop`, data `b"Hello World"*1000`):
    `gen_iscc_code_v0([data, instance])` → `ISCC:KUABGAWXH2VYUQWIVYLQGD2NZH6TM`; `gen_sum_code_v0`
    (with and without `add_units`) → same; `SumHasher` → `ISCC:KUAN7YLATENGET3HQKFIMYFOQ24G6`. No
    example passes `wide`/128 bits (grepped the call sites)
- Ruby probe (rebuilt with `rake compile`, Ruby 3.4.13): `gen_audio_code_v0([1.9])` == `[1]` ==
    `[Rational(19,10)]`; `[-1.9]` == `[-1]`; `bits: 64.9` == `bits: 64`;
    `alg_minhash_256([1.9,2.9,3.9])` == `[1,2,3]`; `gen_iscc_id_v1(1.7e15, 5.9, 0.9)` == the integer
    call; `["1"]`/`[nil]`/`bits: "64"` → `TypeError`; `[Float::NAN]` → `FloatDomainError`; `[1e20]`
    → `RangeError`
- The release facts come from `release.yml`: inputs at lines 6-42; `prepare-release` checks the
    version against Cargo.toml, commits `Package.swift`, then runs `git tag … || echo` + push and
    creates the GitHub Release with `IsccLib.xcframework.zip`. Auth per job: `id-token: write` for
    crates.io/PyPI/npm/RubyGems, `MAVEN_*` secrets for both Maven jobs, `NUGET_API_KEY`,
    `contents: write` for FFI. Exists-check step present in every publish job except `publish-ffi`
- All 6 next.md greps print nothing. `llms.txt` contains Ruby, .NET, C++, Swift, Kotlin.
    `git diff -U0 crates/` shows only `///`/`/** */` lines
- `uv run zensical build` rc=0 ("No issues found");
    `uv run pytest tests/test_check_docs_nav.py tests/test_c_ffi_docs.py` 18 passed;
    `scripts/check_docs_nav.py` and `scripts/check_spec_paths.py` OK; `mise run check` rc=0
    (mdformat reflowed the development.md paragraphs on the first run)

**Next:** the JNI `NativeLoader` fallback (next.md names it as next in line). It needs Java tests
and a `-Dos.arch=riscv64` probe.

**Notes:**

- Left on purpose (out of scope per next.md): swift-bindings.md:393 (Verification Criteria
    "force-updates tag"), the Ferrostar row in its comparison table (describes Ferrostar's own
    pattern), and the `KAA` placeholder input in `docs/ruby-api.md`
- `docs/index.md` Available Bindings shows SPM `from: "0.7.0"`. That is fine: `docs/index.md` is a
    `version_sync.py` target (line 284)
- Also beyond the letter of next.md: `llms.txt` line 5 still said "all 10 gen\_\*\_v0 functions", so
    I rewrote it with the same 9 + convenience wording as `index.md`
- `zensical build` wiped `site/` (gitignored); `gen_llms_full.py` was not re-run, nothing tracked
    depends on it
