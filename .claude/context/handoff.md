## 2026-11-21 — C FFI accepts `(NULL, 0)` everywhere and checks inner pointers, via one `slice_or_empty` helper

**Done:** Every pointer+length input of `crates/iscc-ffi` now goes through the private
`slice_or_empty` helper. Length 0 gives an empty slice whatever the pointer is. NULL with a non-zero
length sets `"<name> must not be NULL"`. A second private helper, `rows_or_empty`, handles the
parallel row arrays (video, `soft_hash_video_v0`, `alg_simhash`): it runs the outer arrays and each
row through `slice_or_empty` and names a bad row `frame_sigs[i]` / `digests[i]`. That removes the UB
from a NULL inner row. The 7 .NET stack-sentinel branches are gone.

**Files changed:**

- `crates/iscc-ffi/src/lib.rs`: `slice_or_empty` + `rows_or_empty` helpers; 20 call sites (image,
    audio, video, mixed, data, instance, iscc_code, encode_base64, encode_component, encode_seq,
    decode_seq, simhash, minhash, cdc, soft_hash_video, both hasher updates). The `alg_simhash`
    `num_digests == 0` shortcut is dropped (the helper path reaches the core's empty-list error).
    `# Safety` docs on each touched export say a pointer may be NULL when its length is 0;
    `iscc_encode_seq` errors now read "`units` is NULL with `num_units > 0`"
- `crates/iscc-ffi/include/iscc.h`, `packages/dotnet/Iscc.Lib/NativeMethods.g.cs`: regenerated (doc
    comments only)
- `crates/iscc-ffi/src/tests.rs`: flipped 6 len-0 NULL tests (image, audio, data, instance,
    encode_base64, encode_seq) and renamed video/mixed/iscc_code/soft_hash ones to `_null_empty`.
    Added NULL + len > 0 tests with the parameter name (pixels, cv, data, codes, units, frame_sigs,
    frame_lens, digests, digest_lens, features, digest, both hasher updates), inner NULL row + len >
    0 (video, soft_hash, simhash) and inner NULL row + len 0 (same three). Helpers
    `assert_str_matches_core` / `assert_buf_matches_core` check the FFI result against the core call
    on the same empty input: same output on `Ok`, the exact core message (never "must not be NULL")
    on `Err`
- `crates/iscc-ffi/tests/test_iscc.c`: block 35 (13 `(NULL, 0)` / NULL-row-len-0 cases; each
    compares the result to the same call with a non-NULL pointer and checks the error has no "must
    not be NULL") and block 36 (6 NULL + len > 0 cases, one bad case per call, message checked)
- `packages/dotnet/Iscc.Lib/IsccLib.cs`: removed the 7 `IsEmpty` sentinel branches
- `packages/dotnet/CLAUDE.md`: "Empty span sentinel" section → "Empty spans"; rewrote the pitfall

**Verification:**

- `cargo test -p iscc-ffi`: 120 passed. Before the test edits, the new lib.rs failed exactly the 5
    old len-0 rejection tests (image did not fail because the core also rejects empty pixels), so
    the flipped tests track the behaviour change
- `cargo clippy -p iscc-ffi --all-targets -- -D warnings`: clean
- `grep -n 'slice::from_raw_parts(' crates/iscc-ffi/src/lib.rs` → only line 267, inside
    `slice_or_empty`
- C test (`cc -Wall -Wextra`, no warnings): exit 0, 142 passed, 0 failed
- Regeneration no-op: reran cbindgen + `cargo build -p iscc-ffi`, then `cmp` against the earlier
    output: both files identical
- `dotnet test packages/dotnet/Iscc.Lib.Tests/ -e LD_LIBRARY_PATH=$(pwd)/target/debug`: 153 passed
    (`ArrayInputTests.cs` unchanged). `grep -n sentinel` on IsccLib.cs + dotnet CLAUDE.md: no output
- C++ (cmake via `uv run --with cmake`, Debug + ASAN, build in /tmp): 112 passed, 0 failed
- `mise run check`: exit 0. `prek run --hook-stage pre-push --files <7 changed files>`: all passed
    (clippy, cargo test, ty, ruff S/C901, pytest)

**Next:** The out-of-range integer issue (napi, WASM, JNI, Py, Rb), then the published-docs issue,
which can now cite the FFI's `(NULL, 0)` behaviour.

**Notes:**

- `PinnedRows` (IsccLib.cs ~590) was left alone as next.md said. Its doc comment still says the
    `max(Count, 1)` padding exists so "the core's own error surfaces instead of the FFI's 'must not
    be NULL'". After this step that is redundant but true. The reviewer may file dropping the
    padding together with C++ `detail::safe_data` as one cleanup
- `crates/iscc-ffi/CLAUDE.md` was not in scope. Its "NULL-check all incoming pointers" safety rule
    still holds (NULL + len > 0 is checked) but does not mention `slice_or_empty`
- Not a core or hot-path change: no bench or CRAP run (`iscc-ffi` is outside both gates)
