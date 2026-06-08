# Local Development

Status: active scaffold workflow before full implementation.

## Requirements

Expected tools:

- Rust toolchain pinned by `rust-toolchain.toml`;
- Python 3.11+ for auxiliary tools;
- Docker Desktop or compatible container runtime;
- PostgreSQL for local store tests;
- PowerShell on Windows;
- Google Cloud CLI only for staging/deployment phases.

## Local Principle

The first developer workflow must run without cloud credentials.

Cloud credentials are only required for:

- staging deploy;
- Cloud SQL integration;
- Cloud Storage artifact tests;
- Secret Manager wiring.

## Current Commands

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
cargo run -p dialectica-cli -- ladybug-check fixtures/canonical-capsules/conflict-situation-capsule
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
python -m json.tool fixtures/example-capsules/user-capsule.example.json > $null
python -m json.tool fixtures/example-capsules/situation-capsule.example.json > $null
python -m json.tool fixtures/example-capsules/tool-capsule.example.json > $null
python -m json.tool fixtures/example-capsules/output-capsule.example.json > $null
```

Source/proposal fixture commands:

```powershell
cargo run -p dialectica-cli -- source-pack-check fixtures/golden-policy-capsule/source-pack/source_pack.json
cargo run -p dialectica-cli -- proposal-check fixtures/golden-policy-capsule/build_request.json fixtures/golden-policy-capsule/source-pack/source_pack.json fixtures/golden-policy-capsule/proposals
cargo run -p dialectica-cli -- build-plan fixtures/golden-policy-capsule/build_request.json fixtures/golden-policy-capsule/source-pack/source_pack.json fixtures/golden-policy-capsule/proposals
cargo run -p dialectica-cli -- review-check fixtures/golden-policy-capsule/build_request.json fixtures/golden-policy-capsule/source-pack/source_pack.json fixtures/golden-policy-capsule/proposals fixtures/golden-policy-capsule/review-decisions
cargo run -p dialectica-cli -- promote-check fixtures/golden-policy-capsule/build_request.json fixtures/golden-policy-capsule/source-pack/source_pack.json fixtures/golden-policy-capsule/proposals fixtures/golden-policy-capsule/review-decisions
cargo run -p dialectica-cli -- build-fixture fixtures/golden-policy-capsule --out $env:TEMP\dialectica-golden-v3
cargo run -p dialectica-cli -- context-pack $env:TEMP\dialectica-golden-v3 --workflow conflict_map
```

These commands run without cloud credentials and do not call model providers.

Local document capsule builder:

```powershell
cargo run -p dialectica-cli -- build-docs --type situation --input .\docs --out $env:TEMP\dialectica-doc-capsule --title "Local Situation Capsule" --workflow decision_brief
cargo run -p dialectica-cli -- validate $env:TEMP\dialectica-doc-capsule\package
cargo run -p dialectica-cli -- inspect $env:TEMP\dialectica-doc-capsule\package
```

Codex MCP registration:

```powershell
cargo run -p dialectica-cli -- mcp-config
cargo run -p dialectica-mcp
```

Local API preview:

```powershell
$env:DIALECTICA_FIXTURE_CAPSULE_DIR = "$PWD\fixtures\canonical-capsules\conflict-situation-capsule"
cargo run -p dialectica-api
```

The API serves `/health`, `/version`,
`/v1/capsules/{capsule_id}/manifest`,
`/v1/capsules/{capsule_id}/graph-preview`,
`/v1/capsules/{capsule_id}/praxis-context-pack`, and fixture read receipts.

Optional migration-fixture commands:

```powershell
cargo run -p dialectica-cli -- inspect fixtures/golden-policy-capsule/expected-bundle
cargo run -p dialectica-cli -- ontology-plan fixtures/golden-policy-capsule/expected-bundle
cargo run -p dialectica-cli -- schema-export schemas/capsule-3.0
```

## Planned Environment Variables

See `.env.example`.

Local-only values:

- `DIALECTICA_ENV=local`
- `DIALECTICA_DATABASE_URL=postgres://postgres:postgres@localhost:5432/dialectica`
- `DIALECTICA_ARTIFACT_ROOT=./artifacts`
- `DIALECTICA_CAPSULE_SPEC_VERSION=3.0`
- `DIALECTICA_LEGACY_BUNDLE_SCHEMA_VERSION=0.1.0`

## Local Data

Do not use private policy documents as committed fixtures.

Allowed fixture content:

- synthetic policy memos;
- public-domain documents;
- short excerpted test documents with clear provenance;
- generated source packs for contract behavior.

## Local Validation Loop

1. Run formatter.
2. Run unit tests.
3. Validate and inspect the canonical v3 fixture.
4. Parse example capsule envelopes.
5. Validate the legacy migration fixture.
6. Inspect bundle output.
7. Run source and temporal evals after `dialectica-eval` is real.
8. Update docs if contract behavior changed.

## Windows Notes

Prefer PowerShell-compatible commands in docs and scripts.

Do not require WSL for the base local workflow unless a later ADR accepts that
constraint.
