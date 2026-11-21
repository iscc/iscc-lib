## 2026-11-21 — Go `GenMetaCodeV0` enforces the 128,000-byte meta limit, pinned by a shared vector

**Done:** `GenMetaCodeV0` checks meta size the same way the Rust core does, in the same order.
First, a raw meta string longer than `MetaTrimMeta*4/3 + 256` (170,922 bytes) is rejected before any
decoding. Second, a decoded payload longer than `MetaTrimMeta` (Data-URL bytes or canonical JSON) is
rejected. Added the shared vector `gen_meta_code_v0/test_0004_meta_payload_over_limit` (canonical
JSON of 128,001 bytes, `error`, no `diverges`) to all 5 copies and regenerated the C header.

**Files changed:**

- `packages/go/code_meta.go`: const `metaPreDecodeLimit = MetaTrimMeta*4/3 + 256` and helper
    `checkMetaStringSize` (pre-decode, error
    `iscc: meta string exceeds size limit (%d > %d bytes)`); post-decode check inline after the
    payload is resolved (`iscc: decoded meta payload exceeds   size limit (%d > %d bytes)`)
- `packages/go/code_meta_test.go`: `TestGenMetaCodeV0MetaSizeLimit` (JSON and Data-URL, 128,000
    passes and 128,001 is rejected with the post-decode message) and
    `TestGenMetaCodeV0MetaPreDecodeLimit` (a mostly-whitespace JSON string of exactly 170,922 bytes
    passes, 170,923 is rejected with the pre-decode message; pins the constant at 170922)
- `crates/iscc-lib/tests/data-edge.json` + 4 vendored copies (dotnet, go, kotlin, swift): the
    vector, spliced in as text (the value is 127,993 `x`; the asserted canonical length is 128,001)
- `crates/iscc-ffi/tests/data_edge_vectors.h`: regenerated with
    `uv run --script scripts/gen_data_edge_vectors.py`

**Verification:**

- iscc-core 1.5.0 probe on both builds: `--python 3.10` (compiled wheel) and `--python 3.15` (pure
    Python) both raise
    `ValueError meta payload size (128001 bytes) exceeds META_TRIM_META (128000   bytes)`
- `packages/go`: `CGO_ENABLED=0 go test -count=1 ./...` ok;
    `GOARCH=386 CGO_ENABLED=0 go test   -count=1 ./...` ok; `go vet ./...` and
    `GOARCH=386 go vet ./...` clean
- The vector catches the old behaviour: in a /tmp copy of `packages/go` with HEAD's `code_meta.go`,
    `go test -run DataEdge` FAILS with
    `gen_meta_code_v0/test_0004_meta_payload_over_limit: expected   an error, got map[… iscc:ISCC:AAAWKLHFXNUBQAI7 …]`
- `grep -n MetaTrimMeta packages/go/code_meta.go`: line 72/75 (post-decode check) and line 143
    (`metaPreDecodeLimit`, which the pre-decode check uses)
- `cargo test -p iscc-lib`: all test binaries ok (315 + 44 + 28 + … passed, 0 failed)
- `uv run pytest tests/test_data_edge.py tests/test_vendored_fixtures.py   tests/test_gen_data_edge_vectors.py`:
    165 passed
- C FFI: built the way the CI step does (plus `-Wall`, no warnings); 148 passed, 0 failed, including
    `PASS: data_edge/gen_meta_code_v0/test_0004_meta_payload_over_limit`
- C++: cmake Debug + `SANITIZE_ADDRESS=ON` built with no warnings; 119 passed, 0 failed, same vector
    PASS
- `mise run check`: exit 0

**Next:** the `[audit]` issue "`@iscc/wasm` docs show a Node.js setup the published package cannot
run" (docs only; probe a packed `wasm-pack build --target web` release build first).

**Notes:**

- Not run locally: the Node, WASM, JNI, Ruby, .NET, Swift and Kotlin harnesses. They all go through
    the unchanged Rust core, which rejects the vector (the Rust and C/C++ harnesses prove it), and
    they need no harness edit. CI runs them
- The ~128 KB C string literal caused no compiler warning (gcc `-Wall`, C++ ASAN build)
- No Rust code changed, so the iai and CRAP baselines are not affected
