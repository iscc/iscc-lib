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
    `#[pymodule(name = "_lowlevel", gil_used = true)]` (lib.rs:697). Per-hop recipe +
    advisory-clearance caveat (`cargo audit`/`deny` absent) in `learnings-archive.md`

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
- `decode_length` returns multiples of 32 bits for standard MainTypes, multiples of 64 for
    ISCC-CODE, and multiples of 8 for ID (C FFI: length index for 64-bit codes is 1, not 0)

## CI/CD

- Windows GHA runners default to `pwsh` shell. Steps using bash syntax (`$(...)`, `$GITHUB_OUTPUT`,
    `grep`, `sed`) MUST specify `shell: bash`. Existing publish jobs avoid this by only running
    version extraction on `ubuntu-latest`, but per-matrix version steps (like in `build-ffi`) hit
    Windows. Always check `shell:` declarations when adding `run:` steps to cross-platform matrices
- **Release pipeline pattern**: boolean input → build job → smoke test job → publish job. 6 smoke
    test jobs (test-wheels, test-napi, test-wasm, test-gem, test-jni, test-ffi) gate publish. Each
    tests linux-x86_64 artifact on ubuntu-latest
- **Tag-triggered vs dispatch-triggered releases**: `workflow_dispatch` with `--ref v<tag>` checks
- **Swift release job is tag-dependent**: `build-xcframework` uses `GITHUB_REF_NAME` (not
    `Cargo.toml` like all other release jobs) for version/tag, so the `--ref main` re-trigger
    convention breaks for Swift — needs a spec fix to derive version from `Cargo.toml`
- **Release input count**: Now 9 boolean inputs (crates-io, pypi, npm, maven, ffi, rubygems, nuget,
    maven-kotlin, swift). When re-triggering individual registries, always use `--ref main`
- **Version sync**: `version_sync.py` manages 16 targets (including root `Package.swift`
    releaseTag). `--check` mode exits 1 on mismatch
- **`semver` + `coverage` CI jobs** (iter 93/94) — fully landed, details in `learnings-archive.md`.
    Key facts: `semver` is INFORMATIONAL pre-1.0 (`continue-on-error: true`, becomes enforcing at
    v1.0.0 by dropping it; `rust-core.md` line 372 checkbox stays `[ ]` until then); `coverage` is
    enforcing (no `continue-on-error`). `mise run semver` / `mise run coverage` run them locally
- **`cargo binstall` + `Swatinem/rust-cache` poisoning** (iter 100): rust-cache restores cargo's
    `.crates.toml`/`.crates2.json` install *metadata* WITHOUT the `~/.cargo/bin/<tool>` binary, so a
    plain `cargo binstall -y <tool>` sees "already installed", skips, and the next invocation dies
    with `error: no such command: <tool>` → CI RED on every run. Fix: add `--force` so binstall
    always reinstalls regardless of the cached record (small binary = negligible re-download). This
    is gate *strengthening*, not circumvention
- **CRAP gate (iter 96/97, ci-cd.md)** — full mechanics in `learnings-archive.md`. Key facts:
    `Coverage + CRAP` job runs report-only `--format github`/`sarif`, then ENFORCING
    `cargo crap --lcov lcov.info --baseline .crap-baseline.json --fail-regression`.
    `.crap-baseline.json` is COMMITTED (97 funcs/10 files); `mise run crap:baseline` regenerates it
    byte-identical. `.cargo-crap.toml` MUST list `crates/iscc-lib/benches/**` (built-in default
    matches repo-root only, else `bench_cdc_chunks` leaks at CRAP 42)
- **`--fail-regression` does NOT catch NEW high-CRAP functions** (Codex, iter 97): a brand-new
    uncovered function has no baseline entry, so it reports `★ N new` and exits 0 — the gate only
    blocks WORSENING of existing entries. To also block new risky code, pair with `--fail-above 30`
    (current max CRAP ~22.3, safely below 30). Filed as a [review] issue
- **`Perf (iai-callgrind)` CI job** (iter 107, ci-cd.md "Performance"): standalone `perf` job (no
    `needs:`, NO `continue-on-error`): apt valgrind → cargo-binstall →
    `cargo binstall -y --force iai-callgrind-runner@0.16.1` (`--force` load-bearing) →
    `cargo bench -p iscc-lib --bench iai_benches` → guard step → upload `target/iai/` as
    `iai-baseline`. First run has NO baseline so it only measures (exit 0); >10% gate + committed
    baseline = slice 2b (#3 open)
- **STRIP zero-collection bug (iter 108, FIXED — corrects the WRONG iter-107 claim)**:
    `[profile.bench]` DOES inherit `strip = true` from root `[profile.release]` (Cargo: bench is
    based on release). Verified iter-108: with no override the bench binary is `stripped` / **0**
    `__iai_callgrind_wrapper` symbols → iai's `--toggle-collect=*::__iai_callgrind_wrapper_mod::*`
    matches nothing → every bench `summary: 0` while exiting 0 (FALSE GREEN; the real CI artifact
    from run 27742285656 had all-zero `.out`s). The iter-107 review's "bench doesn't inherit release
    strip" claim was wrong (inspected a different binary). FIX:
    `[profile.bench] strip = false,   debug = true` → `not stripped` / **11** symbols → real counts.
    CI guard `grep -rEq '^summary: [1-9]' target/iai/` fails the job on zero collection (defends the
    override)
- **Running iai-callgrind locally**: valgrind 3.19 + `iai-callgrind-runner` 0.16.1 ARE in the
    devcontainer (iter-107 "valgrind absent" claim also wrong). Kernel blocks the `personality`
    syscall iai's `setarch -R` (ASLR-disable) uses → set `IAI_CALLGRIND_ALLOW_ASLR=true` to skip it
    (wired into `mise run bench:iai` + ci.yml bench step). ASLR = cache-sim noise, NOT `Ir` — safe

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
    import `gen_meta_code_v0`/`gen_text_code_v0`, which need `meta-code`/`text-processing`. Lib +
    tests build fine; only the bench target breaks. Scope clippy to the lib
    (`--no-default-features -- -D warnings`, no `--all-targets`) to avoid a false regression. CI
    never runs this combo

## Documentation Maintenance

- **"10 gen functions" vs "9 conformance functions"**: iscc-lib has 10 `gen_*_v0` functions, but
    `data.json` conformance vectors cover only 9 (no gen_sum_code_v0). Files that test/benchmark
    against data.json should say "9"; general library descriptions should say "10". Avoid blanket
    "9→10" find-and-replace — it corrupts conformance-scoped files. iscc-core-ts also implements
    only 9 (no gen_sum_code_v0) — verify external projects' function tables before claiming "all 10"

## State Verification

- **Never trust state.md claims about external state** (registry publications, CI status, infra) —
    frequently stale. Verify each independently against the source (`cargo search`, `npm view`,
    Maven Central API, `pip index versions`, Go module proxy); don't batch-assume "all works"/"not
    published"

## CID Process

- **Context growth**: learnings.md and agent memory grow monotonically; no agent auto-prunes.
    Archive completed-phase entries periodically to prevent token bloat
- **Detect concurrent CID loops** (iter 97): if `state.md`/context files change in the working tree
    mid-review, or `mise run check` reports spurious "files were modified by this hook" on a file
    the advance never touched (e.g. `standardrb-fix` flagging when no `.rb` is dirty), suspect a
    race. Check `ps aux | grep -E 'cid:run|claude -p CID iteration'`: TWO `mise run cid:run` or two
    different `iteration N` agents = duplicate loops racing the same branch — they clobber context
    files and race pushes. Flag HUMAN REVIEW REQUESTED so a human kills the duplicate; do NOT kill
    processes yourself, and do NOT push (the second loop will collide)
- **Pre-push mdformat blocks on non-conforming context files**: the pre-push hook runs mdformat
    (`--wrap 100 --number`, isolated `mdformat-mkdocs[recommended]` env) on every file changed in
    the push range — incl. `next.md` and per-agent `MEMORY*.md`. A non-conforming file rejects the
    whole batch push even though staged-only `git commit` passed. define-next MUST run
    `mise run format` before committing; review can unblock by reformatting + amending (match the
    hook args exactly — local plugin set differs)

## Devcontainer Scripts (exec bit / Windows bind mount)

- Windows bind mount uses `core.fileMode = false`, so git ignores on-disk exec bits — invoke
    devcontainer scripts via `bash foo.sh` and keep convenience steps non-fatal. Full write-up in
    `learnings-archive.md`.
