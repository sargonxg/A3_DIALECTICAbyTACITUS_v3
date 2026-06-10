# PRAXIS Capsule - Format Specification v3.0

Status: canonical contract for DIALECTICA builders and PRAXIS readers.

This is the capsule format for DIALECTICA -> PRAXIS. One container
(`.capsule`), four macro types, built to capture real knowledge, tacit
expertise, and reasoning, and to serve agents as both substrate and guidance.

## 1. What A Capsule Is

A capsule is a portable, governed, self-describing knowledge object. Every
capsule, regardless of type, has two faces:

- **Substrate**: the structured knowledge an agent reasons over: evidence,
  atomic claims, entities, relations, episodes, embeddings, and source state.
  The "what."
- **Guidance**: how to reason with the substrate: methods, mental models,
  tacit heuristics, traps to avoid, and the runtime contract. The "how."

> A capsule is a model of a situation plus a model of how to think about it.

The four macro types package different mixes of substrate and guidance, but
they share one container, one layer vocabulary, and one set of building blocks.

| Type | Question | Dominant face | Role |
| --- | --- | --- | --- |
| `user` | Who am I working as or for? | guidance about the analyst | personalize voice, audience, intellectual style, standing positions |
| `situation` | What is going on? | substrate | the grounded, trust-layered model of a case |
| `tool` | How should I think? | guidance method | an executable analytical method plus tacit expertise |
| `output` | What should I produce? | guidance deliverable | the deliverable template and contract |

PRAXIS composes a set:

```text
1 User + 1..n Situation + 0..n Tool + 1 Output
```

The composed context tells the agent who it is working for, what is known, how
to reason, and what to produce.

## 2. The `.capsule` Container

A capsule is a Zip archive with extension `.capsule` and MIME type:

```text
application/vnd.tacitus.praxis-capsule+zip
```

The first Zip entry is an uncompressed `mimetype` file holding that exact
string, following the EPUB sniffing pattern. There is one extension for all
four types; `manifest.json.type` discriminates the type. Versions live in the
manifest, not in the suffix.

Filename pattern:

```text
<title-slug>.<short-id>.capsule
```

Portability rule:

- canonical files are open JSON, JSONL, JSON-LD, and Markdown;
- `graph.jsonld` is the rebuildable semantic graph contract;
- `graph/ladybug/capsule.lbug` is required for every promoted capsule as the
  embedded queryable graph projection;
- Ladybug projections are read-only for PRAXIS and must carry digest receipts;
- non-required caches remain optional, regenerable, and excluded from promotion
  decisions.

## 3. Layer Model

The capsule knowledge model has nine layers. In canonical `graph.jsonld`, these
are named graphs. Some layers also have dedicated files.

| # | Layer | Named graph | Face | Holds |
| --- | --- | --- | --- | --- |
| 1 | Evidence | `g:evidence` | substrate | sources, spans, hashes, rights, provenance |
| 2 | Claim | `g:claims` | substrate | atomic typed claims |
| 3 | Graph / relation | `g:situation` | substrate | entities and relations |
| 4 | Temporal / episodic | `g:temporal` | substrate | episodes, intervals, causal links |
| 5 | Semantic / ontology | `g:ontology` | substrate | ACO core, domain cores, OWL/SHACL |
| 6 | Reasoning / guidance | `g:reasoning` | guidance | devices, heuristics, mental models, traps, annotations |
| 7 | Memory | `g:memory` | meta | semantic, episodic, procedural build memory |
| 8 | Governance / trust | `g:governance` | meta | trust tiers, review, signoff, dissent, corroboration |
| 9 | Runtime / contract | `g:runtime` | guidance | how an agent must retrieve, cite, combine, and act |

Substrate and guidance live in one connected graph. A claim can link to its
evidence, episode, annotations, heuristics, caveats, traps, review decisions,
and runtime rules. Layers are views over a connected graph, not separate
stores.

## 4. Shared Envelope: `manifest.json`

```json
{
  "capsule_id": "cap_01HQ...",
  "spec_version": "3.0",
  "version": 4,
  "type": "situation",
  "category": "country_risk",
  "title": "Country X - coup and aftermath",
  "owner_uid": "...",
  "situation_id": "...",
  "cores": ["aco", "country_risk"],
  "created_at": "...",
  "updated_at": "...",
  "provenance_root_hash": "sha256:...",
  "signature": "...",
  "depends_on": [],
  "layers_present": [
    "evidence",
    "claims",
    "situation",
    "temporal",
    "ontology",
    "reasoning",
    "governance",
    "runtime"
  ]
}
```

Only four `type` values are PRAXIS-importable:

```text
user | situation | tool | output
```

Everything else is an internal layer, lens, payload, listing field, or review
object inside one of those four types.

## 5. Shared Building Blocks

### 5.1 Claim: `claims.jsonl`

Claims are the atoms of substrate.

```json
{
  "id": "clm_...",
  "text": "The army chief publicly rejected the certification.",
  "primitive": "Claim",
  "subject": "ent_army_chief",
  "predicate": "rejects",
  "object": "ent_ec_cert",
  "cores": ["aco", "country_risk"],
  "source_span": {
    "source_id": "src_07",
    "start": 1840,
    "end": 1979
  },
  "confidence": 0.74,
  "corroboration": {
    "score": 0.82,
    "sources": ["src_07", "gdelt:...", "wd:Q..."]
  },
  "trust_layer": "T1",
  "disputed": false,
  "valid_from": "2026-02-14",
  "valid_to": null,
  "observed_at": "...",
  "superseded_by": null,
  "episode_id": "ep_precoup_2026",
  "origin": "extracted",
  "annotations": ["ann_crux_01"]
}
```

### 5.2 Entity And Relation: `graph.jsonld`

Entities carry grounded identifiers where available. Relations are typed; the
conflict-bearing relations are first-class:

```text
(:Actor)-[:CONTENDS_WITH {over, means, intensity}]->(:Actor)
(:Office)-[:FRICTION_WITH {issue}]->(:Office)
(:Actor)-[:HOLDS]->(:Interest)
(:Actor)-[:WIELDS]->(:Leverage)
(:Actor)-[:BOUND_BY]->(:Constraint)
(:Actor)-[:MADE]->(:Commitment)
```

### 5.3 Episode: `episodes.json`

```json
{
  "id": "ep_precoup_2026",
  "type": "regime_phase",
  "label": "Pre-coup constitutional order",
  "t_start": "2025-11-01",
  "t_end": "2026-02-14",
  "boundary_event": "evt_coup_declaration",
  "fuzzy": true,
  "participants": ["ent_army", "ent_president"],
  "member_claims": ["clm_..."],
  "state_before": "Civilian government holds nominal control.",
  "state_after": "Army asserts executive authority.",
  "causes": ["ep_election_dispute_2026"],
  "leads_to": ["ep_postcoup_transition"]
}
```

### 5.4 Evidence: `evidence/sources.jsonl`

Each source record must carry source id, URI, title, publisher, retrieval time,
content hash, rights, and chunk references. Source blobs may live in
`evidence/blobs/` and are referenced by hash.

## 6. Tacit Knowledge And Reasoning

Tacit knowledge is captured as structured guidance objects attached at capsule
level or to a specific claim, entity, episode, or output rule. These objects
must be reachable from the connected graph.

### 6.1 Heuristic: `reasoning/heuristics.json`

An expert judgment rule.

```json
{
  "id": "heu_premobilization",
  "statement": "A military actor's public rejection of a civilian institution near a constitutional deadline signals elevated escalation risk.",
  "trigger": "(:Actor{type:'Military'})-[:rejects]->(:Institution) within N days of a (:Constraint{type:'deadline'})",
  "inference": "raise escalation_risk to HIGH; probe for mobilization indicators",
  "scope": ["aco", "country_risk"],
  "confidence": 0.7,
  "source": "expert:desk; derivation:case-pattern",
  "examples": ["case_..."]
}
```

### 6.2 Mental Model / Lens

Reusable lenses such as Galtung's ABC triangle, Fisher-Ury
interests-vs-positions, principal-agent, two-level games, escalation ladders,
stakeholder analysis, conflict mapping, or ACH belong primarily in Tool
Capsules.

### 6.3 Salience Prior: `reasoning/salience.json`

```json
{
  "target": "primitive:Commitment",
  "weight": 0.9,
  "rationale": "Commitments predict behavior better than narrative."
}
```

### 6.4 Anti-Pattern / Trap: `reasoning/traps.json`

```json
{
  "id": "trap_mirror_imaging",
  "name": "Mirror-imaging",
  "description": "Assuming an adversary reasons as we do.",
  "detection_prompt": "Where has an actor's motive been inferred from our own framing rather than their stated interests?"
}
```

Traps feed the `critique()` verb directly.

### 6.5 Precedent / Analogy: `reasoning/precedents.json`

```json
{
  "this": "cap_country_x",
  "resembles": "cap_country_y_2021",
  "basis": "contested certification plus neutral-then-aligned army",
  "lessons": ["..."]
}
```

### 6.6 Annotation: `reasoning/annotations.json`

```json
{
  "id": "ann_crux_01",
  "target": "clm_...",
  "author": "analyst:GC",
  "type": "crux",
  "text": "This is the hinge: if true, escalation is near-certain.",
  "affects_trust": false
}
```

Reusable domain and method expertise belongs in Tool Capsules. Individual
analyst style and standing heuristics belong in User Capsules.
Situation-specific judgments and caveats belong as annotations inside
Situation Capsules.

## 7. Four Types In Detail

All types share the envelope, container, layer vocabulary, and building blocks.
They differ in which layers they populate heavily.

| Layer | User | Situation | Tool | Output |
| --- | --- | --- | --- | --- |
| Evidence | light | heavy | optional | optional |
| Claim | standing positions | heavy | optional | optional |
| Graph / relation | optional affiliations | heavy | required patterns | optional lineage |
| Temporal / episodic | optional | heavy | if temporal device | optional |
| Semantic / ontology | persona schema | domain cores | method primitives | output schema |
| Reasoning / guidance | style and heuristics | open questions, annotations | primary payload | template and contract |
| Memory | history/preferences | build provenance | usage notes | lineage |
| Governance / trust | privacy/review | heavy | device validation | output approval |
| Runtime / contract | voice/audience | freshness/citation | how to apply | format/cite/refusal |

### 7.1 User Capsule: `payload.user.json`

```json
{
  "role": "UN political affairs officer",
  "seniority": "senior",
  "expertise": ["Middle East", "mediation"],
  "audience": "Security Council members",
  "voice": "measured, institutional",
  "intellectual_style": ["interests-over-positions", "scenario-minded", "skeptical of single-source"],
  "standing_positions": [{ "claim": "...", "since": "..." }],
  "preferred_devices": ["fisher_ury_interests", "scenario_analysis"],
  "heuristics_ref": "reasoning/heuristics.json"
}
```

### 7.2 Situation Capsule

The situation payload is the substrate:

- `claims.jsonl`
- `graph.jsonld`
- `episodes.json`
- `evidence/`
- `review/`
- `reasoning/annotations.json`
- `reasoning/precedents.json`
- `reasoning/traps.json`

The bulk of AGON and KAIROS output lands here when those adapters are present.
Open questions live in `review/review.json`.

### 7.3 Tool Capsule: `payload.tool.json`

```json
{
  "id": "stakeholder_analysis",
  "purpose": "Map actors by influence x interest.",
  "required_primitives": ["Actor", "Interest", "Leverage", "Constraint"],
  "graph_queries": ["MATCH (a:Actor)-[:HOLDS]->(i:Interest) RETURN a, collect(i)"],
  "procedure": [
    "Score influence x interest salience.",
    "Cluster coalitions.",
    "Flag swing actors."
  ],
  "heuristics": ["heu_..."],
  "traps": ["trap_mirror_imaging"],
  "output_schema": "StakeholderMatrix",
  "critique_prompts": ["Which high-influence actor is missing from the sources?"]
}
```

A Tool Capsule is pure guidance made executable. It declares the graph patterns
it needs, the moves, the tacit heuristics and traps, and the typed output.

### 7.4 Output Capsule: `payload.output.json`

```json
{
  "format": "decision_memo",
  "sections": ["BLUF", "situation", "options", "risks", "recommendation"],
  "max_words": 900,
  "citation_style": "claim_id_inline",
  "runtime_contract": {
    "must_cite": true,
    "hedge_T3": true,
    "surface_disputed": true,
    "freshness_days": 30,
    "combination_rules": "dedupe by QID; flag conflicts",
    "refusal": ["do not assert disputed claims as fact"]
  }
}
```

## 8. One Connected Graph; Ladybug Materializes It

`graph.jsonld` is the canonical graph serialization. Named graphs correspond to
layers. The substrate and guidance layers form a single connected graph:
claims <-> evidence, claims <-> episodes, claims <-> annotations/heuristics/traps,
actors <-> relations, and runtime rules <-> outputs.

For queryability, a promoted capsule must ship a Ladybug projection:

```text
graph/ladybug/capsule.lbug
graph/ladybug/projection_manifest.json
graph/ladybug/schema.cypher
graph/ladybug/queries.cypher
graph/ladybug/build_receipt.json
```

Ladybug materializes graph traversal and read-only Cypher inspection. The
projection is required for promoted capsules and carries its own digest
receipts, but it is still rebuildable from `graph.jsonld` and excluded from the
external integrity envelope. Oxigraph, vector indexes, and full-text indexes
may be added later as optional derived caches.

## 9. Trust Layers

Trust layers apply to Situation claims:

- **T1 Vetted**: assert.
- **T2 Corroborated**: attribute.
- **T3 Needs corroboration**: hedge.
- **Disputed flag**: surface, do not silently resolve.

Anything an agent proposes through `improve()` or `connect()` enters as T3 and
must pass human gating before it counts.

## 10. Agent View

Every bundle ships two generated entry points:

- `agent_context.md`: compiled, bounded, self-citing context block.
- `operations.md`: self-describing card explaining how to operate the capsule
  with no database.

`agent_context.md` should follow this order:

```text
CONTRACT
SITUATION FRAME
EPISODES
ACTOR AND FRICTION MAP
FACTS BY TRUST
REASONING SCAFFOLD
OPEN QUESTIONS
OPERATIONS
```

The verb set must work with or without an engine:

```text
seek · understand · connect · critique · improve · apply_device · diff · compose
```

Substrate verbs query or traverse. Guidance verbs follow encoded methods,
heuristics, and traps. `improve` and `connect` emit T3 proposals back into
gating.

## 11. Composition Across Types

`compose(1 User + 1..n Situation + 0..n Tool + 1 Output)` merges bundles into a
bounded, self-citing context block:

- dedupe entities by grounded id where possible;
- reconcile or flag conflicting claims, never silently merge;
- layer facts by trust;
- inject the User persona and Output contract at the top;
- precompute each Tool device over the Situation graph;
- preserve citations, caveats, rights, freshness, and review gates.

The result tells the agent who it is, what is known and how sure, how to think,
and what to produce.

## 12. Validation And Integrity

- SHACL shapes per loaded core validate the graph.
- `integrity/envelope.json` records canonical file leaves, a path-bound Merkle
  root, and Ed25519 author/publisher signatures over the envelope payload.
- The integrity envelope signs canonical package files, including
  `manifest.json`, and excludes only `integrity/envelope.json` plus rebuildable
  `graph/ladybug/*` projection files.
- `provenance_root_hash` and `signature` remain manifest compatibility
  metadata in v3.0; verification uses the external integrity envelope.
- `graph/ladybug/projection_manifest.json` records the exact digest of
  `graph.jsonld`, `capsule.lbug`, `schema.cypher`, and `queries.cypher`.
- The Ladybug database is opened read-only by PRAXIS. DIALECTICA rebuild jobs
  own writes and must regenerate receipts.
- Generated `agent_context.md` and `operations.md` must be reproducible from
  canonical files.

## 13. Full Bundle

```text
<title-slug>.<short-id>.capsule
├── mimetype
├── manifest.json
├── claims.jsonl
├── graph.jsonld
├── episodes.json
├── evidence/
│   ├── sources.jsonl
│   └── blobs/
├── reasoning/
│   ├── devices.json
│   ├── heuristics.json
│   ├── salience.json
│   ├── traps.json
│   ├── precedents.json
│   ├── annotations.json
│   └── plan.json
├── payload.<type>.json
├── review/
│   └── review.json
├── runtime.json
├── integrity/
│   └── envelope.json
├── agent_context.md
├── operations.md
└── graph/
    └── ladybug/
        ├── capsule.lbug
        ├── projection_manifest.json
        ├── schema.cypher
        ├── queries.cypher
        └── build_receipt.json
```

Situation Capsules may omit `payload.situation.json` when the substrate files
are the payload. User, Tool, and Output Capsules must include their matching
payload file.

## 14. Foundation Cut

The first build must hold scope without weakening the target:

- required layers: evidence, claims, situation graph, temporal, ontology,
  reasoning, governance, runtime;
- required compiled views: `agent_context.md` and `operations.md`;
- required guidance objects: `devices`, `annotations`, and `traps`;
- supported but initially sparse: `heuristics`, `salience`, `precedents`,
  `memory`, Oxigraph cache, and non-required retrieval caches;
- ship one canonical Situation Capsule fixture and keep User, Tool, and Output
  examples aligned to the same manifest vocabulary;
- all promoted capsules must validate as v3 and include a read-only embedded
  Ladybug graph projection.

Substrate is the knowledge. Guidance is the judgment. The capsule carries both
as a portable `.capsule` with open semantic files and an embedded queryable
graph database that humans gate as it grows into the Canon.
