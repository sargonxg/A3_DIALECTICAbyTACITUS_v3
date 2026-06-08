# PRAXIS Capsule Specification

Status: draft contract for foundation build implementation.

## Definition

A PRAXIS Capsule is a signed, portable knowledge-work object that can be
stored, shared, reviewed, combined, and used by humans and PRAXIS agents.

It contains:

- a model of a user, situation, tool, or output;
- the evidence behind that model;
- a model of how to reason, act, reuse, or hand off within the declared scope.

## Bundle Shape

The canonical export is a directory or compressed archive:

```text
capsule/
  manifest.json
  capsule.json
  source_ledger.jsonl
  temporal_ledger.jsonl
  ontology_slice.json
  graph_slice.json
  graph_semantics.jsonld
  graph_constraints.json
  reasoning_playbook.json
  language_profile.json
  agent_guidance.json
  retrieval_pack.jsonl
  output_contracts.json
  review_ledger.jsonl
  rights_profile.json
  marketplace_listing.json
  capsule_health.json
  eval_report.json
  checksums.sha256
  signature.json
```

## `manifest.json`

Required fields:

- `capsule_id`
- `schema_version`
- `title`
- `summary`
- `created_at`
- `compiled_at`
- `tenant_id`
- `project_id`
- `status`
- `freshness`
- `source_count`
- `review_state`
- `capsule_type`
- `rights_profile`
- `graph_profile`
- `reasoning_profile`
- `language_profile`
- `agent_guidance_profile`
- `compatibility_profile`
- `bundle_digest`
- `compiler_version`
- `capsule_health`

## `capsule.json`

Required sections:

- `identity_context`: user, team, institution, mandate, audience;
- `situation_context`: issue, geography, sector, time horizon, constraints;
- `analytical_context`: frames, assumptions, uncertainties, questions;
- `policy_context`: instruments, authorities, stakeholders, tradeoffs;
- `risk_context`: known risks, unknowns, edge cases, escalation triggers;
- `usage_context`: intended PRAXIS workflows and forbidden uses.

## `source_ledger.jsonl`

One record per source or source span.

Required fields:

- `source_id`
- `source_type`
- `title`
- `uri`
- `publisher`
- `published_at`
- `retrieved_at`
- `language`
- `license_or_access`
- `trust_status`
- `span_id`
- `span_locator`
- `hash`
- `notes`

## `temporal_ledger.jsonl`

One record per temporal claim.

Required fields:

- `claim_id`
- `claim_text`
- `valid_time_start`
- `valid_time_end`
- `published_at`
- `observed_at`
- `status`
- `confidence`
- `supersedes`
- `superseded_by`
- `source_ids`

Allowed statuses:

- `current`
- `stale`
- `superseded`
- `forecast`
- `contested`
- `unknown`

## `ontology_slice.json`

Contains the local vocabulary and semantic contract needed for this capsule.
It is not a global taxonomy and must not assume that actor/claim analysis is the
right ontology for every capsule.

The ontology slice should be built from a capsule-specific ontology blueprint:

- User Capsules emphasize role, authority, preference, privacy, and output
  style semantics.
- Situation Capsules emphasize actors, claims, events, risks, policy
  instruments, source state, stakeholder lenses, domain semantics, scenario
  hypotheses, caveats, and decision clocks.
- Tool Capsules emphasize method steps, required inputs, intellectual and
  philosophical lenses, source standards, failure modes, expert caveats, and
  review criteria.
- Output Capsules emphasize artifact structure, claim lineage, source receipts,
  reuse rules, and caveats.

Source packs, domain ontologies, stakeholder maps, scenario branches, expert
endorsements, and graph modules are modeled as internal ontology layers, graph
lenses, review metadata, or marketplace metadata inside those four classes.
They are not PRAXIS-importable capsule classes.

Required sections:

- `ontology_id`
- `version`
- `namespace`
- `language`
- `domains`
- `terms`
- `mappings`
- `frame_memberships`
- `deprecations`
- `review_notes`

Every source-backed definition must point to source spans or review notes.
Every material term must carry enough scope and review state for PRAXIS to know
whether it can be used directly, caveated, hidden, or escalated.

## `graph_slice.json`

Contains the compact graph needed by PRAXIS.

Top-level required sections:

- `schema_version`
- `capsule_id`
- `graph_profile`
- `nodes`
- `edges`
- `communities`
- `layout_hints`
- `health`

Node types:

- `actor`
- `institution`
- `source`
- `source_span`
- `event`
- `claim`
- `concept`
- `policy_instrument`
- `risk`
- `decision`
- `reasoning_device`
- `review_action`
- `output_contract`
- `rights_policy`

Edge types:

- `supports`
- `contradicts`
- `causes`
- `influences`
- `incentivized_by`
- `depends_on`
- `mentions`
- `authored_by`
- `regulated_by`
- `reviewed_by`
- `supersedes`
- `belongs_to_frame`
- `uses_device`
- `forbidden_for`
- `has_output_rule`
- `has_rights_policy`

Every edge must include provenance:

- `source_ids`
- `created_by`
- `created_at`
- `confidence`
- `review_state`
- `temporal_scope`
- `explanation`

Use `docs/GRAPH_PROFILE_REGISTRY.md` for canonical node classes, edge classes,
approved aliases, graph profile names, and PRAXIS preview payloads.

The list above is a shared export vocabulary. The capsule-specific ontology can
add local properties, aliases, and domain terms, but promoted graph nodes and
edges must normalize back to registered classes or explicitly declare how PRAXIS
should treat the local class.

## `graph_semantics.jsonld`

Contains an optional linked-data representation of the graph and capsule
identity. The first implementation should keep this simple and compatible with
normal JSON processing.

Design anchors:

- JSON-LD for linked-data serialization;
- PROV-O for generation and review provenance;
- SKOS for controlled concepts;
- ODRL for rights and usage semantics.

## `graph_constraints.json`

Defines the required graph profile for the capsule type.

Required sections:

- `node_classes`
- `edge_classes`
- `required_edge_fields`
- `review_state_rules`
- `temporal_rules`
- `source_provenance_rules`
- `praxis_visualization_hints`

Example:

```json
{
  "graph_profile": "situation_graph_v1",
  "graph_lens": "stakeholder_power_lens",
  "required_node_classes": ["actor", "institution", "claim", "source", "risk"],
  "required_edge_fields": [
    "source_ids",
    "source_span_ids",
    "temporal_scope",
    "review_state",
    "explanation"
  ]
}
```

## `reasoning_playbook.json`

This is where DIALECTICA captures expert thinking, not only expert facts.

Required sections:

- `mental_models`
- `policy_heuristics`
- `philosophical_lenses`
- `adversarial_questions`
- `causal_questions`
- `temporal_questions`
- `red_flags`
- `recommended_output_patterns`
- `reviewer_guidance`

Examples:

- distributional analysis;
- institutional capacity analysis;
- incentive mapping;
- second-order effects;
- legitimacy and consent;
- precautionary principle;
- game-theoretic actor response;
- path dependency;
- epistemic humility and uncertainty disclosure.

## `language_profile.json`

This is where DIALECTICA captures human-gated language. It is separate from
`output_contracts.json` because the same language rules can apply across
briefs, memos, stakeholder maps, scenario updates, and agent handoffs.

Required sections:

- `profile_id`
- `primary_language`
- `secondary_languages`
- `audience_register`
- `approved_terms`
- `deprecated_terms`
- `blocked_phrases`
- `framing_rules`
- `translation_notes`
- `citation_language`
- `uncertainty_language`
- `review_state`

Every material term or framing rule should include:

- a stable rule or term id;
- the approved wording or blocked wording;
- rationale;
- source span ids or review action ids;
- valid scope;
- review state.

Example:

```json
{
  "profile_id": "language:policy-brief-en-v1",
  "primary_language": "en",
  "secondary_languages": ["fr", "es"],
  "audience_register": "ministerial_decision_brief",
  "approved_terms": [
    {
      "term_id": "term:state-aid",
      "label": "state aid",
      "definition": "Public support that may affect competition and requires jurisdiction-specific caveats.",
      "review_state": "approved_with_caveats"
    }
  ],
  "blocked_phrases": ["guaranteed legal compliance"],
  "framing_rules": [
    {
      "rule_id": "language:caveat-legal-status",
      "rule": "Use 'may require authority review' instead of stating legal clearance.",
      "review_state": "approved"
    }
  ],
  "citation_language": "Use source receipts for factual and legal-sensitive claims.",
  "uncertainty_language": "State confidence and unresolved evidence gaps plainly.",
  "review_state": "approved_with_caveats"
}
```

## `agent_guidance.json`

This is the model-facing execution contract for PRAXIS agents. It is separate
from the reasoning playbook because a method can be reusable across many
workflows while agent permissions, tool policy, citation rules, stop
conditions, and handoff rules can differ by capsule.

Required sections:

- `allowed_workflows`
- `tool_policy`
- `citation_policy`
- `graph_use_policy`
- `language_profile_refs`
- `reasoning_sequence`
- `context_budget_policy`
- `stop_conditions`
- `handoff_policy`
- `audit_receipts_required`

Example:

```json
{
  "allowed_workflows": ["decision_brief", "stakeholder_map"],
  "tool_policy": {
    "allowed_tools": ["capsule_search", "source_preview", "graph_preview"],
    "blocked_tools": ["automated_legal_opinion"]
  },
  "citation_policy": "cite_source_span_for_every_nontrivial_claim",
  "graph_use_policy": "prefer approved current edges; show needs_review edges as warnings",
  "language_profile_refs": ["language:policy-brief-en-v1"],
  "reasoning_sequence": ["decision_clock_v1", "stakeholder_scan_v1"],
  "context_budget_policy": "include graph focus nodes and contested claims first",
  "stop_conditions": ["material_claim_missing_source", "rights_policy_blocks_workflow"],
  "handoff_policy": "ask reviewer before public or legal-sensitive output",
  "audit_receipts_required": ["capsule_id", "bundle_digest", "source_span_ids", "graph_edge_ids"]
}
```

## `retrieval_pack.jsonl`

Records optimized for PRAXIS context injection.

Required fields:

- `chunk_id`
- `source_ids`
- `text`
- `embedding_ref`
- `tags`
- `temporal_status`
- `citation_hint`
- `review_state`
- `intended_use`

## `output_contracts.json`

Defines what the capsule is meant to help generate.

Initial output types:

- policy memo;
- decision brief;
- stakeholder map;
- scenario analysis;
- risk register;
- research synthesis;
- evidence table;
- talking points;
- legislative or regulatory analysis;
- PRAXIS agent workflow context pack.

Each output contract should include:

- required citations;
- required uncertainty handling;
- recommended structure;
- forbidden claims;
- escalation criteria;
- reviewer expectations.

## `review_ledger.jsonl`

Required fields:

- `review_id`
- `reviewer_id`
- `reviewer_role`
- `reviewed_object_type`
- `reviewed_object_id`
- `decision`
- `scope`
- `notes`
- `created_at`
- `expires_at`

Allowed decisions:

- `approved`
- `rejected`
- `needs_revision`
- `approved_with_caveats`
- `escalated`

## `rights_profile.json`

Defines how the capsule may be used, shared, exported, or listed.

Required sections:

- `owner`
- `allowed_workflows`
- `prohibited_workflows`
- `export_policy`
- `sharing_policy`
- `source_license_summary`
- `sensitive_fields`
- `redaction_rules`
- `marketplace_policy`
- `expires_at`

## `marketplace_listing.json`

Optional for private capsules, required for listed capsules.

Required fields:

- `listing_id`
- `capsule_id`
- `title`
- `capsule_type`
- `domain_tags`
- `geography`
- `language`
- `review_level`
- `reviewer_summary`
- `freshness_status`
- `source_count`
- `rights_summary`
- `known_caveats`
- `compatible_capsules`
- `fork_policy`
- `eval_snapshot`

## `capsule_health.json`

Required fields:

- `capsule_id`
- `schema_version`
- `source_coverage`
- `unsupported_claim_count`
- `stale_claim_count`
- `contested_claim_count`
- `graph_provenance_coverage`
- `ontology_coverage`
- `review_coverage`
- `reasoning_device_coverage`
- `output_contract_completeness`
- `praxis_eval_score`
- `blocking_warnings`
- `recommended_next_actions`

Capsule health is a gate. It should not be only a UI score.

## Compatibility Rules

- New optional fields may be added in minor schema versions.
- Required fields require a major schema version bump.
- Deprecated fields must remain readable for at least one major version.
- PRAXIS must reject capsule bundles with unsupported major versions.
- Capsule validators must produce actionable error paths.

## Foundation Acceptance Criteria

The first implementation is acceptable when:

- a fixture bundle can be generated from local source files;
- the bundle validates against the schema;
- the source ledger can prove where each derived claim came from;
- the review ledger can block promotion;
- PRAXIS can consume the manifest and retrieval pack;
- the eval harness can compare raw and capsule-augmented outputs.
