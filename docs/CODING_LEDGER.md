# Coding Ledger

Status: active control file for the first functional DIALECTICA build.

This ledger turns the architecture docs into a coding sequence. Keep it updated
whenever a crate, service, migration, fixture, or deployment gate changes.

## North Star

DIALECTICA builds PRAXIS Capsules: signed, portable, reviewable knowledge-work
objects that humans and AI agents can use interchangeably.

The first functional app is not complete until a developer can:

1. ingest a deterministic source pack;
2. build source, temporal, ontology, graph, reasoning, review, and rights
   records;
3. build agent guidance records for PRAXIS workflows;
4. validate the bundle contract;
5. block promotion when review is missing;
6. emit a PRAXIS context pack;
7. run a local API health check;
8. run a task-handler path without cloud credentials;
9. prove the result with contract tests and fixture evals.

## Active Scaffold

| Area | Path | Current status | First real implementation |
| --- | --- | --- | --- |
| Workspace | `Cargo.toml` | created | keep all crates in one Cargo workspace |
| Capsule contract | `crates/dialectica-capsule` | scaffolded with manifest/review primitives | add Serde structs and JSON Schema |
| Compiler | `crates/dialectica-compiler` | scaffolded with review-gated emit check | deterministic bundle writer and checksums |
| Store | `crates/dialectica-store` | scaffolded with migration families | SQLx migrations and repository interfaces |
| Evals | `crates/dialectica-eval` | scaffolded with check result primitive | fixture outcome and source-fidelity evals |
| CLI | `crates/dialectica-cli` | scaffolded with `doctor` | `validate`, `inspect`, `build-fixture` |
| API | `services/dialectica-api` | scaffolded binary | Axum health, manifest, context-pack routes |
| Task handler | `services/dialectica-task-handler` | scaffolded binary | Cloud Tasks HTTP handler |
| Contract tests | `tests/dialectica-contract-tests` | scaffolded | bundle contract and graph constraint tests |

## Command Gate

These commands are mandatory before each commit once code exists:

```powershell
cargo fmt --all -- --check
cargo check --locked --workspace --all-targets
cargo clippy --locked --workspace --all-targets -- -D warnings
cargo test --locked --workspace
cargo run -p dialectica-cli -- doctor
python -m compileall tools/python
python -m unittest discover tools/python/tests
```

Future gates:

```powershell
cargo run -p dialectica-cli -- validate fixtures/golden-policy-capsule
cargo run -p dialectica-cli -- inspect fixtures/golden-policy-capsule
```

## Build Lanes

### Lane A: Capsule Contract

Goal: make the capsule bundle schema real.

Acceptance contract: [Lane A Acceptance](LANE_A_ACCEPTANCE.md).

Deliver:

- manifest structs;
- source ledger structs;
- temporal ledger structs;
- ontology slice structs;
- embedded graph structs;
- agent guidance structs;
- review ledger structs;
- rights profile structs;
- JSON Schema snapshots;
- validator error paths.

Done when:

- fixture bundle validates locally;
- invalid bundle reports precise paths;
- schema snapshots are committed.

### Lane B: Fixture And CLI

Goal: make a deterministic capsule build possible without cloud credentials.

Deliver:

- `fixtures/golden-policy-capsule/source-pack/`;
- `fixtures/example-capsules/*.example.json` validation;
- expected bundle records;
- reviewer correction;
- CLI `validate`;
- CLI `inspect`;
- CLI `doctor`.

Done when:

- one command validates the fixture;
- one command prints the capsule graph/review/source summary.

### Lane C: Store And Migrations

Goal: make PostgreSQL the operational source of truth.

Deliver:

- SQLx migrations;
- repositories for capsules, sources, claims, graph, review, rights, exports;
- idempotent job writes;
- local Postgres setup notes.

Done when:

- migrations run forward from empty database;
- repository tests pass against a test database or fixture adapter.

### Lane D: Compiler

Goal: assemble deterministic signed bundle directories.

Deliver:

- bundle directory writer;
- deterministic JSON/JSONL ordering;
- checksums;
- manifest generation;
- PRAXIS context-pack generation;
- review-gate enforcement.

Done when:

- fixture build is reproducible;
- draft/unreviewed objects cannot be promoted;
- bundle digest changes only when canonical content changes.

### Lane E: API And Task Handler

Goal: expose the first working backend surface.

Acceptance contract: [API Slice 1](API_SLICE_1.md).

Deliver:

- `GET /health`;
- `GET /version`;
- `POST /v1/capsule-jobs`;
- `GET /v1/capsule-jobs/{job_id}`;
- `GET /v1/capsules/{capsule_id}/manifest`;
- `GET /v1/capsules/{capsule_id}/graph-preview`;
- `GET /v1/capsules/{capsule_id}/praxis-context-pack`;
- task-handler endpoint for queued compile work.

Done when:

- API boots locally;
- health route returns version and dependency posture;
- PRAXIS context-pack shape matches `docs/API_CONTRACT.md`.

### Lane F: Evals

Goal: prove capsules improve actual policy work.

Deliver:

- raw prompt baseline;
- loose-doc baseline;
- capsule-augmented output;
- citation fidelity checks;
- temporal correctness checks;
- reasoning-device adherence checks.

Done when:

- eval report clearly says what improved and what failed;
- failure cases update the ledger instead of being hidden.

## No-Touch Rules

- Do not add a required graph database.
- Do not add Kubernetes.
- Do not wire PRAXIS production calls before local fixture validation works.
- Do not add autonomous memory promotion.
- Do not let model extraction write canonical truth without review state.
- Do not add secrets to fixtures, logs, docs, or snapshots.
- Do not start broad multi-agent backend implementation before Lane A merges.

## Ledger Protocol

Every implementation commit should update this file or `docs/BUILD_LEDGER.md`
when it changes:

- crate ownership;
- command gates;
- schema status;
- fixture status;
- API surface;
- migration status;
- deployment path;
- known blockers.

## Current Next Action

Implement Lane A until `dialectica-capsule` owns the real bundle structs and
schema snapshots, then implement Lane B so the CLI can validate the golden
fixture before any cloud or PRAXIS integration work.
