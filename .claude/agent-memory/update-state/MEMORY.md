# Update-State Agent Memory

Codepaths, patterns, and key findings accumulated across CID iterations.

## Exploration Shortcuts

- **Java files**: `find crates/iscc-jni -type f | sort` — lists all JNI bridge files
- **Per-crate READMEs**: `find crates/ -name "README.md"` — currently returns nothing (none exist)
- **CI jobs in a run**:
    `gh run view <id> --repo iscc/iscc-lib --json jobs --jq '.jobs[] | {name, conclusion}'`
- **Latest CI runs**:
    `gh run list --repo iscc/iscc-lib --branch main --limit 3 --json status,conclusion,url,workflowName,databaseId`
- **Java native method count**:
    `grep -c 'native ' crates/iscc-jni/java/src/main/java/io/iscc/iscc_lib/IsccLib.java`
- **Incremental diff**: `git diff <assessed-at-hash>..HEAD --stat`

## Codebase Landmarks

- `crates/iscc-jni/src/lib.rs` — 763-line Rust JNI bridge, 29 `extern "system"` functions
- `crates/iscc-jni/java/src/main/java/io/iscc/iscc_lib/IsccLib.java` — 331-line Java wrapper, 29
    native methods
- `crates/iscc-jni/java/pom.xml` — Maven build config, JDK 17, JUnit 5 + Gson, Surefire 3.5.2 with
    `java.library.path=target/debug`
- `crates/iscc-jni/java/src/test/java/io/iscc/iscc_lib/IsccLibTest.java` — 338-line JUnit 5 test
    class, 9 `@TestFactory` methods, 46 conformance vectors (CI-verified at HEAD)
- `.devcontainer/Dockerfile` — includes `openjdk-17-jdk-headless` and `maven`
- `.github/workflows/ci.yml` — 6 jobs: Rust, Python, Node.js, WASM, C FFI, Java (no Go yet)
- `crates/` — 6 crates: iscc-lib, iscc-py, iscc-napi, iscc-wasm, iscc-ffi, iscc-jni
- `docs/howto/` — 4 files: rust.md, python.md, nodejs.md, wasm.md (no java.md or go.md yet)

## Recurring Patterns

- **Incremental review**: compare `assessed-at` hash vs HEAD `--stat` first, then re-verify only
    affected sections. Always carry forward sections where no relevant files changed.
- **Tier 1 symbol count**: target says "22" but implementation has 23 (target.md counting error)
- **CI now has 6 jobs**: Rust, Python, Node.js, WASM, C FFI, Java. All 6 pass at HEAD. Go job
    pending.
- **Java `target/` directory**: Maven compile output in `crates/iscc-jni/java/target/` — covered by
    root `.gitignore`'s `target/` pattern, not committed

## Gotchas

- `packages/go/` does not exist — Go bindings are not started (new target section as of 0a10f73)
- No per-crate `README.md` files exist anywhere — new target section added but work not started
- The `state.md` section order must include both Go Bindings and Per-Crate READMEs sections (added
    to target in commit `0a10f73`)
- `gh run list` needs `--repo iscc/iscc-lib` to avoid GraphQL projects error; also needs `--json`
    fields
