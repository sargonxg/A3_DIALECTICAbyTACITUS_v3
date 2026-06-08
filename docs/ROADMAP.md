# Roadmap

## Phase 0: Source of Truth

Status: active.

Deliverables:

- imported build instructions;
- README;
- architecture docs;
- deployment docs;
- capsule spec;
- ADRs;
- GitHub templates.

## Phase 1: Contract Runtime

Deliverables:

- Rust workspace;
- capsule schema crate;
- fixture capsule bundle;
- local validator CLI;
- contract tests;
- CI for formatting and tests.

Acceptance:

- sample capsule validates locally and in CI.

## Phase 2: Canonical Store

Status: deferred until the local source-pack, proposal, compiler, and
context-pack loop works.

Deliverables:

- PostgreSQL schema;
- migrations;
- source ledger tables;
- capsule job tables;
- review ledger tables;
- local Postgres development path.

Acceptance:

- fixture data round-trips through PostgreSQL and exports a valid capsule.

## Phase 3: Ingestion foundation build

Deliverables:

- local document ingestion;
- text normalization;
- source span records;
- extraction proposal records;
- extraction run receipts;
- review-trigger routing;
- idempotent worker execution.

Acceptance:

- fixture source pack creates source ledger, normalized artifacts, proposal
  records, and review triggers without calling a model provider.

## Phase 4: Capsule Compiler

Deliverables:

- manifest compiler;
- temporal ledger compiler;
- ontology and graph slice compiler;
- reasoning playbook compiler;
- retrieval pack compiler;
- checksum and signature files.

Acceptance:

- generated capsule bundle passes contract evals and cannot include unreviewed
  model proposals in promoted PRAXIS context.

## Phase 5: Review Gate

Deliverables:

- review decision model;
- reviewer notes;
- promotion gate;
- blocked bundle behavior;
- audit receipts.

Acceptance:

- unreviewed required records, including LLM-proposed graph edges, language
  rules, rights rules, and expert reasoning steps, cannot be promoted.

## Phase 6: PRAXIS Adapter

Deliverables:

- manifest endpoint;
- context-pack endpoint;
- capsule-set combine endpoint;
- status and warning mapping for PRAXIS.

Acceptance:

- PRAXIS can use one capsule in a controlled workflow.

## Phase 7: Eval Harness

Deliverables:

- golden fixture;
- raw baseline comparison;
- capsule-augmented comparison;
- source, temporal, and reasoning scores.

Acceptance:

- eval report shows measurable improvement or produces actionable failures.

## Phase 8: Cloud Run Staging

Deliverables:

- container images;
- Cloud Run API;
- Cloud Run task handler;
- Cloud SQL staging;
- Cloud Storage staging bucket;
- Cloud Tasks queue.

Acceptance:

- staging can compile and validate one fixture capsule.

## Phase 9: Advanced Semantic and Graph Adapters

Deliverables:

- richer ontology mapping;
- graph enrichment;
- contradiction detection;
- causal hypothesis support;
- temporal graph summaries.

Acceptance:

- adapters improve eval scores and remain non-canonical unless promoted by ADR.

## Phase 10: Production Pilot

Deliverables:

- one policy capsule type;
- human review workflow;
- production deployment;
- observability dashboards;
- rollback plan;
- PRAXIS-visible capsule receipts.

Acceptance:

- a reviewed capsule improves a real PRAXIS policy workflow.
