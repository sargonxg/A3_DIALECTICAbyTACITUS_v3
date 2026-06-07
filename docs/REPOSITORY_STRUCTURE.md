# Repository Structure

## Top Level

```text
README.md
AGENTS.md
Cargo.toml
rust-toolchain.toml
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
tools/
.github/
```

## `docs/`

Source-of-truth documentation, architecture decisions, and build plans.

Changes here should explain why the system works the way it does.

## `assets/`

Lightweight repository graphics and documentation assets.

Current assets:

- `dialectica-mark.svg`;
- `capsule-stack.svg`;
- `embedded-graph.svg`.

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

## `tools/`

Auxiliary developer tools.

Current tools:

- `tools/python`: fixture reports, eval helpers, and graph sanity checks.

## `.github/`

GitHub workflows, issue templates, PR template, and CODEOWNERS.

GitHub labels and branch protection should be configured in the repository UI
after the first push.
