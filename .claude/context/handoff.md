> **HUMAN REVIEW RESOLVED (2026-11-18, Titusz → option 1, stay strict):** base32 decoding keeps
> rejecting `=` padding and non-zero spare bits on every surface (decisions.md 2026-11-18). The
> Python ISCC-SEQ differential test asserts that divergence explicitly; the spec criterion now says
> so. The interactive review found the pure-Go port is *not* strict — it accepts spare-bit aliases
> such as `ISCC:GAAAAAAAAB` — filed as "Go `decodeBase32` accepts spare-bit aliases that Rust
> rejects" in issues.md.

## 2026-11-18 — Review of: ISCC-SEQ codec in the Rust core (`encode_seq` / `decode_seq` / `read_unit`)

**Verdict:** PASS_WITH_NOTES

**Summary:** A clean, faithful port of iscc-core 1.5.0 `read_unit` / `encode_seq` / `decode_seq` on
private raw-`u32` header helpers. The typed `decode_header` / `encode_header` are now thin wrappers,
and their behaviour and existing tests are unchanged. `test_seq.rs` (44 tests) covers every
`test_seq.py` case plus the IEP-0020 vectors, and the conformance selftest runs both seq sections
under every feature set. Scope: 3 non-test files plus the generated `.crap-baseline.json`.

**Verification:**

- [x] `cargo test -p iscc-lib` passes (lib 296, test_seq 44, others ok), including
    `test_conformance_selftest_passes`. The CI feature matrix also passes: `--no-default-features`
    245/44, `--all-features` 296/44, `text-processing` 266/44
- [x] `cargo clippy -p iscc-lib --all-targets -- -D warnings`, workspace clippy, both CI feature
    clippy lines, and `cargo fmt -p iscc-lib --check` are clean
- [x] `test_seq.rs` asserts the exact outputs for `270000000000`, `301000000000300000000000` and
    `faaa-aaaaaaaa`, plus an `Err` with a message substring for each of the 7 Table-2 inputs
- [x] `iscc_decode` and `iscc_decompose` reject `ISCC:GAIAAAAAAA` and a SubType-8 unit
- [x] `mise run bench:iai:check` exits 0 with 16 benchmarks OK (`four_units` −4.6%). The
    `.iai-baseline.json` commit is still `ecdee58`
- [x] After `mise run coverage`, the CI CRAP line exits 0 (116 unchanged)
- [x] `mise run check` exit 0. The unpushed range has no suppressions, skips or weakened gates

**Issues found:**

- Codec-wide divergence: `decode_base32` is stricter than the reference. Pre-existing, not caused by
    this step, and filed as `normal` (see the escalation above)
- The CRAP baseline regen also re-scored untouched functions, because the toolchain changed. CI's
    last run used rustc 1.99.1 (`run_data_tests` 10.3), while rustc 1.100.0 gives 9.4. CI's
    `@stable` is now 1.100.0, with cargo-llvm-cov 0.9.1 and cargo-crap 0.2.2 — the same as local.
    The old baseline was red locally (3 regressions). **CI CRAP on this sha is unverified**
- Fixed in review (step 9): `conformance.rs` module doc line was 114 chars (rewrapped, same line
    count). `test_seq_mismatch_and_parse_failures_fail` never exercised the mismatch arm, because
    `ISCC:GAAAAAAAAAAA` failed earlier with "Trailing data", and it bundled all three bad cases in
    one call. It now runs one case per call with a valid mismatch unit, and all 6 arms are confirmed
- Deleted the `cid:run` fallback-model issue. Its premise ("identical to the primary") no longer
    holds: human commit `e6e5c70` deliberately set `--fallback-model claude-opus-5`, which differs
    from `claude-opus-5-5`, so removing the flag would undo a human choice
- `decisions.md` was rotated 457 → 367 lines using the runner's own `archive_overflow`, which moved
    the two oldest Unicode-freeze entries. `learnings.md` IDv1 notes were archived

**Codex review:** Two runs, one on HEAD (docs-only: clean) and one on `5d2840e`.

- (P2) Padded and trailing-bit base32 accepted by the reference → real, filed and escalated (above)
- (P2) `bzqaqyyh…` (multibase `b` + `cc01` + unit) is re-read as a different plain-base32 unit after
    uppercasing → **dismissed**: iscc-core 1.5.0 returns the identical bytes (`0e6008…`). This is
    the reference's documented uppercase-first behaviour
- (P3) Conformance failure test was too weak → fixed in review (above)
- (P3) The `iscc_decompose` doc's "only 64-bit units" alignment rule is imprecise. What matters is
    each unit's total byte length being a multiple of 5, so 224-bit units also align and a 64-bit
    unit with a 3-byte header does not → **dismissed**: the Rust doc is the iscc-core 1.5.0
    docstring verbatim, as next.md required

**Next:** Once Titusz has settled the base32 question, start the Python `encode_seq` / `decode_seq`
binding with the differential test against `iscc_core` 1.5.0 (the dev dependency). Base the
differential reject set on `test_seq.py`, and handle padded and trailing-bit spellings as the
decision says. If the decision is still pending, any non-Python surface (napi, wasm, ffi with an
`iscc.h` + `NativeMethods.g.cs` regen in the same step) can go first. Keep the Go `iscc_clean` issue
ahead of the Go seq slice, and the Tier-1 33 → 35 docs sweep last.

**Notes:**

- Confirm CI is green on the pushed sha, especially the CRAP job. If it goes red on the lowered
    entries (`run_*_tests`, `gen_meta_code_v0`), CI and local are measuring differently. Find out
    why before regenerating the baseline again, and never widen thresholds
- `read_unit` returns `Err("Malformed ISCC-HEADER at offset N")` for out-of-range offsets, including
    `usize::MAX`. Python slices leniently, so this matches the reference message without panicking
- Local tooling for the iai and CRAP gates (valgrind, iai-callgrind-runner 0.16.1, cargo-llvm-cov,
    cargo-crap 0.2.2) is installed in this container. Coverage + CRAP takes ~6 min, iai ~2 min
