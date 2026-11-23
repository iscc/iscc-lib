---
name: api-design
description: Advance-agent API design decisions — video ragged input, Python int range, Go meta JCS, alg_simhash empty input, json_to_data_url
metadata:
  type: project
---

# API design decisions (moved from MEMORY.md, iter 277)

- Video ragged input (250): `soft_hash_video_v0` / Go `SoftHashVideoV0` reject unequal frame-sig
    lengths via private `check_frame_sig_lengths` (deliberate iscc-core deviation: ref truncates)
- Python int range (265): out-of-range ints → `OverflowError` (decision 2026-11-21); video
    `extract_frame_sigs` uses `PyLong_AsLongLong` (C `long` is 32-bit on Windows) +
    `PyOverflowError`. Pinned in `tests/test_int_range.py`; IDv1 `checked_id_arg` stays `ValueError`
- Go meta JCS (251, 266): `parseMetaJSON` writes canonical JSON itself (`writeCanonical*`): keys
    sorted by UTF-16 units (`sortedKeysUTF16`), float `-0` → `0`, scalars via Encoder
    `SetEscapeHTML(false)` + `unescapeLineSeparators`; `checkSurrogateEscapes` rejects lone
    surrogates (runs after Unmarshal). Never `json.Marshal`. Lone surrogates cannot be a vector
    (serde_json rejects the fixture). Write non-ASCII in Go tests as JSON `\uXXXX` escapes inside
    raw strings; `grep -nP '[^\x00-\x7f]'` the test file. Probe Rust via `il.json_to_data_url`
- `alg_simhash` rejects `[]` (225, core + Go); generators keep 32 zeros via Rust `alg_simhash_inner`
    / Go unexported `algSimhash` — never call the public form from a generator
- `json_to_data_url` combines `parse_meta_json` + `build_meta_data_url`. JCS canonical, media type
    depends on `@context` key. 5 cross-binding constants → MEMORY-archive.md
