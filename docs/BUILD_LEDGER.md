# Build Ledger

This file records major build decisions, evidence, and next actions.

## 2026-06-07 - Repository Initialization

Status: active repository baseline; commit and push requested by user.

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
- added repository concept review, GitHub profile guidance, and first-class
  `language_profile.json` support across README, capsule spec, examples,
  PRAXIS context-pack docs, and Lane A acceptance.
- added durable research ledger, future-agent build guide, agent build-flow
  diagram, and research-memory diagram so future agents can trace source links
  into implementation decisions and refresh triggers.
- started the first functional coding pass: Rust capsule bundle structs,
  validation findings, bundle directory loader, JSON Schema export, CLI
  `validate`/`inspect`/`schema-export`, golden policy expected-bundle fixture,
  and Lane A contract tests.
- added capsule-specific ontology blueprint planning to the Rust contract and
  CLI so actor/claim graphs remain one profile rather than the universal
  capsule ontology.
- added final pre-push repository polish: ontology factory diagram,
  next-code-build plan, refreshed GitHub front door, and source refresh notes
  for Cloud Run worker pools, MCP tools/auth, Graphiti/Zep, LadybugDB, and
  OpenAI Agents SDK tracing/guardrails.
- added post-push improvement guidelines covering active gaps, quality bar,
  improvement sequence, review checklist, research/dependency policy, and
  gap-ledger protocol.
- updated CI checkout steps to the current Node 24 runtime line after GitHub
  Actions warned that Node 20 actions are being deprecated.
- switched the repository from a closed-source posture to Apache-2.0, added
  TACITUS attribution in `NOTICE`, added `CITATION.cff`, and recorded the
  decision in ADR-006.

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
- latest research ledger checked against MCP resources, Microsoft GraphRAG,
  Graphiti/Zep, temporal knowledge graph papers, RAG, LadybugDB, JSON-LD,
  PROV-O, SKOS, SHACL, ODRL, LegalDocML, Cloud Run, Cloud SQL, Cloud Tasks,
  Firestore, and OpenAI Agents SDK source anchors.
- local Rust toolchain checked with `cargo --version`.
- workspace scaffold is expected to pass `cargo fmt --all -- --check`,
  `cargo check --locked --workspace --all-targets`,
  `cargo clippy --locked --workspace --all-targets -- -D warnings`,
  `cargo test --locked --workspace`, `cargo run -p dialectica-cli -- doctor`,
  `python -m compileall tools/python`, and
  `python -m unittest discover tools/python/tests`.
- first functional capsule validation was checked with
  `cargo run -p dialectica-cli -- validate fixtures/golden-policy-capsule/expected-bundle`,
  `cargo run -p dialectica-cli -- inspect fixtures/golden-policy-capsule/expected-bundle`,
  `cargo run -p dialectica-cli -- ontology-plan fixtures/golden-policy-capsule/expected-bundle`,
  and the then-current legacy schema export; the current canonical gate exports
  to `schemas/capsule-3.0`.
- improvement pass reconciled `README.md`, `AGENTS.md`,
  `docs/SOURCE_OF_TRUTH.md`, `docs/README.md`, `docs/CODING_LEDGER.md`,
  `docs/NEXT_CODE_BUILD_PLAN.md`, and the docs CI required-file list around
  `docs/IMPROVEMENT_GUIDELINES.md`.
- latest CI warning checked against the official `actions/checkout` release
  notes, which document the Node 24 runtime line.
- license choice checked against Apache Software Foundation guidance and
  GitHub/Citation File Format documentation.
## 2026-06-08 - Front-Door Simplification

Status: ready for commit and push.

Actions:

- replaced the dense README banner with a clearer accessible SVG mark;
- shortened the README, `AGENTS.md`, and `docs/README.md` start paths;
- kept the complete documentation library available behind `docs/README.md`;
- updated the scaffold audit, concept review, asset notes, and improvement
  guidelines with the simplicity standard.

Evidence:

- front-door simplification checked against GitHub README relative-link/image
  guidance and MDN SVG accessibility guidance for `role="img"`, accessible
  names, and SVG title metadata;
- `assets/dialectica-mark.svg` parses as XML;
- remote GitHub About metadata checked with `gh repo view`; description,
  homepage, topics, default branch, and Apache-2.0 license match the repository
  profile guidance;

## 2026-06-08 - Four Capsule Contract

Status: active taxonomy and validation contract.

Actions:

- fixed the PRAXIS-importable capsule classes to User, Situation, Tool, and
  Output;
- moved source proof, domain semantics, stakeholder maps, scenario branches,
  expert picks, and graph modules into internal semantic layers, graph lenses,
  review metadata, or marketplace metadata;
- renamed the legacy method fixture to `tool-capsule.example.json`;
- added README diagrams for the four capsule model and capsule anatomy;
- added Rust validation and schema pattern guidance for the four allowed
  `capsule_type` values;
- updated API, graph, ontology, data-model, local-development, CI, and example
  docs to prevent the old taxonomy from reappearing.

Evidence:

- stale taxonomy search now leaves only intentional negative tests for
  unsupported old types;
- graph registry keeps the broad node and edge vocabulary while narrowing the
  top-level capsule API;
- LadybugDB remains documented as an optional projection adapter, not canonical
  state.
- terminology scan found no banned product-shortcut, messy-context,
  proprietary-license, or placeholder-marker language in the checked repository
  scope.

## 2026-06-08 - Fine-Tuning And Graph Universe Intake

Status: research captured; no dependency promotion.

Actions:

- recorded Gemma/Unsloth-style LoRA fine-tuning as a future extractor
  distillation lane, not a foundation build requirement;
- recorded Awesome Graph Universe as a discovery index rather than a stack
  decision;
- recorded the GraphGeeks Agentic Graph RAG workshop as a future reference for
  hybrid graph/vector/full-text retrieval, observability, guardrails, and evals;
- updated dependency policy, eval criteria, research backlog, foundation
  non-goals, and research ledger.

Evidence:

- local repo and remote GitHub repo checked; the repository is no longer empty
  and contains the current Rust/docs/fixtures/scaffold on `main`;
- Google Gemma 4 12B developer guide and Hugging Face model card checked for
  current model/fine-tuning and multimodal claims;
- Unsloth notebook index checked as the implementation-reference source for
  fine-tuning workflows;
- GraphGeeks `awesome-graph-universe` and `odsc-agentic-ai-summit-2025`
  checked for graph registry and hybrid GraphRAG patterns.

## 2026-06-08 - Canonical Capsule Spec v3 Alignment

Status: canonical contract promoted; compiler migration still pending.

Actions:

- promoted `docs/CAPSULE_SPEC.md` to the definitive v3 `.capsule` contract;
- moved the source-of-truth priority so the capsule spec outranks older
  implementation scaffolding;
- added canonical v3 Rust types for `PraxisCapsuleManifest` and
  `PraxisCapsulePackage`;
- added CLI auto-detection for v3 packages versus legacy expected bundles;
- added a canonical conflict Situation Capsule fixture under
  `fixtures/canonical-capsules/conflict-situation-capsule`;
- added contract tests proving the canonical v3 fixture validates and that
  non-macro top-level types such as `stakeholder` are rejected;
- updated README, Lane A acceptance, build plan, API contract, data model,
  formal model, structure guide, and embedded graph docs to target
  `manifest.json.type = user | situation | tool | output`;
- marked the old `0.1.0` expected-bundle shape as legacy compatibility rather
  than the product contract.

Evidence:

- canonical v3 fixture includes `mimetype`, `manifest.json`, `claims.jsonl`,
  `graph.jsonld`, `episodes.json`, `evidence/sources.jsonl`, `reasoning/`,
  `review/review.json`, `runtime.json`, `agent_context.md`, and
  `operations.md`;
- v3 validation now checks required files, manifest layer vocabulary, macro
  type values, non-empty compiled views, JSON parseability, and minimum
  Situation Capsule claim/source records;
- remaining gap: the compiler still needs to generate the v3 package and
  archive rather than relying on hand-authored fixtures.

## 2026-06-08 - Code Audit And Build-Gate Update

Status: audit complete; local contract scaffold verified; functional engine
build remains pending.

Actions:

- added `docs/CODE_AUDIT_2026_06_08.md` as the current truth about what is
  coded versus planned;
- added the audit doc to the documentation index, agent start path, and CI
  required-doc gate;
- promoted canonical v3 fixture validation and inspection into the active CI
  and local command gates;
- updated `.env.example` and local docs to use
  `DIALECTICA_CAPSULE_SPEC_VERSION=3.0` while keeping
  `DIALECTICA_LEGACY_BUNDLE_SCHEMA_VERSION=0.1.0` explicit;
- updated the coding ledger with the next v3-first implementation sequence;
- recorded that compiler, store, API, task handler, ingestion, review UI, and
  PRAXIS frontend integration are not yet functional.

Evidence:

- `cargo fmt --all -- --check` passed;
- `cargo check --locked --workspace --all-targets` passed;
- `cargo clippy --locked --workspace --all-targets -- -D warnings` passed;
- `cargo test --locked --workspace` passed;
- `cargo run -p dialectica-cli -- doctor` passed;
- `cargo run -p dialectica-cli -- validate fixtures/canonical-capsules/conflict-situation-capsule` passed;
- `cargo run -p dialectica-cli -- inspect fixtures/canonical-capsules/conflict-situation-capsule` passed;
- `cargo run -p dialectica-cli -- validate fixtures/golden-policy-capsule/expected-bundle` passed with the expected stale-claim warning;
- `cargo run -p dialectica-cli -- schema-export $env:TEMP\dialectica-audit-schemas` passed;
- `python -m compileall tools/python` passed;
- `python -m unittest discover tools/python/tests` passed;
- a tighter committed-secret scan found no environment-style secret assignments.

Blocking build gaps:

1. v3 compiler writer and source-pack inputs;
2. `.capsule` archive writer, deterministic digest, and signature envelope;
3. deep v3 cross-layer validator;
4. PRAXIS context-pack exporter;
5. local Axum API;
6. PostgreSQL migrations and repositories;
7. ingestion and human review workflow;
8. PRAXIS frontend integration after the local contract is stable.

## Active Decisions

| ID | Decision | Status | Where |
| --- | --- | --- | --- |
| ADR-001 | Capsule bundle is the portable product contract | accepted | `docs/decisions/ADR-001-capsule-bundle-source-of-truth.md` |
| ADR-002 | Cloud Run first, GKE Autopilot later if proven | accepted | `docs/decisions/ADR-002-cloud-run-first-deployment.md` |
| ADR-003 | PostgreSQL first operational store | accepted | `docs/decisions/ADR-003-postgres-first-operational-store.md` |
| ADR-004 | Rust service stack: Tokio, Axum, SQLx, Serde, Schemars, tracing | accepted | `docs/decisions/ADR-004-rust-service-stack.md` |
| ADR-005 | Benchmark-informed capsule engine posture | accepted | `docs/decisions/ADR-005-benchmark-informed-capsule-engine-posture.md` |
| ADR-006 | Apache-2.0 open-source license with citation metadata | accepted | `docs/decisions/ADR-006-open-source-license-and-citation.md` |

## Next Build Tasks

1. Define typed source-pack inputs and canonical deterministic serialization
   rules.
2. Implement deterministic v3 package writer in `crates/dialectica-compiler`.
3. Add `.capsule` archive writing with `mimetype` first, stable entry order,
   LF line endings, and explicit cache exclusion from digest scope.
4. Add `build-fixture` so a canonical capsule can be generated from source-pack
   records and review decisions.
5. Add checksum, Merkle-root, and signature placeholders with stable diff
   output.
6. Deepen v3 validators for claims, sources, temporal episodes, graph,
   reasoning, review, runtime, and generated agent views.
7. Export the first PRAXIS context pack from the canonical v3 package.
8. Validate the four example capsule envelopes against a shared top-level
   contract.
9. Turn `services/dialectica-api` into a local fixture-mode Axum service.
10. Add SQLx migrations in `crates/dialectica-store`.
11. Add ingestion records for documents, PDFs, and user/assistant discussion
    turns after the compiler loop works.
12. Add task-handler and Cloud Run staging skeleton only after the local API
    runs.

## Open Product Questions

- Which first policy domain should be the golden demo capsule?
- Which PRAXIS surface should show capsule receipts first?
- Which expert-review workflow is required for the first pilot?
