# Next Code Build Plan

Date: 2026-06-07

Status: active implementation plan for the next coding session.

## Objective

Turn the current contract scaffold into a working local capsule engine:

```text
source pack
  -> reviewed canonical records
  -> deterministic bundle compiler
  -> PRAXIS context pack
  -> local API preview
```

The next build should not add broad infrastructure. It should make one capsule
build path executable end to end with local files and no cloud credentials.

## Current Executable Surface

Already implemented:

- Rust workspace and service crates;
- typed capsule bundle structs;
- bundle directory loader;
- first validation report with precise findings;
- schema export including `ontology_blueprint.schema.json`;
- CLI `doctor`;
- CLI `validate`;
- CLI `inspect`;
- CLI `ontology-plan`;
- CLI `schema-export`;
- golden policy expected bundle;
- contract tests for sourceability, temporal warnings, graph registry,
  ontology blueprint families, and schema snapshots.

Current proof commands:

```powershell
cargo fmt --all -- --check
cargo check --locked --workspace --all-targets
cargo clippy --locked --workspace --all-targets -- -D warnings
cargo test --locked --workspace
cargo run -p dialectica-cli -- validate fixtures/golden-policy-capsule/expected-bundle
cargo run -p dialectica-cli -- inspect fixtures/golden-policy-capsule/expected-bundle
cargo run -p dialectica-cli -- ontology-plan fixtures/golden-policy-capsule/expected-bundle
cargo run -p dialectica-cli -- schema-export schemas/capsule-0.1.0
python -m compileall tools/python
python -m unittest discover tools/python/tests
```

## Phase 1: Deterministic Bundle Writer

Goal: make `dialectica-compiler` write a bundle directory from typed records.

Deliver:

- `BundleWriter` in `crates/dialectica-compiler`;
- deterministic JSON writer;
- deterministic JSONL writer;
- required file list from `docs/CAPSULE_SPEC.md`;
- checksum manifest placeholder;
- compiler receipt with input record counts and review state.

Acceptance:

- running the writer twice over the same input produces byte-identical files;
- missing review state blocks promoted export;
- contract tests compare generated output to the golden expected bundle.

## Phase 2: Source-Pack Builder

Goal: stop treating the golden bundle as hand-authored output only.

Deliver:

- `fixtures/golden-policy-capsule/source-pack/source_pack.json`;
- normalized source-span fixture records;
- extraction proposal fixture records;
- human correction fixture records;
- `cargo run -p dialectica-cli -- build-fixture fixtures/golden-policy-capsule`;
- generated bundle lands in a temp/output directory before overwrite.

Acceptance:

- `build-fixture` can regenerate the golden bundle;
- every generated claim, edge, ontology term, language rule, and guidance item
  has source-span ids or review-action ids;
- unreviewed proposed records remain visible in lineage but cannot enter the
  PRAXIS context pack.

## Phase 3: PRAXIS Context Pack Export

Goal: create the first PRAXIS-consumable payload.

Deliver:

- `ContextPack` type in `dialectica-capsule`;
- compact context records with source receipts, temporal status, graph focus
  nodes, ontology blueprint, language rules, stop conditions, and review caveats;
- CLI `context-pack <bundle-dir>`;
- JSON Schema snapshot for `context_pack.schema.json`.

Acceptance:

- PRAXIS can read the context-pack JSON without needing PostgreSQL or a graph
  database;
- rejected and expired objects are hidden by default;
- stale or contested claims appear as warnings.

## Phase 4: Local API Slice

Goal: make `dialectica-api` a real Axum service in local fixture mode.

Deliver:

- `GET /health`;
- `GET /version`;
- `GET /v1/capsules/{capsule_id}/manifest`;
- `GET /v1/capsules/{capsule_id}/graph-preview`;
- `GET /v1/capsules/{capsule_id}/praxis-context-pack`;
- local config that points at `fixtures/golden-policy-capsule/expected-bundle`.

Acceptance:

- API boots locally with `cargo run -p dialectica-api`;
- health route reports fixture mode and schema version;
- manifest, graph preview, and context pack routes return deterministic JSON;
- no cloud credentials are required.

## Phase 5: Store Migration Skeleton

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

## Phase 6: Deployment Rail

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
- Keep LadybugDB, Graphiti, MCP, and vector stores as adapters until an ADR
  promotes one.
- Keep ontology blueprints capsule-specific.
- Keep every promoted object source-backed or review-backed.
- Keep every code slice covered by contract tests.
- Update `docs/CODING_LEDGER.md` and `docs/BUILD_LEDGER.md` with every new
  executable surface.
