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
- added coding-start scaffold: Rust workspace, initial crates, service
  binaries, contract-test package, refreshed visual identity, coding ledger,
  and scaffold audit.
- added coherence pass for GitHub front door, embedded graph story, graph
  profile registry, Lane A acceptance, API Slice 1, Python support tooling, and
  Rust toolchain pin.
- added research-backed graph/ontology pass covering LadybugDB projection,
  W3C semantic anchors, explicit `agent_guidance.json`, capsule structure
  guide, and four example capsule envelopes.

Evidence:

- target repository cloned from `https://github.com/sargonxg/A3_DIALECTICAbyTACITUS_v3`;
- repository was empty at clone time;
- imported build instructions revision is dated `2026-06-07`;
- deployment decision checked against current Google Cloud documentation.
- graph and semantic-layer direction checked against JSON-LD, SHACL, PROV-O,
  SKOS, ODRL, VC/DID, PostgreSQL JSON, pgvector, Graphiti, GraphRAG, MCP, and
  OpenAI Agents SDK sources.
- updated research checked against LadybugDB, RDF 1.2, JSON-LD 1.1, PROV-O,
  SKOS, SHACL 1.2, ODRL, OWL, OASIS LegalDocML/Akoma Ntoso, Microsoft
  GraphRAG, Graphiti, Cloud Run, and Cloud Tasks sources.
- local Rust toolchain checked with `cargo --version`.
- workspace scaffold is expected to pass `cargo fmt --all -- --check`,
  `cargo check --locked --workspace --all-targets`,
  `cargo clippy --locked --workspace --all-targets -- -D warnings`,
  `cargo test --locked --workspace`, `cargo run -p dialectica-cli -- doctor`,
  `python -m compileall tools/python`, and
  `python -m unittest discover tools/python/tests`.

## Active Decisions

| ID | Decision | Status | Where |
| --- | --- | --- | --- |
| ADR-001 | Capsule bundle is the portable product contract | accepted | `docs/decisions/ADR-001-capsule-bundle-source-of-truth.md` |
| ADR-002 | Cloud Run first, GKE Autopilot later if proven | accepted | `docs/decisions/ADR-002-cloud-run-first-deployment.md` |
| ADR-003 | PostgreSQL first operational store | accepted | `docs/decisions/ADR-003-postgres-first-operational-store.md` |
| ADR-004 | Rust service stack: Tokio, Axum, SQLx, Serde, Schemars, tracing | accepted | `docs/decisions/ADR-004-rust-service-stack.md` |
| ADR-005 | Benchmark-informed capsule engine posture | accepted | `docs/decisions/ADR-005-benchmark-informed-capsule-engine-posture.md` |

## Next Build Tasks

1. Complete `docs/LANE_A_ACCEPTANCE.md`.
2. Implement real capsule bundle structs in `crates/dialectica-capsule`.
3. Add JSON Schema generation and schema snapshots.
4. Add a stakeholder-analysis fixture source pack under `fixtures/`.
5. Add CLI validation for the four example capsule envelopes.
6. Implement CLI `validate`, `inspect`, and `schema-export`.
7. Implement graph-slice and graph-constraint validators using
   `docs/GRAPH_PROFILE_REGISTRY.md`.
8. Add SQLx migrations in `crates/dialectica-store`.
9. Implement deterministic bundle compiler and checksums.
10. Add API Slice 1 health, manifest, graph-preview, context-pack, and receipt
   routes.
11. Add Dockerfile and local compose file.
12. Add Cloud Run staging deployment skeleton.

## Open Product Questions

- Which first policy domain should be the golden demo capsule?
- Should the initial license remain proprietary or shift to a dual-license
  model later?
- Which PRAXIS surface should show capsule receipts first?
- Which expert-review workflow is required for the first pilot?
