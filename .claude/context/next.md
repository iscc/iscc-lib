# Next Work Package

## Step: Implement gen_text_code_v0 with MinHash

## Goal

Implement `gen_text_code_v0` — the Text Content-Code generator — which uses MinHash (not SimHash) to
produce a similarity-preserving hash from plain text. This is the third gen function and unblocks
`gen_data_code_v0` which also uses MinHash.

## Scope

- **Create**: `crates/iscc-lib/src/minhash.rs` — MinHash module ported from `bio-codes/iscc-sum`
- **Modify**: `crates/iscc-lib/src/lib.rs` — add `pub(crate) mod minhash`, implement
    `soft_hash_text_v0` and `gen_text_code_v0`, replace stub test with conformance tests
- **Modify**: `Cargo.toml` (root) — add `xxhash-rust` to `[workspace.dependencies]`
- **Modify**: `crates/iscc-lib/Cargo.toml` — add `xxhash-rust` to `[dependencies]`
- **Reference**:
    - `bio-codes/iscc-sum` via deepwiki — `src/minhash.rs` (minhash, minhash_compress, minhash_256,
        MPA/MPB constants) and `Cargo.toml` (xxhash-rust dependency)
    - `iscc/iscc-core` via deepwiki — `code_content_text.py` (gen_text_code_v0, soft_hash_text_v0)
    - `crates/iscc-lib/src/simhash.rs` — reuse `sliding_window` function
    - `crates/iscc-lib/src/utils.rs` — reuse `text_collapse` function
    - `crates/iscc-lib/tests/data.json` — conformance vectors under `gen_text_code_v0`

## Implementation Notes

### CRITICAL: This is MinHash, NOT SimHash

The learnings file incorrectly states `gen_text_code_v0` uses SimHash. It actually uses **MinHash
with xxhash**. The pipeline is:

```
text_collapse(text) → sliding_window(collapsed, 13) → xxh32(ngram.as_bytes()) per ngram → alg_minhash_256(features)
```

### minhash.rs — Port from bio-codes/iscc-sum

Port the following from `bio-codes/iscc-sum/src/minhash.rs`:

1. **Constants** — `MPA` and `MPB`: arrays of 64 `u64` values (from `src/constants.rs` in iscc-sum).
    Include them directly in `minhash.rs` to keep file count down.

2. **`minhash(features: &[u32]) -> Vec<u64>`** — 64-dimensional MinHash using universal hash
    functions. For each (a, b) pair from MPA/MPB, compute:
    `min over all features f: ((a.wrapping_mul(f as u64).wrapping_add(b)) & MAXI64) % MPRIME) & MAXH`
    Constants: `MAXI64 = u64::MAX`, `MPRIME = (1 << 61) - 1`, `MAXH = (1 << 32) - 1`. Empty
    features → return `vec![MAXH; 64]`. **Do NOT use rayon** — keep the core crate
    dependency-light. Sequential iteration is fine.

3. **`minhash_compress(mhash: &[u64], lsb: u32) -> Vec<u8>`** — Extract `lsb` least-significant bits
    from each hash value, interleave them (iterate bitpos 0..lsb, then over all hash values), pack
    into bytes (MSB-first bit packing within each byte).

4. **`alg_minhash_256(features: &[u32]) -> Vec<u8>`** — Public function: calls `minhash` then
    `minhash_compress` with `lsb=4`. Returns 32 bytes (64 × 4 bits = 256 bits).

### gen_text_code_v0 in lib.rs

```
fn soft_hash_text_v0(text: &str) -> Vec<u8>:
    ngrams = sliding_window(text, 13)
    features = ngrams.iter().map(|ng| xxhash_rust::xxh32::xxh32(ng.as_bytes(), 0)).collect()
    alg_minhash_256(&features)

pub fn gen_text_code_v0(text: &str, bits: u32) -> IsccResult<String>:
    collapsed = text_collapse(text)
    characters = collapsed.chars().count()
    hash_digest = soft_hash_text_v0(&collapsed)
    component = encode_component(MainType::Content, SubType::Text, Version::V0, bits, &hash_digest)
    return Ok(format!("ISCC:{component}"))
```

- **SubType::Text** maps to value 0 in ST_CC enum — verify this exists in `codec.rs`
- `characters` is computed but not returned yet (no result struct). For now, compute it for the
    conformance test to verify, but only return the ISCC string. The conformance test should verify
    both the ISCC and the character count independently.
- Empty text is valid: `text_collapse("")` → `""`, `sliding_window("", 13)` → `[""]`, xxh32 of empty
    bytes → a specific u32, minhash_256 of that → a specific digest.

### xxhash-rust dependency

Add to root `Cargo.toml`:

```toml
xxhash-rust = { version = "0.8", features = ["xxh32"] }
```

Add to crate `Cargo.toml`:

```toml
xxhash-rust.workspace = true
```

### Conformance vectors (5 test cases)

| Name                           | Input          | Bits | Expected ISCC                                                | Chars |
| ------------------------------ | -------------- | ---- | ------------------------------------------------------------ | ----- |
| test_0000_empty_str            | ""             | 64   | ISCC:EAASL4F2WZY7KBXB                                        | 0     |
| test_0001_hello_world          | "Hello World"  | 64   | ISCC:EAASKDNZNYGUUF5A                                        | 10    |
| test_0002_hello_world_256_bits | "Hello World"  | 256  | ISCC:EADSKDNZNYGUUF5AMFEJLZ5P66CP5YKCOA3X7F36RWE4CIRCBTUWXYY | 10    |
| test_0003_i18n                 | unicode text   | 256  | ISCC:EADTJCW2DT555KK6...                                     | 42    |
| test_0004_more                 | long paragraph | 128  | ISCC:EABZHFKU6PNI7UVWYEEIQLOYHYLX6                           | 249   |

### Learnings correction

The learnings file states `gen_text_code_v0` uses
`text_collapse → sliding_window(3) → BLAKE3 → alg_simhash`. This is **wrong**. The correct pipeline
is `text_collapse → sliding_window(13) → xxh32 → alg_minhash_256`. The advance agent should note
this for the review agent to correct.

## Verification

- `cargo test -p iscc-lib` passes (all existing 76 tests + new text code tests)
- All 5 `gen_text_code_v0` conformance vectors produce matching ISCC codes
- Character counts match for all 5 test vectors
- `cargo clippy -p iscc-lib -- -D warnings` clean
- `cargo fmt -p iscc-lib --check` clean
- No `unsafe` code
- minhash module has unit tests for `alg_minhash_256` (at minimum: empty features, single feature,
    known output)

## Done When

The advance agent is done when all 5 `gen_text_code_v0` conformance vectors pass, the minhash module
has unit tests, and all quality gates (clippy, fmt, existing tests) remain green.
