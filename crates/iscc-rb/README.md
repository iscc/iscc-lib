# iscc-lib

High-performance Ruby bindings for **ISO 24138:2024 — International Standard Content Code (ISCC)**.

Native Rust extension via [Magnus](https://github.com/matsadler/magnus) for content identification
and matching.

## Installation

```bash
gem install iscc-lib
```

## Quick Start

```ruby
require "iscc_lib"

result = IsccLib.gen_meta_code_v0("Hello World", description: "A greeting")
puts result.iscc      # => "ISCC:..."
puts result.name      # => "Hello World"
puts result.metahash  # => "1e20..."
```

## Documentation

Full documentation at [lib.iscc.codes](https://lib.iscc.codes).

## License

Apache-2.0
