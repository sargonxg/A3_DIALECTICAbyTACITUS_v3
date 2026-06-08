# DIALECTICA Agent Instructions

This repository builds DIALECTICA v3, the TACITUS capsule intelligence engine
for PRAXIS.

## Start Here

Before implementation work, read the active build path first:

1. `docs/SOURCE_OF_TRUTH.md`
2. `docs/CODING_LEDGER.md`
3. `docs/NEXT_CODE_BUILD_PLAN.md`
4. `docs/LLM_CONTEXT_EXTRACTION_ARCHITECTURE.md`
5. `docs/MISSING_WORK_AUDIT_2026_06_08.md`
6. `docs/CODE_AUDIT_2026_06_08.md`
7. `docs/CAPSULE_SPEC.md`
8. `docs/ENGINEERING_BASELINE.md`
9. `docs/IMPROVEMENT_GUIDELINES.md`
10. `docs/SCAFFOLD_AUDIT.md`

Use `docs/README.md` as the complete index when deeper product, graph,
ontology, deployment, PRAXIS, or research context is needed.

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
- Promoted capsules must include `graph/ladybug/capsule.lbug` plus its
  projection manifest and receipts. `graph.jsonld` remains the rebuildable
  semantic graph contract.
- PRAXIS Firestore remains canonical for PRAXIS user-facing capsule visibility,
  user library state, and cockpit UX state.
- Cloud Run is the first deployment target.
- Ladybug is the required embedded graph projection for promoted capsules.
  Other graph, semantic, vector, MCP, and memory systems remain adapters until
  an ADR changes that.
- Every derived claim needs provenance.
- Human review gates are part of the data model.
- Human-gated language is part of the data model.
- Agent guidance is a first-class bundle layer for PRAXIS workflow use.
- Ontology blueprints are capsule-specific. Actor/claim graphs are one profile,
  not the universal ontology for all capsules.
- LLM extraction is proposal-only until Rust validation and human review gates
  promote records.
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
- `cargo run -p dialectica-cli -- validate fixtures/canonical-capsules/conflict-situation-capsule`
- `cargo run -p dialectica-cli -- inspect fixtures/canonical-capsules/conflict-situation-capsule`
- `cargo run -p dialectica-cli -- ladybug-check fixtures/canonical-capsules/conflict-situation-capsule`
- `cargo run -p dialectica-cli --features ladybug -- ladybug-query fixtures/canonical-capsules/conflict-situation-capsule "MATCH (n:CapsuleNode) RETURN count(n) AS node_count;"`
- `cargo run -p dialectica-cli -- validate fixtures/golden-policy-capsule/expected-bundle`
- `cargo run -p dialectica-cli -- inspect fixtures/golden-policy-capsule/expected-bundle`
- `cargo run -p dialectica-cli -- ontology-plan fixtures/golden-policy-capsule/expected-bundle`
- `cargo run -p dialectica-cli -- schema-export schemas/capsule-3.0`
- `python -m compileall tools/python`
- `python -m unittest discover tools/python/tests`

Example capsule envelopes under `fixtures/example-capsules/` must keep the
same top-level bundle sections.

## External Actions

Do not push, deploy, publish packages, create cloud resources, or change
credentials without explicit user approval.
