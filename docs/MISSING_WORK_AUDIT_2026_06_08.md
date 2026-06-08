# Missing Work Audit - 2026-06-08

Status: build-gap audit after canonical v3 alignment, Ladybug projection
validation, and the fixture-mode source/proposal/review/promotion/compiler/API
implementation.

The repository is coherent and locally verified as a contract scaffold. It is
not yet the production capsule-building engine. This audit lists what remains
before DIALECTICA can ingest live policy material, use LLMs to extract context,
route durable human review, build graphs and semantic layers at scale, persist
`.capsule` artifacts, and serve production PRAXIS traffic.

## Current Truth

| Surface | State |
| --- | --- |
| Canonical v3 fixture validation | Working |
| Legacy migration fixture validation | Working |
| CLI `doctor`, `validate`, `inspect`, `ontology-plan`, `ladybug-check`, `source-pack-check`, `proposal-check`, `build-plan`, `review-check`, `promote-check`, `build-fixture`, `archive`, `context-pack`, `schema-export` | Working |
| Rust contract tests | Working |
| Fixture source-pack contract | Working |
| Fixture extraction proposals and model receipts | Working |
| Fixture review-trigger router | Working |
| Fixture reviewer decisions and promotion normalization | Working |
| Live source-pack ingestion | Missing |
| Live LLM extraction orchestration | Missing |
| Interactive review UI and live reviewer workflow | Missing |
| Deterministic fixture-mode v3 compiler | Working |
| `.capsule` archive writer | Working |
| PRAXIS context-pack export | Working |
| PostgreSQL migrations | Missing |
| Fixture-backed HTTP API | Working |
| Store-backed HTTP API | Missing |
| Task handler | Missing |
| Eval harness | Missing |
| PRAXIS frontend integration | Missing |

## P0 Build Gaps

These block the first real capsule-building loop.

### 1. Source Pack And Span Ledger

Implemented for fixture mode:

- `SourcePack` Rust type;
- `SourceDocument` Rust type;
- `SourceSpan` Rust type;
- local source-pack fixture;
- stable hash and locator rules;
- prompt-injection quarantine metadata;
- rights/access metadata.

Still missing:

- real document/PDF/conversation ingestion;
- full source text extraction;
- binary artifact staging;
- PDF/document/conversation source categories;
- cryptographic validation of artifact hashes beyond fixture-shape checks.

Acceptance:

- one local source pack can be loaded without cloud credentials;
- every span has stable id, locator, source id, hash, language, and access
  status;
- source pack validation fails on missing hashes or unstable locators.

### 2. LLM Extraction Proposal Contract

Implemented for fixture mode:

- `dialectica-extractor` crate;
- `ExtractionRun` type;
- `ModelInvocationReceipt` type;
- `ExtractionProposal` envelope;
- proposal types for claims, episodes, graph nodes, graph edges, ontology terms,
  reasoning devices, language rules, output rules, caveats, and rights rules;
- deterministic JSON Schema export for proposal records;
- local proposal fixture records;
- review-trigger routing for Plus/promoted proposal sets.

Still missing:

- model provider traits and clients;
- live source-bound prompt orchestration;
- extraction retry and error envelopes;
- provider fallback policy;
- eval benchmark gates before any fine-tuned or cheaper extractor replaces a
  stronger teacher model.

Acceptance:

- fixture proposals can be loaded without calling a model;
- every proposal has source spans, model receipt, confidence, uncertainty, and
  review triggers;
- proposals cannot be compiled as canonical records directly.

### 3. Review Router And Human Gates

Implemented for fixture mode:

- deterministic review trigger rules;
- reviewer decision file;
- promotion rules from proposal to compiler-ready promoted records;
- caveat propagation into promoted records;
- rejected-object lineage preservation in promotion summaries.

Still missing:

- review coverage matrix;
- caveat and expiry propagation;
- interactive review queue and reviewer UI.

Acceptance:

- causal graph edges, language rules, rights rules, expert reasoning rules, and
  material claims require review before promoted export;
- rejected records remain visible in lineage but hidden from default PRAXIS
  context packs;
- approved-with-caveats records carry caveats into `agent_context.md` and
  context-pack JSON.

### 4. Deterministic Compiler

Implemented for fixture mode:

- v3 package writer;
- stable JSON/JSONL writer;
- `.capsule` archive writer;
- compiler receipt;
- deterministic generated views.

Still missing:

- production-grade Merkle root and checksum map;
- signature envelope;
- byte-for-byte generated canonical fixture comparison.

Acceptance:

- same input records generate byte-identical canonical output;
- missing review blocks promoted output;
- canonical fixture can be regenerated from source pack, proposal records, and
  reviewer decisions.

### 5. Deep v3 Validator

Missing:

- claim-to-source span validation;
- episode-to-claim/source validation;
- graph node/edge reference validation;
- JSON-LD named graph checks;
- ontology blueprint compatibility checks;
- reasoning device reference checks;
- language/review/rights/runtime checks;
- generated `agent_context.md` and `operations.md` coverage checks.

Acceptance:

- invalid cross-layer references fail with precise paths;
- stale or contested records warn but do not disappear;
- graph edges that affect reasoning cannot pass without source or review
  grounding.

### 6. PRAXIS Context Pack

Implemented for fixture mode:

- `PraxisContextPack` Rust type in `dialectica-compiler`;
- CLI `context-pack`;
- source receipt compaction;
- graph focus-node projection;
- review caveat projection;
- language and reasoning guidance projection.

Still missing:

- stable context-pack type in `dialectica-capsule`;
- JSON Schema export after the type moves to the stable contract crate;
- store-backed capsule lookup.

Acceptance:

- PRAXIS can consume context-pack JSON without a graph database;
- context pack excludes rejected and expired records by default;
- every included item has a provenance receipt.

## P1 Build Gaps

These are required for a usable local service after the P0 loop works.

### 1. Local Axum API

Implemented for fixture mode:

- `GET /health`;
- `GET /version`;
- capsule manifest route;
- graph preview route;
- PRAXIS context-pack route;
- deterministic read-receipt route.

Still missing:

- deterministic error envelope;
- request id and tracing.

### 2. PostgreSQL Store

Missing:

- SQLx migrations for sources, spans, proposals, reviews, claims, episodes,
  graph nodes, graph edges, ontology terms, reasoning devices, language rules,
  rights rules, exports, jobs, and eval reports;
- repository traits;
- transaction boundaries;
- idempotency keys.

### 3. Task Handler

Missing:

- authenticated Cloud Tasks-compatible handler;
- job state transitions;
- retry and idempotency rules;
- compile/export/eval jobs;
- dead-letter runbook.

### 4. Document, PDF, And Conversation Intake

Missing:

- upload staging;
- file hashing;
- text extraction;
- PDF page locators;
- user/assistant discussion capture;
- source trust classification;
- private-data flags;
- rights/access gates.

### 5. Eval Harness

Missing:

- source fidelity eval;
- temporal correctness eval;
- graph/reference integrity eval;
- reasoning-device adherence eval;
- language-rule adherence eval;
- raw prompt versus capsule comparison;
- failure ledger.

### 6. PRAXIS Frontend Integration

Missing in the PRAXIS repo:

- capsule build/import view inside the existing PRAXIS cockpit or repository
  surfaces;
- source upload;
- capsule build job status;
- graph preview;
- source receipt inspector;
- review queue;
- context-pack import;
- Firestore visibility mirror for PRAXIS user-facing library state.

## P2 Expansion

These are valuable after the local loop and API are real.

- broadened Ladybug graph algorithms beyond the required embedded projection;
- optional Oxigraph/RDF projection;
- optional MCP read-only capsule resource server;
- optional marketplace listing and expert-pick workflows;
- optional fine-tuned extractor after enough reviewed proposals exist;
- optional Graphiti/Zep-style temporal adapter after eval evidence;
- optional Cloud Run worker pools after simple Cloud Run service limits are
  proven.

## Missing Rust Work Packages

| Work package | Likely files |
| --- | --- |
| Source pack schema | implemented in `crates/dialectica-extractor`, `fixtures/golden-policy-capsule/source-pack/`, `schemas/capsule-3.0/source_pack.schema.json` |
| Extractor crate | implemented in `crates/dialectica-extractor`, `Cargo.toml`, `tests/dialectica-contract-tests` |
| Proposal schemas | implemented in `crates/dialectica-extractor`, `schemas/capsule-3.0/` |
| Review router | fixture-mode router, reviewer decisions, and promotion normalization implemented in `crates/dialectica-extractor`; live review UI still pending |
| Compiler writer | implemented for fixture mode in `crates/dialectica-compiler` |
| Context pack | implemented for fixture mode in `crates/dialectica-compiler`, `crates/dialectica-cli` |
| API routes | fixture mode implemented in `services/dialectica-api` |
| Store migrations | `crates/dialectica-store/migrations/` |
| Task handler | `services/dialectica-task-handler` |
| Eval checks | `crates/dialectica-eval` |

## Next Implementation Sequence

1. Expand v3 validator cross-layer checks.
2. Harden checksum/signature envelope.
3. Add generated-fixture comparison.
4. Add PostgreSQL migrations.
5. Add store-backed API jobs and artifact lookup.
6. Add live ingestion and model-provider orchestration.
7. Add task-handler routes.
8. Add PRAXIS frontend integration after the API/context-pack contract is
   stable.

## Claim Boundary

The repository may claim that it defines and validates the first v3 capsule
contract and can compile reviewed fixture records into local v3 packages,
archives, context packs, and fixture API responses. It must not claim that it
extracts live context, builds capsules from documents, serves production PRAXIS
traffic, or performs durable human-gated review until those capabilities exist
as commands, tests, routes, and fixtures.
