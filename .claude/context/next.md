# Next Work Package

## Step: Propagation slice 7 — Unicode boundary vectors in the Swift test suite

## Goal

Gate `packages/swift` on the 12 Unicode 16.0.0 boundary vectors, taking criterion 3 of the `normal`
`[human]` issue **"Declare and gate a Unicode data version (DECIDED)"**, remainder (b), from **10 of
11** binding surfaces to **11 of 11** — the last ungated surface.

Not a bounce (iteration 160 passed review). This step **overturns the "Swift is CI-proof-only"
framing in state.md, handoff.md and `packages/swift/CLAUDE.md`**: swift.org publishes a **Debian
12** toolchain, this container is Debian 12 x86_64, and while scoping I downloaded it, built the
package and ran the existing suite (**9 tests, 0 failures**), then prototyped exactly the code this
step adds in a throwaway `/tmp` copy of the package (**12 tests, 0 failures**) and mutation-probed
it. Swift is locally verifiable — do not claim otherwise, and do not skip the local run.

## Scope

- **Create**:
    - `packages/swift/Tests/IsccLibTests/unicode_boundary.json` — byte-identical vendored copy, made
        with `cp` only (never Write/Edit)
    - `packages/swift/Tests/IsccLibTests/UnicodeBoundaryTests.swift` — the boundary suite
- **Modify** (2 non-test/non-doc files, within the 3-file budget):
    - `packages/swift/Package.swift` — add `.copy("unicode_boundary.json")` to the existing
        `resources:` array of the `IsccLibTests` target (line 27)
    - `.gitignore` — add a `.build/` entry (SwiftPM's default scratch dir is currently untracked-but-
        unignored; `build/` and `build-*/` at lines 11-12 do **not** match it)
    - `tests/test_vendored_fixtures.py` (test) — register the new copy under the
        `crates/iscc-lib/tests/unicode_boundary.json` key in `VENDORED_COPIES`
    - `docs/unicode.md` (doc) — the propagation paragraph (lines ~138-145) must name Swift and its
        vendored copy
    - `packages/swift/CLAUDE.md` (doc) — file layout, test patterns, and the now-false "macOS-only
        testing" / "Cannot run Swift tests in the Linux devcontainer" claims
- **Reference**:
    - `packages/swift/Tests/IsccLibTests/ConformanceTests.swift` — the `Bundle.module` +
        `JSONSerialization` loader shape to mirror
    - `crates/iscc-lib/tests/unicode_boundary.json` — canonical fixture (7 `text_clean` + 5
        `text_collapse` + `_metadata`)
    - `packages/go/unicode_boundary_test.go` — the other vendored-copy surface
    - `tests/test_vendored_fixtures.py` — the byte-identity gate
    - `.github/workflows/ci.yml` lines 236-258 — the macOS `swift` job (needs **no** edit)

## Not In Scope

- **Do not add, edit or reorder any vector in the canonical fixture.** Eleven consumers now assert
    exactly 7 `text_clean` + 5 `text_collapse`; a new row reds them all at once. A new vector is its
    own deliberate slice.
- **Do not touch the root `Package.swift`**, the XCFramework build script, `release.yml`, or
    `ci.yml`. The macOS `swift` job runs `swift test` over the whole test target, so it picks the
    new tests up with no workflow edit (this is how slices 1-6 landed).
- Do not install a Swift toolchain into the image, `mise.toml`, the devcontainer, or CI, and do not
    commit anything from `/tmp`. The toolchain is a local verification aid only.
- Do not regenerate the UniFFI bindings (`Sources/IsccLib/iscc_uniffi.swift`,
    `Sources/iscc_uniffiFFI/*`) — they are generated and unchanged by this step.
- Do not copy a `delete_filter_output` oracle column into the Swift test. Scalar-level equality
    against `outputs.result` already discriminates the delete-filter design (measured below).
- Do not refactor `ConformanceTests.swift` or fold the two suites together.
- Do not touch `.crap-baseline.json` or `.iai-baseline.json` — no Rust or Python source moves here,
    so neither gate can fire.
- Leave the other open `normal` items (the `rubygems/configure-rubygems-credentials@v2.1.0` pin, the
    exhaustive `specs/ci-cd.md` job table, the authorized major dependency bumps) for later steps.

## Implementation Notes

### Getting a working Swift toolchain (measured this iteration)

`swift`/`swiftc` are absent from `$PATH` and there is no PyPI escape hatch, but swift.org ships a
Debian 12 build and this container is Debian 12 x86_64. It may still be unpacked from my scoping run
— check `/tmp/swifttc/swift-6.1.2-RELEASE-debian12/usr/bin/swift --version` first; if the directory
is gone, re-fetch (784 MB, ~5 min, no `sudo` needed, all runtime libs already present):

```bash
mkdir -p /tmp/swifttc
curl -fsSL -o /tmp/swifttc/swift.tar.gz \
    https://download.swift.org/swift-6.1.2-release/debian12/swift-6.1.2-RELEASE/swift-6.1.2-RELEASE-debian12.tar.gz
tar -xzf /tmp/swifttc/swift.tar.gz -C /tmp/swifttc
export PATH=/tmp/swifttc/swift-6.1.2-RELEASE-debian12/usr/bin:$PATH
swift --version   # Swift version 6.1.2 (swift-6.1.2-RELEASE), x86_64-unknown-linux-gnu
```

Then, from the repo root:

```bash
cargo build -p iscc-uniffi                      # ~20 s, leaves no tree diff
cd packages/swift && swift test \
    --scratch-path /tmp/swiftbuild \
    -Xlinker -L/workspace/iscc-lib/target/debug \
    -Xlinker -rpath -Xlinker /workspace/iscc-lib/target/debug
```

**`--scratch-path` is load-bearing**: a bare `swift build`/`swift test` writes
`packages/swift/.build/`, which is not ignored today and would show up as untracked in
`git status --porcelain`. Adding `.build/` to `.gitignore` (in scope) closes that hole permanently;
keep using `--scratch-path` anyway so the tree stays clean either way.

### The one real hazard: Swift `String ==` folds canonical equivalence

**Measured on Swift 6.1.2:** `"e" + U+0301 == U+00E9` → `true`, and `U+1100 U+1161 == U+AC00` →
`true`. `XCTAssertEqual` uses `==`, so a naive string comparison makes **three of the four sequence
vectors vacuous** — exactly the vectors that exist to distinguish the U+FFFF sentinel map from a
delete filter. I proved this in the prototype: with the fixture's
`text_clean/test_0004_seq_u0378_blocks_canonical_composition` expectation mutated to the delete-
filter value U+00E9, string comparison **passed** while scalar comparison **failed**
(`("[101, 769]") is not equal to ("[233]")`).

So compare **Unicode scalar values**, not strings:

```swift
static func scalars(_ text: String) -> [UInt32] {
    return text.unicodeScalars.map { $0.value }
}
```

and assert
`XCTAssertEqual(scalars(actual), scalars(expected), "Failed vector: \(section)/\(name)")`. (UTF-8
byte arrays work equally well; scalars give the more readable failure message.) Put a short comment
on the helper saying why — this is non-obvious and a future editor will otherwise "simplify" it back
to `==`.

### Test file shape (prototyped end-to-end, compiles warning-free, all green)

`packages/swift/Tests/IsccLibTests/UnicodeBoundaryTests.swift`, mirroring `ConformanceTests.swift`:

- `import Foundation` / `import XCTest` / `@testable import IsccLib`;
    `final class UnicodeBoundaryTests: XCTestCase`.
- Three `static let` constants pinning the metadata guard: version `"16.0.0"`, `text_clean` count
    `7`, `text_collapse` count `5`.
- `static var fixture: [String: Any]` from
    `Bundle.module.url(forResource: "unicode_boundary", withExtension: "json")!` — same immediately-
    invoked-closure shape as `ConformanceTests.dataJson`; `Bundle.module` works on Linux and macOS
    once the `.copy(...)` resource entry exists.
- `static func vectors(for section: String) -> [(String, [String: Any])]` sorted by case name (copy
    the `ConformanceTests.vectors(for:)` body).
- Three test methods: `testFixtureMetadata` (version + both counts), `testTextCleanBoundaryVectors`,
    `testTextCollapseBoundaryVectors`. The two vector methods can share one
    `static func assertVectors(section:transform:file:line:)` helper taking `(String) -> String`;
    pass `{ textClean(text: $0) }` / `{ textCollapse(text: $0) }` (the UniFFI free functions take a
    labelled `text:` argument and do **not** throw).
- Zero skips — the Swift bindings wrap the Rust core, so all 12 vectors must pass. Keep the
    `file:`/`line:` forwarding so a failure points at the call site.

### Vendored copy and its gate

1. `cp crates/iscc-lib/tests/unicode_boundary.json packages/swift/Tests/IsccLibTests/` — **`cp`
    only**. The file is pure ASCII with `\uXXXX`-style escapes; a Write/Edit payload can decode
    them to literal UTF-8 and silently break byte-identity. Verify immediately with `cmp`.
2. `Package.swift` line 27 becomes
    `resources: [.copy("data.json"), .copy("unicode_boundary.json")]`.
3. In `tests/test_vendored_fixtures.py`, append
    `"packages/swift/Tests/IsccLibTests/unicode_boundary.json"` to the list under the
    `crates/iscc-lib/tests/unicode_boundary.json` key. The gate discovers copies by **basename**
    via `git ls-files` and set-compares against the table, so an unregistered copy reds
    `test_no_unregistered_tracked_copy`. Do not rename the copy.

### Docs

- `docs/unicode.md`: the paragraph at lines ~138-145 enumerates who consumes the fixture and how.
    Add Swift as a *vendored copy* consumer alongside the pure-Go package (it is not a
    canonical-path reader). If a "10 of 11 / Swift outstanding" style statement exists anywhere on
    the page, it must become complete/11 of 11 — re-derive from disk, do not trust prose.
- `packages/swift/CLAUDE.md`: add `UnicodeBoundaryTests.swift` and `unicode_boundary.json` to the
    file-layout block and a short "Boundary tests" bullet under *Test Patterns* (12 vectors, scalar
    comparison, why). Correct the two false claims — the *CI* section's "Swift is not available on
    Linux CI runners" is about the **runner image** and stays, but "Cannot run Swift tests in the
    Linux devcontainer (no `libiscc_uniffi` available)" and the *macOS-only testing* pitfall are now
    wrong: record the swift.org Debian 12 tarball + `--scratch-path` recipe instead. While editing
    the layout block, drop the stale `SmokeTests.swift` line (that file does not exist) and add the
    existing `Sources/IsccLib/Constants.swift`; do not otherwise rewrite the file.

## Verification

- `cmp crates/iscc-lib/tests/unicode_boundary.json packages/swift/Tests/IsccLibTests/unicode_boundary.json`
    exits 0
- `git ls-files -- '*unicode_boundary.json' | wc -l` → **3** (canonical, `packages/go/testdata/`,
    `packages/swift/Tests/IsccLibTests/`)
- `uv run pytest -q tests/test_vendored_fixtures.py` → **9 passed** (was 8)
- `uv run pytest --collect-only -q` → **400** tests collected (was 399)
- `cargo build -p iscc-uniffi` exits 0 and `git status --porcelain -- crates/` stays empty
- `swift build --scratch-path /tmp/swiftbuild -Xlinker -L$PWD/target/debug` from `packages/swift`
    exits 0 with no compiler warning in the log
- `swift test --scratch-path /tmp/swiftbuild -Xlinker -L<repo>/target/debug -Xlinker -rpath -Xlinker <repo>/target/debug`
    from `packages/swift` prints `Executed 12 tests, with 0 failures` (9 conformance + 3 boundary)
    and exits 0
- `swift package dump-package` at the repo root exits 0 (root manifest untouched)
- `git status --porcelain` shows no `.build/` and no untracked build output anywhere;
    `grep -q '^\.build/$' .gitignore` succeeds
- `grep -c 'unicodeScalars' packages/swift/Tests/IsccLibTests/UnicodeBoundaryTests.swift` ≥ 1 and
    `grep -c 'XCTAssertEqual(actual, expected' packages/swift/Tests/IsccLibTests/UnicodeBoundaryTests.swift`
    → 0 (the canonical-equivalence hole stays closed)
- `grep -c -i 'swift' docs/unicode.md` ≥ **2**
- `uv run scripts/check_docs_nav.py` → `OK: 23 documentation pages consistent`
- `uv run zensical build` exits 0 with "No issues found"
- `mise run check` — all hooks Passed, nothing modified
- `git status --porcelain -- .github/workflows/ .crap-baseline.json .iai-baseline.json crates/ Package.swift`
    → empty

**Recommended extra evidence (leaves no tree diff):** copy the package to `/tmp`
(`cp -r packages/swift /tmp/swiftprobe-pkg`), mutate the copied fixture's
`text_clean/test_0004_seq_u0378_blocks_canonical_composition` expectation to the delete-filter value
U+00E9, and confirm the suite reds there. Never mutate the tracked tree to prove a gate.

## Done When

`swift test` runs the 12 Unicode boundary assertions green from this container with zero skips, the
vendored copy is registered and byte-identical, the docs name Swift as the eleventh gated surface,
and every verification command above passes.
