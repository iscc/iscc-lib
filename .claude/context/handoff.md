## 2026-11-22 — Specs and API docs state shipped facts and the shipped error rules

**Done:** The three specs now state what shipped (no `swift` checkbox; doc lists restated as rules
that point to the `zensical.toml` nav, the README Installation section and the `docs/index.md` tab
order; feature flags as they are in `crates/iscc-lib/Cargo.toml`). `encode_component`'s short-digest
rule is documented on the 7 missing surfaces, `c-ffi-api.md`'s streaming warning says a failed first
`_finalize()` consumes, and the README list names Swift. Comments and docs only, no code changes.

**Files changed:**

- `.claude/context/specs/swift-bindings.md`: :397 criterion now says the XCFramework builds only
    when `version` is set and there is no `swift` checkbox
- `.claude/context/specs/documentation.md`: how-to/reference bullets point to the `zensical.toml`
    nav; tab order = README Installation languages in `docs/index.md` Quick Start order; bindings
    criterion = every language in the README Installation section. No language or page lists left
- `.claude/context/specs/c-ffi-dx.md` §5: #16 and "~82K lines" gone; states
    `default =   ["meta-code"]`, which features pull in which crates, and that `serde_json` is not
    optional
- `docs/c-ffi-api.md`: error sentence after the `iscc_encode_component` table; streaming warning
    rewritten like `howto/c-cpp.md`
- `README.md`: Swift added to the "What is iscc-lib" list
- `crates/iscc-ffi/src/lib.rs` (`# Returns`), `crates/iscc-napi/src/lib.rs`,
    `crates/iscc-wasm/src/lib.rs` (JS `Error`), `crates/iscc-uniffi/src/lib.rs` (`IsccUniError`),
    `packages/cpp/include/iscc/iscc.hpp` (`IsccError`, which is what `check_ptr` throws),
    `packages/dotnet/Iscc.Lib/IsccLib.cs` (`<exception cref="IsccException">`, which is what
    `ConsumeNativeString` throws): `encode_component` doc comment names the out-of-range, MainType
    ISCC (5), invalid `bit_length` and digest-shorter-than-`bit_length / 8` cases
- Regenerated: `crates/iscc-ffi/include/iscc.h`, `packages/dotnet/Iscc.Lib/NativeMethods.g.cs`,
    `packages/swift/Sources/IsccLib/iscc_uniffi.swift` and
    `packages/kotlin/src/main/kotlin/uniffi/iscc_uniffi/iscc_uniffi.kt` (doc comment plus the
    `encode_component` checksum, 11010 → 629). The stray `iscc_uniffiFFI.h`/`.modulemap` were
    deleted; the generated FFI header matched the committed one apart from whitespace

**Verification:**

- The three "prints nothing" greps (`#16`, `includes .swift. checkbox`,
    `Python, Rust, Java, Node.js, WASM, C/C++`) print nothing; `check_spec_paths.py`: "OK: every
    tree entry and backticked path in 15 specs resolves", exit 0
- `grep -c 'bit_length / 8\|bitLength / 8'` gives 1 in each of the 7 files
- Freshness: after `touch crates/iscc-ffi/src/lib.rs && cargo build -p iscc-ffi` plus the CI
    cbindgen command, `git diff --exit-code` on `NativeMethods.g.cs` and `iscc.h` gives exit 0
    (staged state compared). The UniFFI step's `run` body, pulled out of `ci.yml` with PyYAML and
    run with `RUNNER_TEMP=/tmp/rt bash -e`, gives exit 0
- `cargo clippy --workspace --exclude iscc-rb --all-targets -- -D warnings` clean;
    `cargo test -p iscc-uniffi -p iscc-ffi`: 121 + 31 passed, 0 failed
- `uv run zensical build` exit 0 ("No issues found"); `mise run check` exit 0 on the third run (runs
    1 and 2 rewrote whitespace in the uniffi output. Kotlin needs two passes: trimming trailing
    spaces leaves a blank line at EOF for end-of-file-fixer)

**Next:** No `normal` blockers left for v0.8.0 in this issue's area. The `[audit]` issue can be
closed, since none of its evidence matches the tree any more.

**Notes:**

- Out of scope, seen while editing: the `bit_length` row in the `c-ffi-api.md` table and the FFI
    `///` say "multiple of 32, 32 to 256" and leave out the ID range (64 to 96 in steps of 8) that
    `rust-api.md` gives. My error sentences say "invalid `bit_length`" and do not repeat a range
- `documentation.md` tab rule: "other pages follow the same order" is my restatement of the old
    "standard tab order" rule. It covers how-to/docs pages that show fewer languages
