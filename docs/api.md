---
icon: lucide/book-open
description: Python API reference for ISCC code generation functions and result types.
---

# API Reference

Python bindings for the ISCC (ISO 24138:2024) code generation functions. All functions accept typed
parameters and return **JSON strings** — use `json.loads()` to parse the result.

```python
import json
from iscc_lib import gen_meta_code_v0

result = json.loads(gen_meta_code_v0("Example Title"))
print(result["iscc"])  # "ISCC:..."
```

## Functions

::: iscc_lib
