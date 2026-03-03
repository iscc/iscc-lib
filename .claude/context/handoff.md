## 2026-03-03 — Define feature flags and gate meta-code + text-processing

**Done:** Added Cargo feature flags (`meta-code`, `text-processing`) to `iscc-lib` so embedded
consumers can opt out of heavy dependencies (serde_json_canonicalizer, unicode-normalization,
unicode-general-category). Default behavior unchanged — all features on. Dependencies made optional,
all code and tests properly gated behind `#[cfg(feature = "...")]` attributes.

**Files changed:**

- `crates/iscc-lib/Cargo.toml`: Added `[features]` section with `default = ["meta-code"]`,
    `text-processing` (unicode deps), `meta-code` (implies text-processing + JCS). Made 3 deps
    optional.
- `crates/iscc-lib/src/utils.rs`: Gated `text_clean`, `text_collapse`, `NEWLINES`, `is_c_category`,
    `is_cmp_category`, and their imports behind `text-processing`. Gated individual test functions.
- `crates/iscc-lib/src/lib.rs`: Gated meta-code items (constants, helpers, `gen_meta_code_v0`,
    `json_to_data_url`, `conformance` module) behind `meta-code`. Gated `gen_text_code_v0` and
    `soft_hash_text_v0` behind `text-processing`. Split `pub use utils::` into gated/ungated lines.
    Gated individual test functions.
- `crates/iscc-lib/src/simhash.rs`: Gated `sliding_window_strs` behind `text-processing` and
    `sliding_window_bytes` behind `meta-code` (both `pub(crate)` with no ungated callers — dead code
    without gating). Gated their test functions.
- `crates/iscc-lib/tests/test_text_utils.rs`: Gated integration tests for `text_clean`/
    `text_collapse` behind `text-processing`. Split
    `test_crate_root_imports`/`test_module_path_imports` into ungated + gated variants.

**Verification:**

- `cargo test -p iscc-lib` — 314 tests pass (312 original + 2 new from split import tests)
- `cargo test -p iscc-lib --no-default-features` — 249 tests pass (gated tests excluded)
- `cargo test -p iscc-lib --no-default-features --features text-processing` — 283 tests pass
- `cargo clippy -p iscc-lib -- -D warnings` — clean
- `cargo clippy -p iscc-lib --no-default-features -- -D warnings` — clean
- `cargo check -p iscc-lib --no-default-features` — succeeds
- `mise run check` — all 14 pre-commit hooks pass
- `uv run prek run --stage pre-push --all-files` — all 15 pre-push hooks pass

**Next:** CI workflow changes — add `--no-default-features` and per-feature CI jobs to validate
feature flag combinations in CI. Also, adapting `conformance_selftest()` to work with partial
features (skip disabled code types instead of being entirely gated) is a follow-up step.

**Notes:** Had to modify `simhash.rs` (4th file, beyond the 3-file scope in next.md) because
`sliding_window_strs` and `sliding_window_bytes` are `pub(crate)` functions whose only callers are
gated code. Without gating them, `cargo clippy --no-default-features -- -D warnings` fails with
dead-code errors. The changes are minimal (2 `#[cfg]` attributes + gating their test functions). The
doctest for `json_to_data_url` is automatically excluded when the feature is off (Rust handles this
via the `#[cfg]` on the function).
