# Handoff

## 2026-07-28 — Migrate the JNI binding from jni 0.21 to jni 0.22

**Done:** Bumped the workspace `jni` pin to `"0.22"` (resolved 0.22.4), deleted its `# held:` block,
and reworked all 33 `extern "system"` functions in `crates/iscc-jni/src/lib.rs` onto the
`EnvUnowned::with_env(...).resolve::<ThrowRuntimeExAndDefault>()` pattern. The exception contract is
unchanged: `throw_and_default`/`throw_state_error` still call `env.throw_new` themselves and return
`Ok(T::default())`, so the policy (which checks `exception_check` first) never overwrites them.

**Files changed:**

- `Cargo.toml`: `jni = "0.21"` → `"0.22"`, `# held: jni` comment block removed (criterion/uniffi
    holds untouched)
- `crates/iscc-jni/src/lib.rs`: full 0.21→0.22 API migration (details in Notes)
- `crates/iscc-jni/CLAUDE.md`: type-mapping table, error-handling, local-frame, and pitfalls
    sections updated to the 0.22 API
- `Cargo.lock` (generated): jni 0.21.1→0.22.4, jni-sys 0.3.1→0.4.1, +jni-macros/simd_cesu8/simdutf8,
    −cesu8/thiserror 1.x/windows-sys 0.45 stack; `cargo tree -i` confirms jni and every new
    transitive reach only `iscc-jni`

**Verification:** (all six next.md checks, run this session)

- `grep -n 'jni = ' Cargo.toml` → line 49 `jni = "0.22"`; `grep -n 'held:'` → only criterion (44)
    and uniffi (51)
- Negative grep for
    `JNIEnv|env.get_string|byte_array_from_slice|get_array_length|   _object_array_element|get_int_array_region|push_local_frame|pop_local_frame`
    → no match (exit 1)
- `cargo clippy -p iscc-jni --all-targets -- -D warnings` → exit 0 (compiled jni 0.22.4); also ran
    the pre-push `cargo clippy --workspace --all-targets -- -D warnings` → exit 0 (only the
    pre-existing proc-macro-error2 future-incompat warning from iai-callgrind)
- `cargo build -p iscc-jni` then `mvn clean test -f crates/iscc-jni/java/pom.xml` → exit 0, **82
    tests (69 IsccLibTest + 13 UnicodeBoundaryTest), 0 failures** — same totals as the pre-bump tree
    (iter-166 baseline); `git status --porcelain crates/iscc-jni/java` empty.
    `strings target/debug/libiscc_jni.so` shows `jni-0.22.4` / `jni-sys-0.4.1` (fresh artifact, not
    a stale 0.21 build)
- `mise run audit` → exit 0 (advisories/bans/licenses/sources ok)
- `mise run check` → exit 0, 18 hooks Passed, tree unchanged after (only runner-owned
    `iterations.jsonl` dirty beyond the intended files)

**Next:** The dependency-refresh issue's last CID-schedulable item is now done. Remaining refresh
entries are human/release-gated (release.yml action bumps, criterion 0.8 MSRV, uniffi 0.32
Swift-toolchain). Small follow-up candidate observed at 167 and still open: the stale
`RString::from_slice` mention at `crates/iscc-rb/CLAUDE.md:108`.
`.claude/context/specs/java-bindings.md:22` still says "the `jni` crate (v0.21)" — next.md ruled
that spec edit out of scope; review decides.

**Notes:** next.md's per-function sketch (`-> jstring`, `Ok(s.into_raw())`) does not compile:
`EnvOutcome::resolve` requires `T: Default`, and raw pointer aliases (`jstring`/`jobject`) implement
no traits. Deviations, all Java-invisible (every wrapper is `#[repr(transparent)]`, ABI identical):

1. Extern fns return wrapper types (`JString<'local>`, `JObject<'local>`, `JByteArray<'local>`,
    `JObjectArray<'local, _>`) instead of raw `jstring`/`jobject`/... — these implement `Default`
    (null).
2. Array params are element-typed (`JObjectArray<'local, JString<'local>>`, `<JIntArray>`,
    `<JByteArray>`): 0.22 removed `From<JObject> for JString`, and typed `get_element` eliminates
    every `unsafe from_raw` cast in the extractors (the only remaining `unsafe` is the jlong
    opaque-pointer pattern).
3. `jni-sys` 0.4 makes `jboolean = bool` — the `!= 0` conversions are gone (a Java-visible no-op).
4. `find_class`/`throw_new` now take `AsRef<JNIStr>` and `new_object` takes
    `AsRef<MethodSignature>`; `&str` is no longer accepted, so literals use `jni_str!`/`jni_sig!`
    (compile-time encoding; not among next.md's excluded macros) and dynamic messages use
    `JNIString::from(msg)`.
5. `jni::objects::JValue` is deprecated → `jni::JValue`.
6. `byte_array_from_slice` replaced by a `build_byte_array` helper (`JByteArray::new` + `set_region`
    with a per-byte `u8 as i8` copy); the trivial `extract_byte_array` wrapper was dropped in
    favour of direct `env.convert_byte_array(&arr)` calls.
7. Helper extractors now return `jni::errors::Result<T>` instead of `Result<T, String>` (needed by
    `with_local_frame`'s `E: From<Error>` bound); thrown messages are the same `Display` output.
    The hand-rolled "failed to push local frame" message is gone — frame errors now surface as the
    underlying jni error text.
8. `try_to_string(null)` returns `Error::NullPtr` → still `IllegalArgumentException`, matching the
    old `get_string(null)` behaviour; `IllegalStateException` paths covered by the streaming-hasher
    tests in the 82-test run.

No consumer floor moves: jni 0.22.4 declares `rust-version = "1.85"` == workspace MSRV; Java side
untouched (still JDK 17). `packages/kotlin` (UniFFI/JNA) untouched. Core crate untouched — no
baseline refresh (`.crap-baseline.json`/`.iai-baseline.json`) needed or made.
