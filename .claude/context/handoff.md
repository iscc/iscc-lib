## 2026-11-21 — Review of: Make the published-package docs match what 0.7.0 ships

**Verdict:** PASS_WITH_NOTES

**Summary:** The nine scoped Markdown files now describe the published 0.7.0 artifacts and the code
as it runs, and the `normal` `[audit]` issue "Published-package docs contradict what ships" is
closed and deleted. One sentence the step added was false (the Java `java.library.path` fallback "on
other platforms"); the review corrected it in three places. No code, workflow or version changed.

**Verification:**

- [x] The stale-phrase grep over the 9 files prints nothing — exit 1, no output
- [x] `grep -n "result.iscc" README.md` prints no JavaScript line, and the WASM block has
    `await init()` — lines 172/229/249/260/274 are Rust, Ruby, C++, Swift and Kotlin;
    `await init();` is at line 284
- [x] `uv run zensical build` exits 0 — "No issues found"; `gen_llms_full.py` re-run after it (22
    pages)
- [x] `uv run pytest tests/` passes — 769 passed, docs gates included
- [x] `mise run check` exits 0
- [x] The advance diff lists only `.md` files

Probes on the published artifacts (beyond next.md's list):

- Java: the Maven Central JAR `iscc-lib-0.7.0.jar` holds five `META-INF/native/{os}-{arch}/`
    libraries. A three-line program compiled against it ran `genMetaCodeV0` and
    `conformanceSelftest` on Linux x86_64 with no library path set
- Swift: `Package.swift` at tag `v0.7.0` fetches `IsccLib.xcframework.zip`, and the SHA-256 of the
    release asset equals `releaseChecksum`
- Java fallback: with a working library on `java.library.path`, `-Dos.arch=riscv64` fails with
    "Unsupported architecture" and `-Dos.name=FreeBSD` with "Unsupported OS". A recognized pair
    without a bundled library (Windows on aarch64) loads it
- Kotlin: the published POM declares JNA with `<scope>runtime</scope>`, and the JAR holds Linux and
    macOS libraries for both architectures and Windows x86-64

**Issues found:**

- **Fixed in review:** `crates/iscc-jni/README.md` and `docs/howto/java.md` said a user on a
    platform the JAR does not cover can put their own build on `java.library.path`. The loader
    throws on an unknown OS or architecture before it tries that path. Both files now state the
    limit
- **Filed `low`, untriaged:** "JNI `NativeLoader` rejects an unknown OS or architecture before the
    `java.library.path` fallback" (the code behind the corrected sentence; pre-existing)
- **Filed `low`, untriaged:** "`docs/architecture.md` sections below the graph cover half the
    surfaces" ("Workspace Layout", "Crate Summary" with `iscc-ffi` → "Source", and a test matrix
    that names `cargo test` as the WASM runner; pre-existing, not scoped by next.md)
- Not filed: `docs/development.md` now states the pytest timeout ("120 s per-test timeout"), a
    number copied by hand from `.pre-commit-config.yaml`. Correct today
- Not filed: the Kotlin docs do not mention the Android libraries the JAR carries. Loading them
    through JNA on Android is unverified, so no claim is the right choice

**Codex review:** One finding (P2), the same Java fallback claim as above: `detectOs()` and
`detectArch()` throw before `System.loadLibrary`. Already corrected in this review. Nothing else.

**Next:** No `critical` or `normal` issue is open, so nothing is left for CID to pick up: the next
define-next should write `## Step: NONE`. Every open entry is `low`, and each is either
human-directed or awaits triage by Titusz. The last v0.8.0 blocker is closed; cutting v0.8.0 is a
human release through the `/release` skill.

**Notes:**

- next.md called `java.library.path` an "optional override". It is not: the bundled library is
    loaded first, and the path is read only when the JAR has no library for a recognized platform.
    The advance agent did not copy that wording
- The Java how-to tip "Build from source" now says "for development on this repository", like the
    Kotlin tip
- Review edits were prose in two `.md` files, formatted with mdformat. No code or config was
    touched, so the full check was not re-run
- `issues.md` is at 298 of 300 lines and `learnings.md` at 200 of 200: the next entry in either
    needs an equal cut
