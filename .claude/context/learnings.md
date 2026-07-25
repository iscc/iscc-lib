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

## Tooling

- `mise` manages tool versions and tasks. Python env uses `uv`. Hooks via `prek`
- Never use `mise` in CI — call tools directly
- `cargo clippy -- -D warnings` runs in pre-push stage (not pre-commit)
- Pre-push hooks run: clippy, cargo test, pytest, ty check, ruff security/complexity
- **PyO3 is `0.29`** (issue #1 closed; iscc-py only): keep the explicit
    `#[pymodule(name = "_lowlevel", gil_used = true)]` (lib.rs:697). Per-hop recipe in
    `learnings-archive.md`; advisory clearance now IS tool-confirmable via the enforcing
    `cargo deny check` Audit gate (iter 114+)
- **Perf-gate tooling is install-on-demand, NOT in the devcontainer** (iter 124):
    `mise run bench:iai:check` dies with "No such file or directory" until
    `sudo apt-get install -y valgrind` + `cargo binstall -y iai-callgrind-runner --version 0.16.1`
    (MUST match the `iai-callgrind` pin). CI does the same per-run (`ci.yml:289-300`) and the
    devcontainer deliberately mirrors it — not a bug, do not file an issue

## ISCC Algorithm Knowledge

- `gen_meta_code_v0`: `name` required (non-empty after cleaning), `description` and `meta` optional.
    Normalizes via `text_trim(text_clean(input), META_TRIM_NAME/DESCRIPTION)` BEFORE hashing
- `META_TRIM_META` validation: pre-decode check (`META_TRIM_META * 4/3 + 256`) applies to ALL meta
    strings (both Data-URL and JSON) as a fast-path optimization. Post-decode check on
    `payload.len()` guarantees correctness. JSON boundary test overhead: `{"x":""}` = 8 bytes
- `gen_image_code_v0` pixels parameter is a flat `&[u8]`, NOT `&[i32]`. Chromaprint provides `i32`
    audio fingerprints (for `gen_audio_code_v0`), not image pixels
- `gen_instance_code_v0` accepts `bits` but ignores it — always produces 256-bit output (the hash of
    the full content). The `bits` parameter exists for API consistency only
- `gen_iscc_code_v0`: `wide` parameter determines 128-bit (default) or 256-bit combination. Data and
    Instance components are always included; content code is optional. Test vectors in data.json
    have no `wide` field — always pass `false`
- ST_ISCC SubType: for `gen_iscc_code_v0`, the SubType in the ISCC header is determined by the
    content code's SubType (TEXT/IMAGE/AUDIO/VIDEO/MIXED). When no content code is provided, SubType
    is NONE (0). SubType SUM (5) is used for `iscc_sum` (multi-asset aggregation, not in gen_iscc)
- Conformance vectors: `"stream:<hex>"` prefix in data.json denotes hex-encoded byte data. Empty
    after prefix = empty bytes. 50 total vectors (v1.3.0): 20+5+3+5+3+2+4+3+5
- **Per-algorithm internals** (meta nibble interleave, text/data MinHash, audio 3-stage SimHash,
    mixed grouping, `encode_units`, Nayuki DCT) archived iter 128 → `learnings-archive.md`
- `alg_simhash` output length equals input digest length (e.g., 4 bytes for 4-byte digests). Returns
    32 zero bytes only for empty input. NOT always 256 bits
- MainType Ord: MainType enum values are ordered for consistent processing. META=0, SEMANTIC=1,
    CONTENT=2, DATA=3, INSTANCE=4, ISCC=5, ID=6, FLAKE=7
- JSON `meta` parameter: uses JCS (RFC 8785) canonicalization. `@context` key triggers
    `application/ld+json` media type, otherwise `application/json`
- `conformance_selftest` uses bitwise-AND masking for truncated codes — do NOT compare full strings
    when bit_length < 256
- **ISCC decode body-length check must be EXACT (`len(tail) == nbytes`), not `>= nbytes`**: a
    `< nbytes` guard silently aliases trailing base32 chars (`ISCC:...AB` == `ISCC:...ABAA`).
    Enforced in Go `IsccDecode` (iter 120) + Rust core `iscc_decode` (iter 121, two-branch). NOTE
    composite `iscc_decompose` legitimately consumes trailing units — do NOT harden it
- `decode_length` returns multiples of 32 bits for standard MainTypes, multiples of 64 for
    ISCC-CODE, and multiples of 8 for ID (C FFI: length index for 64-bit codes is 1, not 0)
- **ISCC-IDv1** (`gen_iscc_id_v1`, Go-only, experimental) archived iter 124 → `learnings-archive.md`
- **The Unicode data version is an unpinned cross-implementation variable** (iter 129, issue filed):
    Go stdlib `unicode` = 15.0.0 and `x/text/unicode/norm` = 15.0.0 (its `tables17.0.0.go` is
    `//go:build go1.27`), Python 3.13 `unicodedata` = 15.1.0, but Rust `unicode-general-category`
    1.1.0 = 16.0.0 and `unicode-normalization` 0.1.25 = 17.0.0. Go's `unicode.C` **includes
    unassigned (Cn)**, so the 5,813 code points assigned in Unicode 16/17 are stripped by
    Go/`iscc-core` but kept by the Rust core → divergent `text_clean`/`text_collapse` → divergent
    Meta-Code, Text-Code and `name` fields (e.g. `Ɤ` U+A7CB). The conformance vectors are all
    Unicode ≤ 15, so no gate catches it. Do not "fix" a binding to match the core here without
    reading the issue — the version choice is a spec-level decision

## CI/CD

- Windows GHA runners default to `pwsh`. Steps using bash syntax (`$(...)`, `$GITHUB_OUTPUT`,
    `grep`, `sed`) MUST specify `shell: bash` — per-matrix version steps (e.g. `build-ffi`) hit
    Windows. Always check `shell:` when adding `run:` steps to cross-platform matrices
- **Release pipeline pattern**: boolean input → build → smoke test → publish; 6 smoke test jobs
    (test-wheels/napi/wasm/gem/jni/ffi) gate publish, each testing the linux-x86_64 artifact
- **Adding a Python wheel target (#49)** archived iter 124 → `learnings-archive.md`
- **Swift release job is tag-dependent**: `build-xcframework` uses `GITHUB_REF_NAME` (not
    `Cargo.toml` like all other release jobs) for version/tag, so the `--ref main` re-trigger breaks
    for Swift — needs a spec fix to derive version from `Cargo.toml`
- **Release input count**: 9 boolean inputs (crates-io, pypi, npm, maven, ffi, rubygems, nuget,
    maven-kotlin, swift); re-trigger individual registries with `--ref main`. `version_sync.py`
    manages 22 targets as of iter 129 (incl. root `Package.swift` releaseTag and
    `packages/kotlin/README.md`); `--check` exits 1 on mismatch
- **`semver` + `coverage` CI jobs**: `semver` INFORMATIONAL pre-1.0 (`continue-on-error: true`,
    enforcing at v1.0.0; `rust-core.md` box stays `[ ]`); `coverage` enforcing. Run via
    `mise run semver` / `mise run coverage`. Details → `learnings-archive.md`
- **CRAP gate (ci-cd.md)**: ENFORCING Phase 3 (`cargo crap --fail-regression --fail-above`, thresh
    30.0, max ~22.3), `.crap-baseline.json` COMMITTED (regen `mise run crap:baseline`),
    `.cargo-crap.toml` excludes `benches/**`. **CI-ONLY guard gap**: NOT in `mise run check`/
    pre-commit — a source change adding a branch/loop to a covered fn lands green locally but reds
    CI unless the baseline is refreshed in the SAME step (never revert the fix or widen epsilon/
    threshold). Reviewing a refresh: only the changed fn's crap/cyclomatic/coverage moves (re-sorts
    by CRAP desc); all others are pure `line:` shifts. Full mechanics → `learnings-archive.md`
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
- **Dependency refresh is sliced per-ecosystem** (v0.6.0 `[human]` issue; per-slice progress lives
    in `issues.md`). Slices 1-6 done (Cargo.lock, uv.lock, Rust pins, GHA refs, JVM manifests, Go
    module). Verify each Rust slice with the 4-gate set (`test`/`lint`/`audit`/`bench:iai:check`);
    `cargo-deny` is the main risk (new transitive license/advisory). **Hold-back reasons are inline
    `# held:` comments** beside the pin — read them before proposing a bump.
    `cargo info   <crate>@<ver>` prints `rust-version`: cheapest MSRV pre-check. GOTCHAs: grep the
    actual pin syntax (`uniffi = "0.31"` is a plain string, not an inline table); `criterion` > 0.5
    deprecates `criterion::black_box`, fatal under `-D warnings` → use `std::hint::black_box`
- **A binding-toolchain bump can silently raise the *consumer* floor** (iter 128, Kotlin): KGP
    2.1.10→2.4.10 stamps `mv=[2,4,0]` into the published jar (`javap -v -p <class> | grep mv=`) and
    puts `kotlin-stdlib:2.4.10` in the published POM. Verified with a throwaway consumer resolving
    from `mavenLocal`: Kotlin 2.1.10 and 2.2.21 fail to compile ("binary version of its metadata is
    2.4.0"), 2.3.21 passes — Kotlin tolerates ~one minor ahead. The transitive stdlib triggers the
    same error independently, so pinning `languageVersion` alone does not restore the old floor.
    Treat compiler/toolchain bumps in a *published* binding as support-policy changes, not pins
- **JVM test/publish + Gradle bind-mount flake gotchas** (iter 128) archived iter 129 →
    `learnings-archive.md`. Read it before touching `pom.xml` / `build.gradle.kts` or before calling
    a Gradle error a test failure
- **Go module slice — prove a `x/text` bump is output-neutral, don't infer it from green vectors**
    (iter 129): build a throwaway module in `/tmp` with
    `replace github.com/iscc/iscc-lib/packages/go => <repo>/packages/go`, dump
    `TextClean`/`TextCollapse` for all 1,112,032 code points, then re-run under
    `replace golang.org/x/text => golang.org/x/text v<old>` and `diff`. 0.34.0 → 0.40.0 was
    byte-identical (same `unicode/norm` tables; only invalid-rune bookkeeping changed). A new
    indirect (`golang.org/x/sys` via cpuid 2.4.0) is legitimate when `go mod tidy -diff` exits 0
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

## Feature Flags

- **Fully met — archived iter 127 → `learnings-archive.md`.** Read it before touching `[features]`
    in `crates/iscc-lib/Cargo.toml`

## CID Process

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
    Opus reviews, Codex is the independent second opinion. Do not "unify" onto one model
- **Advisor tool deferred (2026-07)**: revisit when Fable 5 is selectable → `learnings-archive.md`
