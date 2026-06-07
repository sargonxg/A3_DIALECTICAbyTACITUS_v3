# Implementation Blueprint

## Build Order

The implementation should move in narrow, testable slices.

## Slice 1: Rust Workspace and Contract Types

Create:

- `crates/dialectica-capsule`
- `crates/dialectica-cli`

Implement:

- manifest structs;
- source ledger structs;
- temporal ledger structs;
- review ledger structs;
- bundle validation errors;
- JSON schema generation.

Validation:

- `cargo fmt`
- `cargo clippy`
- `cargo test`
- schema snapshot tests.

## Slice 2: Golden Fixture

Create:

- `fixtures/golden-policy-capsule/source-pack/`
- expected capsule bundle;
- expected validation report.

Implement CLI commands:

```powershell
cargo run -p dialectica-cli -- validate fixtures/golden-policy-capsule
cargo run -p dialectica-cli -- inspect fixtures/golden-policy-capsule
```

## Slice 3: PostgreSQL Store

Create:

- `crates/dialectica-store`
- migrations;
- repository interfaces;
- local Postgres instructions.

Implement:

- source ledger writes;
- capsule job state;
- review decisions;
- bundle export records.

## Slice 4: Compiler

Create:

- `crates/dialectica-compiler`

Implement:

- deterministic bundle assembly;
- checksums;
- manifest generation;
- PRAXIS context pack generation.

## Slice 5: API and Task Handler

Create:

- `services/dialectica-api`
- `services/dialectica-task-handler`

Implement:

- job creation;
- job polling;
- manifest endpoint;
- context-pack endpoint;
- review endpoint;
- export endpoint;
- health/version endpoints.

## Slice 6: Evals

Create:

- `crates/dialectica-eval`

Implement:

- contract evals;
- source-fidelity evals;
- temporal evals;
- PRAXIS outcome comparison harness.

## Slice 7: Cloud Run Staging

Create:

- Dockerfile;
- staging deployment config;
- Cloud SQL migration path;
- Cloud Tasks queue config;
- storage bucket naming convention.

## Engineering Constraints

- Do not add model-powered extraction before fixture and validation are stable.
- Do not add graph adapters before graph slice export works from Postgres.
- Do not add PRAXIS production integration before staging can serve one valid
  fixture capsule.
- Do not optimize for scale before source fidelity and review gates work.

## First Agent Work Pack

Agent A: capsule schema and CLI.

Agent B: Postgres data model and migrations.

Agent C: PRAXIS API contract and context pack.

Agent D: eval fixtures and test harness.

Merge order:

1. schema and fixture;
2. validator;
3. store;
4. compiler;
5. API;
6. evals;
7. deployment.
