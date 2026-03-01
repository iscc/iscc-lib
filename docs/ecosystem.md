---
icon: lucide/globe
description: Official and community implementations of the ISCC standard (ISO 24138:2024).
---

# Ecosystem

The [ISCC](https://iscc.codes) (International Standard Content Code) is an open standard
([ISO 24138:2024](https://www.iso.org/standard/77899.html)) for content-derived identification of
digital media assets. Community implementations help broaden adoption across languages and
platforms.

---

## Official Implementations

These implementations are maintained by the [ISCC Foundation](https://iscc.io).

### iscc-core — Python Reference

The canonical reference implementation of ISO 24138. All other implementations validate correctness
against its conformance test vectors (`data.json`).

|            |                                                                 |
| ---------- | --------------------------------------------------------------- |
| Repository | [iscc/iscc-core](https://github.com/iscc/iscc-core)             |
| Package    | [iscc-core on PyPI](https://pypi.org/project/iscc-core/)        |
| Language   | Python                                                          |
| License    | Apache-2.0                                                      |
| Coverage   | All 9 `gen_*_v0` functions                                      |
| Role       | Reference implementation — defines the conformance test vectors |

### iscc-lib — Rust + Polyglot Bindings

High-performance Rust core with bindings for Python, Node.js, WebAssembly, Go, Java, and C. This
project.

|            |                                                                                                                                                                                                                      |
| ---------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Repository | [iscc/iscc-lib](https://github.com/iscc/iscc-lib)                                                                                                                                                                    |
| Packages   | [crates.io](https://crates.io/crates/iscc-lib), [PyPI](https://pypi.org/project/iscc-lib/), [npm (@iscc/lib)](https://www.npmjs.com/package/@iscc/lib), [npm (@iscc/wasm)](https://www.npmjs.com/package/@iscc/wasm) |
| Language   | Rust (core) + Python, Node.js, WASM, Go, Java, C bindings                                                                                                                                                            |
| License    | Apache-2.0                                                                                                                                                                                                           |
| Coverage   | All 9 `gen_*_v0` functions — full conformance with iscc-core                                                                                                                                                         |
| Role       | Performance-optimized polyglot implementation                                                                                                                                                                        |

## Community Implementations

!!! note "Independent projects"

    Community implementations are independently maintained and may not track the latest specification
    changes. Verify conformance status before using in production.

### iscc-core-ts — TypeScript

A standalone TypeScript implementation of the ISCC core functions for the JavaScript ecosystem.
Created by François Branciard with funding from NGI Zero Core (NLnet / European Commission).
Maintained by the original author under the ISCC Foundation GitHub org.

|            |                                                                   |
| ---------- | ----------------------------------------------------------------- |
| Repository | [iscc/iscc-core-ts](https://github.com/iscc/iscc-core-ts)         |
| Package    | [iscc-core-ts on npm](https://www.npmjs.com/package/iscc-core-ts) |
| Language   | TypeScript (Node.js, Browser via bundler)                         |
| License    | Apache-2.0                                                        |
| Version    | 1.0.0                                                             |
| Status     | Stable — security audited, full conformance with iscc-core v1.2.2 |

**Function coverage:**

The project implements all 9 `gen_*_v0` functions from ISO 24138, plus additional functions
(`gen_iscc_id_v0`, `gen_iscc_id_v1`, `gen_flake_code_v0`):

| Function               | Tests |
| ---------------------- | ----- |
| `gen_meta_code_v0`     | 16    |
| `gen_text_code_v0`     | 5     |
| `gen_image_code_v0`    | 3     |
| `gen_audio_code_v0`    | 5     |
| `gen_video_code_v0`    | 3     |
| `gen_mixed_code_v0`    | 2     |
| `gen_data_code_v0`     | 4     |
| `gen_instance_code_v0` | 3     |
| `gen_iscc_code_v0`     | 5     |

**Conformance testing:** 263 tests derived from the official `data.json` conformance vectors,
running across 4 module modes (CJS, CJS-isolated, ESM, ESM-isolated) for 1,052 total test
executions. Includes a dedicated `conformance.test.ts` suite.

**Security:** Completed an external security audit by Radically Open Security (ROS) with all
findings remediated. See the
[audit remediation report](https://github.com/iscc/iscc-core-ts/blob/main/docs/audit-remediation-report.md)
for details.

## Contributing an Implementation

The ISCC standard is open and community implementations in any language are welcome. To ensure
interoperability:

1. **Use the official test vectors** — validate against the `data.json` file from
    [iscc-core](https://github.com/iscc/iscc-core). This file contains inputs and expected outputs
    for all 9 `gen_*_v0` functions.
2. **Reference the specification** — [ISO 24138:2024](https://www.iso.org/standard/77899.html)
    defines the algorithms. The [iscc-core](https://github.com/iscc/iscc-core) Python source serves
    as the executable specification.
3. **Open an issue or PR** — let the community know about your implementation by opening an issue on
    [iscc-lib](https://github.com/iscc/iscc-lib) or [iscc-core](https://github.com/iscc/iscc-core).
