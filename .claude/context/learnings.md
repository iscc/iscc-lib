# Learnings

High-signal pitfalls, patterns and verified conventions from CID iterations. The review agent
maintains this file — append, prune, and archive completed-phase entries to `learnings-archive.md`.
**Size budget: keep under 200 lines**; over that, archive entries about fully-met target sections.

## Architecture

- Hub-and-spoke: `iscc-lib` (pure Rust core) → binding crates (py, napi, wasm, ffi, jni, rb,
    uniffi), each depending only on the core. Tier 1 = 35 symbols `pub use`d at crate root (IDv1
    177, `encode_seq`/`decode_seq` 195); Tier 2 is `pub(crate)`, never crosses FFI. `packages/go` is
    NOT a binding — a pure-Go reimplementation
- **Binding input-marshalling traps**: WASM `serde_wasm_bindgen` `Vec<T>` accepts ANY iterable (napi
    `Vec<String>` throws `ArrayExpected`): never document "throws on non-array" for WASM (204).
    **The FFI's `CStr` truncates at an embedded NUL**, so both C-FFI wrappers reject it first (213):
    .NET strings go through `ToNativeUtf8`, C++ through `detail::checked_c_str`; probe
    `x + "\0junk"` on every string input. JNI, Ruby + UniFFI keep the NUL, the core rejects. **An
    EMPTY container marshals to NULL** (`fixed` over `[]`, `.data()` on `{}`), so the FFI's "must
    not be NULL" hides the core's error: .NET jagged inputs pin only via `PinnedRows` (220), C++
    vectors only via `detail::safe_data` (227, `tests/test_cpp_safe_data.py`). **Never take `Vec<T>`
    from Ruby**: Magnus `to_vec` holds a slice, so a shrinking `to_str` segfaulted (212); use
    `array_to_vec`. `RBignum::to_i32` truncates negative Bignums: `i32` only via `array_to_i32_vec`
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
- A dependency shipping DATA TABLES (Unicode, locale, tz) can change output without any `data.json`
    vector catching it (every vector predates Unicode 16) — call out table-version changes in dep
    bumps explicitly
- **String-input divergences from iscc-core** hit any differential on *string* inputs (`encode_seq`,
    `iscc_decode`): (1) strict base32 by decision (2026-11-18), padding + spare bits rejected, Rust
    \+ Go pinned (199); a padding negative must be validly padded (`ISCC:GAAAAAAAAA======`:
    `EEAY…====` fails in the ref too); (2) Python `strip()`/`split()` also take U+001C–U+001F: Rust
    codec fixed (198), Go codec (200: Go `unicode.IsSpace` + U+001C–U+001F IS the `strip()` set, 29
    code points, probed both ways), text utils (203); (3) ASCII-only upper-casing (201) rejects
    `ı`/`ſ`/`ß`/`ﬁ`: ref `iscc_decode` rejects too, but ref `encode_seq` Unicode-`upper()`s first
    and ACCEPTS them — only those whose `upper()` is ASCII (`ẞ` U+1E9E fails in both, 222) — **the
    reference's reject set differs per entry point; probe each before calling a fix
    "reference-matching"**. A bare `b…` string reads as plain base32 in BOTH: not a defect

## Tooling

- `mise` for tools/tasks, `uv` for the Python env, `prek` for hooks; never use `mise` in CI — call
    tools directly. Pre-push-**only** gates: clippy `-D warnings`, cargo test, pytest, `ty check`
- **Generator-only Python deps go in a PEP 723 script, never in `[dependency-groups]`** (133):
    inline `# /// script` metadata, `uv run --script <path>`, `[tool.ty.src] exclude` **only if it
    imports a non-project dep** (159). A dep a *pytest* test imports in-process must be a dev-group
    dep (142, `pyyaml`). Generated Rust must be data-only + rustfmt-stable; generated C must be
    ASCII + LF + one trailing newline, else prek hygiene hooks rewrite it and break the
    regeneration-no-op gate — a **third-party** generator you can't fix (uniffi bindgen, 172) always
    trips it, so verify no-op modulo `sed 's/[[:space:]]*$//'`, never `git status`
- **C/C++ read `data-edge.json` from a generated header** (219): `gen_data_edge_vectors.py` renders
    `crates/iscc-ffi/tests/data_edge_vectors.h`; a vector = JSON edit + regen. **A new section is
    one unsplittable 12-surface fan-out** (226): each harness and the generator fail closed on it.
    Checks over generated source tokenize escapes, split on lines, and tally visited rows
- **Perf-gate tooling installs on demand, NOT in the devcontainer**: `mise run bench:iai:check` dies
    until `apt-get install valgrind` + `cargo binstall iai-callgrind-runner@0.16.1` (pin-matched)
- **A docs page lives in FOUR places** — disk (`docs/**/*.md` minus `includes/`), `zensical.toml`
    `nav`, `ORDERED_PAGES`, `docs/llms.txt` (23 pages) — all gated by `scripts/check_docs_nav.py`
    (145/146). **`zensical build` wipes `site/`, so `gen_llms_full.py` MUST run after it**
- **Devcontainer push**: `git -c credential.helper='!gh auth git-credential' push`

## ISCC Algorithm Knowledge

- **Unicode 16.0.0 pin REMOVED 2026-07-30** (decisions.md, human): `text_clean`/`text_collapse`
    mirror the reference on whatever tables dependencies ship; every freeze-era mechanism and note
    (sentinel, frozen `Final_Sigma`, boundary vectors, sweep gate) is historical — never resurrect
- **A data-driven fixture is self-referential — assert CONTENT, not shape** (141): ungate guards on
    version, case counts, code points + skip-list keys (ASCII no-op swaps stay green forever). A
    test that iterates the code's OWN table (`for mt in seqMainTypes`) cannot see it shrink — pin it
    **Per-algorithm internals**, normalization order, `data.json` counts, API-parameter facts,
    **ISCC-IDv1** details, the three codec rules → `learnings-archive.md`

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
- **Release pipeline** + `version_sync.py`'s 21 targets → `learnings-archive.md`. `@iscc/wasm`
    metadata = wasm-pack 0.13.1 output of its `Cargo.toml` (214); no `description` → badge line
- **`release.yml` is `workflow_dispatch`-only — no CI run and no CID push ever exercises it.** Its
    invariants are executable gates since iters 142/144/146: `scripts/check_release_workflow.py`
    (guard shape, artifact wiring, `needs:` graph; prek hook +
    `tests/test_check_release_workflow.py`) plus the CI-only bidirectional `--check-action-inputs`.
    Never hand-retype either into a heredoc
- **A docs install claim is a claim about the PUBLISHED artifact — install it** (238): source gems
    up to 0.7.0 do not build. Probe: `gem fetch iscc-lib --platform ruby`, then
    `GEM_HOME=/tmp/x GEM_PATH=/tmp/x gem install <file> --local`
- **Ruby source gem = `scripts/build_ruby_source_gem.py` only** (239-241). `cross-gem@v1` reads the
    rb_sys pin UNANCHORED (first `Gemfile.lock` line containing `rb_sys`), so the gemspec declares
    `rb_sys` only next to `core/`. The staged manifest is its own workspace: of the root, only
    `[profile.release]` reaches it (copied key lines, LTO install ≈ 55 s). Tests need `ruby` on PATH
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
    (191/192 `iscc_clean` detail → `learnings-archive.md`)
- **`Audit (cargo-deny)` gate — ENFORCING**: root `deny.toml` (v2, `yanked = "deny"`, `ignore` list
    with a per-entry justification) + `audit` CI job (`cargo-deny@0.19.9`, not in the devcontainer —
    install the prebuilt release) + `mise run audit`; reads Cargo.lock so green locally is
    authoritative. A yanked crate/fresh RustSec advisory reds it on ANY push with no code change —
    try `cargo update -p <crate>` FIRST; ignore only when `cargo update --dry-run` keeps the crate
    AND the feature edge pinning it is proven at the latest upstream (196: uniffi_bindgen 0.32.2
    hard-enables askama `config` → `basic-toml`; crates.io `/dependencies` shows `features`)
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
- **JVM/Gradle gotchas** (128) → `learnings-archive.md`, read before touching `pom.xml`/`.kts`. A
    *concurrent* Gradle 9 build (background Codex) makes `clean` fail "Unable to delete" (165)
- **Prove a new gate with a REAL regression in a THROWAWAY repo, not a synthetic typo** (iters
    144/152/163): `git archive HEAD | tar -x -C /tmp/x && git init` gives a probe tree where
    `git add`/`git rm` are free; the cheapest *real* regression is the guarded file's own previous
    version (`git show HEAD~1:<path>`). A **set-equality** gate is blind twice: equal *empty* sets
    pass (hence a count floor) and a **duplicated** row passes (keep row order to count repeats —
    163). A **test-framework major** is the same: a "≥ N passed" floor cannot see a silent collapse
    of parameterized rows (164: xunit v2/v3 both gave 104), so demand the SAME total as the pre-bump
    tree (`git archive HEAD~1`) or derive it from the FIXTURE (166: `data.json` vector counts +
    static `@Test` count). **`mvn test` reuses stale test classes** ("Nothing to compile"), so only
    `mvn clean test` proves the new framework compiles. A **bench-harness** major:
    `cargo bench --no-run` only links — `cargo bench -p iscc-lib --bench benchmarks -- --test` runs
    each body once (171)

## CID Process

- Never force-push `develop` in a loop. **Never trust state.md/handoff claims about external state**
    (registries, CI, upstream tags): verify at the source (`npm view`, `gh api`, …)
- **Pre-push mdformat blocks on non-conforming context files**: it runs over every file in the push
    range (incl. `next.md`, per-agent `MEMORY*.md`), so one non-conforming file rejects the batch
    although `git commit` passed. Unblock by reformatting + amending
- **Never write an exact count, substring `grep -c`, unverified CLI flag, or unverified
    `#[deprecated]` claim into a criterion** (139: `ruff format --check` saw 155 not 153; 148:
    `--fail-above 30.0` is a `cargo crap` syntax error; 168: a "must not appear" grep banned the
    *undeprecated* `Env::byte_array_from_slice`, reverted 169). Confirm attributes in
    `~/.cargo/registry/src/*/<crate>-<ver>/`; assert the *gate* (exit code); copy gate lines from
    `ci.yml`
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
- **Spec paths are gated** (224/228): `scripts/check_spec_paths.py` checks tree entries + backticked
    paths against `git ls-files` (never disk); `*` stays in one segment, `**` crosses `/`. Fix the
    spec, or list build output in `GENERATED`. Conan generates `iscc.pc` / `find_package(iscc)` at
    install time. **A negative assertion must reach the branch it names** (228: no `*`, no regex)
- **Tier 1 doc parity is gated** (233): `scripts/check_tier1_docs.py` derives the symbols from
    `lib.rs`; NAMES only, a return type needs a run. A crate-root `pub` form it cannot parse fails
    (242): add a derive rule or a named `EXCLUDED_ITEMS` entry. **`#[wasm_bindgen_test]` calls the
    Rust fn, never the JS export glue** (234): prove a JS return shape from Node (`require()`) and
    the `.d.ts`, built by `wasm-pack build --target nodejs crates/iscc-wasm --out-dir /tmp/x`
