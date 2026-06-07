# Security Policy

## Reporting

This repository is proprietary. Report security issues privately to TACITUS
project maintainers. Do not open public issues containing secrets, exploit
details, private documents, or customer data.

## Scope

Security review is required for:

- authentication and authorization;
- tenant isolation;
- source ingestion;
- capsule export;
- review workflows;
- model provider integration;
- secrets handling;
- deployment infrastructure.

## Required Practices

- Never commit secrets.
- Use Secret Manager or environment variables for credentials.
- Treat source documents as untrusted input.
- Preserve provenance for model-derived claims.
- Validate capsule bundles before PRAXIS consumption.
- Keep service accounts least-privilege.
- Review dependency and container image risks before production deploy.
