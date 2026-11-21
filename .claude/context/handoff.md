## 2026-11-21 — Review of: C FFI accepts `(NULL, 0)` everywhere and checks inner pointers, via one `slice_or_empty` helper

**Verdict:** PASS_WITH_NOTES

**Summary:** Every pointer+length input of `crates/iscc-ffi` now goes through `slice_or_empty` (row
arrays through a second private helper, `rows_or_empty`). Length 0 is an empty input, NULL with a
non-zero length is a named error, and a NULL inner row is no longer undefined behaviour. The 7 .NET
sentinel branches are gone. The work matches next.md and all six criteria pass. The Codex review
found two real defects on top (a per-row allocation and a double free in two new C test macros);
both were two-line fixes and the review applied them.

**Verification:**

- [x] `cargo test -p iscc-ffi` passes with the new tests — 121 passed (120 from advance, 1 added by
    the review); `cargo clippy --workspace --all-targets -- -D warnings` clean
- [x] `grep -n 'slice::from_raw_parts(' crates/iscc-ffi/src/lib.rs` prints only the line inside
    `slice_or_empty` — and this is now a Rust test (`test_slices_are_built_only_in_slice_or_empty`)
- [x] The C test compiles (`-Wall -Wextra`, also with `-fsanitize=address`) and exits 0 — 142 passed
- [x] Regeneration is a no-op — after cbindgen and `cargo build -p iscc-ffi`, `git status` on
    `iscc.h` and `NativeMethods.g.cs` is empty (checked again after the review's fixes)
- [x] `dotnet test` passes (153), `grep -n sentinel` on `IsccLib.cs` + `packages/dotnet/CLAUDE.md`
    prints nothing
- [x] C++ suite passes against the rebuilt FFI (Debug + ASAN, 112 passed); `mise run check` exits 0

**Issues found:**

- `rows_or_empty` built the indexed row name with `format!` for every valid row. Measured with a
    release build and 1,000 four-byte digests: `iscc_alg_simhash` took 260-330 µs per call, 150-170
    µs after the fix. **Fixed in review:** `slice_or_empty` takes `name: impl Display`, the row call
    passes `format_args!`, so the name is built only on rejection. The 17 other call sites are
    unchanged
- `CHECK_NULL_EMPTY_STR` / `CHECK_NULL_EMPTY_BUF` in `test_iscc.c` passed a freeing comparison to
    `ASSERT_EQ`, which evaluates its argument again when it prints a failure: a failing check ended
    in a double free instead of a FAIL line. **Fixed in review:** the result is stored first. Proof:
    two deliberate mismatches under ASAN give two clean FAIL lines and no sanitizer report
- The step made four comments false ("several C FFI functions reject a NULL pointer before checking
    the count"): `iscc.hpp` (`detail::safe_data`), `packages/cpp/CLAUDE.md`,
    `tests/test_cpp_safe_data.py`, and the `PinnedRows` remarks in `IsccLib.cs`. **Fixed in
    review**, comment text only
- Filed `low`, untriaged: "C++ `safe_data` and .NET `PinnedRows` padding guard a NULL rejection the
    FFI no longer has". Both workarounds are now redundant but correct; whether to drop them is a
    call for Titusz
- Nit, not filed: no .NET test passes an empty span to `GenImageCodeV0`, `EncodeBase64` or
    `AlgMinhash256`. The FFI side of each is covered in Rust and C

**Codex review:** two P2 findings, both confirmed and fixed (the first two entries above). Codex
also re-ran the Rust, C and .NET suites green.

**Next:** The `normal` `[audit]` issue "Out-of-range integer arguments wrap silently on JS, raise
undocumented classes elsewhere" (napi, WASM, JNI, Python, Ruby; 5-6 non-test files, so cite the
audit valve). After that the published-docs issue.

**Notes:**

- Resolved and deleted: the `[audit]` issue "C FFI treats `(NULL, 0)` two opposite ways and never
    checks inner pointers". The rule is recorded as a decision (2026-11-21) and in
    `crates/iscc-ffi/CLAUDE.md` → Safety Rules
- `docs/c-ffi-api.md` now states the `(NULL, 0)` rule under "Memory Model" and the `iscc_encode_seq`
    error list matches the header. The published-docs step can build on that
- `ASSERT_EQ` and the other C test macros evaluate `actual` twice on failure. Noted in
    `crates/iscc-ffi/CLAUDE.md` → C tests: pass a stored value, never a call that frees
- `iscc-ffi` is outside the iai and CRAP gates, so no gate would have caught the per-row allocation.
    For a marshalling change on a row loop, time it once with a release build
- decisions.md went over 400 lines with the new entry and was rotated with the runner's
    `archive_overflow`; `decisions-archive.md` is part of this commit
