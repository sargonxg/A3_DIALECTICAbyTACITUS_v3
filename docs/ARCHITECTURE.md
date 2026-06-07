# DIALECTICA v3 Architecture

## Goal

DIALECTICA turns policy evidence, user context, analyst interactions, expert
reasoning, and review decisions into portable PRAXIS Capsules.

The architecture is designed around one principle:

> The capsule bundle is the product. Engines are supporting machinery.

## System Context

```text
                 PRAXIS cockpit and agentic workflows
                              |
                              v
                    DIALECTICA Capsule API
                              |
        +---------------------+----------------------+
        |                                            |
        v                                            v
 Ingestion and extraction workers           Capsule compiler
        |                                            |
        v                                            v
 Canonical PostgreSQL records         Signed capsule bundle in storage
        |                                            |
        v                                            v
 Review gate and eval harness          PRAXIS capsule library adapter
```

## Main Components

### Capsule API

Responsibilities:

- create ingestion and compile jobs;
- expose capsule manifests and bundle metadata;
- let PRAXIS request, resolve, combine, and inspect capsules;
- expose review and promotion endpoints;
- enforce tenant, user, and project authorization.

The API should stay thin. It coordinates durable jobs and reads canonical state.

### Ingestion Workers

Responsibilities:

- receive source artifacts from PRAXIS, uploads, connectors, or fixtures;
- normalize text, metadata, document spans, and language;
- extract entities, claims, dates, actors, relationships, and uncertainties;
- create source ledger records;
- write extraction run receipts;
- enqueue review and compile work.

Workers must be idempotent. Reprocessing the same source version should not
silently duplicate semantic records.

### Temporal Layer

Responsibilities:

- distinguish event time, publication time, ingestion time, and review time;
- mark freshness, staleness, supersession, forecast status, and uncertainty;
- preserve conflicting claims instead of forcing premature reconciliation;
- let PRAXIS ask what was true at a specific time.

Temporal modeling is not optional for policy analysis.

### Semantic and Ontology Layer

Responsibilities:

- define controlled vocabularies for domains, actors, instruments, sectors, and
  analytic frames;
- map extracted terms to ontology nodes with confidence and review state;
- preserve alternate frames where experts disagree;
- expose compact ontology slices inside capsule bundles.

The ontology starts pragmatic and local. It should evolve from repeated capsule
builds and expert corrections.

### Graph Layer

Responsibilities:

- represent actors, institutions, claims, sources, concepts, policies, events,
  causal hypotheses, and dependencies;
- preserve provenance for every edge;
- export a compact capsule graph for PRAXIS agents;
- support graph summaries, neighborhood retrieval, and contradiction detection.

For the MVP, PostgreSQL is canonical. Graph engines may be added as derived
adapters after the base schema and capsule export prove useful.

### Review Gate

Responsibilities:

- capture expert approval, rejection, correction, escalation, and uncertainty;
- make review decisions visible in the capsule bundle;
- distinguish machine extraction from human judgment;
- preserve reviewer identity, timestamp, basis, and scope;
- block capsule promotion when required gates fail.

Review data is part of the capsule, not a separate admin afterthought.

### Capsule Compiler

Responsibilities:

- assemble a deterministic capsule bundle;
- include manifest, source ledger, temporal ledger, ontology slice, graph slice,
  reasoning playbook, retrieval packs, output contracts, and review ledger;
- sign and checksum bundle components;
- write bundle artifacts to storage;
- produce PRAXIS-ready summaries and compact context packs.

The compiler must be deterministic enough for test fixtures and reproducible
bundle checks.

### Eval Harness

Responsibilities:

- test whether capsules improve PRAXIS responses;
- catch unsupported claims, missing citations, temporal mistakes, and weak
  reasoning transfer;
- compare raw LLM output against capsule-augmented output;
- score capsule bundle validity and usefulness before promotion.

## Canonical Data Flow

1. PRAXIS or a local fixture creates a capsule job.
2. Sources are written to immutable artifact storage.
3. Ingestion workers parse and normalize source material.
4. Extractors propose entities, claims, temporal facts, and relationships.
5. Canonical records are written to PostgreSQL with provenance.
6. Reviewers approve, reject, correct, or annotate records.
7. The compiler assembles a signed capsule bundle.
8. The eval harness checks bundle validity and PRAXIS utility.
9. Promoted capsules become visible to PRAXIS.

## Canonical Stores

### PostgreSQL

PostgreSQL stores:

- tenants, users, projects, capsule jobs;
- source records and source spans;
- extraction runs and model receipts;
- entities, claims, temporal facts, graph edges, ontology mappings;
- review decisions and promotion state;
- embeddings when pgvector is enabled;
- bundle manifests and export receipts.

### Object Storage

Object storage stores:

- original source files;
- normalized text artifacts;
- capsule bundle directories and archives;
- eval output artifacts;
- signed manifests and checksums.

### PRAXIS Adapter Store

PRAXIS may mirror selected capsule metadata into Firestore or another existing
PRAXIS store for UI visibility. That mirror is not the authoritative capsule
record unless an ADR explicitly changes this boundary.

## Trust Boundaries

- External sources are untrusted.
- User-uploaded documents are untrusted until parsed and scanned.
- Model extractions are proposals, not facts.
- Human review decisions are trusted only within their recorded scope.
- Capsule bundles are trusted only after validation, checksum verification, and
  promotion state checks.
- PRAXIS must be able to inspect the capsule provenance before using it in a
  high-stakes output.

## Failure Modes to Design For

- source deleted or superseded after capsule compile;
- two sources disagree about the same claim;
- model extracts a plausible but unsupported claim;
- reviewer approves only part of a capsule;
- stale capsule is reused in a new policy context;
- ontology mapping creates a false equivalence;
- graph edge looks causal but is only correlational;
- PRAXIS requests a capsule with insufficient citation density.

## Architecture Decisions

See:

- [ADR-001: Capsule Bundle as Source of Truth](decisions/ADR-001-capsule-bundle-source-of-truth.md)
- [ADR-002: Cloud Run First Deployment](decisions/ADR-002-cloud-run-first-deployment.md)
- [ADR-003: PostgreSQL First Operational Store](decisions/ADR-003-postgres-first-operational-store.md)
