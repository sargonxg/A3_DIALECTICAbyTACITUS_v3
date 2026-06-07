# Repository Concept Review

Date: 2026-06-07

Status: working audit for repository narrative, product coherence, and
implementation readiness.

## Review Scope

This pass reviewed the repository as a GitHub front door for DIALECTICA by
TACITUS:

- README narrative and diagrams;
- product boundary between TACITUS, PRAXIS, and DIALECTICA;
- capsule contract and example fixtures;
- human-gated knowledge, reasoning, language, and review story;
- engineering scaffold for Rust, PostgreSQL, Cloud Run, CI, and PRAXIS
  integration;
- GitHub About metadata readiness.

## Current Assessment

The concept is coherent:

- TACITUS is the company and product ecosystem.
- PRAXIS is the visible workbench and agentic workflow cockpit.
- DIALECTICA is the engine that builds, improves, reviews, signs, stores, and
  serves PRAXIS Capsules.
- PRAXIS Capsules are portable knowledge-work objects that humans and AI agents
  can inspect and use interchangeably.

The strongest product idea is not "better memory." It is a reviewed context
backbone for serious knowledge work: source-grounded facts, time, ontology,
embedded graph, reasoning devices, language rules, rights, and review gates
compiled into a reusable object.

## Audience Check

### Software Engineers

Engineers should see a buildable backend:

- Rust workspace and command gates;
- typed capsule bundle contract;
- PostgreSQL-first canonical store;
- signed export bundle;
- Cloud Run first deployment path;
- fixture examples and CI checks;
- adapter posture for graph, semantic, and memory systems.

The next engineering improvement is to implement Lane A schemas and validators
against the example capsules.

### Policy Teams

Policy teams should see why this is not another chat layer:

- source spans and review receipts;
- temporal status and decision horizons;
- embedded graph for actors, claims, sources, risks, and decisions;
- expert reasoning devices and failure modes;
- language profiles that preserve approved terminology, caveats, and audience
  register;
- human promotion gates before context affects high-stakes work.

The next product improvement is a larger golden policy capsule that proves the
workflow from source pack to PRAXIS context pack.

### Investors And Operators

Investors should see a defensible layer:

- PRAXIS becomes stronger as the capsule library grows;
- expert-reviewed methods and language become reusable assets;
- marketplace listings can carry rights, review level, lineage, freshness, and
  eval evidence;
- the core backend is deployable without prematurely requiring Kubernetes or a
  heavy graph database;
- optional adapters can improve retrieval without displacing the canonical
  signed bundle.

The next business proof is a clear before/after evaluation: PRAXIS with a
capsule should produce a more source-faithful, temporally disciplined, and
expert-shaped output than raw prompting.

## Improvements Made In This Pass

- Added a stronger README opening for engineers, policy teams, and investors.
- Clarified the TACITUS system map: PRAXIS is visible, DIALECTICA is the
  capsule engine, AGON/KAIROS are future inputs.
- Promoted human-gated language to a first-class capsule layer.
- Added `language_profile.json` to the capsule bundle contract.
- Added language profile expectations to PRAXIS context packs and read
  receipts.
- Added example `language_profile` sections to the four example capsule
  envelopes.
- Added a GitHub Profile document with recommended description, website, and
  topics.

## Remaining Product Gaps

- The actual GitHub repository About metadata is still empty until explicitly
  updated through GitHub.
- The example capsules are intentionally small; the repo still needs a richer
  golden policy capsule with multiple sources, contested claims, review
  objects, graph warnings, and measured PRAXIS output improvement.
- The current diagrams explain the architecture, but a future PRAXIS-facing
  demo should show a loaded capsule moving from graph/source/reasoning/language
  to a generated brief with receipts.
- The language profile is now specified, but Lane A must turn it into typed
  structs, schema output, validation errors, and fixture snapshots.

## Concept Bar

DIALECTICA should be judged by one question:

Can it build a capsule that a human analyst, expert reviewer, and PRAXIS agent
can all use without losing sources, time, graph context, reasoning method,
language discipline, rights, or review state?

If yes, it becomes the backbone of PRAXIS augmented generation. If not, it is
below the TACITUS product bar.
