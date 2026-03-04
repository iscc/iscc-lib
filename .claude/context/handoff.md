## 2026-03-04 — Create Ruby API reference page

**Done:** Created `docs/ruby-api.md` — the Ruby API reference page documenting all 32 Tier 1 public
symbols (5 constants, 10 gen functions, 4 text utilities, 5 encoding/codec functions, 5 algorithm
primitives, 2 streaming hashers, 1 diagnostic function). Added `{ "Ruby API" = "ruby-api.md" }` nav
entry to `zensical.toml` under the Reference section.

**Files changed:**

- `docs/ruby-api.md`: Created — 780-line API reference with method signatures, parameter tables,
    return types, code examples, result classes table, constants table, and error handling section.
    Follows the structural pattern of `docs/java-api.md` adapted for Ruby conventions (keyword
    arguments, module-level methods, Hash subclass results).
- `zensical.toml`: Added `{ "Ruby API" = "ruby-api.md" }` to the Reference nav section after Java
    API.

**Verification:**

- `test -f docs/ruby-api.md` → PASS (file exists)
- `grep -q 'ruby-api.md' zensical.toml` → PASS (nav entry found)
- `grep -c 'gen_.*_code_v0' docs/ruby-api.md` → 46 (≥10, all gen functions documented multiple times
    across signatures, tables, result classes)
- `grep -c 'IsccLib' docs/ruby-api.md` → 78 (≥30, Ruby module referenced throughout)
- `wc -l < docs/ruby-api.md` → 780 (within 400-800 range)
- `uv run zensical build` → "Build finished in 4.42s" (site builds cleanly, `/ruby-api/` appears in
    output)
- `mise run check` → all 15 hooks pass (including mdformat, TOML check)

**Next:** With the Ruby API reference page complete, the Ruby bindings should now meet all spec
requirements (spec line 310). Remaining items: (1) RubyGems account setup (human action), (2) root
README Go quickstart example is stale (uses old WASM-bridge API).

**Notes:** No surprises. The page is 780 lines, near the upper end of the 400-800 target range. This
is due to the Ruby API having keyword arguments on gen functions and result class documentation that
Java doesn't need (Java uses positional args and a single decode result class). The howto guide
error section says `RuntimeError` (not a custom error class) which matches the actual Magnus bridge
implementation.
