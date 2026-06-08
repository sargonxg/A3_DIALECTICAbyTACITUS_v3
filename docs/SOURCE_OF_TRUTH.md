# DIALECTICA v3 Source of Truth

This file defines how to use the repository before implementation starts.

## Document Priority

When documents conflict, use this order:

1. `docs/CAPSULE_SPEC.md`
2. `docs/SOURCE_OF_TRUTH.md`
3. `docs/CODING_LEDGER.md`
4. `docs/ENGINEERING_BASELINE.md`
5. `docs/LLM_CONTEXT_EXTRACTION_ARCHITECTURE.md`
6. `docs/MISSING_WORK_AUDIT_2026_06_08.md`
7. `docs/LANE_A_ACCEPTANCE.md`
8. `docs/API_SLICE_1.md`
9. `docs/GRAPH_PROFILE_REGISTRY.md`
10. `docs/CAPSULE_STRUCTURE_GUIDE.md`
11. `docs/API_CONTRACT.md`
12. `docs/DATA_MODEL.md`
13. `docs/ARCHITECTURE.md`
14. `docs/PRAXIS_INTEGRATION.md`
15. `docs/ABOUT_DIALECTICA.md`
16. `docs/FOUNDATION_BUILD.md`
17. `docs/CAPSULE_FORMAL_MODEL.md`
18. `docs/CAPSULE_TYPES_AND_MARKETPLACE.md`
19. `docs/EMBEDDED_GRAPH_AND_SEMANTIC_LAYER.md`
20. `docs/GRAPH_ONTOLOGY_RESEARCH_NOTES.md`
21. `docs/ONTOLOGY_BLUEPRINTS.md`
22. `docs/RESEARCH_LEDGER.md`
23. `docs/AGENT_BUILD_GUIDE.md`
24. `docs/NEXT_CODE_BUILD_PLAN.md`
25. `docs/IMPROVEMENT_GUIDELINES.md`
26. `docs/IMPLEMENTATION_PHASE_PLAN.md`
27. `docs/REPOSITORY_CONCEPT_REVIEW.md`
28. `docs/GITHUB_PROFILE.md`
29. `docs/EXPERT_REVIEW_AND_MARKETPLACE.md`
30. `docs/CAPSULE_BUILD_EXAMPLES.md`
31. `docs/INTELLECTUAL_TOOLS.md`
32. `docs/DEPLOYMENT.md`
33. `docs/TECH_BENCHMARK.md`
34. `docs/PYTHON_TOOLING.md`
35. `docs/DIALECTICA_v3_BUILD_INSTRUCTIONS.md` as reference context
36. ADRs in `docs/decisions/`
37. Implementation notes in issues, PRs, and comments

If an implementation contradicts a higher-priority document, update the
document through an ADR or stop and ask for a product decision.

## Product Boundary

TACITUS is the company.

PRAXIS is the visible policy workbench, user cockpit, and agentic workflow
surface.

DIALECTICA is the internal capsule intelligence engine that builds portable
PRAXIS Capsules.

AGON and KAIROS are future perception subsystems. They may feed DIALECTICA
through versioned adapters, but they are not required for the first working foundation build.

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
- language profile

Do not market DIALECTICA as a standalone buyer-facing product until TACITUS
makes that decision explicitly.

## Foundation Build Constraints

The foundation build must prove that a capsule improves PRAXIS output compared with raw LLM
generation.

The foundation build must include:

- a portable `.capsule` bundle with MIME marker and v3 `manifest.json`;
- canonical files: `claims.jsonl`, `graph.jsonld`, `episodes.json`,
  `evidence/sources.jsonl`, `reasoning/`, `review/review.json`,
  `runtime.json`, `agent_context.md`, and `operations.md`;
- source and provenance records linked to claims and graph nodes;
- temporal episode model;
- ontology named graph and capsule-specific semantic cores;
- embedded connected graph where substrate and guidance are traversable
  together;
- explicit runtime contract for PRAXIS workflow use;
- capsule-specific ontology blueprint and semantic-layer plan;
- source-bound LLM proposal records with model receipts and review triggers;
- human review and trust-layer records;
- rights and marketplace-readiness metadata when promotion requires it;
- concrete policy fixture examples;
- PRAXIS integration contract;
- eval fixtures that measure whether the capsule helps.

The foundation build must not require:

- Kubernetes;
- a dedicated graph database;
- a commercial vector database;
- a complex event-sourcing platform;
- a full expert-review marketplace;
- AGON or KAIROS as separate production systems.

## Engineering Rules

- Keep the capsule bundle canonical and portable.
- Keep PostgreSQL as the first DIALECTICA operational source of truth.
- Keep PRAXIS Firestore canonical for PRAXIS user-facing capsule visibility and
  cockpit state.
- Keep graph and semantic engines as adapters, caches, or derived views until
  the capsule contract is stable.
- Store source artifacts immutably.
- Keep every derived object traceable to source spans, model runs, rules, or
  expert decisions.
- Treat expert review as data, not only as workflow UI.
- Do not silently add dependencies that change the deployment class.
- Every model-powered extraction must have an eval or fixture.
- Model-powered extraction creates proposals only; canonical records require
  Rust validation and required review.
- Every schema change must include migration notes and compatibility impact.

## Implementation Entry Criteria

Before building service code, the repo needs:

- Rust workspace scaffold;
- capsule JSON schema;
- fixture capsule bundle;
- typed example capsules for user, situation, tool, and output
  profiles;
- contract tests for manifest, source ledger, review ledger, and export;
- graph profile registry alignment;
- language profile contract;
- deployment skeleton for local Docker and Cloud Run;
- CI check for formatting, tests, and docs presence.

Current implementation status: the first executable capsule-contract slice now
includes Rust legacy bundle structs, v3 package validation, schema export, a
legacy expected-bundle fixture, a canonical v3 Situation Capsule fixture, and
CLI `validate`, `inspect`, `ontology-plan`, and `schema-export` commands.
Extractor, compiler, store, API, task handler, and eval capabilities remain
future phases.

## First Definition of Done

Phase 1 is done when a developer can run one local command to:

1. create a valid sample capsule bundle from fixtures;
2. validate it against the v3 capsule spec;
3. inspect the source ledger and review ledger;
4. export the bundle as a `.capsule` archive and extracted directory
   projection;
5. run contract tests in CI.
