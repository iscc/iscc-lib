## 2026-06-16 — Add core `streaming::SumHasher` struct and drive `gen_sum_code_v0` from it

**Done:** Added a reusable single-pass `streaming::SumHasher` to the `iscc-lib` core crate
(`new() → update(&[u8]) → finalize(bits, wide, add_units)`). It composes an inner `DataHasher` and
`InstanceHasher`, feeding the same slice to both per `update`. Refactored `gen_sum_code_v0` to drive
the new hasher, removing the duplicated dual-hasher read loop. This is the "core first" prerequisite
of issue #37.

**Files changed:**

- `crates/iscc-lib/src/streaming.rs`: added `pub struct SumHasher` + `Default` impl; extended module
    docstring and `use` imports (`SumCodeResult`, `gen_iscc_code_v0`); added 8 unit tests (empty,
    small data, multi-update streaming invariance, units toggle, wide vs narrow, datahash/filesize
    parity, default, and a cross-check that `SumHasher` equals file-based `gen_sum_code_v0` across
    `(bits, wide, add_units)` combinations using a `tempfile::NamedTempFile`).
- `crates/iscc-lib/src/lib.rs`: `gen_sum_code_v0` now opens the file, reads in `IO_READ_SIZE` chunks
    into a single `streaming::SumHasher`, and delegates composition to
    `hasher.finalize(bits, wide, add_units)`. Error mapping for open/read unchanged.

**Verification:**

- `cargo test -p iscc-lib`: 269 passed (incl. 8 new `SumHasher` tests + the existing 9-test
    `gen_sum_code_v0` suite, all green).
- `cargo clippy -p iscc-lib -- -D warnings`: clean.
- `cargo fmt -p iscc-lib --check`: clean.
- `--no-default-features` and `--all-features`: tests/clippy clean — `SumHasher` is
    feature-independent.
- Full `mise run check` (pre-commit) and `prek --hook-stage pre-push` (clippy, cargo test, ty, ruff,
    pytest, ruby): all green.

**Next:** The remainder of issue #37 — expose `SumHasher` in the PyO3 wrapper (`crates/iscc-py`) and
the wasm-bindgen wrapper (`crates/iscc-wasm`). That step should also promote `SumHasher` to a
crate-root Tier 1 `pub use` (currently reachable only via `iscc_lib::streaming::SumHasher`) and bump
the documented Tier 1 count ("2 streaming types" → 3) in `crates/iscc-lib/CLAUDE.md`,
`docs/rust-api.md`, and any README symbol tables. Issue #39 (GIL release via `py.allow_threads`) is
a separate follow-up.

**Notes:**

- Per next.md "Not In Scope", I intentionally did NOT re-export `SumHasher` at the crate root and
    did NOT change the documented 32 crate-root Tier 1 symbol count / "2 streaming types" wording.
    `SumHasher` is reachable for the bindings step via the already-`pub mod streaming`.
- `gen_sum_code_v0` is now a thin file-I/O wrapper; the dual-hasher composition logic lives solely
    in `SumHasher::finalize` (single source of truth, confirmed by
    `test_sum_hasher_matches_gen_sum_code_v0`).
- No public API change: `gen_sum_code_v0` signature/return type untouched; no Tier 1/Tier 2 symbol
    altered. Hot path unchanged (same hashers, same single-pass read loop) — no benchmark regression
    expected. </content> </invoke>
