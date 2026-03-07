## 2026-03-07 — Add C++ documentation (README, howto, root README)

**Done:** Created `packages/cpp/README.md` following the .NET README pattern, updated
`docs/howto/c-cpp.md` to replace the hand-written RAII wrapper section with comprehensive `iscc.hpp`
documentation (include paths, gen functions, streaming, error handling, codec, conformance), and
updated `README.md` to add C++ to the polyglot bullet, ecosystem list, install section, and
quickstart.

**Files changed:**

- `packages/cpp/README.md`: Created — badges, "What is ISCC", installation (tarball + source), quick
    start, API overview tables (10 gen functions, streaming, utilities), links, license
- `docs/howto/c-cpp.md`: Updated intro to mention `iscc.hpp` as recommended C++ approach; updated
    pre-built binaries tip to mention `iscc.hpp`; replaced lines 327-411 (old hand-written RAII
    wrapper) with comprehensive `iscc.hpp` section covering include paths (tarball flat vs
    CMake/source), quick start, gen functions, dual-hasher streaming, error handling,
    codec/utilities, and conformance verification
- `README.md`: Added "C++" to polyglot bullet (line 26) and ecosystem paragraph (line 50); added "C
    / C++" install section with GitHub Release link mentioning `iscc.hpp`; added C++ quickstart with
    `iscc::gen_meta_code_v0` example

**Verification:**

- `grep -c 'iscc.hpp' docs/howto/c-cpp.md` → 12 (≥5 required) ✓
- `grep -c 'iscc::gen_meta_code_v0' docs/howto/c-cpp.md` → 2 (≥1 required) ✓
- `grep -c 'iscc::DataHasher' docs/howto/c-cpp.md` → 2 (≥1 required) ✓
- `grep 'C++' README.md` → shows C++ in install and quickstart sections ✓
- `grep -c 'iscc.hpp' README.md` → 2 (≥1 required) ✓
- `test -f packages/cpp/README.md` → exits 0 ✓
- `grep -c 'gen_meta_code_v0' packages/cpp/README.md` → 2 (≥1 required) ✓
- `grep -c 'What is ISCC' packages/cpp/README.md` → 1 ✓
- `mise run check` → all 15 pre-commit hooks pass ✓
- Pre-push hooks → all pass (clippy, cargo test, pytest, type check, security, complexity, Ruby) ✓

**Next:** Remaining C++ issue sub-items: package manager manifests (`vcpkg.json`, `conanfile.py`),
`gen_mixed_code_v0` test gap in `test_iscc.cpp`, and nested vector null-safety hardening
(`safe_data` for inner elements). These are all explicitly listed as "Not In Scope" for this step.

**Notes:** The tarball flat layout vs `<iscc/iscc.hpp>` CMake include path distinction is now
documented in the howto guide as requested — tarball users use `#include "iscc.hpp"` with
`-I/path/to/tarball`, CMake/source users use `#include <iscc/iscc.hpp>`. mdformat reformatted some
line wrapping in the admonition blocks. No code changes — purely documentation.
