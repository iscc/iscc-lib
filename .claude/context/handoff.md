## 2026-07-29 — Review of: Mint `gen_iscc_id_v1` on the C FFI surface

**Verdict:** PASS

**Summary:** The advance agent exported `iscc_gen_iscc_id_v1(u64, u16, u8) -> *mut c_char` from
`crates/iscc-ffi`, regenerated the committed `iscc.h` via cbindgen, and added golden + realm-error C
tests. Diff is scope-clean (1 source file, 1 generated header, 1 test file), the body mirrors the
existing gens exactly, all gates green, and Codex found nothing. First of the 5 remaining typed-int
#43 fan-out surfaces; the header-freshness path is now exercised.

**Verification:**

- [x] `cargo build -p iscc-ffi` — clean; `cargo clippy -p iscc-ffi --all-targets -- -D warnings` —
    clean; `cargo fmt -p iscc-ffi --check` — clean.
- [x] cbindgen regen idempotent — `git diff --exit-code iscc.h` empty after re-running the CI
    command; `iscc_gen_iscc_id_v1(uint64_t, uint16_t, uint8_t)` at iscc.h:386.
- [x] `gcc … test_iscc.c -I include -L target/debug -liscc_ffi …` compiles;
    `LD_LIBRARY_PATH=target/debug /tmp/test_iscc` → **83 passed, 0 failed**, incl. golden + realm=2
    NULL + last_error non-NULL.
- [x] `mise run check` — all prek pre-commit hooks Passed.
- [x] (probe) Golden `ISCC:MAIGHFECJMOPMIAB` for `(1751831876325218, 1, 0)` confirmed byte-identical
    against `iscc-core` oracle.

**Issues found:**

- (none) Core signature, `.map(|r| r.iscc)` projection, and no-`checked()`/no-NULL-guard rationale
    all correct — core `gen_iscc_id_v1` re-validates all three ranges in order and
    `result_to_c_string` maps the error to NULL + last_error.

**Codex review:** Clean — confirmed the C ABI function delegates to the validated core impl, follows
existing error/memory conventions, and is reflected in the generated header. No findings.

**Next:** #43 fan-out continues on the next typed-int surface — **jni IDv1 minting** (returns a
`#[repr(transparent)]` wrapper per decisions.md 2026-07-28; no `checked()` — core re-checks), then
rb, uniffi→Swift/Kotlin, dotnet (regenerate `NativeMethods.g.cs`), cpp. One surface per step. After
the fan-out, the Tier-1 32→33 doc/count sweep across the stale sites named in #43.

**Notes:** `iscc.h` +21 lines is the committed delta, not drift (regen is a no-op at HEAD). **Review
correction:** next.md/advance deferred `NativeMethods.g.cs` to "the dotnet step", but `iscc-ffi`'s
csbindgen `build.rs` regenerates that tracked file on EVERY build — the pre-push clippy hook rebuilt
it and rejected the push ("files were modified"). I regenerated + committed the +22-line P/Invoke
decl (idempotent; `dotnet build` clean, 0 warnings) as a mechanical fix; the C# consumer + golden
test are still owed in the dotnet step. Any future FFI-symbol step must regen `NativeMethods.g.cs`
in-step, like `iscc.h`. learnings.md 199; decisions.md 374, runner rotates.
