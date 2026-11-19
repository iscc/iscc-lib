# Next Work Package

## Step: data-edge.json fan-out, group 3 — C and C++ harnesses via one generated header

## Goal

Last fan-out group of the v0.8.0 criterion "Shared edge-case vectors run everywhere": the C FFI test
and the C++ wrapper test run every `data-edge.json` vector from a header that a recovered generator
renders, with a pytest gate that fails when the header is stale.

## Alternatives Considered

- **Chosen:** C + C++ together. The behaviour is fixed by `edge-vectors.md` → Rules (which also
    allows a generated table), both toolchains run here, and it closes the last surface gap
- **Rejected:** the `normal` Python `encode_seq` docstring issue. It is a two-line docs fix with no
    dependency on this work, while C/C++ are what remains of a v0.8.0 criterion

## Scope

**Fan-out:** `edge-vectors.md` → Rules (every vector runs, `except` is the only deviation, an
unknown section fails, the rejection is asserted on the raw call only), applied to C and C++. Commit
the generator + header + pytest gate first, then C, then C++.

- **Create**: `scripts/gen_data_edge_vectors.py` (recover `scripts/gen_ffi_boundary_vectors.py` from
    `ea69169` with `git show ea69169:<path>` and adapt it; keep its PEP 723 block, `escape_c_string`
    octal rule and pure-ASCII/LF output), the generated `crates/iscc-ffi/tests/data_edge_vectors.h`,
    `tests/test_gen_data_edge_vectors.py` (adapt
    `git show ea69169:tests/test_gen_ffi_boundary_vectors.py`)
- **Modify**: `crates/iscc-ffi/tests/test_iscc.c`, `packages/cpp/tests/test_iscc.cpp`,
    `packages/cpp/tests/CMakeLists.txt` (include dir for the header), `crates/iscc-ffi/CLAUDE.md`
    and `packages/cpp/CLAUDE.md` (name the header and the regenerate command, count-free wording)
- **Reference**: `.claude/context/specs/edge-vectors.md`, `crates/iscc-lib/tests/data-edge.json`,
    `tests/test_data_edge.py` (expectation logic, `SURFACE_IDS`), `crates/iscc-ffi/include/iscc.h`,
    `packages/cpp/include/iscc/iscc.hpp`, `git show ea69169 -- crates/iscc-ffi/tests/test_iscc.c`

## Not In Scope

- Any change to `data-edge.json`, the FFI, `iscc.hpp`, or the core. If a vector fails on a surface
    for a behavioural reason, add no `except`, leave that surface's harness out and name the vector
    in the handoff (stop rule from 217)
- The C++ gen wrappers passing `ptrs.data()` (NULL for an empty vector) where `encode_seq` uses
    `detail::safe_data`: the input is still rejected. Note it in the handoff, do not fix it here
- `ci.yml` / `release.yml` edits: the C compile finds a quote-include next to `test_iscc.c`
- Registering the header in `VENDORED_COPIES` (it is not a JSON copy)

## Implementation Notes

- **One header for both surfaces**, valid C99 and C++17: positional aggregate initialisers only, no
    designated initialisers, no zero-length arrays. C++ gets it via
    `target_include_directories(test_iscc PRIVATE …/crates/iscc-ffi/tests)` on the test target —
    never on the public `iscc` INTERFACE library (vcpkg/Conan consumers inherit that).
- **Strings carry an explicit length** (e.g. `struct { const char *ptr; size_t len; }`): two vectors
    hold an embedded NUL, and C++ must build `std::string(ptr, len)` so `detail::checked_c_str` sees
    it. The C harness fails a row it runs whose `strlen(ptr) != len` (a missing `c: skip` must go
    red, not hash the prefix).
- **The generator resolves `except`**: each row carries one expectation for `c` and one for `cpp`
    (ok / error / skip), from `outputs`/`error` overridden by `except[<surface>]`. The harnesses
    never name a vector; they loop over count macros.
- **The generator fails closed** (`SystemExit`) on a section it has no renderer for, on an
    unexpected input arity, and on an `except` key outside the surface ids. That is this surface
    pair's "unknown section fails" rule; pytest pins it on a mutated fixture copy in `tmp_path`.
- **Per-section shapes**: `iscc_decode` (string → maintype, subtype, version, length index, digest
    bytes); `encode_seq` (string list → bytes); `gen_text_code_v0` (string, bits → `iscc`);
    `gen_mixed_code_v0` (string list, bits → `iscc`); `gen_iscc_code_v0` (string list, `wide` =
    `inputs[1]` if present else false → `iscc`). Both surfaces return the gen result as one string,
    so they compare `iscc` only. Error rows carry zeroed outputs.
- **Lists**: an empty list must reach the C FFI as a non-NULL pointer with count 0 (emit a sentinel
    element), so the core's empty-list check rejects it, not the NULL guard. C++ passes an empty
    `std::vector<std::string>`.
- **Rejection**: C — NULL string, `ok == false`, or NULL buffer data from the raw FFI call; C++ —
    the raw `iscc::` call throws `iscc::IsccError` (any other exception is a failure). Never match
    message text. Free every returned value.
- **Harness shape per surface**: one check function per section that takes a row and returns
    pass/fail without printing, plus a loop printing `PASS: data_edge/<section>/<name>` or `FAIL:`
    per row through the file's existing helpers. Negative tests feed the check functions synthetic
    rows, one bad case each: an expected rejection that succeeds (`ISCC:GAAAAAAAAA`), an output
    mismatch (a wrong expected `iscc`), and for C the length/NUL mismatch.
- **pytest gate**: fresh render equals the tracked header byte for byte; a mutated expected value
    changes the render; unknown section, bad arity and bad `except` key raise `SystemExit`; the NUL
    `gen_text_code_v0` row renders as c=skip, cpp=error. Derive counts from the fixture, never a
    literal. Load the script by path as the `ea69169` test did.
- Nothing touches `crates/iscc-lib/src` or `tests/*.rs`, so CRAP and iai are unaffected.

## Verification

- `uv run pytest tests/test_gen_data_edge_vectors.py tests/test_data_edge.py tests/test_vendored_fixtures.py`
    passes, and `mise run check` exits 0
- `uv run --script scripts/gen_data_edge_vectors.py` leaves
    `git status --porcelain crates/iscc-ffi/tests` empty
- `grep -nE 'case_fold|spare_bits|ascii_twin|canonical_unit|empty_string|base32_padding|empty_list|embedded_nul'`
    over `crates/iscc-ffi/tests/test_iscc.c` and `packages/cpp/tests/test_iscc.cpp` prints nothing
    (no data-edge vector named in harness code; existing `data.json` `test_0001` comments are fine)
- C: `cargo build -p iscc-ffi`, then the `ci.yml` "Compile C test program" and "Run C test program"
    lines exit 0, and the output has a `PASS: data_edge/<section>/<name>` line for every fixture
    vector whose `except.c` is not `skip`, plus a PASS line for each negative test
- C++: after `cargo build -p iscc-ffi`, configure a fresh build dir with
    `uv run --with cmake cmake -S packages/cpp -B /tmp/cpp-edge` plus the `ci.yml` "Configure CMake"
    flags (`FFI_LIB_DIR` as an absolute path to `target/debug`), build it, and run
    `/tmp/cpp-edge/tests/test_iscc` with `LD_LIBRARY_PATH=target/debug`: exit 0, no ASAN report, a
    `PASS: data_edge/…` line for every fixture vector whose `except.cpp` is not `skip`, and a PASS
    line for each negative test

## Done When

C and C++ run every `data-edge.json` vector from a generated header whose staleness pytest catches,
and every check above passes (a surface held back under the stop rule is the one sanctioned
exception, reported in the handoff).
