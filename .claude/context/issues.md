# Issues

Tracked issues for the CID workflow. Priorities: `critical` (next iteration), `normal` (weighed
against gaps), `low` (when nothing else remains). Source tags: `[human]`, `[review]`. Optional
fields: `**Spec:**` (spec gap link), `**Upstream:**` (external repo). The review agent deletes
resolved issues after verification (history in git).

<!-- Add issues below this line -->

## Update reference to iscc-core v1.3.0 `critical` [human]

The upstream `iscc/iscc-core` Python reference has released v1.3.0 (tag exists, our shallow clone is
pinned at v1.2.2). The update includes new conformance test vectors and functional changes that
iscc-lib must track.

**What changed (v1.2.2 → v1.3.0):**

1. **New conformance vectors in `data.json`** (4 new Meta-Code tests):

    - `test_0017_meta_jcs_float_as_integer` — JCS number canonicalization (float `1.0` → integer `1`)
    - `test_0018_meta_jcs_large_float` — JCS large float canonicalization (`1e+20` → integer string)
    - `test_0019_description_trim` — description trimming at 4096-byte boundary (ASCII)
    - `test_0020_description_trim_i18n` — description trimming at 4096-byte boundary (multi-byte
        UTF-8)

2. **`META_TRIM_META` size limit** (`options.py`): new `meta_trim_meta = 128_000` option. The
    `gen_meta_code_v0` function now raises `ValueError` if decoded meta payload exceeds this limit.
    Added to `conformance_critical` set.

3. **`data.json` metadata header**: new top-level `_metadata` object with `generated`, `generator`,
    and `description` fields. Our vector loader must tolerate or skip this entry.

4. **Codec validation tightening** (`codec.py`): `iscc_validate` now checks `(MainType, Version)`
    combinations against `SUBTYPE_MAP` instead of just checking `v in (0, 1)`.

5. **`iscc_nph_compare`** — new high-level comparison function (not conformance-critical for us
    yet).

**Action items:**

- Update `reference/iscc-core` shallow clone to v1.3.0
- Vendor updated `data.json` test vectors
- Ensure vector loader handles the new `_metadata` key
- Implement `META_TRIM_META` limit in Meta-Code generation
- Verify JCS number canonicalization and description trimming produce matching output
- Update codec validation if `iscc_validate` is implemented

## Add programming language logos to README and docs `low` [human]

Add logos/icons for the supported programming languages (Rust, Python, etc.) to the README and
documentation pages where appropriate. Visual language indicators help users quickly identify
binding availability and make the project more approachable.
