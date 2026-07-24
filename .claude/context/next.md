# Next Work Package

## Step: Reject trailing bytes in Rust core `iscc_decode`

## Goal

Close the trailing-byte alias gap in the stability-committed Rust core so
`iscc_decode("ISCC:MAIGHFECJMOPMIABAA")` no longer decodes identically to canonical
`ISCC:MAIGHFECJMOPMIAB`. This resolves the open `normal` `[review]` issue "Rust core `iscc_decode`
silently accepts trailing bytes" and fixes the gap for all 11 bindings that delegate to the core —
the exact parallel of the Go fix landed in iter 120.

## Scope

- **Create**: (none)
- **Modify**: `crates/iscc-lib/src/lib.rs` (add the "too long" rejection branch in `iscc_decode`,
    update its docstring, add one focused test in the inline `#[cfg(test)] mod tests`)
- **Reference**:
    - `crates/iscc-lib/src/lib.rs` lines ~211–247 (`iscc_decode` body + docstring)
    - `crates/iscc-lib/src/codec.rs` `decode_length` (line ~355) — confirms canonical body length
        equals `nbytes` for every MainType, including `MainType::Iscc` composites (test at line ~848)
    - `packages/go/codec.go` `IsccDecode` — the just-landed two-branch reference pattern
    - `.claude/context/issues.md` — issue "Rust core `iscc_decode` silently accepts trailing bytes"

## Not In Scope

- Do NOT touch `codec::iscc_decompose` (`crates/iscc-lib/src/codec.rs:484`) — it has its own body
    loop that legitimately consumes trailing units; its behavior must stay identical.
- Do NOT modify any binding crate/package (py, napi, wasm, ffi, jni, rb, uniffi, dotnet, cpp, swift,
    kotlin) — the stricter validation propagates automatically through delegation.
- Do NOT change the `iscc_decode` signature or its return tuple shape — this is a robustness fix
    (rejecting previously-accepted malformed input), not an API change.
- Do NOT touch `encode_component`, `soft_hash_codes_v0`, or the "too short" branch of `iscc_decode`
    (its `"too short"` message substring is asserted elsewhere — keep it intact).
- Do NOT flip the Semver gate to enforcing or cut v1.0.0 (both held by Titusz).

## Implementation Notes

- The current guard at line 234 is `if tail.len() < nbytes { ... "too short" ... }`, followed by
    `tail[..nbytes].to_vec()` at line 245 which silently drops excess bytes. Add a second,
    independent branch after the "too short" one:
    `if tail.len() > nbytes { return Err(IsccError::InvalidInput(format!("decoded body too long: expected {nbytes} digest bytes, got {}", tail.len()))); }`
    Keep them as two distinct branches (mirrors the Go form) so any test asserting the
    `"too short"` substring stays green. After both guards, `tail.len() == nbytes`, so
    `tail[..nbytes]` is now a full copy.
- Update the `iscc_decode` doc comment: the summary (line ~214, "truncated to exactly the encoded
    bit-length") and the `# Errors` section (line ~223, currently only "shorter than the expected
    digest length") should state that the decoded body must equal exactly the expected length —
    inputs with trailing bytes are rejected.
- **Conformance-safe** for the same byte-alignment reason as the Go fix: canonical ISCC base32
    round-trips to exactly `nbytes` for byte-aligned headers (N bytes → ceil(8N/5) chars → N bytes),
    so `tail == digest` for every vendored vector. This holds for variable-length ISCC-IDv0 and for
    `MainType::Iscc` composites (verified: `decode_length` returns the full composite body length).
- Add one test in the inline test module (near the other `iscc_decode` tests, ~line 1995). Suggested
    shape: take a known-canonical code (e.g. `"ISCC:AAAZXZ6OU74YAZIM"` used at line 1951, or
    round-trip `encode_component(0,0,0,64,&[0xaa;8])`), append two base32 chars that decode to at
    least one extra byte (e.g. `"AA"`), assert `iscc_decode` returns `Err(InvalidInput)` whose
    message contains `"too long"`, and assert the canonical form still decodes successfully. Verify
    the appended-suffix input actually base32-decodes to `> nbytes` bytes before asserting (adjust
    the suffix if `"AA"` collapses to zero extra bytes).

## Verification

- `cargo test -p iscc-lib` passes (all existing tests + the new trailing-byte rejection test).
- `cargo test -p iscc-lib --all-features` passes (feature-gated `text-processing`/`meta-code`
    paths).
- `cargo clippy -p iscc-lib --all-features -- -D warnings` is clean.
- `cargo fmt -p iscc-lib --check` passes.
- The new test asserts both: `iscc_decode` of a canonical code with trailing base32 chars returns
    `Err(IsccError::InvalidInput)` with message containing `"too long"`, AND the canonical code (no
    suffix) still decodes to its expected digest.
- `test_iscc_decode_truncated_input`, the `test_iscc_decode_round_trip_*` tests, and
    `conformance_selftest`/`test_*_conformance_vectors` all stay green (composite `iscc_decompose`
    path and all vendored vectors unaffected).

## Done When

`cargo test -p iscc-lib --all-features`, `cargo clippy -p iscc-lib --all-features -- -D warnings`,
and `cargo fmt -p iscc-lib --check` all pass with the new trailing-byte rejection test in place, and
`iscc_decode` rejects inputs whose decoded body exceeds the expected digest length while every
canonical vector still round-trips.
