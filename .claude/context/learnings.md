# Learnings

High-signal pitfalls, patterns and verified conventions from CID iterations. The review agent
maintains this file — append, prune, and archive completed-phase entries to `learnings-archive.md`.
**Size budget: keep under 200 lines**; over that, archive entries about fully-met target sections.

## Architecture

- Hub-and-spoke: `iscc-lib` (pure Rust core) → binding crates (py, napi, wasm, ffi, jni, rb,
    uniffi), each depending only on the core. Tier 1 = 32 symbols `pub use`d at crate root; Tier 2
    is `pub(crate)`, never crosses FFI. `packages/go` is NOT a binding — a pure-Go reimplementation

## Reference Implementation

- **`iscc-core` output is not stable across CPython versions** (5,185 code points differ 3.13 vs
    3.14 — `text_clean`/`text_collapse` strip `C` incl. unassigned `Cn`, so output tracks
    `unicodedata.unidata_version`; iscc-core#137). Always name the interpreter:
    `uv run --python 3.13 --no-project --with iscc-core …`
- Any dependency shipping DATA TABLES (Unicode, locale, tz) must be proven output-neutral by a
    **differential sweep** (`mise run unicode:sweep`), never by a green vector suite — every
    `data.json` vector predates Unicode 16

## Tooling

- `mise` for tools/tasks, `uv` for the Python env, `prek` for hooks; never use `mise` in CI — call
    tools directly. Pre-push-**only** gates: clippy `-D warnings`, cargo test, pytest, `ty check`
- **Generator-only Python deps go in a PEP 723 script, never in `[dependency-groups]`** (iter 133):
    inline `# /// script` metadata, `uv run --script <path>`, and `[tool.ty.src] exclude` **only if
    it imports a non-project dep** (a stdlib-only generator needs no exclusion — iter 159). A dep a
    *pytest* test imports in-process cannot be PEP 723 and must be a dev-group dep (142, `pyyaml`).
    Generated Rust must be data-only + rustfmt-stable; generated C must be ASCII + LF + one trailing
    newline, or the prek hygiene hooks rewrite it and break the regeneration-no-op gate
- **Every "not locally verifiable" toolchain claim so far has been false**: `cmake` via
    `uv run --with cmake cmake …` (configure into a fresh gitignored `build-*/`, never the stale
    `packages/cpp/build/`), `swift` via swift.org's Debian 12 tarball (`packages/swift/CLAUDE.md`),
    and the release-only Kotlin publish — `gradlew publishMavenPublicationToStagingRepository`
    writes to `build/staging-deploy` and skips signing without `MAVEN_GPG_PASSPHRASE`, so the whole
    release path runs offline and credential-free
- **Perf-gate tooling is install-on-demand, NOT in the devcontainer**: `mise run bench:iai:check`
    dies until `apt-get install -y valgrind` + `cargo binstall -y iai-callgrind-runner@0.16.1`
    (pin-matched to the dep)
- **A docs page lives in FOUR places — disk (`docs/**/*.md` minus `includes/`), `zensical.toml`
    `nav`, `ORDERED_PAGES`, `docs/llms.txt` (23 pages) — all gated by `scripts/check_docs_nav.py`**
    (145/146). **`zensical build` wipes `site/`, so `gen_llms_full.py` MUST run after it**

## ISCC Algorithm Knowledge

- **Unicode data version — declared 16.0.0, enforced by a `U+FFFF` SENTINEL MAP** (found iter 129,
    ruled + implemented iter 148; per-runtime table, deltas, repro `Ɤ` U+A7CB → `issues.md`):
    `text_clean`/`text_collapse` **replace** code points unassigned in 16.0.0 with
    `UNASSIGNED_SENTINEL` before normalization (vendored 731-range table, regen
    `uv run --script scripts/gen_unicode16_unassigned.py`); the *unchanged* category-`C` filter then
    removes it, and `U+FFFF` is permanently `Cn`/`ccc = 0`/undecomposable. Never "fix" one binding
    to match another
- **`str::to_lowercase()` decides `Final_Sigma` from the COMPILER's Unicode tables** (iter 156):
    rustc 1.97 ships 17.0, which moved U+0295 `Ll`→`Lo`, so a bare `.to_lowercase()` made hash
    output a function of the rustc version. `text_collapse` lowercases via `to_lowercase_unicode16`,
    which pre-substitutes each `Σ` with σ/ς decided from vendored `Cased`/`Case_Ignorable` tables
    (`utils/unicode16_case.rs`, regen `scripts/gen_unicode16_case.py`) and only then delegates to
    std. The rule is NOT "no `Cased` char follows": `ΑΣ,Β` → `αςβ`, `ΑΣ.Β` → `ασβ`
- **A UCD *derived* property a runtime does not expose can be recovered behaviourally** (156):
    `(ch+Σ).lower()` ends in ς ⟺ `Cased ∧ ¬Case_Ignorable`; `("A"+ch+Σ).lower()` ⟺
    `Cased ∨ Case_Ignorable`. Audit such a table against category-only bounds — a behavioural
    generator is not its own oracle (bounds → `decisions.md` 2026-07-27)
- **Any Unicode differential MUST include multi-code-point sequences** — deleting a `Cn` code point
    changes ADJACENCY, mapping it does not, and a per-code-point sweep scores the superseded
    delete-filter design 0 failures; only `base_mark` / `jamo` / `sigma` expose it
- **The differential sweep is a committed fail-closed gate since iter 157, hardened 158**:
    `mise run unicode:sweep` + the `unicode-sweep` CI job — 1,112,064 scalars × 8 contexts × 2 fns
    vs installed `iscc-core` on CPython 3.14, must print the byte-frozen
    `TOTAL 17793024 comparisons, 0 divergences`. **Re-run it for every Unicode-table or toolchain
    bump.** A bare `uv run scripts/unicode_sweep.py` REFUSES (only those two rebuild-first paths
    supply `--rebuilt`)
- **Boundary vectors: `crates/iscc-lib/tests/unicode_boundary.json`** (141; 12 vectors = 7
    `text_clean` + 5 `text_collapse`; the 4 **sequence** ones from 149 are the only discriminators
    of sentinel vs delete-filter, so a binding suite needs **no oracle column**). ASCII `\uXXXX`,
    `data.json`-shaped, loader `tests/test_unicode_boundary.rs`, deliberately NOT merged into
    `data.json`. **All 11 native surfaces + pure-Go gated as of iter 161**, so a new vector costs 12
    suites: 8 read the canonical fixture, Go/Swift keep byte-identity-gated copies, C/C++ share ONE
    generated header (`crates/iscc-ffi/tests/unicode_boundary_vectors.h`, gated by a pytest
    `render(fixture) == tracked` anchor, NOT `VENDORED_COPIES`)
- **A binding can pass a boundary vector for the WRONG reason** (150/161): `packages/go` has no
    freeze rule, but its 15.0 tables make U+20C1/U+A7F1 `Cn`, so its category-`C` filter
    coincidentally matches (go1.27 flips five cases red — see issues.md); Swift's `String ==` folds
    canonical equivalence, so compare `unicodeScalars.map { $0.value }` arrays instead
- **A data-driven fixture is self-referential — assert its CONTENT, not just its shape** (141):
    cases swapped for ASCII no-ops stay green forever, hence the **ungated** guards on version, case
    counts and code points; probe each, and give any skip list a *stale-key* guard
- **Per-algorithm internals**, normalization order, `data.json` shape/counts, API-parameter facts,
    **ISCC-IDv1**, the three codec rules (all test-pinned) → `learnings-archive.md`

## CI/CD

- **A binding suite's runner is not `cargo test`** (iter 151): `cargo test -p iscc-wasm` reports
    `0 passed` — only `wasm-pack test --node …` runs `#[wasm_bindgen_test]`, so clippy
    `--all-targets` proves compilation, never coverage. **Every gitignored native artifact goes
    stale silently** — Ruby `.so` (`rake compile`), napi `.node`, JNI `.so`; rebuild, then probe
    `text_clean("a"+U+A7F1+"b") == "ab"` (stale → `aSb`). CI rebuilds first, so this is local-only
- **A suite reading a fixture OUTSIDE its own build tree must declare it as a build input** (154):
    Gradle's `Test` task tracks only its project tree, so a `unicode_boundary.json` edit left
    `./gradlew test` `UP-TO-DATE` — a silent stale green (fixed with
    `inputs.file(…).withPathSensitivity(PathSensitivity.NONE)`; NONE hashes contents only, so the
    varying absolute path never forces a re-run). All 12 suites are probed and Gradle was the only
    offender: MSBuild `<Content Link=…>`, SwiftPM `.copy(...)` and the C/C++ *compile-time* include
    (CMake depfiles) are safe by construction; CI is immune either way (fresh checkout)
- **Release pipeline pattern** + `version_sync.py`'s 21 targets → `learnings-archive.md`
- **`release.yml` is `workflow_dispatch`-only — no CI run and no CID push ever exercises it.** Its
    invariants are executable gates since iters 142/144/146: `scripts/check_release_workflow.py`
    (guard shape, artifact wiring, `needs:` graph; prek hook +
    `tests/test_check_release_workflow.py`) plus the CI-only, bidirectional `--check-action-inputs`.
    Never hand-retype either into a heredoc; job-level `uses:` is unscanned by design → archive
- **A fail-open gate must publish a resolved/total counter** — without it "all checked" and "nothing
    checked" are the same green (read `action-inputs: resolved R of T`, not the job status), and
    "transport failure degrades to a warning" is NOT met by `except OSError` (`IncompleteRead` is an
    `HTTPException`, captive-portal HTML raises `yaml.YAMLError`)
- **`semver` + `coverage` CI jobs**: `semver` INFORMATIONAL pre-1.0 (enforcing at v1.0.0),
    `coverage` enforcing. `mise run semver` / `mise run coverage`
- **CRAP gate (ci-cd.md)**: ENFORCING — CI runs `cargo crap` with both `--fail-regression` and a
    **bare** `--fail-above` (30.0 lives in `.cargo-crap.toml`, so `--fail-above 30.0` is a syntax
    error); the baseline is COMMITTED (`mise run crap:baseline` after `mise run coverage`).
    **CI-ONLY gap:** a new branch in a covered fn — or merely **moving lines below the edit point**
    — is green locally and red in CI unless the baseline moves in the SAME step (never widen
    epsilon/threshold)
- **`Perf (iai-callgrind)` gate — ENFORCING (#3)**: `[profile.bench] strip = false, debug = true` is
    load-bearing (stripped binary → all benches `summary: 0` false-green) → `learnings-archive.md`
- **`Audit (cargo-deny)` gate — ENFORCING**: root `deny.toml` (v2, `yanked = "deny"`, two dev-only
    iai-callgrind advisories ignored) + `audit` CI job (`cargo-deny@0.19.9`) + `mise run audit`;
    reads Cargo.lock, so green locally is authoritative. A yanked crate or fresh RustSec advisory
    reds it on ANY push with no code change — fix with `cargo update -p <crate>` (confirm dev-only
    reach: `cargo tree -i <crate> -e no-dev` = empty), NOT a `deny.toml` ignore
- **A prek `types:` tag is not a file-extension guess — probe it** with a **staged, deliberately
    dirty** file (`uv run prek run <hook> --files <p>`; `Skipped` = the tag misses). `.pyi` is
    tagged `pyi`, not `python` (that hole skipped the published `_lowlevel.pyi`, closed iter 139).
    **A `files:`-scoped hook never sees deletions** — pair any consistency hook with a pytest anchor
    test against the real tree. Formatter caveats → `learnings-archive.md`
- **A binding-toolchain bump can silently raise the *consumer* floor** — in a *published* binding
    that is a support-policy change reserved for Titusz (floor: Kotlin 2.3 or newer; `mavenLocal`
    proof recipe + the four docs → `learnings-archive.md`)
- **JVM test/publish + Gradle bind-mount flake gotchas** (iter 128) → `learnings-archive.md`. Read
    it before touching `pom.xml` / `build.gradle.kts`, or before calling a Gradle error a failure.
    Gradle 9 writes `build/reports/problems` at the END of every build, so a *concurrent* build (a
    background Codex run) makes `clean` fail "Unable to delete directory … New files were found" —
    re-run sequentially before believing it (165)
- **A floating `@vN` GitHub Action tag is a publisher convention, NOT a guarantee** — confirm with
    `gh api repos/<o>/<r>/git/matching-refs/tags/v<N>` before writing `@vN` (the `releases/latest`
    endpoint proves a release exists, not that `@vN` resolves). `astral-sh/setup-uv` and
    `rubygems/configure-rubygems-credentials` publish only exact tags. **Measure what a branch pin
    drops:** `gh api repos/<o>/<r>/compare/<tag>...main --jq .ahead_by` — a PR title understates it
    (162: `@main` was 31 commits + a rebuilt `dist/` ahead of `v2.1.0`). Majors →
    `.claude/agent-memory/advance/deps-refresh.md`
- **An action-major bump is statically verifiable far past "the tag exists"** — the `inputs`/
    `outputs` diff is automated (`--check-action-inputs`); what stays manual is every intervening
    major's *default* changes (recipe + the two silent biters → `learnings-archive.md`)
- **Prove a new gate with a REAL regression in a THROWAWAY repo, not a synthetic typo** (iters
    144/152/163): `git archive HEAD | tar -x -C /tmp/x && git init` gives a probe tree where
    `git add`/`git rm` are free; the cheapest *real* regression is the guarded file's own previous
    version (`git show HEAD~1:<path>`) — that is the drift the gate was built for. A
    **set-equality** gate is blind twice: equal *empty* sets pass (hence a count floor) and a
    **duplicated** row passes (keep row order long enough to count repeats — iter 163). Same shape
    for a **test-framework major**: a "≥ N passed" floor cannot see a silent collapse of
    parameterized rows (164: xunit v2 and v3 both gave 104), so demand the SAME total as the
    pre-bump tree (`git archive HEAD~1 | tar -x`) or — cheaper — derive it from the FIXTURE (166:
    per-function `data.json` vector counts + static `@Test` count). **`mvn test` silently reuses
    stale test classes** ("Nothing to compile"), so only `mvn clean test` proves the new framework
    compiles
- **A markdown-table parity gate must anchor to its own section** (163): `specs/ci-cd.md` carries 14
    backticked first-column rows under `## Version Management` that a whole-file scan would read as
    bogus job rows — so the real file, not the fixture, is what proves the anchor load-bearing
- **ci.yml sets `cancel-in-progress: true` per ref** — a follow-up develop commit cancels the
    previous sha's in-flight run (`cancelled`, not `failure`); let it conclude when a Done-When
    needs green CI on a sha. Each develop commit triggers TWO runs (push + the open develop→main PR)

## CID Process

- Never force-push to `develop` during a CID loop (agents commit incrementally); **feature flags**
    are fully met → `learnings-archive.md`
- **Never trust state.md/handoff claims about external state** (registry publications, CI status,
    upstream tags) — verify at the source (`cargo search`, `npm view`, Maven Central, `gh api`)
- **Human-handoff vs IDLE (iter 111)**: if the remaining `normal` issues are all
    `HUMAN REVIEW REQUESTED` spec amendments, strict `**IDLE**` (all `low`) is NOT met — flag
    `**HUMAN REVIEW REQUESTED**` (runner "pause") instead, without churn
- **Pre-push mdformat blocks on non-conforming context files**: the hook runs mdformat with
    `--wrap 100` + `--number` in an isolated env, over every file in the push range — incl.
    `next.md` and per-agent `MEMORY*.md` — so one non-conforming file rejects the whole batch even
    though staged-only `git commit` passed. Unblock by reformatting + amending
- **Never write an exact count, a substring `grep -c`, or an unverified CLI flag into a verification
    criterion** (iter 139: `ruff format --check` saw 155 files, not 153. Iter 148:
    `--fail-above   30.0` is a `cargo crap` syntax error). Assert the *gate* (exit code) and anchor
    greps instead; copy gate invocations from `ci.yml`, never from memory
- **next.md's Implementation Notes are a hypothesis, not a spec — algorithms *and* prose alike**
    (142: a prescribed rule that could not resolve `wheels-*`; 143: two false Unicode safety claims
    shipped verbatim into published docs). advance implements the *intent* and documents any
    deviation; review re-derives every quantitative or "never/always" claim from its source
- **next.md must never task advance with editing `issues.md`** (iter 135): advance's protocol
    forbids writing it and review owns issue progress/resolution. A slice-progress ledger paragraph
    belongs in the handoff Notes for review to append — advance correctly refused and quoted it
- **A differential gate's power lives in its CASE SET, not its case COUNT** (157): a pin on the case
    total catches a *shrunken* sweep, not a *swapped* one — prove it against a **superseded real
    design**, assert which rows light up, and commit that as a test
