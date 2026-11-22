## 2026-11-22 — Review of: Decode vector that pins the 256-bit limit, and the sum-code bits range

**Verdict:** PASS

**Summary:** The decode row `iscc_decode/test_0006_unit_above_256` now carries the reference
encoder's 288-bit Meta-Code, which iscc-lib 0.7.0 decodes and HEAD rejects, so the row fails on a
build without the bound. The five fixture copies are byte-identical, the C header regenerates to a
no-op, and the two `gen_sum_code_v0` doc rows and the `genSumCodeV0` Javadoc state 64 to 256. The
diff touches only what next.md scoped. The `normal` `[human]` issue is closed and deleted.

**Verification:**

- [x] The row pins the bound — from empty `/tmp/r304rev`, the 0.7.0 wheel prints
    `(<MT.META: 0>, <ST.NONE: 0>, <VS.V0: 0>, 8, b'\x00…#')` (36 bytes), that is `(0, 0, 0, 8, …)`.
    The new string appears once in each of the 5 JSON files (all md5 `26a75922…`) and once in
    `data_edge_vectors.h`; `AAEAAAICAMCAKBQ` appears in none
- [x] `uv run pytest tests/test_data_edge.py tests/test_vendored_fixtures.py tests/test_gen_data_edge_vectors.py`
    — 211 passed, including the iscc-core cross-check of the `diverges` markers
- [x] `cargo test -p iscc-lib --test test_data_edge` — 7 passed; `packages/go`
    `CGO_ENABLED=0 go test -count=1 ./...` — ok
- [x] `uv run --script scripts/gen_data_edge_vectors.py`, then
    `git status --porcelain crates/iscc-ffi/tests/data_edge_vectors.h` — prints nothing
- [x] `git grep -n "32 to 256" -- docs/rust-api.md docs/java-api.md` shows no sum-code row; lines
    302 and 331 read "64 to 256"; the `genSumCodeV0` `@param bits` lists 64 to 256 without 32
- [x] `uv run zensical build` exit 0; `mise run check` exit 0

Extra probes (2 of 3): iscc-core 1.5.0 raises "288 is not a valid LN" for the new string on CPython
3.10 and 3.15, so the row is right to carry no `diverges`. `SumHasher.finalize(32)` raises on the
0.7.0 wheel ("ISCC unit code too short"), 64 returns a code.

**Issues found:**

- The rustdoc of `streaming::SumHasher::finalize` (`crates/iscc-lib/src/streaming.rs:198`) still
    said "from 32 to 256". It is the same false range on the sibling of `gen_sum_code_v0`, and
    next.md did not list it. Fixed in this review as a one-word comment edit ("from 64 to 256"); no
    other surface states a range for the sum code or `SumHasher` (checked `docs/`, the Python
    wrapper and stubs, the WASM crate, `c-ffi-api.md`, `ruby-api.md`)
- No new issue filed

**Codex review:** No actionable findings. It confirmed that the vector matches the reference
encoder, that the copies and the header are in sync, and that the documented range matches runtime
behaviour; it ran the Rust, Python, Go and C tests.

**Next:** Three `normal` `[audit]` issues block v0.8.0. Take "Conan recipe downloads the FFI archive
without a checksum" first, as next.md ranked it (3 non-test files plus tests: `conanfile.py`, a hash
script after the pattern of `scripts/update_vcpkg_hashes.py`, `release.yml`). The other two are the
Rust conformance runner duplication and the spec and API-doc drift (9 files: audit-scope step or a
split into specs and docs).

**Notes:**

- The nine harnesses that advance did not run (.NET, Java, Ruby, C++, Kotlin, Swift, napi, WASM, and
    C apart from Codex's run) get their first run of the new row in CI on this push. The row keeps
    its shape and `error` text, and all Rust-backed surfaces share the core path that the Rust test
    covers, so a red job there would point at a harness, not at the vector
- No behaviour change: no CRAP, iai or semver impact. The review's comment edit keeps the line count
    of `streaming.rs`
