## 2026-11-21 — Review of: Realign agent-facing specs and CLAUDE.md files with the code

**Verdict:** PASS_WITH_NOTES

**Summary:** The advance commit rewrote all seven release-trigger claims and the four audit
contradictions in five specs and three crate `CLAUDE.md` files. Docs only, no code or workflow
change. I checked each rewritten sentence against `release.yml` and the binding sources: all hold
except one clause in the Swift Release Flow, which I corrected. Both issues are closed.

**Verification:**

- [x] `git grep -n -i -E 'tag[- ]triggered|tag trigger as|tags: \[v\*|Tag push triggers' -- .claude/context/specs crates/*/CLAUDE.md`
    prints nothing — exit 1, no output
- [x] `git grep -n 'allow_threads' -- .claude/context/specs` and
    `git grep -n 'const \[mt' -- .claude/context/specs` print nothing — both exit 1
- [x] `grep -q iscc-uniffi … && grep -q read_unit … && grep -q SEQ_MAINTYPES crates/iscc-lib/CLAUDE.md`
    exits 0
- [x] `git grep -n 'parameters with defaults' -- .claude/context/specs/swift-bindings.md` prints
    nothing
- [x] `uv run scripts/check_spec_paths.py` and `uv run scripts/check_tier1_docs.py` exit 0 ("15
    specs", "20 full and 2 exact Tier 1 docs")
- [x] `mise run check` passes (all 20 hooks); the advance commit touches only
    `.claude/context/specs/`, `crates/*/CLAUDE.md`, the handoff and advance memory

Probes beyond next.md (3):

- The JS recipe in `rust-core.md` runs on the built napi addon: `iscc_decode` returns an object with
    `maintype, subtype, version, length, digest`, and the old array destructure throws "not
    iterable"
- `actions/checkout` source at `v7`: with `fetch-depth: 0` it fetches every tag (see below)
- All 18 `pub` items of `codec.rs` are named in `crates/iscc-lib/CLAUDE.md`; every `inputs.<name>`
    the specs mention exists in `release.yml`; the UniFFI exports and the generated Kotlin signature
    carry no defaults; `py.detach` has 12 sites and `allow_threads` none

**Issues found:**

- **Fixed in review** (`specs/swift-bindings.md`, Release Flow step 6): the new text said the force
    push overwrites "a stale remote tag of the same name". It does not. The checkout fetches every
    tag, `git tag` fails on the existing name, the step continues, and `push --force` pushes the
    unchanged tag. Step 6 now says an existing tag is not moved and shows the real commands; the
    closing paragraph limits the checksum guarantee to a tag the workflow creates.
- **Filed `normal`**: "`docs/development.md` tells maintainers to release by pushing a tag". The
    "Publish" section says `git push origin vX.Y.Z` triggers the workflow. It starts nothing, and a
    later dispatch then leaves that tag on the commit before the `Package.swift` checksum commit.
    The next.md sweep was scoped to specs and `CLAUDE.md` files, so it could not see this page. The
    same entry covers the Swift spec heading "Force-Update Tag Pattern" and its intro, which no
    longer match the flow below them.
- Not filed: `c-ffi-dx.md` section 4 reads "Add to `release.yml`" for jobs that exist (specs state
    requirements, so this is wording). `crates/iscc-py/CLAUDE.md` names Linux x86_64 only in its
    wheel matrix.
- Three stale entries in `git worktree list` (`/tmp/iscc-review-*`, marked prunable) predate this
    step. A spike review would trip on them; `git worktree prune` removes them.

**Codex review:** one finding (P2), the same Swift step-6 clause: existing tags are retained, not
overwritten, and the checksum guarantee holds only for newly created tags. Fixed as described above.
Codex confirmed the API corrections match the implementation.

**Next:** This push carries the three `cid(human)` CI changes (Python 3.15 leg, .NET 8/10 matrix) to
the remote for the first time. Check the CI run on the pushed head first.

- If it is green: Ruby floor 3.1 → 3.3 (`normal` [human], decided).
- If it is red: repair the failing job; the cause is in the human CI changes, because this step
    changed no code.
- A docs-only alternative that adds no CI risk: one bundle for `docs/development.md` "Publish" (new
    issue), the Swift heading, `docs/llms.txt` + ISO wording, the `KAA` comments and the pkg-config
    version.

**Notes:**

- Closed: "`ci-cd.md` describes a tag trigger the release workflow does not have" and "Specs and the
    core `CLAUDE.md` contradict the code in four places".
- `decisions.md` was 422 lines; the runner's `archive_overflow` rotated the two oldest entries into
    `decisions-archive.md` (now 396). No decision was recorded this review.
- `learnings.md` stays at 200 lines: the `release.yml` bullet was compressed to carry the
    tag-not-moved fact.
- The unpushed `cid(human)` and `cid(meta)` commits contain no gate weakening. The `if: always()` in
    `ci.yml` is on the new `dotnet` aggregator job, which fails unless the matrix succeeded.
