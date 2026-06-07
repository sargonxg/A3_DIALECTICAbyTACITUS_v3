# DIALECTICA Agent Instructions

This repository builds DIALECTICA v3, the TACITUS capsule intelligence engine
for PRAXIS.

## Start Here

Before implementation work, read the active coding authority first:

1. `docs/SOURCE_OF_TRUTH.md`
2. `docs/ABOUT_DIALECTICA.md`
3. `docs/CODING_LEDGER.md`
4. `docs/ENGINEERING_BASELINE.md`
5. `docs/LANE_A_ACCEPTANCE.md`
6. `docs/API_SLICE_1.md`
7. `docs/GRAPH_PROFILE_REGISTRY.md`
8. `docs/CAPSULE_STRUCTURE_GUIDE.md`
9. `docs/GRAPH_ONTOLOGY_RESEARCH_NOTES.md`
10. `docs/REPOSITORY_CONCEPT_REVIEW.md`
11. `docs/GITHUB_PROFILE.md`
12. `docs/SCAFFOLD_AUDIT.md`
13. `docs/CAPSULE_SPEC.md`
14. `docs/API_CONTRACT.md`
15. `docs/DATA_MODEL.md`
16. `docs/ARCHITECTURE.md`
17. `docs/IMPLEMENTATION_BLUEPRINT.md`
18. `docs/AGENTIC_WORKFLOWS.md`
19. `docs/BUILD_LEDGER.md`

Use `docs/DIALECTICA_v3_BUILD_INSTRUCTIONS.md` as product/reference context.
When it conflicts with the active Rust-first coding docs above, follow
`docs/SOURCE_OF_TRUTH.md`, `docs/CODING_LEDGER.md`, and
`docs/ENGINEERING_BASELINE.md`.

## Non-Negotiables

- PRAXIS is the visible product surface.
- DIALECTICA is the internal engine.
- The capsule bundle is the portable contract.
- PostgreSQL is the first DIALECTICA operational source of truth for build,
  review, graph, export, and bundle state.
- PRAXIS Firestore remains canonical for PRAXIS user-facing capsule visibility,
  user library state, and cockpit UX state.
- Cloud Run is the first deployment target.
- Graph and semantic engines are adapters until an ADR changes that.
- Every derived claim needs provenance.
- Human review gates are part of the data model.
- Human-gated language is part of the data model.
- Agent guidance is a first-class bundle layer for PRAXIS workflow use.
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

Current command gate:

- `cargo fmt --all -- --check`
- `cargo check --locked --workspace --all-targets`
- `cargo clippy --locked --workspace --all-targets -- -D warnings`
- `cargo test --locked --workspace`
- `cargo run -p dialectica-cli -- doctor`
- `python -m compileall tools/python`
- `python -m unittest discover tools/python/tests`

Example capsule envelopes under `fixtures/example-capsules/` must keep the
same top-level bundle sections.

## External Actions

Do not push, deploy, publish packages, create cloud resources, or change
credentials without explicit user approval.
