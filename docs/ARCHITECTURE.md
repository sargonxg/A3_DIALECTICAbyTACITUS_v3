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
- call source-bound LLM extraction passes that propose entities, claims, dates,
  actors, relationships, reasoning devices, language rules, and uncertainties;
- create source ledger records;
- write extraction proposal records and model invocation receipts;
- route required human review gates;
- enqueue review and compile work.

Workers must be idempotent. Reprocessing the same source version should not
silently duplicate semantic records.

LLM extraction is proposal-only. See
[LLM Context Extraction Architecture](LLM_CONTEXT_EXTRACTION_ARCHITECTURE.md)
and [ADR-007](decisions/ADR-007-llm-extraction-proposal-boundary.md).

### Temporal Layer

Responsibilities:

- distinguish event time, publication time, ingestion time, and review time;
- mark freshness, staleness, supersession, forecast status, and uncertainty;
- preserve conflicting claims instead of forcing premature reconciliation;
- let PRAXIS ask what was true at a specific time.

Temporal modeling is not optional for policy analysis.

### Semantic and Ontology Layer

Responsibilities:

- generate capsule-specific ontology blueprints from capsule type, domain,
  source pack, user scope, and intended PRAXIS workflow;
- define controlled vocabularies for the capsule's actual matter: users,
  domains, actors, instruments, sectors, source types, methods, outputs, and
  analytic frames as needed;
- map extracted terms to ontology nodes with confidence and review state;
- preserve alternate frames where experts disagree;
- expose compact ontology slices inside capsule bundles.

The ontology starts pragmatic and local. Actor/claim analysis is a powerful
situation and stakeholder lens, not a universal ontology. Each capsule type
must be able to develop its own semantic layers and then map local terms back to
shared graph classes for PRAXIS interoperability.

### Graph Layer

Responsibilities:

- represent the relationship map selected by the capsule ontology blueprint:
  actors, institutions, claims, sources, concepts, policies, events, methods,
  outputs, rights, causal hypotheses, and dependencies where relevant;
- preserve provenance for every edge;
- export a compact embedded graph for PRAXIS agents and graph visualization;
- export optional JSON-LD semantic projections for interoperability;
- validate node/edge classes against capsule-type graph profiles;
- support graph summaries, neighborhood retrieval, and contradiction detection.

For the foundation build, PostgreSQL is canonical. Graph engines may be added as derived
adapters after the base schema and capsule export prove useful.

### Review Gate

Responsibilities:

- capture expert approval, rejection, correction, escalation, and uncertainty;
- make review decisions visible in the capsule bundle;
- distinguish machine extraction from human judgment;
- preserve reviewer identity, timestamp, basis, and scope;
- block capsule promotion when required gates fail.

Review data is part of the capsule, not a separate admin afterthought.

### Marketplace and Lineage Layer

Responsibilities:

- expose capsule type, review level, freshness, rights, caveats, and lineage;
- support expert-pick and certified capsule listings;
- preserve fork ancestry and inherited review scope;
- prevent local forks from inheriting approval for changed content;
- make marketplace objects inspectable before PRAXIS loads them.

Marketplace data is not marketing metadata. It is a trust and compatibility
contract for reusable capsules.

### Capsule Compiler

Responsibilities:

- assemble a deterministic capsule bundle;
- include manifest, source ledger, temporal ledger, ontology slice, graph slice,
  graph semantics, graph constraints, reasoning playbook, agent guidance,
  retrieval packs, output contracts, rights profile, marketplace metadata, and
  review ledger;
- sign and checksum bundle components;
- write bundle artifacts to storage;
- produce PRAXIS-ready summaries, compact context packs, and model-facing
  agent guidance.

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
4. Extractors propose entities, claims, temporal facts, relationships,
   reasoning devices, language rules, and review triggers.
5. Rust validators and cross-check agents normalize and inspect proposals.
6. Reviewers approve, reject, correct, or annotate records where required.
7. Canonical records are written to PostgreSQL with provenance and review
   state.
8. The compiler assembles a signed capsule bundle.
9. The eval harness checks bundle validity and PRAXIS utility.
10. Promoted capsules become visible to PRAXIS.

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
- [ADR-007: LLM Extraction Proposal Boundary](decisions/ADR-007-llm-extraction-proposal-boundary.md)
