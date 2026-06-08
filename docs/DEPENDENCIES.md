# Dependency Strategy

This document lists candidate dependencies and constraints. It is not permission
to add every dependency immediately.

## Principles

- Add dependencies only when they remove real implementation risk.
- Prefer boring infrastructure for the foundation build.
- Keep the capsule bundle readable without proprietary services.
- Keep PostgreSQL as the first operational source of truth.
- Treat graph, ontology, and model providers as adapters, with Ladybug as the
  required embedded query projection for promoted capsules.
- Pin versions once implementation begins.

## Runtime Language

Primary implementation language: Rust.

Rationale:

- strong type system for contract-heavy capsule schemas;
- good async service support;
- predictable binaries for Cloud Run;
- strong JSON, SQL, and testing ecosystem.

Python is auxiliary tooling only. Keep it under `tools/python/` for fixture
reports, eval summaries, research utilities, and graph sanity checks. Do not
use Python as the canonical API, compiler, store, or bundle validation runtime
without a new ADR.

## Rust Candidates

Validate current versions before adding.

| Area | Candidate | Use |
| --- | --- | --- |
| async runtime | `tokio` | service and worker runtime |
| HTTP API | `axum` | Capsule API and task handler |
| serialization | `serde`, `serde_json` | capsule bundle and API contracts |
| schema | `schemars` or generated JSON Schema | capsule validation |
| database | `sqlx` | PostgreSQL access and migrations |
| tracing | `tracing`, `tracing-subscriber` | structured logs |
| errors | `thiserror`, `anyhow` | library and binary error handling |
| config | `figment` or `config` | environment configuration |
| testing | `insta`, `assert_json_diff`, `testcontainers` | snapshot and integration tests |
| auth | `jsonwebtoken`, cloud IAM middleware | service auth |

Initial recommendation:

- `tokio` for async runtime and scheduling;
- `axum` for HTTP APIs and Cloud Tasks handlers;
- `serde` and `serde_json` for capsule contracts;
- `schemars` for JSON Schema generation;
- `sqlx` for PostgreSQL, migrations, and compile-time checked queries where
  practical;
- `tracing` for logs and spans;
- `thiserror` for library errors and `anyhow` for CLI and binary boundaries;
- `clap` for the local validator CLI;
- `insta` for stable capsule fixture snapshots.

Source anchors:

- Tokio: <https://tokio.rs/>
- Axum: <https://github.com/tokio-rs/axum>
- SQLx: <https://github.com/launchbadge/sqlx>
- Schemars: <https://docs.rs/schemars/latest/schemars/>

## Cloud Dependencies

| Service | Foundation Role |
| --- | --- |
| Cloud Run | API, task handler, jobs, optional worker pools |
| Cloud SQL PostgreSQL | canonical operational store |
| Cloud Storage | source artifacts and capsule bundle exports |
| Cloud Tasks | durable task dispatch and retries |
| Pub/Sub | optional fanout and pull worker integration |
| Secret Manager | secrets and signing material |
| Artifact Registry | container images |
| Cloud Build or GitHub Actions | CI/CD |
| Cloud Logging and Monitoring | observability |

## Database Extensions

Candidate extensions:

- `pgvector` for embeddings;
- `pg_trgm` for text matching;
- `uuid-ossp` or native UUID generation strategy;
- full-text search indexes where useful.

Do not add a separate vector database for the foundation build unless eval evidence proves
PostgreSQL is insufficient.

## Graph and Semantic Adapters

Graph and semantic engines stay derived from capsule records. Ladybug is the
exception that ADR-008 promotes into the required embedded projection shipped
inside every promoted capsule. PostgreSQL remains the operational service
store, and JSON-LD remains the portable semantic source graph.

Candidate adapter classes:

- PostgreSQL graph tables and recursive queries;
- RDF/OWL export adapter;
- property graph export adapter;
- temporal graph summarizer;
- ontology mapping service;
- LadybugDB projection adapter for required embedded graph exploration,
  read-only Cypher, and graph algorithms in promoted capsules.

Promotion rule:

An adapter becomes required infrastructure only after:

- it has contract tests;
- it improves capsule quality in evals;
- it has operational runbooks;
- an ADR approves the dependency.

Current adapter posture:

| Adapter | Status | Reason |
| --- | --- | --- |
| PostgreSQL projection | required | keeps runtime simple and deployable on Cloud SQL |
| JSON-LD export | required | preserves standards-compatible semantic layer |
| LadybugDB / `lbug` | required projection dependency | embedded property graph engine required for promoted capsule graph projection; compile with `--features ladybug` for build/query commands |
| Graphiti | optional research adapter | useful temporal graph pattern, but would add Python/service dependencies |
| GraphRAG | optional research adapter | useful for corpus-level community summaries after small deterministic graph slices work |
| Kuzu/LanceDB/BAML/Opik-style hybrid RAG | optional research pattern | useful as a reference for graph + vector + full-text retrieval, extraction observability, and guardrail evaluation after PRAXIS context-pack export works |

## Model Providers

Model providers should be abstracted behind extraction, classification,
summarization, and reasoning interfaces.

Required behavior:

- record provider and model alias;
- record prompt/template version;
- record input digests, output digests, and timestamps;
- preserve source grounding;
- support replay or fixture mode for tests;
- fail closed when citation requirements are not met.

## Extractor Distillation

Fine-tuned open models can become useful after DIALECTICA has enough reviewed
capsule data, but they are not foundation build dependencies.

Candidate pattern:

```text
teacher model + capsule schema
  -> machine extraction proposals
  -> human corrections and promotion decisions
  -> reviewed training rows
  -> LoRA/QLoRA extractor adapter
  -> benchmark gate
  -> optional extraction provider
```

Training-row shape:

- system: capsule extraction grammar, schema version, ontology blueprint, and
  review policy;
- user: source chunk, source metadata, prior span ids, and allowed output
  object types;
- assistant: typed proposal records only, never promoted truth.

Promotion rule:

A fine-tuned extractor can propose capsule records only after it beats the
prompted baseline on schema validity, source-span grounding, temporal
classification, graph-edge validity, rights/language rule extraction, and
reviewer acceptance. The teacher model remains the fallback until the eval
evidence and an ADR promote the adapter.

Current stance:

- treat Gemma-class LoRA/QLoRA recipes and Unsloth notebooks as implementation
  references, not architecture decisions;
- keep model ids and provider aliases out of core crates until provider
  interfaces exist;
- record model, adapter, dataset digest, prompt/template version, and eval
  report in every extraction receipt;
- never allow model output to self-promote without human review state.
