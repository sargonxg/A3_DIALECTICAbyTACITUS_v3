# Lane A Acceptance

Status: required acceptance contract for the first implementation lane.

Lane A makes the canonical capsule contract real. Do not start store, API,
model extraction, or deployment code beyond scaffolding until Lane A can
validate a v3 PRAXIS Capsule package.

## Scope

Owned paths:

```text
crates/dialectica-capsule/
crates/dialectica-cli/
tests/dialectica-contract-tests/
schemas/
fixtures/canonical-capsules/
fixtures/golden-policy-capsule/
docs/CAPSULE_SPEC.md
docs/GRAPH_PROFILE_REGISTRY.md
docs/CAPSULE_STRUCTURE_GUIDE.md
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

## Canonical Contract

Lane A targets [Capsule Spec v3.0](CAPSULE_SPEC.md). The production artifact is
a `.capsule` Zip archive. Tests may validate the extracted directory projection.

Required canonical files:

```text
<title-slug>.<short-id>.capsule
├── mimetype
├── manifest.json
├── claims.jsonl
├── graph.jsonld
├── episodes.json
├── evidence/
│   └── sources.jsonl
├── reasoning/
│   ├── devices.json
│   ├── annotations.json
│   └── traps.json
├── review/
│   └── review.json
├── runtime.json
├── agent_context.md
└── operations.md
```

`payload.user.json`, `payload.tool.json`, and `payload.output.json` are required
for User, Tool, and Output Capsules. Situation Capsules may use the substrate
files as their payload.

`graph/ladybug/` is required for promoted v3 capsules and must remain
regenerable from `graph.jsonld` with digest-verified projection receipts.
Other cache material remains optional and outside the canonical validation
surface.

## Required Rust Types

`crates/dialectica-capsule` must define typed structs or validators for:

- `PraxisCapsuleManifest`;
- `PraxisCapsulePackage`;
- `CapsuleManifest` legacy compatibility;
- `CapsuleBundle` legacy compatibility;
- source/evidence records;
- claim records;
- episode records;
- ontology/semantic layer records;
- graph records;
- reasoning devices, annotations, traps, heuristics;
- review and trust records;
- runtime contract records;
- output contract records;
- validation findings.

All exported structs should derive or implement:

- `Debug`;
- `Clone` where reasonable;
- `serde::Serialize`;
- `serde::Deserialize`;
- JSON Schema generation where the type is part of the public contract.

## Schema Outputs

Lane A must export v3 schemas:

```text
praxis_capsule_manifest.schema.json
praxis_capsule_package.schema.json
```

Legacy schema outputs under `schemas/capsule-0.1.0/` remain compatibility
snapshots until the compiler is fully migrated. New compiler work must target
`schemas/capsule-3.0/` once the v3 typed model is complete.

## Fixture Outputs

Required canonical fixture:

```text
fixtures/canonical-capsules/conflict-situation-capsule/
```

It must include:

- v3 `manifest.json`;
- the PRAXIS Capsule MIME marker;
- at least one claim;
- at least one evidence source;
- JSON-LD graph named graphs for evidence, claims, situation, temporal,
  ontology, reasoning, governance, and runtime;
- at least one reasoning device;
- at least one annotation;
- at least one trap;
- review and trust-layer data;
- runtime verb contract;
- generated `agent_context.md`;
- generated `operations.md`.

Legacy fixture:

```text
fixtures/golden-policy-capsule/expected-bundle/
```

This is retained to keep earlier tests useful, but it is not the product
contract. Compiler work must migrate it to the v3 shape or generate both legacy
and v3 projections only during transition.

## Validation Finding Shape

All validation findings must serialize to this shape:

```json
{
  "code": "missing_v3_bundle_file",
  "path": "graph.jsonld",
  "message": "Required v3 capsule file 'graph.jsonld' is missing.",
  "severity": "error",
  "object_id": "cap_123",
  "help": "Compile the capsule using the v3 bundle shape from docs/CAPSULE_SPEC.md."
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
cargo run -p dialectica-cli -- validate fixtures/canonical-capsules/conflict-situation-capsule
cargo run -p dialectica-cli -- inspect fixtures/canonical-capsules/conflict-situation-capsule
cargo run -p dialectica-cli -- validate fixtures/golden-policy-capsule/expected-bundle
cargo run -p dialectica-cli -- schema-export schemas/capsule-3.0
```

`validate` must fail non-zero for invalid v3 packages or invalid legacy bundles.

`inspect` must print:

- capsule id;
- capsule type;
- review state placeholder or review state;
- source count;
- claim count;
- graph node and edge counts when available;
- warnings.

## Tests

Required tests:

- canonical v3 Situation Capsule fixture passes validation;
- canonical v3 rejects non-macro top-level types such as `stakeholder`;
- missing v3 required files fail validation;
- legacy golden bundle still passes until migration is complete;
- stale temporal claim in the legacy fixture produces a warning;
- unsupported legacy top-level capsule type fails validation;
- schema export includes v3 manifest/package schemas;
- example capsules under `fixtures/example-capsules/` parse and share the same
  top-level bundle sections until they are migrated.

## Completion Gate

Lane A is complete only when these pass:

```powershell
cargo fmt --all -- --check
cargo check --locked --workspace --all-targets
cargo clippy --locked --workspace --all-targets -- -D warnings
cargo test --locked --workspace
cargo run -p dialectica-cli -- doctor
cargo run -p dialectica-cli -- validate fixtures/canonical-capsules/conflict-situation-capsule
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
