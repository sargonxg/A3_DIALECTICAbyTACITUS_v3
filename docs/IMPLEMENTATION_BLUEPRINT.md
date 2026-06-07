# Implementation Blueprint

## Build Order

The implementation should move in narrow, testable slices.

## Slice 1: Rust Workspace and Contract Types

Acceptance contract: `docs/LANE_A_ACCEPTANCE.md`.

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

- `cargo fmt --all -- --check`
- `cargo check --locked --workspace --all-targets`
- `cargo clippy --locked --workspace --all-targets -- -D warnings`
- `cargo test --locked --workspace`
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
- agent guidance generation;
- PRAXIS context pack generation.

## Slice 5: API and Task Handler

Acceptance contract: `docs/API_SLICE_1.md`.

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
- `GET /health`;
- `GET /version`.

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

Agent B: golden fixture expectations.

Agent C: graph profile registry and validation cases.

Agent D: read-only store/API/eval planning against the draft schema.

Merge order:

1. schema and fixture;
2. validator;
3. graph-slice validation;
4. store migrations;
5. compiler;
6. API;
7. evals;
8. deployment.

Store, API, and eval agents may research in parallel, but their code should not
merge until Lane A is complete.
