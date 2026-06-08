# Improvement Guidelines

Date: 2026-06-08

Status: active quality and gap-control guide for the first executable build.

## Purpose

This guide records the improvement pass after the repository front door,
capsule contract, ontology planner, and next-code plan were established. Use it
before starting a coding session, after finishing a coding session, and whenever
the plan starts to drift into broad infrastructure instead of a working capsule
engine.

The standard is simple:

> DIALECTICA is improving only when PRAXIS receives a more source-faithful,
> temporally aware, review-gated, and agent-usable capsule than it had before.

## Current Gap Audit

The current plan is directionally right, but these gaps must be handled
explicitly while coding.

| Priority | Gap | Why it matters | Required improvement |
| --- | --- | --- | --- |
| P0 | Deterministic bundle output is underspecified. | A capsule cannot be trusted, signed, diffed, or cached if two equal inputs produce different bytes. | Define canonical JSON/JSONL ordering, newline rules, digest scope, and fixture comparison tests before calling the compiler real. |
| P0 | Source-pack input is fixture-only. | The repo can validate source/proposal records, but it still cannot ingest real documents, PDFs, or conversations. | Keep fixtures canonical, add reviewer decisions and promotion records next, then live ingestion after the compiler loop works. |
| P0 | LLM extraction proposal records are fixture-only. | The engine has proposal contracts, but no live model-provider calls or fallback policy. | Add live providers only after proposal promotion and eval gates exist. |
| P0 | Promotion gates are too coarse. | Manifest-level review is not enough; PRAXIS must avoid unreviewed claims, graph edges, language rules, and output contracts. | Add object-level promotion policy and tests for rejected, expired, caveated, stale, and unreviewed objects. |
| P0 | PRAXIS context pack is still conceptual. | The product value is proven only when PRAXIS can consume a compact capsule payload without internal DIALECTICA state. | Implement `ContextPack`, schema export, CLI export, and fixture assertions before store/API/cloud work. |
| P1 | API behavior needs a stricter local contract. | Local Axum routes should not become ad hoc JSON endpoints. | Define response envelopes, error codes, content types, fixture mode metadata, and stable route tests. |
| P1 | Evals are scheduled too late in the plan. | If evals wait until the end, the compiler may optimize for valid structure instead of better policy output. | Add fixture eval scaffolding as soon as context-pack export exists, even before model-powered extraction. |
| P1 | Checksums and signatures are placeholders. | This is the trust boundary for portable capsules and marketplace-style reuse. | Add digest verification before signing; signing can remain placeholder until key management is designed. |
| P1 | Store migrations may pull the team into infrastructure early. | PostgreSQL matters, but it should not block local capsule proof. | Keep migrations after fixture build and context pack; design repository traits so local fixture mode remains first-class. |
| P2 | Example capsules are not yet validated by one shared envelope. | Examples can drift into marketing samples instead of executable contracts. | Add one validator path for user, situation, tool, and output examples. |
| P2 | Research ledger is strong but not yet release-gated. | Future agents may add graph or memory adapters because they look attractive. | Require an ADR and eval evidence before promoting any graph, memory, vector, or MCP adapter to a required dependency. |

## Improvement Series

Use this sequence when improving the repository. Do not skip ahead to cloud,
marketplace, graph databases, or PRAXIS production integration before the local
loop is executable.

### Series 1: Make The Compiler Real

Prerequisite: source-pack and proposal records from Series 2 must exist before
the compiler consumes extracted context. For implementation, Series 1 and Series
2 should move together in small vertical slices.

Deliver:

- canonical JSON serialization;
- canonical JSONL serialization;
- required bundle file inventory;
- generated manifest fields;
- bundle digest calculation;
- compiler receipt;
- byte-stability tests against a temp directory;
- golden fixture comparison tests.

Definition of done:

- running the compiler twice on the same fixture produces byte-identical
  output;
- changing one source record changes the digest;
- missing review state blocks promoted export;
- generated output can be validated by the current CLI.

### Series 2: Make The Source Pack Real

Deliver:

- `source_pack.json` schema and Rust types: implemented;
- source artifact metadata: implemented for fixtures;
- normalized source-span records: implemented for fixtures;
- machine proposal records: implemented for fixtures;
- model invocation receipts: implemented for fixtures;
- review-trigger routing: implemented for Plus/promoted fixtures;
- human correction records;
- review decision records;
- lineage from every generated object to source spans or review actions.

Definition of done:

- `build-fixture` regenerates the golden policy bundle from source-pack inputs;
- proposed but unreviewed records remain visible in lineage;
- LLM-generated proposals cannot become canonical records directly;
- PRAXIS context export excludes rejected, expired, and unreviewed objects by
  default.

### Series 3: Make The PRAXIS Context Pack Real

Deliver:

- `ContextPack` type;
- `context_pack.schema.json`;
- CLI `context-pack <bundle-dir>`;
- compact retrieval records;
- graph focus records;
- temporal warnings;
- language and output rules;
- read-receipt hints;
- stop conditions and handoff policy.

Definition of done:

- PRAXIS can consume the JSON without PostgreSQL, Firestore, or a graph
  database;
- stale and contested claims appear as warnings;
- blocked workflows and rights constraints are visible;
- every included object has source-span ids, review-action ids, or explicit
  expert-note lineage.

### Series 4: Make The Local API Boring And Deterministic

Deliver:

- Axum service in fixture mode;
- `GET /health`;
- `GET /version`;
- manifest route;
- graph-preview route;
- PRAXIS context-pack route;
- deterministic error envelope;
- route-level tests.

Definition of done:

- the API starts without cloud credentials;
- all routes return stable JSON for the golden fixture;
- errors include code, message, details, and request id;
- no route exposes internal graph-engine or model-provider details.

### Series 5: Add Storage Without Breaking Local Proof

Deliver:

- SQLx migrations;
- repository traits;
- idempotency keys;
- local Postgres runbook;
- fixture adapter preserved for tests.

Definition of done:

- migrations apply from an empty database;
- local fixture build still works when no database exists;
- repository tests cover write, read, duplicate, and rollback paths.

### Series 6: Add Deployment Only After Runtime Proof

Deliver:

- Dockerfile;
- Cloud Run service config;
- Cloud Run job config;
- task-handler HTTP target;
- Secret Manager variable list;
- deploy workflow skeleton.

Definition of done:

- containers build locally;
- deployment config is reviewable;
- no Kubernetes, graph database, vector database, or remote PRAXIS dependency
  becomes required for local development.

## Capsule Quality Bar

Every promoted capsule must satisfy these ten invariants.

1. Sourceable: material claims, terms, edges, language rules, and output rules
   point to source spans, review actions, or explicit expert notes.
2. Temporal: claims distinguish valid time, observed time, recorded time,
   freshness, supersession, and uncertainty.
3. Review-gated: PRAXIS-visible objects have review state, scope, caveats, and
   expiry where needed.
4. Capsule-specific: ontology and semantic layers fit the capsule type and
   workflow instead of forcing every capsule into one actor/claim graph.
5. Portable: the bundle can be read outside DIALECTICA services.
6. Deterministic: identical inputs produce identical bundle bytes and digest.
7. Rights-bounded: permissions, prohibitions, duties, sharing limits, and
   blocked workflows travel with the capsule.
8. Agent-usable: PRAXIS receives compact guidance for retrieval, reasoning,
   citation, graph use, stopping, and handoff.
9. Evaluated: at least one fixture or eval proves why the capsule is better
   than loose context for the intended workflow.
10. Observable: every build and export produces receipts that future agents and
    reviewers can inspect.

## Coding Guidelines

- Start each session from `docs/CODING_LEDGER.md` and this guide.
- Keep the active path short. Deep docs are allowed, but README, `AGENTS.md`,
  and `docs/README.md` must make the next build step obvious in ten links or
  fewer.
- Keep each slice vertical: contract, fixture, CLI, tests, docs.
- Add model-provider calls only after deterministic source-pack and proposal
  fixtures work locally.
- Add cloud only after local API and context-pack export work.
- Add graph or memory adapters only behind an ADR, adapter boundary, and eval.
- Keep Postgres and the signed bundle canonical for DIALECTICA.
- Keep Firestore as the PRAXIS visibility mirror, not the DIALECTICA truth
  store.
- Treat every external input as untrusted, including uploaded documents,
  source metadata, reviewer notes, model outputs, and prior capsules.
- Update `docs/CODING_LEDGER.md` or `docs/BUILD_LEDGER.md` when executable
  behavior changes.
- Do not call a capability real until there is a command, fixture, and test
  proving it.

## Review Guidelines

Use this checklist before committing an implementation slice.

- Does the slice improve the local capsule build loop?
- Does it preserve deterministic output?
- Does every promoted object have source or review backing?
- Does it block rejected, expired, unreviewed, or rights-blocked objects from
  PRAXIS context packs by default?
- Does it expose caveats instead of silently smoothing them away?
- Does it keep advanced adapters optional?
- Does it include tests that would fail if the behavior regressed?
- Does the documentation say what is real now versus planned later?

## Research And Dependency Guidelines

Research is useful only when it changes an implementation decision.

- Store source links and conclusions in `docs/RESEARCH_LEDGER.md`.
- Store open questions in `docs/RESEARCH_BACKLOG.md`.
- Promote a technology only through an ADR when it changes canonical storage,
  deployment class, security posture, or PRAXIS contract.
- Prefer official docs, specs, papers, and primary repositories over blog
  summaries.
- Re-check cloud, MCP, graph, memory, and model-runtime assumptions before
  implementing an integration because those surfaces change quickly.

## Gap Ledger Protocol

When a gap is found, record it in one of three ways:

- P0: blocks the next executable proof; fix before continuing.
- P1: does not block the next proof, but must be addressed before production
  staging.
- P2: quality, polish, or future scale improvement; keep visible, but do not
  interrupt the local loop.

Every P0 or P1 gap needs:

- owner or next lane;
- file or command evidence;
- expected behavior;
- acceptance test;
- decision record if the fix changes architecture.

## Immediate Next Improvement

The next coding session should start with Series 1 and Series 2 together:

```text
typed source pack -> proposal records -> review routing -> deterministic compiler
```

That is the shortest path from a strong repository scaffold to a real engine.
