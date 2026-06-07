# Graph, Ontology, And Capsule Research Notes

Date: 2026-06-07

Status: research-backed design update for embedded graph, ontology, adapter,
and deployment decisions.

## Research Sources Checked

Primary source anchors:

- LadybugDB homepage: <https://ladybugdb.com/>
- LadybugDB GitHub: <https://github.com/LadybugDB/ladybug>
- LadybugDB Rust tutorial: <https://docs.ladybugdb.com/tutorials/rust/>
- LadybugDB graph algorithms: <https://docs.ladybugdb.com/extensions/algo/>
- RDF 1.2 concepts: <https://w3c.github.io/rdf-concepts/spec/>
- JSON-LD 1.1: <https://www.w3.org/TR/json-ld/>
- PROV-O: <https://www.w3.org/TR/prov-o/>
- SKOS: <https://www.w3.org/TR/skos-reference/>
- SHACL 1.2 Core: <https://www.w3.org/TR/shacl12-core/>
- ODRL Information Model: <https://w3c.github.io/poe/model/>
- OWL: <https://www.w3.org/OWL/>
- OASIS LegalDocML / Akoma Ntoso: <https://www.oasis-open.org/committees/tc_home.php?wg_abbrev=legaldocml>
- Microsoft GraphRAG docs: <https://microsoft.github.io/graphrag/>
- Graphiti docs: <https://help.getzep.com/graphiti/getting-started/welcome>
- Cloud Run container contract: <https://docs.cloud.google.com/run/docs/container-contract>
- Cloud Tasks HTTP target docs: <https://docs.cloud.google.com/tasks/docs/creating-http-target-tasks>

## Main Findings

### 1. LadybugDB Is A Strong Adapter Candidate, Not The First Source Of Truth

LadybugDB positions itself as an embedded graph database with Cypher querying,
Rust/Python/Node access, columnar storage, full-text and vector retrieval
features, and support for in-memory use. Its Rust tutorial starts from an
explicit schema. Its graph algorithm extension runs algorithms over projected
graphs rather than operating directly on all database tables.

Design consequence:

- add `ladybug_projection_v1` as an optional graph adapter profile;
- use LadybugDB later for local capsule graph exploration, graph algorithms,
  influence/community analysis, and fast projected graph queries;
- keep PostgreSQL and signed bundle files canonical until evals prove the
  adapter should become operationally required.

### 2. Standards Should Shape The Bundle Without Forcing An RDF Runtime

The W3C stack is directly useful:

- JSON-LD serializes linked data using JSON.
- RDF gives stable identifiers and datatypes.
- PROV-O models provenance across systems and contexts.
- SKOS models controlled vocabularies, thesauri, classifications, synonyms, and
  broader/narrower concept relationships.
- SHACL describes graph constraints and can inspire deterministic validation.
- ODRL expresses permissions, prohibitions, and duties.
- OWL can represent richer relations and allow consistency checks or inferred
  knowledge, but it should remain optional until there is a clear policy need.

Design consequence:

- make `ontology_slice.json`, `graph_semantics.jsonld`, and
  `graph_constraints.json` first-class bundle layers;
- add source-backed definitions, mappings, frame memberships, deprecations, and
  validation constraints;
- keep the Rust validator JSON-first, with RDF/OWL/SHACL adapters later.

### 3. Legal And Policy Sources Need Document Semantics

OASIS LegalDocML/Akoma Ntoso is relevant because policy capsules often cite
laws, bills, judgments, parliamentary documents, regulations, or administrative
acts. It emphasizes legal-document interchange, metadata, long-term access,
citations, and cross-references.

Design consequence:

- add `legal_document_profile` fields to source records when a capsule includes
  law, regulation, judgment, or official proceedings;
- preserve citation structure, issuing authority, jurisdiction, document date,
  and amendment/supersession relationships;
- model legal authority separately from political feasibility.

### 4. GraphRAG And Temporal Graphs Confirm The Capsule Direction

Microsoft GraphRAG emphasizes knowledge graph extraction, community hierarchy,
community summaries, and graph-based retrieval over unstructured text. Graphiti
emphasizes temporal knowledge graphs for AI agents, incremental updates, and
custom entities.

Design consequence:

- keep capsule graphs small, typed, and reviewable before trying large
  automatic graph construction;
- include communities and `why_surfaced` explanations in graph previews;
- treat temporal validity and provenance as required edge metadata.

### 5. Google Cloud Deployment Direction Remains Correct

Cloud Run services must listen on the injected port and are suited for HTTP
services. Cloud Tasks can call HTTP targets and supports service-account based
authentication with OIDC tokens for handlers running on Cloud Run. This matches
the current DIALECTICA split: API service, task-handler service, jobs, Cloud SQL,
Cloud Storage, Cloud Tasks, and Secret Manager.

Design consequence:

- keep Cloud Run first;
- expose `/health` and `/version` locally;
- add separate liveness/readiness probes only when staging or production needs
  distinct Cloud Run health checks;
- keep task handlers idempotent and authenticated by service account.

## Updated Capsule Layer Decision

Add `agent_guidance.json` as an explicit bundle layer. Reasoning playbooks are
for method transfer; output contracts are for artifact shape; agent guidance is
for model execution policy.

Required fields:

- `allowed_workflows`;
- `tool_policy`;
- `citation_policy`;
- `graph_use_policy`;
- `reasoning_sequence`;
- `context_budget_policy`;
- `stop_conditions`;
- `handoff_policy`;
- `audit_receipts_required`.

## Graph Adapter Profiles

| Adapter profile | Status | Role |
| --- | --- | --- |
| `embedded_graph_v1` | required | canonical compact graph inside the bundle |
| `postgres_projection_v1` | required for runtime | relational graph tables and JSONB extension fields |
| `jsonld_projection_v1` | required for export compatibility | semantic linked-data view |
| `ladybug_projection_v1` | optional | local/analytical graph acceleration and algorithms |
| `graphiti_projection_v1` | optional | temporal graph research and future memory adapter |
| `graphrag_projection_v1` | optional | corpus-level community summaries and large-source synthesis |

No adapter may become authoritative without an ADR, eval evidence, and an
operations runbook.

## Ontology Profile

Every capsule with policy content should include:

- `ontology_id`, `version`, `namespace`, `language`;
- `terms` with labels, definitions, source spans, synonyms, and broader/narrower
  relationships;
- `mappings` from source terms to capsule concepts;
- `frame_memberships` for analytical frames;
- `legal_document_profile` when a source is a law, rule, judgment, bill, or
  public proceeding;
- `deprecations` for terms or mappings that should no longer guide PRAXIS.

## Coding Implications

Lane A should produce Rust structs for:

- `AgentGuidance`;
- `ToolPolicy`;
- `CitationPolicy`;
- `GraphUsePolicy`;
- `OntologyProfile`;
- `LegalDocumentProfile`;
- `GraphAdapterProfile`;
- `SemanticExportProfile`.

Lane B should validate the four small example capsules before the larger golden
policy fixture is introduced.
