# Next Work Package

## Step: .NET — one private helper pins every jagged-array input; empty inputs reach the core

## Goal

Resolve the `normal` issue ".NET string-array pinning is copied three times; the gen copies pass
NULL for `[]`". One private helper in `IsccLib.cs` pins the inner arrays for **all six**
jagged-array methods, so an empty or null input fails the same way everywhere and `[]` surfaces the
core's error, not the FFI's "must not be NULL".

## Alternatives Considered

- **Chosen:** .NET pinning helper — a real behaviour defect on a published surface (wrong error for
    `[]` on 4 methods, `NullReferenceException` on null input), locally verifiable with
    `dotnet test`, one source file. The cheaper doc-only issues fix no behaviour.
- **Rejected:** Dependabot `soupsieve`/`setuptools` refresh. Probed: soupsieve 3.0.2 and 4.x both
    need `requires-python >=3.11.5`, while `uv.lock` resolves for `>=3.10` (abi3-py310). So
    `uv lock --upgrade-package soupsieve` cannot reach 4.0.0 without a floor or dependency-graph
    decision (fork marker, dropping `mdformat-web` on 3.10). That needs Titusz; it is not a scoping
    call.
- **Rejected:** CI-red fix for Kotlin. The `kotlin-gradle-plugins-bom:2.4.10` POM now resolves
    (portal → 200 after redirect), so it was an outage. The review push re-runs CI.

## Scope

- **Modify**: `packages/dotnet/Iscc.Lib/IsccLib.cs` (the helper + the 6 call sites),
    `packages/dotnet/CLAUDE.md` ("Array parameter marshalling", "Empty span sentinel" bullets)
- **Create**: `packages/dotnet/Iscc.Lib.Tests/ArrayInputTests.cs`
- **Reference**: `packages/dotnet/Iscc.Lib.Tests/SeqTests.cs` (assertion style),
    `crates/iscc-ffi/src/lib.rs` (`iscc_gen_video_code_v0` L418, `iscc_gen_mixed_code_v0` L466,
    `iscc_alg_simhash` L1200, `iscc_soft_hash_video_v0` L1362 — the NULL guards)

## Not In Scope

- No `data-edge.json` vector: these inputs already fail on every surface, and edge harnesses assert
    the rejection only, never the message (`specs/edge-vectors.md` L24-26). Only the .NET message is
    wrong today
- The C++ empty-vector issue (`iscc.hpp`, `low`, untriaged) — another surface; skip it
- `ReadOnlySpan` single-array sites (`GenDataCodeV0`, `AlgMinhash256`, …) already use the stack
    sentinel — leave them alone
- No public signature change; no `NativeMethods.g.cs` or `iscc-ffi` edit

## Implementation Notes

The six sites: `GenVideoCodeV0`, `SoftHashVideoV0` (`int[][]` + lens), `AlgSimhash` (`byte[][]` +
lens), `GenMixedCodeV0`, `GenIsccCodeV0`, `EncodeSeq` (`string[]` → `ToNativeUtf8` → `byte[][]`, no
lens). Today each has its own `GCHandle` loop and `finally`; only `EncodeSeq` guards nulls and has
an empty sentinel.

Contract for every site after the change (design the helper shape yourself, e.g. a disposable
generic `where T : unmanaged` pin scope, or a static method that fills pointer/len arrays and
returns handles to free in one `finally`):

- **Null array or null element → `ArgumentNullException`** on all six (decided here, matching
    `EncodeSeq`; the parameter name in the message as `EncodeSeq` does). Today the gen/video/simhash
    sites throw `NullReferenceException`
- **Zero count → non-NULL pointers.** Pass a stack sentinel for the pointer array **and**, where the
    FFI takes one, the lens array: `iscc_gen_video_code_v0` / `iscc_soft_hash_video_v0` NULL-check
    `frame_sigs` and `frame_lens` separately. `iscc_alg_simhash` short-circuits on count 0 before
    its NULL checks, so `AlgSimhash([])` must keep returning 32 bytes
- `EncodeSeq`'s current special-cased empty branch goes away in favour of the shared path
- Every handle is freed in `finally`, also when the helper throws halfway through pinning (a null
    element at index 2 must not leak handles 0-1)

Expected core messages (read from `crates/iscc-lib/src/lib.rs`): `GenMixedCodeV0([])` → "at least 2
Content-Codes required for mixing"; `GenIsccCodeV0([])` → "at least 2 ISCC unit codes required";
`GenVideoCodeV0([])` and `SoftHashVideoV0([])` → "frame_sigs must not be empty" (gen_video delegates
to soft_hash, L759); `EncodeSeq([])` → "at least one ISCC-UNIT" (existing test).

Tests in `ArrayInputTests.cs`, one `[Fact]` per case, no classes of shared state: for each of the 4
empty-input methods assert `IsccException` whose message contains the core text **and** does not
contain "must not be NULL"; `ArgumentNullException` for a null array and a null element on each of
the 6 methods. `AlgSimhash_EmptyInput_Returns32Bytes` (SmokeTests) and SeqTests must stay green.

No Rust source changes, so CRAP/iai/semver are untouched.

## Verification

- `cargo build -p iscc-ffi` then
    `dotnet test packages/dotnet/Iscc.Lib.Tests/ -e LD_LIBRARY_PATH=$(pwd)/target/debug` exits 0,
    with the new `ArrayInputTests` tests in the run
- `git status --porcelain packages/dotnet/Iscc.Lib/NativeMethods.g.cs crates/iscc-ffi/include/iscc.h`
    prints nothing after that build
- Reviewer-read: all six methods route through the one helper; no `GCHandle.Alloc` call remains in
    their bodies outside it
- `dotnet build packages/dotnet/Iscc.Lib.Tests/Iscc.Lib.Tests.csproj` prints no warning line that
    names `IsccLib.cs`
- `mise run check` exits 0

## Done When

All six jagged-array methods share one pinning helper, empty input reaches the core's error, null
input throws `ArgumentNullException`, and every verification check passes.
