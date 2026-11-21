# Next Work Package

## Step: Generated C edge header keeps every string literal within 4,095 bytes

## Goal

Fix the `normal` issue "Generated `data_edge_vectors.h` holds a 128 KB string literal that MSVC
cannot compile". The generator writes any string over the ISO C99 literal minimum (4,095 bytes) into
a buffer that a header-provided init function fills at runtime. That keeps the C and C++ test
programs portable to MSVC, and a later large vector cannot break them again.

## Alternatives Considered

- **Chosen:** header-literal fix. It is left over from iteration 281, touches only test tooling, and
    gcc can check it here (`-Woverlength-strings`) although no MSVC is available.
- **Rejected:** `[audit]` "`@iscc/wasm` docs show a Node.js setup the published package cannot run"
    (handoff option 2). It is equally small and goes next. This fix comes first because the next
    data-edge vector would make it bigger.
- **Rejected:** a decisions.md entry limiting the test programs to gcc/clang. It gives up
    portability that a small generator change keeps.

## Scope

- **Modify**: `scripts/gen_data_edge_vectors.py`; `crates/iscc-ffi/tests/data_edge_vectors.h`
    (regenerated, not hand-edited); `crates/iscc-ffi/tests/test_iscc.c`;
    `packages/cpp/tests/test_iscc.cpp`
- **Modify (tests/docs)**: `tests/test_gen_data_edge_vectors.py`; `crates/iscc-ffi/CLAUDE.md` (:145)
    and `packages/cpp/CLAUDE.md` (:97): one sentence each on the init call
- **Reference**: MSVC C2026 (16,380 bytes per literal piece) and C1091 (65,535 after concatenation):
    https://learn.microsoft.com/en-us/cpp/error-messages/compiler-errors-1/compiler-error-c2026,
    https://learn.microsoft.com/en-us/cpp/error-messages/compiler-errors-1/fatal-error-c1091

## Not In Scope

- Do not change `data-edge.json`, its 5 copies or any other harness. The fixture stays
    byte-identical.
- Do not change `struct iscc_edge_str`/`iscc_edge_list` or the per-section check functions.
- Do not render short strings any differently: all rows under the threshold stay byte-identical.
- No Windows/MSVC CI job (separate `low` issue "No CI job runs a test suite on Windows").

## Implementation Notes

Design (no struct change, no per-call-site accessor):

- Threshold `C_LITERAL_MAX = 4095` (the C99 minimum literal length; gcc's `-Woverlength-strings`
    warns above it in C mode, which serves as the local stand-in for MSVC). Measure the decoded byte
    length, not the escaped text.
- A string longer than that is rendered as `static char <ident>_<field>_buf[<len + 1>];`
    (zero-initialised, so it ends in NUL for `strlen`). Its row field is `{<ident>_..._buf, <len>}`:
    the address of a static array is a constant initialiser in C and C++.
- The header ends (before the postamble) with `static void iscc_edge_init(void)`, which `memcpy`s
    each buffer from literal pieces of at most 4,095 bytes, using explicit byte counts so an
    embedded NUL survives. Emit the function even when no buffer exists (an empty body) so both
    harnesses can always call it. Add `#include <string.h>`.
- Route **every** string the generator emits through this rule (`c_str`, `byte_list_arrays` items,
    row names): one form per surface, not a meta-only special case. Unique identifiers per field:
    the row `ident` plus a field or item suffix.
- `test_iscc.c` and `test_iscc.cpp` call `iscc_edge_init()` once in `main`, before the first edge
    section runs. A missing call fails closed in C (`strlen` mismatch), but C++ would read 128 KB of
    NUL, which the core also rejects as an error. So add a pytest asserting that both harness
    sources contain the call.
- Update the preamble comment that describes the row layout so it mentions the buffers.

Tests to add to `tests/test_gen_data_edge_vectors.py`:

- Tokenise the tracked header's string literals, join adjacent pieces, decode the escapes (reuse
    `c_escape_chars`-style scanning) and assert each is at most `C_LITERAL_MAX` bytes. Count the
    literals that were checked and assert the count is non-zero.
- On a `tmp_path` fixture with an oversized meta that contains non-ASCII text, assert that the
    rendered init pieces, unescaped and joined, equal the original UTF-8 bytes and that the declared
    buffer length is `len + 1`.

Ordering trap: `test_tracked_header_matches_fresh_render` stays the regeneration gate. Regenerate
with `uv run --script scripts/gen_data_edge_vectors.py`. No Rust code changes, so CRAP/iai are not
affected.

## Verification

- `uv run pytest tests/test_gen_data_edge_vectors.py tests/test_data_edge.py tests/test_vendored_fixtures.py`
    passes, including the new literal-length, oversized-render and harness-init-call tests
- `gcc -std=c99 -Woverlength-strings -Werror -fsyntax-only -x c crates/iscc-ffi/tests/data_edge_vectors.h`
    exits 0 (it fails on the current header)
- The C test program, built and run as in the `c-ffi` job of `ci.yml`, reports 0 failed, and
    `test_0004_meta_payload_over_limit` reports PASS
- The C++ suite (`uv run --with cmake cmake` Debug + ASAN build of `packages/cpp/tests`, as in the
    `cpp` job) reports 0 failed, with the oversized row PASS
- `grep -c 'iscc_edge_init()' crates/iscc-ffi/tests/test_iscc.c packages/cpp/tests/test_iscc.cpp`
    shows at least 1 for each file
- `mise run check` exits 0

## Done When

All six verification criteria pass, and the regenerated header contains no string literal over 4,095
bytes.
