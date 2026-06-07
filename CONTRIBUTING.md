# Contributing

DIALECTICA is open source under the Apache License 2.0. Contributions should be
scoped, documented, and tied to the capsule build plan.

By submitting a contribution, you agree that it is provided under the Apache
License 2.0 unless a separate written agreement with TACITUS says otherwise.

## Before You Start

Read:

- `README.md`
- `LICENSE`
- `NOTICE`
- `CITATION.cff`
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
