# Next Work Package

## Step: Docs accuracy batch — architecture.md covers every surface, no "10 standard functions" or Sum-Code-as-string claims

## Goal

Close two `normal` docs issues in one docs-only step: "`docs/architecture.md` sections below the
graph cover half the surfaces" and "Six docs pages count 10 standard functions or give Sum-Code a
string result". Both block v0.8.0 and touch overlapping pages.

## Alternatives Considered

- **Chosen:** the docs batch — handoff "Next", fully specified by the two issues, no probe or
    decision needed, and it shares `architecture.md` between both issues.
- **Rejected:** `@iscc/wasm` Node.js docs (`[audit]`) — needs a packed `--target web` release build
    probe; better as its own step next. One-shot/streaming dedup — a core hot-path change with the
    iai baseline in play; `alg_minhash_256([])` needs a behaviour decision first.

## Scope

- **Modify**: `docs/architecture.md`, `docs/ecosystem.md`, `docs/howto/nodejs.md`,
    `docs/howto/java.md`, `docs/java-api.md`, `docs/c-ffi-api.md` (docs only, outside the file cap)
- **Reference**: `.github/workflows/ci.yml` (each job's test command),
    `.github/workflows/release.yml` (release assets, registries), `crates/iscc-ffi/src/lib.rs:905`
    (`iscc_gen_sum_code_v0` → `IsccSumCodeResult`),
    `crates/iscc-jni/java/src/main/java/io/iscc/iscc_lib/IsccLib.java:161`,
    `crates/iscc-napi/index.d.ts:193`, each crate/package `CLAUDE.md` for streaming/test layout

## Not In Scope

- `docs/howto/wasm.md:82` and `crates/iscc-wasm/README.md` — they belong to the `[audit]` WASM
    Node.js docs issue (next step, needs a probe)
- The "All 10" sentences in `howto/{dotnet,go,kotlin,python,ruby,rust,swift}.md`, `ruby-api.md` and
    `tutorials/getting-started.md` — library-wide counts that already say "typed results"; leave
    them
- `notes/` (never updated in sweeps), `README.md`, per-crate READMEs
- Any change to code, tests, or the doc gates

## Implementation Notes

**`architecture.md`**

- "Crate Summary": add rows for `iscc-rb` (RubyGems), `iscc-uniffi` (not published itself; feeds
    Swift/Kotlin), `packages/dotnet` (NuGet), `packages/cpp` (vcpkg/Conan + FFI tarballs),
    `packages/swift` (SPM via Git tag + XCFramework release asset), `packages/kotlin` (Maven Central
    `io.iscc:iscc-lib-kotlin`). `iscc-ffi` → "GitHub Release assets" (prebuilt
    `iscc-ffi-v*-<target>` archives — confirm the name pattern in `release.yml`), not "Source".
- "Workspace Layout" tree: add one line per missing directory (`iscc-rb/`, `iscc-uniffi/`,
    `packages/{cpp,dotnet,kotlin,swift}/`); you may collapse the long Go file list to one line. Fix
    the `release.yml` comment (it publishes to every registry, not three). If you keep it partial
    anywhere, say "excerpt" in the prose above the tree.
- "Per-Binding Adaptation": add Ruby, C#, C++, Swift, Kotlin rows. State only what the code shows
    (class names, method names, disposal/RAII); do not claim GIL/GVL release or async support
    without reading the binding source.
- "How It Works": 9 `gen_*_v0` sections in `data.json` (iscc-core has no `gen_sum_code_v0`). Item 2
    ("via relative path or `include_str!`") must stay true for C/C++ (generated header) — reword if
    not. Mention `data-edge.json` in one sentence if it fits naturally.
- "Cross-Language Test Matrix": add Ruby, C#, C++, Swift, Kotlin rows; WASM runner is
    `wasm-pack test --node` (from `ci.yml:124`), Rust/C rows checked against `ci.yml` too. Vector
    access per row from the actual test sources (C/C++ read `data_edge_vectors.h`; check how they
    get `data.json`).

**`ecosystem.md`**: iscc-core Coverage → the 9 `gen_*_v0` functions with `data.json` vectors;
iscc-core-ts → "all 9 standard `gen_*_v0` functions" (drop "9 of the 10"); "Contributing" item 1 → 9
functions. iscc-lib section: intro and Language row name all 11 surfaces (Python, Node.js, WASM, C,
C++, Java, Go, Ruby, C#, Swift, Kotlin); Packages row lists every registry (or says "among others");
Coverage → 9 standard functions plus `gen_sum_code_v0`.

**Return-type sentences**: rewrite so the nine string-returning functions are named as such and
Sum-Code's type is named: Node `NapiSumCodeResult`, Java `SumCodeResult` (both `howto/java.md` and
`java-api.md`), C `iscc_IsccSumCodeResult` freed with `iscc_free_sum_code_result` (C names keep the
`iscc_` prefix — `tests/test_c_ffi_docs.py` gates it). Verify `gen_iscc_id_v1`'s return type on each
surface before any sentence lumps it in. Line-wrap at 100 (mdformat reflows).

## Verification

- `git grep -nE '10 .gen_' -- docs/ecosystem.md docs/architecture.md` prints nothing
- `git grep -nE 'All 10 .*(return a .String|as a string|heap-allocated ISCC string|return an ISCC string)' -- docs/howto/nodejs.md docs/howto/java.md docs/java-api.md docs/c-ffi-api.md`
    prints nothing, and each of those four "Code generation" paragraphs names its Sum-Code result
    type
- `docs/architecture.md` "Crate Summary", "Per-Binding Adaptation" and "Cross-Language Test Matrix"
    each have a row for Ruby, C#/.NET, C++, Swift and Kotlin; no `iscc-ffi` row says `Source`; the
    WASM matrix row names `wasm-pack test`
- `uv run pytest tests/test_c_ffi_docs.py` passes and `uv run scripts/check_tier1_docs.py` exits 0
- `uv run zensical build` exits 0
- `mise run check` exits 0

## Done When

All six verification criteria pass with only the six listed docs files modified.
