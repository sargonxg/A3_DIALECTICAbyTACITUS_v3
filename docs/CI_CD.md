# CI/CD

## Goal

Keep `main` always able to build documentation and, once code exists, validate
the capsule contract.

## Current CI

The repository currently includes a lightweight GitHub Actions workflow:

```text
.github/workflows/docs.yml
```

It verifies required source-of-truth docs exist.

## First Code CI

When Rust code lands, CI should run:

```powershell
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo run -p dialectica-cli -- validate fixtures/golden-policy-capsule
```

## Later Staging CI

After containerization:

- build container image;
- scan image;
- push to Artifact Registry;
- deploy to Cloud Run staging;
- run `/healthz`, `/readyz`, `/version`;
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
