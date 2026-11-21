# Next Work Package

## Step: User-docs accuracy batch — pkg-config version, Sum-Code prefix, llms.txt/ISO scope, Publish section, Ruby Float

## Goal

Close five `normal` docs issues that block v0.8.0 in one docs-only pass: "The pkg-config example in
the C/C++ how-to says `Version: 0.2.0`", "How-to examples show the Sum-Code prefix as `KAA`", "Docs
overstate language coverage and what ISO 24138 specifies" (`[audit]`), "`docs/development.md` tells
maintainers to release by pushing a tag", and "Ruby integer list elements accept a Float and
truncate it" (decided: keep the conversion, fix the docs).

## Alternatives Considered

- **Chosen:** the docs batch. All five are decided, docs-only (two comment-only source edits), and
    together they clear half the remaining `normal` blockers in one session.
- **Rejected:** the JNI `NativeLoader` fallback. It is a real code defect, but it needs Java tests
    and a foreign-platform probe, so it is better as its own step. It is next in line.

## Scope

- **Modify** (docs): `docs/howto/c-cpp.md`,
    `docs/howto/{dotnet,go,java,kotlin,nodejs,python,ruby,rust,swift,wasm}.md` (comment lines only),
    `docs/java-api.md`, `docs/llms.txt`, `docs/index.md`, `docs/benchmarks.md`,
    `docs/development.md`, `docs/ruby-api.md`, `.claude/context/specs/swift-bindings.md`
- **Modify** (comment-only source, outside the file cap per decision 2026-11-20):
    `crates/iscc-jni/java/src/main/java/io/iscc/iscc_lib/SumCodeResult.java` (`iscc` field Javadoc),
    `crates/iscc-rb/src/lib.rs` (doc comment on `array_to_i32_vec`, and any sibling `array_to_vec`
    comment making the same "TypeError for a non-Integer" claim)
- **Reference**: `.github/workflows/release.yml` (inputs at lines 6-42, `prepare-release`),
    `scripts/version_sync.py` (`TARGETS`), learnings → "Release pipeline" / "prepare-release"

## Not In Scope

- `docs/howto/wasm.md` Node.js setup and return-type sentences (separate `[audit]` issue, needs a
    packed-build probe). Edit only the `KAA` comment lines in that file
- `docs/architecture.md` and the "Crate Summary" table in `docs/development.md` (separate
    architecture issue)
- The Swift spec's Verification Criteria line and target.md's "force-updates the tag" criterion.
    Change only the heading, the intro paragraph and the line-9 summary
- Changing Ruby's conversion behaviour, adding Ruby tests, or adding a `data-edge.json` vector (the
    edge-vectors rules exclude wrong-typed elements)
- Making the pkg-config line a `version_sync.py` target. Use a placeholder
- The `iscc_decompose("ISCC:KAA...")` placeholder input in `docs/ruby-api.md`. It is an input, not a
    result claim

## Implementation Notes

**pkg-config (`c-cpp.md:86`):** replace `0.2.0` with a placeholder the reader fills in (e.g.
`Version: X.Y.Z` plus a one-line note to use the installed release). Learnings: never hand-type a
version in docs.

**Sum-Code prefix:** every `"ISCC:KAA..."` comment in `docs/howto/*.md` and `docs/java-api.md:338`
sits after `gen_sum_code_v0` or after `gen_iscc_code_v0([dataCode, instanceCode])`. Both return
SubType SUM, so `ISCC:KUA...` for 64-bit, non-wide. Run one example of each kind once (Python via
`uv run python -c`, after `mise run py:develop`) to confirm the prefix, and record the outputs in
the handoff. If an example passes `wide=True` or extra units, use the prefix that call actually
returns. `SumCodeResult.java:13` says `KAC`: fix it the same way.

**llms.txt (`docs/llms.txt:5,9`):** name all 11 surfaces (Rust, Python, Node.js, WASM, Go, Java,
Ruby, C#/.NET, C/C++, Swift, Kotlin). Give each install command, or point to the install section of
the index/getting-started page (keep it short). The file is hand-written, not generated. Check
`scripts/check_docs_nav.py` still passes.

**ISO scope:** `docs/index.md:17,26` and `docs/benchmarks.md:15` must say the standard specifies 9
`gen_*_v0` functions and that `gen_sum_code_v0` is an iscc-lib convenience on top (Data-Code +
Instance-Code in one pass). Keep "10 functions implemented" where it is only a count.

**`docs/development.md` → Publish (:351-385):** after the merge to `main`, tell the reader to
dispatch the Release workflow with a `version` input (Actions page or
`gh workflow run release.yml --ref main -f version=X.Y.Z`; check the input names in `release.yml`).
Say explicitly not to push `vX.Y.Z` by hand: `prepare-release` creates `vX.Y.Z` and
`packages/go/vX.Y.Z` after the `Package.swift` checksum commit and never moves an existing tag. The
registry table must cover every chain: crates.io, PyPI, npm `@iscc/lib` + `@iscc/wasm`, Maven
Central `io.iscc:iscc-lib` and `io.iscc:iscc-lib-kotlin`, RubyGems, NuGet `Iscc.Lib`, FFI tarballs
and the XCFramework on GitHub Releases, and SPM/Go via the tags. Read the auth methods from
`release.yml`; do not guess them. Name all 8 checkboxes, or say the table is an excerpt. Also fix
the tree comment at `:259` (`release.yml # Publish to …`) so it lists all registries or none.

**Swift spec (`swift-bindings.md:8-9, 231-236`):** rename the heading (e.g. "Release Process —
Checksum Commit, Then Tag"). In the intro, say the workflow commits the checksum first and then
creates the tag once, so it never moves one. Ferrostar may stay as the model for the toggle. Spec
paths are gated by `scripts/check_spec_paths.py`.

**Ruby Float:** build the extension (`mise exec -- bundle exec rake compile` in `crates/iscc-rb`;
never `rake clobber`) and probe `IsccLib.gen_audio_code_v0([1.9])` against `[1]`. Also probe a
scalar such as `IsccLib.gen_audio_code_v0([1], bits: 64.9)` and the `alg_minhash_256` features. In
`docs/ruby-api.md` → Error Handling (:854), describe what you measured: Integer parameters and
integer list elements accept a Float (and anything with `to_int`) and truncate it toward zero, as
Ruby's own `Array#[]` does. Make the doc comment(s) in `lib.rs` match. Comment edits only: no code
change.

No text hot path is touched, so neither the iai nor the CRAP gate is affected.

## Verification

- `grep -rnE 'ISCC:KA[AC]\.\.\.' docs/howto docs/java-api.md crates/iscc-jni/java/src/main` prints
    nothing, and the handoff records the measured prefix for one Sum-Code and one
    `gen_iscc_code_v0([data, instance])` example
- `grep -nE '^Version: [0-9]' docs/howto/c-cpp.md` prints nothing
- `grep -nE '(specified|defined) by ISO 24138|Complete ISO 24138 coverage' docs/index.md docs/benchmarks.md`
    prints nothing; `docs/llms.txt` names Ruby, .NET, C++, Swift and Kotlin
- `grep -nE 'git (tag|push origin) v|tag push triggers' docs/development.md` prints nothing, and
    `grep -n 'Force-Update Tag Pattern' .claude/context/specs/swift-bindings.md` prints nothing
- `docs/ruby-api.md` → Error Handling states the Float conversion; the `array_to_i32_vec` doc
    comment no longer claims `TypeError` for every non-Integer; the only edits under `crates/` are
    comment lines
- `uv run zensical build` exits 0,
    `uv run pytest tests/test_check_docs_nav.py tests/test_c_ffi_docs.py` passes, and
    `mise run check` exits 0

## Done When

All six verification criteria pass, and the five issues' "Resolved when" conditions hold in the
tree.
