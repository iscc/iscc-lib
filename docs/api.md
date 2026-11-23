---
icon: lucide/book-open
description: Python API reference for ISCC code generation functions and result types.
---

# API Reference

Python bindings for the ISCC (ISO 24138:2024) code generation functions. The `gen_*` functions
accept typed parameters and return result objects — `dict` subclasses whose keys are also readable
as attributes.

```python
from iscc_lib import gen_meta_code_v0

result = gen_meta_code_v0("Example Title")
print(result["iscc"])  # "ISCC:..."
print(result.iscc)  # same value
```

## Functions

::: iscc_lib
