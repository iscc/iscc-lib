## 2026-06-17 — Release the GIL during Python hashing (`py.allow_threads`)

**Done:** Wrapped the pure-Rust CPU-bound compute in `py.allow_threads(...)` at all 7 call sites in
`crates/iscc-py/src/lib.rs` — the 4 one-shot byte-data functions (`gen_image_code_v0`,
`gen_data_code_v0`, `gen_instance_code_v0`, `gen_sum_code_v0`) and the 3 streaming `update()`
methods (`DataHasher`, `InstanceHasher`, `SumHasher`). Resolves issue #39. Output bytes are
unchanged; this is purely a concurrency improvement so threaded consumers can overlap hashing
instead of serializing on the GIL.

**Files changed:**

- `crates/iscc-py/src/lib.rs`: 4 one-shot functions now compute inside `py.allow_threads(|| ...)`
    with `PyDict` construction kept outside the closure (GIL held). The 3 `update()` methods gained
    an injected `py: Python<'_>` parameter (invisible to Python, no `.pyi` change), take the
    `&mut inner` borrow with the finalized-error check while the GIL is held, then release the GIL
    around `inner.update(data)`.
- `tests/test_gil.py` (new): 7 deterministic concurrency-correctness tests using a 16-thread
    `ThreadPoolExecutor`. Cover all 7 release sites — identical-payload and distinct-payload
    one-shot runs plus chunked streaming through each hasher — asserting byte-identical output to
    the single-threaded path. They verify correctness under concurrency, not a speedup.

**Verification:**

- `cargo build -p iscc-py` compiles (proves `Ungil`/`Send` bounds hold at every call site — the
    borrowed `&[u8]` and `&mut iscc_lib::*Hasher` are `Ungil + Send`, no copy needed).
- `cargo clippy -p iscc-py -- -D warnings` clean.
- `cargo fmt -p iscc-py --check` clean.
- `grep -c "allow_threads" crates/iscc-py/src/lib.rs` → 7.
- `maturin develop` succeeds; `pytest tests/` → 286 passed (existing conformance/smoke/streaming
    suites unchanged + 7 new `test_gil.py`).
- `ruff check tests/test_gil.py` and `ruff format --check tests/test_gil.py` clean.
- `mise run check`: all hooks Passed for my files (Rust formatting, Ruff linting/formatting). The
    only "Failed" hook is `mdformat`, which reformats pre-existing CID context files (`next.md`,
    define-next memory) — no markdown is in my implementation scope; reverted those files (see
    Notes).

**Next:** With #39 closed, the highest-value remaining backlog item is the **npm
`optionalDependencies` fix (#38)** — a confirmed install-breaking bug for downstream consumers with
a pre-recorded fix decision (drop the `napi prepublish -t npm` injection, keep the bundled-package
model). It touches the release workflow + `crates/iscc-napi/package.json`. After that, the **PyO3
0.23 → 0.29 security migration** (closes two RustSec advisories) is the next normal-priority item.

**Notes:**

- No Python-facing signature changed: PyO3 auto-injects the `py: Python<'_>` parameter on the
    `update()` methods, so it is not visible from Python and `_lowlevel.pyi` stays untouched.
- Borrow soundness: kept the borrowed `&[u8]` (no owned `Vec<u8>` copy) — it compiled, and the
    public `__init__.py` wrapper already coerces inputs to immutable `bytes` before calling
    `_lowlevel`, so the buffer cannot be mutated by another thread during the release.
- No size-threshold optimization added (next.md flagged it as premature; the wrapper already feeds
    64 KiB chunks). Unconditional release is the simplest correct choice.
- The `finalize()` methods were intentionally left GIL-held (out of scope per #39 — the bulk compute
    is in `update()`/the one-shot path; `finalize()` builds a `PyDict` which needs the GIL).
- **CID process friction (out of scope):** `mise run format`/`check` runs `prek --all-files`, and
    `mdformat` reformats `.claude/context/next.md` and `.claude/agent-memory/define-next/MEMORY.md`
    every time because they were committed in a non-mdformat-conforming state. I reverted them so
    they are not staged. The define-next agent writes these without mdformat; consider running
    mdformat on them when authored to stop this recurring churn during advance/review.
