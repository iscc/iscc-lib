## 2026-11-19 — `data-edge.json` seed set with the Rust and Python harnesses

**Done:** Created `crates/iscc-lib/tests/data-edge.json`. It holds 15 vectors in 5 sections: the
spec seed set plus the 3 migrated strict-base32 rows (2 case-fold rows were already seeds). It runs
from a Rust harness (`check_doc` → failure list) and a Python suite (schema, iscc_lib, iscc_core
1.5.0 reference, seed presence). `STRICT_BASE32_DIVERGENCES` and its 2 tests are gone from
`test_seq.py`. The file and its Rust test are excluded from the packaged crate.

**Files changed:**

- `crates/iscc-lib/tests/data-edge.json`: new. It was generated with
    `json.dumps(indent=2, ensure_ascii=False)`, the same style as data.json (literal `ſ`/`ß`; NUL
    written as `\u0000`). It has no `_metadata` key, so harnesses see function sections only
- `crates/iscc-lib/tests/test_data_edge.rs`: new. It reads the file at runtime via
    `CARGO_MANIFEST_DIR`. Unknown sections fail through `KNOWN_SECTIONS`, `except.rust` is honoured,
    and only the keys present in `outputs` are compared. It has 4 negative tests (one bad case each)
    plus an `except` test and a section-coverage test
- `tests/test_data_edge.py`: new. It has `validate_vector` + `validate_except`, one caller per
    section shared by iscc_lib and iscc_core, and parametrized ids `section/name`. Tests: schema on
    every vector, iscc_lib outcome (`ValueError` on errors), reference iff-`diverges`, a hard-coded
    15-row `SEED_SET` presence list, and 11 malformed-shape negatives
- `crates/iscc-lib/Cargo.toml`: `exclude = ["tests/data-edge.json", "tests/test_data_edge.rs"]`
- `tests/test_vendored_fixtures.py`: `data-edge.json` is in `FIXTURE_BASENAMES`, there is a
    `*data-edge.json` pathspec, and the canonical path is registered with an empty copy list
- `tests/test_seq.py`: removed the list and its 2 tests, and updated the module docstring to point
    at data-edge.json

**Verification:**

- `cargo test -p iscc-lib --test test_data_edge`: 7 passed, including the 4 negatives. The same test
    also passes 7/7 under `--no-default-features`
- `uv run pytest tests/test_data_edge.py tests/test_seq.py tests/test_vendored_fixtures.py`: 151
    passed (test_data_edge.py alone: 73)
- `grep -n STRICT_BASE32_DIVERGENCES tests/test_seq.py`: no match (exit 1)
- `cargo package --list -p iscc-lib --allow-dirty`: the only `data` line is `tests/data.json`
- `cargo clippy -p iscc-lib --all-targets -- -D warnings`: clean
- `mise run check`: exit 0, re-run with the new files staged. The first run had them untracked and
    missed a ruff BLE001; fixed
- Pre-push gates run by hand: `uv run ty check` passes (one `sorted(object)` diagnostic fixed),
    `cargo test -p iscc-lib` passes (all binaries ok, 0 failed), full `uv run pytest`: 583 passed
- Throwaway probe: `matches()` returns False for a wrong encode_seq output, for the reference
    accepting a strict row, and for a wrong `characters` field

**Next:** The binding fan-out. Copy `data-edge.json` next to the 4 vendored `data.json` copies, fill
the empty `VENDORED_COPIES` entry, and add a harness on each of the other 11 surfaces. The
`iscc_decode` output encoding is now fixed as `[mt, st, vs, li, "bytes:<hex>"]`.

**Notes:**

- Rust `gen_text_code_v0` is compiled out without `text-processing`. In that build the harness
    returns `Ok(None)` for the section (a feature-gated skip, like `conformance_selftest`), not an
    unknown-section failure
- Rust `gen_iscc_code_v0` reads an optional `inputs[1]` bool as `wide` (default false). Python calls
    `f(*inputs)` for gen sections
- The `except.c: skip` on the encode_seq NUL row is from next.md. The other `except` maps are the
    spec example, copied verbatim
- Not caused by this step: `cargo clippy -p iscc-lib --all-targets --no-default-features` fails
    because `benches/benchmarks.rs:9` and `benches/iai_benches.rs:31` import the gated
    `gen_meta_code_v0`/`gen_text_code_v0` (E0432). CI's no-default clippy (ci.yml:36) has no
    `--all-targets`, so CI is unaffected. `--tests --no-default-features` is clean
- Deviation: next.md says that, for the reference, "any exception = error". ruff BLE001 rejects
    `except Exception`, so `outcome()` catches `(ValueError, AssertionError)`. Those are the classes
    iscc_core raises here (`binascii.Error` for the NUL unit is a ValueError subclass). Any other
    class propagates and turns the test red instead of passing silently
