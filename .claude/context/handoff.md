## 2026-02-24 — Harden `iscc_decompose` against truncated input

**Done:** Added bounds checks to all five unchecked slice operations in `iscc_decompose` so that
malformed or truncated base32 input returns `IsccError::InvalidInput` instead of panicking. Three
guards cover the three code paths: standard unit (body < nbytes), wide mode ISCC-CODE (body < 32),
and non-wide ISCC-CODE (body < main_types.len() * 8 + 16). The non-wide path uses a single upfront
check that covers both dynamic units and static Data+Instance slicing.

**Files changed:**

- `crates/iscc-lib/src/codec.rs`: Added 3 bounds checks with descriptive error messages in
    `iscc_decompose` (standard unit path, wide mode path, non-wide ISCC-CODE path). Added 6 new test
    functions and a `make_truncated_iscc` test helper that builds valid headers with insufficient
    body bytes.

**Verification:** `cargo test -p iscc-lib` passes — 193 total (187 existing + 6 new), 0 failures.
`cargo clippy -p iscc-lib -- -D warnings` clean. `mise run check` passes all pre-commit hooks. All 6
new tests assert `Err(IsccError::InvalidInput(_))` — none trigger panics.

**Next:** The `iscc_decompose` issue in issues.md can be resolved. The next highest-impact remaining
issues are `soft_hash_codes_v0` accepting too-short Content-Codes, `gen_meta_code_v0` empty Data-URL
handling, `alg_simhash` panic on mismatched digests, and `sliding_window` panic on width < 2 — all
normal-priority robustness fixes.

**Notes:** The Python reference doesn't crash on truncated input because Python slicing silently
truncates — but it produces silently wrong output. The Rust implementation is strictly better: it
rejects invalid input early with a clear error message. No architectural decisions were needed; this
was a straightforward defensive coding fix.
