## 2026-11-20 — Review of: C++ wrappers pass non-NULL pointers for every vector input

**Verdict:** PASS

**Summary:** `iscc.hpp` now has one `detail::safe_data` template, and every vector input reaches the
FFI through it, so `{}` reports the core's own error instead of the FFI's "must not be NULL". The
diff stays inside next.md's scope (one header, its test file, the package `CLAUDE.md`). The issue
"C++ list wrappers pass NULL for an empty vector" is resolved and deleted.

**Verification:**

- [x] `cargo build -p iscc-ffi`, then the CI `cpp` job's CMake Debug + `SANITIZE_ADDRESS=ON` build
    and `test_iscc` — exit 0, 107 passed, 0 failed (fresh build dir `build-review/`, cmake from
    `uvx cmake`)
- [x] A passing `{}` test for each of `gen_mixed_code_v0`, `gen_iscc_code_v0`, `gen_video_code_v0`,
    `soft_hash_video_v0` and `alg_simhash`, each checking the core message from next.md — all five
    PASS lines present
- [x] `grep -n '\.data()' packages/cpp/include/iscc/iscc.hpp | grep -v 'v\.data()'` — prints
    nothing; the only `.data()` is inside `safe_data`
- [x] `uv run prek run --files <the 3 files>` — exit 0 (and `mise run check` exit 0)
- [x] `packages/cpp/CLAUDE.md` no longer describes `.data()` as an accepted way to pass an input
    vector — both Type Mapping rows and the pitfall say "always `detail::safe_data()`"

Extra probes (2 of 3):

- The previous header (`git show HEAD~1:…/iscc.hpp`) against the current tests gives 103 passed, 4
    failed: exactly `gen_mixed_code_v0`, `gen_iscc_code_v0`, `gen_video_code_v0` and
    `soft_hash_video_v0`, each with the FFI's "must not be NULL" message. The tests see the defect
- `g++ -std=c++17 -Wall -Wextra -Wpedantic -Werror -fsyntax-only` on the test file — exit 0. CMake
    sets no warning flags, so the CI build would not show a warning in the template

**Issues found:**

- Fixed here: four lines in `iscc.hpp` were longer than 100 characters (the header had none before).
    They are wrapped; the suite was rebuilt and is still 107/0
- Fixed here: nothing kept the "never `.data()`" rule after this step. `alg_simhash`,
    `alg_minhash_256`, `encode_component` and the hasher `update` overloads behave the same with
    either pointer form, so no behaviour test can hold it for them. `tests/test_cpp_safe_data.py`
    (pytest, pure Python) now fails on any `.data()` in `iscc.hpp` outside the helper, and
    `packages/cpp/CLAUDE.md` names it
- Spec, corrected as the closed `[human]` issue's `**Spec:**` field asked: the `rust-core.md`
    algorithm table showed `sliding_window` and `alg_simhash` returning a bare `Vec`. Both rows now
    show `IsccResult<…>`, and `sliding_window`'s parameter is named `seq` as in the code
- Nit, not filed: the `data_edge/gen_mixed_code_v0/…empty_list` and `gen_iscc_code_v0` rows stay
    green with the old header. They assert a rejection, not its message, which is what
    `edge-vectors.md` asks of them. The message tests in block `42a` cover the difference
- Not this loop's file: the advance `MEMORY.md` is 153 lines, over its 140-line budget

**Codex review:** No actionable findings. Codex also built the C++ suite under AddressSanitizer and
got 107 passed.

**Next:** The issue "`scripts/check_spec_paths.py` accepts paths the specs do not have" (`normal`,
three sub-fixes with one test each: `*` must stay within one path segment, `NOT_PATHS` only for
backticked spans, and the two replay tests on a fixed path set). It was second in line in the last
two handoffs.

**Notes:**

- Seven `normal` issues remain: the spec path gate, the JNI null-handle guard, the JNI pending
    exception, Ruby negative Bignums, the `iscc-ffi` `lib.rs` split, derived facts in docs, and the
    `data-edge.json` harness compare in Swift and Python
- Still open for Titusz (carried over): a digest list with mixed lengths. iscc-core 1.5.0 silently
    truncates `[b"\x00", b"\x00\x00"]`, Rust and Go reject it. No decision covers it, so no vector
    was added
- The FFI still checks NULL before the count in four of the five list functions and the count first
    in `iscc_alg_simhash`. next.md ruled the reorder out of scope: it would change FFI behaviour for
    C callers. No issue filed; the C++ and .NET wrappers no longer depend on the order
- No Rust, no public `iscc-lib` API and no benchmarked path was touched. `cmake` is not on `$PATH`
    in the devcontainer; `uvx cmake` works
