## 2026-11-21 — Review of: Generated C edge header keeps every string literal within 4,095 bytes

**Verdict:** PASS_WITH_NOTES

**Summary:** The generator now writes every string over 4,095 bytes as a static buffer that the
header's `iscc_edge_init()` fills from literal pieces, and both test programs call it. All six
criteria pass, every other row of the header is byte-identical, and the `normal` MSVC issue is
closed. One note: the fix could not be run on MSVC, and Microsoft's own pages disagree on whether a
4,095-byte piece is allowed (filed `low`).

**Verification:**

- [x] `uv run pytest tests/test_gen_data_edge_vectors.py tests/test_data_edge.py tests/test_vendored_fixtures.py`
    — 172 passed on the advance commit, 173 after the review's added test. The new literal scanner
    reports 128,002 bytes on the previous header, so the test fails on the old form
- [x] `gcc -std=c99 -Woverlength-strings -Werror -fsyntax-only -x c crates/iscc-ffi/tests/data_edge_vectors.h`
    — exit 0; exit 1 on the header of the define-next commit
- [x] C test program built and run as the `c-ffi` job — 148 passed, 0 failed,
    `test_0004_meta_payload_over_limit` PASS (also clean with `-Wall -Wextra`)
- [x] C++ suite (`uv run --with cmake cmake`, Debug + ASAN) — 119 passed, 0 failed, oversized row
    PASS
- [x] `grep -c 'iscc_edge_init()'` — 1 in `test_iscc.c`, 1 in `test_iscc.cpp`
- [x] `mise run check` — exit 0

Beyond next.md: the generator re-run is a no-op and `data-edge.json` is unchanged. A `/tmp` fixture
with an oversized list item, digest item and row name renders one buffer each, compiles under
`gcc -std=c99 -pedantic -Werror` and `g++ -std=c++17 -Werror`, and reads back correctly under ASAN.

**Issues found:**

- **Filed `low`, untriaged:** "Generated C header fills long strings from 4,095-byte literals, over
    one documented MSVC limit". The C2026 page allows 16,380 bytes per quoted piece; the C language
    reference page "Maximum String Length" (2021) says 2,048. Unmeasured: no MSVC here or in CI.
    Lowering `C_LITERAL_MAX` to 2,047 would satisfy both pages
- Fixed in review: `escape_c_string` lost its last caller in this step and survived only through its
    own unit test. Removed; the test now calls `escape_c_bytes`
- Added in review: `test_oversized_list_item_and_row_name_render_buffers`. The advance tests covered
    only the `c_str` path, not list items (`_i<k>_buf`) or row names (`_name_buf`)
- Nit, not filed: without the init call the C++ program still passes, because the only long row
    expects an error. The pytest that asserts the call in both sources is the guard (decisions.md)
- Nit, not filed: both Microsoft URLs in next.md return 404 without `?view=msvc-170`

**Codex review:** No actionable findings. Codex re-ran the 172 Python tests, the C program (148) and
the C++ ASAN suite (119) and the header compiler checks, all green.

**Next:** The `[audit]` issue "`@iscc/wasm` docs show a Node.js setup the published package cannot
run" (2 non-test files). Probe the Node example against a packed `--target web` release build, as
the entry asks. Other open `normal` issues, any of which is a valid pick: the `alg_minhash_256([])`
decision, the vcpkg portfile hashes, the WASM hasher `finalize()` rule, the Rust API docs tiers, the
one-shot/streaming duplication, the Rust and Go `data.json` re-parsing.

**Notes:**

- decisions.md has a new entry on why long strings are runtime-filled buffers (a `const` array
    initializer would push the header over the 256 KB added-file limit). Two older entries rotated
    into `decisions-archive.md`
- A new data-edge vector with a long string now needs no special care in C: the generator handles
    any string in any field. The header is 150 KB of the 256 KB hook limit
- No Rust code changed: no CRAP, iai or API impact
