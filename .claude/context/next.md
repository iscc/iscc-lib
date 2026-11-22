# Next Work Package

## Step: Specs and API docs state shipped facts and the shipped error rules

## Goal

Close the `[audit]` issue "Specs and API docs still describe shipped work as planned, or leave out
shipped rules", the last `normal` blocker of v0.8.0: three specs say what shipped, and every surface
documents the short-digest rule of `encode_component` and the failed-finalize rule.

## Alternatives Considered

- **Chosen:** the whole issue in one step. Every edit is a spec, a docs page or a `///` /
    `<summary>` comment, plus regenerated outputs. None of those count against the cap (decision
    2026-11-20: comment-only edits and regenerated files), so the reviewer's two-step split is not
    needed. It would cost one more iteration before the release.
- **Rejected:** the docs and comments first and the specs in a later step (the handoff's split). The
    two halves share no risk, and the spec edits are deletions and pointer sentences.

## Scope

- **Modify** (specs/docs, not counted): `.claude/context/specs/swift-bindings.md`,
    `.claude/context/specs/documentation.md`, `.claude/context/specs/c-ffi-dx.md`,
    `docs/c-ffi-api.md`, `README.md`
- **Modify** (comment-only, not counted): `packages/dotnet/Iscc.Lib/IsccLib.cs`,
    `packages/cpp/include/iscc/iscc.hpp`, `crates/iscc-napi/src/lib.rs`,
    `crates/iscc-wasm/src/lib.rs`, `crates/iscc-uniffi/src/lib.rs`, `crates/iscc-ffi/src/lib.rs`
- **Regenerated** (not counted): `crates/iscc-ffi/include/iscc.h`,
    `packages/dotnet/Iscc.Lib/NativeMethods.g.cs`,
    `packages/swift/Sources/IsccLib/iscc_uniffi.swift`,
    `packages/kotlin/src/main/kotlin/uniffi/iscc_uniffi/iscc_uniffi.kt`
- **Reference**: `docs/rust-api.md` (the `encode_component` error sentence, about line 566),
    `docs/howto/c-cpp.md:185-189` (failed-finalize wording), `zensical.toml` nav,
    `.github/workflows/release.yml` inputs, `crates/iscc-lib/Cargo.toml` `[features]`,
    `.github/workflows/ci.yml` (`c-ffi` freshness diff; UniFFI freshness step about line 368)

## Not In Scope

- Code changes of any kind. The rules already hold on every surface. Only the text is missing
- The how-to pages of .NET, C++, JS, Swift and Kotlin, and `docs/index.md`. The issue names
    reference pages and API comments only
- The `low` issues about version-sync file lists, the Python `CLAUDE.md` test list, or the
    failed-finalize tests on C FFI/C++/.NET/UniFFI. They stay `low`
- Copying a list of pages, languages or bindings into a spec. Point to the nav or the README instead

## Implementation Notes

**Specs:**

- `swift-bindings.md:397`: the criterion "Release workflow includes `swift` checkbox" contradicts
    :265-267 and `release.yml`. Replace it with the shipped fact: the XCFramework is built only when
    the `version` input is set, and there is no `swift` checkbox.
- `documentation.md`: the how-to list (:50), the reference list (:52), the tab order (:61) and the
    "Available Bindings" criterion (:94-95) copy stale lists. Restate them as rules. One how-to
    guide and the Tier 1 reference pages per language, as in the `zensical.toml` nav. Tabs cover the
    languages the README installs, in the order `docs/index.md` uses (read it, do not invent one).
    The bindings table lists every language in the README's Installation section. No language or
    page lists.
- `c-ffi-dx.md` §5 (:134-138): GitHub #16 is closed. State what ships. `default = ["meta-code"]`;
    `text-processing` brings in the Unicode crates; `meta-code` adds `serde_json_canonicalizer`;
    `serde_json` is not optional. Read `crates/iscc-lib/Cargo.toml` for the exact facts. Drop the
    "~82K lines" figure unless you measure it.
- Run `uv run python scripts/check_spec_paths.py`. Spec paths are gated against `git ls-files`.

**Docs and comments:**

- `docs/c-ffi-api.md:742-744` warning: add that the first `_finalize()` consumes the state even when
    it fails (e.g. invalid `bits`), worded as in `docs/howto/c-cpp.md`.
- `encode_component` error contract: add a sentence adapted from `rust-api.md` to `c-ffi-api.md`
    (after the parameter table: returns `NULL` and sets the last error), the FFI `///` (the
    `# Returns` section), `IsccLib.cs` `<summary>`/`<exception>`, `iscc.hpp`, napi, WASM and UniFFI.
    Use each surface's error idiom: NULL plus last error / `IsccException` / `iscc::IsccError` or
    whatever `check_ptr` throws (read it) / JS `Error` / `IsccUniError`. Every surface must name the
    case where the digest is shorter than `bit_length / 8` bytes, and may also list the other
    invalid inputs.
- `README.md` "What is iscc-lib" (:49-51): add Swift to the language list.
- Regeneration: a `///` edit on an `iscc-ffi` export moves `iscc.h` **and** `NativeMethods.g.cs`
    (`cargo build -p iscc-ffi`, then cbindgen as `ci.yml`'s `c-ffi` job does; commit both). A `///`
    edit on a UniFFI export moves its checksum in both the Swift and the Kotlin bindings. Regenerate
    with the `ci.yml` UniFFI step's `gen=` command and delete any stray `.modulemap`. uniffi output
    is not hook-clean, so compare modulo trailing whitespace, as the CI step does. napi's
    `index.d.ts` is not tracked.

## Verification

- `grep -n '#16' .claude/context/specs/c-ffi-dx.md`,
    `grep -n 'includes .swift. checkbox' .claude/context/specs/swift-bindings.md` and
    `grep -n 'Python, Rust, Java, Node.js, WASM, C/C++' .claude/context/specs/documentation.md` all
    print nothing; `uv run python scripts/check_spec_paths.py` exits 0
- The `encode_component` doc block in each of the 7 files (`docs/c-ffi-api.md`, `IsccLib.cs`,
    `iscc.hpp`, the napi, WASM, UniFFI and FFI `lib.rs`) states the short-digest rejection
    (`bit_length / 8` or `bitLength / 8`); `docs/c-ffi-api.md`'s streaming warning says a failed
    first `_finalize()` consumes; `README.md`'s "What is iscc-lib" list names Swift
- Generated outputs are fresh: the `c-ffi` job's header and `NativeMethods.g.cs` diff, and the
    UniFFI freshness step body extracted from `ci.yml` and run with `bash -e`, both exit 0
- `cargo clippy --workspace --exclude iscc-rb --all-targets -- -D warnings` clean and
    `cargo test -p iscc-uniffi -p iscc-ffi` passes
- `uv run zensical build` exits 0 and `mise run check` exits 0

## Done When

All verification criteria pass with no behaviour change on any surface, and the audit issue's
evidence list no longer matches the tree.
