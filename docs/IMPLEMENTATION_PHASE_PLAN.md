# Implementation Phase Plan

Date: 2026-06-07

Status: active coding plan for the first working DIALECTICA capsule engine.

## Objective

Build DIALECTICA into a working capsule engine that can create, validate,
inspect, compile, store, and serve PRAXIS Capsules. The first working capability
must prove that a capsule carries the knowledge layers PRAXIS needs: sources,
time, ontology, graph, reasoning devices, language rules, agent guidance, review
state, rights, and output contracts.

## Phase 1: Executable Capsule Contract

Status: in progress, first slice implemented.

Delivered in the first coding pass:

- typed Rust bundle structs in `dialectica-capsule`;
- Serde load path for a bundle directory;
- JSON Schema export;
- deterministic validation findings;
- sourceability checks for graph edges;
- registered graph node and edge checks;
- temporal stale-claim warnings;
- human-review gate checks;
- golden policy expected-bundle fixture;
- CLI `validate`, `inspect`, and `schema-export`.

Remaining:

- expand validation to all required Lane A cases;
- validate the four single-file example capsules against a shared envelope
  contract;
- add checksum and signature placeholder validation.

Verification:

```powershell
cargo run -p dialectica-cli -- validate fixtures/golden-policy-capsule/expected-bundle
cargo run -p dialectica-cli -- inspect fixtures/golden-policy-capsule/expected-bundle
cargo run -p dialectica-cli -- schema-export schemas/capsule-0.1.0
cargo test --locked --workspace
```

## Phase 2: Deterministic Compiler

Goal: build a capsule from canonical records into a bundle directory.

Deliver:

- deterministic JSON and JSONL ordering;
- checksums for every layer;
- bundle digest;
- signature envelope placeholder;
- rejected-object lineage preservation;
- review gate enforcement before export;
- PRAXIS context-pack projection.

Acceptance:

- running the compiler twice against the same fixture produces the same digest;
- unreviewed promoted content fails;
- rejected objects remain visible in lineage but are hidden from PRAXIS context
  packs by default.

## Phase 3: Local Source-Pack Builder

Goal: guide an LLM or human through building capsule records from a small source
pack without cloud credentials.

Deliver:

- source-pack manifest;
- source span normalization;
- claim extraction input contract;
- human review correction file;
- reasoning-device selection file;
- language-rule review file;
- generated expected-bundle from source-pack plus review decisions.

Acceptance:

- a developer can run one local command to build the golden expected bundle;
- every generated claim points to a source span or review action;
- model-generated proposals cannot become promoted truth without review state.

## Phase 4: Store And Migrations

Goal: make Cloud SQL PostgreSQL the operational ledger.

Deliver:

- SQLx migrations for capsules, sources, claims, graph, ontology, reasoning,
  language, agent guidance, review, rights, exports, and eval reports;
- repository traits;
- idempotent job state;
- local Postgres workflow.

Acceptance:

- migrations run from an empty database;
- repository tests cover create/read/update paths for the golden capsule;
- bundle export can be reconstructed from database records.

## Phase 5: API And Task Handler

Goal: serve PRAXIS and queue capsule work.

Deliver:

- Axum API service;
- `GET /health`;
- `GET /version`;
- `POST /v1/capsule-jobs`;
- `GET /v1/capsule-jobs/{job_id}`;
- `GET /v1/capsules/{capsule_id}/manifest`;
- `GET /v1/capsules/{capsule_id}/graph-preview`;
- `GET /v1/capsules/{capsule_id}/praxis-context-pack`;
- Cloud Tasks-compatible task handler.

Acceptance:

- API boots locally;
- no cloud credentials are needed for local fixture mode;
- PRAXIS context pack includes source receipts, graph warnings, language rules,
  and audit receipts.

## Phase 6: Evals And PRAXIS Proof

Goal: prove the capsule improves policy work versus raw prompting.

Deliver:

- raw prompt baseline;
- loose-doc baseline;
- capsule-augmented answer;
- source fidelity score;
- temporal correctness score;
- reasoning-device adherence score;
- language-rule adherence score;
- failure ledger.

Acceptance:

- eval report states what improved and what failed;
- failures create actionable build tasks;
- PRAXIS can show capsule receipts for an answer.

## Phase 7: Cloud Run Staging

Goal: deploy the first service rail without Kubernetes.

Deliver:

- Dockerfile;
- Cloud Run API config;
- Cloud Run task-handler config;
- Cloud SQL connection settings;
- Cloud Storage bundle bucket;
- Cloud Tasks queues;
- Secret Manager references;
- GitHub Actions deploy job.

Acceptance:

- staging health endpoint returns version and dependency posture;
- task handler accepts authenticated queue calls;
- rollback is documented.

## Phase 8: Adapter Expansion

Goal: add optional graph, MCP, memory, and marketplace adapters only after the
core capsule engine works.

Candidate adapters:

- LadybugDB projection;
- JSON-LD export;
- MCP read-only capsule resource server;
- PRAXIS Firestore visibility mirror;
- marketplace listing and expert-pick workflows.

Acceptance:

- adapters read from canonical capsule records or bundles;
- no adapter becomes authoritative without an ADR, eval evidence, and runbook.
