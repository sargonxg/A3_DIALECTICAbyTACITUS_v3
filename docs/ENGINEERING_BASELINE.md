# Engineering Baseline

## Purpose

This file defines the implementation base for DIALECTICA so future coding
agents do not improvise the stack.

## Language Responsibilities

| Layer | Language | Why |
| --- | --- | --- |
| Capsule contract | Rust | type safety, schema ownership, deterministic validation |
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
  owns portable bundle structs, schema generation, validation, and versioning

dialectica-compiler
  owns deterministic bundle assembly, checksums, signing hooks, and context-pack export

dialectica-store
  owns SQLx migrations, repositories, transactions, and idempotency

dialectica-eval
  owns deterministic quality checks and PRAXIS-vs-baseline comparisons

dialectica-cli
  owns local developer workflows: doctor, validate, inspect, build-fixture

dialectica-api
  owns PRAXIS-facing HTTP endpoints

dialectica-task-handler
  owns Cloud Tasks execution endpoints
```

Dependency direction:

```text
api/task-handler -> store/compiler/capsule/eval
compiler         -> capsule
store            -> capsule-compatible IDs and records
eval             -> capsule
cli              -> capsule/compiler/eval/store as needed
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
3. `fixtures/golden-policy-capsule`: source pack and expected bundle.
4. `dialectica-compiler`: deterministic bundle writer and checksums.
5. `dialectica-store`: migrations and repository interfaces.
6. `dialectica-api`: health, version, manifest, graph preview, context pack.
7. `dialectica-task-handler`: queued compile path.
8. `dialectica-eval`: fixture eval reports.
9. Python reports and adapters where they reduce implementation risk.

## Required Local Gate

```powershell
cargo fmt --all -- --check
cargo check --locked --workspace --all-targets
cargo clippy --locked --workspace --all-targets -- -D warnings
cargo test --locked --workspace
cargo run -p dialectica-cli -- doctor
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
- Capsule structure: `docs/CAPSULE_STRUCTURE_GUIDE.md`
- Graph/ontology research: `docs/GRAPH_ONTOLOGY_RESEARCH_NOTES.md`
- Python support: `docs/PYTHON_TOOLING.md`
