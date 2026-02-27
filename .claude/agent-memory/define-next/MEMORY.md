# Define-Next Agent Memory

Scoping decisions, estimation patterns, and architectural knowledge accumulated across CID
iterations.

**Size budget:** Keep under 200 lines. Archive stale entries to `MEMORY-archive.md`.

## Scope Calibration Principles

- CI job additions are small, single-file changes that provide high value. Pattern: copy existing
    job structure, swap language-specific setup action and build/test commands
- Critical issues always take priority regardless of feature trajectory
- Multiple small issues in the same crate are a natural batch (e.g., 3 fixes touching 2 files)
- README files are "create" operations — less risky than code changes. Doc files excluded from
    3-file limit
- When CI is red, formatting/lint fixes are always the first priority regardless of handoff "Next"
- Prefer concrete deliverables over research tasks when both are available
- Generic API optimization: prefer generic bounds (`AsRef<T> + Ord`) over concrete type changes to
    avoid cascading modifications across binding crates

## Architecture Decisions

- Java conformance tests use `data.json` via relative path from Maven's working directory
- Maven Surefire `-Djava.library.path` points to `target/debug/` for finding native cdylib
- Go bindings are being rewritten as pure Go (no WASM, no wazero, no binary artifacts)
- All binding conformance tests follow the same structure: load data.json, iterate per-function
    groups, decode inputs per signature, compare `.iscc` output
- `gen_iscc_code_v0` test vectors have no `wide` parameter — always pass `false`
- `"stream:<hex>"` prefix denotes hex-encoded byte data for Data/Instance-Code tests

## Go Pure Rewrite — Scoping Strategy

- **Algorithm batching**: CDC (~300 lines Rust) + MinHash (~260 lines) + SimHash+SlidingWindow (~330
    lines) = ~890 lines → 3 Go files. DCT and WTA-Hash are separate (only for Image/Video-Code)
- **Port order (dependency graph)**: codec (done) → text utils (done) → algorithms (CDC, MinHash,
    SimHash done; DCT, WTA-Hash next) → gen\_\*\_v0 functions → streaming hashers → conformance
    selftest → WASM removal
- **Coexistence pattern**: new pure Go files live alongside WASM bridge (`iscc.go`) in same `iscc`
    package. WASM removal only after all pure Go modules pass conformance
- **Codec**: stdlib only (no external deps). ~570 lines Go + ~930 lines tests
- **Text utils**: `golang.org/x/text/unicode/norm` for NFKC/NFD. ~130 lines + ~190 lines tests
- **Test verification**: `go build ./...` (compiles with WASM code) + `go test -run TestXxx` +
    `go vet ./...`. Full suite at each step (both pure Go and WASM tests pass)
- **Shared types**: `DecodeResult` struct in `iscc.go` referenced by `codec.go` (same package). When
    WASM is removed, struct moves to appropriate file
- **Step 4 (DCT+WTA-Hash)**: COMPLETE — DCT unexported `algDct`, WTA-Hash exported `AlgWtahash`
- **After step 4**: all 7 algorithm modules complete (codec, utils, CDC, MinHash, SimHash, DCT,
    WTA-Hash). Step 5 is gen functions + streaming hashers — needs 3-4 sub-steps
- **Step 5a (meta+text gen)**: COMPLETE — GenMetaCodeV0 + GenTextCodeV0 done. 3 files (xxh32.go,
    code_content_text.go, code_meta.go). BLAKE3 dep added. JCS via stdlib json.Marshal. 21/21
    conformance vectors pass
- **Step 5b (data+instance gen)**: COMPLETE — GenDataCodeV0, GenInstanceCodeV0, DataHasher,
    InstanceHasher all done. 7/7 conformance vectors pass
- **Step 5c (image+audio gen)**: GenImageCodeV0 + GenAudioCodeV0. Creates 2 files
    (code_content_image.go, code_content_audio.go). Deps: `algDct` (unexported), `AlgSimhash`.
    Image: 2D DCT → 8×8 block extraction → median comparison → bitstring (NO WTA-Hash — that's video
    only). Audio: int32 → 4-byte BE → multi-stage SimHash (full + quarters + sorted thirds). 8
    conformance vectors (3 image + 5 audio)
- **Go function naming**: pure Go gen functions are package-level (e.g., `GenMetaCodeV0`) and
    coexist with WASM bridge methods (`(rt *Runtime) GenMetaCodeV0`) — no naming conflict in Go
- **Conformance vector format**: data.json inputs are positional arrays. For data/instance:
    `[stream_hex, bits]` where stream_hex = `"stream:<hex>"`. For meta: `[name, desc, meta, bits]`.
    For text: `[text, bits]`. For image: `[pixel_array, bits]` (1024 uint8s). For audio:
    `[cv_array, bits]` (variable-length int32s)
- **Image-Code algorithm**: row-wise DCT → transpose → column-wise DCT → transpose back → extract
    four 8×8 blocks at (0,0),(1,0),(0,1),(1,1) → per-block median comparison → bitstring. Does NOT
    use WTA-Hash (that's Video-Code only). Handoff incorrectly mentioned WTA-Hash for image
- **Audio-Code simhash inner**: Rust uses `alg_simhash_inner` (pub(crate), skips validation). Go
    `AlgSimhash` validates but error is safe to discard (`_, _`) since all digests are 4-byte
    equal-length. Output is 4 bytes (not 32) when input digests are 4 bytes each
- **arraySplit helper**: needed for audio code only. Equivalent to `more_itertools.divide`. Put in
    audio code file (unexported)
- **Result types**: pure Go gen functions return rich structs (`*MetaCodeResult`, `*TextCodeResult`,
    `*DataCodeResult`, `*InstanceCodeResult`) unlike the WASM bridge which returns only
    `(string, error)`

## CI/Release Patterns

- Release workflow has `workflow_dispatch` inputs: `crates-io`, `pypi`, `npm`, `maven` (booleans)
- All publish jobs have idempotency checks (version-existence pre-check, `skip` output)
- `build-jni` matrix: 5 platforms with `native-dir`/`lib-name` matching NativeLoader conventions
- Go CI job currently builds WASM binary before running tests — must simplify after rewrite

## Recurring Patterns

- When new symbols are added to bindings but docs predate them, batch howto guide + README update
    into one step (2 files). Verification is grep-based
- Interactive sessions can break CI (e.g., Python ruff format) — always check state.md CI section
- After PR merge, always switch back to develop: `git checkout develop`

## Gotchas

- JNI function names encode Java package underscores as `_1`
- WASM howto uses `@iscc/wasm` (not `@iscc/iscc-wasm`). npm lib is `@iscc/lib`
- ISCC Foundation URL is `https://iscc.io`
- Java `byte` is signed — values 128-255 wrap, JNI handles correctly
