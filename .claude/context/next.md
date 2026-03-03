# Next Work Package

## Step: Define feature flags and gate meta-code + text-processing

## Goal

Add Cargo feature flags (`meta-code`, `text-processing`) to `iscc-lib` so embedded consumers can opt
out of heavy dependencies (serde_json_canonicalizer, unicode-normalization,
unicode-general-category). Default behavior is unchanged (all features on). This is the first step
toward resolving issue #16.

## Scope

- **Create**: (none)
- **Modify**:
    - `crates/iscc-lib/Cargo.toml` — define `[features]` section, make deps optional
    - `crates/iscc-lib/src/utils.rs` — gate unicode-dependent code behind `text-processing`
    - `crates/iscc-lib/src/lib.rs` — gate exports, gen functions, helpers, constants, conformance
        module, and tests behind features
- **Reference**:
    - `crates/iscc-lib/src/conformance.rs` — understand what it imports (do NOT modify; gate at module
        level from lib.rs)
    - `.claude/context/issues.md` — issue #16 requirements

## Not In Scope

- **Conformance selftest adaptation** — `conformance_selftest()` is gated behind `meta-code` for now
    (the entire module). Making it adapt to partial features (skip disabled code types while
    remaining available) is a follow-up step
- **CI workflow changes** — adding `--no-default-features` and per-feature CI jobs is a separate
    step
- **Binding crate changes** — binding crates always use default features; no changes needed
- **Gating `serde_json` itself** — it stays as a regular (non-optional) dependency because
    `conformance.rs` uses it for parsing `data.json` test vectors. Fully gating it requires
    restructuring conformance (future work)
- **Documentation updates** for the new features

## Implementation Notes

### Feature definitions in Cargo.toml

```toml
[features]
default = ["meta-code"]
text-processing = ["dep:unicode-normalization", "dep:unicode-general-category"]
meta-code = ["text-processing", "dep:serde_json_canonicalizer"]
```

Make the three dependencies optional:

```toml
unicode-normalization = { workspace = true, optional = true }
unicode-general-category = { workspace = true, optional = true }
serde_json_canonicalizer = { workspace = true, optional = true }
```

Keep `serde_json`, `blake3`, `data-encoding`, `hex`, `xxhash-rust`, `thiserror` as regular
(always-required) dependencies.

### Gates in utils.rs

Gate these items behind `#[cfg(feature = "text-processing")]`:

- `use unicode_general_category::...` and `use unicode_normalization::...` imports
- `NEWLINES` constant
- `is_c_category()` and `is_cmp_category()` helper functions
- `pub fn text_clean()` and `pub fn text_collapse()`
- All tests for the above functions (wrap the test functions, not the entire `mod tests`)

Keep ungated (always available): `text_remove_newlines`, `text_trim`, `multi_hash_blake3`, and their
tests.

### Gates in lib.rs

**`#[cfg(feature = "text-processing")]`** on:

- `pub use utils::{text_clean, text_collapse}` — split the existing single `pub use` line into two:
    one ungated for `text_remove_newlines, text_trim` and one gated for
    `text_clean,   text_collapse`
- `pub fn gen_text_code_v0(...)` and all its tests (`test_gen_text_code_v0_*`)

**`#[cfg(feature = "meta-code")]`** on:

- Constants: `META_TRIM_NAME`, `META_TRIM_DESCRIPTION`, `META_TRIM_META`
- Private helper functions: `interleave_digests`, `meta_name_simhash`, `soft_hash_meta_v0`,
    `soft_hash_meta_v0_with_bytes`, `decode_data_url`, `parse_meta_json`, `build_meta_data_url`
- Public functions: `pub fn json_to_data_url(...)`, `pub fn gen_meta_code_v0(...)`
- Module: `pub mod conformance` and `pub use conformance::conformance_selftest`
- All tests for the above items (`test_gen_meta_code_v0_*`, `test_json_to_data_url_*`,
    `test_soft_hash_meta_v0_*`)

**Pattern for gating test functions**: Use `#[cfg(feature = "...")]` on individual test functions
(not the whole `mod tests` block), since the test module contains tests for both gated and ungated
code.

### Important: `serde_json` in tests

Many test functions in lib.rs use `serde_json::Value` for parsing test data JSON (e.g.,
`test_gen_image_code_v0_conformance`). These are NOT gated — they work because `serde_json` stays as
a regular dependency. Only tests that CALL gated functions need `#[cfg]`.

## Verification

- `cargo test -p iscc-lib` passes (default features = all on, same 312 tests)
- `cargo test -p iscc-lib --no-default-features` compiles and passes (gated tests excluded)
- `cargo test -p iscc-lib --features text-processing` compiles and passes (text but no meta)
- `cargo clippy -p iscc-lib -- -D warnings` clean (default features)
- `cargo clippy -p iscc-lib --no-default-features -- -D warnings` clean
- `cargo check -p iscc-lib --no-default-features` succeeds (proves deps are properly optional)

## Done When

All six verification commands pass — the crate compiles and tests pass with default features, with
`--no-default-features`, and with `--features text-processing`, and clippy is clean for both default
and no-default configurations.
