# Local Development

Status: planned workflow before implementation.

## Requirements

Expected tools:

- Rust stable toolchain;
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

## Planned Commands

```powershell
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo run -p dialectica-cli -- validate fixtures/golden-policy-capsule
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
3. Validate golden fixture.
4. Inspect bundle output.
5. Run source and temporal evals.
6. Update docs if contract behavior changed.

## Windows Notes

Prefer PowerShell-compatible commands in docs and scripts.

Do not require WSL for the base local workflow unless a later ADR accepts that
constraint.
