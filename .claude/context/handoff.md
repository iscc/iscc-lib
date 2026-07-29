## 2026-07-29 — Review of: Mint `gen_iscc_id_v1` on the uniffi surface (Swift + Kotlin)

**Verdict:** PASS

**Summary:** The shared `iscc-uniffi` crate now exports `gen_iscc_id_v1` (+ `IsccIdResult` record),
advancing **both** the Swift and Kotlin surfaces from 32→33 Tier 1 symbols in one core edit (9th and
10th of 11 IDv1 fan-out surfaces, #43). The core fn takes exact-width **unsigned** types
(`u64`/`u16`/`u8`) straight through, so core does the ordered ts→hub→realm validation and the
binding needs no guard. Clean, faithful to next.md, no scope creep, no gate weakening.

**Verification:**

- [x] `cargo build/test/clippy -D warnings/fmt --check -p iscc-uniffi` — all clean; 22 unit tests
    pass incl. new `test_gen_iscc_id_v1` (golden `ISCC:MAIGHFECJMOPMIAB` + round-trip bit-math).
- [x] Swift suite passes — accepted by transitivity: bindings verified byte-identical to fresh
    regeneration, test logic identical to the passing Rust golden test (swift.org toolchain not
    persisted locally; advance ran 13 tests/0 failures).
- [x] Kotlin suite passes — advance ran `./gradlew test` BUILD SUCCESSFUL; **independently confirmed
    by the Codex background run** which executed the Kotlin suite green.
- [x] `grep -c genIsccIdV1` = 1 in both swift `iscc_uniffi.swift` and kotlin `iscc_uniffi.kt`.
- [x] Re-running both bindgen commands leaves tracked bindings unchanged modulo trailing/EOF
    whitespace — re-verified here: only EOF-newline deltas on all three generated files, content
    byte-identical (incl. UniFFI checksum 32674).
- [x] `crates/iscc-uniffi/src/lib.rs:3` docstring reads "all 33 Tier 1 symbols".
- [x] `mise run check` — all prek hygiene + parity hooks Passed.
- [x] (probe) Scope + circumvention: 1 code file + 3 regenerated (declared fan-out) + 2 tests;
    honest fan-out (one core export, mechanically regenerated per surface, no per-surface decision).
    No API break, no hot path. `@{upstream}..HEAD` circumvention scan clean.

**Issues found:**

- (none) — uniffi's exact-width unsigned args (Swift UInt64/UInt16/UInt8, Kotlin ULong/UShort/UByte)
    mean callers cannot pass out-of-width values, so it is genuinely exempt from the open Ruby i64
    marshalling gap (issues.md). No new issue warranted.

**Codex review:** Clean — no findings. Confirms correct fixed-width delegation, regenerated bindings
match the interface, and Rust + Kotlin suites pass with the new golden/round-trip coverage.

**Next:** #43 fan-out has **two minting surfaces left**: **dotnet** (C#) and **cpp** — both
fixed-width FFI over `iscc_ffi`, so exempt from the wide-input marshalling gap; the C# P/Invoke decl
already exists. After both land, do the deferred repo-wide Tier-1 32→33 doc/count sweep (issues.md,
`gen_iscc_id_v1` API-doc entries). #43 stays open (v0.6.0 blocker).

**Notes:** Swift bindgen dumps `iscc_uniffiFFI.h` + a `.modulemap` into `--out-dir`; the tree keeps
the header in `Sources/iscc_uniffiFFI/` with a hand-simplified `module.modulemap` — advance moved
the `.h` and discarded the generated modulemap correctly (recorded in advance memory). The open Ruby
i64 issue is unaffected by this surface. dotnet/cpp are the cleaner remaining picks.
