# Operations

## Operational Objective

DIALECTICA should be boring to operate before it becomes powerful. The foundation build
should favor observable, retryable, reversible workflows over clever hidden
state.

## Required Runtime Endpoints

Every deployable service must expose:

- `/health`: process and local API contract status;
- `/version`: git SHA, build time, schema version, migration version.

Split `/healthz` and `/readyz` only when a deployed environment needs separate
liveness and dependency-readiness probes.

## Observability

Structured logs must include:

- `request_id`
- `tenant_id`
- `project_id`
- `capsule_id`
- `job_id`
- `source_id`
- `review_id`
- `trace_id`
- `component`
- `phase`
- `status`

Metrics should include:

- capsule jobs created;
- capsule jobs completed;
- capsule jobs failed;
- ingestion duration;
- compile duration;
- validation failures;
- review gate failures;
- eval pass rate;
- task retry count;
- database latency;
- storage write failures.

## Alerts

Initial alerts:

- API error rate above threshold;
- task handler retry spike;
- Cloud SQL connection exhaustion;
- capsule compile failure spike;
- bundle validation failure spike;
- storage write failures;
- eval promotion gate failures;
- stale capsule used by PRAXIS after warning threshold.

## Runbooks

### Failed Ingestion Job

1. Inspect job status and last phase.
2. Check source artifact availability and hash.
3. Check parser/extractor error.
4. Confirm whether retry is safe.
5. Requeue idempotent task or mark source as failed with reason.
6. Record decision in build ledger or incident notes.

### Failed Capsule Compile

1. Validate required records exist.
2. Check source ledger reference resolution.
3. Check review gate state.
4. Run local bundle validator against staged bundle.
5. Recompile after fixing canonical records.
6. Do not promote partial bundles.

### Stale Capsule Warning

1. Inspect temporal ledger.
2. Identify stale or superseded claims.
3. Check whether new source ingestion is available.
4. Recompile capsule or block PRAXIS usage depending on severity.
5. Preserve prior bundle for audit.

### PRAXIS Integration Failure

1. Check API health and version.
2. Check capsule manifest compatibility.
3. Check auth and tenant/project mapping.
4. Check PRAXIS adapter response shape.
5. Fall back to no-capsule workflow only if PRAXIS clearly surfaces that status.

## Backup and Recovery

Required:

- Cloud SQL backups;
- object storage versioning or retention policy;
- capsule bundle checksums;
- migration rollback notes;
- export path for promoted capsule bundles.

Capsules should remain inspectable even if optional graph or semantic adapters
are offline.
