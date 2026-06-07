# Agent Guide

This guide defines how future coding agents should work in this repository.

## First Files To Read

Before editing behavior, read:

1. `docs/DIALECTICA_v3_BUILD_INSTRUCTIONS.md`
2. `docs/SOURCE_OF_TRUTH.md`
3. `docs/MVP_DEFINITION.md`
4. `docs/TECH_BENCHMARK.md`
5. `docs/CAPSULE_FORMAL_MODEL.md`
6. `docs/CAPSULE_SPEC.md`
7. `docs/INTELLECTUAL_TOOLS.md`
8. `docs/API_CONTRACT.md`
9. `docs/DATA_MODEL.md`
10. `docs/ARCHITECTURE.md`
11. `docs/IMPLEMENTATION_BLUEPRINT.md`
12. `docs/AGENTIC_WORKFLOWS.md`
13. `docs/BUILD_LEDGER.md`

For deployment work, also read:

- `docs/DEPLOYMENT.md`
- `docs/OPERATIONS.md`
- `docs/SECURITY_AND_PRIVACY.md`

For PRAXIS integration work, also read:

- `docs/PRAXIS_INTEGRATION.md`
- `docs/PRAXIS_REPO_ALIGNMENT.md`
- `docs/EVAL_PLAN.md`

## Build Lanes

### Repo Cartographer

Purpose: map the current source of truth before changes.

Outputs:

- files touched;
- docs that apply;
- current code and test status;
- risks or conflicts.

### Capsule Spec Engineer

Purpose: maintain the capsule schema and fixture bundles.

No-touch unless explicitly requested:

- deployment infrastructure;
- PRAXIS production adapters.

Validation:

- schema validates;
- fixture bundles validate;
- compatibility notes are updated.

### Backend Engineer

Purpose: build Rust crates, API services, workers, and database integration.

Validation:

- `cargo fmt`
- `cargo clippy`
- `cargo test`
- contract tests
- migration tests when schema changes.

### Ingestion Engineer

Purpose: build parsing, source normalization, extraction receipts, and source
ledger records.

Validation:

- source fixtures round-trip;
- unsupported source formats fail clearly;
- every extraction has provenance.

### Graph and Semantic Engineer

Purpose: build ontology slices, graph slices, semantic mapping, and derived
views.

Constraints:

- PostgreSQL remains canonical for MVP;
- graph engines are adapters until an ADR changes this;
- every edge requires provenance and review state.

### Evals Engineer

Purpose: measure capsule quality and PRAXIS answer improvement.

Validation:

- eval fixtures are deterministic;
- raw LLM baseline and capsule-augmented output are compared;
- failures are attached to build ledger entries.

### Security Reviewer

Purpose: review auth, source trust, secrets, tenant isolation, artifact safety,
and supply chain risk.

Validation:

- no hardcoded secrets;
- service account scope is justified;
- capsule export does not leak private data;
- dependency risks are documented.

### Deployment Engineer

Purpose: build local, staging, and production deployment rails.

Validation:

- local contract runtime works without cloud credentials;
- Cloud Run deployment config is reproducible;
- rollback plan exists;
- deployment proof is attached to the build ledger.

## Working Rules

- Do not skip acceptance criteria in the build instructions.
- Do not silently change storage or deployment strategy.
- Do not introduce Kubernetes before an ADR approves it.
- Do not add a graph database as required infrastructure for the MVP.
- Do not remove source, temporal, or review fields to simplify implementation.
- Do not use mock success in runtime proof.
- Keep docs updated in the same PR as behavior changes.

## Pull Request Checklist

Every PR should answer:

- What capsule capability changed?
- What source/provenance behavior changed?
- What PRAXIS integration behavior changed?
- What schema or migration impact exists?
- What evals or tests prove this?
- What deployment or operations risk changed?
