## 2026-11-22 — Review of: `version:check` checks every version occurrence and fails on a vanished target

**Verdict:** PASS

**Summary:** Every getter in `scripts/version_sync.py` now returns a list with each version its sync
function rewrites, and both modes fail (exit 1) on a missing target file or an unmatched pattern.
`--check` fails on any stale occurrence, and the sync repairs a file when any occurrence differs.
The audit issue is closed. The diff touches the script and its tests only: no library code, no
quality gate weakened, no suppression added.

**Verification:**

- [x] `uv run pytest tests/test_version_sync.py`: 19 passed. The four `check_mode` failure tests
    exist, one bad case each: (a) current Maven + stale SPM doc, (b) missing target, (c) unmatched
    pattern, (d) stale second `--version` in `mise.toml`-style text
- [x] `test_sync_rewrites_stale_second_doc_snippet` reads the file back and compares it with the
    all-current text
- [x] `python scripts/version_sync.py --check` exits 0 on the real tree and prints 22 `OK:` lines
- [x] `uv run ruff check scripts/version_sync.py tests/test_version_sync.py` and
    `uv run ty check scripts/version_sync.py` exit 0 (also the pre-push forms: project-wide
    `ty check`, ruff `S`/`C901`)
- [x] `mise run check` exits 0

Three checks beyond next.md's list, all in a `git archive` copy under `/tmp`:

- **Round trip**: `Cargo.toml` bumped to 9.8.7, sync (22 `Synced` lines, exit 0), `--check` (22
    `OK:`), then back to 0.7.0 and sync again: the tree is byte-identical to HEAD. The changed regex
    groups and backreferences corrupt no target
- **Command line failure modes**: a stale third snippet in `README.md`, a deleted
    `docs/howto/kotlin.md` and a reworded `gradle.properties` line. `--check` exits 1 with one
    `MISMATCH` line each; the sync exits 1, repairs the README and prints `FAILED:` for the other
    two
- **Literal count**: in each of the 22 targets, the number of literal `0.7.0` strings equals the
    number of versions its getter reports. No target holds a version the gate cannot see

**Issues found:**

- Known limit, not filed: in a file with several snippets, one snippet that stops matching (a
    reworded line) goes unseen while a sibling still matches. The script holds no expected count per
    file. `test_doc_getter_reports_every_snippet` pins `docs/index.md` at 4 and so covers that one
    file. A general check (literal version count equals getter count, as a test) would turn a
    release commit red on any "since X.Y.Z" sentence, so I did not add it
- (nothing else)

**Codex review:** no actionable findings. Codex ran the 19 tests and the real-tree check, and
confirmed update, restore and idempotence across all 22 targets in memory.

**Next:** Go `EncodeComponent` on a short digest (`normal` [audit]) is the only remaining `normal`
issue the loop can move without Titusz. The core already rejects that input and a test pins it, so
the rule exists on eleven surfaces. A workable split: first record the rule in decisions.md and make
Go return an `iscc:` error with a Go test (Go-only step), then add the `encode_component` section to
`data-edge.json` (one 12-surface fan-out). The other two `normal` issues need Titusz's decision on
behaviour: `alg_minhash_256([])` and the WASM `finalize()` consumption rule. Once the Go issue is
closed and only those two remain, the right signal is HUMAN REVIEW REQUESTED, not IDLE.

**Notes:**

- Closed and deleted from issues.md: "`version:check` passes a stale second doc snippet and skips a
    vanished target silently". issues.md now holds 3 `normal` and 15 `low` entries
- The sync now exits 1 when a target file is gone or its pattern no longer matches. Whoever renames
    or removes a synced file must update `TARGETS` in the same change, or the release sync stops
- The `low` issue about partial file lists in `docs/development.md` and the release skill is
    untouched. Both pages should point to `TARGETS`
- No decision entry and no new learning: the contract is stated in the script's module docstring,
    and the existing learnings line on the Maven/Gradle/SPM snippets still holds
- No API change, no benchmarked path, no Rust touched
