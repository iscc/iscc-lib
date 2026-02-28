## 2026-02-28 — Add C FFI API reference page

**Done:** Created `docs/c-ffi-api.md` documenting all 44 exported `extern "C"` functions from
`crates/iscc-ffi/src/lib.rs`. Added nav entry in `zensical.toml` under Reference (after Python API)
and a reference line in `docs/llms.txt`.

**Files changed:**

- `docs/c-ffi-api.md`: New C FFI API reference page with 10 sections (overview/build, types,
    constants, code generation, text utilities, algorithm primitives, codec operations, streaming,
    diagnostics, memory management, error handling). All functions documented with C signatures and
    parameter tables.
- `zensical.toml`: Added `{ "C FFI" = "c-ffi-api.md" }` under Reference nav section
- `docs/llms.txt`: Added C FFI reference line under Reference section

**Verification:**

- `uv run zensical build` — succeeds (3.39s)
- `grep -q 'c-ffi-api.md' zensical.toml` — exits 0 ✓
- `grep -q 'c-ffi' docs/llms.txt` — exits 0 ✓
- `grep -c 'iscc_gen_' docs/c-ffi-api.md` — returns 21 (≥ 9, all gen functions documented with
    multiple references) ✓
- `grep -c 'iscc_free_' docs/c-ffi-api.md` — returns 29 (≥ 4) ✓
- `grep -q 'iscc_last_error' docs/c-ffi-api.md` — exits 0 ✓
- `mise run check` — all 14 hooks pass ✓

**Next:** The project is in maintenance mode with all functional requirements met. Remaining work is
human-dependent: merge PR #10 (develop → main), trigger releases, configure Maven Central
publishing. The only remaining automated task could be adding a Java API reference page (also
missing per the documentation spec).

**Notes:** next.md says "43 exported functions" but I counted 44 in the source: 4 constants + 2
alloc/dealloc + 9 gen functions + 4 text utils + 2 encoding + 3 codec + 1 conformance + 1 decompose

- 1 sliding_window + 4 algorithm primitives + 8 streaming hasher + 5 memory management + 1
    last_error = 45 total `#[unsafe(no_mangle)]` symbols (but `result_to_c_string`, `string_to_c`,
    `vec_to_c_string_array` etc. are internal helpers, not exported). All exported symbols are
    documented.
