# Next Work Package

## Step: Generalize video API to accept borrowed slices

## Goal

Change `soft_hash_video_v0` and `gen_video_code_v0` from `&[Vec<i32>]` to generic
`&[S] where S: AsRef<[i32]> + Ord`, eliminating per-frame heap allocations in the FFI crate while
remaining backward-compatible with all existing callers. This resolves the `[normal]` iscc-ffi video
frame allocation issue.

## Scope

- **Create**: (none)
- **Modify**:
    - `crates/iscc-lib/src/lib.rs` — generalize `soft_hash_video_v0` and `gen_video_code_v0`
        signatures
    - `crates/iscc-ffi/src/lib.rs` — replace `Vec<Vec<i32>>` construction with `Vec<&[i32]>` in both
        `iscc_gen_video_code_v0` and `iscc_soft_hash_video_v0`
- **Reference**:
    - `crates/iscc-lib/src/lib.rs` lines 525-563 (current video functions)
    - `crates/iscc-ffi/src/lib.rs` lines 390-418, 940-975 (FFI video wrappers with `.to_vec()`)
    - `crates/iscc-lib/tests/test_algorithm_primitives.rs` (existing video tests — should pass
        unchanged)

## Not In Scope

- Changing any other binding crate (iscc-py, iscc-napi, iscc-wasm, iscc-jni) — they pass
    `Vec<Vec<i32>>` which satisfies the generic bounds, so no changes needed
- Adding a benchmark for the FFI video path (no criterion bench for FFI; this is a pure allocation
    removal)
- Changing the Go bindings — they call through FFI/WASM and benefit automatically
- Fixing other `[low]` issues (dct power-of-two, wtahash bounds, etc.)
- Publishing or CI workflow changes

## Implementation Notes

**Core API change** (`crates/iscc-lib/src/lib.rs`):

Replace concrete `&[Vec<i32>]` with generic `S: AsRef<[i32]> + Ord`:

```rust
pub fn soft_hash_video_v0<S: AsRef<[i32]> + Ord>(
    frame_sigs: &[S], bits: u32
) -> IsccResult<Vec<u8>>
```

Same for `gen_video_code_v0`.

Inside `soft_hash_video_v0`, update the body to use `.as_ref()`:

- `frame_sigs[0].as_ref().len()` for column count
- `sig.as_ref().iter()` for column-wise sum iteration
- `BTreeSet<&S>` for deduplication (works because `S: Ord`)

This is **backward compatible**: `Vec<i32>` implements both `AsRef<[i32]>` and `Ord`, so all
existing callers passing `&[Vec<i32>]` compile without changes.

**FFI optimization** (`crates/iscc-ffi/src/lib.rs`):

In both `iscc_gen_video_code_v0` (line ~409) and `iscc_soft_hash_video_v0` (line ~959), change:

```rust
let frames: Vec<Vec<i32>> = sig_ptrs.iter().zip(lens.iter())
    .map(|(&ptr, &len)| unsafe { std::slice::from_raw_parts(ptr, len) }.to_vec())
    .collect();
```

to:

```rust
let frames: Vec<&[i32]> = sig_ptrs.iter().zip(lens.iter())
    .map(|(&ptr, &len)| unsafe { std::slice::from_raw_parts(ptr, len) })
    .collect();
```

This eliminates the `.to_vec()` per-frame allocation — slices borrow directly from the caller's
memory. `&[i32]` satisfies `AsRef<[i32]> + Ord`.

**Why `AsRef<[i32]> + Ord` and not just `&[&[i32]]`**: Changing to a concrete `&[&[i32]]` would
break all 5 binding crates (py, napi, wasm, jni, ffi) since `&[Vec<i32>]` doesn't coerce to
`&[&[i32]]`. The generic approach is backward-compatible — only the FFI crate (the actual
beneficiary) needs modification.

## Verification

- `cargo test -p iscc-lib` passes (all 261 tests — generics are transparent to existing callers)
- `cargo test -p iscc-ffi` passes (62 FFI tests including video wrappers)
- `cargo clippy --workspace --all-targets -- -D warnings` clean
- `grep -c '\.to_vec()' crates/iscc-ffi/src/lib.rs` returns `1` (down from 3 — only the unrelated
    `alg_cdc_chunks` `.to_vec()` on line 906 remains)

## Done When

All four verification commands pass, confirming the generic video API compiles cleanly across the
entire workspace and the FFI per-frame `.to_vec()` allocations are eliminated.
