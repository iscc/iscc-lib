---
icon: lucide/gauge
description: Performance comparison between iscc-lib and the Python reference implementation.
---

# Benchmarks

Performance comparison between **iscc-lib** (Rust) and the
[iscc-core](https://github.com/iscc/iscc-core) Python reference implementation.

---

## Methodology

All benchmarks measure the 9 `gen_*_v0` code generation functions defined by ISO 24138. Two
benchmark suites run with identical inputs to enable direct comparison:

- **Python comparison** — [pytest-benchmark](https://pytest-benchmark.readthedocs.io/) runs both
    `iscc-core` (pure Python) and `iscc-lib` (Rust via PyO3 bindings) in the same process, measuring
    wall-clock time per call.
- **Native Rust** — [Criterion.rs](https://bheisler.github.io/criterion.rs/) measures the pure Rust
    core without PyO3 overhead, providing absolute performance numbers and throughput for
    data-streaming functions.

!!! info "Environment"

    Benchmarks were run in a Linux devcontainer (x86_64). Results vary by hardware — use the
    [reproduction commands](#how-to-reproduce) to measure on your own system.

### Inputs

| Function               | Input                                    |
| ---------------------- | ---------------------------------------- |
| `gen_meta_code_v0`     | Name (25 chars) + description (16 chars) |
| `gen_text_code_v0`     | 1,000-character synthetic text           |
| `gen_image_code_v0`    | 1,024 gradient pixels                    |
| `gen_audio_code_v0`    | 300-element feature vector               |
| `gen_video_code_v0`    | 10 frames × 380 features                 |
| `gen_mixed_code_v0`    | 2 Content-Code strings                   |
| `gen_data_code_v0`     | 64 KB deterministic bytes                |
| `gen_instance_code_v0` | 64 KB deterministic bytes                |
| `gen_iscc_code_v0`     | 4 ISCC unit strings                      |

## Results

### Python comparison (iscc-core vs iscc-lib)

Measured with pytest-benchmark. The speedup factor shows how many times faster the Rust-backed
`iscc-lib` Python package is compared to the pure-Python `iscc-core` reference.

| Function               | iscc-core (Python) | iscc-lib (Rust via Python) |  Speedup |
| ---------------------- | -----------------: | -------------------------: | -------: |
| `gen_meta_code_v0`     |           857.6 µs |                    35.2 µs |  **24×** |
| `gen_text_code_v0`     |         2,757.6 µs |                   184.6 µs |  **15×** |
| `gen_image_code_v0`    |         4,676.8 µs |                   406.8 µs |  **12×** |
| `gen_audio_code_v0`    |         1,195.4 µs |                    53.9 µs |  **22×** |
| `gen_video_code_v0`    |           140.9 µs |                   110.0 µs | **1.3×** |
| `gen_mixed_code_v0`    |            76.2 µs |                     2.8 µs |  **27×** |
| `gen_data_code_v0`     |         7,115.9 µs |                    45.1 µs | **158×** |
| `gen_instance_code_v0` |            67.6 µs |                    17.0 µs |   **4×** |
| `gen_iscc_code_v0`     |            96.8 µs |                     3.8 µs |  **25×** |

### Native Rust (Criterion)

Pure Rust performance without PyO3 binding overhead. Data-streaming functions include throughput
measurements.

| Function               | Input         |     Mean | Throughput |
| ---------------------- | ------------- | -------: | ---------: |
| `gen_meta_code_v0`     | name+desc     |  38.5 µs |          — |
| `gen_text_code_v0`     | 1,000 chars   | 148.8 µs |          — |
| `gen_image_code_v0`    | 1,024 pixels  | 323.1 µs |          — |
| `gen_audio_code_v0`    | 300 features  |  48.1 µs |          — |
| `gen_video_code_v0`    | 10×380 frames |   2.0 µs |          — |
| `gen_mixed_code_v0`    | 2 codes       |   1.8 µs |          — |
| `gen_data_code_v0`     | 64 KB         |  47.2 µs | 1.29 GiB/s |
| `gen_data_code_v0`     | 1 MB          | 745.1 µs | 1.31 GiB/s |
| `gen_instance_code_v0` | 64 KB         |  16.1 µs | 3.80 GiB/s |
| `gen_instance_code_v0` | 1 MB          | 251.5 µs | 3.88 GiB/s |
| `gen_iscc_code_v0`     | 4 units       |   3.1 µs |          — |

## Key findings

- **Speedups range from 1.3× to 158×** across the 9 functions when comparing the Rust-backed Python
    bindings against the pure-Python reference.
- **Data-Code** (`gen_data_code_v0`) shows the largest speedup at **158×**, benefiting from Rust's
    optimized CDC chunking and BLAKE3 hashing (processing 64 KB at 1.29 GiB/s natively).
- **Instance-Code** (`gen_instance_code_v0`) achieves the highest raw throughput at **3.88 GiB/s**
    (BLAKE3-only, no CDC), with a moderate 4× Python-binding speedup because `iscc-core` already
    delegates hashing to a C extension.
- **Video-Code** (`gen_video_code_v0`) shows a modest 1.3× via Python bindings, though the native
    Rust implementation runs at 2.0 µs — the PyO3 overhead of converting nested Python lists to Rust
    vectors dominates at this scale.
- Most functions achieve **12–27× speedups**, with hash-heavy operations (Meta, Text, Audio, Mixed,
    ISCC-CODE) consistently in this range.

## How to reproduce

Build the Rust bindings in release mode and run the comparative benchmark suite:

```bash
# Build release bindings into the project virtualenv
maturin develop --manifest-path crates/iscc-py/Cargo.toml --release

# Run Python comparison benchmarks (iscc-core vs iscc-lib)
pytest benchmarks/python/ --benchmark-only --benchmark-columns=mean,stddev,rounds

# Run native Rust benchmarks (Criterion)
cargo bench -p iscc-lib
```

!!! tip "Interpreting results"

    The Python comparison benchmarks use matching `group` names so pytest-benchmark automatically pairs
    the iscc-core and iscc-lib results for each function. The faster entry (marked `1.0`) is the
    Rust-backed implementation, with the speedup factor shown in parentheses next to the slower entry.
