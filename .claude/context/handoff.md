## 2026-11-20 — Review of: Static uncompared-key sets in the .NET, Java, napi and WASM data-edge harnesses

**Verdict:** PASS

**Summary:** The four harnesses now walk the expected keys, skip a constant per-section set
(`gen_text_code_v0` → `characters`, `gen_mixed_code_v0` → `parts`, every other section → none) and
fail on a missing key. The runtime key intersection is gone on all four. The diff is four test files
in four per-surface commits, exactly as scoped, and every suite is green. The `normal` `[human]`
issue about the lenient harnesses is resolved and deleted.

**Verification:**

- [x] `dotnet test packages/dotnet/Iscc.Lib.Tests/ -e LD_LIBRARY_PATH=$(pwd)/target/debug` exits 0
    after `cargo build -p iscc-ffi` — 153 passed, 0 failed, with the two negatives and the synthetic
    positive
- [x] `mvn clean test -f crates/iscc-jni/java/pom.xml` exits 0 — surefire: `DataEdgeTest` 8/0,
    `IsccLibTest` 108/0
- [x] `npm test` in `crates/iscc-napi` after `npm run build` exits 0 — 182 pass, 0 fail; the three
    data-edge cases are `ok 6`–`ok 8`
- [x] `wasm-pack test --node crates/iscc-wasm --features conformance` exits 0 — 11 + 8 + 94 passed
- [x] `grep -rn "result exposes" …` over the four paths prints nothing, and none of the four
    `compare` functions reads the actual object's key set: each loops over the expected keys and
    asks the actual only "do you have this key"
- [x] `mise run check` exits 0

Beyond next.md's list:

- [x] Static sets checked against the source: `data.json` carries `characters` only for text and
    `parts` only for mixed, and all four surfaces return a bare string (or a one-field record) for
    text, mixed and iscc
- [x] .NET mutant in a throwaway copy (old intersect `Compare` restored): exactly
    `DataEdgeFailsKeyOutsideUncomparedSet` and `DataEdgeFailsResultWithoutIscc` fail, the other six
    pass. With the advance agent's napi mutant that is two of four surfaces run; Java and WASM use
    the same two fixtures
- [x] `cargo clippy --workspace --all-targets -- -D warnings` and the wasm32 `--tests` clippy line
    are clean
- [x] The other eight harnesses (Rust, Python, Go, Ruby, Kotlin, Swift, C, C++) hold no key
    intersection either, so the spec's Verified When line on static uncompared keys now holds

**Issues found:**

- Not filed (cosmetic): the WASM `compare` has no `expected an object, got …` branch like the other
    three. A non-object result there reports `missing field iscc`, which still fails the vector
- Not filed: the `gen_mixed_code_v0` → `parts` entry has no test on any of the four surfaces,
    because `data-edge.json` holds only an error vector for mixed. A wrong entry would fail a future
    mixed vector loudly, so nothing passes by mistake

**Codex review:** Ran over the whole advance range (`--base` the define-next commit). It ran the
four suites itself and reported no actionable findings.

**Next:** The docs step that next.md deferred, both parts `normal` and release-blocking:

1. `docs/howto/java.md` → the "single pass" `genSumCodeV0` example: four arguments
    (`path, bits, wide, addUnits`) and `public final` fields instead of accessor calls. Compile the
    example against `IsccLib.java` / `SumCodeResult.java` rather than trusting a read
2. `docs/howto/ruby.md` and `crates/iscc-rb/README.md`: state which Ruby versions get a precompiled
    gem (3.1–3.3) and that newer ones build from source. This is the wording branch of the Ruby
    issue; widening the cross-gem list in `release.yml` has static evidence only and is the riskier
    branch

After that, the remaining `normal` work is the three parts of "Derived facts in guidance and docs".

**Notes:**

- Test-only step: no binding source, fixture or generated file changed, so the CRAP, iai and semver
    gates are unaffected and no decisions.md entry was needed
- `.NET` never runs an object compare on the real file (its only object-valued vector carries
    `except.dotnet`), so the synthetic positive is the only thing exercising that branch there. Keep
    it when touching the harness
- learnings.md is at its 200-line budget; nothing was added this round
