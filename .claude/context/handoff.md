## 2026-11-20 — Review of: C FFI docs use the `iscc_`-prefixed type names of the generated `iscc.h`

**Verdict:** PASS

**Summary:** Every exported C type name in `docs/c-ffi-api.md` and `crates/iscc-ffi/README.md` is
now the `iscc_`-prefixed name from `iscc.h`. The four struct blocks match the header field for
field, and a pytest derives the type set from the header so the names cannot drift again. The diff
is small, in scope and clean. The `normal` issue is resolved and deleted.

**Verification:**

- [x] `uv run pytest tests/test_c_ffi_docs.py` exits 0. 5 passed at the advance commit, including
    `test_bare_name_is_flagged`; 6 after the review added `docs/howto/c-cpp.md`
- [x] Compile check: the IDv1 snippet (L398-408) and the `encode_seq` snippet (L700-712), each
    pasted verbatim into `main()` after `<stdio.h>` + `"iscc.h"`, pass
    `gcc -std=c11 -fsyntax-only -I crates/iscc-ffi/include` (both exit 0)
- [x] `docs/c-ffi-api.md` contains `--config crates/iscc-ffi/cbindgen.toml` (1 match)
- [x] `git status --porcelain` on `iscc.h` and `NativeMethods.g.cs` prints nothing
- [x] `uv run zensical build` exits 0 ("No issues found")
- [x] `mise run check` exits 0 and leaves the tree clean. Pre-push `ty check` and ruff `S`/`C901` on
    the new test are green too

Probes beyond next.md (3):

- All 38 prototype blocks on the page compile at file scope after `#include "iscc.h"`, with no
    conflicting-type errors. A script compared the four struct blocks with the header: field types
    and names are identical
- The test helper run on the pre-step docs finds 39 bare names in `c-ffi-api.md` and 3 in the
    README, so the test would have caught the defect
- Sibling sweep: `docs/howto/c-cpp.md` and `packages/cpp/README.md` have no bare names. Inbound
    links to the renamed headings: none

**Issues found:**

- next.md's premise "without `--config` the documented command does not produce the committed
    header" is false. cbindgen 0.29.4 with `--crate iscc-ffi` loads `crates/iscc-ffi/cbindgen.toml`
    by itself, and the config-less run writes a byte-identical header. So the command change only
    makes the page match CI, and the `docs/howto/c-cpp.md` L37 command (no `--config`) is correct
- Nit, not filed: bare C names remain in comments and prose in `packages/cpp/CLAUDE.md` (L61, L174)
    and `packages/cpp/include/iscc/iscc.hpp` (L126, L147). No compiled code is affected
- Nit, not filed: the page mixes `size_t` (function parameters, out of scope per next.md) with
    `uintptr_t` (struct fields, `iscc_encode_seq`). Both are the same type on every supported target
- Accepted deviation: the advance agent added a second sentence under "Types" saying the `struct`
    keyword is optional. All six types in `iscc.h` have a `typedef`, so it is true

**Review fix (step 9):** `tests/test_c_ffi_docs.py` now also checks `docs/howto/c-cpp.md`, the third
C-facing page. It is clean today, and before this nothing checked it.

**Codex review:** ran with `--base 219cc49` over both advance commits; no actionable findings ("type
names and struct layouts match the checked-in `iscc.h`").

**Next:** fix the Python `encode_seq` docstrings (`normal` \[human\]: `_lowlevel.pyi:56` and the
`iscc_lib/__init__.py` wrapper must name non-ASCII case-fold aliases next to padding and spare
bits). After that come the `wasm-bindings.md` spec line (`normal`, a file-list fact, no escalation),
then the derived-facts parity check (`normal` [human], the largest item). The Dependabot `soupsieve`
bump still waits on Titusz.

**Notes:**

- The larger-step trial (`low` [human]) is evaluated after iteration 224, so iterations 222-224
    still count toward it
- `learnings.md` is exactly at its 200-line budget: one entry moved to `learnings-archive.md`
