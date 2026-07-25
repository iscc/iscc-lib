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
- **Generator-only Python deps go in a PEP 723 script, never in `[dependency-groups]`** (iter 133,
    `scripts/gen_unicode16_unassigned.py` + `unicodedata2==16.0.0`): inline `# /// script` metadata,
    `uv run --script <path>`, and add the path to `[tool.ty.src] exclude` with a comment (the
    `packages/cpp/conanfile.py` precedent) — keeps `uv.lock` free of a permanent hold-back.
    Generated Rust must be data-only + rustfmt-stable so regeneration is a no-op diff
- **`cargo clippy -p iscc-lib --no-default-features --all-targets` has always failed** — `benches/`
    import `gen_meta_code_v0`/`gen_text_code_v0` unconditionally (E0432); benches require default
    features. The real feature-matrix gate is `--no-default-features` *without* `--all-targets`
    (tests compile there because they are `#[cfg]`-gated). Don't mistake it for a fresh regression
- **Perf-gate tooling is install-on-demand, NOT in the devcontainer**: `mise run bench:iai:check`
    dies with "No such file or directory" until `sudo apt-get install -y valgrind` +
    `cargo binstall -y iai-callgrind-runner --version 0.16.1` (MUST match the pin) — CI mirrors this

## ISCC Algorithm Knowledge

- **Unicode data version — declared 16.0.0 with a freeze rule** (divergence found iter 129, decided
    2026-07-25, **Rust core implemented iter 133**): Go stdlib/`x/text` = 15.0.0, Python 3.13 =
    15.1.0, Rust `unicode-general-category` = 16.0.0 / `unicode-normalization` = 17.0.0, and Go's
    `unicode.C` includes unassigned `Cn` — so 5,813 post-15 code points are stripped by
    Go/`iscc-core` but kept by the Rust core (repro `Ɤ` U+A7CB); no vector catches it. Freeze rule:
    `text_clean`/`text_collapse` remove code points unassigned in Unicode 16.0.0 *before* any
    normalization or category lookup (vendored 731-range table, regen
    `uv run --script scripts/gen_unicode16_unassigned.py`). Deltas: 15.1→16 = 5,185 category + 56
    normalization; 16→17 = 4,803 + 1 — all new assignments. Never "fix" one binding to match
    another; remaining steps → `issues.md`
- **The freeze rule changes ADJACENCY, so it diverges from `iscc-core` on sequences even with
    identical Unicode data** (measured iter 133 review; three worked repros + the spec consequences
    in the open `issues.md` entry "Freeze-rule ordering diverges from iscc-core on sequences").
    Removing a `Cn` code point *before* normalization unblocks contextual transforms `iscc-core`
    (remove *after*) still blocks. A per-code-point sweep cannot see this class — any differential
    sweep must include multi-code-point sequences (base+Cn+mark, jamo+Cn+jamo, Σ+Cn+cased)
- **Boundary vectors live in `crates/iscc-lib/tests/unicode_boundary.json`** (iter 141; ASCII
    `\uXXXX`, `data.json`-shaped, 4 code points × `text_clean`/`text_collapse`) + loader
    `tests/test_unicode_boundary.rs` — propagation source for every binding, deliberately NOT merged
    into `data.json` (vendored upstream + hardcoded count asserts; rationale → `decisions.md`). Two
    vectors are **live** guards: `unicode-normalization` 0.1.25 ships **Unicode 17.0** tables where
    U+A7F1 is `Lm` `<super> 0053` (NFKC → `S`) and U+20C1 is `Sc`, so both leak the moment the
    freeze filter stops running first (probe: `uv run --no-project --with 'unicodedata2==17.0.0'`)
- **A data-driven fixture is self-referential — assert its CONTENT, not just its shape** (iter 141):
    the vector tests compare the implementation against the fixture, so cases swapped for ASCII
    no-ops keep everything green. The ungated metadata guard therefore asserts the exact non-ASCII
    code-point set per section (runs under `--no-default-features` too). Mutation-probe any such
    guard — edit the JSON, watch it fail, `git checkout --` the file
- `gen_meta_code_v0`: `name` required (non-empty after cleaning), `description` and `meta` optional.
    Normalizes via `text_trim(text_clean(input), META_TRIM_NAME/DESCRIPTION)` BEFORE hashing
- Conformance vectors: `"stream:<hex>"` prefix in data.json denotes hex-encoded byte data. Empty
    after prefix = empty bytes. 50 total vectors (v1.3.0): 20+5+3+5+3+2+4+3+5
- **Per-algorithm internals**, settled API-parameter facts and **ISCC-IDv1** →
    `learnings-archive.md`
- `conformance_selftest` masks truncated codes bitwise — never compare full strings below 256 bits
- **ISCC decode body-length check must be EXACT (`len(tail) == nbytes`), not `>= nbytes`** — a loose
    guard silently aliases trailing base32 chars (`ISCC:...AB` == `ISCC:...ABAA`); enforced in Go
    `IsccDecode` + Rust `iscc_decode`. NOTE composite `iscc_decompose` legitimately consumes
    trailing units — do NOT harden it
- `decode_length`: multiples of 32 bits for standard MainTypes, 64 for ISCC-CODE, 8 for ID (C FFI:
    length index for 64-bit codes is 1, not 0)

## CI/CD

- Windows GHA runners default to `pwsh`. Steps using bash syntax (`$(...)`, `$GITHUB_OUTPUT`,
    `grep`, `sed`) MUST specify `shell: bash` — per-matrix version steps (e.g. `build-ffi`) hit
    Windows. Always check `shell:` when adding `run:` steps to cross-platform matrices
- **Release pipeline pattern** + `version_sync.py`'s 21 targets → `learnings-archive.md`
- **Swift release job is tag-dependent**: `build-xcframework` uses `GITHUB_REF_NAME` (not
    `Cargo.toml` like every other release job), so the `--ref main` re-trigger breaks for Swift
- **`release.yml` is `workflow_dispatch`-only — no CI run and no CID push ever exercises it.**
    Verify every edit statically (`yaml.safe_load` + regex over all 29 `if:` values,
    `actionlint@v1.7.7`). Its registry-guard shape (`!cancelled() && !failure() && (<flag>)` on all
    28 non-`prepare-release` jobs) and the `needs`-gating invariant a new job must preserve →
    `learnings-archive.md`
- **`semver` + `coverage` CI jobs**: `semver` INFORMATIONAL pre-1.0 (enforcing at v1.0.0),
    `coverage` enforcing. `mise run semver` / `mise run coverage`
- **CRAP gate (ci-cd.md)**: ENFORCING Phase 3 (`cargo crap --fail-regression --fail-above`, thresh
    30.0, max ~22.3), `.crap-baseline.json` COMMITTED (regen `mise run crap:baseline`). **CI-ONLY
    guard gap**: NOT in `mise run check`/pre-commit — a source change adding a branch to a covered
    fn lands green locally but reds CI unless the baseline is refreshed in the SAME step (never
    widen epsilon/threshold). Mechanics → `learnings-archive.md`
- **`Perf (iai-callgrind)` gate — ENFORCING (#3)**: `[profile.bench] strip = false, debug = true` is
    load-bearing (stripped binary → all benches `summary: 0` false-green) → `learnings-archive.md`
- **`Audit (cargo-deny)` gate — ENFORCING (ci-cd.md)**: root `deny.toml` (config v2,
    `yanked = "deny"`, two dev-only iai-callgrind advisories ignored) + `audit` CI job
    (`cargo-deny@0.19.9`) + `mise run audit`; reads Cargo.lock + metadata, so green locally is
    authoritative. A yanked crate or fresh RustSec advisory reds it on ANY push with no code change
    — fix with `cargo update -p <crate>` (confirm dev-only reach: `cargo tree -i <crate> -e no-dev`
    = empty), NOT a `deny.toml` ignore (ignore ONLY when no patched release exists)
- **Dependency refresh (v0.6.0 `[human]` issue) — all nine locally-verifiable slices closed as of
    iter 140**; only human/major-gated bumps remain (status in `issues.md`). **Hold-back reasons are
    inline `# held:` comments** beside the pin — confirm the stated reason from registry metadata,
    not prose: `cargo info <crate>@<ver>`, `gem specification <gem> -v <ver> --remote`,
    `https://rubygems.org/api/v1/versions/<gem>.json`
- **ruff is 0.16.0 since iter 137** (adoption history, Markdown-fence widening and hook-surface
    caveats → the two `learnings-archive.md` sections): preview a future major with
    `uvx ruff@X.Y.Z check .` — it never touches `uv.lock`. prek hooks are
    `types_or: [python, pyi, markdown]` (`ruff-format`) / `[python, pyi]` (`ruff-check`), a strict
    superset of CI. Rules go in `[tool.ruff.lint] extend-select` — **never `select`**, which drops
    ruff's `E4`/`E7`/`E9`/`F` defaults; every `[tool.ruff*]` setting carries its rationale as an
    inline comment in `pyproject.toml`. **Never `ruff check --fix .`** without `--select` — a
    blanket fix deletes the 13 load-bearing `# noqa: S603/S607` in `tools/`+`scripts/` and reds the
    pre-push security gate
- **A file-mode change needs `git update-index --chmod=+x`, not just `chmod`** (iter 136): with
    `core.fileMode=false` here a plain `chmod +x` is invisible to git — run both, prove it with
    `git ls-files -s <path>` → `100755`
- **A prek `types:` tag is not a file-extension guess — probe it** (`.pyi` is tagged `pyi`, not
    `python`; that hole silently skipped the published `_lowlevel.pyi`, closed iter 139). Prove a
    hook's real surface with a **staged, deliberately dirty** probe file:
    `uv run prek run <hook> --files <probe>` — `Skipped` means the tag misses,
    `files were modified   by this hook` proves it bites (tracked files only, so `git add` first).
    Formatter caveats → `learnings-archive.md`
- **A binding-toolchain bump can silently raise the *consumer* floor** — treat compiler/toolchain
    bumps in a *published* binding as support-policy changes reserved for Titusz, not pins. Current
    floor "Kotlin 2.3 or newer"; the `mavenLocal` proof recipe and the four docs that must move
    together → `learnings-archive.md`
- **JVM test/publish + Gradle bind-mount flake gotchas** (iter 128) → `learnings-archive.md`. Read
    it before touching `pom.xml` / `build.gradle.kts`, or before calling a Gradle error a failure
- **`cargo tree -i <crate>` prints "nothing to print" for proc-macro / target-specific deps** — add
    `--target all`. (The recurring `proc-macro-error2` future-incompat warning is dev-only and
    expected; full attribution → `issues.md` "Known constraint (verified iter 126)")
- **A floating `@vN` GitHub Action tag is a publisher convention, NOT a guarantee** — confirm with
    `gh api repos/<o>/<r>/git/matching-refs/tags/v<N>` before writing `@vN` (the `releases/latest`
    endpoint proves a release exists, not that `@vN` resolves). `astral-sh/setup-uv` stopped
    publishing floating majors after `v7` → pin `@v9.0.0` with a `# exact tag:` comment;
    `rubygems/configure-rubygems-credentials` publishes only exact tags (no `v2`). Current majors →
    `.claude/agent-memory/advance/deps-refresh.md`
- **An action-major bump is statically verifiable far past "the tag exists"** — fetch each new
    major's `action.yml` at the tag ref and diff its declared `inputs`/`outputs` against what the
    workflow passes and reads, then read every intervening major's notes for *default* changes (an
    input surviving is not its default surviving). Full recipe + the two silent biters
    (`setup-node@v5+` caching, `checkout@v6+` token location) → `learnings-archive.md`
- **ci.yml sets `cancel-in-progress: true` per ref** — pushing a follow-up develop commit cancels
    the in-flight run of the previous sha (check-runs conclude `cancelled`, not `failure`); when a
    Done-When needs green CI on a specific sha, let it conclude first. Each develop commit triggers
    TWO runs (push + `pull_request` from the open develop→main PR), so totals are ~2× the job count

## CID Process

- Never force-push to `develop` during a CID loop — agents commit incrementally (branching model
    itself is in CLAUDE.md)
- **Feature flags**: fully met, archived → `learnings-archive.md` (read before touching
    `[features]`)
- **Never trust state.md/handoff claims about external state** (registry publications, CI status,
    upstream tags) — verify at the source (`cargo search`, `npm view`, Maven Central API,
    `pip index versions`, Go module proxy, `gh api`)
- **Detect concurrent CID loops** (iter 97): context files changing mid-review, or `mise run check`
    reporting spurious "files were modified by this hook" on a file advance never touched, means a
    race. Confirm with `ps aux | grep -E 'cid:run|claude -p CID iteration'`, then flag HUMAN REVIEW
    REQUESTED — do NOT kill processes yourself, and do NOT push
- **Human-handoff vs IDLE (iter 111)**: when autonomous work runs out but `normal` issues remain
    that are all `HUMAN REVIEW REQUESTED` spec amendments, strict `**IDLE**` (all issues `low`) is
    NOT met. Flag `**HUMAN REVIEW REQUESTED**` (runner "pause") instead of `**IDLE**` (which runs
    meta-improve, reserved for all-`low`). Don't manufacture churn to avoid the pause
- **Pre-push mdformat blocks on non-conforming context files**: the pre-push hook runs mdformat
    (`--wrap 100 --number`, isolated `mdformat-mkdocs[recommended]` env) on every file in the push
    range — incl. `next.md` and per-agent `MEMORY*.md`. A non-conforming file rejects the whole
    batch push even though staged-only `git commit` passed. define-next MUST run `mise run format`
    before committing; review can unblock by reformatting + amending (match hook args exactly)
- **Never write an exact count or a substring `grep -c` into a verification criterion** (iter 139:
    `ruff format --check` saw 155 files, not 153 — the count drifts whenever a tracked
    `.md`/`.py`/`.pyi` lands, CID's own memory files included; `grep -c 'exclude'` returned 2, not
    0, matching pre-existing `--force-exclude` flags). Assert the *gate* (exit code) and anchor
    greps; advance should report the mismatch and prove the intent, not chase the number
- **next.md must never task advance with editing `issues.md`** (iter 135): advance's protocol
    forbids writing it and review owns issue progress/resolution. A slice-progress ledger paragraph
    belongs in the handoff Notes for review to append — advance correctly refused and quoted it
- **Role model assignment (2026-07)**: `advance` runs on Claude Fable 5 (`model: fable`,
    `effort: xhigh`, runner timeout 3600s); all other roles on `opus`. Deliberate diversity — do not
    "unify" onto one model
