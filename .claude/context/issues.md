# Issues

Tracked issues for the CID workflow. Priorities: `critical` (next iteration), `normal` (weighed
against gaps), `low` (human-directed only — CID loop must skip these). Source tags: `[human]`,
`[review]`. Optional fields: `**Spec:**` (spec gap link), `**Upstream:**` (external repo). The
review agent deletes resolved issues after verification (history in git).

<!-- Add issues below this line -->

## Implement C# / .NET bindings via csbindgen `normal` [human]

Add C# / .NET bindings as a new package in `packages/dotnet/` using `csbindgen` to generate P/Invoke
wrappers from the existing `iscc-ffi` C API. Published to NuGet as `Iscc.Lib`.

**Implementation scope:**

1. **Package setup** (`packages/dotnet/`):

    - .NET class library project (`.csproj`) targeting .NET 8+
    - `csbindgen`-generated P/Invoke bindings from `iscc.h`
    - Idiomatic C# wrapper with PascalCase API, exceptions, Stream support
    - NuGet package spec (SDK-style packaging)
    - Platform-specific native libraries as RID-specific runtime assets
    - Conformance tests via xUnit against `data.json`
    - `README.md` for nuget.org

2. **DevContainer**: Add .NET SDK 8+ to Dockerfile

3. **CI** (`ci.yml`): Add `dotnet` job — `dotnet build` + `dotnet test`

4. **Release** (`release.yml`):

    - Add `nuget` boolean input to `workflow_dispatch`
    - Build and pack NuGet package with native libraries for 5 platforms
    - Publish via `dotnet nuget push` (NuGet API key or OIDC)
    - Idempotency: check version on nuget.org before publishing

5. **Version sync**: Add .NET project version to sync targets

6. **Documentation**: `docs/howto/dotnet.md` how-to guide, update README with C# install/quickstart

7. **Account setup** (manual, human action):

    - Register/verify nuget.org account
    - Reserve `Iscc.Lib` package name
    - Configure API key or OIDC trusted publisher

## Implement C++ idiomatic header-only wrapper `low` [human]

Add an idiomatic C++17 header-only wrapper (`iscc.hpp`) over the existing C FFI, with RAII resource
management and distribution via vcpkg, Conan, and FFI release tarballs.

**Implementation scope:**

1. **Package setup** (`packages/cpp/`):

    - `include/iscc/iscc.hpp` — header-only C++ wrapper with RAII, std types, exceptions
    - `CMakeLists.txt` — CMake config with `find_package(iscc)` support
    - `vcpkg.json` + `portfile.cmake` — vcpkg port manifest
    - `conanfile.py` — Conan recipe
    - `tests/` — C++ conformance tests against `data.json`
    - `README.md` for the package

2. **CI** (`ci.yml`): Add `cpp` job — compile C++ test program, run conformance tests, ASAN check

3. **Release** (`release.yml`): Bundle `iscc.hpp` in existing FFI release tarballs alongside
    `iscc.h`

4. **Documentation**: Update `docs/howto/c-cpp.md` with C++ wrapper examples, RAII patterns

## Implement Swift bindings via UniFFI `low` [human]

Add Swift bindings as a Swift Package in `packages/swift/` using UniFFI-generated code. Requires a
shared UniFFI scaffolding crate (`crates/iscc-uniffi/`) that also serves Kotlin bindings.

**Implementation scope:**

1. **UniFFI crate** (`crates/iscc-uniffi/`):

    - `Cargo.toml` (cdylib, depends on `iscc-lib` + `uniffi`)
    - `src/lib.rs` — UniFFI interface definition (proc macros or UDL) exposing all 32 Tier 1 symbols
    - `uniffi.toml` — binding generation config

2. **Swift package** (`packages/swift/`):

    - `Package.swift` — SPM manifest
    - Generated Swift bindings via `uniffi-bindgen`
    - Conformance tests via XCTest against `data.json`
    - `README.md` for the package

3. **CI** (`ci.yml`): Add `swift` job — `swift build` + `swift test` (may require macOS runner)

4. **Release**: Publish via Git tags (SPM resolves from Git repos, no upload registry)

5. **Documentation**: `docs/howto/swift.md` how-to guide, update README with Swift
    install/quickstart

## Implement Kotlin Multiplatform bindings via UniFFI `low` [human]

Add Kotlin Multiplatform (KMP) bindings in `packages/kotlin/` using UniFFI-generated code from the
shared `crates/iscc-uniffi/` crate. Published to Maven Central as `io.iscc:iscc-lib-kotlin`.

**Depends on:** Swift bindings (shares the UniFFI scaffolding crate)

**Implementation scope:**

1. **Kotlin package** (`packages/kotlin/`):

    - `build.gradle.kts` — KMP project targeting JVM + iOS + macOS
    - Generated Kotlin bindings via `uniffi-bindgen`
    - Platform-specific native libraries per target
    - Conformance tests via kotlin.test against `data.json`
    - `README.md` for the package

2. **CI** (`ci.yml`): Add `kotlin` job — Gradle build + test

3. **Release** (`release.yml`):

    - Add `maven-kotlin` boolean input to `workflow_dispatch`
    - Publish to Maven Central as `io.iscc:iscc-lib-kotlin`
    - GPG signing + Sonatype credentials (same as Java/JNI)

4. **Version sync**: Add Kotlin project version to sync targets

5. **Documentation**: `docs/howto/kotlin.md` how-to guide, update README with Kotlin
    install/quickstart

## Add programming language logos to README and docs `low` [human]

Add logos/icons for the supported programming languages (Rust, Python, etc.) to the README and
documentation pages where appropriate. Visual language indicators help users quickly identify
binding availability and make the project more approachable.
