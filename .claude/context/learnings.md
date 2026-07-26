# Learnings

High-signal pitfalls, patterns and verified conventions from CID iterations. The review agent
maintains this file — append, prune, and archive completed-phase entries to `learnings-archive.md`.

**Size budget:** Keep under 200 lines. When this file exceeds 200 lines, move entries about
fully-met target sections to `learnings-archive.md`.

## Architecture

- Hub-and-spoke: `iscc-lib` (pure Rust core) → binding crates (py, napi, wasm, ffi, jni, rb,
    uniffi). Each depends only on `iscc-lib`, never on another binding. `packages/go` is NOT a
    binding — it is an independent pure-Go reimplementation (see the Unicode entry below)
- Tier 1 API (32 symbols) exposed via `pub use` at crate root. Tier 2 is `pub(crate)` — internal
    only, never crosses FFI. Core is sync; each binding adapts async idiomatically

## Reference Implementation

- Reference code lives in `reference/iscc-core/` (shallow clone, gitignored). Read source files
    directly — do not use deepwiki MCP
- When porting from Python reference, verify against Rust `crates/iscc-lib/src/` first — the Rust
    implementation is the authoritative source for this project
- **`iscc-core` output is not stable across CPython versions** (5,185 code points differ 3.13 vs
    3.14 — `text_clean`/`text_collapse` strip `C` incl. unassigned `Cn`, so the result tracks
    `unicodedata.unidata_version`; upstream iscc-core#137). Always name the interpreter:
    `uv run --python 3.13 --no-project --with iscc-core python -c …`
- Any dependency shipping DATA TABLES (Unicode, locale, tz) must be proven output-neutral by a
    **differential sweep** over all 1,112,032 code points (~2 min; recipe → `learnings-archive.md`),
    never by a green vector suite — every `data.json` vector predates Unicode 16 (only the separate
    `unicode_boundary.json` fixture probes the 16.0 boundary)

## Tooling

- `mise` manages tool versions and tasks; Python env uses `uv`; hooks via `prek`. Never use `mise`
    in CI — call tools directly. Pre-push-**only** gates: clippy `-D warnings`, cargo test, pytest,
    `ty check` (the ruff `S`/`C901` scans also run pre-commit since iter 134)
- **Generator-only Python deps go in a PEP 723 script, never in `[dependency-groups]`** (iter 133):
    inline `# /// script` metadata, `uv run --script <path>`, path in `[tool.ty.src] exclude` —
    keeps `uv.lock` hold-back-free. A dep a *pytest* test imports in-process cannot be PEP 723 and
    must be a dev-group dep (iter 142, `pyyaml`). Generated Rust must be data-only + rustfmt-stable
- **`cargo clippy -p iscc-lib --no-default-features --all-targets` has always failed** (`benches/`
    import `gen_meta_code_v0`/`gen_text_code_v0` unconditionally, E0432) — drop `--all-targets`
- **Perf-gate tooling is install-on-demand, NOT in the devcontainer**: `mise run bench:iai:check`
    dies until `sudo apt-get install -y valgrind` +
    `cargo binstall -y iai-callgrind-runner --version 0.16.1` (MUST match the pin); CI mirrors this
- **A docs page lives in FOUR places — disk (`docs/**/*.md` minus `includes/`), `zensical.toml`
    `nav`, `ORDERED_PAGES`, `docs/llms.txt` (23 pages) — all gated by `scripts/check_docs_nav.py`**
    (iter 145), whose regex nav parser is comment-aware since iter 146: strip `#`-to-EOL only
    *outside* double quotes. **`zensical build` wipes `site/`, so `gen_llms_full.py` MUST run after
    it** — reversed, every per-page `site/**/*.md` reads as missing

## ISCC Algorithm Knowledge

- **Unicode data version — declared 16.0.0, enforced by a `U+FFFF` SENTINEL MAP** (found iter 129,
    ruled + implemented 2026-07-26 iter 148; per-runtime table, deltas, repro `Ɤ` U+A7CB → the
    `issues.md` entry): `text_clean`/`text_collapse` **replace** code points unassigned in Unicode
    16.0.0 with `UNASSIGNED_SENTINEL` before normalization (vendored 731-range table, regen
    `uv run --script scripts/gen_unicode16_unassigned.py`); the *unchanged* category-`C` filter then
    removes the sentinel exactly where the reference removes them (conformance), and `U+FFFF` is
    permanently `Cn`/`ccc = 0`/undecomposable (table invariance). Runtimes still ship different
    tables (Go 15.0, Python 3.13 15.1, Rust crates 16.0/17.0) — never "fix" one binding to match
    another
- **Deleting a `Cn` code point before normalization changes ADJACENCY; mapping it does not** — why
    the iter-133 pre-filter was real non-conformance (deletion unblocks canonical/jamo composition,
    `Final_Sigma`, diaeresis). **Any Unicode differential MUST include multi-code-point sequences**:
    a per-code-point sweep scores the broken design 0 failures; the iter-148 sequence probe (127
    code points × 10 contexts) scored it 504/1270, the sentinel 0/1270 (recipe → review memory)
- **Boundary vectors live in `crates/iscc-lib/tests/unicode_boundary.json`** (iter 141, +4
    **sequence** vectors iter 149; ASCII `\uXXXX`, `data.json`-shaped) + loader
    `tests/test_unicode_boundary.rs` — propagation source for every binding, deliberately NOT merged
    into `data.json` (rationale → `decisions.md`). Two single-code-point cases are **live** 17.0-
    table guards (U+A7F1 `Lm <super> 0053`, U+20C1 `Sc`), but all 4 wrap their code point in ASCII
    and are **deletion-vs-sentinel agnostic** — only the sequence vectors gate that distinction, and
    their expected values already differ from the delete-filter ones, so a binding suite needs **no
    oracle column**. Gated: Rust, Python, Go (3 ruled skips), WASM, Ruby, napi, Java = 6 of 11
    surfaces (`grep unicode_boundary` under-counts — Java's file is `UnicodeBoundaryTest.java`)
- **A binding can pass a boundary vector for the WRONG reason** (iter 150): `packages/go` has no
    freeze rule; its Unicode 15.0 tables make U+20C1/U+A7F1 `Cn`, so the category-`C` filter drops
    them and coincidentally matches the sentinel output. Under go1.27 both become assigned and 5
    green cases flip red — never version-gate a skip list to hide that
- **Vendored vector copies are byte-identity gated** by `tests/test_vendored_fixtures.py` (iter
    152): a `(canonical, copy)` table + a `git ls-files` set check reds on an unregistered *or
    deleted* tracked copy; discovery is by basename, so keep the canonical filenames
- **A "must NOT be" oracle must name the design it came from** (iter 149): a *delete filter* turns
    `e U+A7F1 U+0301` into `U+00E9`; `e U+015A` is the *category-override* failure. next.md labelled
    the whole column "delete filter" and the mismatched value shipped into published docs
- **A data-driven fixture is self-referential — assert its CONTENT, not just its shape** (iter 141):
    cases swapped for ASCII no-ops stay green forever, hence the **ungated** guards on version, case
    counts and code points; mutation-probe each one, and give any skip list a *stale-key* guard
- **Unicode 16.0 assigned 5,185 code points — not "just the 7 new emoji"** (iter 143): 3,995
    Egyptian Hieroglyphs, 7 new scripts, **32 LATIN-named** incl. U+A7CB (our own repro). Never
    write "Latin text is unaffected"; diff assigned-set dumps from two `unicodedata2==<ver>` runs
- **Per-algorithm internals**, `gen_meta_code_v0` normalization order, `data.json` vector shape/
    counts, settled API-parameter facts, **ISCC-IDv1** and the three settled codec rules (all
    test-pinned) → `learnings-archive.md`

## CI/CD

- Windows GHA runners default to `pwsh` — any `run:` step using bash syntax (`$(...)`,
    `$GITHUB_OUTPUT`, `grep`, `sed`) in a cross-platform matrix MUST set `shell: bash`
- **A binding suite's runner is not `cargo test`** (iter 151): `cargo test -p iscc-wasm` reports
    `0 passed` — only `wasm-pack test --node …` runs `#[wasm_bindgen_test]`, so clippy
    `--all-targets` proves compilation, never coverage. **Every gitignored native artifact goes
    stale silently** — Ruby `.so` (`rake compile`), napi `.node` (`napi build --platform`), JNI
    `.so` (`cargo build -p iscc-jni`); rebuild, then probe `text_clean("a"+U+A7F1+"b") == "ab"`
    (stale → `aSb`). CI rebuilds all three first, so this is local-only; `mvn -o -B test -f <pom>`
    works offline and surefire's cwd is the pom's basedir
- **Release pipeline pattern** + `version_sync.py`'s 21 targets → `learnings-archive.md`
- **`release.yml` is `workflow_dispatch`-only — no CI run and no CID push ever exercises it.** Its
    invariants are executable gates since iters 142/144/146: `scripts/check_release_workflow.py`
    (guard shape, artifact wiring, `needs:` graph; prek hook +
    `tests/test_check_release_workflow.py`) plus the CI-only, bidirectional `--check-action-inputs`
    (every `with:` key and `steps.<id>.outputs.<x>` read against each ref's published `action.yml`).
    Never hand-retype either into a heredoc. **By design:** job-level `uses:` is unscanned and an
    all-skipped run stays green — read the `action-inputs: resolved R of T` line, not the job status
    → archive
- **A fail-open gate must publish a resolved/total counter** (iters 144→146) — without it "all
    checked" and "nothing checked" are the same green; "transport failure degrades to a warning" is
    NOT met by `except OSError` (`IncompleteRead` is an `HTTPException`, captive-portal HTML raises
    `yaml.YAMLError`), and a "must be present" check needs its triggering inputs listed to be
    trusted
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
- **v0.6.0 dep refresh + ruff 0.16 adoption are CLOSED** (iters 124–140 → archive). Live rules: ruff
    **0.16.0**; preview a major with `uvx ruff@X.Y.Z check .` (never touches `uv.lock`); rules go in
    `[tool.ruff.lint] extend-select`, never `select`; **never `ruff check --fix .`** without
    `--select` (deletes load-bearing `# noqa: S603/S607`)
- **A prek `types:` tag is not a file-extension guess — probe it** (`.pyi` is tagged `pyi`, not
    `python`; that hole silently skipped the published `_lowlevel.pyi`, closed iter 139). Prove a
    hook's surface with a **staged, deliberately dirty** probe: `uv run prek run <hook> --files <p>`
    — `Skipped` = the tag misses; "files were modified by this hook" = it bites (tracked files
    only). Formatter caveats → `learnings-archive.md`. **A `files:`-scoped hook never sees
    deletions** (added/copied/modified only, verified iter 145) — pair any consistency hook with a
    pytest anchor test against the real tree, which covers the delete case at pre-push and in CI
- **A binding-toolchain bump can silently raise the *consumer* floor** — in a *published* binding
    that is a support-policy change reserved for Titusz (floor: Kotlin 2.3 or newer; `mavenLocal`
    proof recipe + the four docs → `learnings-archive.md`)
- **JVM test/publish + Gradle bind-mount flake gotchas** (iter 128) → `learnings-archive.md`. Read
    it before touching `pom.xml` / `build.gradle.kts`, or before calling a Gradle error a failure
- **A floating `@vN` GitHub Action tag is a publisher convention, NOT a guarantee** — confirm with
    `gh api repos/<o>/<r>/git/matching-refs/tags/v<N>` before writing `@vN` (the `releases/latest`
    endpoint proves a release exists, not that `@vN` resolves). `astral-sh/setup-uv` stopped
    publishing floating majors after `v7` → pin `@v9.0.0` with a `# exact tag:` comment;
    `rubygems/configure-rubygems-credentials` publishes only exact tags. Majors →
    `.claude/agent-memory/advance/deps-refresh.md`
- **An action-major bump is statically verifiable far past "the tag exists"** — the `inputs`/
    `outputs` diff is automated (`--check-action-inputs`); what stays manual is every intervening
    major's *default* changes. Recipe + the two silent biters (`setup-node@v5+` caching,
    `checkout@v6+` token location) → `learnings-archive.md`
- **Prove a new gate with a REAL regression in a THROWAWAY repo, not a synthetic typo** (iters
    144/152): `git archive HEAD | tar -x -C /tmp/x && git init` gives a probe tree where `git add`/
    `git rm` are free. A **set-equality** gate passes vacuously on equal *empty* sets — count floor
- **ci.yml sets `cancel-in-progress: true` per ref** — a follow-up develop commit cancels the
    in-flight run of the previous sha (check-runs conclude `cancelled`, not `failure`); let it
    conclude when a Done-When needs green CI on a specific sha. Each develop commit triggers TWO
    runs (push + `pull_request` from the open develop→main PR), so totals are ~2× the job count

## CID Process

- Never force-push to `develop` during a CID loop — agents commit incrementally; **feature flags**
    are fully met → `learnings-archive.md` (read before touching `[features]`)
- **Never trust state.md/handoff claims about external state** (registry publications, CI status,
    upstream tags) — verify at the source (`cargo search`, `npm view`, Maven Central API,
    `pip index versions`, Go module proxy, `gh api`)
- **Human-handoff vs IDLE (iter 111)**: when autonomous work runs out but the remaining `normal`
    issues are all `HUMAN REVIEW REQUESTED` spec amendments, strict `**IDLE**` (all issues `low`) is
    NOT met — flag `**HUMAN REVIEW REQUESTED**` (runner "pause") instead, without churn
- **Pre-push mdformat blocks on non-conforming context files**: the hook runs mdformat with
    `--wrap 100` + `--number` in an isolated `mdformat-mkdocs[recommended]` env, over every file in
    the push range — incl. `next.md` and per-agent `MEMORY*.md` — so one non-conforming file rejects
    the whole batch even though staged-only `git commit` passed. define-next MUST run
    `mise run format` before committing; review can unblock by reformatting + amending
- **Never write an exact count, a substring `grep -c`, or an unverified CLI flag into a verification
    criterion** (iter 139: `ruff format --check` saw 155 files, not 153; `grep -c 'exclude'`
    returned 2, not 0. Iter 148: `--fail-above 30.0` is a `cargo crap` syntax error). Assert the
    *gate* (exit code) and anchor greps instead, and copy gate invocations from `ci.yml`, never from
    memory
- **next.md's Implementation Notes are a hypothesis, not a spec — algorithms *and* prose alike**
    (iter 142: the prescribed artifact-matching rule could not resolve `wheels-*`; iter 143: two
    false Unicode safety claims shipped verbatim into published docs). advance implements the
    *intent* and documents any deviation; review re-proves the prescribed rule fails, re-derives
    every quantitative or "never/always" claim, and scopes "implementation X agrees with us" to the
    class actually proven
- **next.md must never task advance with editing `issues.md`** (iter 135): advance's protocol
    forbids writing it and review owns issue progress/resolution. A slice-progress ledger paragraph
    belongs in the handoff Notes for review to append — advance correctly refused and quoted it
- **Role model assignment (2026-07)**: `advance` runs on Claude Fable 5 (`model: fable`,
    `effort: xhigh`, timeout 3600s), all other roles on `opus` — deliberate; do not "unify"
