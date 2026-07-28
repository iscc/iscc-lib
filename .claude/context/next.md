# Next Work Package

## Step: Migrate the JNI binding from jni 0.21 to jni 0.22

## Goal

Land the last CID-schedulable item of the `normal` "Dependency review and refresh across the
project" issue: bump the workspace `jni` pin to 0.22 and rework `crates/iscc-jni/src/lib.rs` onto
the `EnvUnowned`/`Env` API, keeping the Java-visible behaviour — including the thrown exception
classes — unchanged.

## Alternatives Considered

- **Chosen:** the jni 0.22 migration — the only unmet, CID-schedulable target criterion left
    ("third-party dependencies are fresh"); every other backlog entry is `low` or
    human/release-gated.
- **Rejected:** correcting the `RString::from_slice` drift at `crates/iscc-rb/CLAUDE.md:108`
    (observed at 167) — a one-line doc fix, too small to be a step and unrelated to this diff.
- **Not sliced:** the crate cannot compile half-migrated, so "plumbing first, `ErrorPolicy` second"
    is not an honest slice. Budget is 2 non-doc files, well inside the cap.

## Scope

- **Modify**: `Cargo.toml` (pin + delete the `# held: jni` comment block),
    `crates/iscc-jni/src/lib.rs`, `crates/iscc-jni/CLAUDE.md` (docs: lines ~41, 42, 53, 101, 183,
    189 teach `env.get_string()` / `throw_and_default`), `Cargo.lock` (generated).
- **Reference**: upstream `docs/0.22-MIGRATION.md`, shipped inside the crate tarball at
    `https://static.crates.io/crates/jni/jni-0.22.4.crate` (untar it under `/tmp`; the crates.io
    **API** returns 403 here). Java contract:
    `crates/iscc-jni/java/src/test/java/io/iscc/iscc_lib/IsccLibTest.java`.

## Not In Scope

- Adopting the new `native_method!` / `jni_mangle!` / `bind_java_type!` macros — keep the manual
    `#[unsafe(no_mangle)] pub extern "system" fn Java_io_iscc_iscc_1lib_IsccLib_*` exports.
- Changing which exception class any function throws, any Java/Kotlin source, or any test.
- The other three refresh items (`uniffi` 0.32, `criterion` 0.8, `release.yml` action bumps) — their
    `# held:` comments stay exactly as they are.
- `.claude/context/specs/java-bindings.md:22` still says "the `jni` crate (v0.21)" — specs are the
    human-owned target; leave it and let review decide.
- `packages/kotlin` does not use this crate (it is UniFFI/JNA) — do not touch or rebuild it.

## Implementation Notes

`jni` 0.22.4 is latest, `rust-version = "1.85"` — exactly the workspace MSRV, so **no consumer floor
moves** (0.22.0/0.22.1 are yanked; pin `"0.22"`). Only `iscc-jni` depends on `jni`.

Per-function shape (33 `extern "system"` fns; `conformanceSelftest` makes no JNI call, so it only
needs its unused `_env` type renamed). Name `'local` explicitly and wrap the existing body:

```rust
pub extern "system" fn Java_..._genTextCodeV0<'local>(
    mut unowned: EnvUnowned<'local>, _class: JClass<'local>, text: JString<'local>, bits: jint,
) -> jstring {
    unowned.with_env(|env| -> jni::errors::Result<jstring> { /* body, Ok(...) on success */ })
        .resolve::<jni::errors::ThrowRuntimeExAndDefault>()
}
```

**Keep the exception contract by keeping the existing throw helpers.** Change their signature to
return `jni::errors::Result<T>` and `&mut Env`:

```rust
fn throw_and_default<T: Default>(env: &mut Env, msg: &str) -> jni::errors::Result<T> {
    let _ = env.throw_new("java/lang/IllegalArgumentException", msg);
    Ok(T::default())   // exception stays pending; resolve() returns the default as today
}
```

Then every existing `Err(e) => return throw_and_default(&mut env, &e.to_string())` arm survives
verbatim modulo `&mut env` → `env`, and success arms gain an `Ok(...)`. This is deliberate:
`ThrowRuntimeExAndDefault` calls `Env::exception_check()` first and will **not** overwrite a pending
exception, so it only ever fires for a genuine panic/`Err`. `IsccLibTest` asserts
`IllegalArgumentException` and `IllegalStateException` — those must keep coming from our own
`throw_new` calls.

API mapping (each old call is `#[deprecated]` in 0.22 and therefore a hard error under
`-D warnings`):

| 0.21                                    | 0.22                                                             |
| --------------------------------------- | ---------------------------------------------------------------- |
| `env.get_string(&s)?.into()`            | `s.try_to_string(env)?` (`JString::try_to_string`)               |
| `env.get_array_length(&a)`              | `a.len(env)` (returns `usize`)                                   |
| `env.get_object_array_element(&a, i)`   | `a.get_element(env, i)`                                          |
| `env.set_object_array_element(&a,i,v)`  | `a.set_element(env, i, &v)`                                      |
| `env.get_int_array_region(&a,0,&mut b)` | `a.get_region(env, 0, &mut b)` (`JIntArray`)                     |
| `env.byte_array_from_slice(buf)`        | `JByteArray::new(env, buf.len())?` + `set_region(env, 0, &[i8])` |
| `push_local_frame` / `pop_local_frame`  | `env.with_local_frame(cap, \|env\| { … })?` (5 sites)            |

`byte_array_from_slice`'s deprecation note is an upstream copy-paste error (it names
`JObjectArray::set_element`); use the `new` + `set_region` pair, converting `&[u8]` → `Vec<i8>` with
a safe per-byte `as i8` cast collected into a `Vec<i8>` — these buffers are digest-sized.

`new_string`, `new_object_array`, `new_object`, `find_class`, `convert_byte_array` and `throw_new`
are **not** deprecated — leave those call sites alone. `JObjectArray<'local>` still defaults its
element type to `JObject`, so element-type params are optional. No `GlobalRef`/`AutoLocal`/`JavaVM`
usage exists in this crate, so the thread-attachment and `Global<T>` parts of the guide do not
apply.

This step touches no `iscc-lib` core source: **do not refresh `.crap-baseline.json` or
`.iai-baseline.json`.** The JNI `.so` is gitignored and goes stale silently —
`cargo build -p iscc-jni` before running Maven, and use `mvn clean test` (plain `mvn test` reuses
stale classes).

## Verification

- `grep -n 'jni = ' Cargo.toml` shows `jni = "0.22"`, and `grep -n 'held:' Cargo.toml` lists only
    the `criterion` and `uniffi` holds
- `grep -En 'JNIEnv|env\.get_string|byte_array_from_slice|get_array_length|_object_array_element|get_int_array_region|push_local_frame|pop_local_frame' crates/iscc-jni/src/lib.rs`
    returns no match
- `cargo clippy -p iscc-jni --all-targets -- -D warnings` exits 0
- `cargo build -p iscc-jni` then `mvn clean test -f crates/iscc-jni/java/pom.xml` exits 0, with
    `git status --porcelain crates/iscc-jni/java` empty (no Java source or test changed)
- `mise run audit` exits 0
- `mise run check` exits 0 and leaves the tracked tree unchanged

## Done When

`crates/iscc-jni` compiles warning-free against `jni = "0.22"`, the Java conformance suite passes
unchanged on the freshly built native library, and all six checks above pass.
