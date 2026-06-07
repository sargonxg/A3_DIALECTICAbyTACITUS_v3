# ADR-001: Capsule Bundle as Source of Truth

Status: accepted

Date: 2026-06-07

## Context

DIALECTICA needs to create context objects that PRAXIS can store, share,
combine, inspect, and use across agentic workflows. If the capsule only exists
as rows in a database or hidden graph state, it is harder to audit, move,
version, test, and reason about.

## Decision

The signed capsule bundle is the portable product contract.

Operational databases and graph/semantic adapters may generate, cache, index, or
serve bundle data, but the exported bundle must remain inspectable and
self-describing.

## Consequences

Positive:

- PRAXIS can consume capsules through a stable contract.
- Capsules can be archived and shared.
- Evals can run against deterministic fixtures.
- Review and provenance can travel with the capsule.
- Future storage engines remain replaceable.

Negative:

- The compiler must maintain compatibility logic.
- Bundles need validation, signatures, and checksums.
- Some derived views may duplicate database state.

## Acceptance Criteria

- A capsule can be exported as a directory or archive.
- A validator can check required files and references.
- PRAXIS can read the manifest and context pack without database access.
- The source ledger and review ledger are included in the bundle.
