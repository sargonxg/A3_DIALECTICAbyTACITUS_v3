# ADR-009: Place The First Capsule Diff Engine In The Compiler

## Status
Accepted

## Date
2026-06-10

## Context
DIALECTICA v3 needs a capsule diff engine as a local proof artifact before
store-backed API, Cloud Run, or PRAXIS production integration. The diff must
compare reviewed capsule versions, produce a structured `diff.json`, and render
a cited change memo that PRAXIS can inspect.

The current workspace already gives `dialectica-compiler` ownership of
deterministic package writing, `.capsule` archive assembly, PRAXIS context-pack
export, review-gate enforcement, and bundle digests. `dialectica-capsule` owns
portable records and validation. `dialectica-extractor` owns source packs,
proposal records, review routing, and promotion normalization.

The strategic plan also anticipates future temporal and conflict-perception
work from KAIROS and AGON, but those systems are not required for the first
local diff proof.

## Decision
Implement the first capsule diff engine inside `dialectica-compiler`.

The v1 diff API should consume two compiled v3 package directories, compare
canonical files and promoted records, and emit:

- `schemas/capsule-3.0/capsule_diff.schema.json`;
- a deterministic `diff.json` with added, retracted, superseded, and changed
  records;
- trust and review-state transitions;
- source-pack deltas keyed by source ids and hashes;
- temporal status changes for claims and episodes already represented in the
  package;
- reasoning-device and heuristic deltas for Tool and Situation capsules;
- a rendered `change-memo.md` that cites source and review receipts from the
  newer capsule.

CLI and MCP surfaces may call this compiler-owned core. REST routes should wait
until store-backed artifacts exist.

## Alternatives Considered

### New `dialectica-diff` crate
This would create a clean boundary, but it adds workspace surface before the
first diff behavior is proven. Rejected for v1.

### Put diff types in `dialectica-capsule`
The capsule crate should own stable portable records and schemas, not comparison
or rendering behavior. Rejected for engine behavior, accepted for any stable
schema structs once the v1 shape settles.

### Create `dialectica-kairos`
Temporal reasoning will eventually deserve a narrower crate if capsule diffs
need independent bi-temporal inference. Rejected for v1 because the current
diff can compare existing capsule temporal ledgers without adding a new
subsystem.

## Consequences
Positive:

- Keeps the local demo gate small: reviewed capsule -> signed package -> diff
  -> cited change memo.
- Reuses compiler digest, package loading, deterministic serialization, and
  PRAXIS context-pack knowledge.
- Avoids prematurely introducing AGON/KAIROS runtime dependencies.

Negative:

- `dialectica-compiler` becomes broader until the diff shape is stable.
- A future extraction/store-backed diff may need refactoring into a dedicated
  crate after the local proof.

## Acceptance Criteria
- Golden v1/v2 capsule pair produces byte-identical `diff.json` across runs.
- `change-memo.md` renders with valid source/review citations.
- `dialectica-cli diff <old-dir> <new-dir> --out <dir>` writes both artifacts.
- Local MCP exposes a read-only diff tool over compiled package directories.
- Eval includes a fixture diff-correctness check.
- No live provider, database, hosted MCP, or cloud dependency is required.
