# Graph Profile Registry

Status: canonical registry for embedded capsule graph profiles.

Use this file when implementing `graph_slice.json`, graph validation,
PostgreSQL graph tables, PRAXIS graph preview APIs, and capsule examples.

## Core Rule

Every graph node and edge in a promoted capsule must use a registered class or
declare an explicit local alias that maps to a registered class. Examples,
fixtures, APIs, and UI previews should not invent edge names ad hoc.

This registry is not the ontology for every capsule. It is the shared
interoperability vocabulary that lets PRAXIS combine, preview, validate, and
route capsules safely. The capsule-specific ontology blueprint decides which
registered classes are relevant and which local semantic layers, terms, and
properties must be captured for the matter at hand.

Use this sequence:

```text
capsule type + workflow
  -> ontology blueprint
  -> local ontology slice
  -> graph profile and aliases
  -> PRAXIS preview/context pack
```

Actor/claim/time graphs are first-class for Situation Capsules, especially
conflict and policy matters. They are not the default mental model for User,
Tool, or Output Capsules. Source proof, stakeholder maps, scenario trees,
domain semantics, expert picks, and graph modules are graph lenses or metadata
inside the four PRAXIS-importable capsule classes.

## Canonical Node Classes

| Node class | Required for | Meaning |
| --- | --- | --- |
| `actor` | situation, user | person, organization, coalition, stakeholder group |
| `institution` | situation, user | public body, agency, court, firm, NGO, platform |
| `source` | situation, tool, output | source document, dataset, interview, note, feed item |
| `source_span` | situation, tool, output | passage, page, table, paragraph, timestamp, or cell |
| `claim` | situation, output | factual, causal, normative, forecast, or procedural claim |
| `event` | situation, output | dated occurrence or procedural milestone |
| `concept` | situation, tool, user | ontology term or frame concept |
| `policy_instrument` | situation, tool | law, rule, subsidy, tax, sanction, target, procedure |
| `risk` | situation, tool, output | analytical, legal, implementation, political, ethical risk |
| `decision` | situation, output | choice point, recommendation, review decision, output action |
| `reasoning_device` | tool, situation, output | method used to reason through the issue |
| `review_action` | all promoted capsules | approval, rejection, caveat, escalation, recertification |
| `output_contract` | output, situation | rules for memo, brief, graph preview, or agent context pack |
| `rights_policy` | all promoted capsules | permissions, prohibitions, duties, export/sharing rules |

## Canonical Edge Classes

| Edge class | Meaning | Notes |
| --- | --- | --- |
| `supports` | source or span supports a claim | source-backed |
| `contradicts` | object conflicts with another claim or object | preserve both sides |
| `mentions` | source/span names an actor, event, institution, or concept | low inference |
| `authored_by` | source was issued by actor or institution | provenance |
| `regulated_by` | actor, sector, instrument, or decision is governed by authority | authority link |
| `influences` | actor or institution can influence another object | non-causal unless stated |
| `incentivized_by` | actor behavior is shaped by incentive or constraint | stakeholder lens |
| `causes` | claim asserts a causal mechanism | requires causal caveat |
| `depends_on` | claim, decision, scenario, or output depends on condition | conditional link |
| `supersedes` | claim or source replaces earlier claim/source | temporal link |
| `belongs_to_frame` | claim/concept belongs to analytical frame | semantic link |
| `uses_device` | output, claim, or analysis used a reasoning device | reasoning trace |
| `reviewed_by` | object was reviewed by review action or reviewer | review trace |
| `forbidden_for` | rights/review policy blocks a workflow | usage control |
| `has_output_rule` | capsule or decision is governed by an output contract | output link |
| `has_rights_policy` | capsule/listing is governed by rights policy | marketplace link |

## Approved Aliases

Examples may use domain language, but exported graph records must normalize to
canonical edges:

| Local phrase | Canonical edge |
| --- | --- |
| `constrains` | `depends_on` |
| `benefits` | `incentivized_by` |
| `criticizes` | `contradicts` |
| `blocks` | `depends_on` with `polarity: "blocking"` |
| `funds` | `influences` with `mechanism: "funding"` |

## Object State Machine

Graph objects move through a review-aware lifecycle:

```text
proposed
  -> needs_review
       -> approved
       -> approved_with_caveats
       -> rejected

approved              -> expired
approved_with_caveats -> expired
```

Required review fields for promoted graph objects:

- `review_state`;
- `review_scope`;
- `review_action_ids`;
- `caveat_ids`;
- `expires_at`;
- `blocked_workflows`;
- `reviewed_at`.

Rejected and expired objects remain in lineage and may appear in audit views,
but PRAXIS should not use them silently for workflow grounding.

## Graph Slice Shape

`graph_slice.json` must use this top-level shape:

```json
{
  "schema_version": "0.1.0",
  "capsule_id": "cap_eu_energy_situation_2026_q3",
  "graph_profile": "situation_graph_v1",
  "nodes": [
    {
      "id": "actor:european-commission",
      "node_type": "institution",
      "label": "European Commission",
      "review_state": "approved",
      "review_scope": "graph_fact",
      "review_action_ids": ["review:legal_editor:2026-06-01"],
      "caveat_ids": [],
      "expires_at": "2026-12-31T23:59:59Z",
      "blocked_workflows": [],
      "reviewed_at": "2026-06-01T12:00:00Z",
      "source_span_ids": ["span:commission_guidance:12"],
      "properties": {
        "jurisdiction": "EU",
        "authority_type": "state_aid_guidance"
      }
    }
  ],
  "edges": [
    {
      "id": "edge:guidelines-regulated-by-commission",
      "from_node_id": "policy:state-aid-guidelines",
      "to_node_id": "actor:european-commission",
      "edge_type": "regulated_by",
      "source_span_ids": ["span:commission_guidance:12"],
      "confidence": 0.86,
      "review_state": "approved_with_caveats",
      "review_scope": "graph_edge",
      "review_action_ids": ["review:legal_editor:2026-06-01"],
      "caveat_ids": ["caveat:state-aid-guidance-update-risk"],
      "expires_at": "2026-12-31T23:59:59Z",
      "blocked_workflows": ["automated_legal_opinion"],
      "reviewed_at": "2026-06-01T12:00:00Z",
      "temporal_scope": {
        "valid_from": "2026-01-01",
        "valid_until": null,
        "status": "current"
      },
      "explanation": "The Commission authority constrains subsidy design."
    }
  ],
  "communities": [
    {
      "id": "cluster:state-aid",
      "label": "State aid and subsidy design",
      "node_ids": ["actor:european-commission", "policy:state-aid-guidelines"],
      "why_surfaced": "high relevance to legal feasibility"
    }
  ],
  "layout_hints": {
    "default_lens": "stakeholder_power_lens",
    "ranked_focus_nodes": ["actor:european-commission"],
    "review_overlay": true,
    "temporal_filter_default": "current"
  },
  "health": {
    "unsupported_edge_count": 0,
    "unreviewed_edge_count": 2,
    "stale_edge_count": 1,
    "contradiction_cluster_count": 1
  }
}
```

## PRAXIS `graph_preview_v1`

The PRAXIS graph preview API should return a small renderable graph, not only
counts:

```json
{
  "schema_version": "graph_preview_v1",
  "capsule_id": "cap_eu_energy_situation_2026_q3",
  "graph_profile": "situation_graph_v1",
  "nodes": [
    {
      "id": "actor:european-commission",
      "label": "European Commission",
      "node_type": "institution",
      "rank": 0.98,
      "why_surfaced": "central authority node for subsidy feasibility",
      "review_state": "approved"
    }
  ],
  "edges": [
    {
      "id": "edge:guidelines-regulated-by-commission",
      "from": "policy:state-aid-guidelines",
      "to": "actor:european-commission",
      "edge_type": "regulated_by",
      "review_state": "approved_with_caveats",
      "source_receipt_links": ["source:commission_guidance#span:12"],
      "temporal_status": "current"
    }
  ],
  "clusters": [
    {
      "id": "cluster:legal-feasibility",
      "label": "Legal feasibility",
      "node_ids": ["actor:european-commission", "policy:state-aid-guidelines"],
      "why_surfaced": "required for decision brief"
    }
  ],
  "review_styles": {
    "approved": "solid",
    "approved_with_caveats": "dashed",
    "needs_review": "muted",
    "rejected": "hidden_by_default"
  },
  "temporal_filters": ["current", "stale", "superseded", "forecast", "contested"],
  "source_receipt_links": ["source:commission_guidance"],
  "warnings": ["2 unreviewed edges hidden by default"]
}
```

## Capsule Type Graph Profiles

The profiles below are defaults. A specific capsule can tighten or extend its
local ontology while still normalizing exported nodes and edges back to the
registered vocabulary.

| Capsule type | Graph profile | Required nodes | Required edges | PRAXIS lens |
| --- | --- | --- | --- | --- |
| User Capsule | `user_context_graph_v1` | `actor`, `concept`, `output_contract`, `rights_policy` | `uses_device`, `has_output_rule`, `has_rights_policy` | user context |
| Situation Capsule | `situation_graph_v1` | `actor`, `institution`, `claim`, `event`, `risk`, `source`, `source_span`, `concept`, `policy_instrument` | `supports`, `mentions`, `contradicts`, `supersedes`, `depends_on`, `regulated_by`, `influences`, `incentivized_by`, `causes` | situation map with source, stakeholder, scenario, and domain lenses |
| Tool Capsule | `tool_method_graph_v1` | `reasoning_device`, `claim`, `risk`, `output_contract`, `concept` | `uses_device`, `depends_on`, `has_output_rule`, `belongs_to_frame`, `supports` | method trace |
| Output Capsule | `output_trace_graph_v1` | `output_contract`, `claim`, `source_span`, `reasoning_device`, `review_action` | `supports`, `uses_device`, `reviewed_by`, `has_output_rule` | artifact trace |

### Situation Graph Lenses

| Lens | Uses | Typical purpose |
| --- | --- | --- |
| `source_proof_lens` | `source`, `source_span`, `claim`, `supports`, `contradicts`, `supersedes` | prove what the source does and does not support |
| `domain_semantic_lens` | `concept`, `institution`, `policy_instrument`, `belongs_to_frame`, `regulated_by` | make local terms, authorities, and instruments legible |
| `stakeholder_power_lens` | `actor`, `institution`, `risk`, `influences`, `incentivized_by` | map actors, incentives, constraints, legitimacy, and missing groups |
| `scenario_causality_lens` | `event`, `claim`, `risk`, `decision`, `causes`, `depends_on` | separate current facts from assumptions, indicators, branches, and triggers |

These lenses do not create new capsule classes. They are visualization,
retrieval, and reasoning views over the embedded graph inside a Situation
Capsule.

## Graph Adapter Profiles

Graph adapters are projections of the embedded graph, not canonical state.

| Adapter profile | Status | Use |
| --- | --- | --- |
| `embedded_graph_v1` | required | compact graph inside the signed bundle |
| `postgres_projection_v1` | required for runtime | relational graph tables plus JSONB extension fields |
| `jsonld_projection_v1` | required for export compatibility | JSON-LD semantic export |
| `ladybug_projection_v1` | optional | embedded graph database projection for local graph exploration, algorithms, and large capsule graph analysis |
| `graphiti_projection_v1` | optional | temporal graph research and future memory adapter |
| `graphrag_projection_v1` | optional | corpus-level community summaries and large-source synthesis |

Adapters may cache, rank, project, or visualize graph records. They must not
promote new canonical graph facts without source spans and review ledger state.

## Semantic Export Profiles

| Profile | Standard anchor | Bundle role |
| --- | --- | --- |
| `jsonld_semantics_v1` | JSON-LD | linked-data serialization for capsule graph and identity |
| `prov_lineage_v1` | PROV-O | source, extraction, review, and compile provenance |
| `skos_concepts_v1` | SKOS | controlled policy concepts, synonyms, broader/narrower terms |
| `shacl_constraints_v1` | SHACL inspiration | graph and ontology validation constraints |
| `odrl_rights_v1` | ODRL | permissions, prohibitions, duties, and reuse policy |
| `owl_inference_v1` | OWL, optional | richer consistency checks or inferred relations after the JSON contract works |
