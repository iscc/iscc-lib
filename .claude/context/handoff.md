## 2026-03-22 — Review of: Fix XCFramework release cache key to include all build inputs

**Verdict:** PASS

**Summary:** The advance agent correctly expanded the XCFramework build cache key in `release.yml`
from 2 globs to 6, adding per-crate Cargo.toml files, root Cargo.toml, build script, and Swift FFI
headers. The change is minimal (2-line diff in the YAML), correctly scoped, and all verification
criteria pass. yamlfix reformatted the multi-line `hashFiles()` into its preferred compact format.

**Verification:**

- [x] `yaml.safe_load` exits 0 — valid YAML confirmed
- [x] `grep -A5 'xcf-cache' | grep 'build_xcframework.sh'` — build script found in cache key
- [x] `grep -A5 'xcf-cache' | grep 'iscc_uniffiFFI'` — Swift headers found in cache key
- [x] `grep -A5 'xcf-cache' | grep 'Cargo.toml'` — both root and per-crate globs present
- [x] `mise run format` — no changes
- [x] `mise run check` — all 15 pre-commit hooks pass

**Issues found:**

- (none)

**Codex review:** No issues found. Codex confirmed the change is syntactically and functionally
consistent with the surrounding release job.

**Next:** Two normal-priority release workflow issues remain: (1) Swift `ref: main` race condition,
(2) root `Package.swift` CI smoke test. The `ref: main` race is more impactful — it can cause
provenance mismatch between tagged source and built binary. However, it's also architecturally
complex (requires rethinking the checksum-commit-back flow). The root `Package.swift` smoke test is
simpler but lower impact. There's also a benchmarks documentation gap (publishing speedup factors).
The define-next agent should weigh complexity vs impact.

**Notes:** XCFramework cache key issue resolved and deleted from issues.md. Remaining: 2 normal + 1
low issues. All are release workflow hardening or cosmetic — no blocking issues for core
functionality.
