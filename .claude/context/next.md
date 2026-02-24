# Next Work Package

## Step: Fix `gen_meta_code_v0` empty Data-URL payload routing

## Goal

Fix correctness divergence from iscc-core when `gen_meta_code_v0` receives a Data-URL whose decoded
payload is empty bytes (`b""`). Currently the Rust code maps empty payloads to `None`, incorrectly
routing into the name/description text path instead of the meta bytes path.

## Scope

- **Create**: none
- **Modify**: `crates/iscc-lib/src/lib.rs` (two changes + new tests)
- **Reference**: `reference/iscc-core/iscc_core/code_meta.py` (lines 62-83 and 142-157)

## Not In Scope

- Changing the public API signature of `gen_meta_code_v0` — this is a bug fix to internal routing
- Fixing the other normal-priority issues (`alg_simhash` panics, `sliding_window` panics) — those
    are separate steps
- Adding conformance test vectors for this edge case — there are none in `data.json`; unit tests
    suffice
- Refactoring `soft_hash_meta_v0` and `soft_hash_meta_v0_with_bytes` into a single function — the
    dual-function pattern works and isn't the problem

## Implementation Notes

There are **two** code changes needed, both in `crates/iscc-lib/src/lib.rs`:

### Change 1: `gen_meta_code_v0` — stop discarding empty payloads

In lines 183-184, the `if decoded.is_empty() { None }` branch incorrectly maps empty decoded
Data-URL payloads to `None`. Remove this branch so `Some(vec![])` flows through to the meta bytes
path.

Python reference (`code_meta.py:62`): `if meta:` is truthy for any non-empty string, including a
Data-URL with empty base64 payload. The string `"data:application/json;base64,"` is truthy, so
Python enters the meta branch and processes `payload = b""`.

**Before:**

```rust
Some(meta_str) if meta_str.starts_with("data:") => {
    let decoded = decode_data_url(meta_str)?;
    if decoded.is_empty() {
        None
    } else {
        Some(decoded)
    }
}
```

**After:**

```rust
Some(meta_str) if meta_str.starts_with("data:") => {
    Some(decode_data_url(meta_str)?)
}
```

### Change 2: `soft_hash_meta_v0_with_bytes` — add empty bytes guard

The text-based `soft_hash_meta_v0` (line 78) correctly handles `None | Some("") => name_simhash` (no
interleaving). But `soft_hash_meta_v0_with_bytes` (line 98) always interleaves, even for empty
bytes.

Python reference (`code_meta.py:142`): `if extra in {None, "", b""}:` returns name-only simhash
without interleaving. This applies to the bytes path too — `b""` gets the same treatment as `None`.

**Before:**

```rust
fn soft_hash_meta_v0_with_bytes(name: &str, extra: &[u8]) -> Vec<u8> {
    let name_simhash = meta_name_simhash(name);

    let byte_ngrams = simhash::sliding_window_bytes(extra, 4);
    ...
    interleave_digests(&name_simhash, &byte_simhash)
}
```

**After:**

```rust
fn soft_hash_meta_v0_with_bytes(name: &str, extra: &[u8]) -> Vec<u8> {
    let name_simhash = meta_name_simhash(name);

    if extra.is_empty() {
        return name_simhash;
    }

    let byte_ngrams = simhash::sliding_window_bytes(extra, 4);
    ...
    interleave_digests(&name_simhash, &byte_simhash)
}
```

### Tests to add

Add at least two test functions:

1. **`test_gen_meta_code_empty_data_url_enters_meta_branch`** — call `gen_meta_code_v0` with
    `name = "Test"`, `meta = Some("data:application/json;base64,")` (empty payload). Assert:

    - Result is `Ok`
    - `result.meta` is `Some(...)` containing the original Data-URL string (not `None`)
    - `result.metahash` equals `multi_hash_blake3(&[])` (BLAKE3 of empty bytes)

2. **`test_soft_hash_meta_v0_with_bytes_empty_equals_name_only`** — call both
    `soft_hash_meta_v0("test", None)` and `soft_hash_meta_v0_with_bytes("test", &[])`. Assert they
    return identical digests (both should be name-only simhash without interleaving).

## Verification

- `cargo test -p iscc-lib` passes (all 247+ existing tests + new tests, 0 failures)
- `cargo clippy -p iscc-lib -- -D warnings` clean
- `cargo test -p iscc-lib -- test_gen_meta_code_empty_data_url` passes (new test)
- `cargo test -p iscc-lib -- test_soft_hash_meta_v0_with_bytes_empty` passes (new test)

## Done When

All verification criteria pass, confirming that `gen_meta_code_v0` with an empty Data-URL payload
enters the meta bytes path and produces output matching the Python reference behavior.
