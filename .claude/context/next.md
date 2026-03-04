# Next Work Package

## Step: Add missing registry badges to global README

## Goal

Add RubyGems, Maven Central, and npm @iscc/wasm version badges to the global `README.md` badge
section, so all published registries are represented. Resolves the "Ensure all packages have
registry badges in global README" normal-priority issue.

## Scope

- **Create**: (none)
- **Modify**: `README.md`
- **Reference**: `README.md` (existing badge format on lines 5-8)

## Not In Scope

- Reordering or restyling existing badges
- Adding badges for unpublished packages (C#, C++, Swift, Kotlin — those are `low` priority)
- Adding badges to per-crate READMEs (they already have appropriate badges)
- Any changes to CI, release workflows, or documentation site

## Implementation Notes

Add three badges to the existing badge block (lines 5-8), maintaining the same format pattern:

1. **RubyGems** (`iscc-lib`):
    `[![Gem](https://img.shields.io/gem/v/iscc-lib.svg)](https://rubygems.org/gems/iscc-lib)`

2. **Maven Central** (`io.iscc:iscc-lib`):
    `[![Maven Central](https://img.shields.io/maven-central/v/io.iscc/iscc-lib.svg)](https://central.sonatype.com/artifact/io.iscc/iscc-lib)`

3. **npm @iscc/wasm**:
    `[![npm wasm](https://img.shields.io/npm/v/@iscc/wasm.svg)](https://www.npmjs.com/package/@iscc/wasm)`

Place them after the existing Go Reference badge (line 8) to keep a logical grouping: Rust → Python
→ npm lib → Go → Gem → Maven → npm wasm. Or interleave logically — the exact order is flexible as
long as all 7 registry badges are present.

## Verification

- `grep -c 'img.shields.io' README.md` outputs `7` (4 existing + 3 new)
- `grep -q 'img.shields.io/gem/v/iscc-lib' README.md` exits 0
- `grep -q 'img.shields.io/maven-central/v/io.iscc/iscc-lib' README.md` exits 0
- `grep -q 'img.shields.io/npm/v/@iscc/wasm' README.md` exits 0
- `mise run check` passes (formatting hooks don't break)

## Done When

All 7 registry version badges (Rust, Python, npm lib, Go, RubyGems, Maven Central, npm wasm) are
present in the global README.md badge section and all verification commands pass.
