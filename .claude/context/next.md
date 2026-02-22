# Next Work Package

## Step: Implement gen_meta_code_v0 for name/description inputs

## Goal

Implement `gen_meta_code_v0` with SimHash-based similarity hashing and text normalization, covering
name-only and name+description input modes. This introduces the SimHash algorithm and text utilities
that are reused by 6+ other gen functions, and verifies against 13 of 16 conformance vectors (the 3
requiring JSON/Data-URL meta objects are deferred).

## Scope

- **Create**: `crates/iscc-lib/src/simhash.rs`, `crates/iscc-lib/src/utils.rs`
- **Modify**: `crates/iscc-lib/src/lib.rs`, `Cargo.toml` (root — add workspace deps),
    `crates/iscc-lib/Cargo.toml` (add crate deps)
- **Reference**: `crates/iscc-lib/src/codec.rs` (for `encode_component`, `MainType::Meta`,
    `SubType::None`), `crates/iscc-lib/tests/data.json` (conformance vectors), iscc-core
    `code_meta.py` / `simhash.py` / `utils.py` via deepwiki

## Implementation Notes

### New dependencies

Add to root `Cargo.toml` `[workspace.dependencies]`:

```toml
unicode-normalization = "0.1"
unicode-general-category = "1"
```

Add to `crates/iscc-lib/Cargo.toml` `[dependencies]`:

```toml
unicode-normalization.workspace = true
unicode-general-category.workspace = true
```

### Module: `simhash.rs` — `pub(crate)`

**`alg_simhash(hash_digests: &[impl AsRef<[u8]>]) -> Vec<u8>`**

Port of iscc-core `simhash.py::alg_simhash`. Algorithm:

1. Determine `n_bytes` from first digest length, `n_bits = n_bytes * 8`
2. Create `vector: Vec<u32>` of length `n_bits`, all zeros
3. For each digest, for each bit position `i`: if bit `i` is set, increment `vector[i]`
4. Threshold: `min_features = hash_digests.len()` (integer, NOT divided by 2 yet)
5. For each bit `i`: if `vector[i] * 2 >= min_features`, set bit `i` in output (this matches
    Python's `vector[i] >= len(hash_digests) / 2` with float division)
6. Bit numbering: MSB first — bit 0 is the highest bit of byte 0. Python uses
    `shash |= 1 << (n_bits - 1 - i)` then `.to_bytes(n_bytes, "big")`
7. Return `Vec<u8>` of `n_bytes` length
8. If input is empty, return zero bytes (32 zero bytes for BLAKE3 digests)

**`sliding_window(seq: &str, width: usize) -> Vec<String>`**

Port of iscc-core `utils.py::sliding_window`. Works on Unicode characters (not bytes):

1. Assert `width >= 2`
2. Collect chars, compute `len = chars.len()`
3. Generate windows: `for i in 0..max(len - width + 1, 1)` yield `chars[i..i+width]`
4. If input is shorter than `width`, return one element containing the full input
5. Return `Vec<String>`

Also provide a bytes variant for future use:

**`sliding_window_bytes(data: &[u8], width: usize) -> Vec<Vec<u8>>`**

Same logic but operates on byte slices.

### Module: `utils.rs` — `pub(crate)`

**`text_clean(text: &str) -> String`**

Port of iscc-core `code_meta.py::text_clean`:

1. NFKC normalize the text (use `unicode_normalization::UnicodeNormalization::nfkc()`)
2. Remove control characters (Unicode General Category starting with "C") EXCEPT these newline
    characters: `\u{000A}`, `\u{000B}`, `\u{000C}`, `\u{000D}`, `\u{0085}`, `\u{2028}`, `\u{2029}`
3. Process lines: split on newlines, allow at most one consecutive empty/whitespace-only line
4. Join with `\n`, strip leading/trailing whitespace

Use `unicode_general_category::get_general_category()` and check if category name starts with 'C'
(i.e., is in the `GeneralCategory::*Control*|*Format*|*Surrogate*|*PrivateUse*|*Unassigned*` set).
The simplest check: match on categories `Cc`, `Cf`, `Cn`, `Co`, `Cs`.

**`text_remove_newlines(text: &str) -> String`**

Port of iscc-core `code_meta.py::text_remove_newlines`:

```rust
text.split_whitespace().collect::<Vec<_>>().join(" ")
```

**`text_trim(text: &str, nbytes: usize) -> String`**

Port of iscc-core `code_meta.py::text_trim`:

Truncate UTF-8 bytes to `nbytes`, then decode back to str ignoring incomplete chars at the boundary,
then trim whitespace. In Rust: find the largest valid UTF-8 prefix of `&text.as_bytes()[..nbytes]`
using `std::str::from_utf8()` or scanning backward to find a char boundary.

**`text_collapse(text: &str) -> String`**

Port of iscc-core `code_meta.py::text_collapse`:

1. NFD normalize
2. Lowercase (`.to_lowercase()` after NFD — operates on the decomposed form)
3. Filter in one pass: keep chars that are NOT whitespace AND whose Unicode General Category does
    NOT start with "C", "M", or "P"
4. NFKC normalize the filtered result

**`multi_hash_blake3(data: &[u8]) -> String`**

Port of iscc-core `utils.py::multi_hash_blake3`:

```rust
let digest = blake3::hash(data);
let mut result = Vec::with_capacity(34);
result.push(0x1e); // BLAKE3 multicodec
result.push(0x20); // 32 bytes length
result.extend_from_slice(digest.as_bytes());
hex::encode(result)
```

Note: this uses `hex` which is currently a dev-dependency. Either promote `hex` to a regular
dependency or inline a hex-encoding function (prefer promoting `hex` since it's tiny).

### Modifications to `lib.rs`

**Add module declarations:**

```rust
pub(crate) mod simhash;
pub(crate) mod utils;
```

**Implement `soft_hash_meta_v0` as a private helper:**

```rust
fn soft_hash_meta_v0(name: &str, extra: Option<&str>) -> Vec<u8>
```

Algorithm:

1. `collapsed_name = utils::text_collapse(name)`
2. `name_ngrams = simhash::sliding_window(&collapsed_name, 3)` (meta_ngram_size_text = 3)
3. `name_hashes: Vec<[u8; 32]>` — BLAKE3 hash each n-gram's UTF-8 bytes
4. `simhash_digest = simhash::alg_simhash(&name_hashes)`
5. If `extra` is `None` or empty string, return `simhash_digest`
6. Otherwise (extra is a non-empty string):
    - `collapsed_extra = utils::text_collapse(extra)`
    - `extra_ngrams = simhash::sliding_window(&collapsed_extra, 3)` (same width for text)
    - `extra_hashes` — BLAKE3 hash each n-gram
    - `extra_simhash = simhash::alg_simhash(&extra_hashes)`
    - Interleave first 16 bytes of each simhash in 4-byte chunks:
        ```
        result[0..4]   = name_simhash[0..4]
        result[4..8]   = extra_simhash[0..4]
        result[8..12]  = name_simhash[4..8]
        result[12..16] = extra_simhash[4..8]
        result[16..20] = name_simhash[8..12]
        result[20..24] = extra_simhash[8..12]
        result[24..28] = name_simhash[12..16]
        result[28..32] = extra_simhash[12..16]
        ```
    - Return 32-byte interleaved digest

**Replace gen_meta_code_v0 stub:**

```rust
pub fn gen_meta_code_v0(
    name: &str,
    description: Option<&str>,
    meta: Option<&str>,
    bits: u32,
) -> IsccResult<String>
```

Algorithm (name/description only — return `Err(NotImplemented)` if `meta` is `Some`):

1. Normalize name: `text_clean → text_remove_newlines → text_trim(128)`
2. If name is empty after normalization, return `Err(InvalidInput("..."))"`
3. Normalize description: `text_clean → text_trim(4096)` (description is "" if None)
4. Compute payload for metahash:
    - `payload = format!("{} {}", name, description).trim().as_bytes()` (space-join, then trim)
    - Handle edge case: if description is empty, payload is just `name.as_bytes()`
    - Actually match Python: `" ".join((name, description)).strip().encode("utf-8")`
5. `meta_code_digest = soft_hash_meta_v0(name, if description.is_empty() { None } else { Some(description) })`
6. `metahash = utils::multi_hash_blake3(payload_bytes)`
7. `meta_code = codec::encode_component(MainType::Meta, SubType::None, Version::V0, bits, &meta_code_digest)?`
8. Return `Ok(format!("ISCC:{meta_code}"))`

**Important edge case**: The `description` parameter is `Option<&str>` in the Rust API but Python
treats `None` and `""` the same way (both become `""`). After normalization, use empty string check
to decide whether to pass extra to `soft_hash_meta_v0`.

### Conformance test

Add a conformance test similar to `test_gen_instance_code_v0_conformance`:

1. Parse `gen_meta_code_v0` section from `data.json`
2. Input format: `[name: str, description: str, meta: str|dict|null, bits: int]`
3. For test cases where `meta` (inputs[2]) is `null`: run the test
4. For test cases where `meta` is non-null: skip with a message (deferred to next step)
5. Assert `iscc` output field matches
6. Also assert `metahash` output field matches (verifies the BLAKE3 payload logic)

Replace the `test_gen_meta_code_v0_stub` with the conformance test.

### Known test vectors for sanity checking

- `gen_meta_code_v0("Die Unendliche Geschichte", None, None, 64)` → `"ISCC:AAAZXZ6OU74YAZIM"`
- `gen_meta_code_v0("Die Unendliche Geschichte", Some("Von Michael Ende"), None, 64)` →
    `"ISCC:AAAZXZ6OU4E45RB5"`

## Verification

- `cargo test -p iscc-lib` passes (all existing 43 tests + new conformance tests)
- All 13 `gen_meta_code_v0` conformance vectors with `meta=null` pass (both `iscc` and `metahash`)
- 3 test vectors with `meta` objects are explicitly skipped (not failing)
- `gen_meta_code_v0("Die Unendliche Geschichte", None, None, 64)` returns
    `Ok("ISCC:AAAZXZ6OU74YAZIM")`
- `gen_meta_code_v0` with `meta=Some(...)` returns `Err(NotImplemented)`
- `cargo clippy -p iscc-lib -- -D warnings` clean
- `cargo fmt -p iscc-lib --check` clean
- No `unsafe` code added

## Done When

All verification criteria pass — `gen_meta_code_v0` produces correct ISCC Meta-Codes and metahashes
for all 13 name/description conformance vectors, and the SimHash + text utility modules are in place
for reuse by subsequent gen functions.
