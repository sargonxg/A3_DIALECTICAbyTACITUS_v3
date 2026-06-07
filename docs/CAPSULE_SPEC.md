# PRAXIS Capsule Specification

Status: draft contract for MVP implementation.

## Definition

A PRAXIS Capsule is a signed, portable analytical context object that can be
stored, shared, reviewed, combined, and used by PRAXIS agents.

It contains both:

- a model of the situation;
- a model of how to think about the situation.

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
  reasoning_playbook.json
  retrieval_pack.jsonl
  output_contracts.json
  review_ledger.jsonl
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
- `bundle_digest`
- `compiler_version`

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

Contains the local vocabulary needed for this capsule.

Required sections:

- `domains`
- `actors`
- `institutions`
- `policy_instruments`
- `concepts`
- `frames`
- `mapping_confidence`
- `review_notes`

## `graph_slice.json`

Contains the compact graph needed by PRAXIS.

Node types:

- `actor`
- `institution`
- `policy`
- `event`
- `claim`
- `source`
- `concept`
- `risk`
- `decision`

Edge types:

- `supports`
- `contradicts`
- `causes`
- `influences`
- `depends_on`
- `mentions`
- `authored_by`
- `reviewed_by`
- `supersedes`
- `belongs_to_frame`

Every edge must include provenance:

- `source_ids`
- `created_by`
- `created_at`
- `confidence`
- `review_state`

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

## Compatibility Rules

- New optional fields may be added in minor schema versions.
- Required fields require a major schema version bump.
- Deprecated fields must remain readable for at least one major version.
- PRAXIS must reject capsule bundles with unsupported major versions.
- Capsule validators must produce actionable error paths.

## MVP Acceptance Criteria

The first implementation is acceptable when:

- a fixture bundle can be generated from local source files;
- the bundle validates against the schema;
- the source ledger can prove where each derived claim came from;
- the review ledger can block promotion;
- PRAXIS can consume the manifest and retrieval pack;
- the eval harness can compare raw and capsule-augmented outputs.
