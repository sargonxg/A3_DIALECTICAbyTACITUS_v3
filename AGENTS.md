# DIALECTICA Agent Instructions

This repository builds DIALECTICA v3, the TACITUS capsule intelligence engine
for PRAXIS.

## Start Here

Before implementation work, read:

1. `docs/DIALECTICA_v3_BUILD_INSTRUCTIONS.md`
2. `docs/SOURCE_OF_TRUTH.md`
3. `docs/FOUNDATION_BUILD.md`
4. `docs/TECH_BENCHMARK.md`
5. `docs/CAPSULE_FORMAL_MODEL.md`
6. `docs/CAPSULE_TYPES_AND_MARKETPLACE.md`
7. `docs/EMBEDDED_GRAPH_AND_SEMANTIC_LAYER.md`
8. `docs/EXPERT_REVIEW_AND_MARKETPLACE.md`
9. `docs/CAPSULE_BUILD_EXAMPLES.md`
10. `docs/CAPSULE_SPEC.md`
11. `docs/INTELLECTUAL_TOOLS.md`
12. `docs/API_CONTRACT.md`
13. `docs/DATA_MODEL.md`
14. `docs/ARCHITECTURE.md`
15. `docs/IMPLEMENTATION_BLUEPRINT.md`
16. `docs/AGENTIC_WORKFLOWS.md`
17. `docs/BUILD_LEDGER.md`

## Non-Negotiables

- PRAXIS is the visible product surface.
- DIALECTICA is the internal engine.
- The capsule bundle is the portable contract.
- PostgreSQL is the first operational source of truth.
- Cloud Run is the first deployment target.
- Graph and semantic engines are adapters until an ADR changes that.
- Every derived claim needs provenance.
- Human review gates are part of the data model.
- Embedded graph previews must be loadable by PRAXIS from the capsule bundle.
- Marketplace metadata must expose review level, rights, lineage, caveats, and
  freshness.
- Do not introduce Kubernetes, a graph database, or a vector database as required
  foundation build infrastructure without an ADR.

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
