## 2026-03-01 — Export META_TRIM_META in Java and Go bindings

**Done:** Added `META_TRIM_META = 128_000` constant to Java (`IsccLib.java`) and Go (`codec.go`)
bindings, completing issue #18 — all 6 binding crates now export this constant.

**Files changed:**

- `crates/iscc-jni/java/src/main/java/io/iscc/iscc_lib/IsccLib.java`: Added
    `public static final int META_TRIM_META = 128_000;` with Javadoc, placed after
    `META_TRIM_DESCRIPTION`
- `crates/iscc-jni/java/src/test/java/io/iscc/iscc_lib/IsccLibTest.java`: Added
    `assertEquals(128_000, IsccLib.META_TRIM_META)` assertion, updated comment from "4" to "5"
    constants
- `packages/go/codec.go`: Added `MetaTrimMeta = 128_000` to the const block, placed after
    `MetaTrimDescription`

**Verification:**

- `cargo test -p iscc-jni` — passed (0 Rust tests, expected; JNI tests are Java-side)
- `cd crates/iscc-jni/java && mvn test -q` — passed (all Java tests including updated
    `testConstants` with 5 assertions)
- `cd packages/go && go test ./... -count=1` — passed (147 tests)
- `cd packages/go && go vet ./...` — clean
- `cargo clippy -p iscc-jni -- -D warnings` — clean
- `grep -c 'META_TRIM_META' IsccLib.java` → 1 (constant declared)
- `grep -c 'MetaTrimMeta' codec.go` → 1 (constant declared)
- `mise run check` — all 14 pre-commit hooks passed

**Next:** Issue #18 is now fully complete (6/6 bindings). Focus shifts to issue #15
(`gen_sum_code_v0` / `SumCodeResult`) as the next major feature work.

**Notes:** Pure source-language constants — no FFI/JNI bridge involved. Maven POM is at
`crates/iscc-jni/java/pom.xml` (not `crates/iscc-jni/pom.xml`). No surprises or technical debt.
