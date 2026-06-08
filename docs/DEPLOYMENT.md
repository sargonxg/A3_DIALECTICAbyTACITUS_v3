# Deployment Plan

## Decision

Use **Cloud Run first** for DIALECTICA v3.

Use **GKE Autopilot later** only when evidence shows Cloud Run cannot support
the runtime shape.

## Why Cloud Run First

DIALECTICA's foundation build is a set of containerized APIs, task handlers, background jobs,
and workers around PostgreSQL and object storage. Cloud Run fits that shape with
less operational load than a Kubernetes cluster.

Current Google Cloud docs support this direction:

- Cloud Run services run containers on managed infrastructure and scale out for
  request-driven workloads.
- Cloud Run worker pools support non-HTTP pull-based background processing.
- Cloud Run integrates with Cloud SQL, Firestore, Cloud Storage, logging, and
  monitoring.
- Cloud Tasks can invoke HTTP handlers on Cloud Run, GKE, Compute Engine, or
  external endpoints with retry and scheduling controls.
- Google documents Cloud Run and GKE as portable container runtimes, which keeps
  a later migration realistic.

Source anchors:

- <https://docs.cloud.google.com/run/docs/overview/what-is-cloud-run>
- <https://docs.cloud.google.com/sql/docs/postgres/connect-run>
- <https://docs.cloud.google.com/tasks/docs/creating-http-target-tasks>
- <https://docs.cloud.google.com/kubernetes-engine/docs/concepts/gke-and-cloud-run>
- <https://docs.cloud.google.com/kubernetes-engine/docs/concepts/autopilot-overview>

## Target Cloud Topology

```text
GitHub
  |
  v
CI: test, lint, build image, scan
  |
  v
Artifact Registry
  |
  +--------------------+---------------------+
  |                    |                     |
  v                    v                     v
Cloud Run API     Cloud Run task handler   Cloud Run jobs
  |                    |                     |
  +---------+----------+----------+----------+
            |                     |
            v                     v
       Cloud SQL PostgreSQL   Cloud Storage
            |
            v
       PRAXIS adapter

Cloud Tasks queues dispatch ingestion, compile, review, and export work.
Pub/Sub may be added for fanout and pull-based worker pools.
Secret Manager provides runtime credentials and signing material.
Cloud Logging, Error Reporting, Trace, and Monitoring provide observability.
```

## Runtime Units

### `dialectica-api`

Cloud Run HTTP service.

Responsibilities:

- capsule job creation;
- capsule lookup and manifest retrieval;
- PRAXIS integration endpoints;
- review and promotion commands;
- health, readiness, and version endpoints.

### `dialectica-task-handler`

Cloud Run HTTP service for Cloud Tasks.

Responsibilities:

- execute idempotent ingestion steps;
- execute compile steps;
- execute review promotion side effects;
- write job receipts and retry-safe status updates.

### Cloud Run Jobs

Initial jobs:

- `capsule-backfill`: rebuild capsule records from fixture or source batches;
- `capsule-eval`: run eval suites against capsule bundles;
- `source-reindex`: regenerate embeddings or graph slices after schema changes.
- `graph-projection-eval`: compare PostgreSQL, JSON-LD, and required embedded
  Ladybug graph projections against fixture and PRAXIS retrieval outcomes.

### Optional Worker Pools

Use Cloud Run worker pools only when the engine needs pull-based background
workers with no public HTTP endpoint, for example Pub/Sub pull consumers or
stream processors.

## Data Plane

### PostgreSQL

Cloud SQL PostgreSQL is the first operational source of truth.

Initial database areas:

- identity and tenant records;
- capsule jobs and states;
- source ledger;
- extraction runs;
- entities and claims;
- temporal facts;
- graph edges;
- ontology mappings;
- review decisions;
- bundle manifests;
- embeddings if pgvector is enabled.

### Object Storage

Cloud Storage stores:

- source artifacts;
- normalized text;
- capsule bundle directories;
- compressed capsule exports;
- checksum and signature files;
- eval artifacts.

### Secrets

Secret Manager stores:

- model provider keys;
- database credentials if not using IAM-only flows;
- signing keys;
- connector credentials;
- PRAXIS integration secrets.

Do not store secrets in GitHub, source fixtures, or capsule bundles.

## Why Not Kubernetes First

Kubernetes adds value when the engine needs:

- complex multi-service orchestration;
- long-running stateful services;
- custom autoscaling behavior;
- advanced networking policies;
- operator-managed infrastructure;
- hardware-specific scheduling;
- sidecars and service mesh requirements;
- stronger workload separation than Cloud Run provides.

The foundation build does not need those by default. Starting on Kubernetes would increase
surface area before the capsule contract, evals, and PRAXIS integration have
proven the product.

## When To Move To GKE Autopilot

Promote selected workloads to GKE Autopilot if at least one condition becomes
true:

- ingestion workers require persistent warm pools with custom autoscaling;
- graph services need long-lived memory, special topology, or co-scheduled
  components;
- optional graph service adapters require persistent warm services rather than
  task-scoped Cloud Run jobs; embedded Ladybug projection builds remain
  task/job-scoped and do not require a running graph service;
- policy teams require private multi-tenant network isolation not practical in
  Cloud Run;
- model-adapter workloads need GPUs or custom hardware scheduling;
- Cloud Run task or job limits block validated production workloads;
- local Kubernetes manifests become materially simpler than Cloud Run configs.

If this happens, keep Cloud Run for public HTTP surfaces if it still fits, and
move only the workloads that need Kubernetes.

## Environment Strategy

Use three environments:

- `local`: Docker Compose or local binaries, local Postgres, fixture bundles;
- `staging`: Cloud Run, staging Cloud SQL, staging storage bucket, test PRAXIS
  adapter;
- `production`: Cloud Run, production Cloud SQL, production storage, locked
  service accounts, release approvals.

Every environment must expose:

- `/health`: process and local API contract status;
- `/version`: git SHA, build time, schema version, and migration version;
- capsule schema version
- database migration version

Add separate `/healthz` and `/readyz` endpoints only when staging or production
probes need distinct liveness and dependency-readiness checks.

## Deployment Phases

### Phase D0: Local Contract Runtime

- Rust workspace builds locally.
- Sample capsule bundle validates.
- Contract tests run without cloud credentials.
- Docker image builds locally.

### Phase D1: Cloud Run Staging

- API deploys to staging.
- Task handler deploys to staging.
- Cloud SQL staging instance is reachable.
- Staging bucket stores a fixture bundle.
- Cloud Tasks can dispatch one idempotent task.

### Phase D2: PRAXIS Staging Integration

- PRAXIS can request a capsule manifest.
- PRAXIS can inspect source and review status.
- PRAXIS can use one capsule in a controlled answer workflow.
- Eval report compares raw and capsule-augmented output.

### Phase D3: Production Pilot

- One policy capsule type is production-gated.
- Human review gate is mandatory.
- Observability dashboards are live.
- Release rollback path is documented.

## Infrastructure As Code

Use Terraform or OpenTofu once the resource names stabilize.

Initial modules:

- Artifact Registry repository;
- Cloud Run services;
- Cloud Run jobs;
- Cloud SQL PostgreSQL;
- Cloud Storage buckets;
- Cloud Tasks queues;
- Pub/Sub topics if needed;
- service accounts and IAM;
- Secret Manager secrets;
- monitoring alerts.

Do not create cloud resources from ad hoc shell commands without capturing the
intended final state in `infrastructure/`.

## Release Gates

Before any production deploy:

- unit tests pass;
- contract tests pass;
- capsule fixture validates;
- migrations are reviewed;
- secrets are loaded through Secret Manager;
- service accounts are least-privilege;
- image vulnerability scan is reviewed;
- eval report is attached to the release;
- rollback plan is written in the build ledger.
