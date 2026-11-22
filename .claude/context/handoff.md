## 2026-11-22 — Decode vector that pins the 256-bit limit, and the sum-code bits range

**Done:** Replaced the input of `iscc_decode/test_0006_unit_above_256` with the reference encoder's
288-bit Meta-Code and rewrote its `note` (canonical file + 4 byte-identical copies, C header
regenerated). The two `gen_sum_code_v0` `bits` doc rows and the `genSumCodeV0` Javadoc now say 64 to
256\.

**Files changed:**

- `crates/iscc-lib/tests/data-edge.json`: row input + `note` (name and `error` unchanged, ASCII)
- `packages/{dotnet/Iscc.Lib.Tests/testdata,go/testdata,kotlin/src/test/resources,swift/Tests/IsccLibTests}/data-edge.json`:
    `cp` of the canonical file (all 5 md5 `26a75922f149272dcd5083f761b9214c`)
- `crates/iscc-ffi/tests/data_edge_vectors.h`: regenerated with
    `uv run --script scripts/gen_data_edge_vectors.py` (one line: new string, length 68)
- `docs/rust-api.md:302`, `docs/java-api.md:331`: "32 to 256" → "64 to 256" (same width, no re-pad)
- `crates/iscc-jni/java/src/main/java/io/iscc/iscc_lib/IsccLib.java:154`: `@param bits` drops 32

**Verification:**

- Pinning probe from empty `/tmp/r304x`:
    `uv run --no-project --with iscc-lib==0.7.0 python -c "import iscc_lib; print(iscc_lib.iscc_decode('ISCC:AAEAAAABAIBQIBIGA4EASCQLBQGQ4DYQCEJBGFAVCYLRQGI2DMOB2HQ7EAQSEIY'))"`
    →
    `(<MT.META: 0>, <ST.NONE: 0>, <VS.V0: 0>, 8, b'\x00\x01\x02\x03\x04\x05\x06\x07\x08\t\n\x0b\x0c\r\x0e\x0f\x10\x11\x12\x13\x14\x15\x16\x17\x18\x19\x1a\x1b\x1c\x1d\x1e\x1f !"#')`
    — 0.7.0 prints its IntEnums by name; the values are `(0, 0, 0, 8, <36 bytes>)`, so 0.7.0
    accepts the row's input
- `grep -c` of the new string: 1 in each of the 5 JSON files and 1 in the header; `AAEAAAICAMCAKBQ`:
    0 in all 6
- `uv run pytest tests/test_data_edge.py tests/test_vendored_fixtures.py tests/test_gen_data_edge_vectors.py`
    — 211 passed
- `cargo test -p iscc-lib --test test_data_edge` — 7 passed (HEAD rejects the row with the expected
    error); `packages/go`: `CGO_ENABLED=0 go test -count=1 ./...` — ok
- `git grep -n "32 to 256" -- docs/rust-api.md docs/java-api.md` no longer lists line 302 or 331
- `uv run zensical build` exit 0; `mise run check` exit 0
- Not run: the other 9 harnesses (.NET, Java, Ruby, C, C++, Kotlin, Swift, napi, WASM). The change
    is one fixture string in a row shape they already handle; the header no-op check is done by the
    reviewer after the commit

**Next:** The bits issue should be closable. Remaining `normal` work: the three `[audit]` issues
(Conan recipe checksum first, per next.md's ranking).

**Notes:** No behaviour change, so no CRAP/iai/semver impact. Other "32 to 256" rows in both pages
are left as next.md scoped (correct for those functions).
