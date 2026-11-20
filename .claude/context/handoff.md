## 2026-11-20 — C++ wrappers pass non-NULL pointers for every vector input

**Done:** The three `detail::safe_data` overloads in `iscc.hpp` are now one
`template <typename T> const T* safe_data(const std::vector<T>& v)` with a read-only
`static const T sentinel{}`. All nine vector-backed `.data()` sites that reach the FFI now go
through it, so `{}` reports the core's error instead of the FFI's "must not be NULL".

**Files changed:**

- `packages/cpp/include/iscc/iscc.hpp`: one template `safe_data` instead of the three overloads, doc
    comment kept. `encode_component` (digest), `alg_simhash` (ptrs, lens), `alg_minhash_256`
    (features), `soft_hash_video_v0` and `gen_video_code_v0` (ptrs, lens), `gen_mixed_code_v0` and
    `gen_iscc_code_v0` (ptrs), and both `DataHasher`/`InstanceHasher::update(vector)` now use it
- `packages/cpp/tests/test_iscc.cpp`: block `42a` adds one `assert_throws_with` test on `{}` for
    each of the five list wrappers, using the core messages from next.md. Block `42b` adds:
    `alg_minhash_256({})` returns 32 bytes; `encode_component(0,0,0,64,{})` raises no error that
    mentions NULL; `DataHasher` and `InstanceHasher` `update(std::vector<uint8_t>{})` + `finalize`
    give the same code as `gen_{data,instance}_code_v0({})`
- `packages/cpp/CLAUDE.md`: both Type Mapping rows now say "always `detail::safe_data()` +
    `.size()`". The "Empty vector data pointers" pitfall now says to always use `safe_data` and
    never `.data()`, including the pointer and length arrays built inside list wrappers

**Verification:**

- `cargo build -p iscc-ffi`, then CMake Debug + `SANITIZE_ADDRESS=ON` (via `uv run --with cmake`,
    built into the gitignored `build-uv/` rather than `build/`) and
    `LD_LIBRARY_PATH=../../target/debug ./build-uv/tests/test_iscc`: exit 0, **107 passed, 0
    failed** (98 at the last review, plus 9 new). PASS lines include `gen_mixed_code_v0(empty)`,
    `gen_iscc_code_v0(empty)`, `gen_video_code_v0(empty)`, `soft_hash_video_v0(empty)` and
    `alg_simhash(empty)`, each checking its core message
- Mutation check: with `gen_mixed_code_v0` and `gen_video_code_v0` switched back to
    `ptrs.data()`/`lens.data()`, the run gives 105 passed, 2 failed
    (`message "codes must not be NULL"` and `"frame_sigs must not be NULL"`). With the header
    restored it gives 107/0 again
- `grep -n '\.data()' packages/cpp/include/iscc/iscc.hpp | grep -v 'v\.data()'` prints nothing (exit
    1). The only `.data()` left is line 48, inside `safe_data`
- `uv run prek run --files <the 3 files>`: exit 0. The first run let mdformat realign the CLAUDE.md
    table; the second run passed with no changes
- `mise run check`: exit 0

**Next:** The gate-leniency issue in `scripts/check_spec_paths.py`, which the last handoff put
second.

**Notes:**

- When the reviewer closes the issue, the `**Spec:**` follow-up from the last handoff still applies:
    the Tier 1 rows in `rust-core.md` for `alg_simhash` and `sliding_window` still show a bare
    `Vec`. I did not touch them; they are out of scope
- `lens` is a `std::vector<size_t>` and goes to `const uintptr_t*`. That is the same type pairing as
    before the change; only the null-safety differs
- I did not run the pre-push stage. The change is C++ only (no Rust or Python touched), and the C++
    suite above is the CI `cpp` job's runner. No hot path and no `iscc-lib` public API was touched
