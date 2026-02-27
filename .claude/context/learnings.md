# Learnings

High-signal pitfalls, patterns, and verified conventions accumulated during CID iterations. The
review agent maintains this file — append new entries, prune stale ones, archive completed-phase
entries to `learnings-archive.md`.

**Size budget:** Keep under 200 lines. When this file exceeds 200 lines, move entries about
fully-met target sections to `learnings-archive.md`.

## Architecture

- Hub-and-spoke: `iscc-lib` (pure Rust core) → 6 binding crates. Each binding depends only on
    `iscc-lib`, never on another binding
- Tier 1 API (30 symbols) exposed via `pub use` at crate root. Tier 2 is `pub(crate)` — internal
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

## ISCC Algorithm Knowledge

- `gen_meta_code_v0`: `name` required (non-empty after cleaning), `description` and `meta` optional.
    Normalizes via `text_trim(text_clean(input), META_TRIM_NAME/DESCRIPTION)` BEFORE hashing
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
    after prefix = empty bytes. 46 total vectors: 16+5+3+5+3+2+4+3+5
- `soft_hash_meta_v0` interleaves name and description features at the nibble level. Trim lengths
    are in bytes, not characters. The returned bytes are the raw SimHash digest
- `gen_text_code_v0` uses MinHash (not SimHash) for the content hash portion. `alg_minhash_256`
    produces 256 bits (32 bytes) from a set of n-gram features. Text n-gram size = 13 (characters)
- `gen_data_code_v0` uses MinHash on CDC chunk hashes. CDC splits binary data into content-defined
    chunks, each chunk is BLAKE3-hashed, the set of chunk hashes is MinHash'd
- `soft_hash_audio_v0` is a multi-stage hash: Chromaprint i32 array → overlapping frames of
    `frame_size` i32s → each frame's individual bits → SimHash across frames → byte digest
- `alg_simhash` output length is always 32 bytes (256 bits) regardless of input. The function hashes
    feature vectors (each a byte slice) into a single 256-bit fingerprint
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
- `conformance_selftest` bitwise AND masking for truncated codes — do NOT compare full strings when
    bit_length < 256
- `decode_length` returns multiples of 32 bits for standard MainTypes, multiples of 64 for
    ISCC-CODE, and multiples of 8 for ID
- C FFI decode: length index for 64-bit codes is 1 (not 0) — `decode_length` uses
    `(length_index + 1) * 32`

## Branching

- `main` is protected — requires PRs with passing CI. `develop` is the CID working branch
- `mise run pr:main` creates PR from develop → main
- Never force-push to develop during a CID loop — agents commit incrementally
- Tag releases on `main` after merging from `develop`: `git tag vX.Y.Z && git push origin vX.Y.Z`

## Go Bindings — Pure Go Rewrite

- Go module path: `github.com/iscc/iscc-lib/packages/go`, package name `iscc`
- Conformance test path: `../../crates/iscc-lib/tests/data.json` (relative from packages/go)
- Go constants: `MetaTrimName`, `MetaTrimDescription`, `IoReadSize`, `TextNgramSize` are
    package-level `const` (idiomatic Go)
- `DecodeResult` struct: `Maintype`, `Subtype`, `Version`, `Length` (all `uint8`) + `Digest`
    (`[]byte`). Returned as `*DecodeResult` from `IsccDecode`
- Go uint32/uint64 arithmetic wraps naturally at overflow, matching Rust's wrapping_add/wrapping_mul
- Go `%` and `&` have equal precedence (both multiplicative), so `x % mprime & maxH` evaluates
    left-to-right as `(x % mprime) & maxH`, matching Rust
- Use `const` (not `var`) for scalar constants — Go supports constant expressions with bit shifts
- `golang.org/x/text/unicode/norm` for NFKC/NFD. `unicode.Is(unicode.C, c)` covers Cc, Cf, Co, Cs
- `TextRemoveNewlines` = `strings.Join(strings.Fields(text), " ")` (one-liner)
- `TextTrim` uses backward byte trimming until `utf8.ValidString` — simpler than Rust but identical
- CDC: `cdcGear` table is `var` not `const` (Go no const arrays). `min()` builtin since Go 1.21+
- MinHash: `mpa`/`mpb` arrays, `minhashFn` naming (avoids Go conflict). `maxi64`/`mprime`/`maxH` are
    `var` not `const` (Go uint64 shift limitation)
- SimHash: `AlgSimhash` returns `([]byte, error)`, `SlidingWindow` returns `([]string, error)`. Uses
    `[]rune` for Unicode-correct SlidingWindow
- Dependency order: codec → utils → algorithms → gen functions → streaming → conformance → cleanup

## CID Process

- **issues.md stale entry gap**: The review agent only cleans up issues resolved in the current
    iteration's advance step — it does NOT sweep the full issues.md backlog. Fix: review agent
    should scan all issues.md entries against state.md "met" sections after reviewing advance work
- **Context growth**: learnings.md and agent memory files grow monotonically. No agent autonomously
    prunes. Manual cleanup required periodically. Archive completed-phase entries to prevent token
    bloat
