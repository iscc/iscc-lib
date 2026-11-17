## 2026-11-17 — ISCC-SEQ codec in the Rust core (`encode_seq` / `decode_seq` / `read_unit`)

**Done:** Ported the ISCC-SEQ codec from iscc-core 1.5.0 (IEP-0020, #45) into `codec.rs`, adding
`read_unit`, `encode_seq`, `decode_seq` and `SEQ_MAINTYPES`. They sit on two private raw-integer
helpers (`decode_header_raw`, `encode_header_raw`); `decode_header` and `encode_header` are now thin
typed wrappers over those helpers. `encode_seq` and `decode_seq` are re-exported at the crate root,
the `encode_seq` / `decode_seq` conformance sections run inside `conformance_selftest`, and
`test_seq.py` is ported to `tests/test_seq.rs`.

**Files changed:**

- `crates/iscc-lib/src/codec.rs`: raw header helpers. The typed wrappers keep the same order (all
    four varnibbles, then the tail `Vec`, then `u8::try_from` / `TryFrom` / `validate_version`).
    `read_unit` builds its window from `data.get(offset..).unwrap_or(&[])`, so an out-of-range
    offset gives "Malformed ISCC-HEADER at offset N" as in the reference, with no panic and no
    `offset + 8` overflow. Checks run in the reference order (malformed → non-canonical → MainType
    on the raw `u32` → `ln > 7` → truncated). `encode_seq` uppercases before `iscc_clean`. Also
    added rustdoc on all four items and aligned the `iscc_decompose` doc with 1.5.0 (lenient reader;
    see `decode_seq`).
- `crates/iscc-lib/src/lib.rs`: `pub use codec::{decode_seq, encode_seq};`
- `crates/iscc-lib/src/conformance.rs`: `decode_bytes` (`bytes:<hex>`), `report_case`,
    `run_encode_seq_tests`, `run_decode_seq_tests`, wired into `conformance_selftest` with no
    feature gate. The module doc is updated. 3 unit tests: both sections have 4 cases and pass; a
    missing section fails; a mismatch, an error or a parse failure fails.
- `crates/iscc-lib/tests/test_seq.rs` (new, 44 tests): every `test_seq.py` case except the
    Python-only ones (`*_type_error`, `*bytes_like`, `memoryview`, `tuple_and_generator`). Plus:
    - 14 unknown SubType/Version headers pinned as **literal hex from iscc-core 1.5.0**, not generated
        by a local encoder, and 4 exact canonical strings
    - the IEP-0020 accepts and the 7 Table-2 rejects, each asserting its error message substring
    - round trip on all 4 `decode_seq` vectors
    - `read_unit` at `len`, `len+1`, `100` and `usize::MAX` → `Err`
    - guards that `iscc_decode` / `iscc_decompose` still reject `ISCC:GAIAAAAAAA` and a SubType-8 unit
- `.crap-baseline.json`: regenerated (`cargo crap ... --format json --output`, after coverage)

**Verification:**

- `cargo test -p iscc-lib`: all suites pass, including `test_seq` 44/44,
    `test_conformance_selftest_passes` and `test_seq_sections_run`
- `--no-default-features` and `--no-default-features --features text-processing`: every test result
    is `ok`. The CI clippy lines for both feature sets are clean
- `cargo clippy -p iscc-lib --all-targets -- -D warnings` clean; `cargo fmt -p iscc-lib --check`
    exit 0
- `mise run bench:iai:check`: exit 0, "OK: 16 benchmark(s) within 10% of baseline (Ir)", run with
    the header refactor in place. `git log -1 --format=%h -- .iai-baseline.json` prints `ecdee58`
    (unchanged)
- CRAP: `mise run coverage` exit 0. Against the old baseline, the CI gate exited 1 with 3
    regressions unrelated to SEQ: `soft_hash_image_v0` +0.1 (0.7.0 Image-Code change, baseline never
    refreshed) and `parse_meta_json` / `json_to_data_url` +0.0 (rounding). After the regen, the CI
    line `cargo crap --lcov lcov.info --baseline .crap-baseline.json --fail-regression --fail-above`
    exits 0 (116 unchanged)
- `mise run check`: exit 0 (see Notes)

**Next:** Binding slices for `encode_seq` / `decode_seq` (Python first, with the differential test
against `iscc_core`), then the other surfaces and their conformance harnesses. Do the Go
`iscc_clean` issue before the Go seq slice. The Tier-1 33 → 35 doc sweep comes last.

**Notes:**

- **Divergence found (pre-existing, not SEQ-specific):** `codec::decode_base32` (data-encoding
    `BASE32_NOPAD`) rejects non-zero trailing bits. So `encode_seq(&["GAAAAAAAAB"])` returns
    `Err("non-zero trailing bits at 9")` here, while `iscc_core.encode_seq(["GAAAAAAAAB"])` returns
    `300000000000` (both probed). The same applies to every base32 decode (`iscc_decode` etc.). The
    Python differential slice should either avoid such inputs or file this as an issue. I left
    `decode_base32` unchanged (out of scope).
- `cargo clippy -p iscc-lib --all-targets --no-default-features` fails on both bench files because
    they import gated `gen_meta/text_code_v0`. This predates this step; CI runs that feature set
    without `--all-targets`.
- Messages match the reference text (e.g. "Malformed ISCC-HEADER at offset 6: insufficient bits for
    varnibble"). Only the nested varnibble text differs.
- Local tooling installed this session: valgrind (needs `apt-get update` first),
    `iai-callgrind-runner` 0.16.1, `cargo-llvm-cov`, `cargo-crap` 0.2.2, all via `cargo install`.
- `mise run check` (LANG=C.UTF-8): the first two runs auto-fixed whitespace and mdformat in files
    written this session; the third run exits 0. A second `mise run coverage` + CI CRAP line also
    exits 0 (116 unchanged), so the regenerated baseline is deterministic.
