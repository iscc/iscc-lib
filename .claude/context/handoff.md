## 2026-11-21 — Docs accuracy batch: architecture.md covers every surface, no "10 standard functions" or Sum-Code-as-string claims

**Done:** `docs/architecture.md` now has Ruby, C#, C++, Swift and Kotlin (plus `iscc-uniffi`) in the
workspace tree, Crate Summary, Per-Binding Adaptation and Cross-Language Test Matrix, with release
channels and test runners taken from `release.yml`/`ci.yml`. `ecosystem.md` counts the 9 standard
`gen_*_v0` functions and names all 11 surfaces. The four "Code generation" paragraphs name the
Sum-Code result type instead of claiming all 10 functions return a string.

**Files changed:**

- `docs/architecture.md`: tree marked as excerpt, one line per crate/package dir (Go file list
    collapsed), `release.yml` comment "every registry"; Crate Summary 13 rows (`iscc-ffi` → GitHub
    Release assets `iscc-ffi-v<version>-<target>`, from `release.yml:738`); 5 new adaptation rows;
    "How It Works" item 2 reworded (vendored copies for .NET/Swift/Kotlin/Go selftest, C++ via
    `conformance_selftest`, C via expected values in the test), item 3 → 9 functions + seq, one
    sentence on `data-edge.json`; matrix 12 rows, WASM = `wasm-pack test --node`
- `docs/ecosystem.md`: iscc-core / iscc-core-ts / Contributing → 9 standard functions; iscc-lib
    intro + Language row list all 11 surfaces; Packages row adds Maven Central, RubyGems, NuGet, Go
    module "among others"; Coverage → 9 standard functions plus `gen_sum_code_v0`
- `docs/howto/nodejs.md`: `gen_sum_code_v0` returns `NapiSumCodeResult` (`index.d.ts:193`)
- `docs/howto/java.md`, `docs/java-api.md`: `genSumCodeV0` returns `SumCodeResult`
    (`IsccLib.java:161`); java-api also names `genIsccIdV1` as string-returning (`IsccLib.java:172`)
- `docs/c-ffi-api.md`: `iscc_gen_sum_code_v0` returns `iscc_IsccSumCodeResult`, freed with
    `iscc_free_sum_code_result()`; `iscc_gen_iscc_id_v1` named as `char*` (`lib.rs:660-666`)

**Verification:**

- `git grep -nE '10 .gen_' -- docs/ecosystem.md docs/architecture.md` → no output
- The `All 10 .*(return a .String|…)` grep over the four pages → no output; each paragraph names its
    Sum-Code type (diff above)
- `git grep -n Source docs/architecture.md` → no output; WASM matrix row names `wasm-pack test`
- `uv run pytest tests/test_c_ffi_docs.py` → 6 passed; `uv run scripts/check_tier1_docs.py` → exit 0
    ("20 full and 2 exact Tier 1 docs match")
- `uv run zensical build` → exit 0, "No issues found"
- `mise run check` → exit 0
- Only the six docs files are modified (`git status --short`, plus the runner's `iterations.jsonl`)

**Next:** the `[audit]` WASM Node.js docs issue (`docs/howto/wasm.md:82` carries the same "all 10
return a string" sentence, and `crates/iscc-wasm/README.md`), with its `--target web` release-build
probe.

**Notes:**

- Facts checked against source: Ruby hashers chain `update` and `finalize(bits:)`
    (`lib/iscc_lib.rb`); .NET `IsccDataHasher` is `IDisposable` over a `SafeHandle`; C++ hashers are
    move-only RAII (`iscc.hpp:640-715`); Swift UniFFI classes free on `deinit`; Kotlin classes are
    `AutoCloseable`; no binding exposes async (`grep 'async \|Task<'` empty). Python GIL claim kept
    (`py.detach` in all three `update`s, `iscc-py/src/lib.rs:643/692/743`)
- `howto/java.md:74` still says "All 10 `gen*V0` methods are static … and throw
    `IllegalArgumentException`" — a library-wide count, correct (`JavaError` maps every non-OOM
    error to it)
- Pre-existing, left as is: the C FFI adaptation row uses generic `ctx_new()/ctx_update()…` names
    rather than the real `iscc_data_hasher_new` etc. Not a count/return-type claim, so out of scope
- The ecosystem Packages row is long (8 links); next.md allowed "among others", which it ends with
