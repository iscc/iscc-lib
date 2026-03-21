## 2026-03-21 — Create UniFFI scaffolding crate with all 32 Tier 1 symbols

**Done:** Created `crates/iscc-uniffi/` with `Cargo.toml` and `src/lib.rs` exposing all 32 Tier 1
symbols from `iscc-lib` via UniFFI proc macros. Added `uniffi = "0.31"` to workspace dependencies
and `crates/iscc-uniffi` to workspace members. The crate uses proc macros only (no UDL files, no
`build.rs`), with `uniffi::setup_scaffolding!()` at the top.

**Files changed:**

- `crates/iscc-uniffi/Cargo.toml`: New crate manifest — cdylib + staticlib + lib, depends on
    iscc-lib (all features) + uniffi (workspace) + thiserror (workspace), `publish = false`
- `crates/iscc-uniffi/src/lib.rs`: UniFFI interface — `IsccUniError` error type, 11 result Records
    (all with `Debug`), 10 gen function wrappers, 4 text utilities, 4 algorithm primitives, 1 soft
    hash, 2 encoding utilities, 3 codec operations, 5 constant getter functions, 2 streaming Object
    types (DataHasher/InstanceHasher with Mutex\<Option<Inner>>), 1 diagnostic function, plus 21
    unit tests
- `Cargo.toml`: Added `crates/iscc-uniffi` to workspace members, added `uniffi = "0.31"` to
    workspace dependencies

**Verification:**

- `cargo build -p iscc-uniffi` — compiles successfully
- `cargo test -p iscc-uniffi` — 21 tests pass (constants, gen functions, text utilities, algorithm
    primitives, streaming, codec, conformance selftest)
- `cargo clippy -p iscc-uniffi -- -D warnings` — clean
- `cargo clippy --workspace --all-targets -- -D warnings` — clean
- `cargo build -p iscc-lib` — still passes (no regressions)
- `cargo test -p iscc-lib` — 317 tests pass (261+31+24+1)
- `cargo test --workspace` — all 422 workspace tests pass
- Pre-commit hooks (formatting, TOML, YAML, JSON) — all pass
- Pre-push Ruby/Python hooks fail — pre-existing environment issue (bundle not on PATH, Python
    module not built), not related to this change

**Next:** Create the Swift package (`packages/swift/`) with `Package.swift`, generated Swift
bindings via `uniffi-bindgen generate`, idiomatic Swift wrapper, and conformance tests. This is the
natural next step now that the UniFFI scaffolding compiles.

**Notes:**

- Used `uniffi = "0.31"` (latest) as specified in next.md. The spec tree mentions `uniffi.toml` and
    `build.rs` but next.md explicitly says both are NOT in scope — proc macro approach works without
    them.
- `usize` to `u64` conversion applied to `TextCodeResult.characters` and `text_trim`'s `nbytes`
    parameter. All other type mappings follow next.md (String for paths, Vec<u8> for bytes, etc.).
- Result records all derive `Debug` (needed for `unwrap_err()` in tests) in addition to
    `uniffi::Record`.
- DataHasher/InstanceHasher have `Default` impls to satisfy clippy's `new_without_default` lint.
- The 32 `#[uniffi::export]` annotations include 2 on `impl` blocks (DataHasher, InstanceHasher) +
    30 on free functions, matching the 32 Tier 1 symbol count.
