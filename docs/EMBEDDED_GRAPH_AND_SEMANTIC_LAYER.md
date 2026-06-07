# Embedded Graph And Semantic Layer

## Purpose

Every serious capsule needs an embedded graph, but not every capsule needs the
same graph. The graph is how PRAXIS sees relationships, provenance, review
state, and usable meaning inside the capsule. A situation capsule may need
actors, institutions, claims, events, risks, and decisions. A user capsule may
need role, authority, preference, privacy, and output-contract relationships. A
thinking-device capsule may need method steps, required inputs, failure modes,
and reviewer caveats.

The graph is **embedded** because it travels with the capsule bundle. It can be
visualized in PRAXIS, projected into PostgreSQL tables, exported as JSON-LD, or
served through an MCP resource without requiring a dedicated graph database.

The canonical graph vocabulary lives in
[Graph Profile Registry](GRAPH_PROFILE_REGISTRY.md). Use that file before
adding node classes, edge classes, graph previews, or capsule-type graph
profiles. The registry is an interoperability floor, not a universal ontology.
The capsule-specific ontology blueprint decides which semantic layers matter for
the capsule being built.

```text
          capsule type + domain + workflow
                       |
                       v
              ontology blueprint
                       |
       +---------------+----------------+
       |                                |
       v                                v
 local semantic layers           graph profile
       |                                |
       +---------------+----------------+
                       v
              embedded graph
```

## Design Rule

PostgreSQL and the signed capsule bundle are canonical. Graph engines,
embedding stores, MCP servers, and memory layers are derived views until an ADR
promotes one of them.

This keeps the first backend operable while preserving a path to richer graph
systems later.

## Graph Slice Files

The capsule carries graph information in three files:

```text
graph_slice.json          compact graph for PRAXIS runtime and UI
graph_semantics.jsonld    linked-data view for interoperability
graph_constraints.json    validation constraints and required profiles
```

`graph_slice.json` is the operational format. `graph_semantics.jsonld` is the
semantic export. `graph_constraints.json` is the validation profile.

The ontology blueprint is a planner contract generated from the manifest or
bundle. It guides the creation of `ontology_slice.json`, `graph_slice.json`,
`reasoning_playbook.json`, and `agent_guidance.json`. It is executable today via:

```powershell
cargo run -p dialectica-cli -- ontology-plan fixtures/golden-policy-capsule/expected-bundle
```

## Node Classes

Canonical node classes are registered in
[Graph Profile Registry](GRAPH_PROFILE_REGISTRY.md). The list below is the
working explanation of the shared vocabulary. Capsule builders may add
domain-specific properties and local aliases, but those aliases must map back to
registered classes for PRAXIS interoperability.

| Node class | Meaning | Required fields |
| --- | --- | --- |
| `actor` | Person, organization, coalition, stakeholder group. | `id`, `label`, `actor_type`, `review_state` |
| `institution` | Public body, agency, court, party, firm, platform, NGO. | `id`, `label`, `jurisdiction`, `authority_type` |
| `source` | Source document, dataset, interview, note, feed item. | `id`, `source_id`, `trust_status`, `published_at` |
| `source_span` | Specific passage, table, page, paragraph, or timestamp. | `id`, `source_id`, `locator`, `hash` |
| `claim` | Factual, causal, normative, forecast, or procedural claim. | `id`, `claim_text`, `claim_type`, `confidence` |
| `event` | Dated occurrence or procedural milestone. | `id`, `label`, `event_time`, `temporal_status` |
| `concept` | Term from the capsule ontology. | `id`, `label`, `definition`, `scheme_id` |
| `policy_instrument` | Law, rule, subsidy, tax, target, sanction, procedure. | `id`, `label`, `instrument_type`, `authority` |
| `risk` | Analytical, implementation, legal, political, or ethical risk. | `id`, `label`, `severity`, `mitigation_status` |
| `decision` | Choice point, recommendation, review decision, or output action. | `id`, `label`, `owner`, `deadline` |
| `reasoning_device` | Method used to reason through the issue. | `id`, `label`, `device_type`, `review_state` |
| `review_action` | Human approval, rejection, caveat, escalation, recertification. | `id`, `reviewer_id`, `decision`, `created_at` |

## Edge Classes

Canonical edge classes are registered in
[Graph Profile Registry](GRAPH_PROFILE_REGISTRY.md). Exported graph records
must use those edge names or approved aliases.

| Edge class | Meaning |
| --- | --- |
| `supports` | Source or span supports a claim. |
| `contradicts` | Source, claim, or reviewer note conflicts with another claim. |
| `mentions` | Source or span mentions an actor, institution, event, or concept. |
| `authored_by` | Source was authored or issued by an actor or institution. |
| `regulated_by` | Actor, sector, or instrument is governed by an authority. |
| `influences` | Actor or institution can influence another actor or decision. |
| `incentivized_by` | Actor behavior is shaped by incentive or constraint. |
| `causes` | Claim asserts a causal mechanism. |
| `depends_on` | Claim, decision, or scenario depends on another condition. |
| `supersedes` | Claim or source replaces an earlier claim or source. |
| `belongs_to_frame` | Claim or concept belongs to an analytical frame. |
| `uses_device` | Output or analysis used a reasoning device. |
| `reviewed_by` | Object was reviewed by a reviewer or review action. |
| `forbidden_for` | Rights or review policy blocks a workflow. |

Every edge must carry:

- `edge_id`;
- `from_node_id`;
- `to_node_id`;
- `edge_type`;
- `source_ids`;
- `source_span_ids`;
- `created_by_run_id`;
- `created_at`;
- `confidence`;
- `review_state`;
- `temporal_scope`;
- `explanation`.

## Graph Slice Shape

`graph_slice.json` must be renderable and validatable:

```json
{
  "schema_version": "0.1.0",
  "capsule_id": "cap_eu_energy_stakeholders_2026_q3",
  "graph_profile": "stakeholder_graph_v1",
  "nodes": [],
  "edges": [],
  "communities": [],
  "layout_hints": {
    "default_lens": "stakeholder_map",
    "ranked_focus_nodes": [],
    "review_overlay": true,
    "temporal_filter_default": "current"
  },
  "health": {
    "unsupported_edge_count": 0,
    "unreviewed_edge_count": 0,
    "stale_edge_count": 0,
    "contradiction_cluster_count": 0
  }
}
```

See `docs/GRAPH_PROFILE_REGISTRY.md` for the full fixture-quality example.

## Semantic Layer

The semantic layer gives stable meaning to the graph. DIALECTICA should borrow
from standards without forcing every capsule into a heavyweight RDF stack or one
global actor/claim ontology.

The required order is:

1. choose the capsule type and intended PRAXIS workflow;
2. generate an ontology blueprint;
3. build the capsule-specific semantic layers;
4. map local terms and graph objects to the shared registry;
5. export standards-shaped views only where they help interoperability.

| Need | Design anchor | DIALECTICA use |
| --- | --- | --- |
| Linked graph JSON | JSON-LD 1.1 | optional `graph_semantics.jsonld` export |
| Provenance | PROV-O | source, extraction, reviewer, and compiler lineage |
| Controlled concepts | SKOS | domain vocabularies, synonyms, broader/narrower terms |
| Graph validation | SHACL | capsule profile constraints and required fields |
| Usage rights | ODRL | permissions, prohibitions, duties, and reuse constraints |
| Review credentials | VC Data Model / DID Core inspiration | portable reviewer attestations when needed |
| Runtime policy | OPA/Rego inspiration | deployment authorization and promotion rules |

The capsule does not need to implement all of these standards fully on day one.
It should design fields so migration to these standards is possible.

See [Graph, Ontology, And Capsule Research Notes](GRAPH_ONTOLOGY_RESEARCH_NOTES.md)
for the latest research-backed adapter and standards decisions.
See [Ontology Blueprints](ONTOLOGY_BLUEPRINTS.md) for the capsule-specific
planner contract.

## Ontology Slice Shape

`ontology_slice.json` should be a working semantic contract for this capsule,
not only a list of topics and not a universal taxonomy:

```json
{
  "ontology_id": "ontology:eu-energy-policy",
  "version": "0.1.0",
  "namespace": "https://tacitus.me/ns/policy/eu-energy#",
  "language": "en",
  "terms": [
    {
      "term_id": "concept:state-aid",
      "label": "State aid",
      "definition": "Public support that may affect market competition.",
      "source_span_ids": ["span:commission_guidance:3"],
      "broader": ["concept:competition-policy"],
      "review_state": "approved"
    }
  ],
  "mappings": [],
  "frame_memberships": [],
  "deprecations": []
}
```

The ontology slice may be compact, but it must answer what PRAXIS needs to know
about the capsule's local meaning: approved concepts, disputed concepts,
synonyms, broader/narrower relations, frame memberships, deprecated terms,
jurisdiction or domain scope, and review state.

## JSON-LD Shape

Example semantic export:

```json
{
  "@context": {
    "dc": "http://purl.org/dc/terms/",
    "prov": "http://www.w3.org/ns/prov#",
    "skos": "http://www.w3.org/2004/02/skos/core#",
    "odrl": "http://www.w3.org/ns/odrl/2/",
    "dialectica": "https://tacitus.me/ns/dialectica#"
  },
  "@id": "urn:praxis:capsule:stakeholders-eu-energy-2026",
  "@type": "dialectica:StakeholderCapsule",
  "dc:title": "EU industrial electricity support stakeholder map",
  "prov:wasGeneratedBy": "urn:dialectica:compile-run:run_2026_06_07_001",
  "dialectica:hasGraph": {
    "@id": "urn:praxis:capsule:stakeholders-eu-energy-2026#graph"
  }
}
```

## Graph Storage In PostgreSQL

The embedded graph maps cleanly to simple relational tables:

```text
graph_nodes(id, capsule_id, node_type, label, ref_table, ref_id, review_state)
graph_edges(id, capsule_id, from_node_id, to_node_id, edge_type,
            confidence, review_state, source_span_id, created_by_run_id)
ontology_terms(id, capsule_id, term_type, label, definition, parent_term_id)
ontology_mappings(id, capsule_id, entity_id, term_id, confidence, review_state)
```

JSONB columns can store extension fields. pgvector can support semantic search
without introducing a separate vector database.

## LadybugDB Projection Adapter

LadybugDB is a candidate adapter for capsule graph exploration, not canonical
state. Its useful fit is:

- local or service-side projected graph analysis;
- Cypher query workflows for graph inspectors;
- influence, community, and centrality algorithms over selected capsule graphs;
- large embedded graph previews where PostgreSQL traversal becomes awkward.

The adapter should read from `graph_slice.json` or PostgreSQL graph tables and
write projection receipts. It must not write promoted capsule facts without the
normal source-span and review-ledger path.

Initial adapter profile:

```json
{
  "adapter_profile": "ladybug_projection_v1",
  "source": "graph_slice.json",
  "mode": "derived_projection",
  "allowed_outputs": ["algorithm_scores", "layout_hints", "query_receipts"],
  "forbidden_outputs": ["canonical_claim", "review_promotion"]
}
```

## PRAXIS Visualization Contract

PRAXIS should be able to load `graph_preview_v1` from the graph preview API:

```json
{
  "schema_version": "graph_preview_v1",
  "capsule_id": "cap_eu_energy_stakeholders_2026_q3",
  "graph_profile": "stakeholder_graph_v1",
  "nodes": [],
  "edges": [],
  "clusters": [],
  "review_styles": {
    "approved": "solid",
    "approved_with_caveats": "dashed",
    "needs_review": "muted",
    "rejected": "hidden_by_default"
  },
  "temporal_filters": ["current", "stale", "superseded", "forecast", "contested"],
  "source_receipt_links": [],
  "warnings": []
}
```

The graph UI should make review state visible. Proposed edges should not look
like approved edges.

## Graph Review Lifecycle

Graph nodes and edges are reviewable objects:

```text
proposed -> needs_review -> approved
                         -> approved_with_caveats
                         -> rejected
                         -> expired
```

Every promoted graph object must carry `review_scope`, `review_action_ids`,
`caveat_ids`, `expires_at`, and `blocked_workflows` when those fields apply.

## Cross-Capsule Graphs

When PRAXIS loads multiple capsules, DIALECTICA should expose a merged graph
view with clear lineage:

```text
Capsule A graph  \
                  -> merged view -> conflict detector -> PRAXIS answer plan
Capsule B graph  /
```

The merge should:

- preserve original capsule ids;
- keep conflicting claims side by side;
- prefer fresher approved claims over stale draft claims;
- warn when ontology blueprints or local term mappings disagree;
- avoid writing merged facts back as canonical without review.

## Graph Health

The graph layer should report:

- unsupported edge count;
- unreviewed edge count;
- stale temporal edge count;
- isolated high-importance nodes;
- overloaded ambiguous concepts;
- contradiction clusters;
- source monoculture risk;
- reviewer coverage gaps.

These checks matter because an impressive graph without provenance is only a
visual hallucination.

## Source Anchors

- JSON-LD 1.1: <https://www.w3.org/TR/json-ld11/>
- SHACL: <https://www.w3.org/TR/shacl/>
- PROV-O: <https://www.w3.org/TR/prov-o/>
- SKOS: <https://www.w3.org/TR/skos-reference/>
- ODRL: <https://www.w3.org/TR/odrl-model/>
- Verifiable Credentials Data Model 2.0: <https://www.w3.org/TR/vc-data-model-2.0/>
- DID Core: <https://www.w3.org/TR/did-1.0/>
- PostgreSQL JSON types: <https://www.postgresql.org/docs/current/datatype-json.html>
- pgvector: <https://github.com/pgvector/pgvector>
