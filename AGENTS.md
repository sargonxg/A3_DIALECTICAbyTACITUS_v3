# DIALECTICA Agent Instructions

This repository builds DIALECTICA v3, the TACITUS capsule intelligence engine
for PRAXIS.

## Start Here

Before implementation work, read:

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

## Non-Negotiables

- PRAXIS is the visible product surface.
- DIALECTICA is the internal engine.
- The capsule bundle is the portable contract.
- PostgreSQL is the first operational source of truth.
- Cloud Run is the first deployment target.
- Graph and semantic engines are adapters until an ADR changes that.
- Every derived claim needs provenance.
- Human review gates are part of the data model.
- Do not introduce Kubernetes, a graph database, or a vector database as required
  MVP infrastructure without an ADR.

## Validation Expectations

When code exists, use the repo's own commands first. Expected future checks:

- `cargo fmt`
- `cargo clippy`
- `cargo test`
- capsule contract tests
- bundle fixture validation
- migration tests
- docs checks

## External Actions

Do not push, deploy, publish packages, create cloud resources, or change
credentials without explicit user approval.
