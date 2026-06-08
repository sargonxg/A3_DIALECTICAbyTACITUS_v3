# Engineering Baseline

## Purpose

This file defines the implementation base for DIALECTICA so future coding
agents do not improvise the stack.

## Language Responsibilities

| Layer | Language | Why |
| --- | --- | --- |
| Capsule contract | Rust | type safety, schema ownership, deterministic validation |
| Local capsule builder | Rust | deterministic document-to-capsule proof loop and PRAXIS artifact bridge |
| LLM extraction orchestration | Rust | proposal schemas, review routing, source-bound model calls, and deterministic promotion gates |
| Compiler | Rust | reproducible bundle writes, checksums, signing path |
| API and task handler | Rust | Cloud Run services, strong boundaries, low runtime overhead |
| Store and migrations | Rust + SQL | PostgreSQL as operational source of truth |
| Contract tests | Rust | same type system as the backend |
| Eval reports and research utilities | Python | fast analysis, tabular reports, notebooks/scripts if needed |
| PRAXIS client integration | TypeScript in PRAXIS repo | user-facing cockpit remains PRAXIS-owned |

Rust is the production spine. Python is a support toolchain, not the canonical
backend.

## Rust Workspace Ownership

```text
dialectica-capsule
  owns portable bundle structs, ontology blueprint planning, schema generation,
  validation, and versioning

dialectica-builder
  owns local document-folder ingestion, deterministic source-pack/proposal
  creation, caveated reviewer decisions, package/archive/context-pack writes,
  and PRAXIS import receipts

dialectica-extractor
  owns source-pack inputs, extraction proposal schemas, model invocation
  receipts, review-trigger routing, reviewer decisions, promotion
  normalization, build-plan contracts, and future provider traits

dialectica-compiler
  owns deterministic bundle assembly, checksums, signing hooks, and
  context-pack export

dialectica-store
  owns SQLx migrations, repositories, transactions, and idempotency

dialectica-eval
  owns deterministic quality checks and PRAXIS-vs-baseline comparisons

dialectica-cli
  owns local developer workflows: doctor, validate, inspect, ontology-plan,
  welcome, build-docs, build-fixture, archive, context-pack, praxis-pack,
  mcp-config

dialectica-api
  owns PRAXIS-facing HTTP endpoints

dialectica-mcp
  owns local Codex MCP stdio tools, resources, and prompts for capsule building

dialectica-task-handler
  owns Cloud Tasks execution endpoints
```

Dependency direction:

```text
api/task-handler -> store/extractor/compiler/capsule/eval
builder          -> compiler/extractor/capsule
extractor        -> capsule
compiler         -> capsule/extractor/graph
store            -> capsule-compatible IDs and records
eval             -> capsule
mcp              -> builder/compiler/capsule/graph
cli              -> builder/capsule/compiler/eval/store as needed
capsule          -> no project-local dependencies
```

## Python Tooling Boundary

Python tools live under `tools/python/` and may be used for:

- fixture report generation;
- eval report rendering;
- one-off source-pack inspection;
- research notebooks or scripts;
- graph export sanity checks;
- marketplace analytics prototypes.

Python tools must not become the canonical write path for capsule truth. If a
Python utility proposes capsule records, the Rust validator and review ledger
still decide whether the records can be promoted.

## First Implementation Order

1. `dialectica-capsule`: real structs, schema, validation errors.
2. `dialectica-cli`: validate and inspect a fixture bundle.
3. `fixtures/canonical-capsules`: canonical v3 package fixture.
4. `dialectica-extractor`: source-pack and proposal schemas with review
   triggers: implemented for fixture mode.
5. reviewer decisions and proposal promotion normalization: implemented for
   fixture mode.
6. `dialectica-compiler`: deterministic v3 package writer, archive writer, and
   context-pack export: implemented for fixture mode.
7. `dialectica-builder`: local text-document folder to package/archive/PRAXIS
   bridge: implemented for local mode.
8. `dialectica-mcp`: Codex stdio adapter over builder/compiler tools:
   implemented for local mode.
9. `dialectica-api`: health, version, manifest, graph preview, context pack:
   implemented for fixture mode.
10. `dialectica-store`: migrations and repository interfaces.
11. `dialectica-task-handler`: queued compile path.
12. `dialectica-eval`: fixture eval reports.
13. Python reports and adapters where they reduce implementation risk.

## Required Local Gate

```powershell
cargo fmt --all -- --check
cargo check --locked --workspace --all-targets
cargo clippy --locked --workspace --all-targets -- -D warnings
cargo test --locked --workspace
cargo run -p dialectica-cli -- welcome
cargo run -p dialectica-cli -- build-docs --type situation --input .\docs --out $env:TEMP\dialectica-doc-capsule --title "Local Situation Capsule" --workflow decision_brief
cargo run -p dialectica-cli -- inspect $env:TEMP\dialectica-doc-capsule\package
cargo run -p dialectica-cli -- mcp-config
cargo run -p dialectica-cli -- doctor
cargo run -p dialectica-cli -- validate fixtures/canonical-capsules/conflict-situation-capsule
cargo run -p dialectica-cli -- inspect fixtures/canonical-capsules/conflict-situation-capsule
cargo run -p dialectica-cli -- validate fixtures/golden-policy-capsule/expected-bundle
cargo run -p dialectica-cli -- inspect fixtures/golden-policy-capsule/expected-bundle
cargo run -p dialectica-cli -- ontology-plan fixtures/golden-policy-capsule/expected-bundle
cargo run -p dialectica-cli -- source-pack-check fixtures/golden-policy-capsule/source-pack/source_pack.json
cargo run -p dialectica-cli -- proposal-check fixtures/golden-policy-capsule/build_request.json fixtures/golden-policy-capsule/source-pack/source_pack.json fixtures/golden-policy-capsule/proposals
cargo run -p dialectica-cli -- build-plan fixtures/golden-policy-capsule/build_request.json fixtures/golden-policy-capsule/source-pack/source_pack.json fixtures/golden-policy-capsule/proposals
cargo run -p dialectica-cli -- review-check fixtures/golden-policy-capsule/build_request.json fixtures/golden-policy-capsule/source-pack/source_pack.json fixtures/golden-policy-capsule/proposals fixtures/golden-policy-capsule/review-decisions
cargo run -p dialectica-cli -- promote-check fixtures/golden-policy-capsule/build_request.json fixtures/golden-policy-capsule/source-pack/source_pack.json fixtures/golden-policy-capsule/proposals fixtures/golden-policy-capsule/review-decisions
cargo run -p dialectica-cli -- build-fixture fixtures/golden-policy-capsule --out $env:TEMP\dialectica-golden-v3
cargo run -p dialectica-cli -- validate $env:TEMP\dialectica-golden-v3
cargo run -p dialectica-cli -- archive $env:TEMP\dialectica-golden-v3 --out $env:TEMP\dialectica-golden-v3.capsule
cargo run -p dialectica-cli -- context-pack $env:TEMP\dialectica-golden-v3 --workflow conflict_map
cargo run -p dialectica-cli -- schema-export schemas/capsule-3.0
python -m compileall tools/python
python -m unittest discover tools/python/tests
```

Add stricter Python linting after Python tools become non-trivial.

## Promotion Rules

Do not promote a capability as functional unless:

- it has a command or route that runs locally;
- it has a test or fixture;
- it is recorded in `docs/CODING_LEDGER.md`;
- failure modes are visible in `docs/SCAFFOLD_AUDIT.md` or a follow-up audit;
- the README does not overclaim the capability.

## Active Reference Files

- Lane A: `docs/LANE_A_ACCEPTANCE.md`
- API Slice 1: `docs/API_SLICE_1.md`
- Graph vocabulary: `docs/GRAPH_PROFILE_REGISTRY.md`
- Ontology blueprints: `docs/ONTOLOGY_BLUEPRINTS.md`
- Capsule structure: `docs/CAPSULE_STRUCTURE_GUIDE.md`
- Graph/ontology research: `docs/GRAPH_ONTOLOGY_RESEARCH_NOTES.md`
- LLM extraction architecture: `docs/LLM_CONTEXT_EXTRACTION_ARCHITECTURE.md`
- Missing work audit: `docs/MISSING_WORK_AUDIT_2026_06_08.md`
- Implementation phases: `docs/IMPLEMENTATION_PHASE_PLAN.md`
- Python support: `docs/PYTHON_TOOLING.md`
- Current code audit: `docs/CODE_AUDIT_2026_06_08.md`
