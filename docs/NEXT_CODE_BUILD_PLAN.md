# Next Code Build Plan

Date: 2026-06-08

Status: active implementation plan for the next coding session.

## Objective

Turn the current contract scaffold into a working local capsule engine:

```text
source pack
  -> LLM proposal records
  -> review-trigger routing
  -> reviewed canonical records
  -> deterministic v3 .capsule compiler
  -> PRAXIS context pack
  -> local API preview
```

The next build should not add broad infrastructure. It should make one capsule
build path executable end to end with local files and no cloud credentials.

Read [Improvement Guidelines](IMPROVEMENT_GUIDELINES.md) before implementing
this plan. That file records the active gap audit and quality bar for the
first executable build.

## Current Executable Surface

Already implemented:

- Rust workspace and service crates;
- typed capsule bundle structs;
- canonical v3 package structs;
- bundle directory loader;
- first validation report with precise findings;
- schema export including `ontology_blueprint.schema.json`;
- CLI `doctor`;
- CLI `validate`;
- CLI `inspect`;
- CLI `ontology-plan`;
- CLI `schema-export`;
- canonical v3 conflict Situation Capsule fixture;
- golden policy expected bundle;
- contract tests for canonical v3 validation, rejected top-level types,
  sourceability, temporal warnings, graph registry, ontology blueprint families,
  and schema snapshots.

Not yet implemented:

- source-pack ingestion;
- LLM extraction proposal schema;
- model invocation receipts;
- review-trigger routing;
- v3 package writer;
- `.capsule` archive writer;
- Merkle/checksum/signature envelope;
- context-pack export;
- HTTP API routes;
- PostgreSQL migrations;
- document/PDF/user discussion ingestion;
- human review queue;
- PRAXIS frontend integration.

Current proof commands:

```powershell
cargo fmt --all -- --check
cargo check --locked --workspace --all-targets
cargo clippy --locked --workspace --all-targets -- -D warnings
cargo test --locked --workspace
cargo run -p dialectica-cli -- validate fixtures/canonical-capsules/conflict-situation-capsule
cargo run -p dialectica-cli -- inspect fixtures/canonical-capsules/conflict-situation-capsule
cargo run -p dialectica-cli -- validate fixtures/golden-policy-capsule/expected-bundle
cargo run -p dialectica-cli -- inspect fixtures/golden-policy-capsule/expected-bundle
cargo run -p dialectica-cli -- ontology-plan fixtures/golden-policy-capsule/expected-bundle
cargo run -p dialectica-cli -- schema-export schemas/capsule-3.0
python -m compileall tools/python
python -m unittest discover tools/python/tests
```

## Phase 1: Source Pack And Extraction Proposal Contract

Goal: make the input side of the engine real without calling a model provider.

Deliver:

- Rust `SourcePack`, `SourceDocument`, and `SourceSpan` types;
- source-pack fixture with at least one document-like source and one
  user/assistant discussion source;
- Rust `ExtractionRun` and `ModelInvocationReceipt` types;
- Rust `ExtractionProposal` envelope;
- proposal payloads for claim, episode, graph node, graph edge, ontology term,
  reasoning device, language rule, caveat, rights rule, and output rule;
- review-trigger router;
- CLI validation for source pack and proposal fixture records.

Acceptance:

- fixture source pack validates locally;
- fixture proposal records validate locally;
- every proposal includes source spans, confidence, uncertainty, model receipt,
  and review triggers;
- no proposal can be exported as canonical without review or deterministic
  promotion rules;
- tests prove LLM extraction is proposal-only.

## Phase 2: Deterministic Bundle Writer

Goal: make `dialectica-compiler` write the canonical v3 extracted package from
typed records, then assemble a `.capsule` archive.

Deliver:

- `BundleWriter` in `crates/dialectica-compiler`;
- v3 `manifest.json` with `spec_version: "3.0"` and `type`;
- `mimetype` as the first uncompressed archive entry;
- canonical files: `claims.jsonl`, `graph.jsonld`, `episodes.json`,
  `evidence/sources.jsonl`, `reasoning/`, `review/review.json`,
  `runtime.json`, `agent_context.md`, and `operations.md`;
- deterministic JSON writer;
- deterministic JSONL writer;
- archive writer for `.capsule`;
- Merkle root and projection-digest scope for open canonical files plus
  required `graph/ladybug/*` projection receipts;
- compiler receipt with input record counts and review state.

Acceptance:

- running the writer twice over the same input produces byte-identical files;
- canonical JSON and JSONL output uses stable field order, stable record order,
  LF line endings, and final newlines;
- digest calculation has an explicit scope and excludes only fields that cannot
  be known before digest calculation;
- missing review state blocks promoted export;
- rejected, expired, and unreviewed objects remain in lineage but are blocked
  from promoted PRAXIS export;
- contract tests compare generated output to the canonical v3 fixture.

Do not move to API or store work until this phase has executable proof.

## Phase 3: Source-Pack Builder

Goal: stop treating the golden bundle as hand-authored output only.

Deliver:

- `fixtures/golden-policy-capsule/source-pack/source_pack.json`;
- normalized source-span fixture records;
- extraction proposal fixture records;
- human correction fixture records;
- object-level review coverage matrix;
- `cargo run -p dialectica-cli -- build-fixture fixtures/golden-policy-capsule`;
- generated bundle lands in a temp/output directory before overwrite.

Acceptance:

- `build-fixture` can regenerate the golden bundle;
- every generated claim, edge, ontology term, language rule, and guidance item
  has source-span ids or review-action ids;
- unreviewed proposed records remain visible in lineage but cannot enter the
  PRAXIS context pack.

## Phase 4: PRAXIS Context Pack Export

Goal: create the first PRAXIS-consumable payload from `agent_context.md`,
`operations.md`, and the canonical v3 graph/claim/source files.

Deliver:

- `ContextPack` type in `dialectica-capsule`;
- compact context records with source receipts, temporal status, graph focus
  nodes, ontology blueprint, language rules, stop conditions, and review caveats;
- CLI `context-pack <bundle-dir>`;
- JSON Schema snapshot for `context_pack.schema.json`.

Acceptance:

- PRAXIS can read the context-pack JSON without PostgreSQL and can inspect the
  capsule graph through the embedded Ladybug database;
- PRAXIS can also read `agent_context.md` directly as the bounded first LLM
  context block;
- rejected and expired objects are hidden by default;
- stale or contested claims appear as warnings.
- context-pack tests assert that every included claim, graph edge, language rule,
  and output rule has source-span ids, review-action ids, or explicit expert
  note lineage.

## Phase 5: Local API Slice

Goal: make `dialectica-api` a real Axum service in local fixture mode.

Deliver:

- `GET /health`;
- `GET /version`;
- `GET /v1/capsules/{capsule_id}/manifest`;
- `GET /v1/capsules/{capsule_id}/graph-preview`;
- `GET /v1/capsules/{capsule_id}/praxis-context-pack`;
- local config that points at `fixtures/golden-policy-capsule/expected-bundle`.
- deterministic response envelope and error shape.

Acceptance:

- API boots locally with `cargo run -p dialectica-api`;
- health route reports fixture mode and schema version;
- manifest, graph preview, and context pack routes return deterministic JSON;
- error responses include code, message, details, and request id;
- no cloud credentials are required.

## Phase 6: Store Migration Skeleton

Goal: prepare Cloud SQL PostgreSQL without making it a blocker for local proof.

Deliver:

- SQLx migrations for capsules, sources, spans, temporal claims, ontology
  terms, graph nodes, graph edges, review actions, rights profiles, exports,
  and eval reports;
- repository traits;
- idempotency keys for ingestion and compile jobs;
- local Postgres runbook.

Acceptance:

- migrations apply from an empty database;
- repository tests use a fixture adapter or disposable local Postgres;
- bundle export can still run from local fixture records when Postgres is
  absent.

## Phase 7: Deployment Rail

Goal: prepare deployability after the local loop works.

Deliver:

- Dockerfile for API and task handler;
- Cloud Run service YAML;
- Cloud Run job YAML for eval/backfill/reindex;
- Cloud Tasks queue notes;
- Cloud SQL connection notes;
- Secret Manager variable list;
- GitHub Actions deploy skeleton.

Acceptance:

- local container builds;
- Cloud Run configuration is documented and reviewable;
- no Kubernetes dependency is introduced.

## Engineering Constraints

- Keep capsule bundle and PostgreSQL canonical.
- Keep Firestore as PRAXIS visibility mirror only.
- Keep Ladybug required as the embedded graph projection for promoted capsules.
- Keep Oxigraph, Graphiti, MCP, vector stores, and memory systems as optional
  adapters/caches until an ADR promotes one.
- Keep ontology blueprints capsule-specific.
- Keep every promoted object source-backed or review-backed.
- Keep LLM extraction proposal-only until validation and review promote records.
- Keep every code slice covered by contract tests.
- Keep P0/P1 gaps from `docs/IMPROVEMENT_GUIDELINES.md` visible in the ledger
  until they have command or test evidence.
- Update `docs/CODING_LEDGER.md` and `docs/BUILD_LEDGER.md` with every new
  executable surface.
