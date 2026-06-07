# ADR-005: Benchmark-Informed Capsule Engine Posture

Status: accepted

Date: 2026-06-07

## Context

The AI memory and knowledge ecosystem includes temporal graph engines, memory
layers, graph RAG pipelines, agent orchestration runtimes, and context protocols.
DIALECTICA needs to learn from those systems while staying focused on PRAXIS
policy capsules.

## Decision

DIALECTICA v3 will not adopt a generic memory-layer posture.

It will build a policy-grade capsule compiler with:

- source ledgers;
- temporal ledgers;
- ontology slices;
- graph slices;
- expert reasoning playbooks;
- human review gates;
- PRAXIS context packs;
- signed portable bundles.

Graph, vector, MCP, and memory systems remain adapters until evals and an ADR
promote them.

## Alternatives Considered

### Required temporal graph engine

Rejected for foundation build. Temporal graph semantics are essential, but the runtime
dependency is not. Encode semantics first in Postgres and bundle exports.

### Required memory platform

Rejected for foundation build. Policy capsules need reviewed memory, not uncontrolled
personalization.

### Required GraphRAG pipeline

Rejected for foundation build. GraphRAG can be expensive and batch-oriented. Use small graph
slices and evals first.

### Required agent orchestration framework

Rejected for DIALECTICA foundation build. PRAXIS already owns visible agent runs and runtime
proof. DIALECTICA should serve context and receipts.

## Consequences

Positive:

- preserves PRAXIS product simplicity;
- avoids premature infrastructure lock-in;
- keeps capsule bundles portable;
- makes review and provenance first-class;
- leaves room for adapters after evidence.

Negative:

- requires building some semantic plumbing directly;
- delays advanced graph runtime experiments;
- requires disciplined evals before dependency promotion.
