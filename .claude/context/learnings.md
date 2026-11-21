# Learnings

High-signal pitfalls, patterns and verified conventions from CID iterations. The review agent
maintains this file — append, prune, and archive completed-phase entries to `learnings-archive.md`.
**Size budget: keep under 200 lines**; over that, archive entries about fully-met target sections.

## Architecture

- Hub-and-spoke, API tiers → the `CLAUDE.md` files. `packages/go` is a pure-Go port, NOT a binding
- **Binding input-marshalling traps**: WASM `serde_wasm_bindgen` `Vec<T>` accepts ANY iterable (napi
    `Vec<String>` throws `ArrayExpected`): never document "throws on non-array" for WASM (204).
    **The FFI's `CStr` truncates at an embedded NUL**, so both C-FFI wrappers reject it first (213):
    .NET strings go through `ToNativeUtf8`, C++ through `detail::checked_c_str`; probe
    `x + "\0junk"` on every string input. JNI, Ruby + UniFFI keep the NUL, the core rejects. **An
    EMPTY container marshals to NULL** (`fixed` over `[]`, `.data()` on `{}`); the FFI reads
    `(NULL, 0)` as empty via `slice_or_empty` (255): `PinnedRows` padding and C++ `safe_data` are
    redundant. **Never take `Vec<T>` from Ruby**: Magnus `to_vec` holds a slice, so a shrinking
    `to_str` segfaulted (212); use `array_to_vec`. `RBignum::to_i32` truncates negative Bignums:
    `i32` only via `array_to_i32_vec`
- **WASM marshalling (268/269)**: the wasm-bindgen glue converts every typed parameter. Numbers and
    `bool` arrive as `JsValue` and are type-checked in Rust; `&[u8]` still converts and a number for
    a `&str` traps. **An `Option<T>` before a `JsValue` override loses its `?` in the `.d.ts`**
    unless it has its own `unchecked_optional_param_type`: diff the WHOLE built `.d.ts` against the
    previous build. **A js-sys import without `catch` lets a JS exception unwind through Rust
    frames** (4,000 to 8,000 throws kill the instance): read via `Reflect::get` (2× marshalling
    cost, decisions.md); `serde_wasm_bindgen::from_value` still has it. Probe a RELEASE build from
    Node. **Two `git archive` trees on ONE `CARGO_TARGET_DIR` package the first tree's wasm twice**
- **Binding fan-out traps** (IDv1 177-190 → `learnings-archive.md`): **`iscc-ffi`'s csbindgen
    `build.rs` rewrites tracked `NativeMethods.g.cs` on EVERY build** (pre-push clippy: "files were
    modified") — regen+commit it with `iscc.h` in the SAME FFI-symbol step; a `///`-only edit on an
    export moves BOTH (205). **C docs name types `iscc_X`** (cbindgen `[export] prefix`), guarded by
    `tests/test_c_ffi_docs.py`; `cbindgen --crate iscc-ffi` loads the crate's `cbindgen.toml`
    itself, so `--config` changes nothing (221)

## Reference Implementation

- **`iscc-core` output is not stable across CPython versions** (5,185 code points differ 3.13 vs
    3.14 — `text_clean`/`text_collapse` strip `C` incl. unassigned `Cn`, so output tracks
    `unicodedata.unidata_version`; iscc-core#137). Always name the interpreter (`--python 3.13`)
- **iscc-core 1.5.0 has two builds that can disagree** (264: compiled wheel cp310 to cp314, pure
    Python on 3.15): probe a vector under `uv run --no-project --python` 3.10 AND 3.15
- **String-input divergences from iscc-core** (decisions of 2026-11-18): the reference's reject set
    differs per entry point, so probe each → `learnings-archive.md` (268)

## Tooling

- Never use `mise` in CI: call the tools directly. Tools, tasks and hook stages → root `CLAUDE.md`
- **Generator-only Python deps go in a PEP 723 script, never in `[dependency-groups]`** (133):
    inline `# /// script` metadata, `uv run --script <path>`, `[tool.ty.src] exclude` **only if it
    imports a non-project dep** (159). A dep a *pytest* test imports in-process must be a dev-group
    dep (142, `pyyaml`). Generated Rust must be data-only + rustfmt-stable; generated C must be
    ASCII + LF + one trailing newline, else prek hygiene hooks rewrite it and break the
    regeneration-no-op gate — a **third-party** generator you can't fix (uniffi bindgen, 172) always
    trips it, so verify no-op modulo `sed 's/[[:space:]]*$//'`, never `git status`. A `///` edit on
    a uniffi export moves its checksum constant in both outputs (272): regenerate them together
- **C/C++ read `data-edge.json` from a generated header** (219): `gen_data_edge_vectors.py` renders
    `crates/iscc-ffi/tests/data_edge_vectors.h`; a vector = JSON edit + regen. **A new section is
    one unsplittable 12-surface fan-out** (226): each harness and the generator fail closed on it.
    Checks over generated source tokenize escapes, split on lines, and tally visited rows. **A
    harness's uncompared set holds EVERY `data.json` output key its result type lacks** (252:
    `description` missed); a `meta` input is a JSON object (iscc-core reads a string `meta` as a
    data URL). A lone surrogate escape is no vector: serde_json fails on the file (266: Go test)
- **Perf-gate tooling is NOT in the devcontainer** (250): `mise run bench:iai:check` needs
    `apt-get install valgrind` + `cargo install --locked iai-callgrind-runner@0.16.1`
- **A docs page lives in FOUR places**, all gated by `scripts/check_docs_nav.py` (145/146).
    **`zensical build` wipes `site/`, so `gen_llms_full.py` MUST run after it**
- **Devcontainer push**: `git -c credential.helper='!gh auth git-credential' push`

## ISCC Algorithm Knowledge

- **Unicode 16.0.0 pin REMOVED 2026-07-30** (decisions.md): never resurrect a freeze mechanism
- **A data-driven fixture is self-referential — assert CONTENT, not shape** (141): ungate guards on
    version, case counts, code points + skip-list keys (ASCII no-op swaps stay green forever). A
    test that iterates the code's OWN table (`for mt in seqMainTypes`) cannot see it shrink — pin it
    **Per-algorithm internals**, normalization order, `data.json` counts, API-parameter facts,
    **ISCC-IDv1** details, the three codec rules → `learnings-archive.md`
- **Integer widths differ per platform**: Go `int(x)` of a `uint32` wraps on 386 (263: subtract
    unsigned, then `clampInt`); CI runs the Go suite as `GOARCH=386` too (267), 386 `vet` sees no
    wrap. C `long` is 32 bits on Windows (265: `PyLong_AsLongLong`; no Windows CI, prove it from the
    type). Rust `u64 as usize` truncates on 32-bit Android (272: `usize::try_from` + saturate); only
    `cargo test -p iscc-uniffi --target i686-unknown-linux-musl` sees it, and no CI job runs it

## CI/CD

- **A binding suite's runner is not `cargo test`** (151): `cargo test -p iscc-wasm` reports
    `0 passed`; only `wasm-pack test --node …` runs `#[wasm_bindgen_test]`. **Gitignored native
    artifacts go stale silently** (Ruby `.so`, napi `.node`, JNI `.so`): rebuild before probing;
    after a dep bump `strings <artifact>` shows the dep version linked in (167). Local-only
- **The JVM type-checks nothing a native method returns** (168): an `Object[]` returned where
    `String[]` is declared survives every element read — assert `getClass().getName()` and probe
    under `JAVA_TOOL_OPTIONS=-Xcheck:jni mvn -f crates/iscc-jni/java/pom.xml test` (log shows
    `Picked up JAVA_TOOL_OPTIONS` twice). **`-DargLine=…` is a no-op** (208, `mvn -X`): the pom's
    `<argLine>` beats the user property, so all earlier `-DargLine` "zero warnings" runs were
    vacuous. CI runs no `-Xcheck:jni`. **jni 0.22 (230)**: `*_with_catch*` calls (`new_string`,
    `find_class`) CLEAR the JVM exception (OOM → `NoMemory`), `post_check_ex` ones (`new_object`)
    leave it pending. HotSpot 17 `-Xcheck:jni` is SILENT on `ThrowNew` over a pending exception:
    assert the class. Trigger: shadow result class with a throwing ctor first on a child `-cp`
- **Release pipeline** + `version_sync.py`'s targets → `learnings-archive.md`. Its doc targets are
    the Maven/Gradle/SPM snippets only: any other version in docs is hand-typed, so write none (244)
- **`release.yml` is `workflow_dispatch`-only — no CI run and no CID push ever exercises it.** Its
    invariants are gated: `scripts/check_release_workflow.py` (prek hook + tests) and the CI-only
    `--check-action-inputs`; never hand-retype either. **`prepare-release` never moves an existing
    tag** (259): `fetch-depth: 0` fetches every tag, `git tag … || echo` keeps it and the
    `push --force` is a no-op, so a hand-pushed tag stays on the pre-checksum commit
- **`setup-dotnet` installs the latest LTS runtime before each requested version** (260), a private
    `DOTNET_INSTALL_DIR` included; `skip-lts-prepass: true` stops it (262: SDK 10 still drives)
- **A docs install claim is a claim about the PUBLISHED artifact — install it** (238): source gems
    up to 0.7.0 do not build. Probe: `gem fetch iscc-lib --platform ruby`, then
    `GEM_HOME=/tmp/x GEM_PATH=/tmp/x gem install <file> --local`. **A platform-fallback claim needs
    a foreign-platform run** (257): `java -Dos.arch=riscv64` → `NativeLoader` throws pre-fallback
- **Ruby source gem = `scripts/build_ruby_source_gem.py` only** (239-241). `cross-gem@v1` reads the
    rb_sys pin UNANCHORED (first `Gemfile.lock` line containing `rb_sys`), so the gemspec declares
    `rb_sys` only next to `core/`. The staged manifest is its own workspace: of the root, only
    `[profile.release]` reaches it (copied key lines, LTO install ≈ 55 s). Tests need `ruby` on PATH
- **Ruby floor is 3.3 (270); the dev Ruby is the `mise.toml` `[tools]` pin (271)**: run
    `mise exec -- …` in `crates/iscc-rb` (floor: `mise exec ruby@3.3 -- …`); the pin puts that Ruby
    first on PATH repo-wide, ahead of system 3.1. **Never `rake clobber`: it empties the workspace
    `target/`.** The `lib/` `.so` fits one Ruby series: delete it and `tmp/` to switch
- **CRAP gate (ci-cd.md)**: ENFORCING — CI runs `cargo crap` with `--fail-regression` and a **bare**
    `--fail-above` (30.0 lives in `.cargo-crap.toml`, so `--fail-above 30.0` is a syntax error);
    baseline is COMMITTED (`mise run crap:baseline` after `mise run coverage`). **CI-ONLY gap:** a
    new branch in a covered fn — or merely **moving lines below the edit point** — is green locally,
    red in CI unless the baseline moves in the SAME step (never widen epsilon/threshold). **A new
    stable rustc re-scores untouched fns** (195): read the toolchain in CI's CRAP log before blaming
    the diff. **Rust test helpers (`crates/iscc-lib/tests/*.rs`) score at 0% coverage**
    (pessimistic; CRAP = cc² + cc): cc > 5 fails `--fail-above` in CI only (215: 210). Fix = a
    `const` table of small `fn` callers (216). cargo-crap 0.2.2 counts `?`, `||`, `if`, top-level
    `match` arms; NOT closure bodies, `let … else`, `.then()`. Run the CI CRAP lines on `tests/*.rs`
- **`Perf (iai-callgrind)` gate — ENFORCING (#3)**: `[profile.bench] strip = false, debug = true` is
    load-bearing (stripped → `summary: 0` false-green). **NOT in `mise run check`** — run
    `mise run bench:iai:check` on any benchmarked hot-path change (191's +36% reached red CI). A
    correctness change to a hot path can raise cost irreducibly: regen the baseline in the SAME step
- **`Audit (cargo-deny)` gate — ENFORCING**: root `deny.toml` (v2, `yanked = "deny"`, `ignore` list
    with a per-entry justification) + `audit` CI job (`cargo-deny@0.19.9`, not in the devcontainer —
    install the prebuilt release) + `mise run audit`; reads Cargo.lock so green locally is
    authoritative. A yanked crate/fresh RustSec advisory reds it on ANY push with no code change —
    try `cargo update -p <crate>` FIRST; ignore only when `cargo update --dry-run` keeps the crate
    AND the feature edge pinning it is proven at the latest upstream (196, reasons in `deny.toml`)
- **`cargo deny` is not the only advisory source** (235): it was green with rustix 1.1.4 while osv
    listed RUSTSEC-2026-0486. One pass over the tracked lockfiles, run from an empty `/tmp` dir:
    `go run github.com/google/osv-scanner/v2/cmd/osv-scanner@latest scan source -r <repo>` (~1 min;
    Cargo, uv, Gemfile, Maven, .NET, Go — NOT npm, Gradle or Actions pins). Expected: exit 1 with
    exactly the 3 unmaintained crates that `deny.toml` ignores
- **A prek `types:` tag is not a file-extension guess — probe it** with a **staged, deliberately
    dirty** file (`uv run prek run <hook> --files <p>`; `Skipped` = tag misses). `.pyi` is tagged
    `pyi`, not `python` (that hole skipped the published `_lowlevel.pyi`, closed 139). **A
    `files:`-scoped hook never sees deletions** — pair any consistency hook with a pytest anchor
    test against the real tree. Formatter caveats → `learnings-archive.md`
- **JVM/Gradle gotchas** (128, 165) → `learnings-archive.md`, read before touching `pom.xml`/`.kts`
- **Prove a new gate with a REAL regression in a THROWAWAY repo, not a synthetic typo** (iters
    144/152/163): `git archive HEAD | tar -x -C /tmp/x && git init` gives a probe tree where
    `git add`/`git rm` are free; the cheapest *real* regression is the guarded file's own previous
    version (`git show HEAD~1:<path>`). A **set-equality** gate is blind twice: equal *empty* sets
    pass (hence a count floor) and a **duplicated** row passes (keep row order to count repeats —
    163). A **test-framework major** is the same: a "≥ N passed" floor cannot see a silent collapse
    of parameterized rows (164: xunit v2/v3 both gave 104), so demand the SAME total as the pre-bump
    tree (`git archive HEAD~1`) or derive it from the FIXTURE (166: `data.json` vector counts +
    static `@Test` count). **`mvn test` reuses stale test classes** ("Nothing to compile"), so only
    `mvn clean test` proves the new framework compiles

## CID Process

- Never force-push `develop`. **Verify external state at the source** (`npm view`, `gh api`, …)
- **Pre-push mdformat runs over every file in the push range** (`next.md`, `MEMORY*.md` too): one
    non-conforming file rejects the batch although `git commit` passed. Reformat + amend
- **Never write an exact count, substring `grep -c`, unverified CLI flag, or unverified
    `#[deprecated]` claim into a criterion** (139, 148, 168: a "must not appear" grep banned an
    *undeprecated* API, reverted 169). Confirm attributes in the crate source under
    `~/.cargo/registry/src/`; assert the *gate* (exit code); copy gate lines from `ci.yml`
- **next.md's Implementation Notes are a hypothesis, not a spec — algorithms *and* prose alike**
    (142: an unworkable rule; 143: two false claims shipped into published docs). advance implements
    the *intent* and documents any deviation; review re-derives every "never/always" claim
- **One bad case per call when testing failure arms** (195): a negative test that feeds a mismatch,
    an error and a parse failure to ONE runner call stays green even if the mismatch arm breaks —
    any other bad case still fails the call. **Go `AllocsPerRun(…, func(){ _ = f() })` reads 0 even
    when `f` allocates** (the copy is optimized away) — assign to a package-level sink (202). And
    check that each "mismatch" fixture reaches the comparison: `ISCC:GAAAAAAAAAAA` (7 bytes) failed
    earlier, with "Trailing data". Positive mirror: one input form per trim site (200: outer-only
    forms missed the post-colon trims). **Assert a rejection on the raw API call alone** (215):
    `pytest.raises(ValueError)` around a caller that also converts the result (`dict()`, `int()`)
    stays green when the binding RETURNS garbage, because those conversions raise `ValueError` too.
    **And compare EVERY expected output key on a full-result surface** (217: Ruby's
    `actual.keys & expected.keys` passed a result without `iscc`); a reduced result leaves only a
    STATIC key set uncompared, gen `outputs` keys ⊆ the function's `data.json` keys (236: validator
    \+ C generator). `outputs: {}` is rejected. Swift `String ==` is canonical equivalence: compare
    `unicodeScalars`. Parsed `NSNumber` casts are loose: read via `DataEdge.scalar` (232)
- **A differential gate's power lives in its CASE SET, not its case COUNT** (157): a pin on the case
    total catches a *shrunken* sweep, not a *swapped* one — prove it against a **superseded real
    design**, assert which rows light up, and commit that as a test. A one-way sweep proves only ⊇:
    "exactly" needs the reverse too, candidates derived live (198: `Z*`/`Cc`/`Cf` − strip set)
- **Spec paths are gated** (224/228/247): `scripts/check_spec_paths.py` checks tree entries +
    backticked paths (`.github/` too) against `git ls-files` (never disk) and fails on a tree whose
    next paragraph lacks "not every file". Specs and per-crate `CLAUDE.md` files hold NO complete
    file list: point to the code's own list (`TARGETS`, 247). `*` = one segment, `**` crosses `/`.
    **A negative assertion must reach the branch it names** (228). **Removing a derived fact**: join
    lines (`tr '\n' ' '`); sweep for table rows that start with a backticked path (247: one missed)
- **Tier 1 doc parity is gated** (233): `scripts/check_tier1_docs.py` derives the symbols from
    `lib.rs`, NAMES only; an unparsable crate-root `pub` fails (242): add a derive rule or an
    `EXCLUDED_ITEMS` entry. `rust-api.md` signatures: `tests/test_rust_api_signatures.py`; doc
    examples compile in a `/tmp` cargo project (243), C++ `main()` blocks with `g++` (244)
