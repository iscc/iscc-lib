# Next Work Package

## Step: Go `GenMetaCodeV0` enforces the 128,000-byte meta limit, pinned by a shared vector

## Goal

Fix the `normal` `[audit]` issue "Go `GenMetaCodeV0` never enforces the 128,000-byte meta payload
limit": the pure-Go port must reject an oversized meta payload the way the Rust core and iscc-core
do. Per the collecting rule, the divergence is pinned by one `data-edge.json` vector that all 12
harnesses run.

## Alternatives Considered

- **Chosen:** Go meta limit. A correctness divergence on a Tier 1 function (Go returns a code where
    every other surface and the reference raise), one source file, and it closes a "Go port behaves
    differently" gap with a shared vector
- **Rejected:** `@iscc/wasm` Node docs (handoff "Next"). They are docs only and need a packed-build
    probe first. A wrong result from a conformance-tested port matters more; the docs come next

## Scope

- **Modify**: `packages/go/code_meta.go`
- **Modify (test data / generated, outside the cap)**: `crates/iscc-lib/tests/data-edge.json` and
    its 4 vendored copies (`packages/{dotnet/Iscc.Lib.Tests/testdata,go/testdata}/data-edge.json`,
    `packages/kotlin/src/test/resources/data-edge.json`,
    `packages/swift/Tests/IsccLibTests/data-edge.json`); the regenerated
    `crates/iscc-ffi/tests/data_edge_vectors.h`
- **Tests**: `packages/go/code_meta_test.go`
- **Reference**: `crates/iscc-lib/src/lib.rs` (`gen_meta_code_v0`, pre-decode + post-decode checks
    near :363-385), `reference/iscc-core/iscc_core/code_meta.py:62-76`,
    `scripts/gen_data_edge_vectors.py`, `.claude/context/specs/edge-vectors.md`

## Not In Scope

- Any Rust core or binding change: all Rust-backed surfaces already reject. If a binding harness
    fails on the new vector, that is a finding to report, not a reason to edit the harness to skip
    it (only `except` in the file may deviate, and none should be needed)
- A passing boundary vector at exactly 128,000 bytes in `data-edge.json`. Its `outputs.meta` would
    add another ~170 KB data URL to all 5 copies. The 128,000-byte pass stays a Go unit test
- The `docs/howto/dotnet.md:430` wording of `MetaTrimMeta`, and the duplicate-parser `[audit]` issue
    (`code_meta_test.go` `metaArg` copy): separate issues
- Changing `MetaTrimMeta` docs in `packages/go/README.md` / `docs/howto/go.md`: they already state
    the limit correctly; after the fix they are true

## Implementation Notes

- Mirror the core exactly, in this order, inside `GenMetaCodeV0` when `meta != nil`:
    1. **Pre-decode bound** on the raw meta string (both forms, as Rust does):
        `MetaTrimMeta*4/3 + 256` (= 170,922). Reject when `len(metaStr)` exceeds it, before
        `decodeDataURL`/`parseMetaJSON`
    2. **Post-decode bound**: reject when `len(metaPayload) > MetaTrimMeta`, for both the Data-URL
        payload and the canonical JSON bytes
- Error text follows the Go style (`iscc: ` prefix) with the core's wording, e.g.
    `iscc: decoded meta payload exceeds size limit (%d > %d bytes)` and
    `iscc: meta string exceeds size limit (%d > %d bytes)`
- Go tests (table style is fine): for the JSON form and the Data-URL form each, a payload of exactly
    128,000 bytes succeeds and 128,001 bytes errors. Build the JSON form as `{"a":"xxx…"}`: the
    canonical length is `len(value) + 8`. Add one pre-decode case: a JSON meta string longer than
    170,922 bytes whose canonical form is tiny (mostly whitespace) is rejected. The Rust core
    rejects that too, so Go matches it
- **Shared vector**: add `gen_meta_code_v0` → `test_0004_meta_payload_over_limit` with inputs
    `["Hello", "", {"a": "<127,993 x>"}, 64]` (canonical JSON = 128,001 bytes; check the length with
    a quick Python `json.dumps(obj, separators=(",", ":"), ensure_ascii=False)` count), an `error`
    key, a `note` naming the Go port, and **no** `diverges` (iscc-core raises `ValueError` on the
    dict input). Before committing, probe iscc-core on both builds
    (`uv run --no-project --python 3.10 --with iscc-core==1.5.0` and the same with `3.15`) and
    confirm both raise. Then copy the file to the 4 vendored paths and regenerate the header with
    `uv run --script scripts/gen_data_edge_vectors.py`
- The header gets one ~128 KB C string literal. No C/C++ build in CI uses `-pedantic`/`-Werror`, so
    gcc/clang accept it. If a local C or C++ build warns or fails on it, stop and report it in the
    handoff. Do not split the vector or change the generator in this step
- No hot-path Rust code changes, so neither the iai baseline nor CRAP is affected

## Verification

- In `packages/go`: `CGO_ENABLED=0 go test ./...` passes (the data-edge suite runs the new vector),
    `GOARCH=386 go test ./...` passes, and `go vet ./...` is clean
- `grep -n 'MetaTrimMeta' packages/go/code_meta.go` shows the constant used in both the pre-decode
    and the post-decode check
- `cargo test -p iscc-lib` passes (the Rust data-edge harness rejects the new vector)
- `uv run pytest tests/test_data_edge.py tests/test_vendored_fixtures.py tests/test_gen_data_edge_vectors.py`
    passes. That covers iscc-core agreeing with no `diverges`, 5 identical copies, and a fresh
    header
- The C FFI and C++ test programs build and pass with the regenerated header (as the `c-ffi` and
    `cpp` CI jobs run them)
- `mise run check` exits 0

## Done When

An oversized meta payload is an error in Go as it is in the core, Go tests pin the 128,000/128,001
boundary for both meta forms, and every harness that runs `data-edge.json` rejects the new vector.
