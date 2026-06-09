# Coding Ledger

Status: active control file for the first functional DIALECTICA build.

Current audit result: executable v3 contract scaffold plus fixture-mode
source/proposal/review/promotion/compiler/archive/context/API contract and local
document-folder capsule builder verified on 2026-06-08; PDF/OCR/conversation
ingestion and the durable capsule-building service are not yet implemented. See
[Code Audit 2026-06-08](CODE_AUDIT_2026_06_08.md),
[Missing Work Audit 2026-06-08](MISSING_WORK_AUDIT_2026_06_08.md), and
[LLM Context Extraction Architecture](LLM_CONTEXT_EXTRACTION_ARCHITECTURE.md).

This ledger turns the architecture docs into a coding sequence. Keep it updated
whenever a crate, service, migration, fixture, or deployment gate changes.

## North Star

DIALECTICA builds PRAXIS Capsules: signed, portable, reviewable knowledge-work
objects that humans and AI agents can use interchangeably.

The first functional app is not complete until a developer can:

1. ingest a deterministic source pack;
2. create source-bound LLM proposal records with model receipts and review
   triggers;
3. build evidence, claim, episode, ontology, graph, reasoning, review, and
   runtime records;
4. build generated `agent_context.md` and `operations.md` for PRAXIS workflows;
5. validate the v3 `.capsule` contract;
6. block promotion when review is missing;
7. emit a PRAXIS context pack;
8. run a local API health check;
9. run a task-handler path without cloud credentials;
10. prove the result with contract tests and fixture evals.

## Active Scaffold

| Area | Path | Current status | First real implementation |
| --- | --- | --- | --- |
| Workspace | `Cargo.toml` | created | keep all crates in one Cargo workspace |
| Capsule contract | `crates/dialectica-capsule` | v3 package validator plus legacy structs, validation, and schema export implemented | expand validators and checksum/signature contract |
| Builder | `crates/dialectica-builder` | local text-document folder to source pack, proposals, caveated decisions, package, `.capsule`, PRAXIS pack, and import receipt implemented | add PDF/conversation ingestion and human-editable proposal/review cycles |
| Extractor | `crates/dialectica-extractor` | fixture-mode source-pack, proposal envelope, model receipt, build-plan, reviewer decision, promotion normalization, schema export, and review-trigger routing implemented | provider traits and live model orchestration |
| Compiler | `crates/dialectica-compiler` | deterministic fixture-mode v3 package writer, `.capsule` archive writer, PRAXIS context-pack exporter, and review-blocking tests implemented | harden canonical checksums, signature envelope, and generated-fixture comparison |
| Store | `crates/dialectica-store` | scaffolded with migration family names only | SQLx migrations and repository interfaces |
| Evals | `crates/dialectica-eval` | scaffolded with check result primitive only | fixture outcome, source-fidelity, temporal, reasoning, and PRAXIS comparison evals |
| CLI | `crates/dialectica-cli` | `welcome`, `build-docs`, `doctor`, `validate`, `inspect`, `ontology-plan`, `ladybug-check`, `source-pack-check`, `proposal-check`, `build-plan`, `review-check`, `promote-check`, `build-fixture`, `archive`, `context-pack`, `praxis-pack`, `mcp-config`, and `schema-export` implemented | add durable job commands after store exists |
| API | `services/dialectica-api` | fixture-backed Axum health, version, manifest, graph-preview, context-pack, and read-receipt routes implemented | store-backed jobs, auth, and artifact lookup |
| MCP | `services/dialectica-mcp` | Hardened Codex stdio MCP server with protocol router, output schemas, structuredContent, welcome, build, inspect, validate, status, archive, PRAXIS pack, ontology-plan, resources, and prompt implemented | add hosted/authenticated Streamable HTTP `/mcp` only after threat model, auth, tenant checks, and store-backed artifact IDs |
| Task handler | `services/dialectica-task-handler` | scaffolded binary that prints store env | Cloud Tasks-compatible HTTP handler |
| Contract tests | `tests/dialectica-contract-tests` | canonical v3 fixture, source-pack/proposal validation, review-gate routing, reviewer-decision validation, promotion normalization, generated compiler package, archive, context-pack, API route, and legacy migration tests implemented | deep-validator and store-backed job tests |

## 2026-06-08 Audit Result

The current repo can be trusted for contract-first coding, but not yet for
serving PRAXIS or building capsules from user material.

Verified as working:

- canonical v3 Situation Capsule fixture validates and inspects;
- unsupported top-level capsule types are rejected;
- legacy expected-bundle fixture still validates during migration;
- fixture source pack validates;
- fixture extraction proposals validate;
- fixture build plan routes Plus/promoted review gates before compilation;
- fixture reviewer decisions validate;
- fixture promotion normalization produces compiler-ready promoted records;
- fixture compiler writes a valid canonical v3 package;
- fixture archive writer emits `.capsule` with `mimetype` first;
- fixture context-pack export produces PRAXIS-readable JSON;
- local document-folder builder produces a compiled package, `.capsule`,
  `praxis-context-pack.json`, `praxis-import.json`, and build-source trace;
- Codex MCP stdio server advertises schema-backed capsule build, inspect,
  validate, status, archive, PRAXIS pack, ontology, resource, and prompt
  surfaces;
- fixture Axum API serves health, version, manifest, graph preview, context
  pack, and deterministic read receipts;
- schema export works;
- Rust workspace checks, clippy, tests, and Python auxiliary tests pass;
- CI now runs canonical v3 fixture validation before the legacy migration
  fixture.

Not yet built:

- PDF/OCR/conversation source-pack ingestion;
- live LLM extraction orchestration and provider clients;
- production-grade Merkle/checksum/signature envelope;
- store-backed HTTP API routes and build jobs;
- task-handler route;
- PostgreSQL migrations;
- document/PDF/user-discussion ingestion;
- ontology/semantic-layer creation workflow;
- human review queue;
- frontend integration in PRAXIS.

## Command Gate

These commands are mandatory before each commit once code exists:

```powershell
cargo fmt --all -- --check
cargo check --locked --workspace --all-targets
cargo clippy --locked --workspace --all-targets -- -D warnings
cargo test --locked --workspace
cargo run -p dialectica-cli -- welcome
cargo run -p dialectica-cli -- build-docs --type situation --input .\docs --out $env:TEMP\dialectica-doc-capsule --title "Local Situation Capsule" --workflow decision_brief
cargo run -p dialectica-cli -- inspect $env:TEMP\dialectica-doc-capsule\package
cargo run -p dialectica-cli -- mcp-config
cargo run -p dialectica-cli -- doctor
cargo run -p dialectica-cli -- validate fixtures/canonical-capsules/conflict-situation-capsule
cargo run -p dialectica-cli -- inspect fixtures/canonical-capsules/conflict-situation-capsule
cargo run -p dialectica-cli -- validate fixtures/golden-policy-capsule/expected-bundle
cargo run -p dialectica-cli -- inspect fixtures/golden-policy-capsule/expected-bundle
cargo run -p dialectica-cli -- ontology-plan fixtures/golden-policy-capsule/expected-bundle
cargo run -p dialectica-cli -- source-pack-check fixtures/golden-policy-capsule/source-pack/source_pack.json
cargo run -p dialectica-cli -- proposal-check fixtures/golden-policy-capsule/build_request.json fixtures/golden-policy-capsule/source-pack/source_pack.json fixtures/golden-policy-capsule/proposals
cargo run -p dialectica-cli -- build-plan fixtures/golden-policy-capsule/build_request.json fixtures/golden-policy-capsule/source-pack/source_pack.json fixtures/golden-policy-capsule/proposals
cargo run -p dialectica-cli -- review-check fixtures/golden-policy-capsule/build_request.json fixtures/golden-policy-capsule/source-pack/source_pack.json fixtures/golden-policy-capsule/proposals fixtures/golden-policy-capsule/review-decisions
cargo run -p dialectica-cli -- promote-check fixtures/golden-policy-capsule/build_request.json fixtures/golden-policy-capsule/source-pack/source_pack.json fixtures/golden-policy-capsule/proposals fixtures/golden-policy-capsule/review-decisions
cargo run -p dialectica-cli -- build-fixture fixtures/golden-policy-capsule --out $env:TEMP\dialectica-golden-v3
cargo run -p dialectica-cli -- validate $env:TEMP\dialectica-golden-v3
cargo run -p dialectica-cli -- archive $env:TEMP\dialectica-golden-v3 --out $env:TEMP\dialectica-golden-v3.capsule
cargo run -p dialectica-cli -- context-pack $env:TEMP\dialectica-golden-v3 --workflow conflict_map
cargo run -p dialectica-cli -- schema-export schemas/capsule-3.0
python -m compileall tools/python
python -m unittest discover tools/python/tests
```

Active Lane A/B gate:

```powershell
cargo run -p dialectica-cli -- validate fixtures/canonical-capsules/conflict-situation-capsule
cargo run -p dialectica-cli -- inspect fixtures/canonical-capsules/conflict-situation-capsule
cargo run -p dialectica-cli -- validate fixtures/golden-policy-capsule/expected-bundle
cargo run -p dialectica-cli -- inspect fixtures/golden-policy-capsule/expected-bundle
cargo run -p dialectica-cli -- ontology-plan fixtures/golden-policy-capsule/expected-bundle
cargo run -p dialectica-cli -- source-pack-check fixtures/golden-policy-capsule/source-pack/source_pack.json
cargo run -p dialectica-cli -- proposal-check fixtures/golden-policy-capsule/build_request.json fixtures/golden-policy-capsule/source-pack/source_pack.json fixtures/golden-policy-capsule/proposals
cargo run -p dialectica-cli -- build-plan fixtures/golden-policy-capsule/build_request.json fixtures/golden-policy-capsule/source-pack/source_pack.json fixtures/golden-policy-capsule/proposals
cargo run -p dialectica-cli -- review-check fixtures/golden-policy-capsule/build_request.json fixtures/golden-policy-capsule/source-pack/source_pack.json fixtures/golden-policy-capsule/proposals fixtures/golden-policy-capsule/review-decisions
cargo run -p dialectica-cli -- promote-check fixtures/golden-policy-capsule/build_request.json fixtures/golden-policy-capsule/source-pack/source_pack.json fixtures/golden-policy-capsule/proposals fixtures/golden-policy-capsule/review-decisions
cargo run -p dialectica-cli -- build-fixture fixtures/golden-policy-capsule --out $env:TEMP\dialectica-golden-v3
cargo run -p dialectica-cli -- validate $env:TEMP\dialectica-golden-v3
cargo run -p dialectica-cli -- archive $env:TEMP\dialectica-golden-v3 --out $env:TEMP\dialectica-golden-v3.capsule
cargo run -p dialectica-cli -- context-pack $env:TEMP\dialectica-golden-v3 --workflow conflict_map
cargo run -p dialectica-cli -- schema-export schemas/capsule-3.0
```

## Build Lanes

### Lane A: Capsule Contract

Goal: make the v3 `.capsule` schema real.

Acceptance contract: [Lane A Acceptance](LANE_A_ACCEPTANCE.md).

Deliver:

- v3 manifest/package validation: implemented;
- legacy manifest structs: implemented;
- source/evidence structs: partially implemented through legacy and v3 checks;
- temporal/episode structs: partially implemented through legacy and v3 checks;
- ontology/semantic layer structs: legacy implemented, v3 graph named-graph
  validation started;
- ontology blueprint planner and schema: implemented;
- embedded graph structs: legacy implemented, v3 `graph.jsonld` parser check
  implemented;
- runtime/agent guidance structs: legacy implemented, v3 runtime file check
  implemented;
- review ledger structs: implemented;
- rights profile structs: implemented;
- JSON Schema snapshots: implemented;
- validator error paths: implemented for the first sourceability, graph registry,
  review, manifest, rights, and temporal checks.

Done when:

- canonical v3 fixture validates locally;
- legacy fixture bundle validates locally during migration;
- invalid bundle reports precise paths;
- schema snapshots are committed;
- remaining Lane A cases are tracked before moving to store/API work.

Next validator expansion:

- enforce every claim-to-source span reference;
- enforce graph node/edge references across claims, episodes, reasoning, and
  review objects;
- enforce named graph and graph-lens profile compatibility;
- enforce review caveats and expiration state before promoted export;
- enforce generated `agent_context.md` and `operations.md` coverage for runtime
  instructions.

### Lane B: Fixture And CLI

Goal: make source packs, LLM proposals, review decisions, and deterministic CLI
builds possible without cloud credentials.

Deliver:

- `fixtures/golden-policy-capsule/source-pack/`: implemented with a validated
  fixture;
- source-pack schema: implemented;
- extraction proposal schema: implemented;
- model invocation receipt schema: implemented;
- human review decision fixture: implemented;
- review-trigger router: implemented for fixture proposals;
- `fixtures/example-capsules/*.example.json` validation;
- expected bundle records: implemented for the golden policy capsule;
- reviewer correction;
- CLI `validate`: implemented;
- CLI `inspect`: implemented;
- CLI `doctor`: implemented;

Done when:

- one command validates the source pack and proposal records;
- one command builds or validates the fixture;
- one command prints the capsule graph/review/source summary;
- unreviewed proposal records cannot enter promoted PRAXIS context.

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

Goal: assemble deterministic signed v3 package directories and `.capsule`
archives.

Deliver:

- v3 package directory writer;
- `.capsule` archive writer;
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
- Do not call a model provider from the first extractor tests; fixture
  proposals must validate locally first.
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

Follow [Next Code Build Plan](NEXT_CODE_BUILD_PLAN.md) and
[Improvement Guidelines](IMPROVEMENT_GUIDELINES.md):

1. harden deterministic checksum/signature semantics beyond fixture placeholders;
2. add byte-for-byte generated fixture comparison once canonical generated
   output is accepted;
3. add store-backed build job state and repository interfaces;
4. add source-pack ingestion adapters for documents, PDFs, and user/assistant
   discussion turns;
5. deepen v3 validation across claims, sources, graph, review, reasoning, and
   runtime records;
6. add provider traits and live model extraction behind proposal-only
   boundaries;
7. add PostgreSQL migrations for capsule build state and artifacts;
8. add task-handler routes after store-backed jobs exist;
9. start PRAXIS frontend integration only after the API/context-pack contract is
   stable.

Do not start Cloud Run, MCP, or PRAXIS production integration work until the
local fixture can also persist build state and expose store-backed artifacts.

Gap-control rule: every P0/P1 issue in
`docs/IMPROVEMENT_GUIDELINES.md` must be closed with a command, fixture, test,
or ADR before the related capability is described as functional.
