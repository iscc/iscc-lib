## 2026-03-21 — Review of: Create UniFFI scaffolding crate with all 32 Tier 1 symbols

**Verdict:** PASS

**Summary:** The advance agent created `crates/iscc-uniffi/` exposing all 32 Tier 1 symbols via
UniFFI proc macros, with proper error mapping, result records, streaming Object types, and 21 unit
tests. The implementation is clean, well-documented, and stays strictly within scope. All
verification criteria pass.

**Verification:**

- [x] `cargo build -p iscc-uniffi` compiles successfully
- [x] `cargo test -p iscc-uniffi` passes — 21 tests (constants, gen functions, text utilities,
    algorithm primitives, streaming, codec, conformance selftest)
- [x] `cargo clippy -p iscc-uniffi -- -D warnings` is clean
- [x] `cargo build -p iscc-lib` still passes (no regressions)
- [x] `cargo test -p iscc-lib` still passes — 317 tests (261+31+24+1)
- [x] `cargo clippy --workspace --all-targets -- -D warnings` is clean
- [x] `mise run check` passes (14/15 hooks — Ruby `standardrb-fix` fails due to pre-existing
    `bundle` not on PATH, unrelated to this change)

**Issues found:**

- (none)

**Codex review:** Codex found no actionable issues — confirmed the change is limited to internal
scaffolding and does not affect existing runtime code paths.

**Next:** Create the Swift package (`packages/swift/`) with `Package.swift`, generated Swift
bindings via `uniffi-bindgen generate`, idiomatic Swift wrapper, and XCTest conformance tests. This
is the natural next step now that the UniFFI scaffolding crate compiles and all 32 Tier 1 symbols
are verified.

**Notes:**

- UniFFI 0.31 adds ~30 transitive dependencies to Cargo.lock (askama, cargo_metadata, goblin,
    uniffi_bindgen, etc.) — expected for a code generation framework
- The `iscc-uniffi` crate uses `publish = false` — it won't be published to crates.io
- Streaming types use `Mutex<Option<Inner>>` (thread-safe) vs Ruby's `RefCell<Option<Inner>>`
    (single-threaded) — correct choice for UniFFI which requires `Send + Sync`
- 32 `#[uniffi::export]` annotations: 30 on free functions + 2 on `impl` blocks (DataHasher,
    InstanceHasher) matching the 32 Tier 1 symbol count
- The swift-bindings spec mentions `uniffi.toml` and `build.rs` but next.md correctly excluded them
    — proc macro approach works without either. They may be needed later for binding generation
    customization
- `iscc-uniffi` workspace exclusion in CI `rust` job may be needed (like `iscc-rb`) if it pulls in
    dependencies that aren't available in the CI Rust job. Watch for this when adding CI
