# ADR-003: PostgreSQL First Operational Store

Status: accepted

Date: 2026-06-07

## Context

DIALECTICA needs to store capsule jobs, source ledgers, extraction receipts,
claims, temporal facts, graph edges, ontology mappings, review decisions,
embeddings, and bundle manifests.

The system may eventually use specialized graph, vector, RDF, or temporal
stores, but requiring them too early would slow the MVP and make deployment more
fragile.

## Decision

Use Cloud SQL PostgreSQL as the first operational store.

Use PostgreSQL tables for graph edges and ontology mappings in the MVP. Add
pgvector when embeddings are needed. Treat specialized graph and semantic
engines as derived adapters unless a later ADR promotes one.

## Consequences

Positive:

- simpler deployment;
- easier local development;
- strong transactional model;
- mature migrations and backup workflows;
- enough flexibility for first graph, temporal, and semantic records.

Negative:

- graph traversal and semantic reasoning may become slower at scale;
- specialized query features may require adapters later;
- schema discipline matters because many concerns share one store.

## Acceptance Criteria

- PostgreSQL schema can represent source, temporal, graph, ontology, review,
  and bundle metadata records.
- Capsule export does not depend on a separate graph database.
- Optional adapters can be rebuilt from canonical PostgreSQL and object storage
  records.
