# Learnings

High-signal pitfalls, patterns, and verified conventions accumulated during CID iterations. The
review agent maintains this file — append new entries, prune stale ones, archive completed-phase
entries to `learnings-archive.md`.

**Size budget:** Keep under 200 lines. When this file exceeds 200 lines, move entries about
fully-met target sections to `learnings-archive.md`.

## Architecture

- Hub-and-spoke: `iscc-lib` (pure Rust core) → 7 binding crates (py, napi, wasm, ffi, jni, go, rb).
    Each binding depends only on `iscc-lib`, never on another binding
- Tier 1 API (32 symbols) exposed via `pub use` at crate root. Tier 2 is `pub(crate)` — internal
    only, never crosses FFI boundary
- Sync core, async boundaries: Rust core is synchronous. Each binding adapts idiomatically

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
- **Perf-gate tooling is install-on-demand, NOT in the devcontainer** (iter 124): CI installs
    valgrind + `iai-callgrind-runner@0.16.1` per-run in the `Perf` job (`ci.yml:289-300`), and the
    devcontainer intentionally mirrors this (no bake-in). On a fresh container
    `mise run bench:iai:check` dies with "No such file or directory" until
    `sudo apt-get install -y valgrind` + `cargo binstall -y iai-callgrind-runner --version 0.16.1`
    (version MUST match the `iai-callgrind` pin). Not a devcontainer bug — do not file an issue

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
- `soft_hash_meta_v0` interleaves name and description features at the nibble level. Trim lengths
    are in bytes, not characters. The returned bytes are the raw SimHash digest
- `gen_text_code_v0` uses MinHash (not SimHash) for the content hash portion. `alg_minhash_256`
    produces 256 bits (32 bytes) from a set of n-gram features. Text n-gram size = 13 (characters)
- `gen_data_code_v0` uses MinHash on CDC chunk hashes. CDC splits binary data into content-defined
    chunks, each chunk is xxh32-hashed (not BLAKE3), the set of chunk hashes is MinHash'd
- `soft_hash_audio_v0` is a 3-stage hash: Chromaprint i32 array → 4-byte big-endian digests →
    SimHash (overall 4B + quarters 16B + sorted thirds 12B) = 32 bytes total
- `alg_simhash` output length equals input digest length (e.g., 4 bytes for 4-byte digests). Returns
    32 zero bytes only for empty input. NOT always 256 bits
- `gen_mixed_code_v0` processes multiple content codes: sorts by MainType, groups by SubType,
    soft-hashes each group, then SimHash across groups. The input is a list of ISCC strings (units),
    not raw data
- MainType Ord: MainType enum values are ordered for consistent processing. META=0, SEMANTIC=1,
    CONTENT=2, DATA=3, INSTANCE=4, ISCC=5, ID=6, FLAKE=7
- `encode_units` produces a single bitfield encoding an ordered list of content components included
    in an ISCC-CODE. Used by `gen_iscc_code_v0` to record which units were combined
- DCT uses Nayuki's algorithm (not FFTW/scipy). Image-Code: 8×8 pixel blocks → per-block DCT →
    WTA-Hash across blocks. Video-Code: per-frame DCT → WTA-Hash per frame → SimHash across frames
- JSON `meta` parameter: uses JCS (RFC 8785) canonicalization. `@context` key triggers
    `application/ld+json` media type, otherwise `application/json`
- `conformance_selftest` uses bitwise-AND masking for truncated codes — do NOT compare full strings
    when bit_length < 256
- **ISCC decode body-length check must be EXACT (`len(tail) == nbytes`), not `>= nbytes`**: a
    `< nbytes` guard silently aliases trailing base32 chars (`ISCC:...AB` == `ISCC:...ABAA`).
    Conformance-safe (every vendored vector round-trips exactly). Enforced in Go `IsccDecode` (iter
    120\) + Rust core `iscc_decode` (iter 121, two-branch "too short"/"too long"; all 11 bindings
    inherit). NOTE composite `iscc_decompose` legitimately consumes trailing units — do NOT harden
    it
- `decode_length` returns multiples of 32 bits for standard MainTypes, multiples of 64 for
    ISCC-CODE, and multiples of 8 for ID (C FFI: length index for 64-bit codes is 1, not 0)
- **ISCC-IDv1** (`gen_iscc_id_v1`, Go-only, experimental) archived iter 124 → `learnings-archive.md`

## CI/CD

- Windows GHA runners default to `pwsh`. Steps using bash syntax (`$(...)`, `$GITHUB_OUTPUT`,
    `grep`, `sed`) MUST specify `shell: bash` — per-matrix version steps (e.g. `build-ffi`) hit
    Windows. Always check `shell:` when adding `run:` steps to cross-platform matrices
- **Release pipeline pattern**: boolean input → build → smoke test → publish; 6 smoke test jobs
    (test-wheels/napi/wasm/gem/jni/ffi) gate publish, each testing the linux-x86_64 artifact
- **Adding a Python wheel target (#49)** archived iter 124 → `learnings-archive.md`
- **Tag-triggered vs dispatch-triggered releases**: `workflow_dispatch` with `--ref v<tag>` checks
- **Swift release job is tag-dependent**: `build-xcframework` uses `GITHUB_REF_NAME` (not
    `Cargo.toml` like all other release jobs) for version/tag, so the `--ref main` re-trigger breaks
    for Swift — needs a spec fix to derive version from `Cargo.toml`
- **Release input count**: 9 boolean inputs (crates-io, pypi, npm, maven, ffi, rubygems, nuget,
    maven-kotlin, swift); re-trigger individual registries with `--ref main`. `version_sync.py`
    manages 16 targets (incl. root `Package.swift` releaseTag); `--check` exits 1 on mismatch
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
    `[profile.bench] strip = false,   debug = true` load-bearing (stripped binary → all benches
    `summary: 0` false-green). Full saga → `learnings-archive.md`
- **`Audit (cargo-deny)` gate — ENFORCING (ci-cd.md)**: root `deny.toml` (config v2,
    `yanked = "deny"`, two dev-only iai-callgrind advisories ignored) + `audit` CI job
    (`cargo-deny@0.19.9`) + `mise run audit`. cargo-deny reads Cargo.lock + metadata (NOT artifacts)
    so green locally is authoritative (`cargo binstall cargo-deny@0.19.9`). A yanked crate OR fresh
    RustSec advisory reds the gate on ANY push with no code change — not a regression. Fix with
    `cargo update -p <crate>` (confirm dev-only reach via `cargo tree -i <crate> -e no-dev` =
    empty), NOT a `deny.toml` ignore — ignore ONLY when no patched release exists
- **Dependency refresh is sliced per-ecosystem** (v0.6.0 `[human]` issue). Done: slice 1 Rust
    `cargo update` (iter 124), slice 2 Python `uv lock --upgrade` (iter 125, documented `ruff<0.16`
    hold-back), slice 3 Rust direct pins (iter 126, criterion 0.5→0.7). Verify each Rust slice with
    the 4-gate set (`test`/`lint`/`audit`/`bench:iai:check`); `cargo-deny` is the main risk (new
    transitive license/advisory). **Hold-back reasons are now inline `# held:` comments in the root
    `Cargo.toml`** beside criterion/jni/magnus/uniffi — read them before proposing a bump.
    `cargo info <crate>@<ver>` prints `rust-version`: the cheapest MSRV pre-check (criterion 0.8
    needs 1.86 > our declared 1.85). GOTCHA: grep the actual pin syntax — `uniffi = "0.31"` is a
    plain string, not an inline table. Remaining: per-binding manifests, tooling pins, ruff 0.16
- **`criterion::black_box` is `#[deprecated]` from 0.6 on** — under `clippy -D warnings` that is a
    hard error, so any criterion bump past 0.5 must also move the bench import to
    `std::hint::black_box` (call sites are unchanged; `BenchmarkId`/`Throughput`/macros are stable)
- **`cargo tree -i <crate>` prints "nothing to print" for proc-macro / target-specific deps** — add
    `--target all`. The `proc-macro-error2 v2.0.1` future-incompat warning emitted on every
    `cargo test`/`cargo bench` traces to `iai-callgrind-macros` (dev-only), **not** magnus/rb-sys;
    no fixed release exists (iai-callgrind 0.16.1 is latest), so it stays a warning for now

## Branching

- `main` is protected — requires PRs with passing CI. `develop` is the CID working branch
- `mise run pr:main` creates PR from develop → main
- Never force-push to develop during a CID loop — agents commit incrementally
- Tag releases on `main` after merging from `develop`: `git tag vX.Y.Z && git push origin vX.Y.Z`

## Feature Flags

- `iscc-lib` features: `default = ["meta-code"]`, `text-processing` (unicode deps), `meta-code`
    (implies text-processing + JCS canonicalizer). Three deps are optional
- When gating `pub(crate)` functions behind features, their tests must also be gated — clippy
    `-D warnings` catches dead code in library builds even if test modules reference them
- Gate individual test functions with `#[cfg(feature = "...")]`, not the whole `mod tests` block,
    when the block contains both gated and ungated tests
- `serde_json` stays non-optional because `conformance.rs` uses it for parsing data.json vectors
- **`--no-default-features --all-targets` fails on the `benchmarks` bench** (pre-existing): benches
    import `gen_meta_code_v0`/`gen_text_code_v0` needing `meta-code`/`text-processing`. Lib + tests
    build fine. Scope clippy to the lib (`--no-default-features -- -D warnings`, no `--all-targets`)
    to avoid a false regression. CI never runs this combo
- **blake3 WASM SIMD backend — RESOLVED (#42)**: activated by the `blake3/wasm32_simd` Cargo feature
    (not `-C target-feature=+simd128`); `v128`-opcode counting alone is a FALSE-POSITIVE. Full
    recipe → `learnings-archive.md`

## State Verification

- **Never trust state.md claims about external state** (registry publications, CI status, infra) —
    frequently stale. Verify each independently against the source (`cargo search`, `npm view`,
    Maven Central API, `pip index versions`, Go module proxy); don't batch-assume "all works"/"not
    published"

## CID Process

- **Context growth**: learnings.md and agent memory grow monotonically; no agent auto-prunes.
    Archive completed-phase entries periodically to prevent token bloat
- **Detect concurrent CID loops** (iter 97): context files changing in the working tree mid-review,
    or `mise run check` reporting spurious "files were modified by this hook" on a file advance
    never touched (e.g. `standardrb-fix` with no dirty `.rb`), means a race. Check
    `ps aux | grep -E 'cid:run|claude -p CID iteration'` — two loops = duplicates. Flag HUMAN REVIEW
    REQUESTED so a human kills the duplicate; do NOT kill processes yourself, and do NOT push
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
