"""High-performance ISCC (ISO 24138:2024) implementation backed by Rust."""

from iscc._lowlevel import gen_instance_code_v0

__all__ = ["gen_instance_code_v0"]
