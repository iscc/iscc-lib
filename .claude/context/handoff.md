## 2026-11-21 — Review of: Complete the C/C++ how-to, register it FULL, and drop the 0.4 crate pin

**Verdict:** PASS

**Summary:** `docs/howto/c-cpp.md` now names every Tier 1 symbol through runnable C++ examples and
is registered `FULL` in `scripts/check_tier1_docs.py`. Both minimal-build snippets are version-free
(`cargo add iscc-lib --no-default-features`). The diff is additions only, four files, no source or
gate change. Every output comment in the new blocks matches what the code prints.

**Verification:**

- [x] `uv run scripts/check_tier1_docs.py` exits 0 — `OK: 20 full and 2 exact Tier 1 docs match`
- [x] Registry one-liner exits 0
- [x] `uv run pytest tests/test_check_tier1_docs.py tests/test_c_ffi_docs.py` — 42 passed
- [x] `git grep -nE 'version = "0\.' -- docs crates/iscc-lib/README.md` prints nothing; both files
    hold one `cargo add iscc-lib --no-default-features` line (`cargo add --help` lists the flag)
- [x] Every new C++ block compiles and runs — I extracted all ```` ```cpp ```` blocks with
    `int main()` and built each alone with `g++ -std=c++17 -Wall -Wextra` against a fresh
    `cargo build -p iscc-ffi`: no warnings, exit 0, output equal to the comments for all six new
    blocks (Gen functions, `encode_component`, Text utilities, Encoding utilities, Algorithm
    primitives, Constants). The three older `main()` blocks also run
- [x] `uv run zensical build` exits 0 ("No issues found"); `mise run check` exits 0

Extra checks: with the previous `c-cpp.md` restored in a throwaway copy the gate fails and lists
exactly the 17 symbols from next.md, so the registration is effective. Every C++ function the new
sections use has one `iscc_<name>` export in `iscc.h` and an entry in `docs/c-ffi-api.md`.

**Issues found:**

- Filed `low`, untriaged: "The pkg-config example in the C/C++ how-to says `Version: 0.2.0`". A
    hand-typed version in a copy-paste `.pc` file, pre-existing, same class as the removed 0.4 pin
- Nit, not filed: the closing sentence "Each C++ function above wraps the C function of the same
    name with an `iscc_` prefix" does not hold for `iscc::iscc_decode` and `iscc::iscc_decompose`
    (C: `iscc_decode`, `iscc_decompose`). The examples it gives are right
- Nit, not filed: `FULL` is not strictly alphabetical (`docs/java-api.md` sits before the
    `docs/howto/` group). That order predates this step; the new line is in order within its group
- Not filed: the older C++ blocks without `main()` (first Gen functions block, Codec and utilities,
    ISCC-SEQ, ISCC-IDv1, Error handling) are fragments and were left untouched, as next.md asked

**Issue resolution:**

- "The minimal-build snippet pins `iscc-lib` to 0.4": resolved, entry deleted
- "Derived facts in guidance and docs are written by hand and drift": the docs part is done. The
    entry is rewritten to the one part left (counts, file lists and line numbers in
    `.claude/context/specs/` and the per-crate `CLAUDE.md` files). It is the only `normal` issue

**Codex review:** No actionable findings. Codex re-ran the gate, the 42 tests and the docs build,
and compiled the six new C++ examples with matching output.

**Next:** the last part of "Derived facts in guidance and docs are written by hand and drift":
remove or gate the unchecked counts, file lists and line numbers in `.claude/context/specs/` and the
per-crate `CLAUDE.md` files. It is a wide sweep. Start by measuring it (digit greps plus count words
such as "three" or "all 35", which a digit grep misses), then pick a slice that fits one step:
either one directory (`specs/` or the `CLAUDE.md` files) or one kind of fact. Sub-specs are
correctable for mechanically checkable facts; `target.md` is not.

**Notes:**

- No Rust source changed: CRAP, iai and semver gates are unaffected, no API break
- `scripts/version_sync.py` covers only the Maven, Gradle and SPM snippets in docs. Any other
    version written into a doc is hand-typed and will drift
- The C++ examples have no committed check. next.md excluded a C++ signature-pinning test, so their
    output comments are verified by hand in this step only
