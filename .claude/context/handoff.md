## 2026-07-29 — Mint `gen_iscc_id_v1` on the uniffi surface (Swift + Kotlin)

**Done:** Added the `IsccIdResult` record + `gen_iscc_id_v1(u64,u16,u8)` export to the shared
`iscc-uniffi` crate (types match core exactly — no binding guard/narrowing, so core does the ordered
ts→hub→realm validation). Regenerated the checked-in Swift and Kotlin bindings, taking both surfaces
from 32→33 accepted Tier 1 symbols (9th and 10th of 11 IDv1 fan-out surfaces, #43).

**Files changed:**

- `crates/iscc-uniffi/src/lib.rs`: header docstring 32→33; new `IsccIdResult` Record; new
    `gen_iscc_id_v1` export; in-crate golden+round-trip unit test.
- `packages/swift/Sources/IsccLib/iscc_uniffi.swift` (regenerated — added `FfiConverterUInt16`,
    `IsccIdResult`, `genIsccIdV1`).
- `packages/swift/Sources/iscc_uniffiFFI/iscc_uniffiFFI.h` (regenerated — new FFI decl + checksum).
- `packages/kotlin/src/main/kotlin/uniffi/iscc_uniffi/iscc_uniffi.kt` (regenerated).
- `packages/swift/Tests/IsccLibTests/ConformanceTests.swift`,
    `packages/kotlin/src/test/kotlin/uniffi/iscc_uniffi/ConformanceTest.kt`: golden
    (`ISCC:MAIGHFECJMOPMIAB`) + realm-1/hub-4095 round-trips via generic `iscc_decode` bit-math.

**Verification:**

- `cargo build/test/clippy -D warnings/fmt --check -p iscc-uniffi` — all clean; 22 unit tests pass
    (incl. new `test_gen_iscc_id_v1`).
- Swift: `swift test` (swift.org 6.1.2 Debian12 toolchain) — 13 tests, 0 failures;
    `ConformanceTests` now 10 (was 9), `testGenIsccIdV1` passes.
- Kotlin: `./gradlew test` BUILD SUCCESSFUL; `testGenIsccIdV1()` present in test-results XML, no
    failure. (Only the two known-benign uniffi "Expression is unused" warnings.)
- Regeneration no-op: re-ran both bindgen commands; `diff` modulo trailing-ws
    (`sed 's/[[:space:]]*$//'`) is empty on all three generated files.
- `mise run check` — all prek hygiene + parity hooks Passed (took 3 passes to reach the ws/EOF fixed
    point on the generated files, as expected for third-party bindgen output; nothing hand-edited).
- Grep gates: `genIsccIdV1` count = 1 in both swift/kotlin bindings; docstring reads "all 33 Tier 1
    symbols".

**Next:** #43 fan-out — remaining minting surfaces are **dotnet** (C#) and **cpp** (both fixed-width
FFI over `iscc_ffi`, so exempt from the wide-input marshalling gap). Then the deferred repo-wide
Tier-1 32→33 doc/count sweep (issues.md) once all surfaces land.

**Notes:**

- Swift bindgen dumps `iscc_uniffiFFI.h` + `iscc_uniffiFFI.modulemap` into `--out-dir`
    (`Sources/IsccLib/`), but the tree keeps the header in `Sources/iscc_uniffiFFI/` with a
    hand-simplified `module.modulemap`. I moved the `.h` and discarded the generated modulemap
    (recorded in advance memory).
- Scope: 1 code file (`iscc-uniffi/src/lib.rs`) + 3 regenerated (generated, not counted) + 2 tests
    (not counted). No API break; no hot path touched. Only the own-file docstring 32→33 was changed
    per Not-In-Scope; repo-wide count sweep untouched.
