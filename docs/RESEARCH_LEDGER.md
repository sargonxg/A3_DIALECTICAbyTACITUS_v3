# Research Ledger

Date: 2026-06-07

Status: durable source memory for DIALECTICA design decisions.

## Purpose

This file stores the research conclusions that should survive across future
agent sessions. It is intentionally practical: each source becomes a decision,
constraint, adapter posture, or refresh trigger.

Use this together with:

- `docs/TECH_BENCHMARK.md`;
- `docs/GRAPH_ONTOLOGY_RESEARCH_NOTES.md`;
- `docs/RESEARCH_BACKLOG.md`;
- `docs/decisions/`.

## Current Design Conclusion

DIALECTICA should build PRAXIS Capsules as canonical signed bundles backed by
Cloud SQL PostgreSQL. The embedded graph, semantic layer, reasoning devices,
language profile, review ledger, rights policy, and agent guidance belong
inside the capsule contract. Graph engines, vector stores, MCP servers, temporal
memory systems, and marketplace automation are adapters until an ADR, eval
evidence, and runbook promote them.

```text
official sources + papers
  -> research conclusion
  -> source-of-truth doc
  -> schema or fixture
  -> validation gate
  -> future refresh trigger
```

## Source Ledger

| Source | Checked | Conclusion | DIALECTICA decision | Refresh trigger |
| --- | --- | --- | --- | --- |
| Model Context Protocol resources, <https://modelcontextprotocol.io/specification/2025-11-25/server/resources> | 2026-06-07 | MCP resources expose URI-addressed context from servers to clients. | Later expose read-only capsule manifests, graph previews, receipts, and context packs through an MCP adapter. Do not make MCP the first PRAXIS integration path. | Recheck before implementing MCP, especially auth and prompt-injection guidance. |
| Model Context Protocol repository, <https://github.com/modelcontextprotocol/modelcontextprotocol> | 2026-06-07 | The official repo contains the MCP specification, schema, and docs. | Track spec versions explicitly in adapter metadata. | Recheck current release before coding MCP server support. |
| Microsoft GraphRAG docs, <https://microsoft.github.io/graphrag/> | 2026-06-07 | GraphRAG indexing extracts entities, relationships, claims, communities, summaries, and embeddings from unstructured text. | Use graph/community ideas for corpus-scale future workflows, but begin with small deterministic capsule graph slices. | Recheck before large-source indexing or community-summary features. |
| Graphiti overview, <https://help.getzep.com/graphiti/getting-started/overview> | 2026-06-07 | Graphiti emphasizes temporal context graphs, provenance, incremental episodes, custom entities, hybrid search, and changing relationships. | Encode temporal validity, provenance, and custom entity classes in the capsule graph. Keep Graphiti optional. | Recheck before a temporal graph adapter. |
| Zep paper, <https://arxiv.org/abs/2501.13956> | 2026-06-07 | The paper frames temporal knowledge graphs as agent memory for dynamic conversations and business data. | Treat ongoing interactions as candidate capsule evidence with review state, not uncontrolled memory promotion. | Recheck if adopting a memory benchmark. |
| Temporal KGC survey, <https://arxiv.org/abs/2201.08236> | 2026-06-07 | Static knowledge graph assumptions fail when facts change over time. | Capsule graph edges and claims must carry temporal scope and supersession status. | Recheck before adding temporal reasoning algorithms. |
| Temporal KG representation survey, <https://arxiv.org/abs/2403.04782> | 2026-06-07 | Temporal KG methods model the dynamic evolution of entities and relations. | Add valid time and ingestion/provenance time to graph and claim records. | Recheck before ML-based temporal embedding work. |
| RAG paper, <https://arxiv.org/abs/2005.11401> | 2026-06-07 | Retrieval improves knowledge-intensive generation, but provenance and updating world knowledge remain central problems. | PRAXIS Capsules must preserve source receipts and freshness, not only retrieval snippets. | Recheck if building retrieval eval baselines. |
| LadybugDB homepage, <https://ladybugdb.com/> | 2026-06-07 | LadybugDB is positioned as an embedded columnar graph database with Cypher and Rust/Python/Node access. | Keep `ladybug_projection_v1` as an optional local graph analysis adapter. | Recheck license, crate maturity, and performance before dependency adoption. |
| LadybugDB get started, <https://docs.ladybugdb.com/get-started/> | 2026-06-07 | Ladybug can run embedded, including in-memory mode for temporary graph analysis. | Useful for local graph previews and algorithm experiments, not canonical capsule state. | Recheck Rust API before implementing projection. |
| JSON-LD 1.1, <https://www.w3.org/TR/json-ld11/> | 2026-06-07 | JSON-LD is JSON-compatible linked data and can serialize RDF-style graphs. | Export `graph_semantics.jsonld` while keeping JSON-first Rust validation. | Recheck before linked-data export. |
| PROV-O, <https://www.w3.org/TR/prov-o/> | 2026-06-07 | PROV-O models provenance across systems and contexts. | Source, extraction, review, compile, and export events should map cleanly to provenance records. | Recheck before provenance export or external audit features. |
| SKOS, <https://www.w3.org/TR/skos-reference/> | 2026-06-07 | SKOS supports concept schemes, labels, semantic relations, mappings, and documentation properties. | Use SKOS-shaped ontology slices for terms, synonyms, broader/narrower links, and cross-frame mappings. | Recheck before ontology import/export. |
| SHACL, <https://www.w3.org/TR/shacl/> | 2026-06-07 | SHACL describes and validates RDF graphs. | Use SHACL as a design anchor for graph constraints, while implementing deterministic JSON validation first. | Recheck before RDF validation adapter. |
| ODRL, <https://www.w3.org/TR/odrl-model/> | 2026-06-07 | ODRL models permissions, prohibitions, duties, constraints, and rights policies. | Rights and marketplace rules should map to ODRL-like permission/prohibition/duty structures. | Recheck before marketplace rights enforcement. |
| OWL 2 overview, <https://www.w3.org/TR/owl2-overview/> | 2026-06-07 | OWL treats ontologies as formalized vocabularies for specific domains and communities, with richer semantics and profiles available when needed. | Keep DIALECTICA ontology blueprints local and capsule-specific first; add OWL export or reasoning only as an adapter after JSON contracts and review gates work. | Recheck before formal ontology inference or OWL export. |
| RDF 1.2 concepts, <https://www.w3.org/TR/rdf12-concepts/> | 2026-06-07 | RDF provides an abstract graph data model for linked data. | Preserve a path from embedded capsule graphs to RDF/JSON-LD exports, but keep signed JSON bundles canonical. | Recheck before RDF-native storage or SPARQL features. |
| OASIS LegalDocML / Akoma Ntoso, <https://www.oasis-open.org/committees/tc_home.php?wg_abbrev=legaldocml> | 2026-06-07 | LegalDocML/Akoma Ntoso covers legal document XML, metadata, URI-based citations, and parliamentary/court document structures. | Add `legal_document_profile` for laws, rules, judgments, bills, and public proceedings. | Recheck before legal-source ingestion. |
| Cloud Run overview, <https://docs.cloud.google.com/run/docs/overview/what-is-cloud-run> | 2026-06-07 | Cloud Run is the first deployment target for managed container services and jobs. | Use Cloud Run for API, task handler, and jobs before considering GKE. | Recheck limits before production deployment. |
| Cloud SQL from Cloud Run, <https://docs.cloud.google.com/sql/docs/postgres/connect-run> | 2026-06-07 | Cloud Run can connect to Cloud SQL for PostgreSQL with service account and region-aware configuration. | Use Cloud SQL PostgreSQL as the operational DIALECTICA store. | Recheck connection guidance before staging. |
| Cloud Tasks HTTP targets, <https://docs.cloud.google.com/tasks/docs/creating-http-target-tasks> | 2026-06-07 | Cloud Tasks can dispatch reliable HTTP target tasks and use authentication tokens. | Use Cloud Tasks for ingestion, compile, review, export, eval, and retryable work. | Recheck timeout/auth limits before worker implementation. |
| Firestore docs, <https://firebase.google.com/docs/firestore> | 2026-06-07 | Firestore stores documents in collections and supports realtime app synchronization. | Keep Firestore as the PRAXIS user-facing visibility mirror and cockpit state store. Do not make it DIALECTICA canonical build state. | Recheck PRAXIS repository contracts before adapter work. |
| OpenAI Agents SDK tracing, <https://openai.github.io/openai-agents-python/tracing/> | 2026-06-07 | Agent tracing records runs, model calls, tool calls, guardrails, handoffs, and custom events. | Capsule compilation and PRAXIS context-pack use should emit traceable receipts when integrated with agent runtimes. | Recheck before OpenAI agent orchestration code. |
| OpenAI Agents SDK guardrails, <https://openai.github.io/openai-agents-js/guides/guardrails/> | 2026-06-07 | Guardrails can check user input, tool invocations, and final output. | Model-powered extraction and capsule use should have tool-level guardrails and human review gates. | Recheck before tool-calling workflows. |

## Adopted Product Rules

- A capsule is the product contract, not a prompt, cache, or chat transcript.
- The embedded graph is mandatory because PRAXIS needs source, time,
  relationship, reasoning, language, review, and rights traversal without a
  required graph database.
- Actor/claim/time graphs are one important ontology profile, not the universal
  model for every capsule. Each capsule type should generate its own ontology
  blueprint and semantic layers before graph extraction is promoted.
- The semantic layer should be JSON-first and standards-shaped, not standards
  theater.
- Human review is data. Every review decision needs actor, time, scope, caveat,
  expiry, and affected objects.
- Expert reasoning devices are reusable intellectual tools. They must include
  inputs, steps, failure modes, source requirements, and PRAXIS guidance.
- Human-gated language is a capsule layer. Approved terms, deprecated terms,
  blocked phrases, caveats, audience register, and translation notes must travel
  with the capsule.
- PRAXIS gets context packs and receipts. DIALECTICA keeps build, review,
  graph, export, and signature truth.

## Refresh Before Coding

Refresh these sources immediately before implementation:

1. LadybugDB crate/API docs before adding a dependency.
2. MCP specification and security guidance before exposing capsule resources.
3. Cloud Run, Cloud SQL, and Cloud Tasks limits before staging deployment.
4. Firestore data model in PRAXIS before writing the mirror adapter.
5. OpenAI Agents SDK docs before agent orchestration integration.
6. Graphiti/Zep docs before temporal graph adapter work.

## Future Research Questions

- Which policy-domain ontology should anchor the first golden capsule?
- Which eval most clearly proves PRAXIS augmented generation beats raw
  prompting?
- Which graph preview helps analysts catch wrong claims fastest?
- What minimum expert-review metadata is required for a capsule marketplace?
- Which capsule rights model is strong enough for enterprise and public-sector
  teams without slowing the foundation build?
