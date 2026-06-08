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
cargo run -p dialectica-cli -- doctor
python -m compileall tools/python
python -m unittest discover tools/python/tests
python -m json.tool fixtures/example-capsules/user-capsule.example.json > $null
python -m json.tool fixtures/example-capsules/situation-capsule.example.json > $null
python -m json.tool fixtures/example-capsules/tool-capsule.example.json > $null
python -m json.tool fixtures/example-capsules/output-capsule.example.json > $null
```

Future fixture commands:

```powershell
cargo run -p dialectica-cli -- validate fixtures/golden-policy-capsule/expected-bundle
cargo run -p dialectica-cli -- inspect fixtures/golden-policy-capsule/expected-bundle
```

## Planned Environment Variables

See `.env.example`.

Local-only values:

- `DIALECTICA_ENV=local`
- `DIALECTICA_DATABASE_URL=postgres://postgres:postgres@localhost:5432/dialectica`
- `DIALECTICA_ARTIFACT_ROOT=./artifacts`
- `DIALECTICA_SCHEMA_VERSION=0.1.0`

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
3. Parse example capsule envelopes.
4. Validate golden fixture.
5. Inspect bundle output.
6. Run source and temporal evals.
7. Update docs if contract behavior changed.

## Windows Notes

Prefer PowerShell-compatible commands in docs and scripts.

Do not require WSL for the base local workflow unless a later ADR accepts that
constraint.
