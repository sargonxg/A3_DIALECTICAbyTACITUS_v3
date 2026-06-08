# Code Audit - 2026-06-08

Status: executable contract scaffold and fixture-mode extractor contract
verified; capsule-building service not yet implemented.

This audit checks the repository against the canonical v3 Capsule Spec and the
current goal: DIALECTICA must build portable PRAXIS Capsules that preserve
sourceability, temporality, ontology, semantic layers, embedded graph context,
reasoning guidance, expert review, language guidance, and PRAXIS runtime
instructions.

Related follow-up docs:

- [Missing Work Audit 2026-06-08](MISSING_WORK_AUDIT_2026_06_08.md);
- [LLM Context Extraction Architecture](LLM_CONTEXT_EXTRACTION_ARCHITECTURE.md);
- [ADR-007: LLM Extraction Proposal Boundary](decisions/ADR-007-llm-extraction-proposal-boundary.md).

## Readiness Scores

| Dimension | Score | Meaning |
| --- | ---: | --- |
| Repository coding readiness | 90/100 | Workspace, docs, fixtures, CI, and command gates are coherent enough for focused coding. |
| Capsule contract readiness | 76/100 | Canonical v3 package validation works for the first fixture, but deep cross-layer validation is still missing. |
| Capsule engine readiness | 45/100 | The repo can validate hand-authored packages and fixture source/proposal plans; it cannot yet ingest live material, compile, sign, store, or serve capsules. |
| Production service readiness | 20/100 | API, task handler, database, auth, observability, deployment, and PRAXIS integration are not yet functional. |

## What Is Actually Built

- `crates/dialectica-capsule` owns the current contract surface:
  - v3 constants;
  - `PraxisCapsuleManifest`;
  - `PraxisCapsulePackage`;
  - v3 directory validation;
  - legacy expected-bundle loading and validation;
  - schema export;
  - inspection summary support.
- `crates/dialectica-cli` provides executable developer commands:
  - `doctor`;
  - `validate`;
  - `inspect`;
  - `ontology-plan`;
  - `source-pack-check`;
  - `proposal-check`;
  - `build-plan`;
  - `review-check`;
  - `promote-check`;
  - `schema-export`.
- `crates/dialectica-extractor` provides the first fixture-mode input contract:
  - source packs and source spans;
  - extraction runs and model invocation receipts;
  - proposal records for claims, episodes, graph nodes, graph edges, ontology
    terms, reasoning devices, language rules, caveats, rights rules, and output
    rules;
  - Plus/promoted review-trigger routing;
  - reviewer decision validation;
  - promotion normalization;
  - build-plan typing;
  - schema export.
- `fixtures/canonical-capsules/conflict-situation-capsule/` is the first
  canonical v3 extracted Situation Capsule fixture.
- `fixtures/golden-policy-capsule/expected-bundle/` remains a legacy migration
  fixture and still validates.
- `tests/dialectica-contract-tests` proves:
  - the canonical v3 Situation Capsule validates;
  - unsupported top-level capsule types are rejected;
  - the legacy bundle still validates during migration;
  - sourceability, graph registry, review, temporal warning, and ontology
    blueprint checks still run for the legacy lane.
- `.github/workflows/docs.yml` now gates the canonical v3 fixture, generated v3
  schemas, legacy migration fixture, Rust workspace, and Python utilities.
- `schemas/capsule-3.0/` contains schema snapshots for v3 manifest/package
  records and legacy compatibility records exported by the current crate.

## What Is Not Built Yet

- No v3 capsule compiler writer exists.
- No live source-pack ingestion or normalized source-span builder from
  uploaded material exists.
- No live model-provider extraction, interactive review UI, or provider
  fallback policy exists.
- No `.capsule` zip archive writer exists.
- No deterministic Merkle root, checksum map, signature envelope, or signing
  policy exists.
- No PRAXIS context-pack exporter exists.
- No local HTTP API exists; `dialectica-api` only prints scaffold output.
- No Cloud Tasks-compatible handler exists; `dialectica-task-handler` only
  prints scaffold output.
- No PostgreSQL migrations or SQLx repositories exist.
- No document upload, PDF extraction, user discussion capture, or assistant
  conversation ingestion exists.
- No ontology or semantic-layer creation workflow exists beyond the legacy
  ontology-plan command.
- No review queue, reviewer UI, or human-gated approval workflow exists.
- No eval harness exists beyond the placeholder `EvalCheck` primitive.
- No PRAXIS frontend work is present in this repository. PRAXIS integration
  must be built in the PRAXIS repo after DIALECTICA exposes a stable local API
  and context-pack contract.

## Verification Run

The following commands were run locally on 2026-06-08:

```powershell
cargo fmt --all -- --check
cargo check --locked --workspace --all-targets
cargo clippy --locked --workspace --all-targets -- -D warnings
cargo test --locked --workspace
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
cargo run -p dialectica-cli -- schema-export $env:TEMP\dialectica-audit-schemas
python -m compileall tools/python
python -m unittest discover tools/python/tests
```

Result: all commands passed. The legacy golden fixture still emits the expected
stale-claim warning while returning `valid=true`.

## Findings

### P0 - Compiler Cannot Build The Canonical Capsule

The canonical v3 fixture is hand-authored. `dialectica-compiler` currently only
contains `can_emit_bundle()` for the legacy manifest review gate. This means
DIALECTICA can prove package shape, but cannot yet build the product object.

Required fix:

1. Implement deterministic v3 package writing.
2. Add `.capsule` archive writing.
3. Add contract tests that regenerate the canonical fixture byte-for-byte or
   compare a canonical normalized output tree.

### P0 - API Is A Scaffold, Not A Service

`services/dialectica-api` has no HTTP framework or routes. PRAXIS cannot call
DIALECTICA yet.

Required fix:

1. Add Axum.
2. Implement `GET /health` and `GET /version`.
3. Implement fixture-mode routes for manifest, graph preview, and context pack.
4. Add integration tests over local fixture data.

### P0 - Store Is A Placeholder

`crates/dialectica-store` has no migrations, repositories, or transactions.
Capsule build state, source spans, review actions, graph records, and exports
cannot yet be persisted.

Required fix:

1. Add SQLx migrations.
2. Add repository traits and fixture-backed tests.
3. Preserve bundle export from local files while database work matures.

### P1 - v3 Validation Is Too Shallow

Current validation checks required files, macro type, manifest layers, JSON
parseability, non-empty compiled views, and minimum Situation source/claim
records. It does not yet verify cross-layer integrity.

Required fix:

1. Validate every claim source span.
2. Validate claim, episode, graph, reasoning, review, and runtime references.
3. Validate JSON-LD named graphs and graph-lens profile compatibility.
4. Validate review state and caveats before promoted export.
5. Validate generated `agent_context.md` and `operations.md` against runtime
   expectations.

### P1 - Python Tools Remain Legacy-Auxiliary

Python tooling still exercises small legacy-style examples. It is useful for
support scripts only and must not be treated as a backend capability.

Required fix:

1. Keep Python auxiliary.
2. Add v3 fixture reporting only after Rust validation owns the contract.
3. Avoid a second capsule write path.

### P1 - PRAXIS Frontend Must Wait For A Stable Local Contract

The needed PRAXIS experience is clear: upload documents, build a User,
Situation, Tool, or Output Capsule, inspect source receipts and embedded graphs,
capture assistant discussions, review uncertain records, and import the
approved context pack into Ask PRAXIS. This repository cannot supply that yet.

Required fix:

1. Build local context-pack export in DIALECTICA.
2. Serve it from the local API.
3. Then implement the PRAXIS frontend integration inside the PRAXIS repo.
4. Mirror only PRAXIS-facing visibility state to Firestore; keep DIALECTICA
   canonical records in its bundle and PostgreSQL store.

## Build Ledger Update

The next code phase must be v3-first:

1. `dialectica-compiler`: typed source/proposal/review input plus deterministic
   v3 package writer.
2. `dialectica-compiler`: `.capsule` archive writer with deterministic entry
   order and explicit digest scope.
3. `dialectica-cli`: `build-fixture` that writes to an output directory.
4. `dialectica-capsule`: deep v3 cross-layer validator.
5. `dialectica-capsule`: PRAXIS context-pack type and schema export.
6. `dialectica-cli`: `context-pack <capsule-dir>`.
7. `dialectica-api`: local fixture-mode Axum service.
8. `dialectica-store`: SQLx migrations and repositories.
9. `dialectica-task-handler`: queued compile HTTP target.
10. PRAXIS repo: capsule builder/import/inspect UI after the local API and
    context-pack contract are stable.

## Claim Boundary

The repository may currently claim:

- it defines the canonical v3 capsule contract;
- it validates and inspects one canonical v3 Situation Capsule fixture;
- it retains legacy fixture compatibility during migration;
- it validates fixture-mode source packs and extraction proposals;
- it routes Plus/promoted proposal review gates before compilation;
- it has enough scaffolding to start coding the engine.

The repository must not yet claim:

- that DIALECTICA builds capsules from documents;
- that DIALECTICA calls live extraction models;
- that it has a working backend API;
- that it stores capsule state in PostgreSQL;
- that it serves PRAXIS;
- that it performs human-gated extraction or review;
- that it has a production deployment.
