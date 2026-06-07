# Lane A Acceptance

Status: required acceptance contract for the first implementation lane.

Lane A makes the capsule contract real. Do not start store, API, model
extraction, or deployment code beyond scaffolding until Lane A is complete.

## Scope

Owned paths:

```text
crates/dialectica-capsule/
crates/dialectica-cli/
tests/dialectica-contract-tests/
schemas/
fixtures/golden-policy-capsule/
docs/CAPSULE_SPEC.md
docs/GRAPH_PROFILE_REGISTRY.md
docs/CODING_LEDGER.md
docs/BUILD_LEDGER.md
```

No-touch without explicit coordination:

```text
services/dialectica-api/
services/dialectica-task-handler/
crates/dialectica-store/
infrastructure/
```

## Required Rust Types

`crates/dialectica-capsule` must define typed structs for:

- `CapsuleManifest`;
- `CapsuleBundleIndex`;
- `SourceLedgerRecord`;
- `SourceSpanRecord`;
- `TemporalLedgerRecord`;
- `OntologySlice`;
- `OntologyTerm`;
- `OntologyMapping`;
- `GraphSlice`;
- `GraphNode`;
- `GraphEdge`;
- `GraphCommunity`;
- `GraphHealth`;
- `ReasoningDevice`;
- `OutputContract`;
- `ReviewLedgerRecord`;
- `RightsProfile`;
- `MarketplaceListing`;
- `CapsuleHealthReport`;
- `ValidationError`.

All exported structs should derive or implement:

- `Debug`;
- `Clone` where reasonable;
- `serde::Serialize`;
- `serde::Deserialize`;
- JSON Schema generation once `schemars` is introduced.

## Schema Outputs

Lane A must create deterministic schema files:

```text
schemas/capsule-0.1.0/manifest.schema.json
schemas/capsule-0.1.0/capsule.schema.json
schemas/capsule-0.1.0/source_ledger.schema.json
schemas/capsule-0.1.0/temporal_ledger.schema.json
schemas/capsule-0.1.0/ontology_slice.schema.json
schemas/capsule-0.1.0/graph_slice.schema.json
schemas/capsule-0.1.0/reasoning_playbook.schema.json
schemas/capsule-0.1.0/output_contracts.schema.json
schemas/capsule-0.1.0/review_ledger.schema.json
schemas/capsule-0.1.0/rights_profile.schema.json
schemas/capsule-0.1.0/marketplace_listing.schema.json
```

## Fixture Outputs

Lane A must create the fixture shell:

```text
fixtures/golden-policy-capsule/
  source-pack/
  expected-bundle/
    manifest.json
    capsule.json
    source_ledger.jsonl
    temporal_ledger.jsonl
    ontology_slice.json
    graph_slice.json
    graph_semantics.jsonld
    graph_constraints.json
    reasoning_playbook.json
    retrieval_pack.jsonl
    output_contracts.json
    review_ledger.jsonl
    rights_profile.json
    marketplace_listing.json
    capsule_health.json
    eval_report.json
```

The fixture must include:

- at least five source records;
- at least one source span per factual claim;
- at least one stale or superseded claim;
- at least one contested claim;
- at least one graph edge in `approved_with_caveats`;
- at least one rejected or blocked graph object;
- at least one reasoning device;
- at least one output contract;
- at least one rights rule that blocks a workflow.

## Validation Error Shape

All validation errors must serialize to this shape:

```json
{
  "code": "missing_required_field",
  "path": "graph_slice.edges[0].source_span_ids",
  "message": "Graph edge must include at least one source span or review action.",
  "severity": "error",
  "object_id": "edge:guidelines-regulated-by-commission",
  "help": "Add source_span_ids or review_action_ids before promotion."
}
```

Allowed severities:

- `error`;
- `warning`;
- `info`.

## CLI Commands

Lane A must support:

```powershell
cargo run -p dialectica-cli -- doctor
cargo run -p dialectica-cli -- validate fixtures/golden-policy-capsule/expected-bundle
cargo run -p dialectica-cli -- inspect fixtures/golden-policy-capsule/expected-bundle
cargo run -p dialectica-cli -- schema-export schemas/capsule-0.1.0
```

`validate` must fail non-zero for invalid bundles.

`inspect` must print:

- capsule id;
- capsule type;
- review state;
- source count;
- claim count;
- graph node and edge counts;
- warnings.

## Tests

Required tests:

- valid fixture bundle passes validation;
- missing source span fails validation;
- unregistered graph edge fails validation unless it has an approved alias;
- unreviewed critical graph edge blocks promotion;
- stale temporal claim produces warning;
- rejected review object remains in lineage but is blocked from context pack;
- schema export is deterministic.

## Completion Gate

Lane A is complete only when these pass:

```powershell
cargo fmt --all -- --check
cargo check --locked --workspace --all-targets
cargo clippy --locked --workspace --all-targets -- -D warnings
cargo test --locked --workspace
cargo run -p dialectica-cli -- doctor
cargo run -p dialectica-cli -- validate fixtures/golden-policy-capsule/expected-bundle
python -m compileall tools/python
python -m unittest discover tools/python/tests
```

## Handoff

When Lane A completes, update:

- `docs/CODING_LEDGER.md`;
- `docs/SCAFFOLD_AUDIT.md`;
- `docs/BUILD_LEDGER.md`;
- `docs/CAPSULE_SPEC.md` if schema behavior changed;
- `docs/GRAPH_PROFILE_REGISTRY.md` if graph classes changed.
