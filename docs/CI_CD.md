# CI/CD

## Goal

Keep `main` always able to build documentation and, once code exists, validate
the capsule contract.

## Current CI

The repository currently includes a lightweight GitHub Actions workflow:

```text
.github/workflows/docs.yml
```

It verifies required source-of-truth docs exist and checks the Rust workspace.

## First Code CI

Current code CI runs:

```powershell
cargo fmt --all -- --check
cargo check --locked --workspace --all-targets
cargo clippy --locked --workspace --all-targets -- -D warnings
cargo test --locked --workspace
cargo run -p dialectica-cli -- doctor
python -m compileall tools/python
python -m unittest discover tools/python/tests
python -m json.tool fixtures/example-capsules/user-capsule.example.json
python -m json.tool fixtures/example-capsules/situation-capsule.example.json
python -m json.tool fixtures/example-capsules/thinking-device-capsule.example.json
python -m json.tool fixtures/example-capsules/output-capsule.example.json
```

Add full fixture validation once `fixtures/golden-policy-capsule` exists.

## Later Staging CI

After containerization:

- build container image;
- scan image;
- push to Artifact Registry;
- deploy to Cloud Run staging;
- run `/health`, `/version`;
- add `/healthz` and `/readyz` only when deployment probes require separate
  liveness and readiness checks;
- compile or serve one fixture capsule;
- attach eval report to release notes.

## Production Release Gate

Production release requires:

- source-of-truth docs current;
- schema compatibility documented;
- migrations reviewed;
- contract tests passing;
- eval report passing;
- image scan reviewed;
- secrets stored outside GitHub;
- rollback plan in `docs/BUILD_LEDGER.md`;
- explicit approval to deploy.

## Branch Protection Recommendation

Once the first commit is pushed, configure GitHub branch protection for `main`:

- require pull request before merge;
- require status checks;
- require conversation resolution;
- prevent force pushes;
- restrict who can dismiss reviews.
