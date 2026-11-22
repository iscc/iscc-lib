## 2026-11-22 — Review of: Shared vectors and API pages for the 256-bit unit limit

**Verdict:** PASS_WITH_NOTES

**Summary:** The step did what next.md asked: four `data-edge.json` rows in the 5 fixture copies and
the C header, and the range "32 to 256" on every API page and comment that said only "multiple of
32". All suites are green on all 12 surfaces. Two defects came in with it, both from next.md's own
notes, so the bits issue stays open in a smaller form: the decode row does not pin the bound, and
the sum-code `bits` row got a range whose lower end the code rejects.

**Verification:**

- [x] `uv run pytest tests/test_data_edge.py tests/test_vendored_fixtures.py tests/test_gen_data_edge_vectors.py`
    — 211 passed (includes the iscc-core cross-check of the `diverges` markers)
- [x] `cargo test -p iscc-lib --test test_data_edge` (also the three CI feature sets) and
    `CGO_ENABLED=0 go test -count=1 ./...` (amd64 and `GOARCH=386`) pass; the 4 row names are in the
    canonical file, and the 5 copies have one md5
- [x] Header generator, `cargo build -p iscc-ffi` (after `cargo clean -p iscc-ffi`, so `build.rs`
    ran) and the cbindgen line leave `git status` empty
- [x] `git grep -n "multiple of 32" -- docs crates packages | grep -v 256` — prints the two test
    comments plus three `streaming.rs` lines. Those are false hits: each `///` sentence continues
    with "from 32 to 256." on the next line (in place since 301)
- [x] `uv run zensical build` exits 0, `mise run check` exits 0
- [x] .NET (155), Java (125), Ruby, C (gcc), C++ (cmake + ASAN), Kotlin (`DataEdgeTest` 6/6), Swift
    (11 `DataEdge` tests), napi (in a `git archive` copy) and WASM (8 + 136) pass

**Issues found:**

- **The decode row passes without the bound** (`normal`, in the rewritten bits issue). next.md built
    the string as header `00 08` over 36 bytes. A length index of 8 takes two nibbles, so the header
    is `00 08 00` and the body has 35 bytes. Published iscc-lib 0.7.0, which has no bound, rejects
    the row as "truncated ISCC body". The row's `note` is false as well. The right string is in the
    issue, with measurements on 0.7.0, HEAD, the Go port and both iscc-core builds
- **Sum-code rows say "32 to 256", and `bits=32` fails** (`normal`, same issue): `docs/java-api.md`
    (this step) and `docs/rust-api.md` (since 301). The valid range is 64 to 256. Every other
    function with a changed row accepts 32 and 256 and rejects 0 and 288 (measured on the Python
    binding: meta, text, image, audio, video, mixed, data, instance, both hashers)
- **Filed `low`, untriaged:** `gen_mixed_code_v0` with `bits` below 8 panics in the Go port and in
    debug builds of the Rust core. Pre-existing; found by the range probe. Not for the loop
- Both reference claims in the other three rows hold on iscc-core 1.5.0 under CPython 3.10 (compiled
    wheel) and 3.15 (pure Python)

**Codex review:** No actionable findings. It ran the Rust, Python, C and Go tests, including 32-bit
Go.

**Next:** Close the rest of the bits issue. It is a small rows-and-docs step with no harness change:

- Replace the input of `iscc_decode/test_0006_unit_above_256` with
    `ISCC:AAEAAAABAIBQIBIGA4EASCQLBQGQ4DYQCEJBGFAVCYLRQGI2DMOB2HQ7EAQSEIY` and correct its `note`,
    in the canonical file and the 4 copies, then regenerate the C header
- Change the two `gen_sum_code_v0` `bits` rows (`docs/rust-api.md`, `docs/java-api.md`) to "64 to
    256"
- Verification that proves the row this time: the string decodes on the last release
    (`uv run --no-project --with iscc-lib==0.7.0 python -c "import iscc_lib; print(iscc_lib.iscc_decode('<string>'))"`,
    run from an empty directory) and fails at HEAD

After that, the three `[audit]` `normal` issues remain.

**Notes:**

- A grep criterion of the form `git grep "multiple of 32" | grep -v 256` is line-based. The three
    wrapped `///` sentences in `streaming.rs` will always show up; name them as expected hits
- The step changed no behaviour: no CRAP, iai or semver impact. No quality gate was touched
- `/tmp/r303-verify.sh` runs all 31 checks in about 3 minutes on a warm tree
