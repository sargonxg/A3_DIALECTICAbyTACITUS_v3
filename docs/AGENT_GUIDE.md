# Agent Guide

This guide defines how future coding agents should work in this repository.

## First Files To Read

Before editing behavior, read:

1. `docs/SOURCE_OF_TRUTH.md`
2. `docs/AGENT_BUILD_GUIDE.md`
3. `docs/CODING_LEDGER.md`
4. `docs/ENGINEERING_BASELINE.md`
5. `docs/LANE_A_ACCEPTANCE.md`
6. `docs/API_SLICE_1.md`
7. `docs/RESEARCH_LEDGER.md`
8. `docs/GRAPH_PROFILE_REGISTRY.md`
9. `docs/CAPSULE_STRUCTURE_GUIDE.md`
10. `docs/IMPLEMENTATION_PHASE_PLAN.md`
11. `docs/SCAFFOLD_AUDIT.md`
12. `docs/FOUNDATION_BUILD.md`
13. `docs/TECH_BENCHMARK.md`
14. `docs/CAPSULE_FORMAL_MODEL.md`
15. `docs/CAPSULE_TYPES_AND_MARKETPLACE.md`
16. `docs/EMBEDDED_GRAPH_AND_SEMANTIC_LAYER.md`
17. `docs/EXPERT_REVIEW_AND_MARKETPLACE.md`
18. `docs/CAPSULE_BUILD_EXAMPLES.md`
19. `docs/CAPSULE_SPEC.md`
20. `docs/INTELLECTUAL_TOOLS.md`
21. `docs/API_CONTRACT.md`
22. `docs/DATA_MODEL.md`
23. `docs/ARCHITECTURE.md`
24. `docs/IMPLEMENTATION_BLUEPRINT.md`
25. `docs/AGENTIC_WORKFLOWS.md`
26. `docs/BUILD_LEDGER.md`
27. `docs/DIALECTICA_v3_BUILD_INSTRUCTIONS.md`

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

Current first lane:

- implement Lane A in `docs/CODING_LEDGER.md` before storage, API, or model
  extraction work.

### Backend Engineer

Purpose: build Rust crates, API services, workers, and database integration.

Validation:

- `cargo fmt --all -- --check`
- `cargo check --locked --workspace --all-targets`
- `cargo clippy --locked --workspace --all-targets -- -D warnings`
- `cargo test --locked --workspace`
- `cargo run -p dialectica-cli -- doctor`
- `python -m compileall tools/python`
- `python -m unittest discover tools/python/tests`
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

- PostgreSQL remains canonical for foundation build;
- graph engines are adapters until an ADR changes this;
- every edge requires provenance and review state.
- graph files must remain loadable by PRAXIS without a dedicated graph database.

### Research Scout

Purpose: refresh external sources and convert findings into durable build
memory.

Validation:

- `docs/RESEARCH_LEDGER.md` has source URL, checked date, conclusion,
  DIALECTICA decision, and refresh trigger;
- any design change also updates the affected spec, ADR, fixture, or backlog;
- no implementation dependency is promoted from research alone.

### Review And Marketplace Engineer

Purpose: build human gates, review ledgers, promotion rules, listing metadata,
forking, usage rights, and expert-pick flows.

Constraints:

- machine-generated outputs cannot be promoted without human review;
- inherited review does not automatically approve local forks;
- marketplace listings must expose freshness, caveats, rights, and lineage.

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
- Do not add a graph database as required infrastructure for the foundation build.
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
