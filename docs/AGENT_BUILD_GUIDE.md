# Agent Build Guide

Date: 2026-06-07

Status: active guide for future agents building DIALECTICA for PRAXIS.

## Purpose

This file is the practical entrypoint for coding agents. It converts the product
vision into an execution order that protects the capsule contract, keeps PRAXIS
compatible, and avoids premature infrastructure complexity.

DIALECTICA is not a side memory feature. It is the engine that builds PRAXIS
Capsules: portable, signed, human-gated knowledge objects that humans and AI
agents can inspect and use interchangeably.

## Operating Thesis

Build the smallest complete capsule engine slice before adding advanced
adapters.

```text
source pack
  -> source ledger
  -> temporal ledger
  -> ontology blueprint
  -> ontology slice
  -> embedded graph
  -> reasoning devices
  -> language profile
  -> agent guidance
  -> review ledger
  -> signed bundle
  -> PRAXIS context pack
```

The first product proof is a PRAXIS answer that is more source-faithful, more
temporally disciplined, and more expert-shaped with a capsule than without one.

## Read Order

Before editing behavior, read these files in order:

1. `docs/SOURCE_OF_TRUTH.md`
2. `docs/CODING_LEDGER.md`
3. `docs/ENGINEERING_BASELINE.md`
4. `docs/LANE_A_ACCEPTANCE.md`
5. `docs/API_SLICE_1.md`
6. `docs/RESEARCH_LEDGER.md`
7. `docs/ONTOLOGY_BLUEPRINTS.md`
8. `docs/GRAPH_PROFILE_REGISTRY.md`
9. `docs/CAPSULE_STRUCTURE_GUIDE.md`
10. `docs/CAPSULE_SPEC.md`
11. `docs/PRAXIS_INTEGRATION.md`
12. `docs/SCAFFOLD_AUDIT.md`

Use `docs/DIALECTICA_v3_BUILD_INSTRUCTIONS.md` as reference context. If it
conflicts with a higher-priority file, follow the higher-priority file and open
an ADR if the conflict is material.

## Build Order

### Lane 1: Capsule Contract

Goal: make the bundle executable as a typed product contract.

Build:

- Rust structs for manifest, source ledger, temporal ledger, ontology slice,
  graph slice, reasoning devices, language profile, agent guidance, output
  contracts, review ledger, rights policy, eval report, and signature metadata.
- Capsule-specific ontology blueprint structs and JSON Schema export.
- JSON Schema export.
- Fixture snapshot validation.
- Deterministic validation errors with paths and help text.

Do not build ingestion, models, storage, or API before this lane has passing
tests.

### Lane 2: Golden Policy Fixture

Goal: prove the capsule can represent real policy work.

Build:

- a source pack with at least five source records;
- at least one stale or superseded source;
- at least one contested claim;
- at least one expert caveat;
- at least one stakeholder-analysis reasoning device;
- one capsule-specific ontology blueprint that explains why the selected graph
  profile fits the policy task;
- a language profile with approved, deprecated, and blocked language;
- an embedded graph with source-backed edges.

The fixture must be understandable by a human without running PRAXIS.

### Lane 3: Compiler

Goal: turn canonical records into a portable bundle.

Build:

- deterministic bundle directory export;
- checksums for every layer;
- manifest digest;
- signature envelope;
- local `validate`, `inspect`, and `export` CLI commands;
- graph preview output that PRAXIS can load without a graph database.

### Lane 4: PostgreSQL Store

Goal: make Cloud SQL PostgreSQL the operational source of truth for DIALECTICA.

Build tables for:

- capsule jobs and versions;
- source artifacts and spans;
- claims and temporal facts;
- graph nodes and edges;
- ontology terms and mappings;
- reasoning devices;
- language rules;
- agent guidance;
- review actions;
- rights policies;
- exports and signatures;
- eval reports.

Firestore remains the PRAXIS visibility mirror, not the DIALECTICA canonical
store.

### Lane 5: API Slice

Goal: serve PRAXIS without forcing a new PRAXIS product surface.

Build:

- `GET /health`;
- `GET /version`;
- `GET /v1/capsules/{id}/manifest`;
- `GET /v1/capsules/{id}/graph-preview`;
- `POST /v1/context-packs`;
- `GET /v1/capsules/{id}/receipts`;
- local auth stubs with clear production follow-up notes.

### Lane 6: Review And Eval Gates

Goal: prevent weak context from entering serious workflows.

Build:

- human review actions as durable records;
- promotion states;
- review caveats and expiry dates;
- source-fidelity evals;
- temporal evals;
- reasoning-transfer evals;
- raw versus capsule-augmented PRAXIS comparison.

No machine-generated claim should be promoted without source spans or a review
action.

### Lane 7: Deployment Rail

Goal: make the service deployable on Google Cloud without requiring Kubernetes.

Build:

- Dockerfile;
- local compose file;
- Cloud Run service config for `dialectica-api`;
- Cloud Run service config for `dialectica-task-handler`;
- Cloud Run jobs for backfill, eval, and reindex;
- Cloud Tasks queues;
- Cloud SQL connection config;
- Cloud Storage bundle bucket policy;
- Secret Manager integration notes;
- GitHub Actions deploy skeleton.

GKE is a later promotion path only after Cloud Run limits block validated
workloads.

## Agent Lanes

Use these roles when multiple agents work in parallel:

| Lane | Owns | Must update |
| --- | --- | --- |
| Repo cartographer | source-of-truth map, current status | `docs/SCAFFOLD_AUDIT.md` |
| Research scout | external sources and conclusions | `docs/RESEARCH_LEDGER.md` |
| Capsule spec engineer | bundle schema and fixtures | `docs/CAPSULE_SPEC.md`, fixtures |
| Ontology engineer | semantic layer planner, ontology families, local term mappings | `docs/ONTOLOGY_BLUEPRINTS.md`, `docs/CAPSULE_SPEC.md` |
| Graph engineer | graph vocabulary and constraints | `docs/GRAPH_PROFILE_REGISTRY.md` |
| Store engineer | Postgres schema and migrations | `docs/DATA_MODEL.md` |
| Compiler engineer | deterministic export and signatures | `docs/CODING_LEDGER.md` |
| API engineer | PRAXIS endpoints | `docs/API_SLICE_1.md`, `docs/API_CONTRACT.md` |
| PRAXIS adapter engineer | Firestore mirror and context pack | `docs/PRAXIS_INTEGRATION.md` |
| Eval engineer | quality gates | `docs/EVAL_PLAN.md` |
| Security reviewer | trust, privacy, source injection | `docs/SECURITY_AND_PRIVACY.md` |
| Deployment engineer | Cloud Run rail | `docs/DEPLOYMENT.md`, `docs/OPERATIONS.md` |

## Hard Boundaries

- Do not call the first delivery a prototype shortcut in docs or product copy.
- Do not remove source, temporal, review, rights, language, or agent guidance
  layers to simplify implementation.
- Do not make a graph database required infrastructure without an ADR.
- Do not make a vector database required infrastructure without an ADR.
- Do not introduce Kubernetes before Cloud Run has failed a measured need.
- Do not let Firestore become the only store for DIALECTICA build, review,
  graph, export, or signature state.
- Do not let model output promote itself.
- Do not add a PRAXIS top-level product surface unless PRAXIS explicitly asks
  for one.

## Validation Before Commit

Run the active gates before committing behavior:

```powershell
cargo fmt --all -- --check
cargo check --locked --workspace --all-targets
cargo clippy --locked --workspace --all-targets -- -D warnings
cargo test --locked --workspace
cargo run -p dialectica-cli -- doctor
cargo run -p dialectica-cli -- ontology-plan fixtures/golden-policy-capsule/expected-bundle
python -m compileall tools/python
python -m unittest discover tools/python/tests
```

For docs-only work, also check:

```powershell
git diff --check
rg -n -i "\bM[V]P\b|m[e]ssy policy context|m[e]ssy context|compile m[e]ssy|just another c[o]ntext" README.md docs AGENTS.md fixtures tools
```

If the terminology scan finds a banned phrase, rewrite it before committing.

## Research Memory Rule

When research changes a design decision, update `docs/RESEARCH_LEDGER.md` in
the same commit. Store:

- source URL;
- date checked;
- conclusion;
- DIALECTICA decision;
- refresh trigger.

Research without a repo artifact is not durable enough for future agents.
