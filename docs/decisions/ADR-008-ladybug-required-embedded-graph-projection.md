# ADR-008: Ladybug Required Embedded Graph Projection

## Status
Accepted

## Date
2026-06-08

## Context
DIALECTICA capsules must be portable enough to travel with PRAXIS workflows and
queryable enough for humans, agents, and reviewers to inspect relationships,
source support, caveats, temporal scope, and reasoning devices without
reconstructing the graph manually.

ADR-003 keeps Cloud SQL PostgreSQL as the first operational build store.
ADR-001 keeps the signed `.capsule` bundle as the portable product contract.
Those decisions remain correct, but the previous "Ladybug optional adapter"
posture under-served the product goal: a capsule should carry not only open
semantic files, but also a ready embedded graph database.

Current Ladybug Rust source anchors:

- Ladybug docs: <https://docs.ladybugdb.com/>
- Ladybug Rust tutorial: <https://docs.ladybugdb.com/tutorials/rust/>
- `lbug` crate docs: <https://docs.rs/lbug/0.17.1>
- `lbug` crate: <https://crates.io/crates/lbug/0.17.1>
- Ladybug Rust repository: <https://github.com/LadybugDB/ladybug-rust>

`lbug` 0.17.1 requires Rust 1.81. The repository workspace therefore uses
Rust 1.81 as its minimum Rust version.

## Decision
Every promoted v3 PRAXIS Capsule must include a required embedded Ladybug graph
projection:

```text
graph/ladybug/capsule.lbug
graph/ladybug/projection_manifest.json
graph/ladybug/schema.cypher
graph/ladybug/queries.cypher
graph/ladybug/build_receipt.json
```

The projection is required, signed by digest, and read-only for PRAXIS use.
It is still rebuildable from canonical capsule records:

```text
reviewed records + graph.jsonld -> graph/ladybug/capsule.lbug
```

`graph.jsonld` remains the canonical semantic graph serialization. PostgreSQL
remains the operational build store. Ladybug is the required queryable embedded
graph artifact for promoted capsules, not a path for bypassing source spans,
review decisions, or deterministic compiler rules.

## Consequences
Positive:

- PRAXIS can inspect capsule graphs offline through a real embedded graph
  database.
- Agents can run read-only Cypher queries against the same graph humans see.
- Graph previews, graph receipts, and context-pack projections become easier to
  verify.
- The `.capsule` artifact becomes more useful as a standalone knowledge object.

Negative:

- Promoted capsule artifacts are larger.
- Builders need the `lbug` Rust dependency for projection generation and query
  smoke tests.
- Windows development should have Git Bash `sh.exe` on `PATH` so `lbug` can use
  its prebuilt-library downloader; otherwise it may fall back to CMake.

## Acceptance Criteria
- v3 package validation requires `graph/ladybug/*` projection files.
- Projection manifests carry source graph, schema, query, and database digests.
- `dialectica-cli ladybug-build` can materialize `capsule.lbug` from
  `graph.jsonld`.
- `dialectica-cli ladybug-check` validates projection manifests and digests.
- `dialectica-cli ladybug-query` opens the database read-only.
- The canonical Situation Capsule fixture includes a real `capsule.lbug`
  projection.

## Supersedes
This ADR supersedes earlier language that treated `ladybug_projection_v1` as an
optional research adapter. It refines, but does not replace, ADR-001 and
ADR-003.
