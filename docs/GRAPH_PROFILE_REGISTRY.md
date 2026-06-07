# Graph Profile Registry

Status: canonical registry for embedded capsule graph profiles.

Use this file when implementing `graph_slice.json`, graph validation,
PostgreSQL graph tables, PRAXIS graph preview APIs, and capsule examples.

## Core Rule

Every graph node and edge in a promoted capsule must use a registered class or
declare an explicit local alias that maps to a registered class. Examples,
fixtures, APIs, and UI previews should not invent edge names ad hoc.

## Canonical Node Classes

| Node class | Required for | Meaning |
| --- | --- | --- |
| `actor` | stakeholder, situation, scenario | person, organization, coalition, stakeholder group |
| `institution` | stakeholder, domain, situation | public body, agency, court, firm, NGO, platform |
| `source` | all source-backed capsules | source document, dataset, interview, note, feed item |
| `source_span` | all source-backed capsules | passage, page, table, paragraph, timestamp, or cell |
| `claim` | situation, source, scenario | factual, causal, normative, forecast, or procedural claim |
| `event` | situation, scenario, decision-clock | dated occurrence or procedural milestone |
| `concept` | domain, ontology, situation | ontology term or frame concept |
| `policy_instrument` | domain, situation, stakeholder | law, rule, subsidy, tax, sanction, target, procedure |
| `risk` | situation, scenario, output | analytical, legal, implementation, political, ethical risk |
| `decision` | situation, output, scenario | choice point, recommendation, review decision, output action |
| `reasoning_device` | thinking-device, situation | method used to reason through the issue |
| `review_action` | all promoted capsules | approval, rejection, caveat, escalation, recertification |
| `output_contract` | output, situation | rules for memo, brief, graph preview, or agent context pack |
| `rights_policy` | marketplace, expert-pick | permissions, prohibitions, duties, export/sharing rules |

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
  "capsule_id": "cap_eu_energy_stakeholders_2026_q3",
  "graph_profile": "stakeholder_graph_v1",
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
    "default_lens": "stakeholder_map",
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
  "capsule_id": "cap_eu_energy_stakeholders_2026_q3",
  "graph_profile": "stakeholder_graph_v1",
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

| Capsule type | Graph profile | Required nodes | Required edges | PRAXIS lens |
| --- | --- | --- | --- | --- |
| User Capsule | `user_context_graph_v1` | `actor`, `concept`, `output_contract`, `rights_policy` | `uses_device`, `has_output_rule`, `has_rights_policy` | user context |
| Team Capsule | `team_memory_graph_v1` | `actor`, `institution`, `source`, `output_contract` | `authored_by`, `supports`, `reviewed_by` | team memory |
| Situation Capsule | `situation_graph_v1` | `actor`, `institution`, `claim`, `event`, `risk`, `source_span` | `supports`, `mentions`, `contradicts`, `supersedes`, `depends_on` | situation map |
| Source Capsule | `source_proof_graph_v1` | `source`, `source_span`, `claim`, `concept` | `supports`, `mentions`, `contradicts`, `supersedes` | source proof |
| Domain Capsule | `domain_ontology_graph_v1` | `concept`, `institution`, `policy_instrument`, `source` | `belongs_to_frame`, `regulated_by`, `mentions` | ontology explorer |
| Thinking Device Capsule | `reasoning_device_graph_v1` | `reasoning_device`, `claim`, `risk`, `output_contract` | `uses_device`, `depends_on`, `has_output_rule` | method trace |
| Stakeholder Capsule | `stakeholder_graph_v1` | `actor`, `institution`, `claim`, `risk`, `policy_instrument` | `influences`, `incentivized_by`, `regulated_by`, `supports`, `contradicts` | stakeholder map |
| Scenario Capsule | `scenario_graph_v1` | `event`, `claim`, `risk`, `decision`, `source_span` | `causes`, `depends_on`, `supersedes`, `supports` | scenario tree |
| Output Capsule | `output_trace_graph_v1` | `output_contract`, `claim`, `source_span`, `reasoning_device`, `review_action` | `supports`, `uses_device`, `reviewed_by`, `has_output_rule` | artifact trace |
| Expert Pick Capsule | `expert_pick_graph_v1` | `review_action`, `source`, `claim`, `rights_policy` | `reviewed_by`, `supports`, `has_rights_policy`, `forbidden_for` | trust receipt |
