# Capsule Build Examples

## Purpose

This page shows how real policy capsules are built. The examples are concrete
enough to guide implementation and broad enough to scale across policy domains.

## Example 1: Stakeholder Analysis Capsule

Goal: build a capsule that helps PRAXIS generate a stakeholder map and decision
brief for an industrial electricity price-support policy.

### Inputs

```text
source_pack/
  commission_guidance.pdf
  member_state_budget_note.pdf
  regulator_hearing_transcript.pdf
  industry_association_letter.pdf
  labor_union_statement.pdf
  think_tank_analysis.pdf
  old_price_support_brief_2024.pdf
  analyst_notes.md
```

The source pack intentionally includes one outdated source and one source
conflict so the capsule proves temporal and contradiction handling.

### Build Flow

```text
1. ingest source pack
2. normalize source spans
3. extract actors, claims, dates, and policy instruments
4. map actors to incentives and constraints
5. build embedded graph
6. apply stakeholder-analysis reasoning device
7. identify source gaps and contested claims
8. request expert review
9. compile signed capsule bundle
10. export PRAXIS context pack
```

### Actor Extraction

```json
{
  "actors": [
    {
      "id": "actor:european-commission",
      "label": "European Commission",
      "actor_type": "institution",
      "authority": "state-aid guidance and enforcement",
      "review_state": "approved"
    },
    {
      "id": "actor:energy-intensive-industry",
      "label": "Energy-intensive industry coalition",
      "actor_type": "coalition",
      "incentives": ["lower input costs", "predictable relief"],
      "constraints": ["state aid limits", "public budget scrutiny"],
      "review_state": "needs_review"
    }
  ]
}
```

### Embedded Graph

```text
European Commission
  | regulates
  v
State aid guidelines
  | constrains
  v
Industrial electricity subsidy
  | benefits
  v
Energy-intensive industry coalition
  ^
  | criticizes budget burden
Labor union ---------> Employment risk
```

Graph edges are not just arrows. Each arrow needs source spans, time, review
state, and an explanation.

```json
{
  "edge_id": "edge:guidelines-constrain-subsidy",
  "from_node_id": "policy:state-aid-guidelines",
  "to_node_id": "policy:industrial-electricity-subsidy",
  "edge_type": "constrains",
  "source_span_ids": ["span:commission_guidance:12"],
  "temporal_scope": {
    "valid_from": "2026-01-01",
    "valid_until": null,
    "status": "current"
  },
  "confidence": 0.86,
  "review_state": "approved_with_caveats",
  "explanation": "Guidance limits subsidy design and notification path."
}
```

### Thinking Device: Stakeholder Analysis

The stakeholder-analysis device captures tacit policy method.

```json
{
  "device_id": "stakeholder_analysis_v1",
  "purpose": "Identify who can affect the decision, who is affected, and how incentives shape likely behavior.",
  "inputs_required": [
    "actors",
    "institutions",
    "policy_instruments",
    "claims",
    "temporal_facts",
    "source_spans"
  ],
  "method_steps": [
    "List formal decision makers and informal veto players.",
    "Separate affected stakeholders from influential stakeholders.",
    "Map incentives, constraints, resources, and likely objections.",
    "Identify coalitions, conflicts, and asymmetric information.",
    "Mark source gaps and avoid motive claims without evidence.",
    "Generate decision-relevant implications and engagement options."
  ],
  "failure_modes": [
    "treating loud actors as powerful actors",
    "missing implementation agencies",
    "confusing legal authority with political feasibility",
    "inventing motives not grounded in sources"
  ],
  "review_questions": [
    "Which actor is missing?",
    "Which incentive is inferred rather than sourced?",
    "Which stakeholder can block implementation?",
    "Which claim expires or changes after the next budget update?"
  ]
}
```

### Review Output

```text
Reviewer: domain_expert
Decision: approved_with_caveats
Caveat: industry coalition incentives are well supported, but labor union
position is based on one statement and needs a second source before promotion
to marketplace.
Expiry: 2026-09-30
```

### PRAXIS Context Pack

PRAXIS should receive a compact pack:

```json
{
  "capsule_id": "cap_eu_energy_stakeholders_2026_q3",
  "task_fit": ["stakeholder_map", "decision_brief", "risk_register"],
  "summary": "Stakeholder context for industrial electricity price support.",
  "must_use_sources": ["source:commission_guidance", "source:budget_note"],
  "must_warn": [
    "Labor union position has limited source coverage.",
    "Old 2024 brief is superseded for state-aid constraints."
  ],
  "reasoning_devices": ["stakeholder_analysis_v1", "sourceability_check_v1"],
  "graph_focus": [
    "actor:european-commission",
    "policy:state-aid-guidelines",
    "actor:energy-intensive-industry"
  ],
  "forbidden_claims": [
    "Do not state that the subsidy is legally approved unless a source confirms approval."
  ]
}
```

## Example 2: Decision Clock Capsule

Goal: tell PRAXIS what deadlines, review moments, source expiry dates, and
decision gates matter.

Typical contents:

- event timeline;
- deadlines;
- legal effective dates;
- review expiry dates;
- source publication dates;
- stale claims;
- supersession links;
- indicators to watch.

Graph shape:

```text
Source publication -> Claim validity -> Review expiry -> Decision deadline
       |                   |                 |                 |
       v                   v                 v                 v
  source node         temporal fact      review action       decision node
```

Use this capsule when PRAXIS must answer, "What is true now, what changed, and
what must be decided before the next gate?"

## Example 3: Thinking Device Capsule

Goal: package a reusable intellectual tool, such as ACH, red-team review, or
distributional analysis.

Contents:

- method steps;
- required inputs;
- source minimums;
- common failure modes;
- example application;
- reviewer criteria;
- output contract;
- graph profile.

This capsule can be combined with a Situation Capsule. PRAXIS then gets both
the issue context and the method for reasoning about it.

## Example 4: Output Capsule

Goal: preserve a completed memo, brief, risk register, or scenario analysis as
a reusable source-grounded object.

Contents:

- final artifact;
- citations;
- assumptions;
- reviewer caveats;
- graph slice used;
- reasoning devices used;
- update instructions;
- forbidden reuse;
- next-review date.

Output Capsules create handover memory. A team can return months later and
understand not only what was written, but why it was written that way.

## Implementation Lessons

Real capsules must include:

- source spans, not only source titles;
- temporal state, not only dates;
- graph edges, not only named entities;
- reasoning devices, not only summaries;
- human review, not only automated confidence;
- output contracts, not only generated text;
- bundle signatures, not only JSON files.

That is the difference between generic retrieval and PRAXIS Augmented
Generation with PRAXIS Capsules by TACITUS.
