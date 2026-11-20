## 2026-11-20 — C FFI docs use the `iscc_`-prefixed type names of the generated `iscc.h`

**Done:** Every exported C type name in `docs/c-ffi-api.md` and `crates/iscc-ffi/README.md` now
carries the `iscc_` prefix that cbindgen emits. The four struct blocks are the header's tagged
declarations with `uintptr_t` fields. The generation command is the CI line with `--config`. A
pytest derives the type set from `iscc.h` and fails on any bare name in either doc.

**Files changed:**

- `docs/c-ffi-api.md`: cbindgen command replaced with the ci.yml L138 line; one prefix sentence
    under "Types"; the four struct blocks rewritten as `typedef struct iscc_X { … } iscc_X;` with
    `uintptr_t len`/`count` and `struct iscc_IsccByteBuffer` members (copied from `iscc.h`, field
    comments kept); headings, prose, signatures, both examples, the streaming signatures and the
    free table renamed (32 boundary-aware substitutions, then mdformat reflow)
- `crates/iscc-ffi/README.md`: L79, L105, L106 renamed (table reflowed). The "Streaming" bullet
    (`DataHasher`/`InstanceHasher`, Rust concept names) is unchanged as next.md allows
- `tests/test_c_ffi_docs.py` (new): `header_type_names` (regex `typedef struct iscc_(\w+)`),
    `find_bare_names` (identifier-boundary regex, longest name first, returns `(line, name)`);
    tests: parse anchor (`IsccByteBuffer`, `FfiDataHasher` in set), one per doc file (failure
    message lists `file:line: name`), negative test (bare `IsccByteBuffer` on line 2 flagged,
    prefixed line 1 not), and a longer-name test (`IsccByteBufferArray` reported whole, not as its
    prefix)

**Verification:**

- [x] `uv run pytest tests/test_c_ffi_docs.py`: 5 passed, including `test_bare_name_is_flagged`
- [x] The helper run against `git show HEAD:docs/c-ffi-api.md` (pre-fix doc) reports 39 hits (first:
    line 32 `IsccByteBuffer`), so the test would have caught the defect
- [x] Compile check: IDv1 snippet and `encode_seq` snippet, each copied verbatim (awk-extracted from
    the doc) into its own `/tmp/*.c` inside `int main(void) { … return 0; }` after `<stdio.h>` +
    `"iscc.h"`. `gcc -std=c11 -fsyntax-only -I crates/iscc-ffi/include`: both exit 0. Control: the
    same file with the old `IsccByteBuffer seq` fails with "unknown type name 'IsccByteBuffer'; did
    you mean 'iscc_IsccByteBuffer'?"
- [x] `grep -c -- '--config crates/iscc-ffi/cbindgen.toml' docs/c-ffi-api.md` prints 1
- [x] `git status --porcelain` on `iscc.h` and `NativeMethods.g.cs` prints nothing
- [x] `uv run zensical build` exits 0 ("No issues found")
- [x] `mise run check` exits 0; `prek run --hook-stage pre-push --files <3 files>` exits 0

**Next:** Python `encode_seq` docstrings (the rejected alternative from this step), then the
`wasm-bindings.md` spec line, then the derived-facts parity check.

**Notes:**

- The first pre-push run failed `ty check` (`no-matching-overload` on `str.join` over an unannotated
    `sorted(names, key=len)`); fixed by adding type hints to the three helpers, not a suppression
- I realigned the trailing `//` comments of the IDv1 snippet after the rename made its first line
    longer (whitespace only)
- The prefix sentence names `iscc_IsccByteBuffer` as its example rather than the Rust name, so the
    doc stays free of bare names under the test's rule
- `size_t` parameter types in function signatures are unchanged (out of scope per next.md), so the
    page still mixes `size_t` (signatures) and `uintptr_t` (struct fields and `iscc_encode_seq`)
