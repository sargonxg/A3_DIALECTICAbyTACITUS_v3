# Contributing

DIALECTICA is currently proprietary TACITUS source. Contributions should be
scoped, documented, and tied to the capsule build plan.

## Before You Start

Read:

- `README.md`
- `docs/SOURCE_OF_TRUTH.md`
- `docs/DIALECTICA_v3_BUILD_INSTRUCTIONS.md`
- `docs/CAPSULE_SPEC.md`
- `docs/BUILD_LEDGER.md`

## Pull Requests

Every pull request should include:

- what changed;
- why it matters for capsules or PRAXIS;
- files touched;
- validation run;
- schema or migration impact;
- deployment impact;
- security/privacy impact.

## Architecture Changes

Create an ADR for changes that affect:

- capsule bundle format;
- canonical store;
- deployment platform;
- model provider strategy;
- required graph or semantic engines;
- PRAXIS integration contract;
- review or promotion gates.

## Code Quality

When implementation begins, prefer:

- small PRs;
- contract tests before broad features;
- deterministic fixtures;
- explicit migrations;
- source-grounded model behavior;
- clear failure modes.
