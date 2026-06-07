# Build Ledger

This file records major build decisions, evidence, and next actions.

## 2026-06-07 - Repository Initialization

Status: complete locally, pending commit/push approval.

Actions:

- imported `DIALECTICA_v3_BUILD_INSTRUCTIONS.md`;
- created README and source-of-truth docs;
- documented Cloud Run first deployment path;
- documented Kubernetes/GKE Autopilot promotion criteria;
- added capsule specification draft;
- added architecture, PRAXIS integration, eval, operations, dependency, and
  security docs;
- added ADRs for capsule bundle, Cloud Run, and PostgreSQL decisions;
- added GitHub templates and repository hygiene files.
- added second-pass build scaffolding docs for foundation build definition, API contract,
  data model, local development, CI/CD, repository structure, and dependency
  source anchors.
- added final pre-push polish with README diagrams, SVG mark, tech benchmark,
  formal capsule model, intellectual tools guide, agentic workflow lanes,
  PRAXIS repo alignment, research backlog, and benchmark-informed ADR.
- added post-research capsule architecture expansion covering capsule types,
  embedded graph semantics, human-gated expert review, marketplace mechanics,
  and concrete policy build examples.

Evidence:

- target repository cloned from `https://github.com/sargonxg/A3_DIALECTICAbyTACITUS_v3`;
- repository was empty at clone time;
- imported build instructions revision is dated `2026-06-07`;
- deployment decision checked against current Google Cloud documentation.
- graph and semantic-layer direction checked against JSON-LD, SHACL, PROV-O,
  SKOS, ODRL, VC/DID, PostgreSQL JSON, pgvector, Graphiti, GraphRAG, MCP, and
  OpenAI Agents SDK sources.

## Active Decisions

| ID | Decision | Status | Where |
| --- | --- | --- | --- |
| ADR-001 | Capsule bundle is the portable product contract | accepted | `docs/decisions/ADR-001-capsule-bundle-source-of-truth.md` |
| ADR-002 | Cloud Run first, GKE Autopilot later if proven | accepted | `docs/decisions/ADR-002-cloud-run-first-deployment.md` |
| ADR-003 | PostgreSQL first operational store | accepted | `docs/decisions/ADR-003-postgres-first-operational-store.md` |
| ADR-004 | Rust service stack: Tokio, Axum, SQLx, Serde, Schemars, tracing | accepted | `docs/decisions/ADR-004-rust-service-stack.md` |
| ADR-005 | Benchmark-informed capsule engine posture | accepted | `docs/decisions/ADR-005-benchmark-informed-capsule-engine-posture.md` |

## Next Build Tasks

1. Create Rust workspace under `crates/` and `services/`.
2. Define JSON schema for capsule bundle components.
3. Add a fixture source pack under `fixtures/`.
4. Add the stakeholder-analysis fixture from `docs/CAPSULE_BUILD_EXAMPLES.md`.
5. Implement local capsule bundle validator.
6. Add graph-slice and graph-constraint validators.
7. Add contract tests.
8. Add Dockerfile and local compose file.
9. Add Cloud Run staging deployment skeleton.
10. Add first PRAXIS context-pack endpoint.

## Open Product Questions

- Which first policy domain should be the golden demo capsule?
- Should the initial license remain proprietary or shift to a dual-license
  model later?
- Which PRAXIS surface should show capsule receipts first?
- Which expert-review workflow is required for the first pilot?
