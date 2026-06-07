# DIALECTICA v3 - Build Instructions for Codex

Revision: 5
Date: 2026-06-07
Repo target: `github.com/sargonxg/A3_DIALECTICAbyTACITUS_v3`
Runtime target: Google Cloud project `praxisbytacitus`, region `us-central1`
Consumer: PRAXIS at `praxis.tacitus.me`

You are a senior engine architect building TACITUS's capsule intelligence layer for PRAXIS Augmented Generation.

> Implementation authority note: this file preserves the broad product and
> architecture brief. For current coding work, the Rust-first implementation
> path in `docs/SOURCE_OF_TRUTH.md`, `docs/CODING_LEDGER.md`,
> `docs/ENGINEERING_BASELINE.md`, `docs/LANE_A_ACCEPTANCE.md`, and
> `docs/API_SLICE_1.md` supersedes older Python/FastAPI-oriented scaffolding
> details in this imported brief.

Read this whole file before planning. Execute one phase at a time. Do not skip acceptance criteria. Do not silently substitute dependencies, model names, storage choices, or product surfaces. When a dependency or cloud feature is volatile, verify it against the source anchors in section 20 before implementing.

Normative companion pages added in Revision 5:

- `docs/CAPSULE_TYPES_AND_MARKETPLACE.md` for capsule categories,
  compatibility, marketplace listings, and expert-pick trust levels.
- `docs/EMBEDDED_GRAPH_AND_SEMANTIC_LAYER.md` for embedded graph files, node
  and edge profiles, semantic export anchors, and PRAXIS visualization rules.
- `docs/EXPERT_REVIEW_AND_MARKETPLACE.md` for human gates, review objects,
  reviewer roles, promotion gates, forking, and marketplace safety.
- `docs/CAPSULE_BUILD_EXAMPLES.md` for concrete stakeholder-analysis,
  decision-clock, thinking-device, and output-capsule examples.

## 0. Executive Decision

DIALECTICA v3 builds the engine that turns policy documents, user context, analyst interactions, live-world evidence, expert reasoning, and review decisions into portable PRAXIS Capsules.

The objective is not a generic chatbot memory layer. The objective is to give PRAXIS agents context objects that understand:

- who the user is writing as and for;
- what situation is being analyzed;
- what evidence supports each claim;
- what is temporally true, stale, superseded, disputed, or uncertain;
- what ontology and graph structure makes the situation legible;
- what expert reasoning devices should be applied;
- what output should be produced;
- what must be cited, hedged, refused, or escalated to a human.

The product thesis:

> PRAXIS Augmented Generation with PRAXIS Capsules by TACITUS.

The technical thesis:

> A capsule is a signed, portable, self-describing knowledge-work object: a model of a situation, the evidence behind it, the reasoning tools for using it, and the rules for human and AI use.

The build thesis:

> Contract first, engine-light first, source-cited always. Use graph and semantic engines as adapters and caches, never as the only copy of truth.

## 1. Product Boundary and Naming

TACITUS is the company.
PRAXIS is the user-facing workbench and agentic workflow layer.
DIALECTICA is the internal capsule intelligence engine.
AGON is the conflict, friction, actor, and adversarial-perception subsystem.
KAIROS is the episodic and temporal-perception subsystem.

Public product language:

- Use `PRAXIS`.
- Use `PRAXIS Capsules`.
- Use `Capsules`, `Capsule AI`, or `Capsule Library` in simple user-facing UI.
- Use `DIALECTICA Engine` only in internal docs, architecture diagrams, and integration docs.

Do not ship public copy that presents `DIALECTICA` as a standalone buyer-facing product. There is a real market collision around the name. Do not ship `Context Capsule` as the user-facing brand. Use `PRAXIS Capsules` instead.

Product shape:

- PRAXIS remains the visible cockpit.
- DIALECTICA compiles capsules behind PRAXIS.
- PRAXIS users select and compose capsules.
- PRAXIS agents consume the composed context and operation handles.
- Review, provenance, and trust surfaces should appear inside the existing PRAXIS capsule/workbench surfaces, not as a new primary app.

## 2. Non-Negotiable Architecture Rules

1. The signed capsule bundle is canonical.
   The bundle contains canonical JSON/JSONL records, deterministic semantic projections, compiled agent views, and optional cache files.

2. Engines are adapters, not truth.
   PostgreSQL, Ladybug, Oxigraph, Graphiti, Spanner Graph, and any future graph engine may index, project, query, or accelerate a capsule. None may become the only copy of a capsule.

3. Engine-less mode is mandatory.
   A capsule must remain usable by an LLM or local Python code from the bundle alone. No graph server may be required to inspect, cite, compose, critique, improve, or apply a device to the foundation build capsule.

4. PRAXIS user state is still PRAXIS-owned.
   Firestore mirrors capsule metadata and selected state for PRAXIS UX. DIALECTICA owns compiled bundle artifacts and the build/proposal/review pipeline. Durable runs and operation receipts belong in SQL tables.

5. Every factual claim needs provenance.
   No Situation claim ships without `claim_id`, source span, source hash, temporal metadata, origin, confidence, and trust layer.

6. Machine proposals are never silent edits.
   Agent-generated links, claims, gaps, critiques, and enrichments enter a proposal queue as `machine_proposed` and `T3`. Human or corroboration gates promote them.

7. Trust controls generation.
   The composed context must explicitly tell agents how to assert, attribute, hedge, flag, or refuse based on trust layer and disputed status.

8. Build the judgment layer.
   Do not spend the foundation build rebuilding generic storage, retrieval, entity resolution, or graph database infrastructure. Use adapters and narrow smoke tests.

9. Start boring where boring is correct.
   For foundation build operational storage, prefer Cloud SQL PostgreSQL with JSONB, relational constraints, full text search, and pgvector. Add graph engines after bundle, engine-less operations, and evaluation are working.

10. A demo without eval is not proof.
    Build the benchmark as a first-class product artifact. PRAXIS-with-capsules must beat a generic LLM on citation coverage, temporal correctness, contradiction handling, assumption handling, and analyst usefulness.

## 3. Capsule Type Model

The user composes a context set from four user-selectable capsule types:

| Type | Cardinality | Answers | Holds |
|---|---:|---|---|
| USER | exactly 1 | Who am I working as, for, and with? | role, seniority, expertise, audience, voice, priorities, standing positions, intellectual style, constraints |
| SITUATION | 1 or more | What is going on? | evidence, claims, actors, events, institutions, conflicts, open questions, episodes, temporal states, graph projection |
| TOOL | 0 or more | How should I think about it? | intellectual devices, method steps, graph/retrieval needs, critique prompts, typed outputs |
| OUTPUT | exactly 1 | What should I produce? | deliverable schema, sections, citation style, length, tone, refusal rules, runtime contract |

Composition is the product:

```text
pick(1 USER)
+ pick(1..n SITUATION)
+ pick(0..n TOOL)
+ pick(1 OUTPUT)
= compose()
= one bounded, source-citing, trust-calibrated agent context block
+ operation handles
+ runtime contract
```

Do not add more user-selectable top-level capsule types for foundation build. If more structure is needed, add internal packs inside the four types.

## 4. Internal Packs

Each capsule is made of typed packs. Packs are the real implementation units.

Common packs:

- `IdentityPack`: capsule id, type, category, owner, version, status, labels, created/updated timestamps.
- `SourcePack`: source documents, rights, retrieval time, document hash, extraction hash.
- `EvidencePack`: chunks, spans, OCR/layout metadata, citations, source-to-claim links.
- `ClaimPack`: atomic claims with trust, temporality, source span, confidence, dispute state.
- `GraphPack`: nodes and edges for actors, claims, interests, constraints, events, institutions, episodes, and relationships.
- `SemanticPack`: JSON-LD projection, ontology ids, SHACL validation result, term mappings.
- `TemporalPack`: observed time, valid time, episode boundaries, state changes, supersession.
- `ReviewPack`: machine checks, human signoff, dissent, unresolved questions, promotion history.
- `RuntimePack`: instructions for citation, hedging, refusal, retrieval, freshness, composition, and operation calling.
- `OperationsPack`: self-description for `seek`, `understand`, `connect`, `critique`, `improve`, `apply_device`, `diff`, and `compose`.

Type-specific packs:

- USER adds `PersonaPack`, `AudiencePack`, `VoicePack`, `ExpertisePack`, and `PreferencePack`.
- SITUATION adds `SituationFramePack`, `ActorPack`, `FrictionPack`, `EpisodePack`, `OpenQuestionPack`, and optional `IndicatorPack`.
- TOOL adds `DevicePack`, `ProcedurePack`, `GraphQueryPack`, `OutputSchemaPack`, and `RedTeamPack`.
- OUTPUT adds `DeliverablePack`, `TemplatePack`, `QualityRubricPack`, and `FinalAnswerContract`.

The compiled bundle may include all packs, but the agent context must be bounded and selective. Large packs are retrieved through operations.

## 5. Source of Truth and Storage

### 5.1 Canonical Artifact

The canonical artifact is a signed capsule bundle:

```text
<capsule_id>.capsule/
├── manifest.json
├── records/
│   ├── envelope.json
│   ├── sources.jsonl
│   ├── chunks.jsonl
│   ├── claims.jsonl
│   ├── entities.jsonl
│   ├── edges.jsonl
│   ├── episodes.jsonl
│   ├── devices.json
│   ├── review.json
│   └── runtime.json
├── projections/
│   ├── graph.jsonld
│   ├── graph.nq
│   ├── agent_context.md
│   ├── operations.md
│   └── capsule_summary.json
├── eval/
│   ├── expected_behaviors.json
│   └── eval_receipts.jsonl
└── cache/
    ├── ladybug/              # optional, regenerable
    ├── oxigraph/             # optional, regenerable
    └── indexes/              # optional, regenerable
```

Canonical records live under `records/`. `projections/graph.jsonld` is deterministic and authoritative as a semantic projection, but the simpler record files remain the primary build truth for foundation build. This avoids betting the whole product on RDF edge cases while still supporting semantic interoperability.

### 5.2 Operational Store

Use Cloud SQL PostgreSQL for foundation build operational state:

- capsule metadata and versions;
- source receipts and span indexes;
- claims and claim-source links;
- entities and aliases;
- graph adjacency tables;
- operation receipts;
- proposal queue;
- review gates;
- embeddings via pgvector;
- JSONB snapshots of bundle manifests and runtime contracts.

Recommended foundation build tables:

```text
capsules
capsule_versions
capsule_sources
source_chunks
claims
claim_sources
entities
entity_aliases
graph_edges
episodes
devices
compositions
operation_receipts
proposals
review_events
eval_runs
```

Use PostgreSQL constraints for owner isolation, version immutability, and referential integrity. Use JSONB for evolving metadata, but keep high-value fields relational and indexed.

### 5.3 PRAXIS Mirror

Firestore mirrors only what PRAXIS needs for fast UX:

```text
users/{uid}/situations/{situationId}/capsules/{capsuleId}
users/{uid}/capsuleSelections/{selectionId}
users/{uid}/capsuleProposals/{proposalId}
```

The mirror stores id, title, type, category, version, trust summary, freshness summary, bundle URI, active selection state, and review status. It should not become the only copy of claims, graph edges, source spans, or review events.

### 5.4 GCS

Store signed bundles in GCS:

```text
gs://praxis-dialectica-capsules/{uid}/{capsule_id}/v{version}/bundle.zip
```

Bundle writes are append-only by version. Never mutate an existing signed bundle. Create a new version.

## 6. Graph, Ontology, and Semantic Layer

### 6.1 foundation build Graph Strategy

The foundation build graph is a portable graph record set plus deterministic projections:

- `entities.jsonl`
- `edges.jsonl`
- `episodes.jsonl`
- `claims.jsonl`
- `graph.jsonld`
- optional SQL adjacency tables
- optional engine cache files

For foundation build, queries must work through:

1. pure-Python over bundle records;
2. PostgreSQL adjacency and full-text/vector indexes;
3. optional Ladybug and Oxigraph adapters after smoke tests pass.

This avoids making Graphiti, Ladybug, Oxigraph, or Spanner Graph a blocker for the first proof.

### 6.2 Ontology Cores

A Core is an ontology and extraction pack:

```text
Core = primitives + relations + extraction schema + validation rules + device bindings + templates
```

ACO is the kernel:

- Actor
- Claim
- Interest
- Constraint
- Leverage
- Commitment
- Event
- Narrative

Initial domain cores:

- `country_risk`
- `conflict`
- `mediation`
- `policy_formulation`
- `institutional_actor`
- `electoral_transition`

Core files live under:

```text
backend/dialectica/cores/
```

Each core must define:

- primitive classes;
- relation types;
- JSON Schema or Pydantic models;
- SHACL shapes for semantic projection validation;
- extraction prompt/schema;
- source-span requirements;
- device bindings;
- default output templates.

### 6.3 JSON-LD and RDF

Use JSON-LD 1.1 as the semantic projection format because it is JSON-compatible, Linked Data-capable, and supports graph/dataset expression. Use SHACL for validation. Use RDF 1.2 triple terms only as an optional advanced export path until the implementation stack proves stable.

foundation build rule:

- Claim metadata is canonical in `claims.jsonl`.
- JSON-LD projects claim ids, entities, edges, sources, and graph layers.
- Do not require RDF 1.2 triple terms for foundation build claim metadata.

Named graph projection convention:

```text
g:identity
g:sources
g:claims
g:situation
g:temporal
g:agon
g:kairos
g:reasoning
g:review
g:ontology/aco
```

### 6.4 Engine Adapters

All engines implement one interface:

```python
class CapsuleGraphBackend(Protocol):
    def load_bundle(self, bundle: CapsuleBundle) -> None: ...
    def materialize(self) -> BackendReceipt: ...
    def query(self, query: GraphQuery) -> QueryResult: ...
    def neighborhood(self, node_id: str, depth: int) -> GraphNeighborhood: ...
    def validate(self) -> ValidationReceipt: ...
    def close(self) -> None: ...
```

Backends:

- `PurePythonBackend`: mandatory, offline, uses bundle records.
- `PostgresBackend`: foundation build operational store, uses SQL, full-text search, pgvector, adjacency tables.
- `LadybugBackend`: optional embedded property graph/vector/cache after P6 smoke.
- `OxigraphBackend`: optional RDF/SPARQL/JSON-LD validation and semantic query backend after P6 smoke.
- `GraphitiBackend`: optional temporal/episodic substrate after P7, not required for foundation build.
- `SpannerGraphBackend`: enterprise production adapter, not foundation build.

Ladybug note:

- Current install docs use `pip install ladybug` and `cargo add lbug`.
- Do not use stale package names such as `real_ladybug`.
- Pin only after P0 install and smoke test succeeds on this machine or CI image.

Graphiti note:

- Current Graphiti quick start requires Python 3.10+ and a graph backend such as Neo4j or FalkorDB for the common path.
- Graphiti supports multiple providers and graph configurations, but must remain optional until a real adapter smoke test passes.

Spanner Graph note:

- Spanner Graph is powerful for production graph workloads but requires Spanner Enterprise or Enterprise Plus. Treat it as a later adapter, not the foundation build path.

## 7. Claim, Source, and Trust Model

### 7.1 Claim Record

```json
{
  "id": "clm_01J...",
  "capsule_id": "cap_01J...",
  "text": "The military council announced a suspension of parliament on 2026-02-14.",
  "primitive": "Event",
  "subject": "ent_military_council",
  "predicate": "announced",
  "object": "ent_parliament_suspension",
  "cores": ["aco", "country_risk"],
  "source_span": {
    "source_id": "src_07",
    "chunk_id": "chk_07_003",
    "start": 1840,
    "end": 1979,
    "quote_hash": "sha256:..."
  },
  "confidence": 0.74,
  "trust_layer": "T2",
  "disputed": false,
  "corroboration": {
    "score": 0.82,
    "source_classes": ["primary_document", "news_wire"],
    "supporting_sources": ["src_07", "src_12"]
  },
  "valid_from": "2026-02-14",
  "valid_to": null,
  "observed_at": "2026-02-15T10:22:00Z",
  "superseded_by": null,
  "episode_id": "ep_post_coup_transition",
  "origin": "extracted",
  "review_status": "machine_checked"
}
```

### 7.2 Trust Layers

| Layer | Meaning | Promotion path | Agent behavior |
|---|---|---|---|
| T1 | Vetted | expert signoff or strong multi-source corroboration | may assert as fact with citation |
| T2 | Corroborated | credible source plus partial corroboration and no contradiction | may use with attribution |
| T3 | Needs corroboration | single-source, weak extraction, machine inference, or user note without corroboration | hedge, flag, never assert as settled |
| DISPUTED | Sources conflict | contradiction detected or human dissent recorded | surface disagreement; never choose silently |

Promotion rules:

- `T3 -> T2`: requires corroboration from a credible independent source or human reviewer approval.
- `T2 -> T1`: requires expert signoff or strict triangulation threshold defined in `trust.py`.
- `machine_proposed` always starts at T3.
- DISPUTED is a flag, not a layer. It can apply to T1/T2/T3 claims.
- Contradictory claims remain separate records. Do not merge them into a false compromise.

### 7.3 Source Classes

Use source classes because "two sources" is not the same as two independent evidentiary bases.

Initial classes:

- `primary_document`
- `official_statement`
- `legislation_or_regulation`
- `court_or_institution_record`
- `dataset`
- `news_wire`
- `local_media`
- `expert_interview`
- `user_note`
- `machine_inference`
- `gdelt_event`
- `wikidata_entity`

Triangulation should require distinct source classes unless a human reviewer overrides.

## 8. Temporality and KAIROS

Policy analysis fails when time is flattened. DIALECTICA must track:

- observed time: when the system saw or extracted the claim;
- valid time: when the claim is true in the world;
- publication time: when the source was published;
- retrieval time: when the source was fetched;
- episode membership: which political or analytical phase the claim belongs to;
- supersession: what claim replaces or invalidates another claim;
- freshness requirement: how stale a claim can be for a given output.

Episode record:

```json
{
  "id": "ep_post_coup_transition",
  "type": "regime_phase",
  "label": "Post-coup transition",
  "t_start": "2026-02-14",
  "t_end": null,
  "boundary_event": "evt_coup_declaration",
  "fuzzy": false,
  "participants": ["ent_army", "ent_president", "ent_election_commission"],
  "member_claims": ["clm_...", "clm_..."],
  "member_events": ["evt_..."],
  "state_before": "Civilian government retains nominal executive authority.",
  "state_after": "Military council asserts executive authority and suspends parliament.",
  "causes": ["ep_election_dispute"],
  "leads_to": []
}
```

KAIROS responsibilities:

- form candidate episodes from event clusters;
- label phase boundaries;
- summarize before/after state;
- compute diffs from new event sets;
- mark stale and superseded claims;
- let analysts override episode segmentation.

foundation build diff output:

```json
{
  "added_claims": ["clm_new_01"],
  "superseded_claims": ["clm_old_03"],
  "changed_entities": ["ent_parliament"],
  "episode_changes": [
    {
      "episode_id": "ep_post_coup_transition",
      "change": "state_after_updated",
      "reason": "new decree source confirms parliament remains suspended"
    }
  ],
  "trust_changes": [
    {
      "claim_id": "clm_...",
      "from": "T3",
      "to": "T2",
      "evidence": ["src_12"]
    }
  ]
}
```

## 9. Conflict and Political Perception: AGON

AGON makes the situation legible as a political conflict or institutional friction system.

Initial AGON outputs:

- actor registry;
- actor aliases and Wikidata candidate QIDs;
- interests;
- constraints;
- leverage;
- commitments;
- narratives;
- antagonism edges;
- office-friction edges;
- strongest counter-claim;
- weakest evidentiary link;
- missing actor/source analysis;
- flip conditions.

Example edge:

```json
{
  "id": "edge_01J...",
  "type": "CONTENDS_WITH",
  "source": "ent_military_council",
  "target": "ent_election_commission",
  "over": "certification of election results",
  "means": ["legal_decree", "security_pressure"],
  "intensity": 0.78,
  "claim_ids": ["clm_01", "clm_02"],
  "trust_layer": "T2",
  "valid_from": "2026-02-14"
}
```

AGON extraction must not present inferred antagonism as fact. An inferred edge is a claim-backed analytical edge with its own confidence and trust layer.

## 10. User Capsules

USER capsules encode the role and constraints under which PRAXIS should reason and write.

foundation build USER category:

- `risk_analyst`

Fields:

- `role`
- `organization_type`
- `seniority`
- `expertise_level`
- `audience`
- `preferred_voice`
- `standing_positions`
- `red_lines`
- `time_horizon`
- `risk_tolerance`
- `citation_expectation`
- `known_blind_spots`
- `jurisdiction_or_region_scope`

USER capsules must be inspectable and editable. Do not silently infer sensitive identity, political preference, or protected-class attributes. If the system learns user style from interaction history, store it as a proposal until the user accepts it.

## 11. Tool Capsules: Intellectual Devices

TOOL capsules encode expert reasoning procedures. They are not just prompts. They are executable analytical devices with input requirements, method steps, graph/retrieval needs, output schemas, and critique moves.

foundation build devices:

- `stakeholder_analysis`
- `scenario_analysis`
- `assumption_check`
- `pre_mortem`

Later devices:

- `analysis_of_competing_hypotheses`
- `force_field_analysis`
- `galtung_conflict_triangle`
- `fisher_ury_interests`
- `zopa_batna`
- `indicators_and_warnings`
- `options_and_tradeoffs`
- `decision_tree`
- `risk_register`

Device record:

```json
{
  "id": "stakeholder_analysis",
  "title": "Stakeholder Analysis",
  "purpose": "Map actors by influence, interest, constraints, and leverage.",
  "required_primitives": ["Actor", "Interest", "Leverage", "Constraint"],
  "retrieval_needs": [
    {"kind": "claims_by_primitive", "primitive": "Actor"},
    {"kind": "neighborhood", "edge_types": ["HOLDS_INTEREST", "HAS_LEVERAGE", "HAS_CONSTRAINT"]}
  ],
  "procedure": [
    "Identify all actors with source-backed claims.",
    "Score influence and interest salience with cited rationale.",
    "Cluster actors by coalition or likely alignment.",
    "Flag swing actors and missing high-impact actors.",
    "Return a matrix and critique."
  ],
  "output_schema": "StakeholderMatrix",
  "critique_prompts": [
    "Which high-influence actor is missing from the source base?",
    "Which influence score relies on T3 evidence?"
  ],
  "agent_rules": {
    "cite_every_actor": true,
    "hedge_t3_scores": true,
    "surface_disputed_inputs": true
  }
}
```

Engine-backed mode can precompute device outputs with graph queries. Engine-less mode must follow the same procedure over bundle records and retrieval primitives.

## 12. Output Capsules

OUTPUT capsules specify deliverables and generation constraints.

foundation build OUTPUT:

- `decision_memo`

Initial output templates:

- Decision Memo (BLUF)
- Country Risk Brief
- Options Paper
- Scenario Report
- Negotiation Strategy
- Talking Points
- Intelligence Assessment
- Research Note

Output record:

```json
{
  "format": "decision_memo",
  "sections": ["BLUF", "Situation", "Options", "Risks", "Recommendation", "Open Questions"],
  "max_words": 900,
  "citation_style": "claim_id_inline",
  "freshness_days": 30,
  "tone": "concise_policy_analytic",
  "runtime_contract": {
    "must_cite": true,
    "hedge_T3": true,
    "surface_disputed": true,
    "surface_open_questions": true,
    "do_not_invent_sources": true,
    "refusal_conditions": [
      "No source-backed claim supports the requested factual assertion.",
      "The answer would require asserting a disputed claim as settled.",
      "The user asks for covert operational advice or illegal activity."
    ]
  }
}
```

## 13. Agent Feeding Protocol

`agent_context.md` is the compact context block PRAXIS sends to the model.

Canonical order:

```text
CONTRACT
USER FRAME
OUTPUT SPEC
SITUATION FRAME
TEMPORAL FRAME
ACTOR AND FRICTION MAP
FACTS BY TRUST LAYER
DISPUTED CLAIMS
REASONING DEVICES
PRECOMPUTED DEVICE RESULTS
OPEN QUESTIONS
FRESHNESS WARNINGS
OPERATIONS
REFUSAL AND ESCALATION RULES
```

Rules:

- Contract before content.
- Claims are grouped by trust layer.
- Disputed claims are impossible to miss.
- T3 claims are marked as unverified.
- Device procedures are instructions, not essays.
- Every factual claim has a claim id.
- The context is bounded by token budget.
- The context includes operation handles for deeper retrieval.
- The context says what was omitted due to budget.

Composition must dedupe by:

- entity stable ids;
- Wikidata QID where available;
- source hash;
- claim normalized text;
- event time and actor tuple.

Composition must not erase conflict. If two selected capsules disagree, the composed context must surface the disagreement.

## 14. Capsule Operations

DIALECTICA exposes a stable verb set. All verbs return the same shape whether run engine-less, Postgres-backed, Ladybug-backed, Oxigraph-backed, or Graphiti-backed.

Operation response envelope:

```json
{
  "operation_id": "op_01J...",
  "capsule_id": "cap_01J...",
  "verb": "seek",
  "mode": "engine_less",
  "backend": "pure_python",
  "input": {},
  "result": {},
  "citations": ["clm_...", "src_..."],
  "proposals": [],
  "warnings": [],
  "trust_summary": {"T1": 12, "T2": 8, "T3": 4, "DISPUTED": 1},
  "receipt": {
    "created_at": "2026-06-07T00:00:00Z",
    "bundle_version": 1,
    "deterministic": false
  }
}
```

Verbs:

| Verb | Purpose | May emit proposals? | foundation build mode |
|---|---|---:|---|
| `seek(q)` | answer a question with cited claims and sources | no | engine-less + Postgres |
| `understand()` | summarize actors, dynamics, stakes, timeline, trust state | no | engine-less |
| `connect()` | hypothesize non-obvious links and cross-capsule relationships | yes | engine-less |
| `critique()` | find weak claims, missing actors, assumptions, flip conditions | yes | engine-less |
| `improve()` | produce an enrichment plan and proposed claims/links | yes | engine-less |
| `apply_device(id)` | run a Tool capsule and return typed output | yes, only for inferred findings | engine-less |
| `diff(new_events)` | compute temporal and trust changes | yes | engine-less |
| `compose(set)` | merge selected capsules into one context block | no | engine-less |

Governance:

- `connect`, `critique`, `improve`, and some `apply_device` outputs may propose claims or links.
- Proposed claims are `origin = machine_proposed`, `trust_layer = T3`, `review_status = pending`.
- Operations write receipts.
- Receipts are append-only.
- An operation may never update canonical bundle records directly.

Exposure:

- REST endpoints under `/v1/capsules/...`.
- MCP tools under `capsule.*`.
- Python SDK for in-process test/demo use.

MCP security:

- MCP tools must be read-only by default.
- Mutating tools create proposals only.
- Require explicit auth and owner scope for hosted MCP.
- Treat MCP as a tool surface, not a trust boundary.

## 15. APIs

REST:

```text
GET  /health
GET  /v1/capsules/{capsule_id}
POST /v1/capsules
POST /v1/capsules/{capsule_id}/compile
POST /v1/capsules/{capsule_id}/export
GET  /v1/capsules/{capsule_id}/bundle
POST /v1/capsules/compose

POST /v1/capsules/{capsule_id}/seek
POST /v1/capsules/{capsule_id}/understand
POST /v1/capsules/{capsule_id}/connect
POST /v1/capsules/{capsule_id}/critique
POST /v1/capsules/{capsule_id}/improve
POST /v1/capsules/{capsule_id}/apply-device
POST /v1/capsules/{capsule_id}/diff

GET  /v1/capsules/{capsule_id}/claims
GET  /v1/capsules/{capsule_id}/sources/{source_id}/spans/{span_id}
GET  /v1/capsules/{capsule_id}/graph/neighborhood

GET  /v1/proposals
POST /v1/proposals/{proposal_id}/accept
POST /v1/proposals/{proposal_id}/reject
POST /v1/proposals/{proposal_id}/request-evidence
```

MCP tools:

```text
capsule.seek
capsule.understand
capsule.connect
capsule.critique
capsule.improve
capsule.apply_device
capsule.diff
capsule.compose
capsule.get_claims
capsule.get_source_span
capsule.get_graph_neighborhood
```

Python SDK:

```python
capsule = CapsuleBundle.load("fixtures/coup_case/country_risk.capsule")
answer = capsule.seek("What changed after the coup?")
memo_context = compose([user_capsule, situation_capsule, stakeholder_tool, output_capsule])
```

## 16. Ingestion and External Knowledge

### 16.1 User Documents First

foundation build ingestion starts with user-provided documents and curated fixture sources.

Supported initial inputs:

- PDF
- Markdown
- text
- HTML snapshot
- CSV/JSON datasets
- manually authored source cards

Each source gets:

- source id;
- original URI or upload id;
- title;
- publisher;
- author if known;
- publication time if known;
- retrieval/upload time;
- rights/license;
- file hash;
- extraction hash;
- parser receipt.

Claims without source spans fail validation.

### 16.2 Wikidata

Use Wikidata for entity grounding, not as a blanket source of political truth.

Rules:

- Use search APIs for name-to-candidate discovery.
- Use WDQS SPARQL only for narrow, scoped queries.
- Cache QID candidates and reviewer selections.
- Preserve ambiguity when multiple QIDs match.
- Send an appropriate User-Agent.

### 16.3 GDELT

Use GDELT for event corroboration, media-event signals, and conflict/event pattern enrichment.

Rules:

- Treat GDELT as a corroboration signal, not definitive truth.
- Use BigQuery cost guards.
- Query time-bounded windows only.
- Store query text, parameters, bytes processed, and result hash.
- Map CAMEO/event data to ACO events only through explicit transformation receipts.

### 16.4 Live Web and Search Grounding

Search grounding may enrich claims but may not bypass provenance requirements.

Every grounded result needs:

- query;
- provider;
- retrieved URL;
- retrieved timestamp;
- snippet or fetched span;
- hash;
- generated claim ids.

## 17. Model Policy

All model names live in:

```text
backend/dialectica/providers.py
backend/dialectica/MODEL_MATRIX.md
```

Do not hardcode model ids elsewhere.

Current source-verified aliases as of 2026-06-07:

```python
PRO_MODEL = "gemini-3.1-pro-preview"
FAST_MODEL = "gemini-3.5-flash"
EMBEDDING_MODEL = "gemini-embedding-2"
ANTHROPIC_ANALYSIS_MODEL = "claude-sonnet-4"
```

Provider functions:

```python
get_pro_model()              # structured extraction, hard analysis
get_fast_model()             # cheap operations and summaries
get_deepthink_model(level)   # high-effort critique and device runs
get_analysis_model()         # Anthropic analysis alias with Gemini fallback
get_embedding_model()        # embeddings
get_grounded_model()         # search-grounded extraction/sanity checks
call_model(alias, ...)       # single dispatch path
```

Forbidden outside provider files:

```text
gemini-2.0
gemini-1.5
gemini-3-pro-preview
claude-sonnet-4-20250514
```

Verification:

```bash
rg "gemini-2\.0|gemini-1\.5|gemini-3-pro-preview|claude-sonnet-4-20250514" backend/ tests/ infrastructure/ pyproject.toml
```

If Google or Anthropic model IDs change, update `MODEL_MATRIX.md`, `providers.py`, tests, and this document's source-verified alias block together.

## 18. Security, Privacy, and Governance

Security rules:

- Firebase ID token verification on PRAXIS-authenticated routes.
- OIDC or service-account auth for internal service-to-service calls.
- Owner scope on every capsule, source, proposal, operation, and export.
- No cross-user reads.
- Secrets in Secret Manager.
- No raw secrets in bundle files, logs, test fixtures, or docs.
- Signed URLs for bundle download.
- Append-only operation and review receipts.

Privacy rules:

- USER capsules can contain sensitive work preferences; encrypt at rest and limit export.
- Do not infer protected attributes.
- User interaction-derived style or preference updates are proposals until accepted.
- PII detection runs during ingestion and appears in `review.json`.
- Source rights must be recorded. If rights are unknown, mark the source as restricted.

Governance rules:

- Human signoff is recorded with reviewer id, timestamp, scope, and rationale.
- Expert signoff can promote claims to T1.
- Human dissent creates or preserves DISPUTED status.
- Canon promotion is explicit and human-only.
- Deleting a capsule creates a tombstone; it does not rewrite history.

## 19. Foundation Build

The foundation build is a working PRAXIS Capsule set for a country-risk / coup-style policy case.

Human supplies:

- approximately 15 source documents;
- one target country or fictionalized country case;
- one expected output brief;
- optional expert review notes.

Build:

- USER capsule: `risk_analyst`
- SITUATION capsule: `country_risk`
- TOOL capsules: `stakeholder_analysis`, `scenario_analysis`
- OUTPUT capsule: `decision_memo`

Demo workflows:

1. Produce a cited country-risk decision memo.
2. Ask targeted questions with `seek`.
3. Compare two SITUATION capsules under the same TOOL lens.
4. Update with new events and show what changed using `diff`.
5. Run `critique` and `improve` to produce T3 proposals.

foundation build proof must show:

- source-cited output;
- trust-calibrated generation;
- temporal awareness;
- disputed-claim handling;
- better result than a generic LLM with the same raw documents;
- engine-less operation from bundle files;
- PRAXIS-compatible compose/export contract.

## 20. Source Anchors to Verify Before Implementation

These are the primary sources used for this revision. Re-check them during P0 because several are volatile.

Ladybug:

- Installation docs: `https://docs.ladybugdb.com/installation/`
- GitHub: `https://github.com/LadybugDB/ladybug`

Graphiti:

- Quick start and backend requirements: `https://help.getzep.com/graphiti/getting-started/quick-start`
- Zep vs Graphiti boundary: `https://help.getzep.com/zep-vs-graphiti`

Oxigraph:

- Project/docs: `https://oxigraph.org/`
- PyOxigraph JSON-LD parsing: `https://pyoxigraph.readthedocs.io/en/stable/io.html`

W3C semantic standards:

- JSON-LD 1.1: `https://www.w3.org/TR/json-ld11/`
- SHACL: `https://www.w3.org/TR/shacl/`
- RDF 1.2 Concepts: `https://www.w3.org/TR/rdf12-concepts/`

Google Cloud and models:

- Gemini model lifecycle: `https://docs.cloud.google.com/gemini-enterprise-agent-platform/models/model-versions`
- Gemini 3.1 Pro: `https://docs.cloud.google.com/gemini-enterprise-agent-platform/models/gemini/3-1-pro`
- Gemini 3.5 Flash: `https://docs.cloud.google.com/gemini-enterprise-agent-platform/models/gemini/3-5-flash`
- Gemini Embedding 2: `https://docs.cloud.google.com/gemini-enterprise-agent-platform/models/gemini/embedding-2`
- Claude Sonnet 4 on Vertex: `https://docs.cloud.google.com/gemini-enterprise-agent-platform/models/partner-models/claude/sonnet-4`
- Cloud Run worker pools: `https://docs.cloud.google.com/run/docs/managing/workerpools`
- Spanner Graph query overview: `https://docs.cloud.google.com/spanner/docs/graph/queries-overview`

PostgreSQL:

- pgvector: `https://github.com/pgvector/pgvector`

External knowledge:

- Wikidata data access: `https://www.wikidata.org/wiki/Help:Data_access`
- GDELT data access: `https://www.gdeltproject.org/data.html`
- GDELT BigQuery examples: `https://blog.gdeltproject.org/google-bigquery-gkg-2-0-sample-queries/`

MCP:

- Official MCP docs/spec: `https://modelcontextprotocol.io/`
- Official spec repo: `https://github.com/modelcontextprotocol/modelcontextprotocol`

## 21. Repo Scaffold

```text
A3_DIALECTICAbyTACITUS_v3/
├── README.md
├── pyproject.toml
├── uv.lock
├── Dockerfile
├── Dockerfile.worker
├── docker-compose.yml
├── cloudbuild.yaml
├── .env.example
├── docs/
│   ├── CAPSULE_SPEC.md
│   ├── SOURCE_OF_TRUTH.md
│   ├── OPERATIONS.md
│   ├── AGENT_GUIDE.md
│   ├── PRAXIS_INTEGRATION.md
│   ├── CATEGORIES.md
│   ├── SECURITY_AND_PRIVACY.md
│   ├── MODEL_MATRIX.md
│   ├── EVAL_PLAN.md
│   └── decisions/
│       ├── ADR-001-capsule-bundle-source-of-truth.md
│       ├── ADR-002-postgres-first-operational-store.md
│       └── ADR-003-engine-adapters-not-canonical-state.md
├── infrastructure/
│   ├── setup-gcp.sh
│   ├── deploy-api.sh
│   ├── deploy-worker.sh
│   └── sql/
│       ├── 001_init.sql
│       └── 002_pgvector.sql
├── backend/
│   ├── main.py
│   ├── config.py
│   ├── auth.py
│   ├── logging_config.py
│   ├── dialectica/
│   │   ├── providers.py
│   │   ├── orchestrator.py
│   │   ├── trust.py
│   │   ├── ids.py
│   │   ├── hashing.py
│   │   ├── cores/
│   │   │   ├── aco.py
│   │   │   ├── country_risk.py
│   │   │   ├── conflict.py
│   │   │   └── policy_formulation.py
│   │   ├── devices/
│   │   │   ├── stakeholder_analysis.py
│   │   │   ├── scenario_analysis.py
│   │   │   ├── assumption_check.py
│   │   │   └── pre_mortem.py
│   │   ├── capsule/
│   │   │   ├── schema.py
│   │   │   ├── bundle.py
│   │   │   ├── compiler.py
│   │   │   ├── compose.py
│   │   │   ├── agent_context.py
│   │   │   ├── operations_card.py
│   │   │   ├── semantic_projection.py
│   │   │   └── verify_roundtrip.py
│   │   ├── graph/
│   │   │   ├── base.py
│   │   │   ├── pure_python.py
│   │   │   ├── postgres.py
│   │   │   ├── ladybug.py
│   │   │   ├── oxigraph.py
│   │   │   ├── graphiti.py
│   │   │   └── spanner_graph.py
│   │   ├── operations/
│   │   │   ├── seek.py
│   │   │   ├── understand.py
│   │   │   ├── connect.py
│   │   │   ├── critique.py
│   │   │   ├── improve.py
│   │   │   ├── apply_device.py
│   │   │   └── diff.py
│   │   ├── ingest/
│   │   │   ├── parser.py
│   │   │   ├── chunker.py
│   │   │   ├── source_receipts.py
│   │   │   └── embeddings.py
│   │   ├── agon/
│   │   ├── kairos/
│   │   ├── corroborate/
│   │   │   ├── wikidata.py
│   │   │   └── gdelt.py
│   │   ├── governance/
│   │   │   ├── proposals.py
│   │   │   ├── review.py
│   │   │   └── canon.py
│   │   └── praxis/
│   │       ├── firestore_sync.py
│   │       └── export_contract.py
│   ├── routers/
│   │   ├── health.py
│   │   ├── capsules.py
│   │   ├── compose.py
│   │   ├── operations.py
│   │   ├── retrieve.py
│   │   └── proposals.py
│   ├── mcp/
│   │   ├── server.py
│   │   └── tools.py
│   └── eval/
│       ├── fixtures.py
│       ├── metrics.py
│       ├── run_eval.py
│       └── report.py
├── fixtures/
│   └── coup_case/
└── tests/
    ├── unit/
    ├── integration/
    └── eval/
```

## 22. Build Phases

Each phase must end with docs updates, tests, and a short implementation note. Do not start the next phase until acceptance is met.

### P0 - Scaffold, Source Verification, and Dependency Smoke

Deliver:

- repo skeleton;
- `pyproject.toml`;
- Docker and docker-compose;
- FastAPI `/health`;
- `providers.py`;
- `MODEL_MATRIX.md`;
- docs skeleton;
- source verification log;
- dependency smoke tests.

Dependency smoke must test import/install only:

- FastAPI;
- Pydantic;
- SQLAlchemy or equivalent;
- psycopg;
- pgvector Python client or SQL extension path;
- pyoxigraph;
- ladybug if available;
- graphiti-core as optional;
- google cloud clients;
- MCP SDK.

Acceptance:

- `docker-compose up` boots API.
- `/health` returns 200.
- `pytest` runs at least smoke tests.
- forbidden model grep passes.
- `docs/SOURCE_OF_TRUTH.md`, `docs/MODEL_MATRIX.md`, and ADR-001 exist.
- Optional engines may be marked unavailable, but the health report must say so honestly.

### P1 - Capsule Schema, Bundle Compiler, and Offline Round Trip

Deliver:

- Pydantic schema for four capsule types and all common packs;
- canonical bundle writer/reader;
- deterministic IDs and hashes;
- signing placeholder or local signing implementation;
- fixture bundles for USER, SITUATION, TOOL, OUTPUT;
- `agent_context.md` renderer;
- `operations.md` renderer;
- pure-Python bundle retrieval.

Acceptance:

- all four fixture capsules compile;
- round trip is hash-stable;
- `agent_context.md` groups claims by trust layer;
- source spans are required;
- invalid claims without source spans fail;
- `python -m backend.dialectica.capsule.verify_roundtrip fixtures/coup_case/*.capsule` passes.

### P2 - Engine-Less Compose and Operations

Deliver:

- `compose(set)` over four fixture capsules;
- engine-less `seek`, `understand`, `apply_device`, `critique`, `improve`, `diff`;
- operation receipts;
- proposal envelope.

Acceptance:

- all operations work with no graph DB and no Postgres;
- `seek` returns cited claims;
- `critique` and `improve` create T3 proposals, not edits;
- `apply_device(stakeholder_analysis)` returns a typed matrix;
- `diff` returns structured changes, not prose;
- `docs/OPERATIONS.md` and `docs/AGENT_GUIDE.md` exist.

### P3 - PostgreSQL Operational Store and Ingestion

Deliver:

- SQL migrations;
- source parser/chunker;
- source receipts;
- embeddings path;
- Postgres persistence for capsule metadata, sources, chunks, claims, entities, edges, proposals, review events, and operation receipts.

Acceptance:

- a PDF or markdown source ingests to chunks with hashes;
- chunks can be retrieved by source span;
- claims persist with source links;
- pgvector smoke query works if extension is available;
- Postgres-backed `seek` returns the same cited answer shape as engine-less mode.

### P4 - ACO and Country Risk Extraction

Deliver:

- ACO core;
- `country_risk` core;
- structured extraction via provider aliases;
- actor/entity extraction;
- basic Wikidata candidate grounding;
- source-backed graph edges.

Acceptance:

- fixture corpus yields typed Actor/Claim/Event/Interest/Constraint/Leverage records;
- extracted claims include source spans;
- candidate QIDs are stored as candidates, not silently accepted;
- at least one `CONTENDS_WITH` or equivalent friction edge is created with claim support;
- extraction failure creates a receipt, not partial silent state.

### P5 - KAIROS Temporal Layer

Deliver:

- observed/valid/publication/retrieval time fields;
- episode schema;
- episode segmentation from fixture;
- analyst override fields;
- diff engine.

Acceptance:

- coup fixture has at least three episodes or phases;
- before/after state is source-cited;
- adding a new event creates structured changeset;
- stale/superseded claim behavior is tested.

### P6 - Trust, Disputes, Proposals, and Review Gates

Deliver:

- trust scoring;
- source-class triangulation;
- disputed-claim detection;
- proposal queue;
- accept/reject/request-evidence endpoints;
- review events;
- Canon promotion placeholder.

Acceptance:

- T3 never appears as settled fact in `agent_context.md`;
- disputed claims are surfaced;
- accepted proposal creates a new capsule version;
- rejected proposal remains in review history;
- expert signoff can promote to T1 and is recorded.

### P7 - Optional Engine Adapters

Deliver:

- Ladybug adapter if P0 smoke passed;
- Oxigraph adapter if P0 smoke passed;
- semantic projection validation;
- engine availability in `/health`;
- fallback behavior when adapters are absent.

Acceptance:

- cache materialization is reproducible from bundle records;
- deleting cache and rematerializing yields equivalent query output;
- engine-less tests still pass when adapters are disabled;
- Oxigraph validates JSON-LD projection where available;
- Ladybug query smoke passes where available.

Do not add Graphiti in this phase unless Ladybug/Oxigraph and Postgres paths are stable.

### P8 - Graphiti and Temporal Substrate Adapter

Deliver:

- Graphiti adapter behind feature flag;
- episode write/read mapping;
- search receipts;
- clear boundary between Graphiti mutable context graph and frozen capsule projection.

Acceptance:

- Graphiti can be disabled without breaking capsules;
- Graphiti writes do not mutate signed bundle records;
- frozen capsule graph can be regenerated from canonical records;
- docs state Graphiti is optional.

### P9 - PRAXIS Integration and MCP

Deliver:

- Firestore mirror adapter;
- GCS bundle export;
- `/compose` contract for PRAXIS;
- MCP server with read-only/default proposal semantics;
- PRAXIS integration docs.

Acceptance:

- a capsule appears in PRAXIS mirror shape;
- selected USER+SITUATION+TOOL+OUTPUT composes into one context block;
- MCP client can call `capsule.seek` and `capsule.compose`;
- mutating MCP calls create proposals only;
- `docs/PRAXIS_INTEGRATION.md` exists.

### P10 - Foundation Build Demo and Eval

Deliver:

- country-risk/coup case demo;
- generic LLM baseline;
- PRAXIS-with-capsules run;
- eval report;
- demo script;
- CATEGORIES.md finalized for starter catalog.

Acceptance:

- all demo workflows run engine-less;
- engine-backed modes run where adapters are available;
- eval report includes metrics and examples;
- capsule advantage is demonstrated or gaps are explicitly listed;
- no uncited factual claims in final generated memo.

## 23. Eval Plan

Eval dimensions:

- citation coverage;
- source-span correctness;
- temporal correctness;
- disputed-claim surfacing;
- T3 hedge compliance;
- assumption identification;
- contradiction detection;
- output format adherence;
- analyst usefulness;
- operation determinism where applicable;
- token efficiency.

Suggested thresholds for foundation build:

```text
citation_coverage >= 0.95
uncited_factual_claims == 0
T3_hedge_compliance >= 0.98
disputed_claim_surfacing == 1.0
source_span_precision >= 0.90
temporal_error_rate <= 0.05
format_adherence >= 0.95
```

Baseline:

- generic LLM with raw documents and same output prompt;
- PRAXIS-with-capsules using composed context and operations.

Output:

```text
backend/eval/reports/{timestamp}-mvp-eval.md
```

The eval report must include failures. Do not hide weak results.

## 24. Verification Commands

Run after every phase:

```bash
ruff check backend/
mypy backend/
python -m pytest tests/ -v
rg "gemini-2\.0|gemini-1\.5|gemini-3-pro-preview|claude-sonnet-4-20250514" backend/ tests/ infrastructure/ pyproject.toml
python -m backend.dialectica.capsule.verify_roundtrip fixtures/coup_case/*.capsule
python -m backend.dialectica.operations.verify_engineless fixtures/coup_case/*.capsule
docker-compose up --build
curl -fsS http://localhost:8080/health
```

Phase-specific verification:

```bash
python -m backend.dialectica.eval.run_eval --fixture fixtures/coup_case --mode engine_less
python -m backend.dialectica.graph.smoke --backend postgres
python -m backend.dialectica.graph.smoke --backend ladybug
python -m backend.dialectica.graph.smoke --backend oxigraph
```

Optional adapter smoke failures must not fail the whole build until the phase that explicitly requires that adapter. They must be reported.

## 25. Do Not

- Do not build a separate public DIALECTICA app for foundation build.
- Do not add a new PRAXIS top-level surface unless the human explicitly asks.
- Do not call the public object `Context Capsule`.
- Do not use graph-engine binary files as canonical state.
- Do not require a running graph DB for foundation build operations.
- Do not use Graphiti as required infrastructure before engine-less operations pass.
- Do not use Ladybug package names from stale docs; verify current install path in P0.
- Do not rely on RDF 1.2 triple terms for foundation build claim metadata.
- Do not silently promote machine-generated claims.
- Do not merge contradictory claims.
- Do not ship claims without source spans and hashes.
- Do not hardcode model IDs outside provider files.
- Do not touch PRAXIS auth/rules/middleware/CSP without explicit human approval.
- Do not overbuild the ontology. Start with ACO plus `country_risk`.
- Do not let eval become a decorative report. It is the proof.

## 26. Codex Phase Handoff Template

Use this template when handing Codex a phase:

```markdown
# DIALECTICA v3 - Phase P[NUMBER] Handoff

Read first:
- DIALECTICA_v3_BUILD_INSTRUCTIONS.md
- docs/SOURCE_OF_TRUTH.md
- docs/MODEL_MATRIX.md
- docs/CAPSULE_SPEC.md if present

Objective:
[One paragraph.]

Own:
- [files/directories]

Do not touch:
- PRAXIS auth/rules/middleware/CSP
- model IDs outside providers.py/MODEL_MATRIX.md
- previous signed bundle versions
- optional engine adapters unless this phase owns them

Implement:
1. ...
2. ...
3. ...

Acceptance:
- ...

Verification:
```bash
...
```

Stop and report if:
- source docs contradict this instruction;
- an optional dependency cannot install;
- auth/security changes appear necessary;
- tests fail in a way that implies a source-of-truth change.
```

## 27. Final Build Definition

DIALECTICA v3 is working when:

- a human can provide a policy case source pack;
- DIALECTICA compiles USER, SITUATION, TOOL, and OUTPUT capsules;
- PRAXIS can compose those capsules into one bounded context block;
- a PRAXIS agent can produce a cited, trust-calibrated memo;
- the agent can ask for more evidence through capsule operations;
- temporal changes and disputed claims are handled explicitly;
- the capsule can be operated with no graph database;
- optional graph/semantic engines improve speed or query depth without becoming canonical;
- machine improvements enter a human-gated proposal loop;
- the eval report proves the capsule advantage over raw LLM generation.

That is the foundation build: PRAXIS Augmented Generation with PRAXIS Capsules by TACITUS.
