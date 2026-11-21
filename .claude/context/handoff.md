## 2026-11-21 — Review of: JNI loader tests pass in the release layout, and CI runs the suite in that layout

**Verdict:** PASS

**Summary:** The child-JVM loader tests now find a real fallback library in both layouts (the
`java.library.path` entry in CI, a temp copy of the bundled resource in the release layout), a new
test pins the both-fail error, and the CI `java` job runs the suite a second time with the library
only bundled. The diff touches the two test files and `ci.yml` only, as scoped. The `critical` issue
is resolved and deleted; the loader commit `8a99a3c` and this step are pushed, and CI run
44955385494 is green on all jobs.

**Verification:**

- [x] CI layout: `cargo build -p iscc-jni && mvn -q test -f crates/iscc-jni/java/pom.xml` exits 0 —
    run in a `git archive HEAD` tree with the library only in `target/debug`: IsccLibTest tests=114,
    skipped=0; the three `loaderFallsBack*` tests and `loaderReportsBothFailures` ran
- [x] Release layout: `mvn test` exits 0 with the same four tests run, none skipped — same tree,
    library only under `src/main/resources/META-INF/native/linux-x86_64/`, `target/debug` empty:
    `Tests run: 122, Failures: 0, Errors: 0, Skipped: 0`
- [x] The both-fail test fails without `addSuppressed` — exit 1, `expected: <3> but was: <2>`. Two
    more mutations of mine: without `initCause` it fails the same way; with the bundled-library
    error rethrown instead of kept, `loaderFallsBackOnBrokenBundledLibrary` fails too
- [x] `ci.yml` job `java` has the step (copy to resources, move the library away, `mvn test`), and
    its body exits 0 when run as CI runs it — extracted with PyYAML, run with
    `bash --noprofile --norc -eo pipefail` in the throwaway tree. In CI itself: both steps report
    `Tests run: 122 … Skipped: 0` and `BUILD SUCCESS` (job log of run 44955385494)
- [x] `mise run check` exits 0

**Issues found:**

- Three more pages carry the false "all 10 gen functions return a string" sentence:
    `docs/howto/java.md`, `docs/java-api.md` (`genSumCodeV0` returns `SumCodeResult`) and
    `docs/c-ffi-api.md` (`iscc_gen_sum_code_v0` returns a struct). Pre-existing; added to the
    existing `normal` docs issue, now titled "Six docs pages count 10 standard functions or give
    Sum-Code a string result"
- Not filed: `loaderReportsBothFailures` skips through `writeBrokenBundledLibrary`'s `assumeTrue` on
    a host with no bundled-library slot. Neither CI nor `release.yml` → `test-jni` runs on such a
    host (both `ubuntu-latest`, x86_64)
- Not filed: the new CI step hard-codes `linux-x86_64`. On another runner architecture the parent
    JVM would fail to load the library, so the step fails loudly instead of passing empty

**Codex review:** "No actionable regressions found." It ran the suite in both layouts itself (122
tests, no skips in each).

**Next:** the docs-only batch, both `normal` issues in one step: "`docs/architecture.md` sections
below the graph cover half the surfaces" and "Six docs pages count 10 standard functions or give
Sum-Code a string result" (`ecosystem.md`, `architecture.md`, `howto/nodejs.md`, `howto/java.md`,
`java-api.md`, `c-ffi-api.md`). Scope it with a repo-wide `git grep -niE 'all 10' -- docs` and state
for each hit whether it is a library-wide count (correct) or a return-type claim (check it against
the binding's signature). `docs/howto/wasm.md:82` has the same sentence but belongs to the `[audit]`
WASM docs issue, which needs a probe against a `--target web` build and is better as its own step.

**Notes:**

- No `critical` issue is open. Open `normal` issues: the two docs issues above, the WASM Node.js
    docs (`[audit]`), the one-shot/streaming duplication (`[audit]`, mind the iai baseline) and the
    `alg_minhash_256([])` decision (`[audit]`)
- The advance handoff's open question about `Swatinem/rust-cache` is settled: with
    `target/debug/libiscc_jni.so` moved away, `cargo build -p iscc-jni` restores it in 0.2 s (a hard
    link from `target/debug/deps`), byte-identical. A cache saved without the file is harmless
- A new Java test that spawns a child JVM should take its library directory from
    `assertLoaderProbePasses` / `runLoaderProbe`; the second CI step now catches one that reads
    `target/debug` directly
- Each bundled-library load still leaves an empty `/tmp/iscc-jni-*` directory (the `low` issue,
    human-directed only); the release-layout step adds a few per CI run on an ephemeral runner
- No decisions.md entry: the step made no trade-off beyond what next.md recorded
