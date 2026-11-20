# Next Work Package

## Step: C FFI docs use the `iscc_`-prefixed type names of the generated `iscc.h`

## Goal

Resolve the `normal` issue "`docs/c-ffi-api.md` type names don't match the generated `iscc.h`":
every C type name in the C-facing docs matches the header that cbindgen emits, so the snippets
compile verbatim. A pytest derives the type list from `iscc.h` so the names cannot drift again.

## Alternatives Considered

- **Chosen:** C FFI doc type names. It is a user-facing defect (copied snippets do not compile), it
    can be checked by a compiler, and it is the largest of the small `normal` items
- **Rejected:** the Python `encode_seq` docstrings (two docstrings). Equally ready but smaller in
    user impact, so it goes next. The `wasm-bindings.md` spec line is internal-only. The
    derived-facts parity check is the largest item and deserves its own step. Dependabot `soupsieve`
    is waiting on Titusz

## Scope

- **Create**: `tests/test_c_ffi_docs.py`
- **Modify**: `docs/c-ffi-api.md`, `crates/iscc-ffi/README.md`
- **Reference**: `crates/iscc-ffi/include/iscc.h` (typedefs at L14-121),
    `crates/iscc-ffi/cbindgen.toml` (`[export] prefix = "iscc_"`), `.github/workflows/ci.yml` L138
    (the canonical cbindgen command)

## Not In Scope

- Editing `cbindgen.toml`, adding unprefixed `typedef` aliases, or regenerating `iscc.h` /
    `packages/dotnet/Iscc.Lib/NativeMethods.g.cs`. The resolution is prefixed names in the docs
- `crates/iscc-ffi/CLAUDE.md`: it uses the Rust names on purpose and maps them to the C names
- `docs/java-api.md` and the other language pages: `IsccDecodeResult` is a real Java class there
- Rewriting `size_t` parameter types in the function signatures on the page. This step changes type
    *names* and the four struct blocks only
- Tier 1 symbol parity, a prek hook, or any other part of the derived-facts issue
- The C++ `iscc.hpp` empty-vector entry (`low`, untriaged)

## Implementation Notes

The six exported types are `iscc_IsccByteBuffer`, `iscc_IsccByteBufferArray`,
`iscc_IsccDecodeResult`, `iscc_IsccSumCodeResult`, `iscc_FfiDataHasher` and `iscc_FfiInstanceHasher`
(all have a `typedef`, so the bare `iscc_X` form works without `struct`).

`docs/c-ffi-api.md`:

- Generation command (L24): replace it with the CI line verbatim:
    `cbindgen --config crates/iscc-ffi/cbindgen.toml --crate iscc-ffi --output crates/iscc-ffi/include/iscc.h`.
    Without `--config` the documented command does not produce the committed header
- Add one sentence under "Types" saying that cbindgen prefixes every exported type with `iscc_`
- The four struct blocks (`## Types` L41-75 and `#### IsccSumCodeResult` L343-353) must be the
    header's declarations: tagged `typedef struct iscc_X { … } iscc_X;` with the header's field
    types (`uintptr_t len`/`count`, not `size_t`). Copy them from `iscc.h`; keep the one-line field
    comments. Headings become the prefixed names
- Every other occurrence (prose, signatures L328-802, the IDv1 decode example ~L398, the
    `encode_seq` example ~L698, the streaming hasher signatures L730-765, the free-function table
    L793-802) uses the prefixed name
- Run `mise run format` before committing. mdformat reflows the free-function table once the cells
    get wider

`crates/iscc-ffi/README.md` L79, L105, L106: same rename. The "Streaming" bullet names the Rust
concepts `DataHasher`/`InstanceHasher` in prose and can stay as it is.

`tests/test_c_ffi_docs.py` (pure Python, no compiler, so it runs on every OS):

- Derive the type set from `iscc.h` with a regex over `typedef struct iscc_(\w+)`, and strip the
    prefix to get the bare names
- Anchor the parse: assert that the derived set contains `IsccByteBuffer` and `FfiDataHasher`, so a
    broken regex cannot pass on an empty set
- For each of the two doc files, assert that no bare name occurs at an identifier boundary
    (`(?<![A-Za-z0-9_])Name(?![A-Za-z0-9_])`), so `iscc_IsccByteBuffer` does not match. Report the
    file, line and name on failure
- Add a negative test: a short string with a bare `IsccByteBuffer` is flagged by the same helper
- Module docstring plus a short docstring per function (project convention)

## Verification

- `uv run pytest tests/test_c_ffi_docs.py` exits 0, and the negative test is among the tests that
    pass
- Compile check: put the IDv1 decode snippet (~L395-404) and the `encode_seq` snippet (~L697-711)
    verbatim inside `int main(void) { … return 0; }` after `#include <stdio.h>` and
    `#include "iscc.h"`, in a scratch file outside the repo. Then
    `gcc -std=c11 -fsyntax-only -I crates/iscc-ffi/include <file>.c` exits 0
- `docs/c-ffi-api.md` contains the string `--config crates/iscc-ffi/cbindgen.toml`
- `git status --porcelain crates/iscc-ffi/include/iscc.h packages/dotnet/Iscc.Lib/NativeMethods.g.cs`
    prints nothing
- `uv run zensical build` exits 0
- `mise run check` exits 0

## Done When

The advance agent is done when all six verification criteria pass, with the type names changed in
both doc files and the parity test committed.
