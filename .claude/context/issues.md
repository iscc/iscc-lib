# Issues

Tracked issues for the CID workflow. Priorities: `critical` (next iteration), `normal` (weighed
against gaps), `low` (when nothing else remains). Source tags: `[human]`, `[review]`. Optional
fields: `**Spec:**` (spec gap link), `**Upstream:**` (external repo). The review agent deletes
resolved issues after verification (history in git).

<!-- Add issues below this line -->

## #16 — Add feature flags for embedded/minimal builds [human]

**Priority:** normal **GitHub:** https://github.com/iscc/iscc-lib/issues/16

Add Cargo feature flags (`meta-code`, `text-processing`) so embedded consumers can opt out of heavy
dependencies (~82K source lines for serde/unicode). Default behavior unchanged (all features on).
`conformance_selftest()` must adapt to available features (skip disabled code types). CI must test
`--all-features`, `--no-default-features`, and each feature individually.

## #22 — Add C/C++ how-to guide [human]

**Priority:** normal **GitHub:** https://github.com/iscc/iscc-lib/issues/22 **Spec:**
`.claude/context/specs/c-ffi-dx.md` §1

Create `docs/howto/c-cpp.md` following the same structure as existing per-language guides. Must
include: CMake integration snippet, streaming DataHasher/InstanceHasher walkthrough, ISCC-SUM
one-shot example, error handling and memory management sections, C++ RAII wrapper example, static vs
dynamic linking guidance, and cross-compilation notes. Add to site navigation.

## #25 — Add pre-built FFI release artifacts to GitHub Releases [human]

**Priority:** normal **GitHub:** https://github.com/iscc/iscc-lib/issues/25 **Spec:**
`.claude/context/specs/c-ffi-dx.md` §4

Add `build-ffi` and `publish-ffi` jobs to `release.yml` with a 5-platform matrix (matching JNI/napi
targets). Each platform produces a tarball containing shared lib + static lib + `iscc.h` + LICENSE.
Add `ffi` boolean input to `workflow_dispatch`. Tarballs uploaded as GitHub Release assets with
naming `iscc-ffi-v{version}-{target}.tar.gz`.

## #21 — Add units support to gen_sum_code_v0 [human]

**Priority:** normal **GitHub:** https://github.com/iscc/iscc-lib/issues/21

`gen_sum_code_v0` already computes `data_result.iscc` and `instance_result.iscc` internally but
discards them. Add an `units` field (`Option<Vec<String>>`) to `SumCodeResult` containing the
individual Data-Code and Instance-Code ISCC strings at full `bits` precision. Gate via an
`add_units: bool` parameter. This lets `iscc-sdk` get everything from a single optimized call
instead of making separate `gen_data_code_v0` + `gen_instance_code_v0` calls (two extra file reads).
Update all bindings (Python, Node.js, WASM, JNI, C FFI) to expose the new parameter and field.
