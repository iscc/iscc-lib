# Next Work Package

## Step: C FFI accepts `(NULL, 0)` everywhere and checks inner pointers, via one `slice_or_empty` helper

## Goal

Pick up the `normal` `[audit]` issue "C FFI treats `(NULL, 0)` two opposite ways and never checks
inner pointers": every pointer+length input of `crates/iscc-ffi` goes through one helper (len 0 →
empty slice, NULL with len > 0 → error), which also removes the undefined behaviour on NULL inner
pointers and the 7 .NET stack-sentinel workarounds that the split forced.

## Alternatives Considered

- **Chosen:** FFI `(NULL, 0)` unification. It fixes UB (a NULL `frame_sigs[i]`/`digests[i]` reaches
    `from_raw_parts`), is the smallest of the three release blockers, and the handoff names it next.
- **Rejected:** out-of-range integer issue (5-6 files over napi, WASM, JNI, Py, Rb). A larger
    multi-surface change with no UB; it goes next. The published-docs issue (~11 files) comes last,
    because the FFI change edits `packages/dotnet/CLAUDE.md` first.

## Scope

- **Modify**: `crates/iscc-ffi/src/lib.rs` (helper + every slice call site),
    `packages/dotnet/Iscc.Lib/IsccLib.cs` (drop the 7 `sentinel` branches),
    `packages/dotnet/CLAUDE.md` (the "Empty span sentinel" section and the pitfall at line ~166)
- **Regenerate**: `crates/iscc-ffi/include/iscc.h` and `packages/dotnet/Iscc.Lib/NativeMethods.g.cs`
    (export doc comments move, e.g. the `iscc_encode_seq` errors list "`units` is NULL")
- **Tests**: `crates/iscc-ffi/src/tests.rs`, `crates/iscc-ffi/tests/test_iscc.c`
- **Reference**: `crates/iscc-ffi/CLAUDE.md`, `packages/dotnet/Iscc.Lib.Tests/ArrayInputTests.cs`,
    `packages/cpp/include/iscc/iscc.hpp` (`detail::safe_data`)

## Not In Scope

- Removing C++ `detail::safe_data` and `tests/test_cpp_safe_data.py`. It is one uniform helper on
    the C++ surface and stays correct once the FFI accepts `(NULL, 0)`. Whether to drop it is a
    separate cleanup that the reviewer may file.
- .NET `PinnedRows` (jagged arrays): it does the null check and padded pinning (decision 2026-11-20)
    and stays as it is.
- `*const c_char` string parameters (`ptr_to_str` / `ptr_to_optional_str`): a NULL string is not a
    slice and stays an error.
- `data-edge.json` vectors. NULL pointers cannot be written in JSON, so the collecting rule does not
    apply.
- The integer-range and published-docs issues.

## Implementation Notes

- Helper (private, in `lib.rs`):
    `unsafe fn slice_or_empty<'a, T>(ptr: *const T, len: usize, name: &str) -> Option<&'a [T]>`.
    `len == 0` → `Some(&[])` whatever `ptr` is. NULL with `len > 0` →
    `set_last_error("<name> must not be NULL")` and `None`. Otherwise `from_raw_parts` with a
    `// SAFETY:` comment. Call sites use
    `let Some(x) = (unsafe { … }) else { return <err value>; };` and keep their own error return
    (`ptr::null_mut()`, `null_byte_buffer()`, `false`, …).
- Cover **every** input `std::slice::from_raw_parts` in `lib.rs`: image `pixels`, audio `cv`, data,
    instance, `encode_base64`, `encode_component` `digest`, `decode_seq`, `alg_minhash_256`
    `features`, `alg_cdc_chunks`, both hasher `update`s, and the **outer** arrays (`codes` in mixed
    and `gen_iscc_code_v0`, `units` in `encode_seq`, `frame_sigs`/`frame_lens` in video and
    `soft_hash_video_v0`, `digests`/`digest_lens` in `alg_simhash`). With count 0 these now reach
    the core, which raises its own error (empty list, too few codes). That is the point: the FFI no
    longer hides the core's message.
- Inner pointers: in the video, `soft_hash_video_v0` and `alg_simhash` loops, call the helper per
    row with an indexed name (`frame_sigs[{i}]`, `digests[{i}]`) and stop at the first error. Use a
    plain `for` loop or `collect::<Option<Vec<_>>>()`, whichever reads cleaner. Inner pointers of
    string arrays already go through `ptr_to_str`.
- Rewrite the `# Safety` / errors doc comments of each touched export to say that a pointer may be
    NULL when its length is 0. Then regenerate the header with the cbindgen command in
    `crates/iscc-ffi/CLAUDE.md`. `cargo build -p iscc-ffi` rewrites `NativeMethods.g.cs`; commit
    both in this step (learnings: csbindgen rewrites on every build).
- Flip the Rust tests that pin a len-0 NULL rejection: `test_gen_data_code_v0_null` and
    `test_encode_seq_null_units` (now: the core's empty-list error, not "must not be NULL"), plus
    any other test the change breaks. Add tests: `(NULL, 0)` accepted on each former rejecter
    (image, audio, data, instance, `encode_base64`); NULL + len > 0 rejected with the parameter name
    on a representative set; a NULL inner row with len > 0 rejected for video, `soft_hash_video_v0`
    and `alg_simhash`, and a NULL inner row with len 0 accepted as an empty row. One bad case per
    call (learnings). Mirror the `(NULL, 0)` and inner-NULL cases in `test_iscc.c`.
- .NET: each `if (span.IsEmpty) { … sentinel … }` branch goes; the `fixed` path handles empty spans
    (a NULL pointer with length 0). `ArrayInputTests.cs` already asserts the core's error and no
    "must not be NULL", so it must stay green unchanged.
- Not a core or hot-path change: the CRAP and iai gates do not apply (`iscc-ffi` is outside both).

## Verification

- `cargo test -p iscc-ffi` passes, with the new `(NULL, 0)`, NULL + len > 0 and inner-NULL tests;
    `cargo clippy -p iscc-ffi -- -D warnings` clean
- `grep -n 'slice::from_raw_parts(' crates/iscc-ffi/src/lib.rs` prints only lines inside the body of
    `slice_or_empty`
- The C test compiles and exits 0 (commands in `crates/iscc-ffi/CLAUDE.md` → Build Commands)
- Regeneration is a no-op: after the cbindgen command and `cargo build -p iscc-ffi`,
    `git status --porcelain crates/iscc-ffi/include packages/dotnet/Iscc.Lib/NativeMethods.g.cs` is
    empty, once the regenerated files are committed
- `dotnet test packages/dotnet/Iscc.Lib.Tests/ -e LD_LIBRARY_PATH=$(pwd)/target/debug` passes, and
    `grep -n sentinel packages/dotnet/Iscc.Lib/IsccLib.cs packages/dotnet/CLAUDE.md` prints nothing
- The C++ suite passes against the rebuilt FFI (`packages/cpp/CLAUDE.md` build commands, cmake via
    `uv run --with cmake cmake …`), and `mise run check` exits 0

## Done When

The advance agent is done when every pointer+length input of the C FFI goes through `slice_or_empty`
and all six verification criteria pass.
