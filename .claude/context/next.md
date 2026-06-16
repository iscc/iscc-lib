# Next Work Package

## Step: Narrow internal module visibility to `pub(crate)` (v1.0.0 API-surface lockdown)

## Goal

Stop the internal algorithm modules (`cdc`, `minhash`, `simhash`, `utils`, `conformance`) from
leaking their module *paths* (`iscc_lib::cdc::alg_cdc_chunks`, etc.) into the public API. Changing
them to `pub(crate) mod` curates the surface down to the intended Tier 1 crate-root re-exports
before v1.0.0 + `cargo-semver-checks` lock the API — this is the last allowed breaking change in the
pre-1.0 window (issue: "Narrow internal module visibility to `pub(crate)` before v1.0.0").

## Scope

- **Modify**:
    - `crates/iscc-lib/src/lib.rs` — change 5 module declarations from `pub mod` to `pub(crate) mod`:
        `cdc`, `conformance`, `minhash`, `simhash`, `utils`. Keep `pub mod codec`, `pub mod types`,
        `pub mod streaming` (these stay Tier 1 / Tier 2 public). `dct` and `wtahash` are already
        `pub(crate)`.
    - `crates/iscc-lib/tests/test_algorithm_primitives.rs` (test — excluded from file limit) — remove
        the now-uncompilable module-path tests `test_module_path_imports_simhash`,
        `test_module_path_imports_minhash`, `test_module_path_imports_cdc`.
    - `crates/iscc-lib/tests/test_text_utils.rs` (test — excluded) — remove `test_module_path_imports`
        and `test_module_path_imports_text_processing`.
    - `crates/iscc-lib/CLAUDE.md` (doc — excluded) — update the "Module Layout" table so the
        Visibility column for `utils.rs`, `cdc.rs`, `minhash.rs`, `simhash.rs`, `conformance.rs` reads
        `pub(crate)` instead of `pub`.
- **Reference**:
    - `.claude/context/issues.md` — "Narrow internal module visibility to `pub(crate)` before v1.0.0"
        entry (acceptance details).
    - `.claude/context/specs/rust-core.md` → "API Stability & Performance Invariants".
    - `notes/04-api-compatibility-safety.md` — Tiered API model (already shows modules as
        `pub(crate) mod`; confirm no change needed there).

## Not In Scope

- Do NOT add the `cargo-semver-checks` CI gate in this step — that is a separate issue and must come
    *after* this surface change so the gate locks the curated surface.
- Do NOT touch `pub mod codec`, `pub mod types`, or `pub mod streaming` — they remain public.
- Do NOT rename, move, or change the signatures of any function; only the module *path* visibility
    changes. The crate-root `pub use` re-exports (`alg_cdc_chunks`, `alg_minhash_256`,
    `alg_simhash`, `sliding_window`, `text_*`, `conformance_selftest`) stay exactly as-is.
- Do NOT touch any binding crate (verified: no binding imports via module path).
- Do NOT start the npm `optionalDependencies`, PyO3, SumHasher, or coverage/perf-gate work.

## Implementation Notes

- A `pub use private_mod::item;` re-export from a `pub(crate) mod` is valid Rust — the crate-root
    Tier 1 symbols remain public after narrowing. Only the module *paths* become crate-private.
- The module-path tests being removed are redundant: `test_flat_crate_root_imports`
    (test_algorithm_primitives.rs:289) already exercises all 5 primitives via the crate root, and
    `test_crate_root_imports` / `test_crate_root_imports_text_processing` (test_text_utils.rs:145,
    153\) already cover the text utilities via the crate root. No coverage is lost by deleting the
    `test_module_path_imports*` functions — do not rewrite them to crate-root paths (that would
    duplicate the existing crate-root tests); just delete them.
- After editing, no `iscc_lib::{cdc,minhash,simhash,utils,conformance}::` path may remain anywhere
    under `crates/iscc-lib/tests/`.
- Internal crate code uses direct `crate::`/`use` imports, not the leaked public paths, so
    `pub(crate)` does not break any in-crate caller (verified: no `crate::<mod>::` qualified paths
    in `src/`).

## Verification

- `cargo test -p iscc-lib` passes (all existing tests except the 5 deleted redundant module-path
    tests; conformance vectors still green).
- `cargo clippy -p iscc-lib -- -D warnings` clean.
- `cargo fmt -p iscc-lib --check` clean.
- `cargo build -p iscc-lib` succeeds.
- `grep -nE '^pub mod (cdc|minhash|simhash|utils|conformance);' crates/iscc-lib/src/lib.rs` returns
    nothing (all five narrowed).
- `grep -nE '^pub mod (codec|types|streaming);' crates/iscc-lib/src/lib.rs` still returns exactly 3
    lines (these stay public).
- `grep -rnE 'iscc_lib::(cdc|minhash|simhash|utils|conformance)::' crates/iscc-lib/tests` returns
    nothing.
- A consumer can still import every Tier 1 re-export from the crate root (covered by the retained
    `test_flat_crate_root_imports` / `test_crate_root_imports*` tests).

## Done When

`cargo test -p iscc-lib`, `cargo clippy -p iscc-lib -- -D warnings`, and
`cargo fmt -p iscc-lib --check` all pass with the five modules declared `pub(crate) mod`, no
module-path references remain in the test suite, and the `iscc-lib/CLAUDE.md` Module Layout table
reflects the new visibility.
