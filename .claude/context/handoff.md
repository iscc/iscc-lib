## 2026-11-22 — Review of: Go `AlgMinhash256` returns an error for an empty feature list

**Verdict:** PASS

**Summary:** Go `AlgMinhash256` is now `([]byte, error)` and rejects an empty or nil list with
`features must not be empty`. It delegates to the unexported `algMinhash256`, which both generators
call, so no gen output moved. The diff touches exactly the six files next.md listed (three code
files) and mirrors the `AlgSimhash` / `algSimhash` pattern, doc comment wording included.

**Verification:**

- [x] `go test ./...` passes, and `GOARCH=386 go test ./...` passes — both `ok` with `-count=1` and
    `CGO_ENABLED=0`
- [x] `go vet ./...` clean and `CGO_ENABLED=0 go build ./...` exits 0 — also clean under
    `GOARCH=386`; `gofmt -l packages/go` prints nothing
- [x] The signature grep on `packages/go/minhash.go` prints one line (204)
- [x] The `0xFF` grep on `minhash_test.go` shows no empty-list test — the hits are
    `TestMinhashCompressAllOnes` (lines 61, 71, 72) and the feature value `0xFFFFFFFF` (line 117)
- [x] The docs/spec grep prints `docs/howto/go.md:340` and `go-bindings.md:58`
- [x] `mise run check` exits 0, no hook failed, no file reformatted

Beyond next.md (one probe): a `/tmp` Go program against the checkout and the Python binding return
the same digest for `[7, 42, 2147483648, 4294967295]` (`de7197a4…68b4b8`), the same message for an
empty list (Go bare, Python with the `invalid input:` prefix), and the same codes for
`gen_data_code_v0(b"", 64)` and `gen_text_code_v0("", 64)`.

**Issues found:**

- (none) — a repo-wide `git grep AlgMinhash256` finds no other Go call site and no page with the old
    signature; `packages/go/README.md` and `CLAUDE.md` list the name only

**Codex review:** no actionable findings. It confirmed that empty and nil lists are rejected, that
the generators keep their behaviour through the unchecked helper, and that the Go tests and vet pass
on amd64 and 386.

**Next:** the `alg_minhash_256` section in `data-edge.json` as one 12-surface fan-out (the issue "No
shared vector pins the `alg_minhash_256` empty-list error", rewritten in this review): one error row
and one non-empty row, no `except` entry. The behaviour is fixed by the decision of 2026-11-22 and
by the code on all 12 surfaces, so the step is harness and generator work only. Let it carry the nit
from the review of iteration 298: the XML doc of .NET `AlgMinhash256` (`IsccLib.cs`) and the `///`
of C++ `alg_minhash_256` (`iscc.hpp`) do not name the error. After that comes the other `normal`
issue, "Gen functions accept `bits` above 256".

**Notes:**

- state.md still says Go returns `0xFF` for an empty list (its line on `minhash.go:202`);
    update-state corrects that
- Go's error text has no `invalid input:` prefix, as with every Go error. The `alg_simhash` section
    of `data-edge.json` has the same case (`"error": "hash_digests must not be empty"`, the bare
    message, on a checked function with an unchecked helper): model the new section on it
- The Go signature change is a public break of the Go module. The decision of 2026-11-22 covers it,
    and it is not a Rust core API break, so no `API-BREAK` flag was owed
- learnings.md and decisions.md are unchanged: the step produced no new operational fact and no new
    judgment call
