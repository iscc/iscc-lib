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
    3.14 — `text_clean`/`text_collapse` strip category `C` incl. unassigned `Cn`, so the result
    tracks `unicodedata.unidata_version`). Always name the interpreter, and check both via
    `uv run --python 3.13 --no-project --with iscc-core python -c …`. Upstream: iscc-core#137
- Any dependency shipping DATA TABLES (Unicode, locale, tz) must be proven output-neutral by a
    **differential sweep** over all 1,112,032 code points (~2 min; recipe → `learnings-archive.md`),
    never by a green vector suite — every vendored conformance vector predates Unicode 16

## Tooling

- `mise` manages tool versions and tasks; Python env uses `uv`; hooks via `prek`. Never use `mise`
    in CI — call tools directly
- Pre-push-**only** gates: clippy `-D warnings`, cargo test, pytest, `ty check`. The ruff `S`/`C901`
    scans also run pre-commit since iter 134 (they are in the default select now)
- **PyO3 is `0.29`** (iscc-py only): keep `#[pymodule(name = "_lowlevel", gil_used = true)]`
    explicit. Per-hop upgrade recipe → `learnings-archive.md`
- **`_lowlevel.pyi` is consumer-facing** (wheel ships `py.typed`): stub bodies are docstring-only,
    and edits need `mypy 1.18 --strict` + `pyright 1.1.407`, not just `ty` → `learnings-archive.md`
- **Generator-only Python deps go in a PEP 723 script, never in `[dependency-groups]`** (iter 133):
    inline `# /// script` metadata, `uv run --script <path>`, path in `[tool.ty.src] exclude` —
    keeps `uv.lock` hold-back-free. A dep a *pytest* test imports in-process cannot be PEP 723 and
    must be a dev-group dep (iter 142, `pyyaml`). Generated Rust must be data-only + rustfmt-stable
- **`cargo clippy -p iscc-lib --no-default-features --all-targets` has always failed** — `benches/`
    import `gen_meta_code_v0`/`gen_text_code_v0` unconditionally (E0432). The real feature-matrix
    gate is `--no-default-features` *without* `--all-targets`; not a fresh regression
- **Perf-gate tooling is install-on-demand, NOT in the devcontainer**: `mise run bench:iai:check`
    dies until `sudo apt-get install -y valgrind` +
    `cargo binstall -y iai-callgrind-runner --version 0.16.1` (MUST match the pin); CI mirrors this
- **A docs page lives in FOUR places, all gated by `scripts/check_docs_nav.py`** (iter 145): disk
    (`docs/**/*.md` minus `includes/`), `zensical.toml` `nav`, `ORDERED_PAGES`, and the absolute
    `docs/llms.txt` links — 23 pages. Hole: the nav is regex-parsed, so a commented-out entry counts
- **`zensical build` wipes `site/`, so `gen_llms_full.py` MUST run after it** (`docs.yml` order).
    Reversed, every per-page `site/**/*.md` check reads as missing — an artifact, not a defect

## ISCC Algorithm Knowledge

- **Unicode data version — declared 16.0.0 with a freeze rule** (found iter 129, decided 2026-07-25,
    **Rust core implemented iter 133**; per-runtime table, deltas, repro `Ɤ` U+A7CB and remaining
    steps → the `issues.md` entry): `text_clean`/`text_collapse` remove code points unassigned in
    Unicode 16.0.0 *before* any normalization or category lookup (vendored 731-range table, regen
    `uv run --script scripts/gen_unicode16_unassigned.py`). Runtimes still ship different tables (Go
    15.0, Python 3.13 15.1, Rust crates 16.0/17.0) — never "fix" one binding to match another
- **The freeze rule changes ADJACENCY, so it diverges from `iscc-core` on sequences even with
    identical Unicode data** (measured iter 133 review; three worked repros + the spec consequences
    in the open `issues.md` entry "Freeze-rule ordering diverges from iscc-core on sequences").
    Removing a `Cn` code point *before* normalization unblocks contextual transforms `iscc-core`
    (remove *after*) still blocks. A per-code-point sweep cannot see this class — any differential
    sweep must include multi-code-point sequences (base+Cn+mark, jamo+Cn+jamo, Σ+Cn+cased)
- **Boundary vectors live in `crates/iscc-lib/tests/unicode_boundary.json`** (iter 141; ASCII
    `\uXXXX`, `data.json`-shaped, 4 code points × `text_clean`/`text_collapse`) + loader
    `tests/test_unicode_boundary.rs` — propagation source for every binding, deliberately NOT merged
    into `data.json` (rationale → `decisions.md`). Two are **live** guards: `unicode-normalization`
    0.1.25 ships **Unicode 17.0** tables where U+A7F1 is `Lm` `<super> 0053` (NFKC → `S`) and U+20C1
    is `Sc`, so both leak the moment the freeze filter stops running first
- **A data-driven fixture is self-referential — assert its CONTENT, not just its shape** (iter 141):
    the vector tests compare the implementation against the fixture, so cases swapped for ASCII
    no-ops keep everything green. The ungated metadata guard therefore asserts the exact non-ASCII
    code-point set per section (runs under `--no-default-features` too). Mutation-probe any such
    guard — edit the JSON, watch it fail, `git checkout --` the file
- **Unicode 16.0 assigned 5,185 code points — not "just the 7 new emoji"** (measured iter 143):
    3,995 Egyptian Hieroglyphs, 7 new scripts, **32 LATIN-named** incl. U+A7CB (our own repro).
    Never write "Latin text is unaffected"; measure by diffing assigned-set dumps from two
    `uvx --with unicodedata2==<ver> python` runs
- **Per-algorithm internals**, `gen_meta_code_v0` normalization order, `data.json` vector shape and
    counts, settled API-parameter facts and **ISCC-IDv1** → `learnings-archive.md`
- `conformance_selftest` masks truncated codes bitwise — never compare full strings below 256 bits
- **ISCC decode body-length check must be EXACT (`len(tail) == nbytes`), not `>= nbytes`** — a loose
    guard silently aliases trailing base32 chars (`ISCC:...AB` == `ISCC:...ABAA`); enforced in Go
    `IsccDecode` + Rust `iscc_decode`. Composite `iscc_decompose` legitimately consumes trailing
    units — do NOT harden it
- `decode_length`: multiples of 32 bits for standard MainTypes, 64 for ISCC-CODE, 8 for ID (C FFI:
    length index for 64-bit codes is 1, not 0)

## CI/CD

- Windows GHA runners default to `pwsh`. Steps using bash syntax (`$(...)`, `$GITHUB_OUTPUT`,
    `grep`, `sed`) MUST specify `shell: bash` — per-matrix version steps (e.g. `build-ffi`) hit
    Windows. Always check `shell:` when adding `run:` steps to cross-platform matrices
- **Release pipeline pattern** + `version_sync.py`'s 21 targets → `learnings-archive.md`
- **`release.yml` is `workflow_dispatch`-only — no CI run and no CID push ever exercises it.** All
    its invariants are executable gates since iters 142/144: `scripts/check_release_workflow.py`
    checks guard shape, artifact wiring and the `needs:` graph offline (prek hook +
    `tests/test_check_release_workflow.py`), and `--check-action-inputs` validates every `with:` key
    and `steps.<id>.outputs.<x>` read against each ref's published `action.yml` over the network in
    the CI-only `release-workflow` job. Never hand-retype these into a heredoc again. **Known
    holes:** a *required* input the workflow omits is not caught, and a rate-limited run degrades to
    all-skip warnings while staying green — read the job log, not just its status. Guard-shape
    rationale → `learnings-archive.md`
- **"Any transport failure degrades to a warning" is a contract `except OSError` does not fulfil**
    (iter 144, Codex): `http.client.IncompleteRead` is an `HTTPException`/`ValueError`, and a
    captive-portal HTML body raises `yaml.YAMLError` — both escape and turn the gate red. When a
    gate promises never to red on network trouble, enumerate the non-`OSError` stdlib failure modes
- **`semver` + `coverage` CI jobs**: `semver` INFORMATIONAL pre-1.0 (enforcing at v1.0.0),
    `coverage` enforcing. `mise run semver` / `mise run coverage`
- **CRAP gate (ci-cd.md)**: ENFORCING (`cargo crap --fail-regression --fail-above` 30.0, max ~22.3),
    `.crap-baseline.json` COMMITTED (regen `mise run crap:baseline`). **CI-ONLY gap** — not in
    `mise run check`: a source change adding a branch to a covered fn lands green locally but reds
    CI unless the baseline is refreshed in the SAME step (never widen epsilon/threshold)
- **`Perf (iai-callgrind)` gate — ENFORCING (#3)**: `[profile.bench] strip = false, debug = true` is
    load-bearing (stripped binary → all benches `summary: 0` false-green) → `learnings-archive.md`
- **`Audit (cargo-deny)` gate — ENFORCING**: root `deny.toml` (v2, `yanked = "deny"`, two dev-only
    iai-callgrind advisories ignored) + `audit` CI job (`cargo-deny@0.19.9`) + `mise run audit`;
    reads Cargo.lock, so green locally is authoritative. A yanked crate or fresh RustSec advisory
    reds it on ANY push with no code change — fix with `cargo update -p <crate>` (confirm dev-only
    reach: `cargo tree -i <crate> -e no-dev` = empty), NOT a `deny.toml` ignore
- **v0.6.0 dependency refresh + ruff 0.16 adoption are CLOSED** (iters 124–140; slice history,
    hold-back verification and relock-proof caveats → `learnings-archive.md`). Live rules: ruff is
    **0.16.0**, preview a major with `uvx ruff@X.Y.Z check .` (never touches `uv.lock`); prek
    `types_or` = `[python, pyi, markdown]` (format) / `[python, pyi]` (check), a strict superset of
    CI; rules go in `[tool.ruff.lint] extend-select`, **never `select`** (drops `E4`/`E7`/`E9`/`F`);
    **never `ruff check --fix .`** without `--select` — it deletes load-bearing `# noqa: S603/S607`
- **A prek `types:` tag is not a file-extension guess — probe it** (`.pyi` is tagged `pyi`, not
    `python`; that hole silently skipped the published `_lowlevel.pyi`, closed iter 139). Prove a
    hook's surface with a **staged, deliberately dirty** probe: `uv run prek run <hook> --files <p>`
    — `Skipped` = the tag misses; "files were modified by this hook" = it bites (tracked files
    only). Formatter caveats → `learnings-archive.md`. **A `files:`-scoped hook never sees
    deletions** (added/copied/modified only, verified iter 145) — pair any consistency hook with a
    pytest anchor test against the real tree, which covers the delete case at pre-push and in CI
- **A binding-toolchain bump can silently raise the *consumer* floor** — treat compiler/toolchain
    bumps in a *published* binding as support-policy changes reserved for Titusz. Current floor
    "Kotlin 2.3 or newer"; `mavenLocal` proof recipe + the four docs → `learnings-archive.md`
- **JVM test/publish + Gradle bind-mount flake gotchas** (iter 128) → `learnings-archive.md`. Read
    it before touching `pom.xml` / `build.gradle.kts`, or before calling a Gradle error a failure
- **A floating `@vN` GitHub Action tag is a publisher convention, NOT a guarantee** — confirm with
    `gh api repos/<o>/<r>/git/matching-refs/tags/v<N>` before writing `@vN` (the `releases/latest`
    endpoint proves a release exists, not that `@vN` resolves). `astral-sh/setup-uv` stopped
    publishing floating majors after `v7` → pin `@v9.0.0` with a `# exact tag:` comment;
    `rubygems/configure-rubygems-credentials` publishes only exact tags (no `v2`). Current majors →
    `.claude/agent-memory/advance/deps-refresh.md`
- **An action-major bump is statically verifiable far past "the tag exists"** — the `inputs`/
    `outputs` diff is automated (`--check-action-inputs`); what stays manual is every intervening
    major's *default* changes (an input surviving is not its default surviving). Recipe + the two
    silent biters (`setup-node@v5+` caching, `checkout@v6+` token location) → `learnings-archive.md`
- **Prove a new gate with a REAL regression, not a synthetic typo** (iter 144): downgrading
    `actions/download-artifact@v8` → `@v3` in a temp copy of `release.yml` fired 14 errors
    (`pattern` and `merge-multiple` genuinely dropped across those majors) and `@v999` fired the 404
    path — that is the failure class the gate exists for. A hand-typo'd key only proves string
    comparison works. A **set-equality** gate also passes vacuously on equal *empty* sets — give its
    anchor test a count floor (iter 145)
- **ci.yml sets `cancel-in-progress: true` per ref** — a follow-up develop commit cancels the
    in-flight run of the previous sha (check-runs conclude `cancelled`, not `failure`); let it
    conclude when a Done-When needs green CI on a specific sha. Each develop commit triggers TWO
    runs (push + `pull_request` from the open develop→main PR), so totals are ~2× the job count

## CID Process

- Never force-push to `develop` during a CID loop — agents commit incrementally (branching model
    itself is in CLAUDE.md)
- **Feature flags**: fully met → `learnings-archive.md` (read before touching `[features]`)
- **Never trust state.md/handoff claims about external state** (registry publications, CI status,
    upstream tags) — verify at the source (`cargo search`, `npm view`, Maven Central API,
    `pip index versions`, Go module proxy, `gh api`)
- **Human-handoff vs IDLE (iter 111)**: when autonomous work runs out but the remaining `normal`
    issues are all `HUMAN REVIEW REQUESTED` spec amendments, strict `**IDLE**` (all issues `low`) is
    NOT met — flag `**HUMAN REVIEW REQUESTED**` (runner "pause") instead, and don't manufacture
    churn to avoid it
- **Pre-push mdformat blocks on non-conforming context files**: the hook runs mdformat with
    `--wrap 100` + `--number` in an isolated `mdformat-mkdocs[recommended]` env, over every file in
    the push range — incl. `next.md` and per-agent `MEMORY*.md` — so one non-conforming file rejects
    the whole batch even though staged-only `git commit` passed. define-next MUST run
    `mise run format` before committing; review can unblock by reformatting + amending
- **Never write an exact count or a substring `grep -c` into a verification criterion** (iter 139:
    `ruff format --check` saw 155 files, not 153 — the count drifts whenever a tracked
    `.md`/`.py`/`.pyi` lands, CID's own memory files included; `grep -c 'exclude'` returned 2, not
    0, matching pre-existing `--force-exclude` flags). Assert the *gate* (exit code) and anchor
    greps; advance reports the mismatch and proves the intent instead of chasing the number
- **next.md's Implementation Notes are a hypothesis, not a spec — algorithms *and* prose alike**
    (iter 142: the prescribed artifact-matching rule could not resolve `wheels-*` at HEAD; iter 143:
    two false Unicode safety claims shipped verbatim into published docs). advance implements the
    *intent* and documents any deviation; review re-proves the prescribed rule fails, re-derives
    every quantitative or "never/always" claim, and scopes "implementation X agrees with us" to the
    class actually proven
- **next.md must never task advance with editing `issues.md`** (iter 135): advance's protocol
    forbids writing it and review owns issue progress/resolution. A slice-progress ledger paragraph
    belongs in the handoff Notes for review to append — advance correctly refused and quoted it
- **Role model assignment (2026-07)**: `advance` runs on Claude Fable 5 (`model: fable`,
    `effort: xhigh`, runner timeout 3600s); all other roles on `opus`. Deliberate diversity — do not
    "unify" onto one model
