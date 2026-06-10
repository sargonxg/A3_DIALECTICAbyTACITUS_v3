# Next Code Build Plan

Date: 2026-06-10

Status: active implementation plan after the local MVP capsule-loop hardening,
the 2026-06-09 strategic v3 plan review, and the 2026-06-10 editable review CLI
slice. The local proof lane can now generate editable review decisions and
recompile reviewed output; the next build slice is the diff engine and cited
change memo, not cloud persistence or live model providers.

## Objective

The current local capsule engine now supports an editable review loop:

```text
source pack / local documents / JSONL discussion source
  -> fixture-mode or local-document proposal records
  -> review-trigger routing
  -> generated review_queue.json
  -> human-edited reviewer decisions
  -> re-run promotion
  -> recompile deterministic v3 .capsule + PRAXIS context pack
```

The next build should not add broad infrastructure. The editable review slice is
locally proven; continue toward the Demo Gate by producing version-to-version
diff output and a cited change memo.

Read [Improvement Guidelines](IMPROVEMENT_GUIDELINES.md) before implementing
this plan. That file records the active gap audit and quality bar for the
first executable build.

## Strategic v3 Incorporation

The 2026-06-09 strategic plan is accepted as a planning layer, not as proof of
implemented behavior. The repo governance remains binding:

- `docs/SOURCE_OF_TRUTH.md`, `docs/CODING_LEDGER.md`, this file, and ADRs
  control implementation order;
- every new schema family or architectural boundary needs an ADR before code;
- no planned capability should be described as functional until it has command,
  fixture, and test evidence.

The amended strategic sequence is:

```text
editable review decisions
  -> capsule diff + change memo
  -> deterministic integrity envelope
  -> elicitation protocols
  -> composition inspector + visibility/attribution
  -> live provider and richer ingestion
  -> store-backed API and durable jobs
  -> PRAXIS staging integration
  -> scorecard, Living Capsule export, publish validation
```

The local Demo Gate is: real or fixture documents -> reviewed capsule ->
signed or verifiable package -> version-to-version diff -> cited change memo.
This gate must stay laptop-local and must not require Cloud Run, PostgreSQL,
hosted MCP, or PRAXIS production calls.

## Planned Strategic Slices After Editable Review

### Slice C: Diff Engine And Change Memo

Status: planned, ADR required.

Governance: [ADR-009](decisions/ADR-009-diff-engine-placement.md) proposes
placing the first diff engine inside `dialectica-compiler`.

Deliver:

- `schemas/capsule-3.0/capsule_diff.schema.json`;
- golden old/new capsule fixture pair with planted deltas;
- deterministic `diff.json`;
- cited `change-memo.md` renderer;
- CLI `diff` command and local MCP diff tool;
- eval check for diff correctness.

Acceptance:

- golden-pair diff output is byte-identical across runs;
- memo citations resolve to source or review receipts in the newer capsule;
- no store, cloud, hosted MCP, or live provider call is required.

### Slice D: Deterministic Integrity Envelope

Status: planned, depends on deterministic byte-output hardening.

Deliver:

- explicit canonical-file digest scope;
- Merkle root over canonical files, excluding rebuildable projections/receipts
  only where the manifest already documents that scope;
- `capsule verify` CLI;
- tamper tests;
- Ed25519 signing plan that separates author identity from publisher identity.

Acceptance:

- one-byte mutation fails verification;
- fixture packages verify in CI;
- signing does not hide or overwrite author/publisher attribution.

### Slice I: Elicitation Protocols

Status: planned, ADR-007 proposal-only boundary applies.

Deliver:

- `schemas/capsule-3.0/elicitation_protocol.schema.json`;
- fixture protocols `user.v1`, `situation.v1`, `tool.v1`, and `output.v1`;
- scripted Tool interview fixture using the existing JSONL discussion-capture
  source path;
- completeness scoring;
- local API and MCP read surfaces for protocols and session state.

Acceptance:

- scripted `tool.v1` fixture produces a valid Tool capsule with transcript-span
  provenance for derived records;
- records remain proposals until Rust validation and review promotion.

### Slice J: Composition, Inspector, Visibility, And Attribution

Status: planned, ADR required.

Governance:
[ADR-010](decisions/ADR-010-visibility-attribution-manifest-fields.md)
proposes the visibility and attribution contract.

Deliver:

- deterministic compile of `1 User + 0..n Situation + 0..n Tool + 1 Output`;
- strictest-wins merge of runtime and output contracts;
- context budget tiers `S`, `M`, and `L`;
- compile-inspector payload with contribution map, compression report, merged
  contract, and compiled hash;
- manifest/listing visibility and attribution fields;
- device/heuristic attribution ids in PRAXIS context packs.

Acceptance:

- permuting input capsule order never changes the compiled hash;
- inspector payload is contract-tested;
- generated fixture memo carries Tool device attribution end to end.

## Documentation Hygiene Proposal

`graphify-out/` is generated output and should not be versioned. Keep Graphify
as a local orientation command (`graphify update .`), then verify important
claims against source files.

After the strategic ADRs are reviewed, create `docs/ARCHIVE/` and move docs
that are historical, duplicated, or replaced by the active build path. The
active tier should remain the 12-file start path in `AGENTS.md` plus accepted
ADRs and narrow API/schema contracts.

Proposed archive candidates after inbound-link review:

- historical planning: `FOUNDATION_BUILD.md`, `IMPLEMENTATION_PHASE_PLAN.md`,
  `IMPLEMENTATION_BLUEPRINT.md`, `ROADMAP.md`, `REPOSITORY_CONCEPT_REVIEW.md`;
- narrative/background: `ABOUT_DIALECTICA.md`, `GITHUB_PROFILE.md`,
  `TECH_BENCHMARK.md`, `CAPSULE_BUILD_EXAMPLES.md`, `INTELLECTUAL_TOOLS.md`,
  `AGENTIC_WORKFLOWS.md`;
- older ops/reference docs once active content is lifted:
  `CI_CD.md`, `DEPLOYMENT.md`, `LOCAL_DEVELOPMENT.md`, `PYTHON_TOOLING.md`,
  `DEPENDENCIES.md`, `OPERATIONS.md`.

Do not archive `CAPSULE_SPEC.md`, `API_CONTRACT.md`, `DATA_MODEL.md`,
`PRAXIS_INTEGRATION.md`, `GRAPH_PROFILE_REGISTRY.md`, accepted ADRs, or any
current audit/ledger file until links and source-of-truth priority are updated.

## Current Executable Surface

Already implemented:

- Rust workspace and service crates;
- typed capsule bundle structs;
- canonical v3 package structs;
- bundle directory loader;
- first validation report with precise findings;
- schema export including `ontology_blueprint.schema.json`;
- CLI `doctor`;
- CLI `welcome`;
- CLI `build-docs`;
- CLI `validate`;
- CLI `inspect`;
- CLI `ontology-plan`;
- CLI `source-pack-check`;
- CLI `proposal-check`;
- CLI `build-plan`;
- CLI `review-draft`;
- CLI `review-check`;
- CLI `promote-check`;
- CLI `compile-reviewed`;
- CLI `build-fixture`;
- CLI `archive`;
- CLI `context-pack`;
- CLI `praxis-pack`;
- CLI `eval`;
- CLI `mcp-config`;
- local document-folder builder;
- JSONL user/assistant discussion capture as a local source type;
- Codex MCP stdio adapter;
- deterministic fixture-mode v3 package writer;
- deterministic `.capsule` archive writer;
- fixture-mode PRAXIS context-pack exporter;
- fixture-backed Axum API health, version, manifest, graph-preview,
  context-pack, and read-receipt routes;
- CLI `schema-export`;
- canonical v3 conflict Situation Capsule fixture;
- golden policy source pack fixture;
- golden policy extraction run and proposal fixtures;
- fixture-mode review-trigger routing for Plus/promoted proposals;
- golden policy reviewer decision fixture;
- fixture-mode promotion normalization into compiler-ready records;
- golden policy expected bundle;
- contract tests for canonical v3 validation, rejected top-level types,
  sourceability, temporal warnings, graph registry, ontology blueprint families,
  and schema snapshots.

Not yet implemented:

- editable review-decision API routes;
- live model-provider extraction calls;
- production-grade Merkle/checksum/signature envelope;
- store-backed HTTP API routes and durable build jobs;
- PostgreSQL migrations;
- PDF/OCR/scanned image/web ingestion and richer conversation adapters;
- interactive human review UI beyond the local editable CLI workflow;
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
cargo run -p dialectica-cli -- source-pack-check fixtures/golden-policy-capsule/source-pack/source_pack.json
cargo run -p dialectica-cli -- proposal-check fixtures/golden-policy-capsule/build_request.json fixtures/golden-policy-capsule/source-pack/source_pack.json fixtures/golden-policy-capsule/proposals
cargo run -p dialectica-cli -- build-plan fixtures/golden-policy-capsule/build_request.json fixtures/golden-policy-capsule/source-pack/source_pack.json fixtures/golden-policy-capsule/proposals
cargo run -p dialectica-cli -- review-draft fixtures/golden-policy-capsule/build_request.json fixtures/golden-policy-capsule/source-pack/source_pack.json fixtures/golden-policy-capsule/proposals --out $env:TEMP\dialectica-review-draft --decided-at 2026-06-10T00:00:00Z
cargo run -p dialectica-cli -- review-check fixtures/golden-policy-capsule/build_request.json fixtures/golden-policy-capsule/source-pack/source_pack.json fixtures/golden-policy-capsule/proposals fixtures/golden-policy-capsule/review-decisions
cargo run -p dialectica-cli -- promote-check fixtures/golden-policy-capsule/build_request.json fixtures/golden-policy-capsule/source-pack/source_pack.json fixtures/golden-policy-capsule/proposals fixtures/golden-policy-capsule/review-decisions
cargo run -p dialectica-cli -- compile-reviewed fixtures/golden-policy-capsule/build_request.json fixtures/golden-policy-capsule/source-pack/source_pack.json fixtures/golden-policy-capsule/proposals fixtures/golden-policy-capsule/review-decisions --out $env:TEMP\dialectica-reviewed-v3
cargo run -p dialectica-cli -- schema-export schemas/capsule-3.0
python -m compileall tools/python
python -m unittest discover tools/python/tests
```

## Phase 1: Source Pack And Extraction Proposal Contract

Status: fixture-mode contract plus editable CLI proof implemented.

Goal: make the input side of the engine real without calling a model provider.

Deliver:

- Rust `SourcePack`, `SourceDocument`, and `SourceSpan` types: implemented;
- source-pack fixture with at least one document-like source and one
  user/assistant discussion source: implemented;
- Rust `ExtractionRun` and `ModelInvocationReceipt` types: implemented;
- Rust `ExtractionProposal` envelope: implemented;
- proposal payloads for claim, episode, graph node, graph edge, ontology term,
  reasoning device, language rule, caveat, rights rule, and output rule:
  implemented;
- review-trigger router: implemented for fixture records;
- CLI validation for source pack and proposal fixture records: implemented.

Acceptance:

- fixture source pack validates locally;
- fixture proposal records validate locally;
- every proposal includes source spans, confidence, uncertainty, model receipt,
  and review triggers;
- no Plus/promoted material proposal can proceed without a blocking review gate;
- tests prove LLM extraction is proposal-only.

Remaining before Phase 1 is product-complete:

- provider traits and live source-bound model calls;
- deterministic rule for which Auto Draft proposals can bypass human review.

## Phase 2: Reviewer Decisions And Promotion Records

Status: fixture-mode contract implemented.

Goal: make the human-gated layer explicit before writing canonical bundle files.

Deliver:

- reviewer decision fixture for the golden proposal set: implemented;
- decision statuses: approve, approve_with_caveats, reject,
  request_more_evidence: implemented;
- promotion policy that turns approved proposals into canonical compiler inputs:
  implemented for fixture mode;
- blocking rule that prevents unreviewed Plus/promoted proposals from compiling:
  implemented;
- lineage preservation for rejected and evidence-requested proposals:
  implemented in promoted-record summaries.
- editable review draft generation with deterministic caveats: implemented
  through `dialectica review-draft`;
- recompilation from edited decisions: implemented through
  `dialectica compile-reviewed`.

Acceptance:

- proposal records and reviewer decisions validate together;
- rejected records remain in lineage but do not enter promoted PRAXIS context;
- caveats propagate to the compiler input contract;
- tests prove a missing reviewer decision blocks promoted output.
- tests prove an edited approval drops draft caveats and an edited rejection
  stays out of promoted output on the golden policy fixture.

## Phase 3: Deterministic Bundle Writer

Status: implemented for local fixture mode.

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

Remaining hardening: byte-for-byte generated fixture comparison, stronger
checksum/signature envelope, and deeper cross-layer validators.

## Phase 4: Source-Pack Builder

Status: implemented for fixture source packs and reviewed proposal records.

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

## Phase 5: PRAXIS Context Pack Export

Status: implemented for local fixture-mode compiled packages.

Goal: create the first PRAXIS-consumable payload from `agent_context.md`,
`operations.md`, and the canonical v3 graph/claim/source files.

Deliver:

- `PraxisContextPack` type in `dialectica-compiler`;
- compact context records with source receipts, temporal status, graph focus
  nodes, ontology blueprint, language rules, stop conditions, and review caveats;
- CLI `context-pack <bundle-dir>`;
- JSON Schema snapshot for `context_pack.schema.json` after the context-pack
  type moves into the stable capsule contract crate.

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

## Phase 6: Local API Slice

Status: implemented for fixture mode with Axum.

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

## Phase 7: Store Migration Skeleton

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

## Phase 8: Deployment Rail

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
