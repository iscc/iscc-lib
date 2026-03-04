# Next Work Package

## Step: Create Ruby API reference page

## Goal

Create `docs/ruby-api.md` — the Ruby API reference page listing all 32 Tier 1 public methods with
signatures, parameter tables, return types, and usage examples. This is the last item required
before Ruby bindings reach "met" status (spec line 310).

## Scope

- **Create**: `docs/ruby-api.md`
- **Modify**: `zensical.toml` (add `{ "Ruby API" = "ruby-api.md" }` to Reference nav section)
- **Reference**:
    - `docs/java-api.md` — primary structural template (677 lines, most comprehensive)
    - `docs/rust-api.md` — concise style reference (377 lines)
    - `docs/c-ffi-api.md` — additional pattern reference (745 lines)
    - `docs/howto/ruby.md` — existing Ruby content, examples, and API surface (422 lines)
    - `crates/iscc-rb/lib/iscc_lib.rb` — actual Ruby wrapper with method signatures and result classes

## Not In Scope

- Updating `state.md` — the update-state agent handles that
- RubyGems account setup or publishing — human action item
- Fixing the stale Go quickstart example in root README — separate future step
- Expanding the howto guide — `docs/howto/ruby.md` is already complete (422 lines)
- Adding Ruby examples to multi-language tabbed sections in other doc pages

## Implementation Notes

Follow the structural pattern of `docs/java-api.md` (the most complete reference page):

1. **Front matter**: YAML header with `icon: lucide/book-open` and description

2. **Introduction**: 2-3 sentences about the Ruby gem, require/install snippet

3. **Sections** (in this order):

    - **Constants** — table with name, type, value, description for all 5 constants
        (`META_TRIM_NAME`, `META_TRIM_DESCRIPTION`, `META_TRIM_META`, `IO_READ_SIZE`,
        `TEXT_NGRAM_SIZE`)
    - **Result Classes** — brief table of the 10 `*CodeResult` classes (subclass of `Hash`),
        explaining dual access (`result.iscc` and `result["iscc"]`)
    - **Code Generation Functions** — all 10 `gen_*_v0` functions, each with:
        - Ruby method signature (keyword args with defaults)
        - Parameter table (Parameter | Type | Description)
        - 1-2 sentence description of the algorithm
        - Short code example
        - `---` separator between functions
    - **Text Utilities** — `text_clean`, `text_remove_newlines`, `text_trim`, `text_collapse`
    - **Encoding & Codec** — `encode_base64`, `json_to_data_url`, `encode_component`,
        `iscc_decompose`, `iscc_decode`
    - **Algorithm Primitives** — `sliding_window`, `alg_simhash`, `alg_minhash_256`,
        `alg_cdc_chunks`, `soft_hash_video_v0`
    - **Streaming Hashers** — `DataHasher` and `InstanceHasher` with lifecycle examples (new →
        push/update → finalize)
    - **Diagnostics** — `conformance_selftest`
    - **Error Handling** — `IsccLib::Error` exception pattern

4. **Ruby-specific API details** to get right:

    - All functions called as `IsccLib.function_name(...)` (module-level)
    - Keyword arguments with defaults: `bits: 64`, `add_units: false`, `wide: false`
    - Result objects: `Hash` subclass with attribute-style access (`result.iscc`)
    - Binary data: Ruby strings with `.b` encoding
    - Streaming: `DataHasher.new` / `InstanceHasher.new`, method chaining via `self` return
    - Constants: `IsccLib::CONSTANT_NAME`

5. **Do NOT duplicate howto guide content verbatim** — the API reference is terse and technical
    (signatures + tables + minimal examples). The howto guide has narrative explanations.

6. Target length: ~500-700 lines (between rust-api.md's 377 and c-ffi-api.md's 745).

## Verification

- `test -f docs/ruby-api.md` exits 0 (file exists)
- `grep -q 'ruby-api.md' zensical.toml` exits 0 (nav entry added)
- `grep -c 'gen_.*_code_v0' docs/ruby-api.md` returns 10 (all gen functions documented)
- `grep -c 'IsccLib' docs/ruby-api.md` returns ≥30 (Ruby module referenced throughout)
- `uv run zensical build 2>&1 | tail -1` contains "Documentation built" (site builds cleanly)
- `wc -l < docs/ruby-api.md` returns between 400 and 800 (appropriate reference page size)

## Done When

All verification criteria pass — `docs/ruby-api.md` exists with all 32 Tier 1 symbols documented,
navigation entry is in `zensical.toml`, and the documentation site builds successfully.
