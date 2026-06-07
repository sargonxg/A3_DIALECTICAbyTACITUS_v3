# Repository Structure

## Top Level

```text
README.md
AGENTS.md
Cargo.toml
LICENSE
NOTICE
SUPPORT.md
CONTRIBUTING.md
SECURITY.md
CODE_OF_CONDUCT.md
assets/
docs/
crates/
services/
infrastructure/
fixtures/
tests/
.github/
```

## `docs/`

Source-of-truth documentation, architecture decisions, and build plans.

Changes here should explain why the system works the way it does.

## `assets/`

Lightweight repository graphics and documentation assets.

The current SVG mark is for GitHub presentation, not final TACITUS brand
identity.

## `crates/`

Reusable Rust libraries.

Active crates:

- `dialectica-capsule`;
- `dialectica-store`;
- `dialectica-compiler`;
- `dialectica-eval`;
- `dialectica-cli`.

## `services/`

Deployable Rust service binaries.

Active services:

- `dialectica-api`;
- `dialectica-task-handler`;

Add optional `dialectica-worker` only after pull-based background processing is
proven necessary.

## `infrastructure/`

Terraform/OpenTofu, deployment manifests, and environment configs.

Cloud Run is first. GKE manifests should not appear here until an ADR approves
Kubernetes for a specific workload.

## `fixtures/`

Deterministic source packs, capsule bundles, and eval artifacts.

Fixtures must not contain secrets or private user documents.

## `tests/`

Cross-crate, contract, integration, and fixture tests.

Active test package:

- `dialectica-contract-tests`.

## `.github/`

GitHub workflows, issue templates, PR template, and CODEOWNERS.

GitHub labels and branch protection should be configured in the repository UI
after the first push.
