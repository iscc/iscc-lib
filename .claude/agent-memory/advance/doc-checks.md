---
name: doc-checks
description: how to compile-check Java, C++ and Rust code blocks from the docs in throwaway /tmp projects
metadata:
  type: project
---

# Doc example checks

- Docs code check (238): wrap a Java doc block in a `/tmp` class (`import io.iscc.iscc_lib.*;`),
    `javac -cp crates/iscc-jni/java/target/classes` after `mvn compile`; run with
    `-Djava.library.path=target/debug`. Ruby gem range pinned by `tests/test_ruby_gem_docs.py`. C++
    (244): extract each ```` ```cpp ```` block with `int main()` to /tmp,
    `g++ -std=c++17 -Wall   -I packages/cpp/include -I crates/iscc-ffi/include f.cpp -L target/debug -liscc_ffi`,
    run with `LD_LIBRARY_PATH=target/debug`. zsh: `rm -f glob*` with no match aborts an `&&` chain
- Doc example check (243): extract non-signature \`\`\`rust blocks into a throwaway
    `crates/iscc-lib/tests/zz_*.rs` (`#[test]` + `-> Result<(), Box<dyn Error>>` wrapper), run,
    delete; examples that write files (`example.bin`) land in `crates/iscc-lib/` — remove them
