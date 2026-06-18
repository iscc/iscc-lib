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
- **PyO3 migration 0.23→0.29 COMPLETE** (issue #1 closed, iter 105): `pyo3` lives only in root
    `Cargo.toml` `[workspace.dependencies]` (used by `iscc-py` alone). The 0.28→0.29 final hop was
    ZERO-edit (lib.rs unchanged); 0.29.0 ships both targeted RustSec advisory fixes, so the lockfile
    resolves a single `pyo3 0.29.0` and the wheel no longer carries vulnerable code. Keep the
    explicit `#[pymodule(name = "_lowlevel", gil_used = true)]` (lib.rs:697). Full per-hop recipe +
    silent-gotcha catalog (0.26 detach/`Py<PyAny>`, 0.27 cast rename, 0.28 `gil_used` flip) archived
    in `learnings-archive.md`. CAVEAT: advisories NOT tool-confirmable — `cargo audit`/`cargo deny`
    absent from devcontainer + CI despite `notes/07` mandating it (proxy: no pyo3 \<0.29 in lock)

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
    ISCC-CODE, and multiples of 8 for ID
- C FFI decode: length index for 64-bit codes is 1 (not 0) — `decode_length` uses
    `(length_index + 1) * 32`

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
- **`semver` CI job** (`ci.yml`, iter 93): `obi1kenobi/cargo-semver-checks-action@v2`,
    `package: iscc-lib`, baseline = last crates.io release (auto-detected). INFORMATIONAL pre-1.0
    via `continue-on-error: true` — it reports the post-0.4.0 `pub(crate)` narrowing of
    cdc/conformance/minhash/simhash/utils as `module_missing`/`function_missing` (2 major checks
    failed; expected, not a regression). `mise run semver` runs it locally. Becomes enforcing at
    v1.0.0 by dropping `continue-on-error`; `rust-core.md` line 372 checkbox stays `[ ]` until then
- **`coverage` CI job** (`ci.yml`, iter 94, ci-cd.md Phase 1): standalone, no `needs:`, NO
    `continue-on-error`. `dtolnay/rust-toolchain@stable` w/ `components: llvm-tools-preview` →
    `taiki-e/install-action@v2` (`tool: cargo-llvm-cov`) →
    `cargo llvm-cov -p iscc-lib --lcov --output-path lcov.info` → upload-artifact (`name: lcov`).
    `mise run coverage` mirrors it locally; `lcov.info` is gitignored (137KB / 5156 lines). CI job
    entries now 17 (python-test matrix → 18 actual)
- **`cargo binstall` + `Swatinem/rust-cache` poisoning** (iter 100): rust-cache restores cargo's
    `.crates.toml`/`.crates2.json` install *metadata* WITHOUT the `~/.cargo/bin/<tool>` binary, so a
    plain `cargo binstall -y <tool>` sees "already installed", skips, and the next invocation dies
    with `error: no such command: <tool>` → CI RED on every run. Fix: add `--force` so binstall
    always reinstalls regardless of the cached record (small binary = negligible re-download). This
    is gate *strengthening*, not circumvention
- **CRAP gate (iter 96 Phase 2 + iter 97 Phase 3, ci-cd.md)**: `Coverage + CRAP` job installs
    `cargo binstall -y --force cargo-crap@0.2.2` (`--force` LOAD-BEARING, see entry above), runs
    report-only `--format github` + `--format sarif` (`upload-sarif@v3`, job-level
    `security-events: write`), then an ENFORCING final step
    `cargo crap --lcov lcov.info --baseline .crap-baseline.json --fail-regression` (NOT
    continue-on-error). `.crap-baseline.json` (repo root, COMMITTED, NOT gitignored — only
    `lcov.info`/`crap.sarif` are): envelope `{$schema, version, entries}`, 97 iscc-lib functions /
    10 files. `mise run crap:baseline` regenerates it byte-identical (idempotent).
    `.cargo-crap.toml` `threshold=30`, `missing="pessimistic"`, MUST list
    `crates/iscc-lib/benches/**` explicitly (the built-in `benches/**` default only matches
    repo-root, else `bench_cdc_chunks` leaks at CRAP 42)
- **`--fail-regression` does NOT catch NEW high-CRAP functions** (Codex, iter 97): a brand-new
    uncovered function has no baseline entry, so it reports `★ N new` and exits 0 — the gate only
    blocks WORSENING of existing entries. To also block new risky code, pair with `--fail-above 30`
    (current max CRAP ~22.3, safely below 30). Filed as a [review] issue

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
- **`--no-default-features --all-targets` fails on the `benchmarks` bench** (pre-existing):
    `benches` import `gen_meta_code_v0`/`gen_text_code_v0`, which need the
    `meta-code`/`text-processing` features. The lib + tests build fine with `--no-default-features`;
    only the bench target breaks. When verifying feature configs, scope clippy to the lib
    (`--no-default-features -- -D warnings`, no `--all-targets`) or it reports a false regression.
    CI never runs this combo

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
    the push range — including `next.md` and per-agent `MEMORY*.md`. A non-conforming
    `next.md`/memory (wrong wrap width, misindented fenced code) rejects the whole batch push even
    though `git commit` (staged-only hooks) passed. define-next MUST run
    `uv run mdformat --wrap 100 --number` (or `mise run format`) before committing. Review can
    unblock by reformatting those files (mechanical, no semantic change) and amending — but local
    `uv run mdformat` uses a different plugin set, so match the hook args exactly

## Devcontainer Scripts (exec bit / Windows bind mount)

- Windows bind mount uses `core.fileMode = false`, so git ignores on-disk exec bits — invoke
    devcontainer scripts via `bash foo.sh` and keep convenience steps non-fatal. Full write-up in
    `learnings-archive.md`.
