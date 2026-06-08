# Scaffold Audit

Date: 2026-06-08

Readiness: **88/100 for coding readiness; 38/100 for capsule-engine readiness;
20/100 for app-production readiness**.

The repository has begun the first functional implementation pass. It is not
yet a working backend or capsule-building service, but the capsule contract is
now executable in two lanes: the Rust crate can validate a canonical v3
extracted `.capsule` package, and it can still load the legacy expected-bundle
directory while the compiler migrates.

## Evidence Checked

- `README.md`
- `AGENTS.md`
- `assets/dialectica-mark.svg`
- `assets/README.md`
- `Cargo.toml`
- `.github/workflows/docs.yml`
- `assets/agent-build-flow.svg`
- `assets/research-ledger.svg`
- `crates/`
- `services/`
- `fixtures/`
- `tests/`
- `docs/CODING_LEDGER.md`
- `docs/IMPLEMENTATION_BLUEPRINT.md`
- `docs/API_CONTRACT.md`
- `docs/CAPSULE_SPEC.md`
- `docs/DATA_MODEL.md`
- `docs/DEPLOYMENT.md`
- `docs/RESEARCH_LEDGER.md`
- `docs/AGENT_BUILD_GUIDE.md`
- `docs/BUILD_LEDGER.md`
- `docs/IMPLEMENTATION_PHASE_PLAN.md`
- `docs/CODE_AUDIT_2026_06_08.md`
- `docs/MISSING_WORK_AUDIT_2026_06_08.md`
- `docs/LLM_CONTEXT_EXTRACTION_ARCHITECTURE.md`
- `docs/decisions/ADR-007-llm-extraction-proposal-boundary.md`
- `schemas/capsule-3.0/`
- `schemas/capsule-0.1.0/`
- `fixtures/canonical-capsules/conflict-situation-capsule/`
- `.env.example`

## Ready

- Repository identity is clear: DIALECTICA is the capsule intelligence engine
  for PRAXIS.
- Visual identity assets are present in `assets/`.
- The GitHub front door now has a simpler repository mark and a shorter active
  build path instead of forcing every reader through the full documentation
  library.
- Documentation has a source-of-truth order.
- Cargo workspace is present and testable.
- `dialectica-capsule` now owns v3 package validation, legacy bundle structs,
  validation findings, bundle loading, inspection summaries, ontology
  blueprints, and JSON Schema export.
- `dialectica-cli` now supports `validate`, `inspect`, `ontology-plan`, and
  `schema-export`.
- Canonical v3 conflict Situation Capsule fixture exists under
  `fixtures/canonical-capsules/conflict-situation-capsule/`.
- GitHub CI now validates and inspects the canonical v3 fixture before the
  legacy migration fixture.
- `.env.example` exposes `DIALECTICA_CAPSULE_SPEC_VERSION=3.0` and keeps the
  legacy `0.1.0` bundle version separate.
- `docs/CODE_AUDIT_2026_06_08.md` records the current executable state and the
  P0/P1 build gaps.
- `docs/MISSING_WORK_AUDIT_2026_06_08.md` lists the source-pack, extraction,
  review, compiler, API, store, eval, and PRAXIS frontend gaps.
- `docs/LLM_CONTEXT_EXTRACTION_ARCHITECTURE.md` defines the proposal-only LLM
  extraction pipeline.
- ADR-007 records that LLM extraction cannot write canonical truth directly.
- Legacy golden policy capsule expected-bundle exists under
  `fixtures/golden-policy-capsule/expected-bundle/`.
- Initial crate and service boundaries exist.
- Contract-test crate exists.
- Cloud Run first deployment decision is recorded.
- PostgreSQL-first operational store decision is recorded.
- Embedded graph, semantic layer, review, rights, and marketplace concepts are
  documented before implementation.
- Capsule-specific ontology blueprints are documented in
  `docs/ONTOLOGY_BLUEPRINTS.md`; actor/claim graphs are treated as one profile,
  not the universal capsule ontology.
- Command gates are defined in `docs/CODING_LEDGER.md`.
- Lane A acceptance is explicit in `docs/LANE_A_ACCEPTANCE.md`.
- API Slice 1 is explicit in `docs/API_SLICE_1.md`.
- Graph vocabulary is centralized in `docs/GRAPH_PROFILE_REGISTRY.md`.
- Capsule v3 layers, runtime contract, and generated agent views are explained in
  `docs/CAPSULE_STRUCTURE_GUIDE.md`.
- Research-backed graph and ontology adapter decisions are recorded in
  `docs/GRAPH_ONTOLOGY_RESEARCH_NOTES.md`.
- Source links, academic-paper conclusions, official documentation anchors, and
  refresh triggers are now recorded in `docs/RESEARCH_LEDGER.md`.
- Future-agent coding order is now explicit in `docs/AGENT_BUILD_GUIDE.md`.
- Active gap-control and improvement standards are now recorded in
  `docs/IMPROVEMENT_GUIDELINES.md`.
- Four small example capsule envelopes exist under
  `fixtures/example-capsules/`.
- Python tooling is auxiliary and governed by `docs/PYTHON_TOOLING.md`.
- GitHub repository metadata is aligned with `docs/GITHUB_PROFILE.md`:
  Apache-2.0 license, TACITUS homepage, PRAXIS/capsule topics, and a concise
  description are visible on the remote repository.

## Not Ready Yet

- Capsule validation breadth is incomplete; v3 validation currently checks
  package shape, manifest layer vocabulary, type boundary, non-empty generated
  views, JSON parseability, and minimum Situation claim/source records.
- No source-pack ingestion, document upload, PDF extraction, user discussion
  capture, or assistant conversation ingestion exists.
- No `dialectica-extractor` crate, extraction proposal schema, model invocation
  receipt, or review-trigger router exists.
- No ontology or semantic-layer creation workflow exists beyond the legacy
  `ontology-plan` helper.
- Golden fixture source pack is still only a placeholder; the expected bundle is
  committed, but the compiler does not generate either the legacy bundle or v3
  package yet.
- Example single-file capsule envelopes still need shared-envelope validation.
- PostgreSQL migrations do not exist.
- API routes are scaffold binaries, not HTTP services.
- Cloud Tasks handler is a scaffold binary, not an HTTP target.
- `.capsule` archive assembly, signing, and Merkle-root logic are not
  implemented.
- Ontology blueprint persistence inside signed bundles is not implemented; the
  planner exists as a CLI and schema contract.
- PRAXIS context-pack export is not implemented.
- PRAXIS frontend integration is not implemented in this repository and should
  be done in the PRAXIS repo only after a stable DIALECTICA context-pack/API
  contract exists.
- Evals are only planned.
- Cloud infrastructure has no Terraform/OpenTofu state yet.
- API service still has no HTTP framework.
- Python tooling is intentionally small and should not be treated as backend
  capability.

## Blockers Before Calling It Functional

1. Implement source-pack and extraction proposal records.
2. Add `dialectica-extractor` with model receipts and review-trigger routing.
3. Implement source-pack to v3 `.capsule` package generation.
4. Add deterministic `.capsule` archive assembly, Merkle-root/signature logic,
   and compiler receipts.
5. Expand Lane A validation to every required acceptance case, including
   ontology blueprint compatibility checks.
6. Validate the four example capsule envelopes against a shared-envelope
   contract.
7. Export a PRAXIS context pack from canonical v3 package records.
8. Add local API health and manifest/context-pack routes.
9. Add store migrations for capsules, sources, claims, graph, review, rights,
   and bundle exports.
10. Add fixture evals that compare raw, loose-doc, and capsule-augmented output.

## High-Value Fixes

- Add checksum and signature placeholders with stable diff output.
- Add `justfile` or `xtask` after command flow stabilizes.
- Add Dockerfile only after API and task handler expose real ports.
- Add local Postgres compose only when the first migration exists.
- Add OpenAPI after the API route types are implemented.

## Risk Notes

- The repo has strong architecture docs, but the coding surface must stay
  narrow or it will drift into another planning repository.
- The first coding pass must resist model-powered extraction. Schema, fixture,
  validation, and review gates must work first.
- The graph layer should remain bundle/PostgreSQL-native until eval evidence
  proves that a dedicated graph adapter is needed.

## Recommendation

Continue with Lane A/B from `docs/CODING_LEDGER.md`: make the compiler generate
the canonical v3 package from source-pack records and review decisions, then
broaden validators. Store, API, PRAXIS frontend, and cloud work should wait
until this local capsule loop is deterministic and has a context-pack export.
