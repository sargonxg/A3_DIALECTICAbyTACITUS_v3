# Post-Build Audit - 2026-06-08

Status: audit after commit `7fa7ae81`, local verification, GitHub Actions
success, Graphify refresh, and live dependency/tooling research.

## Executive Result

DIALECTICA now has a real local capsule-build loop for deterministic fixtures.
It is no longer only documentation or contract scaffolding.

It is not yet the production capsule-building service. The next build must move
state out of fixtures and into durable build/session records with PostgreSQL
migrations, repository interfaces, and store-backed API jobs.

## What Is Working

| Surface | Status | Evidence |
| --- | --- | --- |
| Canonical v3 package validation | Working | `validate` and contract tests pass |
| Four macro-capsule type rule | Working | tests reject extra top-level types |
| Embedded Ladybug projection requirement | Working | `ladybug-check` validates fixture projection files |
| Source-pack contract | Working in fixture mode | golden source pack validates |
| Extraction proposal contract | Working in fixture mode | proposals validate with model receipts and source spans |
| Review-trigger routing | Working in fixture mode | Plus/promoted proposals require blocking gates |
| Reviewer decisions | Working in fixture mode | nine required decisions validate |
| Promotion normalization | Working in fixture mode | promoted records become compiler-ready |
| Deterministic compiler | Working in fixture mode | builds canonical v3 package from fixture inputs |
| `.capsule` archive writer | Working | writes archive with `mimetype` first and rejects self-inclusion |
| PRAXIS context-pack exporter | Working in fixture mode | exports JSON context with caveats, claims, graph focus, language rules, and receipts |
| Axum API | Working in fixture mode | serves health, version, manifest, graph preview, context pack, and read receipts |
| CI | Working | GitHub Actions run `27172109186` passed for commit `7fa7ae81` |
| Graphify repo map | Refreshed | `382` nodes, `829` edges, `22` communities |

## What Is Missing

| Gap | Severity | Why it matters | Next implementation shape |
| --- | --- | --- | --- |
| PostgreSQL migrations | Blocker | fixture files cannot support real builds, review queues, retries, or PRAXIS consumption | add SQLx migrations for capsule_builds, sources, proposals, review_decisions, compiled_artifacts |
| Store repository interfaces | Blocker | API cannot start real build sessions without persistence | implement `dialectica-store` repositories and transaction boundaries |
| Store-backed API jobs | Blocker | PRAXIS needs start/inspect/approve/fetch flows | add build-session routes and compile job state |
| Task handler | Blocker | long-running ingestion/compile work should not happen in request handlers | implement Cloud Tasks-compatible local HTTP handler |
| Live source ingestion | High | users need documents, PDFs, notes, and conversations converted to source packs | add local file ingestion first, cloud artifact storage later |
| Live model providers | High | extraction is fixture-only | add provider traits, fixture provider, and one live provider behind env-gated tests |
| Human review workflow | High | review decisions are file fixtures, not user actions | add review queue records and reviewer decision API |
| Checksum/signature hardening | High | current digest is enough for local proof, not production integrity | define canonical digest scope, signature envelope, and compatibility tests |
| Generated fixture comparison | Medium | compiler can drift from canonical expected output without byte-level detection | add stable generated-output snapshot test |
| Eval harness | Medium | no proof yet that capsules improve PRAXIS output | add source-fidelity, temporal-warning, and raw-LLM-vs-context-pack evals |
| PRAXIS adapter | Medium | PRAXIS cannot yet consume the service directly | keep API first; add MCP later only as read-only adapter |
| Observability | Medium | no tracing/logging spans yet for build stages | add `tracing`, request ids, build ids, and receipt ids |
| Auth and tenancy | Medium | fixture API is local-only and unauthenticated | add service auth after store-backed API shape is stable |

## Graphify Findings

Refreshed with:

```powershell
graphify update .
```

Report:

- `382` nodes;
- `829` edges;
- `22` communities;
- god nodes include `main()`, `validate_proposal_set()`, `write_package()`,
  `read_json()`, `load_source_pack()`, `validate_reviewer_decision_set()`, and
  `promote_records()`;
- weakly connected nodes include validation/schema structs that need stronger
  docs or test coverage as validators expand.

Conclusion: the architecture is centered around the right build loop:
proposal validation -> review decision validation -> promotion -> package
write -> API/context use. The graph also confirms the next risk: store, task
handler, and eval crates are thin compared with compiler/extractor/capsule.

## Research Update

Fresh source checks support the current posture:

- Ladybug is still a good required embedded projection for promoted capsules:
  the official repository describes an embedded, serverless graph database with
  Cypher and Rust installation through `cargo add lbug`; the current repo
  release shown during research is `v0.17.1`.
- The DIALECTICA repo already pins `lbug = "0.17.1"` behind the
  `dialectica-graph/ladybug` feature, so the dependency decision is coherent
  with the live source check.
- Graphify is useful as a repo map and architecture memory, but its current
  update path is AST/code oriented. Use it to orient and detect stale graph
  context, then verify conclusions in source.
- Serena is available locally and configured through Codex MCP. Use it for
  symbol-level navigation/refactoring when exposed in-session; otherwise fall
  back to `rg`, Graphify, and exact file reads.

Sources:

- <https://github.com/LadybugDB/ladybug>
- <https://docs.ladybugdb.com/installation/>
- <https://docs.ladybugdb.com/get-started/>
- <https://emelia.io/hub/knowledge-graph-graphify-guide>
- <https://oraios.github.io/serena/02-usage/030_clients.html>
- <https://oraios.github.io/serena/02-usage/020_running.html>

## Recommended Next Build

Build durable state before adding live LLM calls.

Sequence:

1. Add SQLx and first PostgreSQL migrations.
2. Implement store repositories and transaction policies.
3. Add API routes for build sessions:
   `POST /v1/builds`, `GET /v1/builds/{id}`,
   `POST /v1/builds/{id}/reviews`, `POST /v1/builds/{id}/compile`.
4. Add local task-handler route that can compile one stored fixture build.
5. Add generated fixture comparison and checksum/signature envelope tests.
6. Add the first eval checks.
7. Add live ingestion and model-provider adapters only after the durable loop
   works with fixture providers.

## Recommended Coding Prompt

```text
Implement the store-backed DIALECTICA build-session slice.

Read docs/SOURCE_OF_TRUTH.md, docs/CODING_LEDGER.md,
docs/NEXT_CODE_BUILD_PLAN.md, docs/CODE_QUALITY_TOOLING.md, and
docs/POST_BUILD_AUDIT_2026_06_08.md first.

Use the existing fixture build loop as the behavioral contract. Add SQLx
migrations and repository interfaces for build requests, source packs,
proposal sets, reviewer decisions, promoted records, compiled artifacts, and
context-pack receipts. Then add fixture-backed API routes that persist and
inspect build state without cloud credentials.

Do not add live model providers, cloud deployment, PRAXIS frontend code, MCP
server support, or a new graph/vector database in this slice.

Verify with cargo fmt/check/clippy/test, the full fixture CLI gate, Python
tests, graphify update ., and git diff --check. Update ledgers before commit.
```

## Current Readiness Score

| Dimension | Score | Reason |
| --- | --- | --- |
| Concept coherence | 92/100 | four capsule types, proposal boundary, embedded graph, and PRAXIS contract are coherent |
| Local executable proof | 82/100 | fixture loop is real and CI-gated |
| Production backend readiness | 35/100 | store, jobs, auth, deployment, and live ingestion are missing |
| PRAXIS integration readiness | 45/100 | context pack/API shape exists, but no deployed or PRAXIS-owned client path |
| Agent build readiness | 86/100 | docs, graphify, command gates, and audit ledgers are now usable by future agents |
