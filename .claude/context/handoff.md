## 2026-11-18 — Review of: Bind `encode_seq` / `decode_seq` on the C FFI

**Verdict:** PASS_WITH_NOTES

**Summary:** `iscc-ffi` exports `iscc_encode_seq` / `iscc_decode_seq` as thin wrappers that reuse
the existing input loop, the len-0 guard and the existing free functions. The regenerated `iscc.h`
and `NativeMethods.g.cs` are committed with them, and ISCC-SEQ now covers 5 of 11 surfaces. Scope
was clean: 2 non-test source files plus generated artifacts, nothing from Not In Scope. Review added
the spec's byte-direction round-trip test and fixed one contradictory `# Safety` line (details
below).

**Verification:**

- [x] `cargo test -p iscc-ffi`: 98 passed (99 after the review test). It covers the data.json seq
    walk, the round trip and 8 one-input negatives, each pinned to its own error substring
- [x] `cargo clippy -p iscc-ffi --all-targets -D warnings` and `cargo fmt -p iscc-ffi --check` are
    clean. Workspace clippy also exits 0
- [x] The header is fresh: the cbindgen output diffs empty against `iscc.h`, which declares both
    symbols (lines 645/670). Still fresh after the review's doc edit
- [x] The C test compiled with the ci.yml gcc line (also clean under `-Wall -Wextra`):
    `78 passed, 0 failed`, including test 31 (round trip) and test 32 (`num_units = 0`)
- [x] `NativeMethods.g.cs` declares both symbols. `git status --porcelain` on both generated files
    is empty after a build. `dotnet build Iscc.Lib.csproj`: Build succeeded, 0 warnings, 0 errors
- [x] `mise run check`: exit 0, no context files rewritten
- [x] Probe: valgrind on the C test gives 0 errors and "All heap blocks were freed"
- [x] Probe: the core `iscc_conformance_selftest` runs both seq sections. Note that `test_iscc.c`
    does not call it; the FFI vector coverage is the Rust `mod tests` walk

**Issues found:**

- Fixed in review: the spec's round trip ("`encode_seq(decode_seq(x)) == x` for every vector",
    `rust-core.md`) was untested on the FFI. next.md prescribed only the string direction
    (decode-of-encode). Review added `test_seq_round_trip_bytes` over the `decode_seq` vectors, the
    same form the Python, napi and WASM tests use
- Fixed in review: the `iscc_free_byte_buffer` summary now names `iscc_encode_seq`, but its
    `# Safety` still said "algorithm primitive functions". It now says "functions listed above",
    with `iscc.h` and `NativeMethods.g.cs` regenerated (doc-comment line only)
- Nit, not fixed: the `# Safety` text on `iscc_encode_seq` / `iscc_decode_seq` forbids NULL and bad
    UTF-8 even though the code handles both. The text is copied from `iscc_gen_iscc_code_v0` /
    `iscc_encode_component`; it is conservative, not unsafe
- Filed: `lib.rs` is 2870 lines, past the crate's ~2500 split threshold. csbindgen reads only
    `src/lib.rs`, so a split needs care

**Codex review:** (run on `69914f8`, the main advance commit) (1) the round-trip direction: valid,
fixed above. (2) The `# Safety` wording on the two exports: the house-style nit above, not fixed.
(3) The `iscc_free_byte_buffer` `# Safety` contradiction: valid, fixed above.

**Next:** the .NET seq step (`IsccLib.cs` wrappers over `NativeMethods.iscc_encode_seq` /
`iscc_decode_seq`, marshalling `byte** units` the way `GenIsccCodeV0` does), then C++ `iscc.hpp`.
**next.md for every remaining surface must name the BYTE round trip**
(`encode_seq(decode_seq(x)) == x` over the `decode_seq` vectors), not decode-of-encode. Remaining
normal issues are the Dependabot `uv.lock` and Ruby `json` alerts, the WASM npm license, the Ruby
locale, the Python docstring, the napi Buffer leak, c-ffi-api type names, and the `lib.rs` split.
All are small and can be interleaved.

**Notes:** the deferred doc sweep (`docs/c-ffi-api.md`, `crates/iscc-ffi/CLAUDE.md` naming list,
Tier 1 33→35) waits until every surface binds seq. Any `///` edit to an FFI export changes both
generated files, so regenerate and commit them in the same step. No core or hot-path code changed:
iai, CRAP and conformance are unaffected.
