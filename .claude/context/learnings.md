# Learnings

High-signal pitfalls, patterns and verified conventions from CID iterations. The review agent
maintains this file — append, prune, and archive completed-phase entries to `learnings-archive.md`.
**Size budget: keep under 200 lines**; over that, archive entries about fully-met target sections.

## Architecture

- Hub-and-spoke: `iscc-lib` (pure Rust core) → binding crates (py, napi, wasm, ffi, jni, rb,
    uniffi), each depending only on the core. Tier 1 = 33 symbols `pub use`d at crate root (33rd =
    `gen_iscc_id_v1`+`IsccIdResult`, iter 177; docs/count sweep to 33 done, iter 190); Tier 2 is
    `pub(crate)`, never crosses FFI. `packages/go` is NOT a binding — a pure-Go reimplementation
- **ISCC-SEQ core (195)**: `encode_seq`/`decode_seq` at crate root (documented Tier-1 count stays 33
    until all 11 surfaces bind them → 35, one doc sweep); `read_unit`/`SEQ_MAINTYPES` Tier 2 on
    private raw-`u32` header helpers — never the typed `decode_header` path (Go `uint8` truncates
    SubType 4679 → 71, 202). Diff each port vs Rust via the Python binding on a corpus. **The spec
    round trip is BYTES — `encode_seq(decode_seq(x)) == x` over the `decode_seq` vectors** (205:
    next.md prescribed string decode-of-encode; py/napi/WASM/FFI assert bytes)
- **`encode_seq` upper-cases before cleaning, so `b` + base32(`cc01` + unit) can read as a DIFFERENT
    valid bare unit** (`bzqaq4yamaaqaaaaaaaaaaaaaaaaaaaa` → 20 bytes) — iscc-core 1.5.0 returns the
    same bytes; reference behaviour, not a defect (decisions.md 2026-11-18)
- **JS surfaces differ on list/buffer params** (204): WASM
    `serde_wasm_bindgen::from_value::<Vec<T>>` accepts ANY iterable (`Set`, generator), napi
    `Vec<String>` throws `ArrayExpected` — never doc "throws on non-array" for WASM. napi 3.11
    `Buffer::from_napi_value` takes a strong ref BEFORE `napi_get_buffer_info` and never frees it on
    failure, so a non-Buffer object arg leaks it
- **Binding fan-out traps (IDv1 177-190, detail → `learnings-archive.md`)**: a surface with its OWN
    enum (Python `VS`) can break a round trip the core allows; wide-int bindings check semantic
    thresholds in the binding before narrowing. **`iscc-ffi`'s csbindgen `build.rs` rewrites tracked
    `NativeMethods.g.cs` on EVERY build** → pre-push clippy fails "files were modified";
    regen+commit it (like `iscc.h`) in the SAME FFI-symbol step, never "the dotnet step". A
    `///`-only edit on an export (even a review doc fix) moves BOTH files too (205)

## Reference Implementation

- **`iscc-core` output is not stable across CPython versions** (5,185 code points differ 3.13 vs
    3.14 — `text_clean`/`text_collapse` strip `C` incl. unassigned `Cn`, so output tracks
    `unicodedata.unidata_version`; iscc-core#137). Always name the interpreter (`--python 3.13`)
- A dependency shipping DATA TABLES (Unicode, locale, tz) can change output without any `data.json`
    vector catching it (every vector predates Unicode 16) — call out table-version changes in dep
    bumps explicitly
- **A widened decode gate re-exposes latent truncation** (174/175): `decode_header`'s `as u8` before
    `TryFrom` let a multi-nibble value wrap (`257`→`1`, `262`→`Id`) so a malformed header
    canonicalized to valid; now `u8::try_from`-gated. Probe `"MDFZAAAAAAAAAAAAAA"` on header gates
- **`decode_base32("")` returns `Ok(empty)`, not an error** (191; empty guard in `iscc_clean` 192):
    a cleaning helper that can yield `""` (blank/dash-only/`"iscc:"`) must reject empty before
    decode, else `iscc_decompose` returns `Ok([])` where the ref IndexErrors. Probe empty/garbage on
    cleaning
- **iscc-core 1.5.0 `data.json`** (2026-10-17): `encode_seq`/`decode_seq` sections have non-object
    outputs and `bytes:<hex>` values — a harness walking every section breaks (Go now keeps `gen_*`)
- **String-input divergences from iscc-core** hit any differential on *string* inputs (`encode_seq`,
    `iscc_decode`): (1) strict base32 by decision (2026-11-18), padding + spare bits rejected, Rust
    \+ Go pinned (199); (2) Python `strip()`/`split()` also take U+001C–U+001F: Rust codec fixed
    (198), Go codec (200: Go `unicode.IsSpace` + U+001C–U+001F IS the `strip()` set, 29 code points,
    probed both ways), text utils (203); (3) ASCII-only upper-casing (201) rejects `ı`/`ſ`/`ß`/`ﬁ`:
    ref `iscc_decode` rejects too, but ref `encode_seq` Unicode-`upper()`s first and ACCEPTS them —
    **the reference's reject set differs per entry point; probe each before calling a fix
    "reference-matching"**. A bare `b…` string reads as plain base32 in BOTH, so not a defect

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
- **Perf-gate tooling installs on demand, NOT in the devcontainer**: `mise run bench:iai:check` dies
    until `apt-get install valgrind` + `cargo binstall iai-callgrind-runner@0.16.1` (pin-matched)
- **A docs page lives in FOUR places** — disk (`docs/**/*.md` minus `includes/`), `zensical.toml`
    `nav`, `ORDERED_PAGES`, `docs/llms.txt` (23 pages) — all gated by `scripts/check_docs_nav.py`
    (145/146). **`zensical build` wipes `site/`, so `gen_llms_full.py` MUST run after it**
- **Devcontainer** (2026-10-17): export `LANG=C.UTF-8` for local gates (Ruby aborts reading UTF-8
    fixtures otherwise); push via `git -c credential.helper='!gh auth git-credential' push`

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
    under `-Xcheck:jni` (`mvn test -DargLine="-Xcheck:jni -Djava.library.path=$PWD/target/debug"` —
    surefire's `argLine` is *overridden*, so re-supply the library path). All 33 have a caller since
    169
- **Release pipeline pattern** + `version_sync.py`'s 21 targets → `learnings-archive.md`
- **`release.yml` is `workflow_dispatch`-only — no CI run and no CID push ever exercises it.** Its
    invariants are executable gates since iters 142/144/146: `scripts/check_release_workflow.py`
    (guard shape, artifact wiring, `needs:` graph; prek hook +
    `tests/test_check_release_workflow.py`) plus the CI-only bidirectional `--check-action-inputs`.
    Never hand-retype either into a heredoc
- **A fail-open gate must publish a resolved/total counter** — without it "all checked" and "nothing
    checked" are the same green (read `action-inputs: resolved R of T`, not the job status), and
    "transport failure degrades to a warning" is NOT met by `except OSError` (`IncompleteRead` is an
    `HTTPException`, captive-portal HTML raises `yaml.YAMLError`)
- **A rustc floor travels the dependency graph — measure it, don't argue it** (171/172/173): floor
    IS the **max `rust_version` over the non-dev resolve graph**, one `cargo metadata --locked` walk
    (`cargo tree -i <dep> -e no-dev --target all` printing nothing proves a major is dev-only;
    without `--target all` it hides platform-gated transitives). uniffi 0.32's
    `cargo-platform 0.3.3` (1.91) broke `iscc-uniffi` on inherited 1.85 while `-p iscc-lib` stayed
    fine; a per-crate `rust-version` override (173) makes cargo say `requires rustc 1.91` not a
    resolution error. Settle by building — **1.85 is installed**:
    `cargo +1.85.0 check -p <c> --locked`
- `semver` (informational pre-1.0) + `coverage` (enforcing) CI jobs: `mise run semver`/`coverage`
- **CRAP gate (ci-cd.md)**: ENFORCING — CI runs `cargo crap` with `--fail-regression` and a **bare**
    `--fail-above` (30.0 lives in `.cargo-crap.toml`, so `--fail-above 30.0` is a syntax error);
    baseline is COMMITTED (`mise run crap:baseline` after `mise run coverage`). **CI-ONLY gap:** a
    new branch in a covered fn — or merely **moving lines below the edit point** — is green locally,
    red in CI unless the baseline moves in the SAME step (never widen epsilon/threshold). **A new
    stable rustc re-scores untouched fns** (195: 1.99.1 → 1.100.0 lowered 11 entries, raised 3): CI
    uses `@stable`, so read the toolchain in CI's CRAP log before blaming the diff
- **`Perf (iai-callgrind)` gate — ENFORCING (#3)**: `[profile.bench] strip = false, debug = true` is
    load-bearing (stripped binary → all benches `summary: 0` false-green) → `learnings-archive.md`.
    **NOT in `mise run check`** — a benchmarked hot-path change (codec `iscc_clean`, text utils)
    needs `mise run bench:iai:check` run explicitly (iter 191's +36% four_units regression slipped
    past a green `check` into red develop CI). A committed baseline reflects the code AT baseline
    time: a *correctness* change to a hot path can raise cost irreducibly (191/192 reference-correct
    `iscc_clean` = two std `memchr` scans `split_once(':')`+`contains('-')`; a scalar single-pass is
    *slower*, a single SIMD pass needs the out-of-scope `memchr` crate), so removing allocations
    alone won't restore a baseline set for cheaper-but-wrong code — regen the baseline in the SAME
    step
- **`Audit (cargo-deny)` gate — ENFORCING**: root `deny.toml` (v2, `yanked = "deny"`, `ignore` list
    with a per-entry justification) + `audit` CI job (`cargo-deny@0.19.9`, not in the devcontainer —
    install the prebuilt release) + `mise run audit`; reads Cargo.lock so green locally is
    authoritative. A yanked crate/fresh RustSec advisory reds it on ANY push with no code change —
    try `cargo update -p <crate>` FIRST; ignore only when `cargo update --dry-run` keeps the crate
    AND the feature edge pinning it is proven at the latest upstream (196: uniffi_bindgen 0.32.2
    hard-enables askama `config` → `basic-toml`; crates.io `/dependencies` shows `features`)
- **GitHub retires pinned runner labels** (196: `macos-14`, 2026-11-02 → job `cancelled`, no step
    log). Check the `actions/runner-images` README for the successor's arch (`macos-15` = arm64)
- **A prek `types:` tag is not a file-extension guess — probe it** with a **staged, deliberately
    dirty** file (`uv run prek run <hook> --files <p>`; `Skipped` = tag misses). `.pyi` is tagged
    `pyi`, not `python` (that hole skipped the published `_lowlevel.pyi`, closed 139). **A
    `files:`-scoped hook never sees deletions** — pair any consistency hook with a pytest anchor
    test against the real tree. Formatter caveats → `learnings-archive.md`
- **JVM test/publish + Gradle bind-mount flake gotchas** (iter 128) → `learnings-archive.md`; read
    before touching `pom.xml` / `build.gradle.kts`. Gradle 9 writes `build/reports/problems` at the
    END of every build, so a *concurrent* build (a background Codex run) makes `clean` fail "Unable
    to delete directory" — re-run sequentially before believing it (165)
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
- **ci.yml sets `cancel-in-progress: true` per ref** — a follow-up develop commit cancels the prior
    sha's in-flight run (`cancelled`, not `failure`); wait for green on the exact sha. Each develop
    commit triggers TWO runs (push + the open develop→main PR)

## CID Process

- Never force-push to `develop` during a CID loop (agents commit incrementally)
- **Never trust state.md/handoff claims about external state** (registry publications, CI status,
    upstream tags) — verify at the source (`cargo search`, `npm view`, Maven Central, `gh api`)
- **Pre-push mdformat blocks on non-conforming context files**: the hook runs mdformat
    (`--wrap 100 --number`, isolated env) over every file in the push range — incl. `next.md` and
    per-agent `MEMORY*.md` — so one non-conforming file rejects the whole batch even though
    staged-only `git commit` passed. Unblock by reformatting + amending
- **Never write an exact count, substring `grep -c`, unverified CLI flag, or unverified
    `#[deprecated]` claim into a criterion** (139: `ruff format --check` saw 155 not 153; 148:
    `--fail-above 30.0` is a `cargo crap` syntax error; 168: a "must not appear" grep banned the
    *undeprecated* `Env::byte_array_from_slice`, reverted 169). Confirm attributes in
    `~/.cargo/registry/src/*/<crate>-<ver>/`; assert the *gate* (exit code); copy gate lines from
    `ci.yml`
- **next.md's Implementation Notes are a hypothesis, not a spec — algorithms *and* prose alike**
    (142: a prescribed rule that could not resolve `wheels-*`; 143: two false Unicode safety claims
    shipped verbatim into published docs). advance implements the *intent* and documents any
    deviation; review re-derives every quantitative or "never/always" claim from its source
- **One bad case per call when testing failure arms** (195): a negative test that feeds a mismatch,
    an error and a parse failure to ONE runner call stays green even if the mismatch arm breaks —
    any other bad case still fails the call. **Go `AllocsPerRun(…, func(){ _ = f() })` reads 0 even
    when `f` allocates** (the copy is optimized away) — assign to a package-level sink (202). And
    check that each "mismatch" fixture reaches the comparison: `ISCC:GAAAAAAAAAAA` (7 bytes) failed
    earlier, with "Trailing data". Positive mirror: one input form per trim site (200: outer-only
    forms missed the post-colon trims)
- **A differential gate's power lives in its CASE SET, not its case COUNT** (157): a pin on the case
    total catches a *shrunken* sweep, not a *swapped* one — prove it against a **superseded real
    design**, assert which rows light up, and commit that as a test. A one-way sweep proves only ⊇:
    "exactly" needs the reverse too, candidates derived live (198: `Z*`/`Cc`/`Cf` − strip set)
