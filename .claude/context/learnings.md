# Learnings

High-signal pitfalls, patterns, and verified conventions accumulated during CID iterations. The
review agent maintains this file — append new entries, prune stale ones, archive completed-phase
entries to `learnings-archive.md`.

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
- **`iscc-core` output is not stable across CPython versions.** `text_clean`/`text_collapse` strip
    top-level Unicode category `C`, which includes unassigned `Cn`, so the result depends on
    `unicodedata.unidata_version` (15.1.0 on 3.13, 16.0.0 on 3.14 — 5,185 code points differ).
    "Matches the reference" is meaningless without naming the interpreter. Check both runtimes with
    `uv run --python 3.13 --no-project --with iscc-core python …` (and `3.14`) — no project env
    needed Upstream: <https://github.com/iscc/iscc-core/issues/137>
- Any dependency carrying Unicode/locale tables must be proven output-neutral by a **differential
    sweep**, not by a green vector suite: dump the function output for all 1,112,032 code points
    before and after the bump and `diff`. Costs ~2 minutes and is the only thing that catches table
    drift — every vendored conformance vector predates Unicode 16

## Tooling

- `mise` manages tool versions and tasks. Python env uses `uv`. Hooks via `prek`
- Never use `mise` in CI — call tools directly
- `cargo clippy -- -D warnings` runs in pre-push stage (not pre-commit)
- Pre-push hooks run: clippy, cargo test, pytest, ty check, ruff security/complexity
- **PyO3 is `0.29`** (issue #1 closed; iscc-py only): keep the explicit
    `#[pymodule(name = "_lowlevel", gil_used = true)]` (lib.rs:697). Per-hop recipe in
    `learnings-archive.md`; advisory clearance now IS tool-confirmable via the enforcing
    `cargo deny check` Audit gate (iter 114+)
- **`_lowlevel.pyi` stub bodies are docstring-only — no trailing `...`** (iter 131). The wheel ships
    `py.typed` + the stub, so it is consumer-facing: verified accepted by `ty`, `mypy 1.18 --strict`
    and `pyright 1.1.407`, and `ruff format` leaves docstring-only bodies alone (no blank-line
    churn). ruff 0.16 double-reports a docstring + `...` as `PIE790` *and* `PYI048` on the same
    line, so N findings collapse to N/2 deletions. When changing a published `.pyi`, check it
    against mypy/pyright too — the repo gates only run `ty`
- **Perf-gate tooling is install-on-demand, NOT in the devcontainer** (iter 124):
    `mise run bench:iai:check` dies with "No such file or directory" until
    `sudo apt-get install -y valgrind` + `cargo binstall -y iai-callgrind-runner --version 0.16.1`
    (MUST match the `iai-callgrind` pin). CI does the same per-run (`ci.yml:289-300`) and the
    devcontainer deliberately mirrors it — not a bug, do not file an issue

## ISCC Algorithm Knowledge

- `gen_meta_code_v0`: `name` required (non-empty after cleaning), `description` and `meta` optional.
    Normalizes via `text_trim(text_clean(input), META_TRIM_NAME/DESCRIPTION)` BEFORE hashing
- Conformance vectors: `"stream:<hex>"` prefix in data.json denotes hex-encoded byte data. Empty
    after prefix = empty bytes. 50 total vectors (v1.3.0): 20+5+3+5+3+2+4+3+5
- **Per-algorithm internals** (meta nibble interleave, text/data MinHash, audio 3-stage SimHash,
    mixed grouping, `encode_units`, Nayuki DCT) plus settled API-parameter facts (`META_TRIM_META`
    pre/post-decode checks, `gen_image_code_v0` flat `&[u8]` pixels, MainType Ord, JCS `meta`,
    `alg_simhash` length, `gen_instance_code_v0`'s ignored `bits`, `gen_iscc_code_v0`'s `wide`,
    ST_ISCC SubType derivation) archived iters 128/131 → `learnings-archive.md`
- `conformance_selftest` uses bitwise-AND masking for truncated codes — do NOT compare full strings
    when bit_length < 256
- **ISCC decode body-length check must be EXACT (`len(tail) == nbytes`), not `>= nbytes`**: a
    `< nbytes` guard silently aliases trailing base32 chars (`ISCC:...AB` == `ISCC:...ABAA`).
    Enforced in Go `IsccDecode` (iter 120) + Rust core `iscc_decode` (iter 121, two-branch). NOTE
    composite `iscc_decompose` legitimately consumes trailing units — do NOT harden it
- `decode_length` returns multiples of 32 bits for standard MainTypes, multiples of 64 for
    ISCC-CODE, and multiples of 8 for ID (C FFI: length index for 64-bit codes is 1, not 0)
- **ISCC-IDv1** (`gen_iscc_id_v1`, Go-only, experimental) archived iter 124 → `learnings-archive.md`
- **The Unicode data version is an unpinned cross-implementation variable** (iter 129, open
    `[review]` issue): Go stdlib + `x/text/norm` = 15.0.0, Python 3.13 = 15.1.0, but Rust
    `unicode-general-category` = 16.0.0 / `unicode-normalization` = 17.0.0. Go's `unicode.C`
    includes unassigned (Cn), so 5,813 post-15 code points are stripped by Go/`iscc-core` but kept
    by the Rust core → divergent `text_clean`/`text_collapse` → divergent Meta/Text codes and `name`
    (e.g. `Ɤ` U+A7CB). Vectors are all Unicode ≤ 15, so no gate catches it. Do not "fix" a binding
    to match the core — the version choice is a spec-level decision. Evidence → `issues.md`

## CI/CD

- Windows GHA runners default to `pwsh`. Steps using bash syntax (`$(...)`, `$GITHUB_OUTPUT`,
    `grep`, `sed`) MUST specify `shell: bash` — per-matrix version steps (e.g. `build-ffi`) hit
    Windows. Always check `shell:` when adding `run:` steps to cross-platform matrices
- **Release pipeline pattern**: boolean input → build → smoke test → publish; 6 smoke test jobs
    (test-wheels/napi/wasm/gem/jni/ffi) gate publish, each testing the linux-x86_64 artifact. Adding
    a Python wheel target (#49) → `learnings-archive.md`
- **Swift release job is tag-dependent**: `build-xcframework` uses `GITHUB_REF_NAME` (not
    `Cargo.toml` like all other release jobs) for version/tag, so the `--ref main` re-trigger breaks
    for Swift — needs a spec fix to derive version from `Cargo.toml`
- **Release input count**: 9 boolean inputs (crates-io, pypi, npm, maven, ffi, rubygems, nuget,
    maven-kotlin, swift); re-trigger individual registries with `--ref main`. `version_sync.py`
    manages 22 targets as of iter 129; `--check` exits 1 on mismatch
- **`semver` + `coverage` CI jobs**: `semver` INFORMATIONAL pre-1.0 (`continue-on-error: true`,
    enforcing at v1.0.0; `rust-core.md` box stays `[ ]`); `coverage` enforcing. Run via
    `mise run semver` / `mise run coverage`. Details → `learnings-archive.md`
- **CRAP gate (ci-cd.md)**: ENFORCING Phase 3 (`cargo crap --fail-regression --fail-above`, thresh
    30.0, max ~22.3), `.crap-baseline.json` COMMITTED (regen `mise run crap:baseline`),
    `.cargo-crap.toml` excludes `benches/**`. **CI-ONLY guard gap**: NOT in `mise run check`/
    pre-commit — a source change adding a branch/loop to a covered fn lands green locally but reds
    CI unless the baseline is refreshed in the SAME step (never revert the fix or widen epsilon/
    threshold). Full mechanics + how to review a refresh → `learnings-archive.md`
- **`Perf (iai-callgrind)` gate — COMPLETE, ENFORCING (#3)**:
    `[profile.bench] strip = false,   debug = true` is load-bearing (stripped binary → all benches
    `summary: 0` false-green). Full saga → `learnings-archive.md`
- **`Audit (cargo-deny)` gate — ENFORCING (ci-cd.md)**: root `deny.toml` (config v2,
    `yanked = "deny"`, two dev-only iai-callgrind advisories ignored) + `audit` CI job
    (`cargo-deny@0.19.9`) + `mise run audit`. cargo-deny reads Cargo.lock + metadata (NOT artifacts)
    so green locally is authoritative (`cargo binstall cargo-deny@0.19.9`). A yanked crate OR fresh
    RustSec advisory reds the gate on ANY push with no code change — not a regression. Fix with
    `cargo update -p <crate>` (confirm dev-only reach via `cargo tree -i <crate> -e no-dev` =
    empty), NOT a `deny.toml` ignore — ignore ONLY when no patched release exists
- **Dependency refresh is sliced per-ecosystem** (v0.6.0 `[human]` issue; per-slice status lives in
    `issues.md` — all 7 ecosystem slices done; ruff 0.16, `release.yml` refs and deferred majors
    remain). Verify each Rust slice with the 4-gate set (`test`/`lint`/`audit`/`bench:iai:check`);
    `cargo-deny` is the main risk (new transitive license/advisory). **Hold-back reasons are inline
    `# held:` comments** beside the pin — confirm the stated reason from registry metadata, not
    prose: `cargo info <crate>@<ver>` (`rust-version`), `gem specification <gem> -v <ver> --remote`
    (transitive pins), `https://rubygems.org/api/v1/versions/<gem>.json` (`ruby_version`; the v2
    endpoint returns `null`). GOTCHAs: grep the actual pin syntax (`uniffi = "0.31"` is a plain
    string, not an inline table); `criterion` > 0.5 deprecates `criterion::black_box`, fatal under
    `-D warnings` → use `std::hint::black_box`
- **ruff 0.16 adoption is sliced by decision type, not by file** (iters 125/131): run the unpinned
    version with `uvx ruff@0.16.0 check .` — it never touches `uv.lock`, so `ruff<0.16` stays in
    `pyproject.toml` until the tree is clean. Baseline 104 → 26 after slice A. **Never
    `ruff@0.16 check --fix .`**: 15 of the remaining findings are `RUF100` on the load-bearing
    `# noqa: S603/S607` in `tools/`+`scripts/` — 0.16 calls them unused only because `S` is not in
    the default select, and deleting them reds the pre-push `ruff check --select S` gate. Their
    resolution is a lint-config decision (e.g. add `S`/`C901` to `select`), never deletion
- **Ruby gem dev deps (iter 130)**: bundler has no `-C` — use `(cd crates/iscc-rb && bundle …)` with
    `$(ruby -e "puts Gem.user_dir")/bin` on PATH. CI's `ruby/setup-ruby` `bundler-cache: true` is a
    **frozen** install, so prove lock/Gemfile consistency with
    `BUNDLE_FROZEN=true bundle install --local`. `rb_sys` stays pinned EXACTLY at 0.9.123 (it pins
    `rake-compiler-dock = 1.10.0`; 0.9.124 → 1.11.0, 0.9.128 → 1.12.0) to match `tag: 0.9.123` of
    `oxidize-rb/actions/cross-gem` in `release.yml`
- **A binding-toolchain bump can silently raise the *consumer* floor** (iter 128, Kotlin — open
    `[review]` issue): KGP 2.1.10→2.4.10 stamps `mv=[2,4,0]` into the published jar and
    `kotlin-stdlib:2.4.10` into the published POM; a throwaway `mavenLocal` consumer proved Kotlin
    2.1.10/2.2.21 fail and 2.3.21 passes. Treat compiler/toolchain bumps in a *published* binding as
    support-policy changes reserved for Titusz, not pins. Full evidence → `issues.md`
- **JVM test/publish + Gradle bind-mount flake gotchas** (iter 128) archived iter 129 →
    `learnings-archive.md`. Read it before touching `pom.xml` / `build.gradle.kts` or before calling
    a Gradle error a test failure
- **A dep that ships DATA TABLES (Unicode, locale, tz) needs an exhaustive differential, not green
    vectors** (iter 129, `golang.org/x/text` 0.34.0 → 0.40.0): the 50 vendored vectors are all
    Unicode ≤ 15 and cannot detect a table change. Recipe (throwaway `/tmp` module + `replace` to
    the old dep version, dump the whole input space, `diff`) → `learnings-archive.md`
- **`cargo tree -i <crate>` prints "nothing to print" for proc-macro / target-specific deps** — add
    `--target all`. The `proc-macro-error2 v2.0.1` future-incompat warning emitted on every
    `cargo test`/`cargo bench` traces to `iai-callgrind-macros` (dev-only), **not** magnus/rb-sys;
    no fixed release exists (iai-callgrind 0.16.1 is latest), so it stays a warning for now
- **A floating `@vN` GitHub Action tag is a publisher convention, NOT a guarantee** —
    `gh api repos/<o>/<r>/releases/latest` proves a release exists, not that `@vN` resolves. Always
    confirm with `gh api repos/<o>/<r>/git/matching-refs/tags/v<N>` before writing `@vN`.
    `astral-sh/setup-uv` stopped publishing floating majors after `v7` (v8.x/v9.0.0 are exact tags
    only) → pin `@v9.0.0` with a `# exact tag:` comment; it recurs in `release.yml`. Current majors
    as of 2026-07-24 are recorded in `.claude/agent-memory/advance/deps-refresh.md`
    (`upload-pages-artifact` + `deploy-pages` must move as a pair)
- **ci.yml sets `cancel-in-progress: true` per ref** — pushing a follow-up develop commit cancels
    the in-flight run of the previous sha (its check-runs conclude `cancelled`, not `failure`). When
    a step's Done-When needs a green CI on a specific sha, let that run conclude before pushing
    again. Each develop commit also triggers TWO runs (push + `pull_request` from the open
    develop→main PR), so check-run totals are ~2× the job count

## Branching

- `main` is protected — requires PRs with passing CI. `develop` is the CID working branch
- `mise run pr:main` creates PR from develop → main
- Never force-push to develop during a CID loop — agents commit incrementally
- Tag releases on `main` after merging from `develop`: `git tag vX.Y.Z && git push origin vX.Y.Z`

## CID Process

- **Feature flags: fully met — archived iter 127 → `learnings-archive.md`.** Read it before touching
    `[features]` in `crates/iscc-lib/Cargo.toml`

- **Never trust state.md/handoff claims about external state** (registry publications, CI status,
    upstream tags) — frequently stale. Verify independently against the source (`cargo search`,
    `npm view`, Maven Central API, `pip index versions`, Go module proxy, `gh api`)

- **Context growth**: learnings.md and agent memory grow monotonically; no agent auto-prunes.
    Archive completed-phase entries periodically to prevent token bloat

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

- **Role model assignment (2026-07)**: `advance` runs on Claude Fable 5 (`model: fable`,
    `effort: xhigh`) — long-horizon implementation, single requests can run many minutes (runner
    timeout 3600s). All other roles run on `opus`. Deliberate model diversity: Fable implements,
    Opus reviews, Codex is the independent second opinion. Do not "unify" onto one model. (Advisor
    tool deferred 2026-07 — revisit when Fable 5 is selectable; detail → `learnings-archive.md`)
