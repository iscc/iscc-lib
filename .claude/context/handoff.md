## 2026-11-21 — Generated C edge header keeps every string literal within 4,095 bytes

**Done:** `scripts/gen_data_edge_vectors.py` routes every string it emits (`c_str` fields, list
items, row names) through `c_text`: a string over `C_LITERAL_MAX = 4095` bytes becomes a zeroed
`static char <ident>_<field>_buf[len + 1];` with row field `{<buf>, len}`, and the header ends with
`static void iscc_edge_init(void)` that `memcpy`s each buffer from literal pieces of at most 4,095
bytes with explicit counts (emitted with an empty body when no buffer exists; `<string.h>` added).
Both harnesses call `iscc_edge_init()` at the top of their edge block.

**Files changed:**

- `scripts/gen_data_edge_vectors.py`: `C_LITERAL_MAX`, `c_text`, `buffer_decl`, `init_function`;
    renderers/`gen_iscc`/`byte_list_arrays`/`list_arrays` take a per-section `bufs` collector;
    `render_section` returns `(text, bufs)` and declares its buffers first; preamble + docstring
- `crates/iscc-ffi/tests/data_edge_vectors.h`: regenerated. Diff = 2 preamble lines, the include,
    one buffer decl (`iscc_edge_gen_meta_code_v0_3_meta_buf[128003]`), that one meta field, and the
    init function (32 `memcpy` pieces). Every other row is byte-identical
- `crates/iscc-ffi/tests/test_iscc.c`, `packages/cpp/tests/test_iscc.cpp`: one init call each
- `tests/test_gen_data_edge_vectors.py`: callers updated for the new signatures; new tests: literal
    scanner self-test, tracked-header literals ≤ 4,095 (count non-zero), init always defined,
    harnesses call init before the first section, oversized non-ASCII meta on `tmp_path` (buffer
    `len + 1`, contiguous offsets, pieces rebuild the UTF-8, a piece boundary falls mid-sequence),
    NUL-only oversized string, exactly-4,095-byte string stays a literal
- `crates/iscc-ffi/CLAUDE.md`, `packages/cpp/CLAUDE.md`: one sentence each on the init call

**Verification:**

- `uv run pytest tests/test_gen_data_edge_vectors.py tests/test_data_edge.py tests/test_vendored_fixtures.py`
    — 172 passed (generator file alone: 39 passed)
- `gcc -std=c99 -Woverlength-strings -Werror -fsyntax-only -x c` on the header: exit 0 now; exit 1
    on the HEAD header ("string length '128002' is greater than the length '4095'"). Also
    `g++ -std=c++17 -Wall -Wextra -Werror -fsyntax-only -x c++` on the header: exit 0
- C program built as the `c-ffi` job (`gcc … -liscc_ffi`, plus `-Wall -Wextra`, no warnings): 148
    passed, 0 failed, `test_0004_meta_payload_over_limit` PASS. Probe with the init call deleted:
    147 passed, 1 failed (that row) — the C side fails closed
- C++ (`uv run --with cmake cmake` Debug + `SANITIZE_ADDRESS=ON`, as the `cpp` job): 119 passed, 0
    failed, the oversized row PASS
- `grep -c 'iscc_edge_init()'`: test_iscc.c 1, test_iscc.cpp 1
- The test scanner finds 119 literals in the new header (max 4,095 bytes) and 88 in the HEAD header
    (max 128,002); 119 matches an independent regex count
- `mise run check` exit 0; `ruff --select S,C901` and `ty check` on both Python files clean
- Generator re-run is a no-op; `data-edge.json` unchanged vs HEAD

**Next:** The `[audit]` issue "`@iscc/wasm` docs show a Node.js setup the published package cannot
run" (handoff option 2 from iteration 281).

**Notes:**

- Collector design: renderers append `(buf_name, bytes)` to a per-section list passed in, rather
    than returning a third tuple element — fewer signature churns; `render_section` stays pure at
    its boundary (returns its buffers). Ident suffixes: `_name`, `_input`, `_digest`, `_seq`,
    `_text`, `_iscc`, `_input_name`, `_description`, `_meta`, `_simhash`, `_data`, list items
    `_i<k>`
- The literal check measures decoded bytes; a 4,095-byte limit also keeps each piece under MSVC's
    16,380-byte C2026 limit. MSVC itself was not run (none available)
- C++ without the init call would read 128 KB of NUL; the only oversized row expects an error, so it
    would still pass there. The harness-call pytest is the guard for C++ (not probed by build)
- Header grew 147,274 → 149,873 bytes (one octal-escaped copy per piece instead of one literal)
- No Rust code changed: CRAP/iai unaffected, no API change
