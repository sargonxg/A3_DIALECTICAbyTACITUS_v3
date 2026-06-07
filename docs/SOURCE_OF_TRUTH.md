# DIALECTICA v3 Source of Truth

This file defines how to use the repository before implementation starts.

## Document Priority

When documents conflict, use this order:

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
11. `docs/DEPLOYMENT.md`
12. `docs/PRAXIS_INTEGRATION.md`
13. ADRs in `docs/decisions/`
14. Implementation notes in issues, PRs, and comments

If an implementation contradicts a higher-priority document, update the
document through an ADR or stop and ask for a product decision.

## Product Boundary

TACITUS is the company.

PRAXIS is the visible policy workbench, user cockpit, and agentic workflow
surface.

DIALECTICA is the internal capsule intelligence engine that compiles portable
PRAXIS Capsules.

AGON and KAIROS are future perception subsystems. They may feed DIALECTICA
through versioned adapters, but they are not required for the first working MVP.

## Naming Rules

Use these names in public product copy:

- PRAXIS
- PRAXIS Capsules
- Capsules
- Capsule AI
- Capsule Library

Use these names in internal engineering docs:

- DIALECTICA Engine
- capsule compiler
- capsule bundle
- source ledger
- review ledger
- semantic layer
- temporal layer

Do not market DIALECTICA as a standalone buyer-facing product until TACITUS
makes that decision explicitly.

## MVP Constraints

The MVP must prove that a capsule improves PRAXIS output compared with raw LLM
generation.

The MVP must include:

- a portable capsule bundle;
- source and provenance ledger;
- temporal claim model;
- minimal ontology and graph slices;
- human review ledger;
- PRAXIS integration contract;
- eval fixtures that measure whether the capsule helps.

The MVP must not require:

- Kubernetes;
- a dedicated graph database;
- a commercial vector database;
- a complex event-sourcing platform;
- a full expert-review marketplace;
- AGON or KAIROS as separate production systems.

## Engineering Rules

- Keep the capsule bundle canonical and portable.
- Keep PostgreSQL as the first operational source of truth.
- Keep graph and semantic engines as adapters, caches, or derived views until
  the capsule contract is stable.
- Store source artifacts immutably.
- Keep every derived object traceable to source spans, model runs, rules, or
  expert decisions.
- Treat expert review as data, not only as workflow UI.
- Do not silently add dependencies that change the deployment class.
- Every model-powered extraction must have an eval or fixture.
- Every schema change must include migration notes and compatibility impact.

## Implementation Entry Criteria

Before building service code, the repo needs:

- Rust workspace scaffold;
- capsule JSON schema;
- fixture capsule bundle;
- contract tests for manifest, source ledger, review ledger, and export;
- deployment skeleton for local Docker and Cloud Run;
- CI check for formatting, tests, and docs presence.

## First Definition of Done

Phase 1 is done when a developer can run one local command to:

1. create a valid sample capsule bundle from fixtures;
2. validate it against the capsule schema;
3. inspect the source ledger and review ledger;
4. export the bundle as a directory and compressed artifact;
5. run contract tests in CI.
