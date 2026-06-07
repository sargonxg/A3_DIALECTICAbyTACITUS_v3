# ADR-004: Rust Service Stack

Status: accepted

Date: 2026-06-07

## Context

DIALECTICA v3 needs a typed, testable, contract-heavy runtime for capsule
schemas, validators, APIs, workers, and deterministic bundle compilation.

The stack should support local CLI workflows, Cloud Run services, Cloud Tasks
handlers, PostgreSQL, JSON Schema, and structured tracing.

## Decision

Use Rust with:

- `tokio` for async runtime;
- `axum` for HTTP APIs and task handlers;
- `serde` and `serde_json` for data contracts;
- `schemars` for JSON Schema generation;
- `sqlx` for PostgreSQL access and migrations;
- `tracing` for structured logs and spans;
- `thiserror` for library errors;
- `anyhow` for CLI and binary boundaries;
- `clap` for command-line tools;
- `insta` for fixture snapshots.

## Evidence

Tokio is the established async runtime in the Rust ecosystem and documents
network application building blocks, I/O, timers, filesystem, synchronization,
and scheduling facilities.

Axum is maintained by the Tokio project and is explicitly focused on ergonomic,
modular HTTP routing and request handling.

SQLx supports PostgreSQL and the Tokio runtime, with feature flags for runtime
and TLS configuration.

Schemars provides JSON Schema generation and schema value handling suitable for
contract-driven capsule validation.

Source anchors:

- <https://tokio.rs/>
- <https://github.com/tokio-rs/axum>
- <https://github.com/launchbadge/sqlx>
- <https://docs.rs/schemars/latest/schemars/>

## Alternatives Considered

### TypeScript/Node

Pros:

- fast iteration;
- strong web ecosystem;
- aligns with PRAXIS frontend.

Cons:

- weaker fit for deterministic, binary, contract-heavy engine work;
- runtime dependency and packaging complexity for Cloud Run workers;
- less compelling for a long-term engine core.

### Python

Pros:

- strong AI and document-processing ecosystem;
- fast prototyping.

Cons:

- weaker compile-time contract guarantees;
- harder to keep service binaries small and predictable;
- better as an adapter language than the engine core.

### Go

Pros:

- simple services;
- good Cloud Run fit.

Cons:

- less expressive type modeling than Rust for capsule contracts;
- less attractive for eventual parsing/compilation engine work.

## Consequences

Positive:

- strong contract modeling;
- good Cloud Run binary story;
- clear API and worker stack;
- deterministic local tooling;
- safer refactoring as capsule schemas evolve.

Negative:

- slower initial iteration than TypeScript or Python;
- contributors need Rust competence;
- AI/model SDK integrations may require adapter boundaries.
